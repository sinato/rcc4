extern crate rcc4;

fn main() {
    let code = "
int func(int a, int b) {
    return a + b;
}
int main() {
    return func(1, 2);
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
