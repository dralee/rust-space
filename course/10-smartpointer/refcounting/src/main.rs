/* 
* 引用计数Rc<T>智能指针,支持多引用操作
* 2024.04.01 by dralee
*/

//! #引用计数 Rc<T>
//! 
//! reference counting
//! 实现多处指针引用
//! Rc<T>是单线程的


fn main() {
    test();
    test1();

}

/* 
enum List {
    Cons(i32, Box<List>),
    Nil
}
use crate::List::{Cons, Nil};

//Box<T>无法多次引用
fn test(){
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a)); // a被移入b,b拥有a
    let c = Cons(4, Box::new(a)); // use of moved value: `a` value used here after move

}*/

enum List {
    Cons(i32, Rc<List>),
    Nil
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn test(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // strong_count + 1,Rc::clone()只是计数加1,并不会深拷贝,而a.clone是深拷贝
    let c  = Cons(4, Rc::clone(&a)); // strong_count + 1, 可多次引用a
}

fn test1(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}