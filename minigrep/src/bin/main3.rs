/// 优化1：解析传入参数抽取为函数
/// 进一步优化2：解析传入参数抽取为函数，并返回结构体
/// 
use std::env;
use std::fs;

fn main() {
    // 模块化代码
    // 通过 env::args() 获取命令行参数，返回一个迭代器。而后用 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    let config = parse_args(&args);
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

// 传入不可变引用
// 返回切片元组，2个元素，第一个为查询字符串，第二个为文件名
fn parse_args(args : &Vec<String>) -> Config {
    // 这里新创建一个String，拥有单独的所有权
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{query, file_path}
}