/**
 * 线程使用
 * 2024.04.09 by dralee
 */

fn main() {
    println!("specifiy the choice number for run the test(1~4), eg: cargo run 1");
    let option = Option::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    match option.choice {
        1 => test(),
        2 => test1(),
        3 => test2(),
        4 => test3(),
        _=>()
    }
    // test();
    // test1();
}

use std::env;
use std::process;
use std::thread;
use std::time::Duration;

struct Option {
    choice: u32,
    name: String,
}
impl Option {
    fn build(mut args: impl Iterator<Item=String>,) -> Result<Option, &'static str> {
        args.next(); // 跳过第一个参数，即程序文件名

        let choice = match args.next() {
            Some(c) => c,
            None => return Err("Didn't get a query string")
        };
        let num:u32 = choice.parse().unwrap();
        Ok(Option{ choice: num, name: choice})
    }
}

fn test(){
    println!("===================simple not join main=================");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // 1ms
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } // 主线程结束，所有生成的线程都会关闭，无论他们是否已经完成运行!!!
    /*
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
     */
}


fn test1(){
    println!("==================join main==================");
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // 1ms
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } // 主线程结束，所有生成的线程都会关闭，无论他们是否已经完成运行!!!
    // 使用join，将子线程跟主线程合并
    handler.join().unwrap();

    /*
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
     */
}

fn test2(){
    println!("==================join main before main thread==================");
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // 1ms
        }
    });

    // 使用join，将子线程跟主线程合并
    handler.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } // 主线程结束，所有生成的线程都会关闭，无论他们是否已经完成运行!!!

    /*
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
     */
}

/*
fn test3(){
    let v = vec![1,2,3];
    let handler = thread::spawn(||{ // to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword: `move `
        println!("Here's a vector: {:?}", v);
    });
    drop(v); // 如果rust允许这程序运行，则可能v已被翻译，而线程中还未结束，就会导致线程中的引用失效，从而报错
    handler.join().unwrap();
} */

fn test3(){
    println!("==================move into thread==================");
    let v = vec![1,2,3];
    let handler = thread::spawn(move ||{
        println!("Here's a vector: {:?}", v);
    });
    
    //println!("out the vector: {:?}", v); // borrow of moved value: `v` value borrowed here after move
    
    handler.join().unwrap();
}