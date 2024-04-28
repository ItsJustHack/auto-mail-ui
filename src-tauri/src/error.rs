use lettre::transport::smtp::Error as SmtpError;
use lettre::{SmtpTransport, Transport};
use std::io;

#[derive(Debug)]
pub enum MailError {
    Io(io::Error),
    Mail(lettre::error::Error),
    Smtp(SmtpError),
}

impl From<io::Error> for MailError {
    fn from(err: io::Error) -> Self {
        MailError::Io(err)
    }
}

impl From<lettre::error::Error> for MailError {
    fn from(err: lettre::error::Error) -> Self {
        MailError::Mail(err)
    }
}

impl From<SmtpError> for MailError {
    fn from(err: <SmtpTransport as Transport>::Error) -> Self {
        MailError::Smtp(err)
    }
}
