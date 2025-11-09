use holdem_shared::*;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub type PlayerConnections = Arc<Mutex<HashMap<String, TcpStream>>>;

pub struct GameServer {
    pub game_state: Arc<Mutex<GameState>>,
    pub connections: PlayerConnections,
}

impl GameServer {
    pub fn new() -> Self {
        let game_state = GameState {
            players: Vec::new(),
            community_cards: Vec::new(),
            pot: 0,
            current_bet: 0,
            phase: GamePhase::Waiting,
            current_player_idx: 0,
            dealer_idx: 0,
        };

        Self {
            game_state: Arc::new(Mutex::new(game_state)),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn broadcast(&self, message: &ServerMessage, exclude_id: Option<&str>) {
        let connections = self.connections.lock().unwrap();
        let json = serde_json::to_string(message).unwrap() + "\n";

        for (player_id, stream) in connections.iter() {
            if let Some(exclude) = exclude_id {
                if player_id == exclude {
                    continue;
                }
            }

            if let Ok(mut stream) = stream.try_clone() {
                let _ = stream.write_all(json.as_bytes());
                let _ = stream.flush();
            }
        }
    }

    pub fn send_to_player(&self, player_id: &str, message: &ServerMessage) {
        let connections = self.connections.lock().unwrap();
        if let Some(stream) = connections.get(player_id) {
            if let Ok(mut stream) = stream.try_clone() {
                let json = serde_json::to_string(message).unwrap() + "\n";
                let _ = stream.write_all(json.as_bytes());
                let _ = stream.flush();
            }
        }
    }

    pub fn start_game(&self) {
        let mut state = self.game_state.lock().unwrap();

        if state.players.len() < 2 {
            return;
        }

        // 덱 생성 및 섞기
        let mut deck = Deck::new();
        deck.shuffle();

        // 각 플레이어에게 2장씩 배분
        for player in &mut state.players {
            player.hand.clear();
            player.hand.push(deck.deal().unwrap());
            player.hand.push(deck.deal().unwrap());
            player.folded = false;
            player.bet = 0;
        }

        // 커뮤니티 카드 초기화
        state.community_cards.clear();
        state.pot = 0;
        state.current_bet = 10;
        state.phase = GamePhase::PreFlop;
        state.current_player_idx = (state.dealer_idx + 1) % state.players.len();

        // 블라인드 배팅
        let small_blind_idx = (state.dealer_idx + 1) % state.players.len();
        let big_blind_idx = (state.dealer_idx + 2) % state.players.len();

        state.players[small_blind_idx].chips -= 5;
        state.players[small_blind_idx].bet = 5;
        state.players[big_blind_idx].chips -= 10;
        state.players[big_blind_idx].bet = 10;
        state.pot = 15;

        drop(state);

        // 각 플레이어에게 카드 전송
        let state = self.game_state.lock().unwrap();
        for player in &state.players {
            self.send_to_player(
                &player.id,
                &ServerMessage::DealCards {
                    cards: player.hand.clone(),
                },
            );
        }

        self.broadcast(&ServerMessage::GameState(state.clone()), None);
    }

    pub fn handle_player_action(&self, player_id: &str, action: ClientMessage) {
        let mut state = self.game_state.lock().unwrap();

        let player_idx = match state.players.iter().position(|p| p.id == player_id) {
            Some(idx) => idx,
            None => return,
        };

        if player_idx != state.current_player_idx {
            drop(state);
            self.send_to_player(
                player_id,
                &ServerMessage::Error {
                    message: "당신의 턴이 아닙니다".to_string(),
                },
            );
            return;
        }

        let action_str = match action {
            ClientMessage::Fold => {
                state.players[player_idx].folded = true;
                "폴드".to_string()
            }
            ClientMessage::Check => {
                if state.players[player_idx].bet < state.current_bet {
                    drop(state);
                    self.send_to_player(
                        player_id,
                        &ServerMessage::Error {
                            message: "체크할 수 없습니다".to_string(),
                        },
                    );
                    return;
                }
                "체크".to_string()
            }
            ClientMessage::Call => {
                let call_amount = state.current_bet - state.players[player_idx].bet;
                state.players[player_idx].chips -= call_amount;
                state.players[player_idx].bet = state.current_bet;
                state.pot += call_amount;
                "콜".to_string()
            }
            ClientMessage::Raise { amount } => {
                let total_bet = state.current_bet + amount;
                let raise_amount = total_bet - state.players[player_idx].bet;
                state.players[player_idx].chips -= raise_amount;
                state.players[player_idx].bet = total_bet;
                state.current_bet = total_bet;
                state.pot += raise_amount;
                format!("레이즈 {}", amount)
            }
            _ => return,
        };

        self.broadcast(
            &ServerMessage::PlayerAction {
                player_id: player_id.to_string(),
                action: action_str,
            },
            None,
        );

        // 다음 플레이어로 이동
        loop {
            state.current_player_idx = (state.current_player_idx + 1) % state.players.len();
            if !state.players[state.current_player_idx].folded {
                break;
            }
        }

        // 베팅 라운드 종료 체크
        let active_players: Vec<&Player> = state.players.iter().filter(|p| !p.folded).collect();

        if active_players.len() == 1 {
            let winner = active_players[0].clone();
            let pot = state.pot;
            drop(state);
            self.end_game(&winner.id, pot);
            return;
        }

        let all_bets_equal = active_players.iter().all(|p| p.bet == state.current_bet);

        if all_bets_equal {
            drop(state);
            self.next_phase();
        } else {
            self.broadcast(&ServerMessage::GameState(state.clone()), None);
        }
    }

    pub fn next_phase(&self) {
        let mut state = self.game_state.lock().unwrap();
        let mut deck = Deck::new();
        deck.shuffle();

        // 이미 사용된 카드 제거
        for player in &state.players {
            for card in &player.hand {
                deck.cards.retain(|c| c != card);
            }
        }
        for card in &state.community_cards {
            deck.cards.retain(|c| c != card);
        }

        match state.phase {
            GamePhase::PreFlop => {
                state.community_cards.push(deck.deal().unwrap());
                state.community_cards.push(deck.deal().unwrap());
                state.community_cards.push(deck.deal().unwrap());
                state.phase = GamePhase::Flop;
            }
            GamePhase::Flop => {
                state.community_cards.push(deck.deal().unwrap());
                state.phase = GamePhase::Turn;
            }
            GamePhase::Turn => {
                state.community_cards.push(deck.deal().unwrap());
                state.phase = GamePhase::River;
            }
            GamePhase::River => {
                state.phase = GamePhase::Showdown;
                drop(state);
                self.showdown();
                return;
            }
            _ => {}
        }

        for player in &mut state.players {
            player.bet = 0;
        }
        state.current_bet = 0;
        state.current_player_idx = (state.dealer_idx + 1) % state.players.len();

        self.broadcast(&ServerMessage::GameState(state.clone()), None);
    }

    pub fn showdown(&self) {
        let state = self.game_state.lock().unwrap();
        let mut best_player_id = String::new();
        let mut best_value = HandValue {
            rank: HandRank::HighCard,
            values: vec![0],
        };

        for player in &state.players {
            if player.folded {
                continue;
            }

            let mut all_cards = player.hand.clone();
            all_cards.extend(state.community_cards.clone());

            let (_, value) = find_best_hand(&all_cards);

            if value > best_value {
                best_value = value;
                best_player_id = player.id.clone();
            }
        }

        let pot = state.pot;
        drop(state);

        self.end_game(&best_player_id, pot);
    }

    pub fn end_game(&self, winner_id: &str, amount: i32) {
        let mut state = self.game_state.lock().unwrap();

        if let Some(winner) = state.players.iter_mut().find(|p| p.id == winner_id) {
            winner.chips += amount;
        }

        self.broadcast(
            &ServerMessage::GameOver {
                winner_id: winner_id.to_string(),
                amount,
            },
            None,
        );

        state.phase = GamePhase::Waiting;
        state.dealer_idx = (state.dealer_idx + 1) % state.players.len();
    }
}