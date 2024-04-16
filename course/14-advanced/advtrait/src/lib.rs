/**
 * 高级特征
 * 2024.04.16 by dralee
 */
pub trait Iterator {
	type Item; // 具有关联类型特征，Item是一个占位符

	fn next(&mut self) -> Option<Self::Item>;
}
struct Counter {
	data: Vec<u32>,
	index: usize,
}
/*
与泛型不同的，使用泛型时：必须对每个实现中的类型进行注释；因为我们也可
	实现Iterator<String> for Counter或任何其他类型，所以可以有多个
	Iterator for Counter的实现。当特征具有泛型参数时，可以多次为类型
	实现该特征，每次都要更改泛型类型参数具体类型。当使用Counter的next
	方法时，必须提供类型注释来指示我们想要使用的Iterator实现。

对于关联的类型，不需要对类型进行注释，因为不能多次在一个类型上实现一个特征。
	使用关联类型定义，只能选择一次的Item类型，因只能有一个impl Iterator for
	Counter。调用Counter的next方法的任何地方，不必指定想要一个值的迭代器u32。
*/
impl Iterator for Counter {
	type Item = u32;	
	fn next(&mut self) -> Option<Self::Item> {
		let len = self.data.len();
		assert!(len > self.index);
		self.index += 1;
		
		if let Some(&i) = self.data.get(self.index) {
			return Some(i);
		}
		None
	}
}

/**
 * 操作符重载
 * 可通过实现与运算符关联的std::ops列出的操作和相应的特征来重载操作符
 */
use std::ops::Add;
#[derive(Debug,Copy,Clone,PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

impl Add for Point {
	type Output = Point;

	fn add(self, other: Self) -> Self::Output {
		Point {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

struct Millimeters(u32);
struct Meters(u32);

// 使用默认类型参数：不破坏现有代码情况下扩展类型；为了允许大多数用户不需要的特定情况下进行自定义
impl Add<Meters> for Millimeters {
	type Output = Meters;

	fn add(self, rhs: Meters) -> Self::Output {
		Meters(self.0 + (rhs.0*1000))
	}
}
impl ToString for Meters {
	fn to_string(&self) -> String {
		String::from(format!("{} meters", self.0))
	}
}

pub fn test_ops(){
	assert_eq!(
		Point{x: 1, y: 0} + Point { x: 2, y: 3},
		Point{x: 3, y: 3}
	);

	let m = Meters(10);
	let mil = Millimeters(120);
	let milm = mil + m;
	println!("millmeters + meters: {}", milm.to_string());
}

/**
 * 用于消除歧义的完全限定语法：调用同名方法
 * Pilot、Wizard特征都具有fly相同方法名签名，同时Human自己也含有该方法名定义
 */
trait Pilot {
	fn fly(&self);
}
trait Wizard {
	fn fly(&self);
}

struct Human;

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking.");
	}
}
impl Wizard for Human {
	fn fly(&self) {
		println!("Up!");
	}
}
impl Human {
	fn fly(&self) {
		println!("*waving arms furiously*");
	}
}

pub fn test_same(){
	let person = Human;
	person.fly(); // 调用的是默认的Human类型自己的fly()
	Human::fly(&person); // 调用Human的fly() 等价于 person.fly()

	Pilot::fly(&person); // 限定调用Pilot的fly()
	Wizard::fly(&person); // 限定调用Wizard的fly()
}

trait Animal {
	fn baby_name() -> String;
}
struct Dog;

impl Dog {
	fn baby_name() -> String {
		String::from("Spot")
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		String::from("puppy")
	}
}

/*
完全限定语法：<Type as Trait>::function(receiver_if_method, next_arg, ...);
*/
pub fn test_same2(){
	println!("A baby dog is called a {}", Dog::baby_name()); // 调用的是Dog自己类型中的baby_name(): Spot
	//println!("A baby dog is called a {}", Animal::baby_name()); // cannot call associated function on trait without specifying the corresponding `impl` type
																// cannot call associated function of trait
	println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // 指定限定为调用Dog的实现特征Animal的baby_name()
}

/**
 * 使用超特征要求一个特征在另一个特征中的功能
 * */
use std::fmt;

trait OutlinePrint: fmt::Display { // 指定需要实现fmt::Display特征
	fn outline_print(&self) {
		let output = self.to_string();
		let len = output.len();
		println!("{}","*".repeat(len + 4));
		println!("*{}*"," ".repeat(len + 2));
		println!("* {} *", output);
		println!("*{}*", " ".repeat(len + 2));
		println!("{}", "*".repeat(len+4));
	}
}

struct Point2 {
	x: i32,
	y: i32,
}
/*
`Point2` doesn't implement `std::fmt::Display`
the trait `std::fmt::Display` is not implemented for `Point2`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead */
impl OutlinePrint for Point2 {
}

/*
解决上面问题，是实现fmt::Display特征 */
impl fmt::Display for Point2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

pub fn test_outline(){
	let p = Point2{ x: 5, y: 8};
	println!("{}", p);
	p.outline_print();
}

/**
 * 使用NewType模式在外部类型上实现外部特征
 */
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", self.0.join(", "))
	}
}

pub fn test_wrapper(){
	let w = Wrapper(vec![String::from("hello"), String::from("world")]);
	println!("w = {}", w);
}