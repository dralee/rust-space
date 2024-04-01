/* 
* 自定义智能指针
* 2024.04.01 by dralee
*/
fn main() {
    test();

    test2();
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customSmartPointer with data `{}`!", self.data);
    }
}

fn test(){
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    // 调用drop是逆序调用的
    /*
CustomSmartPointer created.
Dropping customSmartPointer with data `other stuff`!
Dropping customSmartPointer with data `my stuff`! 
*/
}

// 手动调用强制释放内存
fn test2(){
    let c = CustomSmartPointer {
        data: String::from("data here"),
    };
    println!("CustomSmartPointer created.");
    //c.drop(); // 无法直接调用 cannot mutate immutable variable `c`
    drop(c); // 使用core::mem::drop进行释放
    println!("CustomSmartPointer dropped before the end of main.");
    /*
CustomSmartPointer created.
Dropping customSmartPointer with data `data here`!
CustomSmartPointer dropped before the end of main.
     */
}