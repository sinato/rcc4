mod error;

use error::CompileError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use rcc_parser::expression::{Exp12, Exp13, Exp16, Exp2, Expression};
use rcc_parser::function::Function;
use rcc_parser::program::Program;
use rcc_parser::statement::{DeclareStatement, ExpressionStatement, ReturnStatement, Statement};
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
        let arguments = function.arguments;

        let i64_type = self.context.i64_type();

        // create param_types
        let mut param_types: Vec<BasicTypeEnum> = vec![];
        for _ in 0..(arguments.len()) {
            param_types.push(i64_type.into());
        }

        let function_value =
            self.module
                .add_function(&identifier, i64_type.fn_type(&param_types, false), None);

        let basic_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(basic_block);

        let mut environment = Environment::new();
        for (i, (argument_identifier, argument_type)) in arguments.into_iter().enumerate() {
            let identifier = argument_identifier.get_token().get_identifier()?;
            let _ty = argument_type.get_token().get_type()?;
            let arg_value = function_value
                .get_nth_param(i as u32)
                .unwrap()
                .into_int_value();
            let pointer_value = self.builder.build_alloca(i64_type, "arg");
            self.builder.build_store(pointer_value, arg_value);
            environment.insert(identifier, pointer_value);
        }

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
        let type_struct = declare_statement.type_struct;
        let identifier = type_struct.get_identifier();
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
        self.emit_expression(expression_statement.expression, environment)
    }

    fn emit_return_statement(
        &self,
        return_statement: ReturnStatement,
        environment: &Environment,
    ) -> Result<()> {
        let return_expression_node = return_statement.expression;
        let ret_value = self.emit_expression(return_expression_node, environment)?;
        self.builder.build_return(Some(&ret_value));
        Ok(())
    }

    fn emit_expression(
        &self,
        node: Expression,
        environment: &'a Environment,
    ) -> Result<IntValue<'a>> {
        self.emit_exp2(node.expression, environment)
    }
    fn emit_exp2(&self, node: Exp2, environment: &'a Environment) -> Result<IntValue<'a>> {
        match node {
            Exp2::Single(exp) => Ok(self.emit_exp12(exp, environment)?),
            Exp2::Eq(operands) => {
                if operands.len() == 1 {
                    let mut operand_itr = operands.into_iter();
                    self.emit_exp12(operand_itr.next().unwrap(), environment)
                } else {
                    // todo emit multiple term (eg. a = b = c)
                    let mut operand_itr = operands.into_iter();
                    let lhs: PointerValue =
                        self.emit_expression_node_as_lhs(operand_itr.next().unwrap(), environment)?;
                    let rhs: IntValue =
                        self.emit_exp12(operand_itr.next().unwrap(), environment)?;
                    self.builder.build_store(lhs, rhs);
                    Ok(rhs)
                }
            }
        }
    }
    fn emit_exp12(&self, node: Exp12, environment: &'a Environment) -> Result<IntValue<'a>> {
        match node {
            Exp12::Single(exp) => Ok(self.emit_exp13(exp, environment)?),
            Exp12::Add(operands) => {
                let mut const_nums: Vec<IntValue> = Vec::new();
                for operand in operands.into_iter() {
                    const_nums.push(self.emit_exp13(operand, environment)?);
                }
                let mut reduced = self.context.i64_type().const_int(0, false);
                for const_num in const_nums.into_iter() {
                    reduced = self.builder.build_int_add(reduced, const_num, "sum");
                }
                Ok(reduced)
            }
        }
    }
    fn emit_exp13(&self, node: Exp13, environment: &'a Environment) -> Result<IntValue<'a>> {
        match node {
            Exp13::Single(exp) => Ok(self.emit_exp16(exp, environment)?),
            Exp13::Mul(operands) => {
                let mut const_nums: Vec<IntValue> = Vec::new();
                for operand in operands.into_iter() {
                    const_nums.push(self.emit_exp16(operand, environment)?);
                }
                let mut reduced = self.context.i64_type().const_int(1, false);
                for const_num in const_nums.into_iter() {
                    reduced = self.builder.build_int_mul(reduced, const_num, "mul");
                }
                Ok(reduced)
            }
        }
    }
    fn emit_exp16(&self, node: Exp16, environment: &'a Environment) -> Result<IntValue<'a>> {
        match node {
            Exp16::Number(number) => Ok(self.context.i64_type().const_int(number, false)),
            Exp16::Identifier(identifier) => {
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
            Exp16::FunctionCall(identifier, parameter_expressions) => {
                if let Some(fn_value) = self.module.get_function(&identifier) {
                    let mut parameters: Vec<BasicValueEnum> = Vec::new();
                    for parameter in parameter_expressions {
                        parameters.push(self.emit_expression(parameter, environment)?.into());
                    }
                    let func_calls_site =
                        self.builder.build_call(fn_value, &parameters, "func_call");
                    Ok(func_calls_site
                        .try_as_basic_value()
                        .left()
                        .unwrap()
                        .into_int_value())
                } else {
                    Err(CompileError::NotFound("function".to_owned()))
                }
            }
        }
    }

    fn emit_expression_node_as_lhs(
        &self,
        node: Exp12,
        environment: &'a Environment,
    ) -> Result<PointerValue<'a>> {
        match node {
            Exp12::Single(exp) => match exp {
                Exp13::Single(exp) => match exp {
                    Exp16::Identifier(identifier) => {
                        if let Some(pointer_value) = environment.get(&identifier) {
                            Ok(pointer_value)
                        } else {
                            Err(CompileError::Undeclared(identifier))
                        }
                    }
                    _ => unimplemented!(),
                },
                _ => Err(CompileError::Unexpect(
                    "Expect declared variable identifier".to_owned(),
                )),
            },
            _ => Err(CompileError::Unexpect(
                "Expect declared variable identifier".to_owned(),
            )),
        }
    }
}
