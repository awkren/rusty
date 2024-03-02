use super::Editor;

pub struct Command {
    pub name: &'static str,
    pub execute: Box<dyn Fn(&mut Editor)>,
}

impl Command {
    pub fn new<F>(name: &'static str, execute: F) -> Self
    where
        F: Fn(&mut Editor) + 'static,
    {
        Self {
            name,
            execute: Box::new(execute),
        }
    }
}

pub fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command_name = parts[0].trim();

    let commands = vec![
        Command::new("wq", |editor| {
            editor.save();
            editor.should_quit = true;
        }), // more commands hereee
    ];

    for command in &commands {
        if command.name == command_name {
            return Some(command.clone());
        }
    }

    None
}
