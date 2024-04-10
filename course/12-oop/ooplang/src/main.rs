/**
 * 面向对象语言特征
 * 泛型上使用特征边界时编译器执行的单态化过程：编译器为每个具体类型生成函数和方法的非泛型实现，
 * 我们用这些函数和方法代替泛型类型参数。单态化产生的代码正在执行静态调度，即编译器在编译时知道
 * 要调用的方法。这与动态调度相反，动态调度是指编译器在编译时无法分辨您正在调用的方法。在动态调
 * 度情况下，编译器发出代码，这些代码将在运行时确定要调用的方法。
 * 
 * 当我们使用 trait 对象时，Rust 必须使用动态调度。编译器不知道可能与使用 trait 对象的代码
 * 一起使用的所有类型，因此它不知道要调用哪个方法在哪个类型上实现。相反，在运行时，Rust 使用
 *  trait 对象内的指针来知道要调用哪个方法。此查找会产生静态调度不会发生的运行时成本。动态调
 * 度还会阻止编译器选择内联方法的代码，这反过来又会阻止某些优化。
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