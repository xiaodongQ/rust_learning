fn main() {
    test_struct();
}

struct User {
    username: String,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
    let rect = Rectangle { width: 30, height: 50,};
    println!("{:?}", rect);
    // {:#?} 会在打印时带上缩进
    println!("{:#?}", rect);
}