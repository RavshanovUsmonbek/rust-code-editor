mod app;
mod file_icons;
mod fs_tree;
mod state;
mod theme;
mod widgets;

use app::EditorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0]),
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Code Editor",
        options,
        Box::new(|_cc| Box::new(EditorApp::default())),
    )
}
