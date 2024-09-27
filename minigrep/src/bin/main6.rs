/// 优化1：解析传入参数抽取为函数
/// 进一步优化2：解析传入参数抽取为函数，并返回结构体
/// 进一步优化3：
///     面向对象编程，Config实例通过对象的方法来返回，而不是通过函数返回值
///     参数解析的错误处理，通过panic!宏来处理
/// 进一步优化4：使用Result<T, E>来处理错误；方法名调整为`build`（语义更合适），并通过`闭包`处理错误
/// 进一步优化5：分离main里的业务逻辑，抽取为 run 函数
///    
/// 
use std::env;
use std::fs;

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
    let ret = run(config);
    match ret {
        // Ok(_) => println!("run success!"),
        Ok(_) => (),
        Err(err) => {
            println!("run error: {}", err);
            std::process::exit(1);
        }
    }
    // 或者直接用 if let Err(e) = run(config) { println!("Application error: {e}"); process::exit(1); } 来处理错误，更为简洁
}

struct Config {
    query : String,
    file_path : String,
}

// 通过 impl 关键字来实现方法
impl Config {
    // 从代码惯例的角度出发，new 往往不会失败，修改函数名为 build
    // 传入参数：动态数组的不可变引用，可以用数组切片来代替
    // 通过Result<T, E>返回错误， 指定 &str 的生命周期为 'static
    fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("args length invalid");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

// std::error::Error 是Rust标准库的一个 trait，定义了错误处理的行为
// dyn 表示动态分派，是Rust中的一种动态分派机制
fn run(config : Config) -> Result<(), Box<dyn std::error::Error>> {
    // 通过std::fs模块的 read_to_string 读取文件内容，其返回结果为 std::io::Result<String>，对应于 Result<T, E>，T为String，E为Error
    // `?`操作符会返回`Result`类型，如果`Ok`则返回`Ok`中的值，如果`Err`则返回`Err`中的值
    let file_contents = fs::read_to_string(config.file_path)?;
    println!("\n==============result:==============");
    
    for line in file_contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }
    Ok(())
}