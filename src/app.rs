use eframe::egui::{self, TextEdit};
use crate::parser::{parse, Page};

pub struct App {
    file_text: String,
    page: anyhow::Result<Page>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_text: "".to_string(),
            page: Err(anyhow::anyhow!("Enter file path")),
        }
    }
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn load_page(&mut self) {
        self.page = parse(&self.file_text);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                match &self.page {
                    Ok(page) => {
                        ui.label(&page.title);
                    }
                    Err(_) => {
                        ui.label("Error");
                    }
                }
                if ui.add(TextEdit::singleline(&mut self.file_text).hint_text("Enter path to file here...").desired_width(f32::INFINITY)).lost_focus() {
                    self.load_page();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.page {
                Ok(page) => {
                    page.render(ui);
                }
                Err(why) => {
                    ui.heading("Could not load page");
                    ui.label(why.to_string());
                }
            }
        });
    }
}