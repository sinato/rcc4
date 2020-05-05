extern crate rcc4;

fn main() {
    let code = "
int main() {
    int a; a = 10;
    int b; b = 3;
    return a + b * 2;
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
