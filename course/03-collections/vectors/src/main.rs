use std::vec;

/**
 * vec集合
 * 2024.03.27 by dralee
 */
fn main() {
    vec_demo();

    iter_vec();
    multtype_vec();
}

fn vec_demo() {
    let v:Vec<i32> = Vec::new();
    let v = vec![1,2,3]; // 使用vec宏创建vec集合

    let mut v = Vec::new();
    v.push(1);
    v.push(3);
    v.push(5);
    v.push(2);
    v.push(4);
    v.push(6);
    
    let third:&i32 = &v[2]; // 第3个元素
    println!("The third element is {third}");

    let third:Option<&i32> = v.get(2); // 获取第3个元素
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    let v = vec![1,2,3,4,5];
    //let does_not_exists = &v[100]; // index out of bounds: the len is 5 but the index is 100
                                         //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let does_not_exists = v.get(100); // 不会报错, None
    if let Some(num) = does_not_exists {
        println!("does not exists value: {}", num);
    } else {
        println!("does not exists.");
    }

    //let mut v = vec![1,2,3,4,5];
    //let first = &v[0]; // 指定不可修改的第1元素
    //v.push(6); // 对集合进行修改  cannot borrow `v` as mutable because it is also borrowed as immutable mutable borrow occurs here
    //println!("the first element is :{}", first); // 使用第一个元素的不可修改变量
    // 集合扩充元素时,可能会重新移动内存,导致原来指向第1个元素的内容失效,而导致错误
    
}

fn iter_vec() {
    let v = vec![1,2,3,5];
    for i in &v {
        print!("{i} ");
    }
    println!();

    let mut v = vec![1,2,3,5];
    for i in &mut v {
        *i += 10; // 遍历修改元素值
    }  // For 循环所持有的对向量的引用防止同时修改整个向量
    println!("the second element: {}", &v[1]);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn multtype_vec() {
    // 为了存放多种类型数据,可通过枚举定义后进行存放
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.58),
        SpreadsheetCell::Text(String::from("yes"))
    ];

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.58),
        SpreadsheetCell::Text(String::from("yes"))
    ];
    let c = row.pop();
    match c {
        Some(cell) => {
            match cell {
                SpreadsheetCell::Int(v) => println!("the int value: {}", v),
                SpreadsheetCell::Float(v) => println!("the float value: {}", v),
                SpreadsheetCell::Text(v) => println!("the text value: {}", v),
            }
        },
        None=> println!("it's none!")
    }
    
}