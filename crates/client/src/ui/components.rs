use eframe::egui;
use holdem_shared::*;

/// 카드 렌더링 컴포넌트
pub fn render_card(ui: &mut egui::Ui, card: &Card) {
    let color = match card.suit {
        Suit::Hearts | Suit::Diamonds => egui::Color32::from_rgb(220, 50, 50),
        Suit::Clubs | Suit::Spades => egui::Color32::from_rgb(50, 50, 50),
    };

    egui::Frame::new()
        .fill(egui::Color32::WHITE)
        .stroke(egui::Stroke::new(2.0, egui::Color32::GRAY))
        .inner_margin(8.0)
        .corner_radius(5.0)
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(50.0, 70.0));
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(card.rank.symbol())
                        .size(24.0)
                        .color(color),
                );
                ui.label(
                    egui::RichText::new(card.suit.symbol())
                        .size(20.0)
                        .color(color),
                );
            });
        });
}

/// 플레이어 정보 카드 렌더링
pub fn render_player_card(ui: &mut egui::Ui, player: &Player, is_me: bool) {
    let frame = if is_me {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(100, 150, 255))
            .inner_margin(10.0)
            .corner_radius(5.0)
    } else {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(200, 200, 200))
            .inner_margin(10.0)
            .corner_radius(5.0)
    };

    frame.show(ui, |ui| {
        ui.set_min_width(120.0);
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(&player.name)
                    .color(egui::Color32::WHITE)
                    .strong(),
            );
            ui.label(
                egui::RichText::new(format!("💰 ${}", player.chips))
                    .color(egui::Color32::WHITE),
            );
            if player.bet > 0 {
                ui.label(
                    egui::RichText::new(format!("🎲 베팅: ${}", player.bet))
                        .color(egui::Color32::WHITE),
                );
            }
            if player.folded {
                ui.label(
                    egui::RichText::new("❌ 폴드")
                        .color(egui::Color32::WHITE),
                );
            }
        });
    });
}

/// 게임 정보 헤더
pub fn render_game_info(ui: &mut egui::Ui, state: &GameState) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("💰 팟: ${}", state.pot))
                .size(18.0)
                .strong(),
        );
        ui.separator();
        ui.label(
            egui::RichText::new(format!("📊 현재 베팅: ${}", state.current_bet))
                .size(18.0),
        );
        ui.separator();
        ui.label(
            egui::RichText::new(format!("🎲 {:?}", state.phase))
                .size(18.0),
        );
    });
}

/// 상태 메시지 표시
pub fn render_status(ui: &mut egui::Ui, message: &str) {
    if !message.is_empty() {
        ui.label(
            egui::RichText::new(message)
                .size(16.0)
                .color(egui::Color32::DARK_GREEN),
        );
    }
}