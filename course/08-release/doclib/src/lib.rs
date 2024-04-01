/*
 * 文档化注释
 * by dralee 2024.04.01
 * 
 * cargo doc --open
 * cargo test,会执行文档中测试示例
*/
//! 添加到包含注释的项目(针对的是整个项目,而不是某行代码或是方法)
//! # doclib
//! 
//! `doclib` is a collection of utilities to make performing certain
//! calculations more convenient

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = doclib::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x:i32) -> i32 {
    x + 1
}


pub mod kinds {
    #[derive(Debug)]
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug)]
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {

        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Yellow => SecondaryColor::Orange,
                PrimaryColor::Blue => SecondaryColor::Purple,
                PrimaryColor::Red=> SecondaryColor::Orange, // 没有对应色
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Red => SecondaryColor::Orange,
                PrimaryColor::Blue => SecondaryColor::Green,
                PrimaryColor::Yellow => SecondaryColor::Green, // 没有对应色
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => SecondaryColor::Purple,
                PrimaryColor::Yellow => SecondaryColor::Green,
                PrimaryColor::Blue => SecondaryColor::Purple, // 没有对应色
            }
        }
    }
}