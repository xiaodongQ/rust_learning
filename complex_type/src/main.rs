// use std::fmt::Error;

fn main() {
    test_struct();

    let _result = test_result();
}

struct User {
    username: String,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

fn test_struct(){
    // 下面需要修改user，则此处需要定义为 mut
    let mut user = User {
        username: String::from("xdtest"),
        email: String::from("test@xd.com"),
    };
    println!("username: {}, email: {}", user.username, user.email);

    // 修改user的值
    user.username = String::from("xdtest2");
    println!("username: {}, email: {}", user.username, user.email);

    // 借助 `#[derive(Debug)]` 和 {:?} 打印结构体信息
    let rect = Rectangle { _width: 30, _height: 50,};
    println!("{:?}", rect);
    // {:#?} 会在打印时带上缩进
    println!("{:#?}", rect);
}

use std::fs::File;
use std::io;
fn test_result() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    // 方式1，使用 match 匹配错误，然后返回
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            return Err(error);
        }
    };

    // 方式2，使用 ? 返回错误
    let _f2 = File::open("hello.txt")?;
    // 最后需要返回一个值，和函数的返回值类型一致
    Ok("success".to_string())
}