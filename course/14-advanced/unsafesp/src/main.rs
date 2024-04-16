/**
 * 不安全超能力
 * 2024.04.15 by dralee
 * 
 * 使用unsafe启动一个包含不安全代码新块，在不安全的Rust中采取五种在安全Rust中不能采取
 * 的行动：取消引用原始指针；调用不安全函数或方法；访问或修改可变静态变量；实现不安全特征；union S的访问字段
 * unsafe 这不会关闭借用检查器或禁用 Rust 的任何其他安全检查：如果你在不安全的代码中使用引用，它仍然会被检查。
 * 该 unsafe 关键字仅允许您访问这五个功能，然后编译器不会检查这些功能以确保内存安全。在不安全的街区内，您仍然
 * 可以获得一定程度的安全性。
 */
fn main() {
    test_dereferencing_raw_pointer();
    unsafe_normal();
    unsafe_normal2();

    test_extern();
    test_global_static();
    test_union();
}

/**
 * 取消引用原始指针
 * 原始指针：允许通过同时具有不可变和可变指针或指向同一位置的多个可变指针来忽略借用规则；
 *   不保证指向有效内存；允许为null；不要实施任何自动清理。
 */
fn test_dereferencing_raw_pointer(){
    // 从引用创建不可变和可变原始指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 value: {}", *r1);
        *r2 = 8;
        println!("r1 value: {}, r2 value: {}, num: {}", *r1, *r2, num);
    }
    
    //dangerous(); // this operation is unsafe and requires an unsafe function or block

    unsafe {
        dangerous();
    }
}

/**
 * 不安全的函数，必须在unsafe块中调用，不然会报：this operation is unsafe and requires an unsafe function or block
 */
unsafe fn dangerous() {

}

/**
 * 在不安全代码上创建安全抽象
 */
fn unsafe_normal(){
    let mut v = vec![1,2,3,4,5,6,7,8];

    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6,7,8]);
}

/*
Rust 的借用检查器无法理解我们正在借用切片的不同部分;它只知道我们从同一个切片借了两次。从根本上说，借用切片的不同部分是可以的，因为这两个切片没有重叠，
但 Rust 还不够聪明，无法知道这一点。当我们知道代码没问题，但 Rust 没有，是时候接触不安全的代码了。
fn split_at_mut(values: &mut [i32], mid: usize)->(&mut [i32], &mut [i32]){
    let len = values.len();
    assert!(mid <= len);
    (&mut values[..mid], &mut values[mid..]) // cannot borrow `*values` as mutable more than once at a time second mutable borrow occurs here
} */

use std::slice;

/**
 * 不安全代码返回安全抽象
 */
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(len >= mid);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }
}

fn unsafe_normal2(){    
    println!("unsafe split...");
    let mut v = vec![1,2,3,4,5,6,7,8];
    // let (a,b) = split_at_mut(v, 4);
    // assert_eq!(a, &mut [1,2,3,4]);
    // assert_eq!(a, &mut [5,6,7,8]);
}

/*
 * 使用extern函数调用外部代码
 * 调用C标准库中abs函数集成，extern声明函数从Rust代码调用总是不安全的
 */
extern "C" { // "C"ABI是最常见的，遵循C编程语言的ABI
    fn abs(input: i32) -> i32;
}
fn test_extern(){
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

/**
 * 提供给其他程序调用的函数
 */
#[no_mangle] // 禁止Rust编译器名称更改
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function from C!");
}

/**
 * 访问或修改可变静态变量
 */
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32){
    unsafe {
        COUNTER += inc;
    }
}

fn test_global_static(){
    println!("wellcome word: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

/**
 * 实现不安全特征
 */
unsafe trait Foo {
    
}

unsafe impl Foo for i32 {
    
}

/**
 * 访问联合字段
 * 访问union字段是不安全的，Rust无法保证当前存储在union实例中数据类型
 * 联合的关键属性是联合的所有字段共享存储，因此，对联合一个字段写入可以覆
 *  盖其他字段，且联合的大小由其最大字段的大小决定。
 * 
 * 联合字段仅限于类型子集：
 *  Copy类型；
 *  References(&T和&mut T)；
 *  ManuallyDrop<T>；
 *  仅包含允许的联合字段的元组和数组
 */
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32
}
fn test_union(){
    let u = MyUnion { f1: 1};
    let f = unsafe { u.f1 };
    println!("f is {f}");
    
    let mut u = MyUnion { f1: 1};
    u.f2 = 3.5f32;
    let f2 = unsafe { u.f2 };
    println!("f2 is {f2}");
}