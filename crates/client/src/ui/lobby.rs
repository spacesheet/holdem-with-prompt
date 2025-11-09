use crate::app::PokerClient;
use eframe::egui;
use holdem_shared::ClientMessage;

/// 연결 전 로비 화면
pub fn render(app: &mut PokerClient, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.heading(
            egui::RichText::new("🃏 텍사스 홀덤 포커")
                .size(32.0)
        );
        
        ui.add_space(30.0);
        
        ui.label("서버 주소: 127.0.0.1:7878");
        
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            ui.label("플레이어 이름:");
            ui.text_edit_singleline(&mut app.player_name);
        });
        
        ui.add_space(10.0);
        
        if ui.button("🔌 서버 연결").clicked() && !app.player_name.is_empty() {
            app.connect();
        }
        
        ui.add_space(20.0);
        
        if !app.status_message.is_empty() {
            ui.label(
                egui::RichText::new(&app.status_message)
                    .color(egui::Color32::RED)
            );
        }
    });
}

/// 연결 후 게임 참가 화면
pub fn render_join(app: &mut PokerClient, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.heading("게임 참가");
        
        ui.add_space(30.0);
        
        ui.horizontal(|ui| {
            ui.label("플레이어 이름:");
            ui.text_edit_singleline(&mut app.player_name);
        });
        
        ui.add_space(10.0);
        
        if ui.button("🎮 게임 참가").clicked() && !app.player_name.is_empty() {
            app.send_message(ClientMessage::Join {
                name: app.player_name.clone(),
            });
        }
        
        ui.add_space(20.0);
        
        if !app.status_message.is_empty() {
            ui.label(&app.status_message);
        }
    });
}