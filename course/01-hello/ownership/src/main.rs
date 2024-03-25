/**
 * ownership 内存所有权
 * 2024.03.25 by dralee
 */
fn main() {
    string_1();
    own_function();
    return_value();
    tuple_ret();

    // 引用解决对象传递被移动问题
    ref_return();
    ref_mut();
    ref_multi();

    // 分片
    slice_ref();
}

fn string_1(){
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1:{s1}"); // 异常，已被移交给s2
    println!("s2:{s2}");

    println!("clone...");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let s3 = "hello";
    let s4 = s3;    
    println!("s3:{s3},&"); // 副本
    println!("s4:{s4},&");
}

fn own_function(){
    let s = String::from("hello");
    takes_ownership(s); // s被移到takes_ownership方法内，此作用域内s不再有效
    //println!("after call s is {}", s); // 报错 value borrowed here after move

    let x = 5;
    makes_copy(x); // x移入makes_copy方法内，但i32具有Copy特性，因此，此作用域内依然可使用x
    println!("after call x is {}",x); // 5
}

fn takes_ownership(s:String) {// 接入s
    println!("{}", s);
}// s被释放

fn makes_copy(x:i32) {
    println!("{}", x)
}// 整数离开作用域后，没有特别事情发生

fn return_value(){
    let s1 = gives_ownership(); // 方法移动其返回值给s1    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2被移到takes_and_gives_back方法中，并由方法返回值移到s3中
    // s2被移动了，不能使用
    println!("s1:{}", s1);
    println!("s3: {}", s3);

} // 至此，s3离开作用域被释放，s2被移动，s1离开作用域被释放

fn gives_ownership() -> String {
    let s = String::from("yours");
    s // s返回值，s被移出本方法调用
}
fn takes_and_gives_back(s: String) -> String {
    s  // s 被返回，并移出方法调用
}

fn tuple_ret(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of \"{}\" is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// 使用引用，解决对象传值被移动问题
fn ref_return(){
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of \"{}\" is {}.", s1, len);

    change(&s1);
    println!("s1: {}", s1);

    let mut s2 = String::from("hello");
    change1(&mut s2);
    println!("s1: {}", s2);
}
fn calculate_length2(s:&String) -> usize {
    s.len()
}
fn change(s: &String) { // s为不可修改的
    println!("immutable value can't be mutated.");
    //s.push_str(", world!"); // 因此不可对其进行修改
}
fn change1(s: &mut String) { // s为可修改的
    s.push_str(", world!"); //成功修改
}

fn ref_mut(){
    let mut s = String::from("hello");

    let s1 = &mut s;
    //let s2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    //println!("{}, {}", s1, s2);
    println!("s1: {}", s1);

    // 不在同一作用域中同时指定多个可修改引用变量
    {
        let r1 = &mut s;
        r1.push_str(",world");
        println!("mutable r1:{}", r1);
    }
    let r2 = &mut s;
    r2.push_str("!");
    println!("mutable r2:{}", r2);
}

fn ref_multi(){
    let mut s = String::from("hello");

    let r1 = &s; // ok
    let r2 = &s; // ok
    //let r3 = &mut s; // 大问题 cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("r1:{}, r2:{}", r1,r2);
    //println!("r3:{}", r3);

    let r3 = &mut s; // ok，因此此时知道读的是哪些值
    println!("r3:{}", r3); 

    //let ref_to_nothing = dangle();
    let no_dangle_ = no_dangle();
    println!("s: {}", no_dangle_);
}

// fn dangle() -> &String { // 悬空批量引用   instead, you are more likely to want to return an owned value
//     let s = String::from("hello");
//     &s // 返回指向方法内的字符串引用
// } // 离开作用域，变量s被释放，但返回了一个引用给外部，很危险，会导致访问已释放的空间

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// slice 分片，使用的也是引用

fn slice_ref(){
    let mut s = String::from("hello world");
    let word = first_word_index(&s);
    let first = &s[0..word];
    let first_ = &s[..word];
    println!("the first world: {}({}), {}",first, word, first_);
    let second = &s[word+1..];
    println!("the second world: {}", second);

    //let word = first_word(&s);
    //s.clear(); // 清空内容  cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here
    
    let word = first_word(&s);
    println!("the first word: {}", word);
    s.clear();

    let string_1 = "hello world";
    let word = first_word_f(&s);
    let word = first_word_f(&string_1[..5]);
    let word = first_word_f(&string_1[..]);
    let word = first_word_f(string_1);
    println!("the first word: {}", word);


    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &a[2..3]);
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
        
    &s[..]
}

// 通用的方式，即可传String也可传str
fn first_word_f(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[..i];
        }
    }
    &s[..]
}