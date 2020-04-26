mod error;

use super::tokenize::tokenize;
use error::CompileError;
use inkwell::context::Context;
use std::path;

pub fn compile(code: String) -> Result<(), CompileError> {
    // tokenize
    let tokens = tokenize(code);

    // emit
    let context = Context::create();
    let module = context.create_module("rcc4");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let function = module.add_function("main", i64_type.fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    if tokens.len() == 1 {
        let val: u64 = tokens.get(0).unwrap().get_number()?;
        let const_x = i64_type.const_int(val, false);
        builder.build_return(Some(&const_x));
    } else if tokens.len() == 3 {
        let operator: String = tokens.get(1).unwrap().get_operator()?;
        let const_x = i64_type.const_int(tokens.get(0).unwrap().get_number()?, false);
        let const_y = i64_type.const_int(tokens.get(2).unwrap().get_number()?, false);
        let ret = match operator.as_ref() {
            "+" => builder.build_int_add(const_x, const_y, "sum"),
            "*" => builder.build_int_mul(const_x, const_y, "sum"),
            _ => unimplemented!(),
        };
        builder.build_return(Some(&ret));
    } else {
        panic!("unexpected number of terms");
    }
    module
        .print_to_file(path::Path::new("compiled.ll"))
        .map_err(|err| From::from(err))
}
