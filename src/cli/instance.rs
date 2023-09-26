use std::{io::prelude::*, net::TcpStream};

use crate::gui::app::{Msg, PlayerMsg};

pub struct Instance {
    // Actions to be sent to instance
    actions: Vec<Msg>,
    // Contains invalid instance actions given as arg
    invalid: Vec<String>,
}

impl Instance {
    /// Creates new [`Instance`]
    pub fn new() -> Self {
        Instance::default()
    }

    /// Parses [`Instance`] arguments
    pub fn parse(&mut self, args: Vec<String>) {
        for arg in args {
            match Instance::get_action_msg(&arg) {
                Some(msg) => self.actions.push(msg),
                None => self.invalid.push(arg),
            }
        }
    }

    /// Submits [`Instance`]
    pub fn submit(&self, ip: &str, port: &str) {
        if !self.invalid.is_empty() {
            self.invalid_msg();
        } else if self.actions.is_empty() {
            eprintln!("No instance arguments given");
        } else {
            self.submit_actions(ip, port);
        }
    }

    /// Gets [`Msg`] by [`Instance`] action
    pub fn get_action_msg(action: &str) -> Option<Msg> {
        match action {
            "pp" | "play-pause" => Some(Msg::Plr(PlayerMsg::Play(None))),
            "next" => Some(Msg::Plr(PlayerMsg::Next)),
            "prev" => Some(Msg::Plr(PlayerMsg::Prev)),
            "shuffle" | "mix" => Some(Msg::Plr(PlayerMsg::Shuffle)),
            "exit" | "close" | "quit" => Some(Msg::Close),
            _ => None,
        }
    }

    /// Prints invalid actions
    fn invalid_msg(&self) {
        let mut invalid = String::from("");
        for arg in self.invalid.iter() {
            invalid = format!("{invalid} {arg}");
        }
        println!("Invalid intance actions:{invalid}");
    }

    /// Submits [`Instance`] actions
    fn submit_actions(&self, ip: &str, port: &str) {
        for action in self.actions.iter() {
            self.send_msg(action, ip, port);
        }
    }

    /// Sends given message to the server
    fn send_msg(&self, msg: &Msg, ip: &str, port: &str) {
        match TcpStream::connect(format!("{ip}:{port}")) {
            Ok(mut stream) => {
                if let Ok(msg) = serde_json::to_string::<Msg>(msg) {
                    _ = stream.write(msg.as_bytes());
                }
            }
            Err(_) => eprintln!("Error connecting to the server"),
        }
    }
}

/// Implements default for Instance
impl Default for Instance {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
            invalid: Vec::new(),
        }
    }
}
