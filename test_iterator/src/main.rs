fn main() {
    test_iter();
    test_next();
}

fn test_iter() {
    let arr = [1, 2, 3];
    // 数组实现了 IntoIterator 特征，编译器通过for语法糖，会将arr数组自动转换为迭代器
    for v in arr {
        println!("{}",v);
    }

    // 对数值序列进行迭代
    for i in 1..10 {
        println!("{}", i);
    }

    // 通过IntoIterator特征的 into_iter 方法，显式转换为迭代器
    for v in arr.into_iter() {
        println!("{}", v);
    }

    // 显式定义迭代器，并进行迭代
    let arr_iter = arr.iter();
    for v in arr_iter {
        println!("{}", v);
    }
}

fn test_next() {
    let arr = [1, 2, 3];
    // next 会改变迭代器其中的状态数据，所以迭代器定义时需要使用 mut 关键字
    let mut arr_iter = arr.into_iter();
    // 若通过iter_mut定义迭代器，则迭代器中的数据会自动变为可变数据，下面断言比较需调整为 assert_eq!(arr_iter.next(), Some(&mut 1)); 形式
    // let mut arr_iter = arr.iter_mut();
    // 通过next方法，显式迭代
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}