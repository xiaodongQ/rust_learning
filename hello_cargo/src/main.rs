fn main() {
    println!("Hello, world!");
    my_function(123);

    let a = {
        let b = 4;
        // 这句不能有分号，否则为表达式，不产生返回值
        b+1
    };
    println!("a:{}", a);

    println!("five():{}", five());
    println!("=============");
    test_loop();
    println!("=============");
    test_for();
}

fn my_function(x : i32) {
    println!("param x:{}", x);
}

fn five() ->i32{
    5
}

fn test_loop(){
    let mut i = 0;
    loop{
        if i>4{
            break
        }
        i = i+1;
        println!("again!");
    }
}

fn test_for(){
    for num in (1..4).rev(){
        println!("{}!", num);
    }
}
