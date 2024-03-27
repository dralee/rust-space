/**
 * Hashmap,跟vec一样存储数据在堆中
 * 2024.03.27 by dralee
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
}
