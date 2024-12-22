mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorret_order() {
        cook_order();
        // 使用 super 提升到父级
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        /*
        因为 back_of_house::Breakfast 具有私有字段，所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例 (这里我们命名为 summer)。
        如果 Breakfast 没有这样的函数，我们将无法在 eat_at_restaurant 中创建 Breakfast 实例
        */
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 使用 use 关键字绝对路径
// use crate::front_of_house::hosting;
// 使用 use 关键字相对路径
use front_of_house::hosting;
// 使用 重导出 技术，让外部代码使用hosting
// pub use crate::front_of_house::hosting;

// 嵌套路径 use
// use std::io::{self, Write};
// use std::{cmp::Ordering, io};

// 使用 glob 运算符将一个路径下所有的项引入作用域
// use std::collections::*;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // use 关键字引入模块
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1 = {:?}, order2 = {:?}", order1, order2);
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
