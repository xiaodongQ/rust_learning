pub mod my_module {
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

// 模块中的测试代码
#[cfg(test)]
mod tests {
    use super::my_module::*;

    #[test]
    fn test_my_function() {
        my_function();
    }
}


