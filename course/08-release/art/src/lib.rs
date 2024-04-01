/*
 * pub use重新导出项目
 * by dralee 2024.04.01
 * 
*/

//! # Art
//! 
//! A library for modeling artistic concepts.


// 为了从公共API中删除内部组织,可以使用pub use重新导出项目
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

/// # kinds
/// 
/// art kinds definitions
///  
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

/// # utils
/// 
/// art utils
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