extern crate rcc4;

fn main() {
    rcc4::compile("10".to_string());
    println!("{:?}", rcc4::run());
}
