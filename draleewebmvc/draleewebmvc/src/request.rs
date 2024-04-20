/**
 * 请求对象
 * 2024.04.20 by dralee
 */
use std::{collections::HashMap};

///
/// http请求实体
/// 
#[derive(Debug)]
pub struct Request {
	pub host: String,
	pub url: String,
	pub path: String,
	pub query: String,
	pub content_type: String,
	pub user_agent: String,
	pub headers: HashMap<String, String>,
	pub method: String,
	pub body: Vec<u8>,
}

impl Request {
	/// 创建请求对象
	pub fn new(data: Vec<u8>) -> Request {
		let mut lines = vec![];
		let mut buffer = vec![];
		let rt = "\r\n".as_bytes();
		let buf_len = data.len();

		for i in 0..buf_len {
			let c = data[i];
			if buffer.len() > 0 && c == rt[0] && i < buf_len - 1 && data[i+1] == rt[1] {
				if let Ok(line) = String::from_utf8(buffer) {
					lines.push(line);
				} else {
					eprintln!("couldn't line buffer convert to string.");
				}
				buffer = vec![];
				continue;
			}
			buffer.push(c);
		}
		
						
		Request::resolve(lines, buffer)
	}

	fn resolve(lines: Vec<String>, buffer: Vec<u8>) -> Request {
		let head_line_items:Vec<&str> = lines[0].split(' ').collect();
		let url = head_line_items[1];
		let method = head_line_items[0];
		let mut path = String::new();
		let mut query = String::new();
		if let Some(index) = url.find('?') {
			path = String::from(&url[..index]);
			query = String::from(&url[index+1..]);
		}

		let mut headers = HashMap::new();
		
		let mut bodys = vec![];
		let mut is_bodys = false;
		let mut host = String::from("");
		let mut user_agent = String::from("");
		let mut content_type = String::from("");
		println!("lines: {:?}", lines);
		for i in 1..lines.len() {
			let line = &lines[i];
			if is_bodys {
				bodys.push(line);
			}
			if line == "\r\n" {
				is_bodys = true;
				continue;
			}
			let hs:Vec<&str> = line.split(':').collect();
			if hs.len() >= 2 {
				let key = hs[0].trim();
				let mut val = hs[1].trim();
				let mut val_str = String::new();
				if hs.len() > 2 {
					val_str = hs[1..].join(":");
					val = &val_str[..];
				}
				
				match key {
					"Host" => host = val.to_string(),
					"User-Agent" => user_agent = val.to_string(),
					"Content-Type" => content_type = val.to_string(),
					_ => ()
				}

				headers.insert(key.to_string(), val.to_string());
			}
		}

		Request {
			headers,
			path,
			method: method.to_string(),
			query,
			body: buffer,
			host: host,
			url: url.to_string(),
			user_agent,
			content_type,
		}
	}
}