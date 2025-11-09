use crate::card::Card;
use serde::{Deserialize, Serialize};

/// 플레이어
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub chips: i32,
    pub hand: Vec<Card>,
    pub bet: i32,
    pub folded: bool,
    pub is_active: bool,
}

impl Player {
    pub fn new(id: String, name: String, chips: i32) -> Self {
        Self {
            id,
            name,
            chips,
            hand: Vec::new(),
            bet: 0,
            folded: false,
            is_active: true,
        }
    }
}

/// 게임 단계
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamePhase {
    Waiting,   // 대기 중
    PreFlop,   // 프리플랍 (홀카드만)
    Flop,      // 플랍 (3장)
    Turn,      // 턴 (4장)
    River,     // 리버 (5장)
    Showdown,  // 쇼다운
}

/// 게임 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub community_cards: Vec<Card>,
    pub pot: i32,
    pub current_bet: i32,
    pub phase: GamePhase,
    pub current_player_idx: usize,
    pub dealer_idx: usize,
}