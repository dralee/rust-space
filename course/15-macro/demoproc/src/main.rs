/**
 * (proc_macro)函数宏示例
 * 2024.04.17 by dralee
 */

 extern crate demoproc;
 use demoproc::make_answer;

 make_answer!();


fn main() {
    println!("{}", answer());
}

