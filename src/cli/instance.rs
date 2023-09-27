use std::{io::prelude::*, net::TcpStream};

use crate::gui::app::{Msg, PlayerMsg};

pub struct Instance {
    // Actions to be sent to instance
    actions: Vec<Msg>,
    // Contains whether invalid action was given
    invalid: bool,
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
                None => {
                    self.invalid = true;
                    eprintln!("Unknown instance action: {arg}");
                    return;
                }
            }
        }
    }

    /// Submits [`Instance`]
    pub fn submit(&self, ip: &str, port: &str) {
        if self.invalid {
            return;
        }
        if self.actions.is_empty() {
            eprintln!("Instance action expected, none given.");
        } else {
            self.submit_actions(ip, port);
        }
    }

    /// Gets [`Msg`] by [`Instance`] action
    pub fn get_action_msg(action: &str) -> Option<Msg> {
        fn get_action_param(action: &str) -> Option<&str> {
            let mut param = action.split("=");
            param.next();
            param.next()
        }

        match action {
            s if s.starts_with("pp") || s.starts_with("play-pause") => {
                if let Some(param) = get_action_param(action) {
                    let param = match param {
                        "play" => Some(true),
                        "pause" => Some(false),
                        _ => return None,
                    };
                    Some(Msg::Plr(PlayerMsg::Play(param)))
                } else {
                    Some(Msg::Plr(PlayerMsg::Play(None)))
                }
            }
            "next" => Some(Msg::Plr(PlayerMsg::Next)),
            "prev" => Some(Msg::Plr(PlayerMsg::Prev)),
            s if s.starts_with("vu") || s.starts_with("volume-up") => {
                if let Some(param) = get_action_param(action) {
                    let param = match param.parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return None,
                    };
                    Some(Msg::Plr(PlayerMsg::VolumeUp(Some(param))))
                } else {
                    Some(Msg::Plr(PlayerMsg::VolumeUp(None)))
                }
            }
            s if s.starts_with("vd") || s.starts_with("volume-down") => {
                if let Some(param) = get_action_param(action) {
                    let param = match param.parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return None,
                    };
                    Some(Msg::Plr(PlayerMsg::VolumeDown(Some(param))))
                } else {
                    Some(Msg::Plr(PlayerMsg::VolumeDown(None)))
                }
            }
            s if s.starts_with("vol") || s.starts_with("volume") => {
                if let Some(param) = get_action_param(action) {
                    let param = match param.parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return None,
                    };
                    Some(Msg::Plr(PlayerMsg::Volume(param)))
                } else {
                    None
                }
            }
            "shuffle" | "mix" => Some(Msg::Plr(PlayerMsg::Shuffle)),
            "exit" | "close" | "quit" => Some(Msg::Close),
            _ => None,
        }
    }

    /// Prints help for instance
    pub fn help(&self) {
        println!("\x1b[92mInstance actions:\x1b[0m");
        println!("\x1b[93m  pp, play-pause\x1b[90m[=(play|pause)]\x1b[0m");
        println!("    Play or pause, no parameter toggles\n");
        println!("\x1b[93m  next\x1b[0m");
        println!("    Plays the next song\n");
        println!("\x1b[93m  prev\x1b[0m");
        println!("    Plays the previous song\n");
        println!("\x1b[93m  vu, volume-up\x1b[90m[=<f32>]\x1b[0m");
        println!(
            "    Sets volume up by step, w/o parameter uses default step\n"
        );
        println!("\x1b[93m  vd, volume-down\x1b[90m[=<f32>]\x1b[0m");
        println!(
            "    Sets volume down by step, w/o parameter uses default step\n"
        );
        println!("\x1b[93m  vol, volume\x1b[0m=<f32>");
        println!("    Sets volume to given value\n");
        println!("\x1b[93m  shuffle, mix\x1b[0m");
        println!("    Shuffles current playlist\n");
        println!("\x1b[93m  exit, close, quit\x1b[0m");
        println!("    Closes running instance");
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
            invalid: false,
        }
    }
}
