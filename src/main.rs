use crate::garden::vegetables::Asparagus;
use core::num;
use rand::Rng;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io::{self, ErrorKind, Read},
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
    recoverable_errors_with_result()
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
