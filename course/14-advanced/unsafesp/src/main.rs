/**
 * 不安全超能力
 * 2024.04.15 by dralee
 * 
 * 使用unsafe启动一个包含不安全代码新块，在不安全的Rust中采取五种在安全Rust中不能采取
 * 的行动：取消引用原始指针；调用不安全函数或方法；访问或修改可变静态变量；实现不安全特征；union S的访问字段
 */
fn main() {
    println!("Hello, world!");
}
