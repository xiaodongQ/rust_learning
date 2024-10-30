fn main() {
    rc_ptr();
    cell_ptr();
    refcell_ptr();
    rc_refcell();
}

fn rc_ptr() {
    use std::rc::Rc;
    let s = String::from("hello, world");
    // 使用Rc类型，Rc::new 创建一个引用计数类型的智能指针
    // 智能指针 Rc<T> 在创建时，引用计数会加1，可通过关联函数 Rc::strong_count(&a) 获取引用计数
    let a = Rc::new(s);
    println!("a referce count1: {}", Rc::strong_count(&a));
    
    // 用 Rc::clone 克隆了一份智能指针，引用计数也会加1
    // 此处clone不是深拷贝，仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
    let b = Rc::clone(&a);
    println!("a referce count2: {}", Rc::strong_count(&a));
    println!("b referce count: {}", Rc::strong_count(&b));

    // 使用 Arc::new 创建智能指针
    use std::sync::Arc;
    let s1 = Arc::new(String::from("test arc"));
    // 可通过 *s1 解引用获取值
    println!("s1 referce count: {}, *s1:{}", Arc::strong_count(&s1), *s1);
}

fn _wrong_ptr() {
    let s = String::from("hello, world");
    // s在这里被转移给a
    let _a = Box::new(s);
    // 报错！此处继续尝试将 s 转移给 b
    // let b = Box::new(s);
}

use std::cell::Cell;
fn cell_ptr() {
    // 此处的"asdf"是&str类型，实现了Copy
    let c = Cell::new("asdf");

    // String 没有实现 Copy 特征，所以Cell中不能存放String类型
    // let c2 = Cell::new(String::from("asdf"));
    // 编译报错：error[E0599]: the method `get` exists for struct `Cell<String>`, but its trait bounds were not satisfied
        // doesn't satisfy `String: Copy`
    // println!("c2:{}", c2.get());

    // 获取当前值，不是实时指针，后面修改不影响此处
    let one = c.get();
    // 获取值到one里后，通过c还能做修改，修改c不影响one的值
    c.set("qwer");
    let two = c.get();
    // 结果：one:asdf, two:qwer, c.get():qwer
    println!("one:{}, two:{}, c.get():{}", one, two, c.get());
    // 不可通过 *c 解引用获取值
    // println!("*c:{}", *c);
}

use std::cell::RefCell;
fn refcell_ptr() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    println!("s1:{}", s1);

    // 编译时编译器不会报错，但违背了借用规则，会导致运行期 panic
    // let s2 = s.borrow_mut();
    // println!("s2:{}", s2);
}

// use std::cell::RefCell;
use std::rc::Rc;
fn rc_refcell() {
    // 用 RefCell<String> 包裹一个字符串，并通过 Rc 创建了它的三个所有者
    let s = Rc::new(RefCell::new("hello world".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // 通过其中一个所有者 s2 对字符串内容进行修改
    s2.borrow_mut().push_str(", xxxxx!");

    // let mut s2 = s.borrow_mut();
    // s2.push_str("xxxxx");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
