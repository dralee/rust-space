/**
 * 线程使用，共享状态传递数据
  * 2024.04.09 by dralee
  * std::marker特征Sync和Send
  * 任何完全由 Send 类型组成的类型也会被自动标记 Send 。几乎所有的基元类型都是 Send ，除了原始指针
 */
fn main() {
    mutex_state();
    mutext_state_thread();
}

use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

// 使用互斥锁允许一次从一个线程访问数据
fn mutex_state(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

/*
fn mutext_state_thread() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // borrow of moved value: `counter` value borrowed here after move
}*/

// 多个线程的多个所有权，由于Rc非多线程安全（原因clone()中+1引用计数不是原子性的）
/*fn mutext_state_thread() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    // Rc不是多线程安全的
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        /*
        `Rc<Mutex<i32>>` cannot be sent between threads safely
within `{closure@11-concurrency/sharestate/src/main.rs:52:36: 52:42}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`, which is required by `{closure@11-concurrency/sharestate/src/main.rs:52:36: 52:42}: Send` */
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //println!("Result: {}", *counter.lock().unwrap()); // borrow of moved value: `counter` value borrowed here after move
}*/


// 多个线程的多个所有权， Arc<T>是原子性的,A是原子（A（原子版本的)RC
fn mutext_state_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // Rc不是多线程安全的
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}