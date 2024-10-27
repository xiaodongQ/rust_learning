use mini_redis::{client, Result};

// 通过该属性（实际是个宏），标记为 异步main函数
#[tokio::main]
async fn main() -> Result<()> {
    // 和服务端建立连接
    // mini-redis提供的client::connect函数也是一个async函数，返回一个 Future（实现了该特征的类型）
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置值
    client.set("hello", "world".into()).await?;
    // 获取值
    let result = client.get("hello").await?;
    println!("get result:{:?}", result);

    Ok(())
}