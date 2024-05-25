use crate::config::{Config, FormData};
use crate::MailError;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::Message;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

pub static EMAIL_TYPE_PATH: &str = "config/email_type.toml";

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
    for file_name in &mail_content.attachements {
        let path = Path::new(file_name);
        let filebody = fs::read(path).expect(&format!(
            "Incapacité de lire la pièce jointe suivante : {:?}, arrêt du programme",
            file_name
        ));
        let content_type = ContentType::parse("application/pdf").unwrap();
        v.push(
            Attachment::new(path.file_name().unwrap().to_str().unwrap().to_owned())
                .body(filebody, content_type),
        )
    }
    Ok(v)
}

pub fn read_template_file(file_path: String) -> String {
    let path = Path::new(&file_path);
    fs::read_to_string(path).unwrap()
}

fn create_id(config: &Config) -> String {
    format!("{} {} <{}>", config.nom, config.prenom, config.envoyeur)
}

pub fn read_emails() -> MailConfig {
    let path = Path::new(EMAIL_TYPE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de type de mail, le fichier a t'il le bon nomet est-il accessible ?");
    toml::from_str(&configuration_file).expect("Mauvais formattage du fichier de configuration")
}

/// This function takes a config as an argument and returns the formatted email
pub fn build_email(
    config: &Config,
    data: &FormData,
    template_chosen: String,
) -> Result<Message, MailError> {
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
                    MultiPart::related().singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(data.message.clone()),
                    ),
                    |acc: MultiPart, el: &SinglePart| acc.singlepart(el.clone()),
                ),
        )?;
    Ok(email)
}
