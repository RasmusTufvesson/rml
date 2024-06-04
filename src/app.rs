use eframe::egui::{self, Id, Sense, TextEdit};
use crate::{parser::{parse, Page}, lua::Executer};

pub struct App {
    file_text: String,
    page: anyhow::Result<Page>,
    executer: Executer,
    show_console: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_text: "".to_string(),
            page: Err(anyhow::anyhow!("Enter file path")),
            executer: Executer::new(),
            show_console: false,
        }
    }
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn load_page(&mut self) {
        self.page = parse(&self.file_text);
        self.executer.console.clear();
        if let Ok(page) = &self.page {
            self.executer.init_lua();
            for script in &page.scripts {
                self.executer.try_run(&script, "script");
            }
        }
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
                let response = ui.add(TextEdit::singleline(&mut self.file_text).hint_text("Enter path to file here...").desired_width(f32::INFINITY));
                if response.lost_focus() && response.ctx.input(|state| state.key_pressed(egui::Key::Enter)) {
                    self.load_page();
                }
            });
        });

        if self.show_console {
            egui::SidePanel::right("console").resizable(false).exact_width(100.0).show(ctx, |ui| {
                for message in &self.executer.console {
                    ui.label(message);
                }
                ui.interact(ui.max_rect(), Id::new("bg_side"), Sense::click()).context_menu(|ui| {
                    if ui.button("Close console").clicked() {
                        self.show_console = false;
                        ui.close_menu();
                    }
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut location = None;
            match &mut self.page {
                Ok(page) => {
                    self.executer.update_document(page, &mut location, ui.ctx());
                    page.render(ui, &mut self.executer);
                }
                Err(why) => {
                    ui.heading("Could not load page");
                    ui.label(why.to_string());
                }
            }
            if let Some(location) = location {
                self.file_text = location;
                self.load_page();
            }
            ui.interact(ui.max_rect(), Id::new("bg_central"), Sense::click()).context_menu(|ui| {
                if ui.button(if self.show_console { "Close console" } else { "Open console" }).clicked() {
                    self.show_console = !self.show_console;
                    ui.close_menu();
                }
            });
        });

    }
}