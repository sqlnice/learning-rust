mod front_of_house {
    // 模块名称为 front_of_house
    pub mod hosting {
        // 模块可以定义层级结构
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
// 模块树的结构
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

fn deliver_order() {}
mod back_of_house {
    // 将枚举设为公有，则它的所有成员都将变为公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
    // 创建公有的结构体和枚举
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peached"),
            }
        }
    }
}

fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
}
