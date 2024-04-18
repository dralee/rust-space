/**
 * 单线程web
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
    net::{TcpListener, TcpStream}
};

/**
 * 可在浏览器中访问：http://127.0.0.1:7070
 * 控制台将打印：Connection established!
 */
fn listen(){
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        
        handle_connection(stream);
    }
}

/*
/** 响应空页面 */
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request:Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n"; // no header no body
    stream.write_all(response.as_bytes()).unwrap(); // 响应
}*/

/*
/**
 * 响应hello.html
 */
 fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request:Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); // 响应
}
*/

/*
/**
 * 根据请求响应对应内容
 */
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // 请求行

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
        stream.write_all(response.as_bytes()).unwrap(); // 响应
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap(); // 响应
    }
}*/


/**
 * 根据请求响应对应内容，优化响应
 */
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // 请求行

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap(); // 响应
    
}