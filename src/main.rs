extern crate rcc4;

fn main() {
    let code = "
int main() {
    return 1 + 2 * 3 + 4;
}"
    .to_owned();
    rcc4::compile(code);
    println!("{:?}", rcc4::run());
}
