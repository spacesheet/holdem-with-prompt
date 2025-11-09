use crate::card::Card;
use crate::game::GameState;
use serde::{Deserialize, Serialize};

/// 클라이언트 -> 서버 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// 게임 참가
    Join { name: String },
    
    /// 게임 시작 준비
    Ready,
    
    /// 폴드 (게임 포기)
    Fold,
    
    /// 체크 (베팅 없이 넘어가기)
    Check,
    
    /// 콜 (현재 베팅 맞추기)
    Call,
    
    /// 레이즈 (베팅 올리기)
    Raise { amount: i32 },
}

/// 서버 -> 클라이언트 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// 연결 환영 메시지
    Welcome { 
        player_id: String, 
        chips: i32 
    },
    
    /// 게임 상태 업데이트
    GameState(GameState),
    
    /// 카드 배분
    DealCards { 
        cards: Vec<Card> 
    },
    
    /// 플레이어 액션 알림
    PlayerAction { 
        player_id: String, 
        action: String 
    },
    
    /// 게임 종료
    GameOver { 
        winner_id: String, 
        amount: i32 
    },
    
    /// 에러 메시지
    Error { 
        message: String 
    },
}