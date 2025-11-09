use crate::network::NetworkClient;
use crate::ui;
use eframe::egui;
use holdem_shared::*;

pub enum ConnectionState {
    Disconnected,
    Connected,
}

pub struct PokerClient {
    pub connection_state: ConnectionState,
    pub player_id: Option<String>,
    pub player_name: String,
    pub my_cards: Vec<Card>,
    pub game_state: Option<GameState>,
    pub network: Option<NetworkClient>,
    pub status_message: String,
    pub raise_amount: i32,
}

impl Default for PokerClient {
    fn default() -> Self {
        Self {
            connection_state: ConnectionState::Disconnected,
            player_id: None,
            player_name: String::new(),
            my_cards: Vec::new(),
            game_state: None,
            network: None,
            status_message: String::new(),
            raise_amount: 20,
        }
    }
}

impl PokerClient {
    pub fn connect(&mut self) {
        match NetworkClient::connect("127.0.0.1:7878") {
            Ok(network) => {
                println!("✅ 서버 연결 성공");
                self.connection_state = ConnectionState::Connected;
                self.status_message = "서버에 연결됨!".to_string();
                self.network = Some(network);
            }
            Err(e) => {
                eprintln!("❌ 연결 실패: {}", e);
                self.status_message = format!("연결 실패: {}", e);
            }
        }
    }

    pub fn send_message(&mut self, msg: ClientMessage) {
        if let Some(network) = &mut self.network {
            network.send(msg);
        }
    }

    pub fn handle_server_messages(&mut self) {
        if let Some(network) = &mut self.network {
            while let Some(msg) = network.receive() {
                match msg {
                    ServerMessage::Welcome { player_id, chips } => {
                        self.player_id = Some(player_id);
                        self.status_message = format!("환영합니다! 칩: ${}", chips);
                    }
                    ServerMessage::GameState(state) => {
                        self.game_state = Some(state);
                    }
                    ServerMessage::DealCards { cards } => {
                        self.my_cards = cards;
                        self.status_message = "카드를 받았습니다!".to_string();
                    }
                    ServerMessage::PlayerAction { player_id, action } => {
                        self.status_message = format!("{}가 {}", player_id, action);
                    }
                    ServerMessage::GameOver { winner_id, amount } => {
                        self.status_message = format!("🎉 {}가 ${} 획득!", winner_id, amount);
                    }
                    ServerMessage::Error { message } => {
                        self.status_message = format!("❌ {}", message);
                    }
                }
            }
        }
    }
}

impl eframe::App for PokerClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_server_messages();
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.connection_state {
                ConnectionState::Disconnected => {
                    ui::lobby::render(self, ui);
                }
                ConnectionState::Connected => {
                    if self.player_id.is_none() {
                        ui::lobby::render_join(self, ui);
                    } else {
                        ui::game::render(self, ui);
                    }
                }
            }
        });
    }
}