fn main() {
    println!("========= test_spwan... =======");
    test_spwan();
    println!("\n========= test_barriers... =======");
    test_barriers();
    println!("\n========= test_condvar... =======");
    test_condvar();
    println!("\n========= test_call_once... =======");
    test_call_once();
}

use std::thread;
use std::time::Duration;
fn test_spwan() {
    // 获取thread::spawn返回的JoinHandle，用于下面的join等待
    // 可在闭包前面添加 move 关键字，这样闭包就会获取所有被引用变量的所有权
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // join 会阻塞当前线程直到 handle 所代表的线程结束
    // 不join则主线程结束时子线程会被强制结束
    handle.join().unwrap();
}

// use std::thread; // 前面已经引入了
use std::sync::{Arc, Barrier};
fn test_barriers() {
    // 数组用于保存线程句柄
    let mut handles = Vec::with_capacity(6);
    // 创建一个线程屏障，等待6个线程，通过线程安全的Arc智能指针来共享
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            // 增加一个线程屏障
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// use std::thread;
// use std::sync::{Arc, Mutex, Condvar};
use std::sync::{Mutex, Condvar};
fn test_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

// use std::thread;
use std::sync::Once;
static mut VAL: usize = 0;
static INIT: Once = Once::new();
// 代码运行的结果取决于哪个线程先调用 INIT.call_once
fn test_call_once() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}