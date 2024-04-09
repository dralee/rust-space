/**
 * 线程使用，消息传递数据
 * 使用通道(channel)
 * 2024.04.09 by dralee
 */
fn main() {
    test();
    test1();
    test2();
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 单个发送，单个接收
fn test(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val); // borrow of moved value: `val` value borrowed here after move
    });

    let received = rx.recv().unwrap(); // recv()阻塞，try_recv()不阻塞
    println!("Got: {}", received);
    println!("=========================");
}

// 单个发送多个消息，单个接收多个消息
fn test1(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();        
            thread::sleep(Duration::from_secs(1));
        }        
    });

    for received in rx { // 通道关闭，迭代结束
        println!("Got: {}", received);
    }
    println!("=========================");
}

// 多个生产者/单个消费者
fn test2(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("=========================");
}