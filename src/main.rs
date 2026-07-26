// gui for the transmarvel sigil picker mod. rewrites the mod's own tbl copies
// so transmarvel only rolls the picks, can also launch the game through
// reloaded-ii
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod data;
mod patch;
mod reloaded;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--apply") {
        let mut app = app::App::new();
        std::process::exit(match app.apply() { Ok(()) => 0, Err(_) => 1 });
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([660.0, 760.0])
            .with_min_inner_size([540.0, 440.0])
            .with_icon(std::sync::Arc::new(egui::IconData { rgba: ui::WINDOW_ICON.to_vec(), width: 256, height: 256 }))
            .with_title("GBFRER Transmarvel Sigil Picker"),
        ..Default::default()
    };
    eframe::run_native("GBFRER Transmarvel Sigil Picker", options, Box::new(|cc| {
        ui::setup_style(&cc.egui_ctx);
        Ok(Box::new(app::App::new()))
    }))
}
