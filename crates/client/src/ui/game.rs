use crate::app::PokerClient;
use crate::ui::components;
use eframe::egui;
use holdem_shared::*;

pub fn render(app: &mut PokerClient, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("🃏 텍사스 홀덤 포커");
        ui.add_space(10.0);

        // 상태 메시지
        components::render_status(ui, &app.status_message);
        ui.add_space(10.0);

        if let Some(state) = app.game_state.clone() {
            // 게임 정보
            components::render_game_info(ui, &state);
            ui.add_space(20.0);

            // 플레이어 정보
            render_players(app, ui, &state);
            ui.add_space(20.0);

            // 커뮤니티 카드
            if !state.community_cards.is_empty() {
                render_community_cards(ui, &state);
                ui.add_space(20.0);
            }

            // 내 카드
            if !app.my_cards.is_empty() {
                render_my_cards(ui, &app.my_cards);
                ui.add_space(20.0);
            }

            // 액션 버튼
            render_actions(app, ui, &state);
        } else {
            ui.label("게임 상태를 불러오는 중...");
        }
    });
}

fn render_players(app: &PokerClient, ui: &mut egui::Ui, state: &GameState) {
    ui.label(
        egui::RichText::new("플레이어")
            .size(18.0)
            .strong()
    );
    ui.add_space(5.0);
    
    ui.horizontal(|ui| {
        for player in &state.players {
            let is_me = Some(&player.id) == app.player_id.as_ref();
            components::render_player_card(ui, player, is_me);
            ui.add_space(10.0);
        }
    });
}

fn render_community_cards(ui: &mut egui::Ui, state: &GameState) {
    ui.label(
        egui::RichText::new("커뮤니티 카드")
            .size(18.0)
            .strong()
    );
    ui.add_space(5.0);
    
    ui.horizontal(|ui| {
        for card in &state.community_cards {
            components::render_card(ui, card);
            ui.add_space(5.0);
        }
    });
}

fn render_my_cards(ui: &mut egui::Ui, cards: &[Card]) {
    ui.label(
        egui::RichText::new("내 카드")
            .size(18.0)
            .strong()
    );
    ui.add_space(5.0);
    
    ui.horizontal(|ui| {
        for card in cards {
            components::render_card(ui, card);
            ui.add_space(5.0);
        }
    });
}

fn render_actions(app: &mut PokerClient, ui: &mut egui::Ui, state: &GameState) {
    match state.phase {
        GamePhase::Waiting => {
            if ui.button(
                egui::RichText::new("🎮 게임 시작 (Ready)")
                    .size(18.0)
            ).clicked() {
                app.send_message(ClientMessage::Ready);
            }
        }
        GamePhase::PreFlop | GamePhase::Flop | GamePhase::Turn | GamePhase::River => {
            render_betting_actions(app, ui, state);
        }
        GamePhase::Showdown => {
            ui.label(
                egui::RichText::new("🎊 쇼다운!")
                    .size(20.0)
                    .color(egui::Color32::GOLD)
            );
        }
    }
}

fn render_betting_actions(app: &mut PokerClient, ui: &mut egui::Ui, state: &GameState) {
    let my_player = state
        .players
        .iter()
        .find(|p| Some(&p.id) == app.player_id.as_ref());

    if let Some(player) = my_player {
        let is_my_turn = state.current_player_idx
            == state
                .players
                .iter()
                .position(|p| p.id == player.id)
                .unwrap();

        if is_my_turn {
            ui.label(
                egui::RichText::new("🎯 당신의 턴!")
                    .size(20.0)
                    .color(egui::Color32::GOLD)
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                // 폴드 버튼
                if ui.button(
                    egui::RichText::new("❌ 폴드")
                        .size(16.0)
                ).clicked() {
                    app.send_message(ClientMessage::Fold);
                }

                ui.add_space(10.0);

                // 체크 또는 콜 버튼
                if player.bet >= state.current_bet {
                    if ui.button(
                        egui::RichText::new("✅ 체크")
                            .size(16.0)
                    ).clicked() {
                        app.send_message(ClientMessage::Check);
                    }
                } else {
                    let call_amount = state.current_bet - player.bet;
                    if ui.button(
                        egui::RichText::new(format!("📞 콜 (${call_amount})"))
                            .size(16.0)
                    ).clicked() {
                        app.send_message(ClientMessage::Call);
                    }
                }

                ui.add_space(10.0);

                // 레이즈 버튼
                ui.add(
                    egui::Slider::new(&mut app.raise_amount, 10..=100)
                        .text("$")
                );
                
                if ui.button(
                    egui::RichText::new("⬆️ 레이즈")
                        .size(16.0)
                ).clicked() {
                    app.send_message(ClientMessage::Raise {
                        amount: app.raise_amount,
                    });
                }
            });
        } else {
            ui.label(
                egui::RichText::new("⏳ 다른 플레이어의 턴입니다...")
                    .size(16.0)
                    .color(egui::Color32::GRAY)
            );
        }
    }
}