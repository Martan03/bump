use crate::config::Config;

use super::instance::Instance;

pub struct Cli {
    /// Server IP
    ip: String,
    /// Server port
    port: String,
    /// Instance cli parser
    instance: Instance,
}

impl Cli {
    /// Creates new Cli
    pub fn new(config: &Config) -> Self {
        Self {
            ip: config.get_server_ip().to_owned(),
            port: config.get_server_port().to_owned(),
            instance: Instance::new(),
        }
    }

    /// Parses given arguments
    pub fn parse(&mut self, args: Vec<String>) {
        if let Some(arg) = args.first() {
            match arg.as_str() {
                "h" | "-h" | "--help" => self.help(),
                "i" | "instance" => {
                    let instance_args: Vec<String> = args
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

    /// Displays CLI help
    fn help(&self) {
        print!("Welcome to \x1b[92mBump\x1b[0m by ");
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
        println!("\x1b[93m  i, instance \x1b[90m[instance-action]\x1b[0m");
        println!("    Sends message given by action to running instance\n");
        self.instance.help();
    }
}
