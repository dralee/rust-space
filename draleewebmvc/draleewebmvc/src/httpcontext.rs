/**
 * http上下文，mvc入口起点
 * 2024.04.19 by dralee
 */
use std::{cell::RefCell, collections::HashMap, io::{self}, rc::Rc};
use crate::streamrw::StreamRW;
use crate::request::Request;

/// HttpContext Http上下文
/// 
pub struct HttpContext {
	headers: Rc<HashMap<Rc<String>, Rc<String>>>,
	path: Rc<String>,
	host: Rc<String>,
	request: Rc<Request>,
}
impl HttpContext {
	/// 创建HttpContext对象
	pub fn new(mut buf: &mut impl io::Read) -> HttpContext {
		// match StreamRW::read_str(&mut buf) {
		// 	Ok(content) => {
		// 		println!("the content: {content}");
		// 	},
		// 	Err(e) => println!("error: {:?}", e),
		// };
		let data = StreamRW::read(buf);
		let request = Request::new(data);
		println!("request: {:#?}", request);
		
		HttpContext {
			host: Rc::clone(&request.host),
			path: Rc::clone(&request.path),
			headers: Rc::clone(&request.headers),
			request: Rc::new(request)
		}
	}

	/// 响应
	pub fn response(&mut self, writer: &mut impl io::Write) {
		let status_line = "HTTP/1.1 200 OK";
		let contents = "<h1>Hello Rust, dralee web context2222!</h1>";
		let len = contents.len();
		let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
		println!("==>{resp}");
		//resp
		// self.stream.write_all(response.as_bytes()).unwrap();
		
		// println!("here....");
		writer.write_all(resp.as_bytes()).unwrap();
		writer.flush().unwrap();
	}
}