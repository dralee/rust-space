/**
 * 高级函数和闭包
 * fn：函数类型
 * Fn：闭包特征(Fn，FnMut，FnOnce)
 * 2024.04.16 by dralee
 */
fn main() {
    test_func();
    test_closure();

}

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32)-> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_func(){
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn test_closure(){
    let list_of_numbers = vec![1,2,3];
    let list_of_strings:Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect(); // 传递闭包参数
    println!("string: {:?}", list_of_strings);

    let list_of_strings:Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();    // 传递函数参数
    println!("string: {:?}", list_of_strings);

    let list_of_statuses:Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("status: {:?}", list_of_statuses);
    // status: [Value(0), Value(1), Value(2), Value(3), Value(4), Value(5), Value(6), Value(7), Value(8), Value(9), Value(10), Value(11), Value(12), Value(13), Value(14), Value(15), Value(16), Value(17), Value(18), Value(19)]
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

/**
 * 闭包作为返回值
 */
/*fn returns_closure() -> dyn Fn(i32) -> i32 { // return type cannot have an unboxed trait object doesn't have a size known at compile-time
    |x| x + 1
}*/

// 不知道闭包大小，可使用智能指针返回
fn returns_closure() -> Box<dyn Fn(i32)-> i32> {
    Box::new(|x| x + 1)
}