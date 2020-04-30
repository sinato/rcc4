mod error;

use super::parse::node::{ExpressionNode, FunctionNode};
use super::tokenize::token::Token;
use error::CompileError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;
use std::path;

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
        node: FunctionNode,
    ) -> Result<(), CompileError> {
        let emitter = Emitter {
            context,
            builder,
            module,
        };
        emitter.emit_function(node)
    }

    fn emit_function(&self, node: FunctionNode) -> Result<(), CompileError> {
        let identifier = node.identifier.get_token().get_identifier()?;
        let _return_type = node.return_type;
        let _argument_types = node.argument_types;
        let block = node.block;

        let i64_type = self.context.i64_type();
        let function = self
            .module
            .add_function(&identifier, i64_type.fn_type(&[], false), None);

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let mut ret_value = i64_type.const_int(1, false);
        for expression_node in block.into_iter() {
            ret_value = self.emit_expression_node(expression_node)?;
        }

        self.builder.build_return(Some(&ret_value));
        self.module
            .print_to_file(path::Path::new("compiled.ll"))
            .map_err(|err| From::from(err))
    }

    fn emit_expression_node(&self, node: ExpressionNode) -> Result<IntValue, CompileError> {
        let i64_type = self.context.i64_type();
        let val = match node.get_operator_clone() {
            Token::Number(num) => i64_type.const_int(num, false),
            Token::Identifier(_) => unimplemented!(),
            Token::Operator(op) => {
                let mut const_nums: Vec<IntValue> = Vec::new();
                for operand in node.get_operand().into_iter() {
                    const_nums.push(self.emit_expression_node(*operand)?);
                }
                match op.as_ref() {
                    "+" => {
                        let mut reduced = i64_type.const_int(0, false);
                        for const_num in const_nums.into_iter() {
                            reduced = self.builder.build_int_add(reduced, const_num, "sum");
                        }
                        reduced
                    }
                    "*" => {
                        let mut reduced = i64_type.const_int(1, false);
                        for const_num in const_nums.into_iter() {
                            reduced = self.builder.build_int_mul(reduced, const_num, "mul");
                        }
                        reduced
                    }
                    _ => unimplemented!(),
                }
            }
            Token::Bracket(_) => unimplemented!(),
            Token::Parenthesis(_) => unimplemented!(),
            Token::Return => unimplemented!(),
            Token::Semicolon => unimplemented!(),
        };
        Ok(val)
    }
}
