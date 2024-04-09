/**
 * 包、模块、使用
 * 2024.03.26 by dralee
 */
use crate::garden::vegetables::Asparagus;

pub mod garden; // 在包含garden中找代码

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {:?}!", plant);
}
