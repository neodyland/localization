use localization::t;

fn main() {
    let name = "John";
    let age = 42;
    let s = t!("ja-JP","default:hello", name, age);
    println!("{}", s);
}