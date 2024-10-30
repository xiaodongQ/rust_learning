fn main() {
    test_raw_pointer();
    test_unsafe_func();
    test_unsafe_static();
}

fn test_raw_pointer() {
    let mut num = 5;

    // 将引用 &num / &mut num 强转为相应的裸指针 *const i32 / *mut i32
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用裸指针是不安全的行为，需要放到unsafe语句块中，否则编译报错
    unsafe {
        // 报错：`r1` is a `*const` pointer, so the data it refers to cannot be written
        // *r1 = *r1 + 1;
        
        // 执行结果 r1 is: 5
        println!("r1 is: {}", *r1);
        *r2 = *r2 + 1;
        // 执行结果 r1 is: 6
        println!("r2 is: {}", *r2);
    }

    // 还可根据智能指针创建裸指针
    let a: Box<i32> = Box::new(10);
    // 解引用再取引用创建
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);
    unsafe {
        println!("b:{}, c:{}", *b, *c);
    }
}

unsafe fn dangerous() {
    // unsafe函数或方法里，不用重复用unsafe限定了
    println!("dangerous() called");
}
fn test_unsafe_func() {
    // unsafe函数或方法调用时，需要包裹在unsafe语句块中
    unsafe {
        dangerous();
    };
}

static mut REQUEST_RECV: usize = 0;
fn test_unsafe_static() {
    // 访问或修改可变static变量时，不加unsafe则会报错
    // note: mutable statics can be mutated by multiple threads: 
        // aliasing violations or data races will cause undefined behavior
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }
}

unsafe trait _Foo {
    // 方法列表
}
unsafe impl _Foo for i32 {
    // 实现相应的方法
}
