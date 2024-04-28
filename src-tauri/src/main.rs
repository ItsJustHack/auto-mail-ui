#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;

use config::{build_config, Config};
use error::MailError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use mail_content::build_email;

pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

fn main() -> Result<(), MailError> {
    // On admettra ici que votre fichier est à la racine du projet.
    let config: Config = build_config();
    let email: Message = build_email(&config)?;
    let creds: Credentials = mail_credentials::build_credentials();

    // Open a remote connection to smtp server
    let mailer = SmtpTransport::relay("ssl0.ovh.net")?
        .credentials(creds)
        .build();

    // Send the email
    //match mailer.send(&email) {
    //Ok(_) => println!("Mail sent successfully"),
    //Err(e) => println!("{:?}", e),
    //}
    //
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
