/**
 * grep之mini版功能
 * 2024.03.30 by dralee
 */
use std::env;
use std::process;
use minigrep3::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        //println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}"); // 使用eprintln!输出异常信息
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep3::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // println!当重定向到文件中时,信息不会在控制台输出   cargo run > output.txt
    // eprintln!当重定向输出到文件中时,信息也会输出到控制台中 cargo run > output.txt
}
