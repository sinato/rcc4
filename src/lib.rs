extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn tokenize(code: String) -> u64 {
    let mut chars = code.chars().peekable();

    let mut s = String::from("");
    while let Some(c) = chars.peek() {
        s += &c.to_string();
        chars.next();
    }
    let num: u64 = s.parse().unwrap_or_else(|err| panic!(err));
    num
}

pub fn compile(code: String) {
    // tokenize
    let num = tokenize(code);

    // emit
    let context = Context::create();
    let module = context.create_module("rcc4");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let function = module.add_function("main", i64_type.fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    let const_x = i64_type.const_int(num, false);
    builder.build_return(Some(&const_x));
    let _ = module.print_to_file(path::Path::new("compiled.ll"));
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
        compile(code);
        let actual = run();
        println!("{:?} => {:?}", actual, expect);
        assert_eq!(actual, String::from(format!("exit code: {}", expect)));
    }

    #[test]
    fn sigle_number() {
        run_test("10".to_string(), "10");
    }
}
