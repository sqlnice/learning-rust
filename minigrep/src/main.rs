use minigrep::Config;
use std::{env, process};
fn main() {
    // 接收命令行参数
    let args: Vec<String> = env::args().collect();
    // env::args 函数返回一个迭代器
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
