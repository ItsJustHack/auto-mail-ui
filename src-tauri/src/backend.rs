#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use crate::config::{build_config, build_identity, Config, FormData, Identity};
use crate::mail_content::{build_email, read_emails, read_template_file, MailConfig};
use crate::{get_config_path, get_resource_path};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

static CONFIG_FILE_PATH: &str = "config.toml";
static EMAIL_LIST_PATH: &str = "./email_list/aprod_users.csv";

#[derive(Serialize)]
pub struct EmailList {
    email_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationSignatureInformation {
    nom: String,
    prenom: String,
    telephone: String,
    envoyeur: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationCredentialsInformation {
    username: String,
    password: String,
}

#[tauri::command]
pub fn save_config(
    config: RegistrationSignatureInformation,
    credentials: RegistrationCredentialsInformation,
) -> Result<(), String> {
    let config_path = get_config_path().unwrap();

    // Créer le dossier de configuration s'il n'existe pas, pour une raison obscure il peut ne pas
    // exister
    if !config_path.exists() {
        fs::create_dir_all(&config_path).map_err(|e| e.to_string())?;
    }

    let config_file_path = config_path.join("config.toml"); // Chemin vers le fichier de configuration
    let credentials_path = config_path.join("credentials.toml"); // Chemin vers le fichier de credentials

    println!("{:?}", config_path);
    let config_toml = toml::to_string(&config).map_err(|e| e.to_string())?;
    fs::write(config_file_path, config_toml).map_err(|e| e.to_string())?;

    let credentials_toml = toml::to_string(&credentials).map_err(|e| e.to_string())?;
    fs::write(credentials_path, credentials_toml).map_err(|e| e.to_string())?;

    Ok(())
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
    let path = get_config_path().unwrap().join(path);
    println!("{:?}", path);
    fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?")
}

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
