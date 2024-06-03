use eframe::egui::{self, TextEdit};
use mlua::Lua;
use crate::parser::{parse, Page};

pub struct Executer {
    pub lua: Lua,
    pub console: Vec<String>,
}

impl Executer {
    pub fn log(&mut self, msg: impl ToString) {
        self.console.push(msg.to_string());
    }

    pub fn try_run(&mut self, code: &str, name: &str) {
        if let Err(why) = self.lua.load(code).set_name(name).exec() {
            self.log(why);
        }
    }
}

pub struct App {
    file_text: String,
    page: anyhow::Result<Page>,
    executer: Executer,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_text: "".to_string(),
            page: Err(anyhow::anyhow!("Enter file path")),
            executer: Executer {
                lua: Lua::new(),
                console: vec![],
            },
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
            self.executer.lua = Lua::new();
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
                if ui.add(TextEdit::singleline(&mut self.file_text).hint_text("Enter path to file here...").desired_width(f32::INFINITY)).lost_focus() {
                    self.load_page();
                }
            });
        });

        if self.executer.console.len() != 0 {
            egui::SidePanel::right("console").show(ctx, |ui| {
                for message in &self.executer.console {
                    ui.label(message);
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.page {
                Ok(page) => {
                    page.render(ui, &mut self.executer);
                }
                Err(why) => {
                    ui.heading("Could not load page");
                    ui.label(why.to_string());
                }
            }
        });
    }
}