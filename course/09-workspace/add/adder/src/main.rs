/**
 * 同一workspace中的入口程序
 * 2024.04.01 by dralee
 */
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
