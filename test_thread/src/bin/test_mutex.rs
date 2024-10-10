use std::sync::Mutex;

fn main() {
    test_mutex();
    test_threads_mutex();
}

fn test_mutex() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);

    {
        // 获取锁，然后deref为`m`的引用
        // lock返回的是Result
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }

    println!("m = {:?}", m);
}

// 多线程的锁
// use std::rc::Rc;
use std::sync::Arc;
use std::thread;
fn test_threads_mutex() {
    // 创建一个整数0，并包装在Mutex中，然后包装在Arc指针中进行共享，Mutex保证并发安全
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // clone增加引用计数并克隆 Arc 的所有权，这样每个线程都有自己的 Arc 实例指向相同的 Mutex
        let counter = Arc::clone(&counter);
        // 创建线程
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // Deref自动解引用，获得指向Mutex内部数据的引用
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 结果为 Result: 10，10个线程都+1
    println!("Result: {}", *counter.lock().unwrap());
}