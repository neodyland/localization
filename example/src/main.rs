use localization::t;
use localization::all;

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
    let s = t!(data.lang, "default:hello", name, age = data.age);
    println!("{}", s);
    // こんにちはJohnさん、42歳ですね！
    let all = all!();
    println!("{:?}", all);
    // {"default:hello": {"en-US": "Hello {{name}}, you are {{age}} years old!", "ja-JP": "こんにちは{{name}}さん、{{age}}歳ですね！"}}
}
