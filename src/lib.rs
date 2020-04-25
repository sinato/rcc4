extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

mod tokenize;

pub fn compile(code: String) -> Result<(), String> {
    // tokenize
    let tokens = tokenize::tokenize(code);

    // emit
    let context = Context::create();
    let module = context.create_module("rcc4");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let function = module.add_function("main", i64_type.fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    if tokens.len() == 1 {
        let const_x =
            i64_type.const_int(tokens.get(0).unwrap().clone().get_number().unwrap(), false);
        builder.build_return(Some(&const_x));
    } else if tokens.len() == 3 {
        let operator: String = tokens.get(1).unwrap().clone().get_operator().unwrap();
        let ret = match operator.as_ref() {
            "+" => {
                let const_x =
                    i64_type.const_int(tokens.get(0).unwrap().clone().get_number().unwrap(), false);
                let const_y =
                    i64_type.const_int(tokens.get(2).unwrap().clone().get_number().unwrap(), false);
                builder.build_int_add(const_x, const_y, "sum")
            }
            "*" => {
                let const_x =
                    i64_type.const_int(tokens.get(0).unwrap().clone().get_number().unwrap(), false);
                let const_y =
                    i64_type.const_int(tokens.get(2).unwrap().clone().get_number().unwrap(), false);
                builder.build_int_mul(const_x, const_y, "sum")
            }
            _ => unimplemented!(),
        };
        builder.build_return(Some(&ret));
    } else {
        panic!("unexpected number of terms");
    }
    module
        .print_to_file(path::Path::new("compiled.ll"))
        .map_err(|_| "hoge".to_string())
}

pub fn run() -> String {
    // run generated IR and get returned status code
    let status = process::Command::new("sh")
        .arg("-c")
        .arg("llvm-as-10 compiled.ll; lli-10 compiled.bc")
        .status()
        .expect("failed to execute process");
    status.to_string()
}

#[cfg(test)]
mod tests {

    use self::super::*;

    fn run_test(code: String, expect: &str) {
        let _ = compile(code);
        let actual = run();
        println!("{:?} => {:?}", actual, expect);
        assert_eq!(actual, String::from(format!("exit code: {}", expect)));
    }

    #[test]
    fn sigle_number() {
        run_test("10".to_string(), "10");
    }

    #[test]
    fn binary_add() {
        run_test("10+20".to_string(), "30");
    }

    #[test]
    fn binary_mul() {
        run_test("10*20".to_string(), "200");
    }
}
