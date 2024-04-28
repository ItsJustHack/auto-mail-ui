use serde::Deserialize;
use std::fs;
use std::path::Path;

static CONFIG_FILE_PATH: &str = "./config/config.toml";

#[derive(Deserialize)]
pub struct Config {
    // attachements: Vec<String>,
    pub nom: String,
    pub prenom: String,
    pub envoyeur: String,
    pub destinataire: String,
    pub entreprise: String,
}

pub fn build_config() -> Config {
    // This helps to have a universal binary that also works on Windows
    let path: &Path = Path::new(CONFIG_FILE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?");

    toml::from_str(&configuration_file).expect("Mauvais formattage du fichier de configuration")
}
