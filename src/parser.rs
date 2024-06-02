use eframe::egui::Ui;

pub type Elements = Vec<Box<dyn Element>>;

pub struct Page {
    pub title: String,
    body: Elements,
}

impl Page {
    pub fn render(&mut self, ui: &mut Ui) {
        for element in &mut self.body {
            element.render(ui, Style::default());
        }
    }
}

pub trait Element {
    fn render(&mut self, ui: &mut Ui, style: Style);

    fn set_inner(&mut self, new: Elements) {}
    fn set_text(&mut self, text: String) {}
}

#[derive(Default, Clone, Copy)]
pub struct Style {
    
}

pub fn parse(path: &str) -> anyhow::Result<Page> {
    todo!()
}