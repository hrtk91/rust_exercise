fn main() {
    // 🌱 Exercise 1: 関数をつないで変換する
    // お題：
    // 文字列を受け取り、以下の処理を関数でつなげて1本にまとめてください：
    // 前後の空白を削除する
    // 大文字に変換する
    // "!" を末尾に追加する
    fn composer(fs: Vec<fn(&str) -> String>) -> impl FnOnce(&str) -> String {
        return move |s: &str| {
            let mut iter = fs.iter();
            let mut result = s.to_string();
            while let Some(f) = iter.next() {
                result = f(result.as_str());
            }
            result
        };
    }

    let shout = composer(vec![trim, to_uppercase, add_exclamation]);
    let resp = shout(" fas lo dar ");
    println!("{}", resp);
}

fn trim(s: &str) -> String {
    s.trim().to_string()
}

fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn add_exclamation(s: &str) -> String {
    format!("{}!", s)
}
