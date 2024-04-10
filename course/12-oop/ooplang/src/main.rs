/**
 * 面向对象语言特征
 * 2024.04.10 by dralee
 */
fn main() {
    test();
    test_draw();
}

use ooplang::{AveragedCollection, Button,SelectBox,Screen};

fn test(){
    let mut av = AveragedCollection::new();
    av.add(1);
    av.add(8);

    println!("average: {}", av.average());
    av.add(3);
    av.add(9);
    println!("average: {}", av.average());
    av.remove();
    println!("average: {}", av.average());
}

fn test_draw(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ]
    };

    screen.run();
}