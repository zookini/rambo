use crate::error::{Error, Result};

pub(super) struct Commands(Vec<Box<Command>>);

impl Commands {
    pub(super) fn new() -> Commands {
        Commands(vec![Box::new(Hello), Box::new(Bye)])
    }

    pub(super) fn handle(&self, message: &str) -> Result<String> {
        let mut split = message.splitn(2, ' ');
        let command = split.next().unwrap();

        match self.0.iter().find(|c| c.name() == command) {
            Some(c) => c.run(&split.next()),
            None => Err(Error::CommandUnknown),
        }
    }
}

trait Command {
    fn name(&self) -> &'static str;

    fn run(&self, args: &Option<&str>) -> Result<String>;
}

struct Hello;

impl Command for Hello {
    fn name(&self) -> &'static str {
        "!hello"
    }

    fn run(&self, _: &Option<&str>) -> Result<String> {
        Ok("Hello there!".into())
    }
}

struct Bye;

impl Command for Bye {
    fn name(&self) -> &'static str {
        "!bye"
    }

    fn run(&self, _: &Option<&str>) -> Result<String> {
        Ok("Good bye!".into())
    }
}
