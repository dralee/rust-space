
use refcellmut::{self, LimitTracker, Messager};
use std::cell::RefCell;

/*
// 由于不可变原因，无法使用
struct MockMessager {
	sent_messages: Vec<String>,
}
impl MockMessager {
	fn new() -> MockMessager {
		MockMessager {
			sent_messages: vec![],
		}
	}
}
impl Messager for MockMessager {
	fn send(&self, msg: &str) { // 由于send是Messager特征，而定义为不可变的，也无法直接修改为&mut self，此时可使用内部可变性
		self.sent_messages.push(String::from(msg)); // cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
											//`self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
	}
}*/

struct MockMessager {
	sent_messages: RefCell<Vec<String>>,
}
impl MockMessager {
	fn new() -> MockMessager {
		MockMessager {
			sent_messages: RefCell::new(vec![]),
		}
	}
}
impl Messager for MockMessager {
	fn send(&self, msg: &str) {
		/*let mut borrow1 = self.sent_messages.borrow_mut();
		let mut borrow2 = self.sent_messages.borrow_mut();
		borrow1.push(String::from(msg));
		borrow2.push(String::from(msg)); */ // 编译通过但运行时错误：already borrowed: BorrowMutError
										 //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

		self.sent_messages.borrow_mut().push(String::from(msg));// 使用RefCell可直接借用为mut
	}
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
	let mock_messager = MockMessager::new();
	let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

	limit_tracker.set_value(80);

	assert_eq!(mock_messager.sent_messages.borrow().len(), 1); // 由于不需要修改，为只读，只需要borrow()
}
