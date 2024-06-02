use eframe::egui::Ui;

pub type Elements = Vec<Box<dyn Element>>;

pub struct Page {
    pub title: String,
    body: Elements,
}

impl Page {
    pub fn render(&mut self, ui: &mut Ui) {
        for element in &mut self.body {
            element.render(ui);
        }
    }
}

pub trait Element {
    fn set_inner(&mut self, new: Elements);
    fn render(&mut self, ui: &mut Ui);

    fn clicked(&mut self) {}
}

pub fn parse(path: &str) -> anyhow::Result<Page> {
    todo!()
}