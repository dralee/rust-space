/**
 * 异常处理
 * 2024.03.28 by dralee
 * 忽略异常配置,在Cargo.toml中配置
 * [profile.release]
 * panic = 'abort'
 * 
 * 运行时,可通过设置环境变量RUST_BACKTRACE=1获取错误的详情
 * $ RUST_BACKTRACE= cargo run
 */
use std::{fs::{File, self}, io::{ErrorKind, Read, self}};

fn main() {
    //panic_demo();

    //panic_outbound();

    result_with();
    //result_with_painic();
    diff_error();

    propagating().unwrap_or(String::from("error occur")); // 为了测试多个异常的情况能往下执行,这里使用unwrap_or形式,而不是expect

    shortcut_propagating().unwrap_or_default();//.expect("there is some error");

    shortcut_propagating2().unwrap_or_default();

    short_read_file();

    println!("the last char is: {}", last_char_of_first_line("hello world").unwrap_or_default());
}

fn panic_demo(){
    panic!("crash and burn!");
}

fn panic_outbound(){
    let v = vec![1,2,3];
    v[99];  // index out of bounds: the len is 3 but the index is 99
}

fn result_with(){
    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(v)=> println!("{:?}",v),
        Err(e)=> println!("failure: {:?}", e)
    }
}

fn result_with_painic(){
    let greeting_file_result = File::open("hello.txt");
    let greeing_file = match greeting_file_result {
        Ok(file)=> file,
        Err(e)=> panic!("Problem opening the file: {:?}", e)
    };
}

fn diff_error(){
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file)=> file,
        Err(err)=>match err.kind() {
            ErrorKind::NotFound=> match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e)=> panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // 以下好多match,进行简化
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 使用expect
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");


    // greeting_file.write(String::from("hello contents in the hello.txt").as_bytes())
    //     .expect("failure to write to the file.");
    // greeting_file.flush().expect("failure to flush the file");
}

fn propagating() -> Result<String, io::Error> {
    println!("propagating...");
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e)=> return Err(e) // 将error向上抛
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e)=> Err(e) // 直接向上抛
    }
}

fn shortcut_propagating() -> Result<String, io::Error> {
    println!("shortcut propagating...");
    let mut username_file = File::open("username.txt")?; // 使用?直接返回Error情况
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 全使用?进行简化调用
fn shortcut_propagating2() -> Result<String, io::Error> {
    println!("shoutcut propagating 2 ...");
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 更简单的读文件
fn short_read_file() -> Result<String, io::Error> {    
    println!("shout for read file ...");
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// !!!只允许使用？返回 Result、 Option 或实现FromResidual的其他类型的函数中的。
// 在main不允许直接使用?
// fn main() {
//   let greeting_file = File::open("hello.txt")?;
//    // the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//    // the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
// }

// 可使用 Ok(())解决上述问题
// main可返回任何实现the std::process::Termination trait类型
// use std::error::Error;
// fn main() -> Result<(),  Box<dyn Error>>{  // 返回值为Ok时,则退出码为0,否则非0
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())   // 直接返回 ()
// }