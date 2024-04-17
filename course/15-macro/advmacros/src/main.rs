/**
 * 高级-宏
 * 使用macro_rules!规则声明宏、及3种过程宏：
 *  >自定义#[derive]达观，用于指定使用结构和枚举上使用的derive属性添加代码；
 *  >类似属性的宏，用于定义可用于任何项的自定义属性；
 *  >类似函数的宏，看起来像函数调用，但对指定为其参数的标记进行操作
 * 2024.04.16 by dralee
 */
fn main() {
    test_custom_macro();


}

/**
 * 宏和函数的区别：
 *  宏是一种编写其他代码的代码方式，被称为元编程。
 *  元编程对于减少必须编写和维护的代码量很有用，这也是函数作用之一。但宏具有函数
 *   所没有的一些附加功能。
 *  函数签名必须声明函数具有参数数量和类型。宏可采用可变数量参数：可用一个参数或
 *   println!("hello {}",name)两个参数调用println!("hello")。此外，编译器
 *   解释代码含义前，宏会展开，因此宏可以在给定类型上实现特征。函数不能，因它在运
 *   行时被调用，且碕在编译时实现特征。
 *  宏的缺点：定定义比函数定义更复杂，由于编写编写Rust代码的Rust代码。由于间接性，
 *   宏定义通常比函数定义更难阅读/理解/维护。
 *  宏和函数间重要区别，在文件中调用宏之前，必须定义或将其纳入范围，而不像函数可以在
 *   任何地方定义和调用函数。
 */
// 使用定义宏
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Pancakes2;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn test_custom_macro(){
    Pancakes::hello_macro();
    Pancakes2::hello_macro();
}