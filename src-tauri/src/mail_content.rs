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
        let filebody = fs::read(path).unwrap_or_else(|_| {
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
            create_attachements(h.mails.get(&template_chosen).unwrap())?
                .iter()
                .fold(
                    MultiPart::related().singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body::<String>(
                                r#"
<html><head><meta http-equiv=3D"Content-Type" content=3D"text/html; charset
=3DUTF-8" /></head><body style=3D'font-size: 10pt'>
<div class=3D"pre" style=3D"margin: 0; padding: 0; font-family: monospace">
Sale merde&nbsp;<br />
<div id=3D"signature">-- <br />
<div style=3D"float: left top;">
<table class=3D"table" style=3D"border: 0px;">
<tbody>
<tr>
<td rowspan=3D"6"><img style=3D"height: 160px; text-align: center; vertical-align: middle;" src=3D"http://www.est-horizon.com/upload/FEH-logo.jpg" /></td>
<td style=3D"font-family: Calibri,Arial;"><span style=3D"font-size: 12pt;">
<strong style=3D"font-size: 1.3em;">Thomas BERTOZZO </strong>- <a style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; text-decoration: none;">
<a>Relations Entreprises du Forum Est-Horizon 2024</a></span></td>
</tr>
<tr>
<td style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; font-family: Calibri,Arial;">Elève ingénieur de l'Ecole des Mines Nancy</td>
</tr>
<tr>
<td style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; font-family: Calibri,Arial;">Campus Artem - 92, Rue du Sergent Blandan; 54000 Nancy</td>
</tr>
<tr>
<td style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; font-family: Calibri,Arial;">Téléphone : +33 (0)<span>7 82 64 07 57</span></td>
</tr>
<tr>
<td style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; font-family: Calibri,Arial;">Mail : <span>bertozzo.thomas</span><span>@est</span><span>-horizon</span><span>.com</span></td>
</tr>
<tr>
<td style=3D"color: rgba(64, 64, 64, 1); font-size: 1.1em; font-family: Calibri,Arial;">Site internet : <a style=3D"text-decoration: none; color: rgba(64, 64, 64, 1);" href=3D"http://www.est-horizon.com/">www.est-horizon.com</a></td>
</tr>
</tbody>
</table>
</div>
</div>
</div>
</body></html>
"#
                                .into(),
                            ),
                    ),
                    |acc: MultiPart, el: &SinglePart| acc.singlepart(el.clone()),
                ),
        )?;
    Ok(email)
}
