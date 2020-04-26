extern crate rcc4;

fn main() {
    rcc4::compile("2*3+5".to_string());
    println!("{:?}", rcc4::run());
}
