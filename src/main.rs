extern crate rcc4;

fn main() {
    let code = "
int main() {
    int a;
    a;
    return 1;
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
