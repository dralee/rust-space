/**
 * 线程使用，消息传递数据
 * 使用通道(channel)
 * 2024.04.09 by dralee
 */
fn main() {
    test();
}

use std::sync::mpsc;

fn test(){
    let (tx, rx) = mpsc::channel();
}