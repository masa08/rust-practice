enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("NONE");
            None
        },
        Some(i) => {
            println!("{}", i+1);
            Some(i + 1)
        },
    }
}

fn main() {
    let coin = Coin::Nickel;
    let res = value_in_cents(coin);
    println!("{}", res);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

