// TcpListener 是一个socket server的结构
// TcpStream 是一个Local socket 和remote socket之间的流
use std::net::{TcpListener, TcpStream}; //用net库中的TcpListener, TcpStream
use std::thread;                        //用于对输入的每个流创建一个线程
use std::time;                          //导入时间，用于延时
use std::io::{self, Read, Write};       //为了处理错误,用到io库


//用来处理流,其格式是 TcpStream,返回一个Result
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];     //创建一个buffer
    for _ in 0..1000 {    //循环1000次
        let bytes_read = stream.read(&mut buf)?;  //从buffer里读内容
        if bytes_read == 0 {         //如果读到0，说明已经结束，回到ok
            return  Ok(());
        }
        println!("echo:{}",String::from_utf8_lossy(&buf[..bytes_read]) );  // 这一句用于打印从client接收的结果
        stream.write(&buf[..bytes_read])?;   //否则就读到之后，写回client

        thread::sleep(time::Duration::from_secs(1));  //延时1s
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //?是简单用法，可换except等，函数返回result类型
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); //给每个线程一个结束，线程返回的句柄，是一个joinhandle，并创建一个容器
    // 取得listener的内容
    for stream in listener.incoming() {
        let stream = stream.expect("failed"); //如果流有问题，就failed

        //如果没问题，分配线程
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));  //创建一个线程来处理，并且写上错误处理
        });
        thread_vec.push(handle);  //把handle加到容器里面去
    }


    //等待线程的结束
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(()) //如果发生错误，则直接返回相应的error，如果没有发生错误，返回ok
}