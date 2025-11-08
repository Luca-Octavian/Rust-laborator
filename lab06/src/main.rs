use std::fs;

trait Parser {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;
impl Parser for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, args: &[&str]) {
        if !args.is_empty() {
            println!("comanda ping nu accepta argumente");
        } else {
            println!("pong!");
        }
    }
}

struct CountCommand;
impl Parser for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) {
        println!("numarul de argumente este: {}", args.len());
    }
}

struct TimesCommand {
    u: u32,
}
impl Parser for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, args: &[&str]) {
        if !args.is_empty() {
            println!("comanda times nu accepta argumente");
        } else {
            self.u += 1;
            println!("apelat de {} ori", self.u);
        }
    }
}

struct LengthCommand;
impl Parser for LengthCommand {
    fn get_name(&self) -> &str {
        "length"
    }

    fn exec(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("comanda length necesita macar un argument");
        } else {
            let mut total_chars = 0;
            for arg in args {
                for _c in arg.chars() {
                    total_chars += 1;
                }
            }
            println!("numarul total de caractere din argumente este: {}", total_chars);
        }
    }
}

fn stop_command() {
    println!("opresc executia...");
}

struct Terminal {
    commands: Vec<Box<dyn Parser>>,
}

impl Terminal {
    fn new() -> Self {
        Self { commands: Vec::new() }
    }

    fn register(&mut self, cmd: Box<dyn Parser>) {
        self.commands.push(cmd);
    }

    fn run(&mut self) {
        let file_path = "fisier.txt";

        if let Ok(s) = fs::read_to_string(file_path) {
            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split_whitespace().collect();
                let cmd_name = parts[0];
                let args = &parts[1..];

                if cmd_name == "stop" {
                    stop_command();
                    break;
                }

                let mut found = false;
                for cmd in &mut self.commands {
                    if cmd.get_name() == cmd_name {
                        cmd.exec(args);
                        found = true;
                        break;
                    }
                }

                if !found {
                    let mut suggested = None;
                    let lower_cmd = cmd_name.to_lowercase();
                    for cmd in &self.commands {
                        if cmd.get_name() == lower_cmd {
                            suggested = Some(cmd.get_name());
                            break;
                        }
                    }
                    if let Some(suggestion) = suggested {
                        eprintln!(
                            "comandă necunoscută '{}'. Ai vrut să pui '{}'?",
                            cmd_name, suggestion
                        );
                    } else {
                        eprintln!("comandă necunoscută '{}'", cmd_name);
                    }
                }
            }
        } else {
            eprintln!("ncu s-a putut citi fișierul '{}'", file_path);
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand));
    terminal.register(Box::new(LengthCommand));
    terminal.register(Box::new(CountCommand));
    terminal.register(Box::new(TimesCommand { u: 0 }));

    terminal.run();
}
