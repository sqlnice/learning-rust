// 模块可以定义层级结构
pub mod hosting;
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
