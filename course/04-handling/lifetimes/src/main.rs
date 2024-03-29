use std::fmt::{self, Display};

/**
 * 生命周期
 * 2024.03.29 by dralee
 */

fn main() {
    valid_ref();

    ref_use();    
    ref_use3();

    lifetime_struct();

    static_lifetime();

    generic_trait_lifetime();
}

/*
fn not_valid_ref(){
    let r;

    {
        let x = 5;
        r = &x;  // `x` does not live long enough borrowed value does not live long enough
    }
    println!("r: {}", r);
}*/

fn valid_ref(){
    let x = 5;
    let r = &x;
    println!("r: {}", r);

}

/* 引用需要指定生命周期参数
// 因为不知道x,y及返回值是否为同一生命周期的引用切片
fn longest(x: &str, y: &str) -> &str { // expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

// 指定splice的生命周期为同一个'a
// 生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn ref_use(){
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long"); // 'a 生命周期
    {
        let string2 = String::from("xyz");             // 'b 生命周期
        let result = longest(&string1, &string2);   // a' 大于 'b
        println!("The longest string is {}", result);
    }
}

/* 生命周期不足
fn ref_use2(){
    let string1 = String::from("long string is long");  // a'
    let result; // b'
    {
        let string2 = String::from("xyz"); // c'
        result = longest(&string1, &string2); // a' 跟 c'不一致,a'>b',而c'<b',虽然肉眼可见的string1比string2长,结果应该是返回string1,跟string2没关系,但编译器看不到这一层
        // `string2` does not live long enough borrowed value does not live long enough
    } // end c'
    println!("The longest string is {}", result);
}*/

// 直接返回x,则与y无关
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
fn ref_use3(){
    let string1 = String::from("long string is long");  // a'
    let result; // b'
    {
        let string2 = String::from("xyz"); // c'
        result = longest2(&string1, &string2); // a' 跟 c'不一致,a'>b',而c'<b'
        // 由于longest2忽略了生命周期问题,因此可能调用成功
    } // end c'
    println!("The longest string is {}", result);
}
/*
fn longest3<'a>(x:&str, y:&str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()   // cannot return value referencing local variable `result`
                      //returns a value referencing data owned by the current function
}*/

struct ImportantExerpt<'a> {
    part: &'a str,
}

impl <'a> ImportantExerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    //有两个输入生命周期，因此 Rust 应用第一个生命周期省略规则，并给出 & self 和公告各自的生命周期。
    //然后，因为其中一个参数是 & self，所以返回类型的生存期为 & self，并且已经计算了所有生存期。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExerpt {
        part: first_sentence
    };

    let a = i.announce_and_return_part("hello");
    println!("level: {}", i.level());
    println!("annouce and return: {}", a);
}

// 省略生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 由于同一切片中切片内容返回,必属于同一切片内的生命周期
        }
    }

    &s[..]
}
/* 这里不需要明确指定同一个lifetime
fn first_word<'a>(s: &'a str) -> &'a str {
    ...
}*/

// 静态生命周期'static,贯穿整个程序
// 所有的字符串字面量都有'static生命周期
fn static_lifetime(){
    let s:&'static str = "I have a static lifetime."; // 该字符串的文本直接存储在程序的二进制文件中，二进制文件总是可用的。因此，所有字符串文字的生命周期是静态的。
    println!("str: {}", s);
}

//泛型参数 特征 生命周期
#[derive(Debug)]
struct Point<T,U> where T: std::fmt::Debug, U: std::fmt::Debug {
    x: T,
    y: U,
}

impl <T,U> Display for Point<T, U>  where T: std::fmt::Debug, U: std::fmt::Debug{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn generic_trait_lifetime(){
    let s1 = String::from("hello");
    let s2 = String::from("what is it");
    let p = Point { x: 10, y: 20};
    let result = longest_with_an_annoucement(&s1, &s2, p);
    println!("the longest text is: {}", result);
}