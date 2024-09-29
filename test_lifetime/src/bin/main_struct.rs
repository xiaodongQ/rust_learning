fn main() {
    test_success();

    test_failed();
}

// 用于结构体打印
#[derive(Debug)]
struct ImportantExcerpt <'a>{
    part: &'a str,
}

// 不保证结构体中引用字段的生命周期 比 结构体本身的生命周期 **更长**
fn test_success() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let text = ImportantExcerpt {
        part: first_sentence,
    };
    println!("info: {:?}", text);
}

fn test_failed() {
    // 结构体定义
    let text;

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        // next() 返回一个 Option<&'a str>，成功时，返回一个切片引用
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        // 引用字段的生命周期 比 结构体本身的生命周期 短，而大括号外又访问了，所以会编译失败
        text = ImportantExcerpt {
            part: first_sentence,
        };
        // 正常打印
        println!("info1: {:?}", text);
    }
    
    // 大括号外访问，结构体和其中字段引用的生命周期不满足生命周期条件，所以会编译失败
    println!("info2: {:?}", text);
}