mod front_of_house { // 模块名称为 front_of_house
  mod hosting { // 模块可以定义层级结构
    fn add_to_waitlist() {}
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
