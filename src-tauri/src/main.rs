#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;

use config::{build_config, Config, FormData};
use error::MailError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use mail_content::build_email;

pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

#[tauri::command]
fn process_form(data: FormData) -> Result<(), String> {
    println!("Received form data: {:?}", data);
    // Faites ce que vous voulez avec les données du formulaire ici.
    let config: Config = build_config(&data);
    println!("config ok");
    // FIXME: Mail error n'est pas reconnu (ne satisfait pas les traits)
    let email: Message = build_email(&config).expect("Erreur lors de la construction du mail");
    println!("email ok");
    let creds: Credentials = mail_credentials::build_credentials();
    println!("creds ok");

    // Open a remote connection to smtp server
    let mailer = SmtpTransport::relay("ssl0.ovh.net")
        .expect("Erreur lors de l'envoi")
        .credentials(creds)
        .build();

    // Send the email
    println!("Prêt à l'envoi");
    match mailer.send(&email) {
        Ok(_) => println!("Mail sent successfully"),
        Err(e) => println!("{:?}", e),
    }
    println!("Mail normalement envoyé");
    //
    Ok(())
}

fn main() -> () {
    // On admettra ici que votre fichier est à la racine du projet.
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_form])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
