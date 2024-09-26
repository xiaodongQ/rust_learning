use std::fs;
use std::error::Error;

// 需要添加 pub 关键字，否则无法在 main.rs 中使用
pub struct Config {
    pub query : String,
    pub file_path : String,
}

// 通过 impl 关键字来实现方法
impl Config {
    // 传入参数：动态数组的不可变引用，可以用数组切片来代替
    // 通过Result<T, E>返回错误， 指定 &str 的生命周期为 'static
    // 此处也要添加 pub 关键字，否则无法在 main.rs 中使用
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
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
pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    // 通过std::fs模块的 read_to_string 读取文件内容，其返回结果为 std::io::Result<String>，对应于 Result<T, E>，T为String，E为Error
    // `?`操作符会返回`Result`类型，如果`Ok`则返回`Ok`中的值，如果`Err`则返回`Err`中的值
    let _contents = fs::read_to_string(config.file_path)?;
    
    println!("file read ok!");
    Ok(())
}