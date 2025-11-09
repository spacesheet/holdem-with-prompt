mod app;
mod network;
mod ui;

use app::PokerClient;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "텍사스 홀덤 포커",
        options,
        Box::new(|_cc| Box::new(PokerClient::default())),
    )
}