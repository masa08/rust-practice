use std::io;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん
    println!("Please input your guess.");   // ほら、予想を入力してね

    // MEMO
    // let hoge     => immutable
    // let mut hoge => mutable
    let mut guess = String::new();

    // MEMO
    // Result型 => ok or errorを返却する
    // Expectはokであれば値を返却し
    // errorであればerror messageを返却する
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように予想しました: {}

}
