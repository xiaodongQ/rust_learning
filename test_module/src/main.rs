
fn main() {
    // 使用模块中的函数
    my_module::my_function();

    // 使用模块中的结构体
    let my_struct = my_module::MyStruct { field: 42 };
    println!("Field value: {}", my_struct.field);

    // 使用模块中的枚举
    let _my_enum = my_module::MyEnum::Variant;
}

mod my_module {
    // 模块中的函数
    pub fn my_function() {
        println!("Hello from my_module!");
    }
    // 模块中的结构体
    pub struct MyStruct {
        // 结构体字段
        pub field: i32,
    }
    // 模块中的枚举
    pub enum MyEnum {
        // 枚举变体
        Variant,
    }
    // 模块中的特征
    pub trait MyTrait {
        // 特征方法
        fn my_method(&self);
    }
}