/**
 * grep之mini版功能
 * 2024.03.30 by dralee
 */
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // 如果需要为unicode编码的参数,则用std::env::args_os
    //dbg!(args);
//  args = [
//     "/mnt/f/workspace/rust-space/course/target/debug/minigrep",
//     "lee",
//     "demo-file.txt",
// ]
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    println!("With text: \n{contents}");

}
