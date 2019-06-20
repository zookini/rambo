use crate::error::{Error, Result};

use irc::client::prelude::*;
use log::{info, warn};

pub trait Messenger {
    fn init(&self) -> Result<()>;

    fn run(&self, handler: impl Fn(&str) -> Result<String>) -> Result<()>;
}

impl Messenger for IrcClient {
    fn init(&self) -> Result<()> {
        Ok(self.identify()?)
    }

    fn run(&self, handler: impl Fn(&str) -> Result<String>) -> Result<()> {
        Ok(self.for_each_incoming(|msg| {
            if let Command::PRIVMSG(channel, message) = msg.command {
                match handler(&message) {
                    Ok(response) => {
                        info!("{}: {}", channel, response);
                        self.send_privmsg(&channel, &response)
                            .unwrap_or_else(|e| warn!("{}", e))
                    }
                    Err(Error::CommandUnknown) => (),
                    Err(e) => warn!("{}: {}", channel, e),
                }
            }
        })?)
    }
}
