/**
 * 流读取、写入
 * 2024.04.19 by dralee
 */
use std::io::Read;

const BUFFER_SIZE:usize = 1024;

pub struct StreamRW {
}
impl StreamRW {
	pub fn read(mut buf: &mut impl Read) -> Vec<u8> {
		let mut received: Vec<u8> = vec![];
		
		let mut buffer = [0u8; BUFFER_SIZE];
		// loop {
		// 	let bytes_read = buf.read(&mut buffer);
		// 	received.extend_from_slice(&buffer[..bytes_read]); // 扩展到接收缓冲中

		// }
		// let mut reader = ;
		loop {
			
		}
	}
}