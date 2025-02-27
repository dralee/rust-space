// server 2025.2.27 by dralee

//use std::collections::HashMap;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;

/**
 * websocket server
 */
struct WsServer {
    clients : Vec<Box<tokio::net::TcpStream>>
}

impl WsServer {
    fn new() -> WsServer {
        WsServer {
            clients : Vec::new()
        }
    }

    fn add_client(&mut self, client : Box<tokio::net::TcpStream>) {
        self.clients.push(client);
    }

    fn remove_client(&mut self, client : Box<tokio::net::TcpStream>) {
        self.clients.retain(|c| *c != client);
    }

    async fn send_message(&self, message : &str) {
        for client in &self.clients {
            let (mut write, _) = client.split();
            if let Err(e) = write.send(Message::Text(message.to_string())).await {
                eprintln!("Error sending message: {}", e);
                return;
            }
        }
    }
}