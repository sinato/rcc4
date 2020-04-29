extern crate rcc4;

fn main() {
    rcc4::compile("1 + 2 * 3 + 4".to_string());
    println!("{:?}", rcc4::run());
}
