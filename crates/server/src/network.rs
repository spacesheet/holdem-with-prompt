use crate::game::GameServer;
use holdem_shared::*;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::Arc;
use uuid::Uuid;

pub fn handle_client(stream: TcpStream, server: Arc<GameServer>) {
    let player_id = Uuid::new_v4().to_string();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    // 연결 저장
    {
        let mut connections = server.connections.lock().unwrap();
        connections.insert(player_id.clone(), stream.try_clone().unwrap());
    }

    println!("✅ 새 플레이어 연결: {}", player_id);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // 연결 종료
            Ok(_) => {
                if let Ok(message) = serde_json::from_str::<ClientMessage>(&line) {
                    handle_message(&server, &player_id, message);
                }
            }
            Err(_) => break,
        }
    }

    // 연결 종료 처리
    cleanup_player(&server, &player_id);
}

fn handle_message(server: &Arc<GameServer>, player_id: &str, message: ClientMessage) {
    match message {
        ClientMessage::Join { name } => {
            let mut state = server.game_state.lock().unwrap();
            let player = Player::new(player_id.to_string(), name.clone(), 1000);
            state.players.push(player);
            drop(state);

            server.send_to_player(
                player_id,
                &ServerMessage::Welcome {
                    player_id: player_id.to_string(),
                    chips: 1000,
                },
            );

            let state = server.game_state.lock().unwrap();
            server.broadcast(&ServerMessage::GameState(state.clone()), None);
            
            println!("📝 {} 참가", name);
        }
        ClientMessage::Ready => {
            println!("🎮 게임 시작 요청");
            server.start_game();
        }
        _ => {
            server.handle_player_action(player_id, message);
        }
    }
}

fn cleanup_player(server: &Arc<GameServer>, player_id: &str) {
    {
        let mut connections = server.connections.lock().unwrap();
        connections.remove(player_id);
    }

    {
        let mut state = server.game_state.lock().unwrap();
        state.players.retain(|p| p.id != player_id);
    }

    println!("❌ 플레이어 연결 종료: {}", player_id);
}