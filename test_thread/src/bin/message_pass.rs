use std::sync::mpsc;
use std::thread;

fn main() {
    test_mpsc();
}

fn test_mpsc() {
    // 创建一个消息通道，返回一个元组 (发送者, 接收者)
    let (tx, rx) = mpsc::channel();
    // 也可显式指定泛型类型
    let (tx2, rx2): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 传输未实现Copy trait的类型
        let s = String::from("hello");
        tx2.send(s).unwrap();
        // 无法再使用s，因为所有权已经转移，报错：value borrowed here after move
        // println!("s: {}", s);
    });

    // 在主线程中接收子线程发送的消息并输出
    // 若接收不到消息，recv方法会阻塞当前线程，直到读取到值，或者通道被关闭
    let recv = rx.recv().unwrap();
    // 使用 try_recv 方法则不会阻塞当前线程，若接收不到消息时，返回一个错误
    // let recv = rx.try_recv().unwrap();
    println!("recv: {}", recv);

    let s = rx2.recv().unwrap();
    println!("s: {}", s);
}