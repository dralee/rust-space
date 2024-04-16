/**
 * 高级-自定义宏
 * 使用macro_rules!规则声明宏、及3种过程宏：
 *  >自定义#[derive]达观，用于指定使用结构和枚举上使用的derive属性添加代码；
 *  >类似属性的宏，用于定义可用于任何项的自定义属性；
 *  >类似函数的宏，看起来像函数调用，但对指定为其参数的标记进行操作
 * 2024.04.16 by dralee
 */
pub trait HelloMacro {
    fn hello_macro();
}
