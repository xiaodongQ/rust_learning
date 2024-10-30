fn main() {
    let instance = ImportantExcerpt {
        part: "hello",
    };
    let part = instance.print_and_return_part("info");
    println!("get part: {}", part);
}

// 用于结构体打印
#[derive(Debug)]
struct ImportantExcerpt <'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt <'a> {
    // 编译器会依据生命周期消除规则，自动进行生命周期推断
    fn print_and_return_part(&self, info: &str) -> &str {
        println!("info: {}", info);
        self.part
    }

    // 此时，返回值标注生命周期为b'，需要指定 a' 和 b' 的生命周期关系，在最后添加 where 'a: 'b
    // 'a: 'b 是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
    fn _print_and_return_part2<'b>(&'a self, info: &'b str) -> &'b str 
    where
        'a : 'b,
    {
        println!("info: {}", info);
        self.part
    }
}

impl<'a: 'b, 'b> ImportantExcerpt <'a> {
    // 上面声明了'b，这里就不需要再声明了
    fn _print_and_return_part3(&'a self, info: &'b str) -> &'b str {
        println!("info: {}", info);
        self.part
    }
}