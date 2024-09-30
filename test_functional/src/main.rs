// 引入一系列trait
// use std::prelude;

fn main() {
    let f = factory(2);
    println!("f(3) = {}", f(3));

    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    println!("{}",weibo.summarize());
}

fn factory(x:i32) -> impl Fn(i32) -> i32 {

    let num = 5;
    move |x| x + num

    // 注意：impl Trait 的返回方式有一个非常大的局限，就是只能返回同样的类型
    // 就算签名一样的闭包，类型也是不同的，因此在这种情况下，就无法再使用 impl Trait 的方式去返回闭包
    // if x > 1{
    //     move |x| x + num
    // } else {
    //     move |x| x - num
    // }
}

// 定义一个特征
pub trait Summary {
    // 签名，不包含具体实现
    fn summarize_author(&self) -> String;

    // 也可以定义默认实现，为类型impl实现时可进行重载
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 定义一个类型，并为其实现特征
#[derive(Debug)]
pub struct Weibo {
    pub username: String,
    pub content: String
}
// 实现特征
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
