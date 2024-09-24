use std::env;

fn main() {
    // 通过 collect 方法输出一个集合类型 Vector
    let args : Vec<String> = env::args().collect();
    // dbg!(&args);

    // 暂只支持传入1个文件
    if args.len() != 3 {
        println!("usage: minigrep <query> <filename>");
        return;
    }

    let query = &args[1];
    let filename = &args[2];
    println!("query:{}, filename:{}", query, filename);

    // 通过std::fs模块的 read_to_string 读取文件内容
    // 返回结果为 std::io::Result<String>，对应于 Result<T, E>，T为String，E为Error
    let contents = std::fs::read_to_string(filename);
    match contents {
        Ok(contents) => println!("{}", contents),
        Err(error) => println!("Problem opening the file: {:?}", error),
    }
    

}