use std::{collections::VecDeque, fs};
use anyhow::anyhow;
use eframe::egui::{Layout, TextBuffer, Ui};
use crate::{elements::{Button, Div, Divider, FakeLink, Heading, Link, Paragraph, Space, WebLink}, lua::Executer};

pub type Elements = Vec<Box<dyn Element>>;

pub struct Page {
    pub title: String,
    body: Elements,
    pub scripts: Vec<String>,
}

impl Page {
    pub fn render(&mut self, ui: &mut Ui, executer: &mut Executer) {
        for element in &mut self.body {
            element.render(ui, Style::default(), executer);
        }
    }

    pub fn set_path_text(&mut self, mut path: VecDeque<usize>, text: String, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.body.get_mut(index) {
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
                executer.log_error("Empty path");
            }
        }
    }

    pub fn set_path_inner(&mut self, mut path: VecDeque<usize>, inner: Elements, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.body.get_mut(index) {
                    Some(element) => {
                        if path.len() == 0 {
                            element.set_inner(inner, executer);
                        } else {
                            element.set_path_inner(path, inner, executer);
                        }
                    }
                    None => {
                        executer.log_error("Invalid path");
                    }
                }
            }
            None => {
                executer.log_error("Empty path");
            }
        }
    }

    pub fn set_path_attr(&mut self, mut path: VecDeque<usize>, attr: String, value: String, executer: &mut Executer) {
        match path.pop_front() {
            Some(index) => {
                match self.body.get_mut(index) {
                    Some(element) => {
                        if path.len() == 0 {
                            element.set_attr(attr, value, executer);
                        } else {
                            element.set_path_attr(path, attr, value, executer);
                        }
                    }
                    None => {
                        executer.log_error("Invalid path");
                    }
                }
            }
            None => {
                executer.log_error("Empty path");
            }
        }
    }
}

pub trait Element {
    fn render(&mut self, ui: &mut Ui, style: Style, executer: &mut Executer);

    fn set_inner(&mut self, new: Elements, executer: &mut Executer) {
        executer.log_error("Element is not a container");
    }

    fn set_text(&mut self, text: String, executer: &mut Executer) {
        executer.log_error("Element does not have text");
    }

    fn set_attr(&mut self, attr: String, value: String, executer: &mut Executer) {
        executer.log_error("Element does not have attributes");
    }

    fn set_path_inner(&mut self, path: VecDeque<usize>, new: Elements, executer: &mut Executer) {
        executer.log_error("Element is not a container");
    }

    fn set_path_text(&mut self, path: VecDeque<usize>, text: String, executer: &mut Executer) {
        executer.log_error("Element is not a container");
    }

    fn set_path_attr(&mut self, path: VecDeque<usize>, attr: String, value: String, executer: &mut Executer) {
        executer.log_error("Element is not a container");
    }
}

#[derive(Default, Clone, Copy)]
pub struct Style {
    
}

pub fn parse_page(path: &str) -> anyhow::Result<Page> {
    let string = fs::read_to_string(path)?;
    let tags = parse_tags(&string)?;
    let page = tags_to_page(tags)?;
    Ok(page)
}

pub fn parse_string(string: &str) -> anyhow::Result<Elements> {
    let tags = parse_tags(&string)?;
    let elements = tags_to_elements(&tags)?;
    Ok(elements)
}

#[derive(Debug, Clone)]
struct Tag {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<TagOrText>,
}

#[derive(Debug, Clone)]
enum TagOrText {
    Tag(Tag),
    Text(String),
}

#[derive(Clone, Copy)]
enum ParseState {
    TagStart,
    TagEnd,
    Attribute,
    AttributeEquals,
    AttributeString,
    TagOrText,
}

fn parse_tags(string: &str) -> anyhow::Result<Vec<TagOrText>> {
    let mut stack = vec![];
    let mut state = ParseState::TagOrText;
    let mut tag_name_buffer = "".to_string();
    let mut string_buffer = "".to_string();
    let mut attribute_buffer = "".to_string();
    let mut attributes = vec![];
    let mut text_buffer = "".to_string();
    let mut child_stack = vec![vec![]];
    for chr in string.chars() {
        let mut new_state = state;
        match &state {
            ParseState::AttributeString => {
                if chr == '"' {
                    new_state = ParseState::Attribute;
                    attributes.push((attribute_buffer.take(), string_buffer.take()));
                } else {
                    string_buffer.push(chr);
                }
            }
            ParseState::Attribute => {
                if chr == '>' {
                    new_state = ParseState::TagOrText;
                    stack.push((tag_name_buffer.take(), attributes.clone()));
                    child_stack.push(vec![]);
                    attributes.clear();
                } else if chr == '=' {
                    new_state = ParseState::AttributeEquals;
                } else {
                    attribute_buffer.push(chr);
                }
            }
            ParseState::AttributeEquals => {
                if chr == '"' {
                    new_state = ParseState::AttributeString;
                } else {
                    return Err(anyhow!("Unexpected character after equals '{}'", chr));
                }
            }
            ParseState::TagStart => {
                if chr == '>' {
                    new_state = ParseState::TagOrText;
                    stack.push((tag_name_buffer.take(), vec![]));
                    child_stack.push(vec![]);
                    attributes.clear();
                } else if chr == ' ' {
                    new_state = ParseState::Attribute;
                } else if chr == '/' {
                    new_state = ParseState::TagEnd;
                } else {
                    tag_name_buffer.push(chr);
                }
            }
            ParseState::TagEnd => {
                if chr == '>' {
                    if tag_name_buffer != stack.last().unwrap().0 {
                        return Err(anyhow!("Closed tag '{}' without having opened it", tag_name_buffer));
                    }
                    tag_name_buffer.clear();
                    let (name, attributes) = stack.pop().unwrap();
                    let children = child_stack.pop().unwrap();
                    let index = child_stack.len() - 1;
                    child_stack[index].push(TagOrText::Tag(Tag { name, attributes, children }));
                    new_state = ParseState::TagOrText;
                } else {
                    tag_name_buffer.push(chr);
                }
            }
            ParseState::TagOrText => {
                if chr == '<' {
                    if text_buffer.len() != 0 && !text_buffer.chars().all(|c| c == ' ' || c == '\n' || c == '\r' || c == '\t') {
                        let index = child_stack.len() - 1;
                        child_stack[index].push(TagOrText::Text(text_buffer.take()));
                    }
                    text_buffer.clear();
                    new_state = ParseState::TagStart;
                } else {
                    text_buffer.push(chr);
                }
            }
        }
        state = new_state;
    }
    Ok(child_stack.pop().unwrap())
}

fn tags_to_page(tags: Vec<TagOrText>) -> anyhow::Result<Page> {
    let mut title = "Untitled".to_string();
    let mut scripts = vec![];
    if let Some(TagOrText::Tag(head)) = tags.get(0) {
        if head.name == "head" {
            for tag in &head.children {
                match tag {
                    TagOrText::Tag(tag) => {
                        if tag.name == "title" {
                            if let Some(TagOrText::Text(text)) = tag.children.get(0) {
                                title = text.clone();
                            } else {
                                return Err(anyhow!("Empty title tag"));
                            }
                        } else if tag.name == "script" {
                            if let Some(TagOrText::Text(text)) = tag.children.get(0) {
                                scripts.push(text.clone());
                            } else {
                                return Err(anyhow!("Empty script tag"));
                            }
                        } else {
                            return Err(anyhow!("Unknown tag '{}' in head", tag.name));
                        }
                    }
                    _ => {}
                }
            }
        } else {
            return Err(anyhow!("Page does not start with head"));
        }
    } else {
        return Err(anyhow!("Page does not start with head"));
    }
    let body = if let Some(TagOrText::Tag(body_tag)) = tags.get(1) {
        if body_tag.name == "body" {
            match tags_to_elements(&body_tag.children) {
                Ok(body) => body,
                Err(why) => return Err(why),
            }
        } else {
            return Err(anyhow!("Second tag is not body"));
        }
    } else {
        return Err(anyhow!("Second tag is not body"));
    };
    Ok(Page { title, body, scripts })
}

fn tags_to_elements(tags: &Vec<TagOrText>) -> anyhow::Result<Elements> {
    let mut elemets = vec![];
    for tag in tags {
        match tag {
            TagOrText::Tag(tag) => {
                elemets.push(tag_to_elemets(tag.clone())?);
            }
            _ => {
                return Err(anyhow!("Text in body tag"));
            }
        }
    }
    Ok(elemets)
}

fn tag_to_elemets(tag: Tag) -> anyhow::Result<Box<dyn Element>> {
    Ok(match tag.name.as_str() {
        "h" => {
            let text = get_text(tag)?;
            Box::new(Heading { text })
        }
        "p" => {
            let text = get_text(tag)?;
            Box::new(Paragraph { text })
        }
        "button" => {
            let on_click = get_attribute(&tag, "onclick").unwrap_or("".to_string());
            let text = get_text(tag)?;
            Box::new(Button { text, on_click })
        }
        "div" => {
            let direction = get_attribute(&tag, "direction");
            let align = get_attribute(&tag, "align");
            let mut inner = vec![];
            for tag in tag.children {
                if let TagOrText::Tag(tag) = tag {
                    inner.push(tag_to_elemets(tag)?);
                } else {
                    return Err(anyhow!("Text in div"));
                }
            }
            if direction.is_none() && align.is_none() {
                Box::new(Div { inner, layout: None })
            } else {
                let layout = Layout {
                    main_dir: match direction {
                        Some(val) => {
                            match val.as_str() {
                                "down" => eframe::egui::Direction::TopDown,
                                "up" => eframe::egui::Direction::BottomUp,
                                "left" => eframe::egui::Direction::RightToLeft,
                                "right" => eframe::egui::Direction::LeftToRight,
                                _ => return Err(anyhow!("Invalid direction '{}'", val)),
                            }
                        }
                        None => eframe::egui::Direction::TopDown,
                    },
                    cross_align: match align {
                        Some(val) => {
                            match val.as_str() {
                                "center" => eframe::egui::Align::Center,
                                "max" => eframe::egui::Align::Max,
                                "min" => eframe::egui::Align::Min,
                                _ => return Err(anyhow!("Invalid align '{}'", val)),
                            }
                        }
                        None => eframe::egui::Align::Min,
                    },
                    ..Default::default()
                };
                Box::new(Div { inner, layout: Some(layout) })
            }
        }
        "space" => Box::new(Space),
        "divider" => Box::new(Divider),
        "weblink" => {
            let dst = match get_attribute(&tag, "dst") {
                Some(dst) => dst,
                None => return Err(anyhow!("No dst attribute for weblink")),
            };
            let text = get_text(tag)?;
            Box::new(WebLink { text, dst })
        }
        "link" => {
            let dst = match get_attribute(&tag, "dst") {
                Some(dst) => dst,
                None => return Err(anyhow!("No dst attribute for link")),
            };
            let text = get_text(tag)?;
            Box::new(Link { text, dst })
        }
        "fakelink" => {
            let on_click = match get_attribute(&tag, "onclick") {
                Some(on_click) => on_click,
                None => return Err(anyhow!("No onclick attribute for fakelink")),
            };
            let text = get_text(tag)?;
            Box::new(FakeLink { text, on_click })
        }
        _ => {
            return Err(anyhow!("Unknown tag '{}'", tag.name));
        }
    })
}

fn get_text(tag: Tag) -> anyhow::Result<String> {
    if let Some(TagOrText::Text(text)) = tag.children.get(0) {
        Ok(text.clone())
    } else {
        Err(anyhow!("Could not find text for element"))
    }
}

fn get_attribute(tag: &Tag, attribute: &str) -> Option<String> {
    if let Some((_, value)) = tag.attributes.iter().find(|(attr, _)| attr == attribute) {
        Some(value.clone())
    } else {
        None
    }
}