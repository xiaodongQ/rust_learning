

fn main() {
    test_simple();
    test_array();
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
        let b = *a + 1;
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