fn main() {
    test_struct();
}

struct User {
    username: String,
    email: String,
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
}