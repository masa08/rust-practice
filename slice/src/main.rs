fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // first_word関数に対して、sを参照する引数ないしは戻り値を記述したので
    // s.clearすると参照が失われることから、エラーが発生する
    // s.clear(); // error! （エラー！）

    println!("the first word is: {}", word);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


