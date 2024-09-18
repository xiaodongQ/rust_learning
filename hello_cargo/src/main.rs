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
    println!("=============");
    my_condition();

    println!("=============");
    test_while();
}

fn my_condition() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
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
            // 加不加分号都可以，因为loop是一个表达式，可以返回值
            break
        }
        i = i+1;
        println!("again!");
    }
    println!("test_loop end!")
}

fn test_for(){
    // rev()表示逆序，此处即3、2、1
    for num in (1..4).rev(){
        println!("{}!", num);
    }

    // 可用`_`忽略变量消除未使用警告
    for _ in 1..4 {
        println!("again!");
    }

    println!("test_for end!")
}

fn test_while() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }

    println!("test_while end!")
}
