extern crate inkwell;

use std::process;

pub mod compile;
pub mod tokenize;

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
        let _ = compile::compile(code);
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
