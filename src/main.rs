extern crate rcc4;

fn main() {
    let code = "
int main() {
    int a;
    1;
    return 1 + 2 * 3 + 4;
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
