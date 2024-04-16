/**
 * 高级-宏
 * 使用macro_rules!规则声明宏、及3种过程宏：
 *  >自定义#[derive]达观，用于指定使用结构和枚举上使用的derive属性添加代码；
 *  >类似属性的宏，用于定义可用于任何项的自定义属性；
 *  >类似函数的宏，看起来像函数调用，但对指定为其参数的标记进行操作
 * 2024.04.16 by dralee
 */
// 简单定义vec!宏
#[macro_export] // 指示每当定义宏的crate进入范围时，都应提供此宏。如没此注解，则无法将宏纳入范围
macro_rules! vec {
    (  $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
/*
用vec![1,2,3]，此宏生成替换调用如下：
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
*/
/*
宏的第二种形式是过程宏，它的作用更像是一个函数（并且是一种过程）。
过程宏接受一些代码作为输入，对该代码进行操作，并生成一些代码作为输出，
而不是像声明性宏那样与模式匹配并用其他代码替换代码。这三种过程宏是自定义派生宏、
类属性宏和函数宏，它们都以类似的方式工作。
*/
/*
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {

}*/