use crate::{lua::Executer, parser::{Element, Elements, Style}};

pub struct Heading {
    pub text: String,
}

impl Element for Heading {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.heading(&self.text);
    }

    fn set_text(&mut self, text: String, _: &mut Executer) {
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

    fn set_text(&mut self, text: String, _: &mut Executer) {
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

    fn set_text(&mut self, text: String, _: &mut Executer) {
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

    fn set_inner(&mut self, new: Elements, _: &mut Executer) {
        self.inner = new;
    }

    fn set_path_inner(&mut self, mut path: std::collections::VecDeque<usize>, new: Elements, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.inner.get_mut(index) {
                    Some(element) => {
                        element.set_path_inner(path, new, executer);
                    }
                    None => {
                        executer.log_error("Invalid path");
                    }
                }
            }
            None => {
                self.inner = new;
            }
        };
    }

    fn set_path_text(&mut self, mut path: std::collections::VecDeque<usize>, text: String, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.inner.get_mut(index) {
                    Some(element) => {
                        if path.len() == 0 {
                            element.set_text(text, executer);
                        } else {
                            element.set_path_text(path, text, executer);
                        }
                    }
                    None => {
                        executer.log_error("Invalid path");
                    }
                }
            }
            None => {
                executer.log_error("Div cannot contain text");
            }
        };
    }
}