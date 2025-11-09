use crate::card::{Card, Rank};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// 핸드 랭킹
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandRank {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

impl HandRank {
    pub fn name(&self) -> &str {
        match self {
            HandRank::HighCard => "하이 카드",
            HandRank::OnePair => "원 페어",
            HandRank::TwoPair => "투 페어",
            HandRank::ThreeOfAKind => "트리플",
            HandRank::Straight => "스트레이트",
            HandRank::Flush => "플러시",
            HandRank::FullHouse => "풀 하우스",
            HandRank::FourOfAKind => "포카드",
            HandRank::StraightFlush => "스트레이트 플러시",
            HandRank::RoyalFlush => "로얄 플러시",
        }
    }
}

/// 핸드 평가 결과
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandValue {
    pub rank: HandRank,
    pub values: Vec<u8>,
}

impl PartialOrd for HandValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self.values.cmp(&other.values),
            other => other,
        }
    }
}

/// 핸드 평가 함수
pub fn evaluate_hand(cards: &[Card]) -> HandValue {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by(|a, b| b.rank.cmp(&a.rank));

    // 플러시 체크
    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);

    // 스트레이트 체크
    let is_straight = check_straight(&sorted_cards);

    // 랭크별 카운트
    let mut rank_counts = std::collections::HashMap::new();
    for card in &sorted_cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }

    let mut counts: Vec<(Rank, usize)> = rank_counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    // 로얄 플러시
    if is_flush && is_straight && sorted_cards[0].rank == Rank::Ace {
        return HandValue {
            rank: HandRank::RoyalFlush,
            values: vec![14],
        };
    }

    // 스트레이트 플러시
    if is_flush && is_straight {
        return HandValue {
            rank: HandRank::StraightFlush,
            values: vec![sorted_cards[0].rank as u8],
        };
    }

    // 포카드
    if counts[0].1 == 4 {
        return HandValue {
            rank: HandRank::FourOfAKind,
            values: vec![counts[0].0 as u8, counts[1].0 as u8],
        };
    }

    // 풀 하우스
    if counts[0].1 == 3 && counts[1].1 == 2 {
        return HandValue {
            rank: HandRank::FullHouse,
            values: vec![counts[0].0 as u8, counts[1].0 as u8],
        };
    }

    // 플러시
    if is_flush {
        let values: Vec<u8> = sorted_cards.iter().map(|c| c.rank as u8).collect();
        return HandValue {
            rank: HandRank::Flush,
            values,
        };
    }

    // 스트레이트
    if is_straight {
        return HandValue {
            rank: HandRank::Straight,
            values: vec![sorted_cards[0].rank as u8],
        };
    }

    // 트리플
    if counts[0].1 == 3 {
        let mut values = vec![counts[0].0 as u8];
        for i in 1..counts.len() {
            values.push(counts[i].0 as u8);
        }
        return HandValue {
            rank: HandRank::ThreeOfAKind,
            values,
        };
    }

    // 투 페어
    if counts[0].1 == 2 && counts[1].1 == 2 {
        return HandValue {
            rank: HandRank::TwoPair,
            values: vec![counts[0].0 as u8, counts[1].0 as u8, counts[2].0 as u8],
        };
    }

    // 원 페어
    if counts[0].1 == 2 {
        let mut values = vec![counts[0].0 as u8];
        for i in 1..counts.len() {
            values.push(counts[i].0 as u8);
        }
        return HandValue {
            rank: HandRank::OnePair,
            values,
        };
    }

    // 하이 카드
    let values: Vec<u8> = sorted_cards.iter().map(|c| c.rank as u8).collect();
    HandValue {
        rank: HandRank::HighCard,
        values,
    }
}

fn check_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }

    // A-2-3-4-5 스트레이트 체크 (Ace low)
    if cards[0].rank == Rank::Ace
        && cards[1].rank == Rank::Five
        && cards[2].rank == Rank::Four
        && cards[3].rank == Rank::Three
        && cards[4].rank == Rank::Two
    {
        return true;
    }

    // 일반 스트레이트
    for i in 0..cards.len() - 1 {
        if cards[i].rank as u8 != cards[i + 1].rank as u8 + 1 {
            return false;
        }
    }
    true
}

/// 최고의 5장 카드 조합 찾기 (7장 중)
pub fn find_best_hand(cards: &[Card]) -> (Vec<Card>, HandValue) {
    if cards.len() < 5 {
        panic!("Need at least 5 cards");
    }

    let mut best_hand = Vec::new();
    let mut best_value = HandValue {
        rank: HandRank::HighCard,
        values: vec![0],
    };

    // 7장 중 5장을 선택하는 모든 조합 (7C5 = 21)
    for i in 0..cards.len() {
        for j in i + 1..cards.len() {
            for k in j + 1..cards.len() {
                for l in k + 1..cards.len() {
                    for m in l + 1..cards.len() {
                        let hand = vec![cards[i], cards[j], cards[k], cards[l], cards[m]];
                        let value = evaluate_hand(&hand);
                        if value > best_value {
                            best_value = value;
                            best_hand = hand;
                        }
                    }
                }
            }
        }
    }

    (best_hand, best_value)
}