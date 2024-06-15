use crate::config::{Config, FormData};
use crate::get_resource_path;
use crate::MailError;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::Message;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

pub static EMAIL_TYPE_PATH: &str = "config/email_type.toml";
pub static SIGNATURE_PATH: &str = "mails/signature";

#[derive(Deserialize)]
pub struct MailConfig {
    #[serde(flatten)]
    pub mails: HashMap<String, MailContent>,
}

#[derive(Deserialize)]
pub struct MailContent {
    pub mail_path: String,
    pub objet: String,
    pub attachements: Vec<String>,
}

fn create_attachements(mail_content: &MailContent) -> Result<Vec<SinglePart>, io::Error> {
    // Create attachements with pdf
    let mut v = Vec::new();
    let resource_dir = get_resource_path().unwrap();
    for file_name in &mail_content.attachements {
        let path = resource_dir.join(file_name);
        let filebody = fs::read(path.clone()).unwrap_or_else(|_| {
            panic!(
                "Incapacité de lire la pièce jointe suivante : {:?}, arrêt du programme",
                file_name
            )
        });
        let content_type = ContentType::parse("application/pdf").unwrap();
        v.push(
            Attachment::new(path.file_name().unwrap().to_str().unwrap().to_owned())
                .body(filebody, content_type),
        )
    }
    Ok(v)
}

pub fn read_template_file(file_path: String) -> String {
    let path = get_resource_path().unwrap().join(&file_path);
    fs::read_to_string(path).unwrap()
}

fn create_id(config: &Config) -> String {
    format!("{} {} <{}>", config.nom, config.prenom, config.envoyeur)
}

pub fn read_emails() -> MailConfig {
    let path = get_resource_path().unwrap().join(EMAIL_TYPE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de type de mail, le fichier a t'il le bon nomet est-il accessible ?");
    toml::from_str(&configuration_file).expect("Mauvais formattage du fichier de configuration")
}

fn change_signature(config: &Config) -> String {
    let path = get_resource_path().unwrap().join(SIGNATURE_PATH);
    fs::read_to_string(path)
        .expect("Temporary error")
        .replace("\n", "")
        .replace("[Nom]", &format!("{} {}", config.nom, config.prenom))
        .replace("[Mail]", &config.envoyeur)
        .replace("[Telephone]", &config.telephone)
}

/// This function takes a config as an argument and returns the formatted email
pub fn build_email(
    config: &Config,
    data: &FormData,
    template_chosen: String,
) -> Result<Message, MailError> {
    let header: String = r#"
        <html><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/></head><body style='font-size: 10pt'>"#
    .into();

    let h: MailConfig = read_emails();
    let email = Message::builder()
        .from(create_id(config).parse().unwrap())
        .to(config.destinataire.parse().unwrap())
        .bcc(config.envoyeur.parse().unwrap())
        .subject(&data.subject)
        .multipart(
            // Attache tous les pièces jointes, magie noire parce que j'ai la flemme d'expliquer
            create_attachements(&h.mails.get(&template_chosen).unwrap())?
                .iter()
                .fold(
                    MultiPart::related().singlepart(SinglePart::html(format!(
                        "{}{}{}",
                        header,
                        data.message.clone().replace("\n", "<br>"), // To transform into HTML, very moche but I don't care for the moment
                        change_signature(&config)
                    ))),
                    |acc: MultiPart, el: &SinglePart| acc.singlepart(el.clone()),
                ),
        )?;
    Ok(email)
}
