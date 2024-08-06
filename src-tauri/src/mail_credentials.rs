use crate::get_config_path;
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use std::fs;

static CREDENTIAL_PATH: &str = "credentials.toml";

#[derive(Deserialize)]
struct MailCredentials {
    username: String,
    password: String,
}

pub fn build_credentials() -> Credentials {
    let path = get_config_path().unwrap().join(CREDENTIAL_PATH);
    let creds = fs::read_to_string(path).expect("incapacité de lire le fichier de credentials");
    let mail_credentials: MailCredentials =
        toml::from_str(&creds).expect("Fichier de credentials mal formatté");
    Credentials::new(
        mail_credentials.username.to_owned(),
        mail_credentials.password.to_owned(),
    )
}
