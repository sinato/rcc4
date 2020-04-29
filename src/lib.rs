extern crate inkwell;

use inkwell::context::Context;
use std::process;

pub mod emit;
pub mod parse;
pub mod tokenize;

pub fn compile(code: String) {
    // tokenize
    let tokens = tokenize::tokenize(code);
    println!("tokens: {:?}", tokens);

    // parse
    let node = *parse::parse(tokens);
    println!("node: {:?}", node);

    // emit
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("my_module");
    match emit::Emitter::emit(&context, &builder, &module, node) {
        Ok(_) => (),
        Err(e) => panic!(format!("{}", e)),
    }
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

    fn run_test(code: &str, expect: &str) {
        compile(code.to_owned());
        let actual = run();
        println!("{:?} => {:?}", actual, expect);
        assert_eq!(actual, String::from(format!("exit code: {}", expect)));
    }

    #[test]
    fn sigle_number() {
        let code = "
        int main() {
            10
        }
        ";
        run_test(code, "10");
    }

    #[test]
    fn binary_add() {
        let code = "
        int main() {
            10+20
        }
        ";
        run_test(code, "30");
    }

    #[test]
    fn multi_add() {
        let code = "
        int main() {
            10+20+30
        }
        ";
        run_test(code, "60");
    }

    #[test]
    fn binary_mul() {
        let code = "
        int main() {
            10*20
        }
        ";
        run_test(code, "200");
    }

    #[test]
    fn multi_mul() {
        let code = "
        int main() {
            2*3*4
        }
        ";
        run_test(code, "24");
    }

    #[test]
    fn multi_add_mul() {
        let code = "
        int main() {
            1 + 2 * 3 + 4
        }
        ";
        run_test(code, "11");
    }
}
