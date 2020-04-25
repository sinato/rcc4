extern crate rcc4;

fn main() {
    match rcc4::compile("10".to_string()) {
        Ok(_) => (),
        Err(err) => panic!(err),
    }
    println!("{:?}", rcc4::run());
}
