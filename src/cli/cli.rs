use crate::config::config::Config;

use super::instance::Instance;

pub struct Cli {
    /// Server IP
    ip: String,
    /// Server port
    port: String,
    /// Cli arguments
    args: Vec<String>,
    /// Instance cli parser
    instance: Instance,
}

impl Cli {
    pub fn new(config: &Config, args: Vec<String>) -> Self {
        Self {
            ip: config.get_server_ip().to_owned(),
            port: config.get_server_port().to_owned(),
            args,
            instance: Instance::new(),
        }
    }

    pub fn parse(&mut self) {
        if let Some(arg) = self.args.get(0) {
            match arg.as_str() {
                "h" | "-h" | "--help" => self.help(),
                "i" | "instance" => {
                    let instance_args: Vec<String> = self
                        .args
                        .iter()
                        .skip(1)
                        .map(|arg| arg.to_owned())
                        .collect();
                    self.instance.parse(instance_args);
                    self.instance.submit(&self.ip, &self.port);
                }
                _ => eprintln!("Invalid argument: {arg}"),
            }
        }
    }

    fn help(&self) {}
}
