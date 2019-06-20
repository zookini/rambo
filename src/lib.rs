pub mod error;

mod command;
mod messenger;

pub fn bot(messenger: impl messenger::Messenger) -> error::Result<()> {
    let commands = command::Commands::new();

    messenger.init()?;
    messenger.run(|m| commands.handle(m))
}
