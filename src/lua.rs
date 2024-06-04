use std::{collections::VecDeque, sync::mpsc::{self, Receiver, SyncSender}};
use eframe::egui::{Context, OpenUrl};
use mlua::{Error, FromLua, Lua, Result, Table, UserData, Value};
use crate::parser::{parse_string, Page};

pub struct Executer {
    pub lua: Lua,
    pub console: Vec<String>,
    changes: Receiver<DocumentChange>,
    changes_sender: SyncSender<DocumentChange>,
}

impl Executer {
    pub fn log(&mut self, msg: impl ToString) {
        self.console.push(msg.to_string());
    }

    pub fn log_error(&mut self, msg: impl ToString) {
        self.console.push("Error: ".to_string() + &msg.to_string());
    }

    pub fn try_run(&mut self, code: &str, name: &str) {
        if let Err(why) = self.lua.load(code).set_name(name).exec() {
            self.log_error(why);
        }
    }

    pub fn update_document(&mut self, page: &mut Page, location: &mut Option<String>, title: &mut Option<String>, ctx: &Context) {
        while let Ok(change) = self.changes.try_recv() {
            match change {
                DocumentChange::Log(text) => self.log(text),
                DocumentChange::SetInner(path, inner) => {
                    let elements = match parse_string(&inner) {
                        Ok(els) => els,
                        Err(why) => {
                            self.log_error(why);
                            continue;
                        }
                    };
                    page.set_path_inner(path, elements, self);
                }
                DocumentChange::SetText(path, text) => {
                    page.set_path_text(path, text, self);
                }
                DocumentChange::SetLocation(link) => {
                    *location = Some(link);
                }
                DocumentChange::OpenLink(link) => {
                    ctx.open_url(OpenUrl::same_tab(link));
                }
                DocumentChange::SetAttr(path, attr, value) => {
                    page.set_path_attr(path, attr, value, self);
                }
                DocumentChange::SetTitle(value) => {
                    *title = Some(value);
                }
            }
        }
    }

    pub fn new() -> Self {
        let (tx, rx) = mpsc::sync_channel(255);
        Self { lua: Lua::new(), console: vec![], changes: rx, changes_sender: tx }
    }

    pub fn init_lua(&mut self) {
        self.lua = Lua::new();
        let sender = self.changes_sender.clone();
        let document = Document { changes_sender: sender };
        self.lua.globals().set("document", document).unwrap();
    }

    pub fn send_change(&self, change: DocumentChange) {
        self.changes_sender.send(change).unwrap();
    }
}

#[derive(Clone)]
pub struct Document {
    pub changes_sender: SyncSender<DocumentChange>,
}

impl<'lua> FromLua<'lua> for Document {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> Result<Self> {
        match value {
            Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
            _ => unreachable!(),
        }
    }
}

impl UserData for Document {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("set_text", |_, this, (path_table, text): (Table, String)| {
            let mut path: VecDeque<usize> = VecDeque::new();
            for part in path_table.sequence_values::<usize>() {
                match part {
                    Ok(index) => path.push_back(index),
                    Err(_) => {
                        return Err(Error::external("Path has non usize elements"));
                    }
                }
            }
            match this.changes_sender.send(DocumentChange::SetText(path, text)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("set_inner", |_, this, (path_table, rml): (Table, String)| {
            let mut path: VecDeque<usize> = VecDeque::new();
            for part in path_table.sequence_values::<usize>() {
                match part {
                    Ok(index) => path.push_back(index),
                    Err(_) => {
                        return Err(Error::external("Path has non usize elements"));
                    }
                }
            }
            match this.changes_sender.send(DocumentChange::SetInner(path.into(), rml)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("set_attr", |_, this, (path_table, attr, value): (Table, String, String)| {
            let mut path: VecDeque<usize> = VecDeque::new();
            for part in path_table.sequence_values::<usize>() {
                match part {
                    Ok(index) => path.push_back(index),
                    Err(_) => {
                        return Err(Error::external("Path has non usize elements"));
                    }
                }
            }
            match this.changes_sender.send(DocumentChange::SetAttr(path.into(), attr, value)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("log", |_, this, text: String| {
            match this.changes_sender.send(DocumentChange::Log(text)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("set_location", |_, this, location: String| {
            match this.changes_sender.send(DocumentChange::SetLocation(location)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("open_url", |_, this, url: String| {
            match this.changes_sender.send(DocumentChange::OpenLink(url)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
        methods.add_method("set_title", |_, this, title: String| {
            match this.changes_sender.send(DocumentChange::SetTitle(title)) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::external("Could not send document change")),
            }
        });
    }
}

pub enum DocumentChange {
    SetText(VecDeque<usize>, String),
    SetInner(VecDeque<usize>, String),
    Log(String),
    SetLocation(String),
    OpenLink(String),
    SetAttr(VecDeque<usize>, String, String),
    SetTitle(String),
}