use localization::t;

struct Data {
    pub lang: String,
    pub age: i32,
}

fn main() {
    let name = "John";
    let data = Data {
        lang: "ja-JP".to_string(),
        age: 42,
    };
    let s = t!(&data.lang.as_str(), "default:hello", name, age = data.age);
    println!("{}", s);
}
