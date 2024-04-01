/* 
* Box<T>智能指针
* 2024.04.01 by dralee
*/
fn main() {
    let b = Box::new(5); // 存储指向i32值5的指针
    println!("b = {}", b);

    cons_list();

    test();

    test_my();
    test_my2();
}

/* 无法知道要分配多大空间
enum List {
    Cons(i32, List), // 递归 recursive type `List` has infinite size
    Nil,
}

fn cons_list(){
    let list = Cons(1, Cons(2, Cons(3, Nil)));
} */

enum List {
    Cons(i32, Box<List>), // 使用Box<T>智能指针,因为指针的大小是确定的(usize)
    Nil,
}

use crate::List::{Cons, Nil};

fn cons_list(){
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

} // list指向的Box的智能指针自动清除

fn test(){
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 自动调用 *(y.deref())
}

use boxpointer::MyBox;

fn test_my(){
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y); // type `MyBox<{integer}>` cannot be dereferenced 需要实现Deref特征
}

fn hello(name:&str){
    println!("Hello, {name}!");
}
fn test_my2(){
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 实现了Deref特征,所以会将MyBox<String>调用deref变成&String,String的Deref实现返回&str

    hello(&(*m)[..]); // 如果String没实现Deref并返回&str,则需要这么调用
}