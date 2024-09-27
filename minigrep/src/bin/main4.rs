/// 优化1：解析传入参数抽取为函数
/// 进一步优化2：解析传入参数抽取为函数，并返回结构体
/// 进一步优化3：
///     面向对象编程，Config实例通过对象的方法来返回，而不是通过函数返回值
///     参数解析的错误处理，通过panic!宏来处理
/// 
use std::env;
use std::fs;

fn main() {
    // 模块化代码
    // 通过 env::args() 获取命令行参数，返回一个迭代器。而后用 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    // 相应地，此处从函数调整为使用结构体的方法来创建实例
    let config = Config::new(&args);
    println!("cmd:{}, query:{}, file_path:{}", &args[0], config.query, config.file_path);

    // 通过std::fs模块的 read_to_string 读取文件内容
    // unwrap 方法用于处理 Result 类型，如果 Result 类型是 Ok，则返回 Ok 中的值，否则程序会 panic
    let file_contents = fs::read_to_string(config.file_path).unwrap();
    println!("\n==============result:==============");
    for line in file_contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }
}

struct Config {
    query : String,
    file_path : String,
}

// 通过 impl 关键字来实现方法
impl Config {
    // 动态数组的不可变引用，可以用数组切片来代替
    // fn new(args : &Vec<String>) -> Config {
    fn new(args : &[String]) -> Config {
        if args.len() != 3 {
            panic!("args len:{} not valid!", args.len());
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{query, file_path}
    }
}
