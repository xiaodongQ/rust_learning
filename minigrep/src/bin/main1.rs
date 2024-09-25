use std::env;
use std::fs;

fn main() {
    // 通过 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    // dbg!(&args);

    // 暂只支持传入1个文件
    if args.len() != 3 {
        println!("usage: minigrep <query> <file_path>");
        return;
    }

    let query = &args[1];
    let file_path = &args[2];
    println!("cmd:{}, query:{}, file_path:{}", &args[0], query, file_path);

    // 通过std::fs模块的 read_to_string 读取文件内容
    // 返回结果为 std::io::Result<String>，对应于 Result<T, E>，T为String，E为Error
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(contents) => println!("{}", contents),
        Err(error) => println!("Problem opening the file: {:?}", error),
    }

    // 进行匹配
    

}