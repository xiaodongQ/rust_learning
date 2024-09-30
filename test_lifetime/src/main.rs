use std::fmt::Display;

fn main() {
    let mark_twain: &str = "Samuel Clemens";
    print_author(mark_twain);

    print(&mark_twain);

    get_memory_location();

    test_nll();

    test_reborrow();
}

// 'static 生命周期
fn print_author(author: &'static str) {
    println!("{}", author);
}

// 特征对象的生命周期也是 'static
fn print<T: Display + 'static>(message: &T) {
    println!("{}", message);
}

// &'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则
fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是"Hello World!"数据依然还会继续存活
}

fn test_nll() {
    let mut s = String::from("hello");
 
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束
 
    let r3 = &mut s;
    println!("{}", r3);
    // 若此处还访问r1,r2，则可变引用r3存在，r1,r2被借用，无法访问。编译器会报错
    // println!("{}", r2);
 }


fn test_reborrow() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // 对于再借用而言，rr 再借用时不会破坏借用规则，但是不能在它的生命周期内再使用原来的借用 r
    let rr: &Point = & *r;
    // rr最后一次使用，基于NLL规则，rr作用域在这里结束

    println!("{:?}", rr);
    // 在 rr 的生命周期外，r 依然可以使用
    r.move_to(10, 10);
    println!("{:?}", r);
}
// 上面需要的结构体和方法定义
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
