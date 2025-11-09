mod game;
mod network;

use game::GameServer;
use network::handle_client;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🃏 텍사스 홀덤 서버 시작: 127.0.0.1:7878");

    let server = Arc::new(GameServer::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server_clone = Arc::clone(&server);
                thread::spawn(move || {
                    handle_client(stream, server_clone);
                });
            }
            Err(e) => {
                eprintln!("연결 에러: {}", e);
            }
        }
    }
}