use crate::config::Config;
use crate::MailError;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::Message;
use serde::Deserialize;
use std::path::Path;
use std::{fs, io};

static EMAIL_TYPE_PATH: &str = "config/email_type.toml";

trait TypeMail {
    fn mail_content(&self) -> MailContent;
}

#[derive(Deserialize)]
struct PremierContact {
    mail_path: String,
    objet: String,
    attachements: Vec<String>,
}

#[derive(Deserialize)]
struct Relance {
    mail_path: String,
    objet: String,
    attachements: Vec<String>,
}

#[derive(Deserialize)]
pub struct MailContent {
    pub mail_path: String,
    pub objet: String,
    pub attachements: Vec<String>,
}

#[derive(Deserialize)]
struct ChooseEmail {
    pub typemail: u8,
    pub premiercontact: PremierContact,
    pub relance: Relance,
}

impl TypeMail for PremierContact {
    fn mail_content(&self) -> MailContent {
        MailContent {
            mail_path: self.mail_path.clone(),
            objet: self.objet.clone(),
            attachements: self.attachements.clone(),
        }
    }
}

impl TypeMail for Relance {
    fn mail_content(&self) -> MailContent {
        MailContent {
            mail_path: self.mail_path.clone(),
            objet: self.objet.clone(),
            attachements: self.attachements.clone(),
        }
    }
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

fn remplace_text(file_path: String, config: &Config) -> Result<String, io::Error> {
    let path = Path::new(&file_path);
    let original_text = fs::read_to_string(path)?;
    let original_text = original_text.replace("[Nom de l'entreprise]", &config.entreprise);
    Ok(original_text.replace(
        "[Votre nom]",
        &format!("{} {}", &config.nom, &config.prenom),
    ))
}

fn create_id(config: &Config) -> String {
    format!("{} {} <{}>", config.nom, config.prenom, config.envoyeur)
}

fn choose_email_content(email_type: &ChooseEmail) -> MailContent {
    match email_type.typemail {
        1 => email_type.premiercontact.mail_content(),
        2 => email_type.relance.mail_content(),
        _ => panic!("Type de mail non valide"),
    }
}

fn build_email_type() -> ChooseEmail {
    let path = Path::new(EMAIL_TYPE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de type de mail, le fichier a t'il le bon nomet est-il accessible ?");

    toml::from_str(&configuration_file).expect("Mauvais formattage du fichier de configuration")
}

/// This function takes a config as an argument and returns the formatted email
pub fn build_email(config: &Config) -> Result<Message, MailError> {
    let mail_content = choose_email_content(&build_email_type());
    let email = Message::builder()
        .from(create_id(config).parse().unwrap())
        .to(config.destinataire.parse().unwrap())
        .bcc(config.envoyeur.parse().unwrap())
        .subject(&mail_content.objet)
        .multipart(
            // Attache tous les pièces jointes, magie noire parce que j'ai la flemme d'expliquer
            create_attachements(&mail_content)?.iter().fold(
                MultiPart::related().singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(remplace_text(mail_content.mail_path, config)?),
                ),
                |acc: MultiPart, el: &SinglePart| acc.singlepart(el.clone()),
            ),
        )?;
    Ok(email)
}
