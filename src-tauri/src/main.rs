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

#[derive(serde::Deserialize, Debug)]
struct FormData {
    email: String,
    entreprise: String,
    subject: String,
    message: String,
}

#[tauri::command]
fn process_form(data: FormData) -> Result<String, String> {
    println!("Received form data: {:?}", data);
    // Faites ce que vous voulez avec les données du formulaire ici.
    Ok("Form submission successful!".into())
}

fn main() -> Result<(), MailError> {
    // On admettra ici que votre fichier est à la racine du projet.
    let config: Config = build_config();
    println!("config ok");
    let email: Message = build_email(&config)?;
    println!("email ok");
    let creds: Credentials = mail_credentials::build_credentials();
    println!("creds ok");

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
        .invoke_handler(tauri::generate_handler![process_form])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
