use serde::Deserialize;
use std::fs;
use std::path::Path;

static CONFIG_FILE_PATH: &str = "./config/config.toml";

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    entreprise: String,
    pub subject: String,
    pub message: String,
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
    pub telephone: String,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub nom: String,
    pub prenom: String,
    pub envoyeur: String,
    pub destinataire: String,
    pub entreprise: String,
    pub telephone: String,
}

pub fn build_identity() -> Identity {
    let path: &Path = Path::new(CONFIG_FILE_PATH);
    let configuration_file = fs::read_to_string(path).expect("Incapacité de lire le fichier de configuration, le fichier a t'il le bon nomet est-il accessible ?");
    // TODO: Renvoyez une erreur
    toml::from_str(&configuration_file).unwrap()
}

/// This function takes a FormData which is parsed when the form is sent and the content of the configuration file in a string
pub fn build_config(form_data: &FormData, configuration_file: &str) -> Config {
    let file_config: FileConfig = toml::from_str(configuration_file)
        .expect("Mauvais formattage du fichier de configuration");
    Config {
        nom: file_config.nom.clone(),
        prenom: file_config.prenom.clone(),
        envoyeur: file_config.envoyeur.clone(),
        telephone: file_config.telephone.clone(),
        destinataire: form_data.email.clone(),
        entreprise: form_data.entreprise.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_build_config_panic() {
        let f: FormData = FormData {
            email: "dummy@example.com".into(),
            entreprise: "Renault".into(),
            subject: "Mon objet de mail".into(),
            message: "Corps de mail".into(),
        };
        build_config(&f, "");
    }

    #[test]
    fn test_build_config() {
        let f: FormData = FormData {
            email: "dummy@example.com".into(),
            entreprise: "Renault".into(),
            subject: "Mon objet de mail".into(),
            message: "Corps de mail".into(),
        };
        let config_str = r#"
        envoyeur = "email@est-horizon.com"
        nom = "nom"
        prenom = "prenom"
        "#;
        assert_eq!(
            build_config(&f, config_str),
            Config {
                nom: "nom".into(),
                prenom: "prenom".into(),
                envoyeur: "email@est-horizon.com".into(),
                telephone: "07 83 92 39 13".into(),
                destinataire: "dummy@example.com".into(),
                entreprise: "Renault".into()
            }
        )
    }
}
