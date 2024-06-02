use crate::parser::{Element, Elements, Style};

pub struct Heading {
    text: String,
}

impl Element for Heading {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style) {
        ui.heading(&self.text);
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Paragraph {
    text: String,
}

impl Element for Paragraph {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style) {
        ui.label(&self.text);
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Button {
    text: String,
    on_click: String,
}

impl Element for Button {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style) {
        if ui.button(&self.text).clicked() {
            println!("Run lua");
        }
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Div {
    inner: Elements,
}

impl Element for Div {
    fn render(&mut self, ui: &mut eframe::egui::Ui, style: Style) {
        for element in &mut self.inner {
            element.render(ui, style)
        }
    }

    fn set_inner(&mut self, new: Elements) {
        self.inner = new;
    }
}