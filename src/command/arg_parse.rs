use std::env;

#[derive(Debug, Clone)]
pub struct CommandLineArgs {
    _program_name: String,
    command_option: String,
}

impl CommandLineArgs {
    fn new(program_name: &str, command_option: &str) -> Self {
        Self {
            _program_name: program_name.into(),
            command_option: command_option.into(),
        }
    }

    pub fn command_option(&self) -> String {
        self.command_option.clone()
    }
}

pub fn parser() -> Option<CommandLineArgs> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Some(CommandLineArgs::new(&args[0], &args[1])),
        3 => Some(CommandLineArgs::new(&args[0], &args[2])),
        _ => None,
    }
}
