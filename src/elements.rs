use crate::{app::Executer, parser::{Element, Elements, Style}};

pub struct Heading {
    pub text: String,
}

impl Element for Heading {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.heading(&self.text);
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Paragraph {
    pub text: String,
}

impl Element for Paragraph {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.label(&self.text);
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Button {
    pub text: String,
    pub on_click: String,
}

impl Element for Button {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, executer: &mut Executer) {
        if ui.button(&self.text).clicked() {
            executer.try_run(&self.on_click, "onclick");
        }
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Div {
    pub inner: Elements,
}

impl Element for Div {
    fn render(&mut self, ui: &mut eframe::egui::Ui, style: Style, executer: &mut Executer) {
        for element in &mut self.inner {
            element.render(ui, style, executer)
        }
    }

    fn set_inner(&mut self, new: Elements) {
        self.inner = new;
    }
}