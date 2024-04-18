/**
 * 派生宏帮助者属性
 * 派生宏可将其他属性添加到它们所在的项的范围内。唯一目的是输入到定义它们的
 *  派生宏中，即所有宏都可以看到它们。
 * 2024.04.18 by dralee
 */
extern crate demohelperattr;
use demohelperattr::HelperAttr;


#[derive(HelperAttr)]
struct Struct {
   #[helper] field: ()
}

fn main() {
    let s = Struct { field: ()};
    
}

