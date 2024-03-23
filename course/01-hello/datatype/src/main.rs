/**
 * 数字类型
 * 2024.03.23 by dralee
 */
fn main() {
    float_1();
    numeric_1();
    bool_1();
    character_1();
    tuple_1();
    array_1();
    
    function_1();
    expression_1();
    control_flow();
}

fn float_1(){
    let x = 2.0; // f64
    let y:f32 = 3.0; // f32

    println!("x:{x}, y:{y}");
}

fn numeric_1(){
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.5; // subtraction
    let product = 5*20; // muliplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // results in -1
    let remainder = 43 % 5; // remainder
    println!("sum:{sum}, difference:{difference}, product:{product}, quotient:{quotient}, truncated:{truncated}, remainder:{remainder}");    
}

fn bool_1(){
    let t = true;
    let f:bool = false; // explicit type annotation
    println!("t: {t}, f: {f}");
}

fn character_1(){
    let c = 'z';
    let z:char = 'ℤ'; // with explicit type annotation
    let heart_eye_cat = '😻';
    println!("c:{c}, z:{z}, heart_eye_cat:{heart_eye_cat}");
}

fn tuple_1(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("tup:{x},{y},{z}");
    println!("tup by index: {five_hundred}, {six_point_four}, {one}");
}

fn array_1(){
    let a = [1,2,3,4,5,6,7];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let b:[i32;5] = [1,2,3,4,5]; // 指定类型跟长度
    let c = [3;5]; // 定义5个3的数组

    let january = months[0];
    let february = months[1];

    print_array(&a, "a");
    print_array(&b, "b");
    print_array(&c, "c");
    println!("{january}");
    println!("{february}");
}

fn print_array(arr: &[i32], msg: &str){
    println!("the array of {msg}:");
    for i in arr {
        print!("{i} ");
    }
    println!();
}

fn function_1(){
    print_labed_measurement(5, 'h');
}

fn print_labed_measurement(value:i32, unit_label:char){
    println!("The measurement is: {value}{unit_label}");
}

fn expression_1(){
    //let x = (let y = 6); // 报错，因为let y=6没有返回值
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}"); // 4

    let sum = sum(3, 8);
    println!("3 + 8 is: {sum}");
}

fn sum(x:i32, y:i32)-> i32{
    //return x + y;
    x + y // 可直接这样返回，最后不要分号
}

fn control_flow(){
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2; // 跳出，并将counter翻倍
        }
    };

    println!("The result is {result}"); // 20

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 跳出标签
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;        
    }
    println!("LIFTOFF!!!");

    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for e in a {
        println!("the value is: {e}");
    }
    println!("range...");
    for number in (1..4).rev(){ // 不含4，包含为:(1..=4)
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}