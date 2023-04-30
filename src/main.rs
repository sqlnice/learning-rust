use crate::garden::vegetables::Asparagus;
use core::num;
use rand::Rng;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display},
    fs::{self, File},
    hash::Hash,
    io::{self, ErrorKind, Read},
    ops::Deref,
    thread,
}; // use 用来将路径引入作用域
pub mod garden; // 告诉编译器应该包含在src/garden.rs文件中发现的代码

fn main() {
    // guess_number()
    // variables_and_mutability()
    // data_types()
    // another_function(5)
    // control_flow()
    // ownership()
    // references_and_borrowing()
    // slices()
    // defining_structs()
    // example_structs()
    // method_syntax()
    // defining_an_enum()
    // match_flow()
    // if_let()
    // defining_modules_to_control_scope_and_privacy()
    // paths_for_referring_to_an_item_in_the_module_tree()
    // vectors()
    // strings()
    // hash_maps()
    // unrecoverable_errors_with_panic()
    // recoverable_errors_with_result()
    // traits()
    // lifetime_syntax()
    // closures()
    // iterators()
    re_box()
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
// 3.4 控制流
fn control_flow() {
    // if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if 是表达式，所以：
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 分支返回的类型必须相同

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }

    // 生成 n 阶斐波那契数列
    // 递归; 会生成大量的重复代码比较慢
    fn fib_1(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib_1(n - 1) + fib_1(n - 2),
        }
    }
    println!("fib_1 Result:{}", fib_1(9));
    // 循环; 每次保存计算的结果，空间换时间
    fn fib_2(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut num1 = 0;
        let mut num2 = 1;
        for _ in 1..n {
            let temp = num1 + num2;
            num1 = num2;
            num2 = temp;
        }
        num2
    }
    println!("fib_2 Result:{}", fib_2(9));
}
// 4 所有权
// 所有权是Rust独有的特性，他让rust无需垃圾回收即可保障内存安全。通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了这些规则，程序不能编译。在运行时，所有权系统的任何功能都不会减慢程序。
// 所有权规则
// - Rust 中每一个值都有一个所有者
// - 值在任意时刻有且只有一个所有者
// - 当所有者离开作用域后，这个值将被丢弃
fn ownership() {
    {
        // s 在此处无效，尚未声明
        let s = "hello"; // s 有效
        println!("{s}");
    } // s 无效，作用域结束

    // 变量与数据交互的方式 - 移动
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); 此处访问 s1 无效
    // 为了确保内存安全，在 let s2 = s1 之后 rust认为 s1 不再有效
    // 可以理解为当 s2 = s1 之后，s1 的指针、长度、容量都被移动到 s2 中了，不是“浅拷贝”。这个操作成为移动

    // 变量与数据交互的方式 - 克隆
    // 可以理解为深拷贝，可能更加消耗资源
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 所有权与函数; 将值传递给函数可能会移动或复制
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以s到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动到函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
    fn takes_ownership(some_string: String) {
        // some_string 进入作用域
        println!("{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。
      // 占用的内存被释放
    fn makes_copy(some_integer: i32) {
        // some_integer 进入作用域
        println!("{}", some_integer);
    }
}
// 4.2 引用与借用; & 符号就是引用，允许你使用值但不获取所有权
// 总结引用的规则：
// - 在任意时间，要么只能有一个可变引用，要么只能有多个不可变引用
// - 引用必须总是有效的
fn references_and_borrowing() {
    println!("引用与借用");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of {} is {}", s1, len);
    fn calculate_length(s: &String) -> usize {
        s.len()
    } // 在这里 s 离开作用域，但他是引用，并不拥有所有权，所以不会销毁
      // 我们将创建一个引用的行为称为借用

    // 可变引用; 不能存在对一个变量有多个可变的引用，会引起数据竞争，因为你不知道是谁改变的
    let mut s = String::from("hello");
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // 悬垂引用；是其指向的内存可能已经被分配给其它持有者
    let reference_to_nothing = no_dangle();
    // fn dangle() -> &String {
    //     // dangle 返回一个字符串的引用
    //     let s = String::from("hello"); // s 是一个新字符串
    //     &s // 返回字符串 s 的引用
    // } // 这里 s 离开作用域并被丢弃。其内存被释放。
    // 解决方法是直接返回 String：
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
}
// 4.3 切片 slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
fn slices() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{word}");
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}
// 所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。
// 5.1 定义结构体
fn defining_structs() {
    struct User {
        active: bool,
        username: String,
    }
    let mut user1 = User {
        active: true,
        username: String::from("sql"),
    };
    let username = user1.username;
    println!("{username}");
    user1.username = String::from("sql2");
    println!("{}", user1.username);

    fn build_user(username: String) -> User {
        User {
            active: true,
            username,
        }
    }
    let user2 = build_user(String::from("sql3"));
    println!("{}", user2.username);
    // 解构 结构更新语法就像带有 = 的赋值，因为它移动了数据
    let user3: User = User {
        username: String::from("sql4"),
        ..user1
    };
    println!("{}", user1.active);

    // 元祖结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // 类单元结构体;类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
// 5.2 结构体示例程序
fn example_structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
// 5.3 结构体的方法语法
fn method_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(self: &Self) -> u32 {
            self.width * self.height
        }
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    let square = Rectangle::square(10);
    println!("square: {:?}", square);
}
// 结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。

// 6.1 枚举的定义
fn defining_an_enum() {
    enum IpAddrkind {
        V4(String),
        V6(String),
    }
    let four = IpAddrkind::V4(String::from("127.0.0.1"));
    let six = IpAddrkind::V6(String::from("::1"));
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 枚举与结构体对比
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 元祖结构体
    struct ChangeColorMessage(i32, i32, i32); // 元祖结构体
                                              // 如果使用不同的结构体，由于他们的类型不同，我们将不能轻易定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型
                                              // 相似点：可以使用impl来为枚举定义方法
    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 是标准库中定义的另一个枚举，应用于 一个值要么有值要么没值
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
}
// 6.2 match 控制流结构
fn match_flow() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
    let dime: Coin = Coin::Dime;
    let result = value_in_cents(dime);
    println!("{result}");

    // 绑定值的模式
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    let Alaska = Coin::Quarter(UsState::Alaska);
    let result = value_in_cents(Alaska);
    println!("{result}");

    // 匹配 Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 匹配是穷尽的;分支必须覆盖所有的可能性

    // 通配模式和_占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 必须将通配分支放在最后;当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值
                       // _ => (), // 返回空元祖;在这里，我们明确告诉 Rust 我们不会使用与前面模式不匹配的值，并且这种情况下我们不想运行任何代码
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
// 6.3 if let 简洁控制流
fn if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // 可以使用 if let 这种更短的方式编写
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // if let else
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}

// 7.1 包和 Crate
// crate 是 Rust 在编译时最小的代码单位
// - 二进制项 可以被编译为可执行程序,必须有 main 函数来定义程序要做的事情
// - 库      没有 main 函数,不会被编译为可执行程序.大部分是指提供函数之类的库,可以理解为 npm 包?

// 包 是提供一系列功能的一个或多个 crate
// 最多存在一个库;只少有一个 crate(无论是库还是二进制项)

// 在此，我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。

// 7.2 定义模块来控制作用域与私有性
fn defining_modules_to_control_scope_and_privacy() {
    let plant = Asparagus {};
    println!("im growing {:?}!", plant);
}
// 7.3 引用模块项目的路径
fn paths_for_referring_to_an_item_in_the_module_tree() {
    // - 绝对路径；以crate根为开头的全路径
    // - 相对路径；从当前模块开始，以self、super或当前模块的标识符开头
}
// 7.4 使用use关键字将路径引入作用域
// 7.5 将模块拆分成多个文件
// Rust 提供了将包分成多个crate，将crate分成模块，以及通过指定绝对/相对路径从一个模块引用另一个模块中定义的项的方式。
// 也可以使用use语句将路径引入作用域。

// 集合
// 8.1 使用 Vector 储存列表
// 在内存中彼此相邻排列所有的值，只能存储相同类型的值
fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3]; // 编译器类型推断

    // 更新
    v.push(4);
    v.push(5);
    // 读取
    let third = &v[2];
    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // 遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * 为解引用运算符
        println!("{i}");
    }
    // 使用枚举来储存多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
// 8.2 使用字符串储存 UTF-8 编码的文本
// 字符串是作为字节的集合外加一些方法实现的
fn strings() {
    // Rust 倾向于确保暴露出可能会出现的错误,字符串是很复杂的数据结构
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    s.push_str("bar");
    s.push('l'); // 添加单个字母
                 // 字符串相加 - 1
    let s1 = String::from("Hello, ");
    let s2 = "world!".to_string();
    // let s3 = s1 + &s2;
    // + 相当于调用一下函数
    // fn add(self, s: &str) -> String
    // println!("{s1}{s2}{s3}"); // s1 失效
    // 字符串相加 - 2
    let s = format!("{s1}-{s2}");

    // 索引字符串;不能直接访问的原因是每个 Unicode 标量值需要两个字节存储,并且有的字虽然保存但是没有意义,最后索引操作预期总是O(1),但是对于 String 不可能保证这样
    let s1 = String::from("hello");
    // let h = s1[0];

    // 字符串 slice
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Зд 每个字母长度为 2 字节
                          // print!("{s}");

    // 遍历字符串
    for c in "Зд".chars() {
        println!("{c}");
        // 3
        // д
    }
    for b in "3д".bytes() {
        println!("{b}");
        // 51
        // 208
        // 180
    }
}

// 8.3 使用 Hash Map 储存键值对
fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); // key value 必须同类型

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // 调用 copied 获取 Option<i32>,调用 unwrap_or 在 score 中没有该键所对应项时设为 0
    println!("{}", score);
    // 访问
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // 所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。
    println!("field_value");

    // 更新
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // 只在键没有对应值时插入键值对;or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // * 解引用
    }
    println!("{:?}", map);
}

// 9 错误处理;Rust 将错误分类为: 可恢复的 和 不可恢复的
// 9.1 用 panic! 处理不可恢复的错误
fn unrecoverable_errors_with_panic() {
    // panic!("crash and burn")
    // let v = vec![1, 2, 3];
    // v[99];
}
// 9.2 用 Result 处理可恢复的错误
fn recoverable_errors_with_result() {
    let greeting_file_result = File::open("hello.txt");
    // 什么b嵌套
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.tet") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    // 使用闭包和 unwrap_or_else 改善
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 失败时 panic 的简写：unwrap 和 expect
    // 失败时 unwrap 会帮我们调用 panic
    let greeting_file = File::open("hello.txt").unwrap();
    // 生产级别的代码中，大部分 Rustaceans 选择 expect
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // 传播错误
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");
    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e), // 不在函数的最后,所以需要 return
    //     };
    //     let mut username = String::new();
    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }
    // 传播错误的简写 ? 运算符
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }
    // ? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接使用链式方法调用来进一步缩短代码
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut username)?;
    //     Ok(username)
    // }
    // 更简短
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    // ? 适用于 Option
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}
// 9.3 要不要 panic !
fn to_panic_or_not_to_panic() {
    // 返回 Result 是定义可能会失败的函数的一个默认选择

    // 实例/代码原型和测试适合 panic
}

// 10 泛型、Trait和生命周期
// 10.1 泛型数据类型
fn syntax() {
    // 函数定义中使用泛型
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            // if item > largest {
            //     largest = item;
            // }
        }
        largest
    }
    let number_list = vec![1, 2, 3, 4, 5, 9, 7];
    let result = largest(&number_list);
    println!("{result}");
    let char_list = vec!['y', 'n', 'm'];
    let result = largest(&char_list);
    println!("{result}");

    // 结构体定义中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // 枚举定义中的泛型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 泛型代码的性能;Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
}
// 10.2 Trait: 定义共同行为
fn traits() {
    // trait 定义了某个特定类型拥有可能与其他类型共享的功能.
    // 定义 trait
    pub trait Summary {
        fn summarize(&self) -> String; // 一行一个方法签名且都以分号结尾
    }

    // 为类型定义 trait
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}, ({})", self.headline, self.author, self.location)
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}:{}", self.username, self.content)
        }
    }
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet:{}", tweet.summarize());

    // 默认实现
    // pub trait Summary {
    //     fn summarize(&self) -> String {
    //         String::from("(Read more...)")
    //     }
    // };

    // trait 作为参数
    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // };

    // Trait Bound 语法 简化书写,实际是语法糖
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {}
    // 通过 + 指定多个 trait bound
    // pub fn notify(item: &(impl Summary + Display)) {}
    pub fn notify<T: Summary + Display>(item: &T) {}
    // 通过 where 简化 trait bound
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {};
    fn some_function<T, U>(t: &T, u: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }
    // 返回实现了 trait 的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        }
    }
    // 使用 trait bound 有条件的实现方法
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    // trait 和 trait bound 让我们使用泛型类型参数来减少重复,并仍然能够向编译器明确指定泛型类型需要拥有哪些行为.因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。在动态类型语言中，如果我们尝试调用一个类型并没有实现的方法，会在运行时出现错误。Rust 将这些错误移动到了编译时，甚至在代码能够运行之前就强迫我们修复错误。另外，我们也无需编写运行时检查行为的代码，因为在编译时就已经检查过了，这样相比其他那些不愿放弃泛型灵活性的语言有更好的性能。
}
// 10.3 使用生命周期来确保引用有效
fn lifetime_syntax() {
    // 大部分生命周期是隐含并可以推断的;
    // 生命周期避免了悬垂引用
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {r}"); // 此时 r 引用的就是 x 最后被释放的内存

    // 借用检查器 borrow checker; 它比较作用域来确保所有的借用都是有效的

    // 函数中的泛型生命周期
    // 泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // let string1 = String::from("abcd");
    // {
    //     let string2 = "xyz";
    //     let result = longest(string1.as_str(), string2);
    //     println!("The longest string is {}", result);
    // }

    // 指定返回参数的生命周期
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为

    // 结构体定义中的生命周期注解
    // 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // 生命周期省略
    // 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。这并不是需要程序员遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。
    // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。
    // 编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
    // 第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数就有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
    // 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
    // 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。只能适用于 方法签名

    // 方法定义中的生命周期注解
    // 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 静态生命周期; 所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime";
    // 大部分情况中，推荐 'static 生命周期的错误信息都是尝试创建一个悬垂引用或者可用的生命周期不匹配的结果。在这种情况下的解决方案是修复这些问题而不是指定一个 'static 的生命周期。

    // 结合泛型类型参数/trait bounds 和生命周期
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
// 11 测试
fn wirting_tests() {
    // 测试函数执行三种操作
    // 1.设置任何所需的数据或状态
    // 2.运行需要测试的代码
    // 3.断言其结果是所期望的
    // 提供的宏 assert! assert_eq! assert_ne!

    // 自定义失败信息
    // assert!(
    //     result.contains("Carol"),
    //     "Greeting did not contain name, value was `{}`",
    //     result
    // );
}
// 11.2 控制测试如何运行
fn running_tests() {}

// 13.1 闭包; 可以保存在一个变量中或作为参数传递给其他函数的匿名函数
fn closures() {
    // 闭包会捕获其环境
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_perference: Option<ShirtColor>) -> ShirtColor {
            // 将被闭包表达式 || self.most_stocked() 用作 unwrap_or_else 的参数
            // unwrap_or_else 的实现会在之后需要其结果的时候执行闭包
            user_perference.unwrap_or_else(|| self.most_stocked())
        }
        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;
            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with perference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // 闭包类型推断和注解
    let example_closure = |x| x; // 默认推断
    let s = example_closure("Hello".to_string());
    // let n = example_closure(5); // 第一次使用 String 值调用 example_closure 时，编译器推断这个闭包中 x 的类型以及返回值的类型是 String。接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则就会得到类型错误。

    // 捕获引用或者移动所有权;闭包可以通过三种方式捕获其环境
    // 1. 不可变借用
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    // 2. 可变借用
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // 在闭包使用之前 不可调用 list,因为所有权在 borrows_mutably 里
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    // 3. 获取所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // 将被捕获的值移出闭包和 Fn trait
    // 闭包捕获和处理环境中的值的方式影响闭包实现的 trait。Trait 是函数和结构体指定它们能用的闭包的类型的方式。取决于闭包体如何处理值，闭包自动、渐进地实现一个、两个或三个 Fn trait。
    // FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
    // FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。
    // Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。
}

// 13.2 使用迭代器处理元素序列
fn iterators() {
    // 迭代器是 惰性的
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 在使用之前没啥用
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterator trait 和 next 方法; 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait
    pub trait Iterator {
        type Item; // Item 类型将是迭代器返回元素的类型
        fn next(&mut self) -> Option<Self::Item>;
        // 此处省略了方法的默认实现
    }
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // 消费迭代器的方法
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
        assert_eq!(total, 6);
    }

    // 产生其他迭代器的方法
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // 使用捕获其环境的闭包
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        return shoes.into_iter().filter(|s| s.size == shoe_size).collect();
    }
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //     #[test]
    //     fn filters_by_size() {
    //         let shoes = vec![
    //             Shoe {
    //                 size: 10,
    //                 style: String::from("sneaker"),
    //             },
    //             Shoe {
    //                 size: 13,
    //                 style: String::from("sandal"),
    //             },
    //             Shoe {
    //                 size: 10,
    //                 style: String::from("boot"),
    //             },
    //         ];
    //         let in_my_size = shoes_in_size(shoes, 10);
    //         assert_eq!(
    //             in_my_size,
    //             vec![
    //                 Shoe {
    //                     size: 10,
    //                     style: String::from("sneaker")
    //                 },
    //                 Shoe {
    //                     size: 10,
    //                     style: String::from("boot")
    //                 },
    //             ]
    //         );
    //     }
    // }
}

// 13.4 性能对比: 循环 VS 迭代器
fn performance() {
    // 迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销
}

// 14.1 采用发布配置自定义构建
fn release_profiles() {
    // Cargo 有两个主要的配置：运行 cargo build 时采用的 dev 配置和运行 cargo build --release 的 release 配置。dev 配置被定义为开发时的好的默认配置，release 配置则有着良好的发布构建的默认配置。
}

// 14.3 Cargo 工作空间
fn cargo_workspace() {
    // 工作空间是一系列共享同样的 Cargo.lock 和输出目录的包
}

// 15 智能指针
// 指针是一个包含内存地址的变量的通用概念.Rust 中最常见的指针是 引用, 以 & 符号为标志并借用所指向的值.
// 另一方面, 智能指针是一类数据结构, 表现类似指针. 但是拥有额外的元数据和功能.
// 智能指针通常使用结构体实现. 不同于结构体的地方在于其实现了 Deref 和 Drop trait. Deref trait 允许智能智能结构体实例表现得像引用一样, 既可以引用又用于智能指针. Drop trait 允许我们自定义当智能指针离开作用域时运行的代码.
// 15.1 使用 Box<T> 指向堆上数据
fn re_box() {
    // Box 允许将一个值放在堆上, 留在栈上的则是指针.
    // 应用场景:
    // 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
    // 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    // 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

    // 使用
    let b = Box::new(5);
    println!("b = {}", b);

    // Box 允许创建递归类型
    // Rust 需要在编译时知道类型占用多少空间. 递归理论上可以无限进行下去.
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // 通过 Box 可解决位置空间问题, 因为 Box 可以提供引用地址的大小
    // Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。这两个 trait 对于在本章余下讨论的其他智能指针所提供的功能中，将会更为重要。让我们更详细的探索一下这两个 trait。
}
// 15.2 通过 Deref trait 将智能指针当做常规引用处理
fn deref() {
    // 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*。通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

    // 常规引用
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); // 会报错 不允许比较数字的引用与数字，因为它们是不同的类型。
    assert_eq!(5, *y); // 通过 * 来追踪引用者所指向的值, 也就是解引用

    // Box
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 上面两种, 有何不同?

    // 自定义智能指针
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T; // type Target = T; 语法定义了用于此 trait 的关联类型
        fn deref(&self) -> &Self::Target {
            &self.0 // deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用。回忆一下第五章 “使用没有命名字段的元组结构体来创建不同的类型” 部分 .0 用来访问元组结构体的第一个元素
        }
    }
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 编译器在底层运行 *(y.deref())

    // 函数和方法的隐式 Deref 强制转换, 可以理解为 JS 中的 Boolean('1')
}
