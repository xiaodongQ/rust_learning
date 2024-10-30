use std::thread;

fn main() {
    println!("=========test get_two_sites=======");
    get_two_sites();

    // 直接调用async函数，不会输出任何结果，因为 Future 还未被执行
    println!("\n=========test download_async=======");
    // download_async("sss");
    
    println!("\n=========test get_two_sites_async=======");
    get_two_sites_async();

    println!("\n=========test test_await=======");
    test_await();
}


fn download(file: &str) {
    println!("download file:[{}]...", file);
}
// 并发线程下载网站
fn get_two_sites() {
    // 创建两个新线程执行任务
    let thread_one = thread::spawn(|| download("https://course.rs"));
    let thread_two = thread::spawn(|| download("https://fancy.rs"));

    // 等待两个线程的完成
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// 并发请求量大时，一个下载任务占用一个线程的模式太重了
async fn download_async(file: &str){
    println!("async download file:[{}]...", file);
}

// async并发下载网站
use futures::executor::block_on;
fn get_two_sites_async() {
    // 异步函数的返回值是一个 Future
    let future_one = download_async("https://course.rs");
    let future_two = download_async("https://course.rs");
    // 使用执行器来使用 Future
    // `block_on`执行器会阻塞当前线程直到指定的`Future`执行完成；其它运行时的执行器(executor)会提供更加复杂的行为
    block_on(future_one);
    block_on(future_two);
}


async fn hello_world() {
    // 使用.await可以等待另一个异步调用的完成
    // 但是与block_on不同，.await并不会阻塞当前的线程，而是异步的等待Future A的完成
    // hello_cat();
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}
fn test_await() {
    let future = hello_world();
    block_on(future);
}