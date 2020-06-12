// 将 io（输入/输出）库引入当前作用域，获取用户输入并打印结果作为输出
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed:{}", guess);
}
