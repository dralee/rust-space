/**
 * 流读取、写入
 * 2024.04.19 by dralee
 */
use std::io::{self, Read, Result};

const BUFFER_SIZE:usize = 1024;

pub struct StreamRW {
}
impl StreamRW {
	///
	/// 读取Stream（流）到 u8缓冲
	/// 
	pub fn read(buf: &mut impl Read) -> Vec<u8> {
		let mut received: Vec<u8> = vec![];
		
		let mut buffer = [0u8; BUFFER_SIZE];
		loop {
			let bytes_read = buf.read(&mut buffer).unwrap();
			received.extend_from_slice(&buffer[..bytes_read]); // 扩展到接收缓冲中

			if bytes_read < BUFFER_SIZE {
				break;
			}			
		}
		
		received
	}

	/// 读取Stream到字符串
	pub fn read_str(mut buf: &mut impl Read) -> Result<String> {
		let received = StreamRW::read(&mut buf);
		String::from_utf8(received).map_err(|_| {
			io::Error::new(
				io::ErrorKind::InvalidData,
				"Couldn't parse received string as utf8",
			)
		})
	}
}