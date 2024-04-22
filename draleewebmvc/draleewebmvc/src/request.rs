/**
 * 请求对象
 * 2024.04.20 by dralee
 */
use std::collections::HashMap;
use std::rc::Rc;

///
/// http请求实体
/// 
#[derive(Debug)]
pub struct Request {
	pub host: Rc<String>,
	pub url: Rc<String>,
	pub path: Rc<String>,
	pub query_string: Rc<String>,
	pub query: Rc<HashMap<Rc<String>, Vec<Rc<String>>>>,
	pub content_type: Rc<String>,
	pub user_agent: Rc<String>,
	pub headers: Rc<HashMap<Rc<String>, Rc<String>>>,
	pub method: Rc<String>,
	pub body: Rc<Vec<u8>>,
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

	/// 解析query参数
	fn resolve_query(query_string: &str) -> HashMap<Rc<String>, Vec<Rc<String>>> {
		let mut query = HashMap::new();
		if query_string.len() == 0 {
			return query;
		}
		query_string.split('&').for_each(|s|{
			let items:Vec<&str> = s.split('=').collect();
			let mut value:String = String::from("");
			let key = items[0].to_string();
			if items.len() >= 1 {				
				if items.len() > 2 {
					value = items[1..].join("=");
				} else {
					value = items[1].to_string();
				}
			}

			query.entry(Rc::new(key)).or_insert_with(Vec::new).push(Rc::new(value));
		});

		query
	}

	/// 解决请求行内容
	fn resolve(lines: Vec<String>, buffer: Vec<u8>) -> Request {
		let head_line_items:Vec<&str> = lines[0].split(' ').collect();
		let url = head_line_items[1];
		let method = head_line_items[0];
		let mut path = String::new();
		let mut query_string = String::new();
		if let Some(index) = url.find('?') {
			path = String::from(&url[..index]);
			query_string = String::from(&url[index+1..]);
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

				headers.insert(Rc::new(key.to_string()), Rc::new(val.to_string()));
			}
		}

		let query = Request::resolve_query(&query_string);

		Request {
			headers: Rc::new(headers),
			path: Rc::new(path),
			method: Rc::new(method.to_string()),
			query_string: Rc::new(query_string),
			query: Rc::new(query),
			body: Rc::new(buffer),
			host: Rc::new(host),
			url: Rc::new(url.to_string()),
			user_agent: Rc::new(user_agent),
			content_type: Rc::new(content_type),
		}
	}
}