/**
 * 迭代器
 * 2024.03.30 by dralee
 * 
 * 迭代器是懒加载的
 */
fn main() {
    iter1();
    iter2()

}

fn iter1(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for value in v1_iter{
        println!("Got: {}", value);
    }
}

fn iter2(){
    let v1:Vec<i32> = vec![1,2,3];
    v1.iter().map(|x| x + 1);
}