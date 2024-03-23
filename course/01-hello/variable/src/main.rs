/**
 * 变量
 * 2024.03.23 by dralee
 */
fn main() {
    variable_1();
    variable_2();
    constant();
    shadowing();
}

fn variable_1() {
    let x = 5;    
    println!("The value of x is: {x}");
    //x = 6;  // 不可变变量不可修改
    //println!("The value of x is: {x}");
}

fn variable_2() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  // mut变量可修改
    println!("The value of x is: {x}");
}

fn constant(){
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("cosntant: {THREE_HOURS_IN_SECOND}");
}

fn shadowing(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "          ";
    let spaces = spaces.len();
    println!("spaces len is: {spaces}");

    let mut spaces = "       ";
    //spaces = spaces.len(); // 类型不匹配，报错
}
