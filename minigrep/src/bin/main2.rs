/// 优化1：解析传入参数抽取为函数；匹配逻辑由 match 调整为 unwrap() 处理
/// 
use std::env;
// 下面read_to_string指定作用域则不需要use std::fs
// use std::fs;

fn main() {
    // 模块化代码
    // 通过 env::args() 获取命令行参数，返回一个迭代器。而后用 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    let (query, file_path) = parse_args(&args);
    if query.len() == 0 || file_path.len() == 0 {
        eprintln!("query:{} or file_path:{} is empty", query, file_path);
        return ;
    }
    println!("cmd:{}, query:{}, file_path:{}", &args[0], query, file_path);

    // 通过std::fs模块的 read_to_string 读取文件内容
    // unwrap 方法用于处理 Result 类型，如果 Result 类型是 Ok，则返回 Ok 中的值，否则程序会 panic
    let file_contents = std::fs::read_to_string(file_path).unwrap();
    println!("\n==============result:==============");
    for line in file_contents.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}


// 传入不可变引用
// 返回切片元组，2个元素，第一个为查询字符串，第二个为文件名
fn parse_args(args : &Vec<String>) -> (&str, &str) {
    // 暂只支持传入1个文件
    if args.len() != 3 {
        println!("usage: minigrep <query> <file_path>");
        return ("", "");
    }

    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
    // 下面语句 和 上条表达式 效果相同
    // return (query, file_path);
}