/**
 * 结构
 * 2024.03.25 by dralee
 */

fn main() {
    demo();

    nick_struct();

    struct_method();
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn demo(){
    let user1 = User{
        active: true,
        username: String::from("somebody123"),
        email: String::from("somebody123@demo.com"),
        sign_in_count: 1,
    };
    println!("the user: {}", user1.username);
    //user1.email = String::from("1@2.com"); // cannot mutate immutable variable `user1`

    
    let mut user1 = User{
        active: true,
        username: String::from("somebody123"),
        email: String::from("somebody123@demo.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("1@2.com"); 
    println!("the user: {}", user1.email);

    let user2 = build_user(String::from("1265@12.com"), String::from("somebody123"));
    println!("user: {}, {}", user2.username, user2.email);

    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("abc@123.com"),
        sign_in_count: 1
    }; // 由于user2的username移动到了user3，因此后面不可再使用user2的username
    println!("user3: {},{}", user3.username, user3.email);
    println!("user2: {}", user2.email);

    let user4 = User {
        email: String::from("a2@123.com"),
        ..user3 // 直接使用user2的属性填充其它属性
    }; // 由于user3的username移动到了user4，因此后面不可再使用user3的username
    println!("user4: {},{}", user4.username, user4.email);

}

fn build_user(email: String, username: String)-> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    } 
}

// 可使用元组形式定义struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 无字段结构（unit type，单元型）
struct AlwaysEqual;

fn nick_struct(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("color: {}.{}.{}", black.0, black.1, black.2);
    println!("origin: {}", origin.0);

    let subject = AlwaysEqual;
    let subject2 = AlwaysEqual;
//    println!("equals: {}", subject. subject2);

    let rect1 = Rectange {
        width: 30,
        height: 62,
    };

    println!("The area of the rectange is {} square pixels.", area(&rect1));
    println!("rect1: is {:?}", rect1); // 需要添加#[derive(Debug)]  
        // rect1: is Rectange { width: 30, height: 62 }
    println!("rect1: is {:#?}", rect1);
//rect1: is Rectange {
//    width: 30,
//    height: 62,
//}
    dbg!(&rect1); // dbg!有返回值
//[01-hello/struct_rd/src/main.rs:99:5] &rect1 = Rectange {
//     width: 30,
//     height: 62,
//}
    let scale = 2;
    let rect2 = Rectange{
        width: dbg!(30 * scale),
        height: 62
    };
    dbg!(&rect2);
// [01-hello/struct_rd/src/main.rs:106:16] 30 * scale = 60
// [01-hello/struct_rd/src/main.rs:109:5] &rect2 = Rectange {
//     width: 60,
//     height: 62,
// }

}

#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

fn area(rectange: &Rectange) -> u32 {
    return  rectange.width * rectange.height;
}

// 方法语法
#[derive(Debug)]
struct Rectange2 {
    width: u32,
    height: u32,
}

impl Rectange2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn struct_method(){
    let rect1 = Rectange2 {
        width: 30,
        height: 62,
    };

    println!(
        "The area of the rectangle is {} sequare pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}