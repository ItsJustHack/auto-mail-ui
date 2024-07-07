#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use crate::config::{build_config, build_identity, Config, FormData, Identity};
use crate::get_resource_path;
use crate::mail_content::{build_email, read_emails, read_template_file, MailConfig};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Serialize;

static CONFIG_FILE_PATH: &str = "./config/config.toml";
static EMAIL_LIST_PATH: &str = "./email_list/aprod_users.csv";

#[derive(Serialize)]
pub struct EmailList {
    email_list: Vec<String>,
}

#[tauri::command]
/// This function returns all the available names for templates using the MailConfig struct
pub fn load_mail_config() -> Result<Vec<String>, String> {
    let mail_config: MailConfig = read_emails();
    let mail_names: Vec<String> = mail_config.mails.keys().cloned().collect();
    Ok(mail_names)
}

#[tauri::command]
/// This function returns all the email already known, inside the "aprod_users.csv file
pub fn get_email_addresses() -> EmailList {
    let path = get_resource_path().unwrap().join(EMAIL_LIST_PATH);
    let temp: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    EmailList { email_list: temp }
}

#[tauri::command]
/// Returns the body and subject of the mail
pub fn change_message(template_chosen: String) -> (String, String) {
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
        // moment
        mail_config
            .mails
            .get(&template_chosen)
            .unwrap()
            .objet
            .clone(),
    )
}

fn read_config_file(path: &str) -> String {
    let path = get_resource_path().unwrap().join(path);
    fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?")
}
use lettre::message::Mailbox;
#[tauri::command]
pub async fn process_form(data: FormData, template_chosen: String) -> Result<(), String> {
    // println!("Received form data: {:?}", data);
    // Faites ce que vous voulez avec les données du formulaire ici.
    let config: Config = build_config(&data, &read_config_file(CONFIG_FILE_PATH));
    //println!("config ok");
    // FIXME: Mail error n'est pas reconnu (ne satisfait pas les traits)
    let email: Message = build_email(&config, &data, template_chosen)
        .expect("Erreur lors de la construction du mail");

    let creds: Credentials = crate::mail_credentials::build_credentials();

    // Open a remote connection to smtp server
    let mailer = SmtpTransport::relay("ssl0.ovh.net")
        .expect("Erreur lors de l'envoi")
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(c) => println!("Mail sent successfully, {:?}", c),
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}
