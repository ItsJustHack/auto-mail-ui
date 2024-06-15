#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::{change_message, get_email_addresses, load_mail_config, process_form};
use error::MailError;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Mutex;

pub mod backend;
pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

lazy_static! {
    static ref RESOURCE_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Obtient le chemin du dossier "resources"
            let resource_path = app.path_resolver().resource_dir();
            *RESOURCE_PATH.lock().unwrap() = resource_path.clone();

            if let Some(ref path) = resource_path {
                println!("Chemin du dossier ressources: {:?}", path);
            } else {
                println!("Impossible de trouver le dossier ressources");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_email_addresses,
            process_form,
            load_mail_config,
            change_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Fonction utilitaire pour obtenir le chemin du dossier "ressources"
pub fn get_resource_path() -> Option<PathBuf> {
    RESOURCE_PATH.lock().unwrap().clone()
}
