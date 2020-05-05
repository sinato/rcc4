mod error;

use super::parse::expression::{ExpressionNode, Operator};
use super::parse::function::Function;
use super::parse::program::Program;
use super::parse::statement::{DeclareStatement, ExpressionStatement, ReturnStatement, Statement};
use error::CompileError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{IntValue, PointerValue};
use std::collections::HashMap;
use std::path;

type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
struct Environment<'ctx> {
    variables: HashMap<String, PointerValue<'ctx>>,
}
impl<'ctx> Environment<'ctx> {
    fn new() -> Environment<'ctx> {
        Environment {
            variables: HashMap::new(),
        }
    }
    fn insert(&mut self, identifier: String, pointer: PointerValue<'ctx>) {
        self.variables.insert(identifier, pointer);
    }
    fn get(&self, identifier: &String) -> Option<PointerValue> {
        let mut cloned_variables = self.variables.clone();
        cloned_variables.remove(identifier)
    }
}

pub struct Emitter<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
}

impl<'a, 'ctx> Emitter<'a, 'ctx> {
    pub fn emit(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        program: Program,
    ) -> Result<()> {
        let emitter = Emitter {
            context,
            builder,
            module,
        };
        emitter.emit_program(program)
    }

    fn emit_program(&self, program: Program) -> Result<()> {
        for function in program.functions {
            self.emit_function(function)?;
        }
        return Ok(());
    }

    fn emit_function(&self, function: Function) -> Result<()> {
        let identifier = function.identifier.get_token().get_identifier()?;
        let _return_type = function.return_type;
        let _argument_types = function.argument_types;

        let i64_type = self.context.i64_type();

        let function_value =
            self.module
                .add_function(&identifier, i64_type.fn_type(&[], false), None);

        let basic_block = self.context.append_basic_block(function_value, "entry");
        let mut environment = Environment::new();
        self.builder.position_at_end(basic_block);

        for statement in function.block.into_iter() {
            match statement {
                Statement::Declare(statement) => {
                    self.emit_declare_statement(statement, &mut environment)?
                }
                Statement::Expression(statement) => {
                    self.emit_expression_statement(statement, &environment)?;
                }
            }
        }
        self.emit_return_statement(function.return_statement, &environment)?;

        self.module
            .print_to_file(path::Path::new("compiled.ll"))
            .map_err(|err| From::from(err))
    }

    fn emit_declare_statement(
        &self,
        declare_statement: DeclareStatement,
        environment: &mut Environment<'ctx>,
    ) -> Result<()> {
        let _ty = declare_statement.ty.get_token().get_type()?;
        let identifier = declare_statement.identifier.get_token().get_identifier()?;
        let i64_type = self.context.i64_type();
        let pointer_value = self.builder.build_alloca(i64_type, "variable");
        environment.insert(identifier, pointer_value);
        Ok(())
    }

    fn emit_expression_statement(
        &self,
        expression_statement: ExpressionStatement,
        environment: &'a Environment,
    ) -> Result<IntValue> {
        self.emit_expression_node(expression_statement.expression_node, environment)
    }

    fn emit_return_statement(
        &self,
        return_statement: ReturnStatement,
        environment: &Environment,
    ) -> Result<()> {
        let return_expression_node = return_statement.expression_node;
        let ret_value = self.emit_expression_node(return_expression_node, environment)?;
        self.builder.build_return(Some(&ret_value));
        Ok(())
    }

    fn emit_expression_node(
        &self,
        node: ExpressionNode,
        environment: &'a Environment,
    ) -> Result<IntValue<'a>> {
        let i64_type = self.context.i64_type();

        match node.get_operator_clone() {
            Operator::Add => {
                let mut const_nums: Vec<IntValue> = Vec::new();
                for operand in node.get_operand().into_iter() {
                    const_nums.push(self.emit_expression_node(*operand, environment)?);
                }
                let mut reduced = i64_type.const_int(0, false);
                for const_num in const_nums.into_iter() {
                    reduced = self.builder.build_int_add(reduced, const_num, "sum");
                }
                Ok(reduced)
            }
            Operator::Mul => {
                let mut const_nums: Vec<IntValue> = Vec::new();
                for operand in node.get_operand().into_iter() {
                    const_nums.push(self.emit_expression_node(*operand, environment)?);
                }
                let mut reduced = i64_type.const_int(1, false);
                for const_num in const_nums.into_iter() {
                    reduced = self.builder.build_int_mul(reduced, const_num, "mul");
                }
                Ok(reduced)
            }
            Operator::Eq => {
                let mut operand_itr = node.get_operand().into_iter();
                let lhs: PointerValue =
                    self.emit_expression_node_as_lhs(*operand_itr.next().unwrap(), environment)?;
                let rhs: IntValue =
                    self.emit_expression_node(*operand_itr.next().unwrap(), environment)?;
                self.builder.build_store(lhs, rhs);
                Ok(rhs)
            }
            Operator::FnCall(function_name) => {
                if let Some(fn_value) = self.module.get_function(&function_name) {
                    let func_calls_site = self.builder.build_call(fn_value, &[], "func_call");
                    Ok(func_calls_site
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value())
                } else {
                    Err(CompileError::NotFound("function".to_owned()))
                }
            }
            Operator::Num(num) => Ok(i64_type.const_int(num, false)),
            Operator::Identifier(identifier) => {
                let variable_pointer = environment.get(&identifier);
                if let Some(variable_pointer) = variable_pointer {
                    let value = self
                        .builder
                        .build_load(variable_pointer, "variable_load")
                        .into_int_value();
                    Ok(value)
                } else {
                    Err(CompileError::Undeclared(identifier))
                }
            }
        }
    }

    fn emit_expression_node_as_lhs(
        &self,
        node: ExpressionNode,
        environment: &'a Environment,
    ) -> Result<PointerValue<'a>> {
        match node.get_operator_clone() {
            Operator::Identifier(identifier) => {
                if let Some(pointer_value) = environment.get(&identifier) {
                    Ok(pointer_value)
                } else {
                    Err(CompileError::Undeclared(identifier))
                }
            }
            _ => Err(CompileError::Unexpect(format!(
                "{:?}",
                node.get_operator_clone()
            ))),
        }
    }
}
