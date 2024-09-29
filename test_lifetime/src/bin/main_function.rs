
fn main() {
    dangle();

    println!("====== test return lifetime======");
    return_lifetime();
}

fn dangle() {
    let x = String::from("hello");

    let r;
    {
        // 让 x 的生命周期比 r 长，就不会有问题，否则引用会超出作用域
        // let x = String::from("hello");
        r = &x; // r is a reference to x
    }
    println!("r: {}", r);
}

// 使用生命周期参数，需要先进行声明（和泛型一样）
// 生命周期标注位于 引用符号& 后面，并添加一个空格
// 此处生命周期标注仅仅说明，这两个参数 x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)。
    // 实际上，这意味着返回值的生命周期与参数生命周期中的较小值一致：
    // 虽然两个参数的生命周期都是标注了 'a，但是实际上这两个参数的真实生命周期可能是不一样的(生命周期 'a 不代表生命周期等于 'a，而是大于等于 'a)
// 由于返回值的生命周期也被标记为 'a，因此返回值的生命周期也是 x 和 y 中作用域较小的那个。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 编译成功：返回值生命周期固定为第一个传入的引用
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 编译失败：悬垂引用
fn longest3<'a>(x: & str, y: &str) -> &'a str {
    String::from("hello").as_str()
}

// 编译成功：返回所有权，并把内部新建字符串的所有权转移给调用者
fn longest4<'a>(x: &'a str, y: &str) -> String {
    String::from("hello")
}

// 编译失败：longest返回值生命周期和string2一致，离开括号后string2生命周期已经结束
fn return_lifetime() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // 此处会报错（括号外使用了result），因为 longest 返回的生命周期是 string1和string2中较小的那个生命周期，离开括号后string2的生命周期已经结束
        result = longest(string1.as_str(), string2.as_str());
    }
    // 注释下面这条访问result的语句，则不会报错，只是警告上面的result没使用
    println!("The longest string is {}", result);
}