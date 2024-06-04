use eframe::egui::Layout;

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

    fn set_attr(&mut self, attr: String, executer: &mut Executer) {
        if attr == "onclick" {
            self.on_click = attr;
        } else {
            executer.log_error(format!("Unknown attribute '{}'", attr))
        }
    }
}

pub struct Div {
    pub inner: Elements,
    pub layout: Option<Layout>,
}

impl Element for Div {
    fn render(&mut self, ui: &mut eframe::egui::Ui, style: Style, executer: &mut Executer) {
        if let Some(layout) = self.layout {
            ui.with_layout(layout, |ui| {
                for element in &mut self.inner {
                    element.render(ui, style, executer);
                }
            });
        } else {
            for element in &mut self.inner {
                element.render(ui, style, executer);
            }
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
                self.set_inner(new, executer);
            }
        }
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
        }
    }

    fn set_path_attr(&mut self, mut path: std::collections::VecDeque<usize>, attr: String, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.inner.get_mut(index) {
                    Some(element) => {
                        if path.len() == 0 {
                            element.set_attr(attr, executer);
                        } else {
                            element.set_path_attr(path, attr, executer);
                        }
                    }
                    None => {
                        executer.log_error("Invalid path");
                    }
                }
            }
            None => {
                self.set_attr(attr, executer);
            }
        }
    }

    fn set_attr(&mut self, attr: String, executer: &mut Executer) {
        match attr.as_str() {
            "direction" => {
                let direction = match attr.as_str() {
                    "down" => eframe::egui::Direction::TopDown,
                    "up" => eframe::egui::Direction::BottomUp,
                    "left" => eframe::egui::Direction::RightToLeft,
                    "right" => eframe::egui::Direction::LeftToRight,
                    _ => {
                        executer.log_error(format!("Invalid direction '{}'", attr));
                        return;
                    }
                };
                if let Some(layout) = &mut self.layout {
                    layout.main_dir = direction;
                } else {
                    self.layout = Some(Layout {
                        main_dir: direction,
                        ..Default::default()
                    });
                }
            }
            "align" => {
                let align = match attr.as_str() {
                    "center" => eframe::egui::Align::Center,
                    "max" => eframe::egui::Align::Max,
                    "min" => eframe::egui::Align::Min,
                    _ => {
                        executer.log_error(format!("Invalid align '{}'", attr));
                        return;
                    }
                };
                if let Some(layout) = &mut self.layout {
                    layout.cross_align = align;
                } else {
                    self.layout = Some(Layout {
                        cross_align: align,
                        ..Default::default()
                    });
                }
            }
            _ => executer.log_error(format!("Unknown attribute '{}'", attr)),
        }
    }
}

pub struct Space;

impl Element for Space {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.add_space(ui.spacing().item_spacing.x);
    }
}

pub struct Divider;

impl Element for Divider {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.separator();
    }
}

pub struct WebLink {
    pub text: String,
    pub dst: String,
}

impl Element for WebLink {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, _: &mut Executer) {
        ui.hyperlink_to(&self.text, &self.dst);
    }

    fn set_text(&mut self, text: String, _: &mut Executer) {
        self.text = text;
    }

    fn set_attr(&mut self, attr: String, executer: &mut Executer) {
        if attr == "dst" {
            self.dst = attr;
        } else {
            executer.log_error(format!("Unknown attribute '{}'", attr))
        }
    }
}

pub struct Link {
    pub text: String,
    pub dst: String,
}

impl Element for Link {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, executer: &mut Executer) {
        if ui.link(&self.text).clicked() {
            executer.send_change(crate::lua::DocumentChange::SetLocation(self.dst.clone()));
        }
    }

    fn set_text(&mut self, text: String, _: &mut Executer) {
        self.text = text;
    }

    fn set_attr(&mut self, attr: String, executer: &mut Executer) {
        if attr == "dst" {
            self.dst = attr;
        } else {
            executer.log_error(format!("Unknown attribute '{}'", attr))
        }
    }
}

pub struct FakeLink {
    pub text: String,
    pub on_click: String,
}

impl Element for FakeLink {
    fn render(&mut self, ui: &mut eframe::egui::Ui, _: Style, executer: &mut Executer) {
        if ui.link(&self.text).clicked() {
            executer.try_run(&self.on_click, "onclick");
        }
    }

    fn set_text(&mut self, text: String, _: &mut Executer) {
        self.text = text;
    }

    fn set_attr(&mut self, attr: String, executer: &mut Executer) {
        if attr == "onclick" {
            self.on_click = attr;
        } else {
            executer.log_error(format!("Unknown attribute '{}'", attr))
        }
    }
}