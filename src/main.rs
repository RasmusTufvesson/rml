use eframe::egui;

mod app;
mod parser;
mod elements;
mod lua;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Rml renderer",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
