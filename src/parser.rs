use std::fs;

use anyhow::anyhow;
use eframe::egui::{TextBuffer, Ui};

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
    let string = fs::read_to_string(path)?;
    let tags = parse_tags(&string)?;
    println!("tags: {:?}", tags);
    Err(anyhow!("Not implemented"))
}

#[derive(Debug)]
struct Tag {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<TagOrText>,
}

#[derive(Debug)]
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