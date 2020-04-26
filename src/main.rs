extern crate rcc4;

fn main() {
    match rcc4::compile::compile("10*3".to_string()) {
        Ok(_) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    println!("{:?}", rcc4::run());
}
