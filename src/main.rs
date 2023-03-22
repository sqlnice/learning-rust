use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    // guess_number()
    // variables_and_mutability()
    // data_types()
    another_function(5)
}

// 2 猜数字游戏
fn guess_number() {
    println!("猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入你猜测的数字");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取错误");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数字是：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小啦"),
            Ordering::Greater => println!("太大啦"),
            Ordering::Equal => {
                println!("你赢啦");
                break;
            }
        }
    }
}

// 3 常见编程概念
// 3.1 变量与可变性
fn variables_and_mutability() {
    // 可变性
    // 通过 mut 关键字，使其可变
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");

    // 常量
    // 不可以是只能在运行时计算出的值
    // 命名全大写加下划线
    const THREE_HOUSE_IN_SECONDS: u32 = 60 * 60 * 3;

    // 隐藏（Shadowing）
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("在内部代码块中x为:{x}")
    }
    println!("在外部代码块中x为:{x}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("spaces的长度为:{spaces}");
}
// 3.2 数据类型
fn data_types() {
    // 标量类型（scalar）代表一个单独的值 整形、浮点型、布尔类型、字符类型

    // - 整形 无小数部分 默认 i32
    // - 浮点型 f32 f64；默认 f64； 所有浮点型都是有符号的；浮点数采用IEEE-754标准
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x}{y}");
    // - 数值运算 支持加减乘除和取余；整数除法会向下舍入到最接近的整数
    // - 字符类型
    // 使用 '' 声明 char 字面量；使用 "" 声明字符串字面量；
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // 复合类型（Compound types） 可以将多个值组合成一个类型
    // - 元祖类型 一旦声明，长度不会变化
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 使用模式匹配来解构
    let (x, y, z) = tup;
    println!("{x}{y}{z}");
    // 使用 . 直接访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // - 数组类型 每个元素必须相同；长度固定
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5]; // 生成 [3, 3, 3, 3, 3]
    let first = a[0]; // 使用索引访问
}
// 3.3 函数
fn another_function(x: i32) {
    // 参数必须指定类型;意味着编译器再也不需要你在代码的其他地方注明类型来指出你的意图。而且，在知道函数需要什么类型后，编译器就能够给出更有用的错误消息
    println!("{x}");

    // 语句和表达式
    // Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。
    // 语句 - 执行一些操作但不返回值的指令
    let y = 6;
    // 表达式 - 计算并产生一个值。
    // 1.上面例子中 6 是表达式，意为返回一个 6
    // 2.函数调用
    // 3.宏调用
    // 4.用大括号创建的新的块作用域也是表达式：
    let y = {
        let x = 3;
        x + 1 // 表达式的结尾没有分号
    };
    println!("The value of y is: {y}");

    // 具有返回值的函数;需要在 -> 后声明它的类型
    fn five() -> i32 {
        5
    }
    five();
}
