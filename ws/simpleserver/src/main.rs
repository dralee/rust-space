// simple web socket server
// 2025.2.27 by dralee
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
//use tokio::prelude::*;
use WsServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动一个 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("WebSocket server listening on ws://127.0.0.1:8080");
    let ws = WsServer::new();

    // 接受连接
    while let Ok((stream, _)) = listener.accept().await {
        // 每次接收到新的连接，创建一个新的任务来处理它
        tokio::spawn(handle_connection(stream, ws));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, ws : WsServer) {
    // WebSocket 握手
    let ws_stream = match accept_async(stream).await {
        Ok(ws_stream) => ws_stream,
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {}", e);
            return;
        }
    };

    println!("New WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();
    ws.add_client(Box::new(stream));

    // 接收客户端消息并发送回去
    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Text(text) => {
                println!("Received: {}", text);
                // 向客户端发送回收到的消息
                // if let Err(e) = write.send(Message::Text(text)).await {
                //     eprintln!("Error sending message: {}", e);
                //     return;
                // }
                ws.send_message(&text).await;
            }
            Message::Close(_) => {
                println!("Client disconnected");
                ws.remove_client(stream);
                break;
            }
            _ => {}
        }
    }
}
