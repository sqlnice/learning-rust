use core::num;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Read},
};
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
    method_syntax()
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
