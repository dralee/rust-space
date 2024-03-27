/**
 * 字符串
 * 2024.03.27 by dralee
 */
fn main() {
    tostring();

    append_str();
    str_access();
}

fn tostring(){
    let data = "initial contents"; // &str类型,splice类型
    let s = data.to_string(); // String类型
    let s = "initial contents".to_string();
    let s = String::from("initial contents"); // String类型

    // UTF-8编码
    let hello = String::from("السلام عليكم");
    println!("> {hello}");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn append_str(){
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1:{s1}, s2:{s2}");

    s1.push('!');
    println!("s1:{s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1被移动到s3,变得不可再使用
    //println!("{s1}"); // borrow of moved value: `s1` value borrowed here after move
    println!("s2:{s2}, s3:{s3}");


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // 移动s1
    println!("s: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format!未移动任何字符串的所有权
    println!("s: {s}");
    println!("s1:{s1}, s3:{s2}, s3:{s3}"); // s1,s2,s3都未被移动

}

fn str_access(){
    let s1 = String::from("hello");
    //let h = s1[0]; // 获取第一个元素,字符串不支持索引
    // the type `String` cannot be indexed by `{integer}`, the trait `Index<{integer}>` is not implemented for `String`
    // the following other types implement trait `Index<Idx>`:

    // String内部使用vec实现
    let hello = String::from("Hola");
    println!("len: {}", hello.len()); // 4
    let hello = String::from("Здравствуйте");
    println!("len: {}", hello.len()); // 24,"Здравствуйте"字节数,每个unicode编码占2字节

    let hello = "Здравствуйте";
    //let answer = &hello[0]; // the type `str` cannot be indexed by `{integer}`
    
    // "नमस्ते"按u8存放为: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,224, 165, 135]
    // "नमस्ते"按Unicode存放字符是:['न', 'म', 'स', '्', 'त', 'े']
    // 这里有六个字符值，但是第四个和第六个不是字母: 它们是本身没有意义的变音符。最后，如果我们把它们看作是字母组合，我们就会得到一个人所称的组成印地语单词的四个字母: ["न", "म", "स्", "ते"]

    // Rust 不允许我们索引字符串以获取字符的最后一个原因是，索引操作总是需要常量时间(O (1))。但是不可能用 String 保证这种性能，因为 Rust 必须遍历从开始到索引的内容，以确定有多少有效字符。

    // 可使用切片,slice实现索引
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {s}");

    // 遍历字符,按字符
    println!("chars:");
    for c in "Зд".chars() {
        println!("{c}");
    }
    
    println!("bytes:");
    // 按字节遍历
    for c in "Зд".bytes() {
        println!("{c}");
    }
}