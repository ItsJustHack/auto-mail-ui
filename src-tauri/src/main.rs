#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use backend::{change_message, get_email_addresses, load_mail_config, process_form, save_config};
use error::MailError;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, Menu, WindowBuilder, WindowMenuEvent};
pub mod backend;
pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

lazy_static! {
    static ref RESOURCE_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref CONFIG_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
}

fn main() {
    // Menu pour la configuration
    let change_config = CustomMenuItem::new("change-config".to_string(), "Changer configuration");
    let menu = Menu::new().add_item(change_config);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event: WindowMenuEvent| match event.menu_item_id() {
            "change-config" => {
                let _new_window = WindowBuilder::new(
                    &event.window().app_handle(),
                    "config",
                    tauri::WindowUrl::App("configuration.html".into()),
                )
                .title("Changer configuration")
                .build()
                .unwrap();
            }
            _ => {}
        })
        .setup(|app| {
            // Obtient le chemin du dossier "resources"
            let resource_path = app.path_resolver().resource_dir();
            let config_path = app.path_resolver().app_config_dir();
            println!("{:?}", config_path);
            *CONFIG_PATH.lock().unwrap() = config_path.clone();
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
            change_message,
            save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Fonction utilitaire pour obtenir le chemin du dossier "ressources"
pub fn get_resource_path() -> Option<PathBuf> {
    RESOURCE_PATH.lock().unwrap().clone()
}

pub fn get_config_path() -> Option<PathBuf> {
    CONFIG_PATH.lock().unwrap().clone()
}
