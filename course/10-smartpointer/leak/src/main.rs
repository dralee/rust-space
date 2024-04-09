/**
 * 循环引用可能导致内存泄漏
 * 2024.04.09 by dralee
 */

fn main() {
    cycle_ref_leak();

    weak_test();
    weak_test_scope();
}

use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

fn cycle_ref_leak(){
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // 指向a
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // a指向b
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail()); // 循环引用，会导致内存溢出
                                                // thread 'main' has overflowed its stack fatal runtime error: stack overflow
}

// 解决循环引用问题
// 使用弱引用Weak<T>，RC::downgrade，只增加weak_count，而不会增加strong_count，不影响对象清理
// Weak<T>可能补清除，验证是否已被清除可使用upgrade，返回为Some则未删除，为None则已被删除

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 指向父节点的弱引用
    children: RefCell<Vec<Rc<Node>>>,
}
fn weak_test(){
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leaf的父指向弱branch
    println!("leaf: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("branch: strong count = {}, weak count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn weak_test_scope(){
    println!("=======================");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leaf的父指向弱branch

        println!("branch: strong count = {}, weak count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
    println!("leaf: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}