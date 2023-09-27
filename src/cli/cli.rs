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

    /// Display CLI help
    fn help(&self) {
        print!("Welcome to help for \x1b[92mBump\x1b[0m by ");
        let name = "Martan03";
        let r = 0;
        let g = 220;
        for (i, c) in name.chars().enumerate() {
            print!("\x1b[38;2;{};{};255m{}", r + i * 25, g - i * 20, c);
        }
        println!("\x1b[92m\n\nUsage:\x1b[0m");
        println!("\x1b[93m  bump\x1b[0m");
        println!("    Starts the GUI app");
        println!("\x1b[93m  bump \x1b[90m[action] [parameters]\x1b[0m");
        println!("    Does the given action with given parameters\n");
        println!("\x1b[92mActions:\x1b[0m");
        println!("\x1b[93m  h, -h, --help\x1b[0m");
        println!("    Display help\n");
        println!("\x1b[93m  i, instance \x1b[90m[action]\x1b[0m");
        println!("    Sends message given by action to running instance\n");
        self.instance.help();
    }
}
