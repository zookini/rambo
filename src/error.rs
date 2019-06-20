use failure::Fail;
use irc::error::IrcError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IRC error {}", _0)]
    Client(#[cause]     IrcError),
    #[fail(display = "Unknown command")]
    CommandUnknown,
}

impl From<IrcError> for Error {
    fn from(e: IrcError) -> Error {
        Error::Client(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
