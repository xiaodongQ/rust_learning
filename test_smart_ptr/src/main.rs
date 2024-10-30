

fn main() {
    test_simple();
    test_array();
    test_box_arr();

    let s = gen_static_str();
    println!("{}", s);

    test_deref();

    test_auto_deref();

    test_drop();
    test_mem_drop();
}

fn test_simple() {
// 创建一个智能指针指向了存储在堆上的 3，并且 a 持有了该指针
    {
        let a = Box::new(3);
        // 智能指针实现了Deref 和 Drop特征，此处会隐式调用Deref特征，对指针进行解引用
        println!("a = {}", a); // a = 3

        // 下面一行代码将报错，无法隐式调用Deref特征解引用
        // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
        // 因此需要显式使用`*`，调用Deref特征解引用
        let _b = *a + 1;
    }

    // 作用域结束，上面的智能指针就被释放了，因为Box实现了Drop特征
    // 下面使用会报错：cannot find value `a` in this scope
    // let c = *a + 2;
}

fn test_array() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

fn test_box_arr() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let _sum = **first + **second;
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
 }

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_deref() {
    let x = 5;
    let y = MyBox::new(x);

    println!("x = {}, y = {}", x, *y);
}

// 隐式 Deref
fn test_auto_deref() {
    // String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
    let s = String::from("hello world");
    // &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}

/*********************** Drop特征 *****************************/
// #[derive(Copy)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
fn test_drop() {
    let mut _foo = Foo;
    // 报错：explicit destructor calls not allowed
    // _foo.drop();
    println!("Running!");
}

fn test_mem_drop() {
    let foo = Foo;

    // 报错：explicit destructor calls not allowed
    // foo.drop();

    // 调用编译器自动生成的drop函数，释放内存
    drop(foo);
    // 以下代码会报错：借用了所有权被转移的值
    // println!("Running!:{:?}", foo);
}


