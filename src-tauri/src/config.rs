use serde::Deserialize;
use std::fs;
use std::path::Path;

static CONFIG_FILE_PATH: &str = "./config/config.toml";

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    entreprise: String,
    subject: String,
    message: String,
}

#[derive(Deserialize)]
pub struct Identity {
    pub nom: String,
    pub prenom: String,
}

#[derive(Deserialize)]
pub struct FileConfig {
    pub nom: String,
    pub prenom: String,
    pub envoyeur: String,
}

pub struct Config {
    pub nom: String,
    pub prenom: String,
    pub envoyeur: String,
    pub destinataire: String,
    pub entreprise: String,
}

pub fn build_identity() -> Identity {
    let path: &Path = Path::new(CONFIG_FILE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?");
    // TODO: Renvoyez une erreur
    toml::from_str(&configuration_file).unwrap()
}

pub fn build_config(form_data: &FormData) -> Config {
    // This helps to have a universal binary that also works on Windows
    let path: &Path = Path::new(CONFIG_FILE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?");

    let file_config: FileConfig = toml::from_str(&configuration_file)
        .expect("Mauvais formattage du fichier de configuration");
    Config {
        nom: file_config.nom.clone(),
        prenom: file_config.prenom.clone(),
        envoyeur: file_config.envoyeur.clone(),
        destinataire: form_data.email.clone(),
        entreprise: form_data.entreprise.clone(),
    }
}
