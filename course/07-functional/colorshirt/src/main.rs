use core::str;
use std::{thread, time::Duration, vec};

/**
 * 功能 闭包
 * 2024.03.30 by dralee
 * 
 * 演化
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
 */
fn main() {
    short_choose();

    closure();
    closure2();
    closure3();
    closure4();
    closure5();
    closure7();
}

fn short_choose(){
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red]
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    // 用户有选择,则按选择的,否则按库存最多的
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // 闭包表达式
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn closure(){
    // 闭包
    let expensive_closure = |num:u32| -> u32 {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("result: {}", expensive_closure(8));
}

fn closure2(){
    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); // 第一次调用闭包,传的是String,编译器自动推断为String,并锁定类型
    //let n = example_closure(5); // mismatched types expected `String`, found integer
}

fn closure3(){
    let list = vec![1,2,3,4,5];
    println!("Before defining closure: {:?}", list);

    // 定义变量接收闭包
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After called closure: {:?}", list);

    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let mut borrow_mutably = || list.push(8);
    borrow_mutably();
    println!("After called closure: {:?}", list);
}

fn closure4(){
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    // 如果想强制闭包取得它在环境中使用的值的所有权，即使闭包主体并不严格需要所有权，可以在参数列表之前使用 move 关键字。
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

/*
 * 闭包将以附加的方式自动实现一个、两个或所有这三个 Fn 特性:
FnOnce 应用于可调用一次的闭包。所有闭包都至少实现了这个 trait，因为所有的闭包都可以被调用。将捕获的值移出主体的闭包只会实现 FnOnce，而不会实现其他任何 Fn 特性，因为它只能调用一次。
FnMut 适用于不将捕获的值移出主体的闭包，但这可能会改变捕获的值。可以多次调用这些闭包。
Fn 适用于不将捕获的值移出主体、不对捕获的值进行变异的闭包，以及不从环境中捕获任何内容的闭包。可以多次调用这些闭包，而不会改变它们的环境，这在多次并发调用闭包等情况下非常重要。
 * 
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn closure5(){
    let mut list = [
        Rectangle{ width: 10, height: 1},
        Rectangle{ width: 3, height: 7},
        Rectangle{ width: 7, height: 12}
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    /*
     * Sort _ by _ key 被定义为接受 FnMut 闭包的原因是它多次调用该闭包: 
     * 对于片中的每个项目调用一次。闭包 | r | r.width 
     * 不捕获、变异或从其环境中移除任何内容，因此它满足特性绑定需求。
     */
}

fn closure6(){
    let mut list = [
        Rectangle{ width: 10, height: 1},
        Rectangle{ width: 3, height: 7},
        Rectangle{ width: 7, height: 12}
    ];

    //let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        //sort_operations.push(value); // cannot move out of `value`, a captured variable in an `FnMut` closure
                                    //move occurs because `value` has type `String`, which does not implement the `Copy` trait
        r.width
    });
    println!("{:#?}", list);
}

fn closure7(){
    let mut list = [
        Rectangle{ width: 10, height: 1},
        Rectangle{ width: 3, height: 7},
        Rectangle{ width: 7, height: 12}
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // 没发生移动,因此ok
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}