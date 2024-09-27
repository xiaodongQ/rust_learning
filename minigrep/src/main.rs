/// 优化1：解析传入参数抽取为函数
/// 进一步优化2：解析传入参数抽取为函数，并返回结构体
/// 进一步优化3：
///     面向对象编程，Config实例通过对象的方法来返回，而不是通过函数返回值
///     参数解析的错误处理，通过panic!宏来处理
/// 进一步优化4：使用Result<T, E>来处理错误；方法名调整为`build`（语义更合适），并通过`闭包`处理错误
/// 进一步优化5：分离main里的业务逻辑
/// 进一步优化6：分离业务逻辑到库包lib.rs中，并在main.rs里use引入；同时业务逻辑 run 中的匹配部分，继续抽取为 search 函数
/// 
use std::env;

use minigrep::Config;

fn main() {
    // 模块化代码
    // 通过 env::args() 获取命令行参数，返回一个迭代器。而后用 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    // 此处 unwrap_or_else 是Result实现的方法，使用闭包来处理错误
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // 标准库，处理进程退出
        std::process::exit(1);
    });
    println!("cmd:{}, query:{}, file_path:{}", &args[0], config.query, config.file_path);

    // 匹配业务逻辑
    // 用 if...let语法替换上一个文件中的match语法，更为简洁
    if let Err(err) = minigrep::run(config) {
        println!("run error: {}", err);
        std::process::exit(1);
    }
}

