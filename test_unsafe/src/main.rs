fn main() {
    test_raw_pointer();
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
}