use std::fmt::{Debug, Display};

/**
 * trait(特性)相当于interface
 */

 // 定义接口
pub trait Summary {
	fn summarize(&self) -> String;

	// 默认实现
	fn short_summarize(&self) -> String {
		String::from("Read more...")
	}

	fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

// NewsArticle文章实现Summary特征
impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}

	fn summarize_author(&self) -> String {
		format!("@{}", self.author)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}
// 实现Summary特征
impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}

	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}

pub fn hello(){
	println!("hello");
}

// trait作为参数
// 将Summary的实现类型作为参数
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

// 这是可实现Summary的不同类型的
pub fn notify2(item1: &impl Summary, item2: &impl Summary){
	println!("notify2: {},{}", item1.summarize_author(), item2.summarize_author());
}

// 如果需要两个参数都必须是实现Summary的同一类型
pub fn notify3<T:Summary>(item1:&T, item2:&T){
	println!("notify3: {},{}", item1.summarize_author(), item2.summarize_author());
}

// 必须同时实现Summary跟Display的特征
pub fn notify4(item: &(impl Summary + Display)){
	println!("notify4: {}", item.summarize_author());
}

// 也可为泛型
pub fn notify5<T: Summary + Display>(item:&T){
	println!("notify5: {}", item.summarize_author());
}

// 这样比较复杂
pub fn some_function<T: Display+Clone, U:Clone+Debug>(t: &T, u:&U)->i32{
	println!("T & U");
	100
}
// 简化条件为where
pub fn some_function2<T,U>(t:&T, u:&U) -> i32 where T: Display + Clone, U: Clone + Debug {
	println!("T & U by where");
	120
}

// 返回值为实现了Summary特征类型
pub fn returns_summarizable() -> impl Summary {
	Tweet {
		username: String::from("hourse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
	}
}

// 虽然都实现Summary,但不允许if else中返回不同类型
// `if` and `else` have incompatible types
//expected `Tweet`, found `NewsArticle`
// pub fn returns_summarizable2(switch: bool) -> impl Summary {
// 	if switch {
// 		Tweet {
// 			username: String::from("hourse_ebooks"),
// 			content: String::from("of course, as you probably already know, people"),
// 			reply: false,
// 			retweet: false
// 		}
// 	} else {
// 		NewsArticle {
// 			headline: String::from("Penguins win the Stanley Cup Championship!"),
// 			location: String::from("Pittsburgh, PA, USA"),
// 			author: String::from("Iceburgh"),
// 			content: String::from("The Pittsburgh Penguins once again are the best \
// 			hockey team in the NHL.")
// 		}
// 	}
// }

struct Pair<T> {
	x: T,
	y: T
}

impl <T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y}
	}
}
impl <T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

// impl <T :Display> ToString for T {
// 	fn to_string(&self) -> String {
// 		format!("({:?})", self)
// 	}	
// }