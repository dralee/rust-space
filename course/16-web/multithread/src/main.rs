/**
 * 多线程web
 * 
 * 解决单线程问题，可使用多线程，多线程有多种方式：线程池、fork/join、单线程异步I/O模型、多线程异步I/O模型
 * 2024.04.18 by dralee
 * 
 * HTTP基于文本的协议，格式(CRLF即\r\n)：
 * 请求格式：
 * Method Request-URI HTTP-Version CRLF
 * headers CRLF
 * message-body
 * 响应格式：
 * HTTP-Version Status-Code Reason-Phrase CRLF
 * headers CRLF
 * message-body
 */
fn main() {
    listen();
}

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};
//use threadpool::ThreadPool;
use multithread::ThreadPool;

/*
/**
 * 直接为每个请求创建一个线程。如果同时请求很多的话，会耗死系统资源（如DoS攻击）
 * 
 * 可在浏览器中访问：http://127.0.0.1:7070、http://127.0.0.1:7070/sleep，并不会因为sleep卡而导致其他请求不可访问
 * 控制台将打印：Connection established!
 */
fn listen(){
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        
        thread::spawn(||{
            handle_connection(stream);
        });
    }
}*/

/**
 * 创建有限的线程数量（线程池）
 * 如果同时请求很多的话，会耗死系统资源（如DoS攻击）
 * 
 * 可在浏览器中访问：http://127.0.0.1:7070、http://127.0.0.1:7070/sleep，并不会因为sleep卡而导致其他请求不可访问
 * 控制台将打印：Connection established!
 */
fn listen(){
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    let pool = ThreadPool::new(4); // 创建工作线程为4的线程池

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        
        pool.execute(||{
            handle_connection(stream);
        });
    }
}

/**
 * 根据请求响应对应内容
 */
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // 请求行

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5)); // 卡5s
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap(); // 响应
    
}