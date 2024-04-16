/**
 * 高级类型
 * 2024.04.16 by dralee
 */
struct A{

}
 /**
  * 使用Newtype模式实现类型安全和抽象
  * 创建具有类型别名的类型同义词
  */
type Kilometers = i32; // 使用这种方式，无法获得从nettype模式中获得的类型检查优势

type Thunk = Box<dyn Fn() + Send + 'static>; // 别名简化类型

fn takes_long_type(f: Thunk) {

}
fn returns_long_type() -> Thunk {
	Box::new(|| println!("return long type!"))
}

pub fn test_type(){
	let x:i32 = 5;
	let y: Kilometers = 5;

	println!("x + y = {}", x + y);

	let f: Thunk = Box::new(|| println!("hi"));
	takes_long_type(f);
	
}

/**
 * 永远不返回的类型, never type
 * 永不返回
 * continue、panic!、loop（不含break）
 */
fn bar() -> ! {
	loop {
		
	}
}

/*
 * 动态大小的类型和Sized特征
 */
