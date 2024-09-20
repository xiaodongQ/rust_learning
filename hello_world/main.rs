fn main() {
    println!("Hello, world!");

    test_owner_ship();

    let mut s = String::from("hello");
    borrow_test(&mut s);
}

fn test_owner_ship() {
    let s = String::from("hello");
    // 所有权转移，会影响s的生命周期；若需要拷贝，则可以使用 s.clone()
    // let s2 = s;
    println!("{}", s);
    task_ownership(s);
    // 这里所有权转移到task_ownership后，已经释放了s，所以这里会报错
    // println!("{}", s);

    println!("===============");
    let n = 888;
    // 这里不影响n的所有权，基本类型不会发生所有权转移
    let n2 = n;
    println!("n: {}, n2: {}", n, n2);
}

fn task_ownership( s : String ) {
    println!("input string: {}", s);
    // 调用结束后，s移出作用域，被释放
}

fn borrow_test(s : &mut String) {
    println!("input string: {}", s);
    s.push_str(" world");
    println!("after string: {}", s);
}