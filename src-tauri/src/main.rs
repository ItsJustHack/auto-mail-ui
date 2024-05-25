#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;

use config::{build_config, build_identity, Config, FormData, Identity};
use error::MailError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use mail_content::{build_email, read_emails, read_template_file, MailConfig};

pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

#[tauri::command]
fn load_mail_config() -> Result<Vec<String>, String> {
    let mail_config: MailConfig = read_emails();
    let mail_names: Vec<String> = mail_config.mails.keys().cloned().collect();
    Ok(mail_names)
}

#[tauri::command]
fn change_message(template_chosen: String) -> (String, String) {
    let mail_config: MailConfig = read_emails();
    let id: Identity = build_identity();

    (
        read_template_file(
            mail_config
                .mails
                .get(&template_chosen)
                .unwrap() // Ne pose pas de problème parce que la clé provient du JS
                .mail_path
                .clone(), // On est pas à 2ms près
        )
        .replace("[Votre nom]", &format!("{} {}", &id.nom, &id.prenom)),
        mail_config
            .mails
            .get(&template_chosen)
            .unwrap()
            .objet
            .clone(),
    )
}

#[tauri::command]
async fn process_form(data: FormData, template_chosen: String) -> Result<(), String> {
    println!("Received form data: {:?}", data);
    // Faites ce que vous voulez avec les données du formulaire ici.
    let config: Config = build_config(&data);
    println!("config ok");
    // FIXME: Mail error n'est pas reconnu (ne satisfait pas les traits)
    let email: Message = build_email(&config, &data, template_chosen)
        .expect("Erreur lors de la construction du mail");
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
        .invoke_handler(tauri::generate_handler![
            process_form,
            load_mail_config,
            change_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
