#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::{change_message, get_email_addresses, load_mail_config, process_form};
use error::MailError;
use tauri::api::shell::open;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

pub mod backend;
pub mod config;
pub mod error;
pub mod mail_content;
pub mod mail_credentials;

fn open_file_with_default_editor(window: &tauri::Window, file_path: &str) {
    let _ = open(&window.shell_scope(), file_path, None);
}

fn main() {
    // On admettra ici que votre fichier est à la racine du projet.
    let open_config_1 = CustomMenuItem::new("open_config_1".to_string(), "Ouvrir Config 1");
    let open_config_2 = CustomMenuItem::new("open_config_2".to_string(), "Ouvrir Config 2");
    let open_config_3 = CustomMenuItem::new("open_config_3".to_string(), "Ouvrir Config 3");

    // Define the menu
    let file_menu = Submenu::new(
        "Fichier",
        Menu::new()
            .add_item(open_config_1)
            .add_item(open_config_2)
            .add_item(open_config_3),
    );

    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(file_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open_config_1" => {
                // Handle the "Ouvrir Config 1" action
                // You can send a message to the front-end or handle it directly here
                println!("Ouvrir Config 1 selected");
                let file_path = "../config/config.toml";
                open_file_with_default_editor(event.window(), file_path);
            }
            "open_config_2" => {
                // Handle the "Ouvrir Config 2" action
                println!("Ouvrir Config 2 selected");
            }
            "open_config_3" => {
                // Handle the "Ouvrir Config 3" action
                println!("Ouvrir Config 3 selected");
            }
            _ => {}
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
