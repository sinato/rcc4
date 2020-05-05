extern crate rcc4;

fn main() {
    let code = "
int func() {
    return 5;
}
int main() {
    return func();
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
