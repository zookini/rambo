fn main() -> rambo::error::Result<()> {
    env_logger::init();
    rambo::bot(irc::client::prelude::IrcClient::new("rambo.toml")?)
}
