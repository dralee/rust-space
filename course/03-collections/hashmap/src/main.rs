/**
 * Hashmap,跟vec一样存储数据在堆中
 * 2024.03.27 by dralee
 * 
 * 像值类型i32等,放到HashMap中是直接复制副本,而String则是直接移动进去
 * 如果插入的是引用,则值不会被移动,引用指向的值必须至少在哈希映射有效的情况下有效。
 */
use std::collections::HashMap;

fn main() {
    demo();
}

fn demo(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // 获取Blue的分数
    println!("blue: {score}");

    for (key,value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 此时field_name,field_value被移入hashMap中,因此原来变量不可使用
    //println!("name:{}, value:{}", field_name, field_value); // borrow of moved value: `field_value` value borrowed here after move

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 插入同一key的值,后面会覆盖前面的值
    println!("{:?}", scores);

    // 使用entry实现不存在则插入数据
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    // 修改value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() { // 按空格分隔单词
        let count = map.entry(word).or_insert(0); // 不存在则插入0
        *count += 1; // 修改value值
    }
    println!("{:?}", map);
}
