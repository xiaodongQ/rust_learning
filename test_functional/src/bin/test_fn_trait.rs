fn main() {
    test();
    test_fn_once();
    test_fn_mut();
    test_fn_mut_param();
    test_fn_trait();
}

/****************** 闭包的类型推导 begin *******************/
fn test() {
    // 自动推断类型
    let sum = |a, b| a + b;
    println!("sum(1, 2) = {}", sum(1, 2));
}
/****************** 闭包的类型推导 end *******************/


/****************** 结构体中的闭包 begin *******************/
// 定义缓存结构体，泛型参数T是闭包类型
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 该字段是一个闭包
    query: T,
    // Option枚举
    value: Option<u32>,
}

// 实现结构体方法
// Fn 是闭包特征(trait)，Fn(u32) -> u32 表示闭包参数为u32，返回值为u32
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // 在则返回
            Some(v) => v,
            // 不在则缓存后返回
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
/****************** 结构体中的闭包 end *******************/


/****************** 闭包的3种Fn特征 *******************/
/* ============= 1. FnOnce ===================== */
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
    // F: FnOnce(usize) -> bool + Copy,
{
    println!("fn_once: {}", func(3));

    // 下面会报错，因为 FnOnce 只能调用一次，上面调用后 func 的所有权已经转移
    // println!("{}", func(4));
    // 上面where子句中，约束里在添加 Copy 特征：`F: FnOnce(usize) -> bool + Copy,`，
    // 则调用时使用的将是它的拷贝，所以并没有发生所有权的转移。那么第二次调用 func(4) 就不会报错了
}
fn test_fn_once() {
    println!("=========== test_fn_once ===========");
    let x = vec![1, 2, 3];
    fn_once( |z|{z == x.len()});

    // 如果要强制闭包转移所有权，可以使用在参数列表前加上 `move` 关键字
    // fn_once( move|z|{z == x.len()});
}

/* ============= 2. FnMut ===================== */
fn test_fn_mut() {
    println!("=========== test_fn_mut ===========");
    let mut s = String::new();

    // 若按此处定义，则update_string调用时编译器会报错，内部不支持变量的可变借用。需要将update_string定义为可变闭包
    // 添加mut关键字，可看到rust-analyzer推断其类型为 `impl FnMut(&str)`
    // let update_string = |str| s.push_str(str);

    let mut update_string = |str| s.push_str(str);

    update_string("hello");
    update_string(", world");

    println!("{:?}", s);
}

fn test_fn_mut_param() {
    println!("=========== test_fn_mut_param ===========");
    let mut s = String::new();
    let update_string =  |str| s.push_str(str);
    // 闭包作为参数传递给函数，此处会转移所有权
    exec_fn_mut(update_string);
    println!("{:?}", s);
}

// 泛型参数标注闭包为 FnMut 特征，并传递可变借用
fn exec_fn_mut<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello2");
    f(", world2");
}

fn test_fn_copy() {
    let s = String::new();
    let update_string =  || println!("test_fn_copy:{}", s);
    update_string();
    println!("{:?}", s);
}

/* ============= 3. Fn ===================== */
fn test_fn_trait() {
    println!("=========== test_fn_trait ===========");
    let mut s = "hello".to_string();

    // 传给 exec_fn 会报错，该闭包中要修改变量，而 Fn 特征要求闭包为不可变借用
    // let update_string =  |str| s.push_str(str);

    let update_string = |str| println!("{}, {}", s, str);
    exec_fn(update_string);

    println!("s: {:?}", s);
}

// 泛型参数标注闭包为 Fn 特征，并传递不可变借用的闭包
fn exec_fn<'a, F: Fn(&'a str)>(f: F) {
    f("world")
}