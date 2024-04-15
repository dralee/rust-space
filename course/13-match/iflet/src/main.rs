/**
 * 模式匹配
 * 2024.04.15
 * by dralee
 */
fn main() {
    test();
    test_while();
    test_loop();
    test_let();
    test_tuple_arg();
    test_x();
    test_some();
    test_muti();
    test_range();
    test_descructuring();
    test_descructuring_enums();
    test_descructuring_enums2();
    test_descructuring_struct_tuples();
    test_ignore();
    test_ignore_multi();
    test_binding();
}

fn test(){
    println!("if let...");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn test_while(){
    println!("while let...");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn test_loop() {
    println!("loop let...");
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn test_let() {
    let (x,y,z) = (1,2,3);
    println!("x:{x},y:{y},z:{z}");

    //let (x,y) = (1,2,3); // mismatched types
                                // expected tuple `({integer}, {integer}, {integer})`  found tuple `(_, _)`
    
    let (x,y,_) = (10,20,30); // 不需要的“30”可直接使用“_”进行忽略
    println!("x:{x},y:{y}");
}

// 传递元组
fn print_coordinates(&(x,y):&(i32,i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn test_tuple_arg(){
    let point = (3,5);
    print_coordinates(&point);
}

fn test_x(){
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn test_some(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn test_muti(){
    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // 1或2
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/**
 * 编译器编译时检查范围是否为空，且由于Rust可判断范围是否为空的唯一类型是char和数值，因
 * 此范围只允许用数值或char值
 */
fn test_range(){
    let x = 5;
    match x {
        1..=5 => println!("one through five"), // 1,2,3,4,5
        _=> println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _=> println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

/**
 * 模式进行解构结构体、枚举和元组
 * match表达式一旦找到第一个匹配模式，就会停止检查其他分支
 */
fn test_descructuring(){
    let p = Point { x: 0, y: 7};
    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 简略为
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point{x,y:0} => println!("On the x axis at {x}"),
        Point{x:0,y}=> println!("On the y axis at {y}"),
        Point{x,y} => {
            println!("On neither axis: ({x},{y})");
        }
    }
}

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn test_descructuring_enums(){
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit=>{
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        },
        Message::Write(text) => {
            println!("Text message: {text}");
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}

enum Color {
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}
enum Message2 {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}
fn test_descructuring_enums2(){
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        },
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        },
        _ => (), // 其他忽略
    }
}

fn test_descructuring_struct_tuples(){
    let ((feet, inches), Point {x, y}) = ((3,10),Point{x:3, y:-12});
    println!("feet: {feet}, inches: {inches}, x:{x}, y:{y}");
}

/**
 * 忽略匹配值
 */
fn foo(_:i32, y: i32){
    println!("This code only uses the y parameter: {}", y);
}
fn test_ignore(){
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwirte an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2,4,8,16,32);
    match numbers {
        (first,_,third,_,fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let _x = 5; // 不会提示未使用，可使用“_”忽略未使用变量的警告
    let y = 10; // if this is intentional, prefix it with an underscore: `_y`

    // 在
    /*let s = Some(String::from("Hello!"));
    if let Some(_s) = s { // 使用“_”，则值直接丢弃，而使用“_s”，值会绑定到变量
        println!("found a string");
    }
    println!("{:?}", s); // borrow of partially moved value: `s` partial move occurs because value has type `String`, which does not implement the `Copy` trait
    */

    let s = Some(String::from("Hello!"));
    if let Some(_) = s { // 使用“_”，不会绑定到变量，则值直接丢弃，而使用
        println!("found a string");
    }
    println!("{:?}", s); 
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}
fn test_ignore_multi(){
    // 忽略多部分的值
    let origin = Point2 { x: 0, y: 0, z: 0};
    match origin {
        Point2{ x, ..} => println!("x is {}", x), // 只需要x部分，其他忽略，比列出y:_,z:_ 要快
    }

    let numbers = (2,4,8,16,32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    /*match numbers {
        (.., second, ..)=>{  // Rust 无法确定在与值匹配之前要忽略元组中有多少个值 second ，然后再忽略多少个值  `..` can only be used once per tuple pattern can only be used once per tuple pattern
            println!("Some numbers: {}", second);
        }
    }*/

    let num = Some(4);
    // 条件匹配
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50)=> println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),  // 匹配x为4、5、6，同时y为true
        _ => println!("no"),
    }
}

/**
 * @绑定
 */
enum Message3 {
    Hello { id: i32},
}

fn test_binding(){
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 }=>{
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}
