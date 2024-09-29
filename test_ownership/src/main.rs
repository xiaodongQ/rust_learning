fn main() {
    let s1 = String::from("hello");
    let res = func_one(s1);
    // println!("s1:{}", s1); //报错，所有权已经转移到函数中了
    println!("return res:{}", res);

    println!("============");
    let s2 = String::from("world");
    func_ref(&s2);

    println!("============");
    let mut s3 = String::from("xyz");
    func_mutref(&mut s3);
    println!("s3:{}", s3);

    let r1 = &mut s3;
    func_mutref(r1);
    println!("s3:{}", s3);
    // println!("r1:{}", r1); // 跟上面一句只能同时有一条，上面是使用非mut借用，此处mut借用

    // 此处再定义一个 &mtx s3 是可以的，why?
    // 貌似只要不同时使用就不报错(某个引用定义了但不使用，和一个使用中的引用并不报错，只是警告未使用)
    let r2 = &mut s3;
    func_mutref(r2);
    println!("s3:{}", s3);

    println!("============");
    println!("return:{}", func_dangling());
}

fn func_one(s: String) -> String{
    println!("input s:{}", s);
    let newstr = String::from("new");
    // 用 newstr 替换下面这句也可以，注意不要分号(表达式)
    return newstr;
} // }结束，s移出作用域，调用drop释放String占用的内存

fn func_ref(s: &String){
    println!("input s:{}", s);
    // s.push_str("111"); //不允许修改借用的值
}

fn func_mutref(s: &mut String){
    println!("input s:{}", s);
    s.push_str("+111"); //允许修改借用的值
}

// fn func_dangling() -> &String{ return &s //这种产生空悬引用的方式会报错 missing lifetime specifier
fn func_dangling() -> String{
    let s = String::from("test dangling");
    s
}