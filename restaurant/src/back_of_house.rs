// fn fix_incorrect_order() {
    // cook_order();
    // super::front_of_house::serving::serve_order();
// }

// fn cook_order() {}

// 構造体の各要素に関しては、それぞれで公開するか非公開にするかを選ぶことができる
pub struct Breakfast {
    pub toast: String,
    pub appetizer: Appetizer,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            appetizer: Appetizer::Soup,
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// enumに指定したヴァリアントはすべて公開される
pub enum Appetizer {
    Soup,
    Salad,
}
