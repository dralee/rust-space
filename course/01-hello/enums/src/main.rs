use core::num;

/**
 * 枚举
 * 2024.03.26 by dralee
 */

fn main() {
    demo();

    message_enum();
    option_enum();
    match_enum();

    plus_match();

    enum_if_let();
}

enum IpAddrKind {
    V4,
    V6
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum IpAddr2 {
    V4(String),
    V6(String)
}

enum IpAddr3 {
    V4(u8,u8,u8,u8),
    V6(String)
}

struct Ipv4Addr {
}
struct Ipv6Addr {
}
enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

fn demo(){
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    //println!("{},{}", four, six);

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
    
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32,i32,i32); // tuple struct

// 可通过impl对enum进行实现
impl Message {
    fn call(&self) {
        
    }
}

fn message_enum(){
    let m = Message::Write(String::from("hello"));
    m.call();
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn option_enum(){
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number:Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> = Some(5);
    //let sum = x + y; // cannot add `Option<i8>` to `i8`
    //the trait `Add<Option<i8>>` is not implemented for `i8`
    //the following other types implement trait `Add<Rhs>`:
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn match_enum (){
    let coin = Coin::Dime;
    let r = value_in_cents(coin);
    println!("coin in cents: {}", r);

    let coin = Coin1::Quarter(UsState::Alaska);
    let r = value_in_cents2(coin);
    println!("coin in cents: {}", r);

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}
enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents2(coin: Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 匹配到有值则值加1，否则返回none
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //None => None, // missing match arm: `None` not covered 如果有哪个值没覆盖到会报错
        None => None,
        Some(i) => Some(i+1),
    }
}

fn plus_match(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other=> move_player(other) // 其他值，需要使用情况
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _=> reroll() // 其他值，不需要使用情况
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _=> () // 其他值，不需要使用情况，同时也不需要处理
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_spaces: u8){
    println!("move player {}", num_spaces);
}
fn reroll(){}

// if let形式
fn enum_if_let(){
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _=> () // 丢弃None情况
    }

    // 可简化为
    if let Some(max) = config_max { // 只接收Some(T)形式，直接丢弃None形式
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin:Coin1 = Coin1::Nickel;
    match coin {
        Coin1::Quarter(state) => println!("State quarter from {:?}!", state),
        _=> count += 1,
    }
    
    // 简化为
    let mut count = 0;
    let coin:Coin1 = Coin1::Nickel;
    if let Coin1::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }else{
        count += 1;
    }
}