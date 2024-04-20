/**
 * http服务
 * 2024.04.19 by dralee
 */
use std::{cell::RefCell, fmt::Error, io::{BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}};
use crate::httpcontext::HttpContext;
use crate::threadpool::ThreadPool;

pub trait HttpServer {
	/// start the http server by the specified port
	/// 
	/// # panic
	/// start failure
	fn start(&mut self, port: usize) -> Result<(),Error>;

	/// stop the http server
	/// 
	/// # panic
	/// stop failure
	fn stop(&self) -> Result<(), Error>;
}

pub struct Server {
	pool: RefCell<ThreadPool>
}
impl Server {
	/// 创建Server对象
	pub fn new() -> Server {		
		let cpu_nums = num_cpus::get();
		println!("the cpu nums is {cpu_nums}");
		let pool = RefCell::new(ThreadPool::new(4));
		Server {
			pool
		}
	}
}
/// 处理连接
fn handle_connection(mut stream: TcpStream) {
	//let buf_reader = BufReader::new(stream.try_clone().unwrap());
	//let request_line = buf_reader.lines().next().unwrap().unwrap();
	// let lines:Vec<_> = buf_reader.lines().map(|line| line.unwrap()).collect();
	// //println!("line: {}", request_line);
	// println!("lines: {:#?}", lines);
	//println!("line: {}",request_line);

	// let status_line = "HTTP/1.1 200 OK";
	// let contents = "<h1>Hello Rust, dralee web!</h1>";
	// let len = contents.len();
	// let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
	
	let peer_addr = stream.peer_addr().expect("Stream has peer_addr");
	eprintln!("Incoming from {}", peer_addr);
	let mut reader = BufReader::new(stream.try_clone().unwrap());
	let mut writer = BufWriter::new(stream);

	let mut context = HttpContext::new(&mut reader);
	// let response = context.response();
	// writer.write_all(response.as_bytes()).unwrap();
	// writer.flush().unwrap();
	context.response(&mut writer);
	//stream.write_all(response.as_bytes()).unwrap();
	//stream.flush();
	println!("end....");
	// context.response();
	// drop(context)
}

impl HttpServer for Server {
	fn start(&mut self, port: usize) -> Result<(),Error> {
		if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
			println!("server started ::{}", port);
			for stream in listener.incoming() {
				match stream {
					Ok(stream) => {
						self.pool.borrow().execute(||{
							handle_connection(stream);
						});
					}
					Err(e) => {
						println!("accept the incomming stream failure: {:?}", e);
					}
				}
			}
		}
		Err(panic!("start server failure."))
	}

	fn stop(&self) -> Result<(), Error> {
		let p = self.pool.borrow_mut();
		drop(p);
		Ok(())
	}
}