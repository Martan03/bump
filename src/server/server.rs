use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use crate::gui::app::{Msg, PlayerMsg};

pub struct Server;

impl Server {
    pub fn handle_client(mut stream: &TcpStream) -> Option<Msg> {
        let buf_reader = BufReader::new(&mut stream);
        let request = buf_reader.lines().next().unwrap().unwrap();

        let (status, filename) = match request.as_str() {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
            "GET /instance/play HTTP/1.1" => {
                return Some(Msg::Plr(PlayerMsg::Play(None)));
            }
            s if s.starts_with("GET /icons") => {
                let parts: Vec<_> = request.split_whitespace().collect();
                let filename = parts[1];
                let image = fs::read_to_string(format!("assets/{filename}"))
                    .unwrap_or(String::from(""));
                Server::send_response(
                    stream,
                    "HTTP/1.1 200 OK".to_owned(),
                    "image/svg+xml".to_owned(),
                    image,
                );
                return None;
            }
            _ => {
                return match serde_json::from_str::<Msg>(&request) {
                    Ok(msg) => Some(msg),
                    Err(_) => None,
                }
            }
        };

        let content =
            fs::read_to_string("src/server/".to_owned() + filename).unwrap();
        let len = content.len();
        let response =
            format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap();
        None
    }

    fn send_response(
        mut stream: &TcpStream,
        status: String,
        content_type: String,
        content: String,
    ) {
        let len = content.len();
        let response = format!(
            "{}\r\nContent-Type: {}\nContent-Length: {}\r\n\r\n{}",
            status, content_type, len, content
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn send_cli_response(mut stream: &TcpStream, msg: &str) {
        _ = stream.write_all(msg.as_bytes());
    }
}
