/**
 * http上下文，mvc入口起点
 * 2024.04.19 by dralee
 */
use std::{cell::RefCell, collections::HashMap, io::{self, BufRead, BufReader, Write}, net::TcpStream};

/// HttpContext Http上下文
/// 
pub struct HttpContext {
	headers: HashMap<String, String>,
	path: String,
	host: String,
}
impl HttpContext {
	/// 创建HttpContext对象
	pub fn new(buf: &mut impl io::Read) -> HttpContext {
		//buf.read(buf)
		// if let Some(reqline) = lines.get(0) {
		// 	let items:Vec<&str> = reqline.split(" ").collect();
		// 	println!("items: {:?}", items);
		// }
		
		HttpContext {
			host:"".to_string(),
			path:"x".to_string(),
			headers:HashMap::new()
		}
	}

	/// 响应
	pub fn response(&mut self) -> String {
		let status_line = "HTTP/1.1 200 OK";
		let contents = "<h1>Hello Rust, dralee web context!</h1>";
		let len = contents.len();
		let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
		println!("==>{resp}");
		resp
		// self.stream.write_all(response.as_bytes()).unwrap();
		
		// println!("here....");
		
	}
}