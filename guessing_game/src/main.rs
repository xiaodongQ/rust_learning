// 将 io（输入/输出）库引入当前作用域，获取用户输入并打印结果作为输出
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Rust 默认使用 i32
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        // 对上面的guess变量进行了隐藏，通过let guess:u32告诉parse解析具体数字类型
        // let guess:u32 = match guess.trim().parse().expect("Please type a number!);
        let guess:u32 = match guess.trim().parse() {
            // 能转换成功则返回一个包含数字的Ok，num1用于接收转换结果
            Ok(num1) => num1,
            Err(_) => {
                // 若此处不用trim()则回车也会算在guess变量中
                println!("guess:[{}] invalid! please type a num!", guess.trim());
                continue;
            }
        };
        println!("You guessed:{}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
