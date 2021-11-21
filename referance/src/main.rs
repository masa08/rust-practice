fn main() {
    let s1 = String::from("hello");

    // 実体ではなく、s1の参照を引数として渡す
    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s)
}

// 関数の引数に参照を取ることを借用と呼ぶ
// 参照は不変なので、渡されたオブジェクトを変更することはできない
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ダングリング参照
// 参照を返すが参照されているオブジェクトがスコープを抜けてしまい、所有権が破棄される状態
// エラーが起きる
