fn main() {
    let number = 3;

    // RubyやJavaScriptなどの言語とは異なり、Rustでは、論理値以外の値が、自動的に論理値に変換されることはありません。
    if number < 5 {
        println!("condition was true");       // 条件は真でした
    } else {
        println!("condition was false");      // 条件は偽でした
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
