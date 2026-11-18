/*
STK Wizard - A management tool for create/edit/modify/etc 
SuperTuxKart servers, written in Rust.

Copyright (C) 2026 Gustavo Barreira

This program is free software: you can redistribute it 
and/or modify it under the terms of the GNU General Public License 
as published by the Free Software Foundation, either version 3 
of the License, or (at your option) any later version.
This program is distributed in the hope that it will be useful, 
but WITHOUT ANY WARRANTY; without even the implied warranty of 
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the 
GNU General Public License for more details.

You should have received a copy of the GNU General Public License 
along with this program. If not, see <https://www.gnu.org/licenses/>. 
*/

// create.rs -> Handles the server creation system.

use serde::{Deserialize, Serialize};
use std::fs;

// Recreating clients.toml structure
#[derive(Debug, Serialize, Deserialize)]
struct Clients {
    default_usr: String,
    default_local: String,
    // It should have something to add new entries,
    // but in another file.rs
}
 #[derive(Debug, Serialize, Deserialize)]
struct Config {
    clients: Clients,
}

// Initializing clients.toml and options menu (in another function)
pub fn init() -> Result<(), Box<dyn std::error::Error>> { 
    let mut config: String = "".to_string(); // clients.toml

    if let Some(mut user_config) = dirs::config_dir() { 
        // clients.toml path -> ~/.config/supertuxkart-wizard/clients.toml
        user_config.push("supertuxkart-wizard");
        user_config.push("clients.toml");

        if user_config.exists() {
            config = fs::read_to_string(user_config).map_err(|e| e.to_string())?;
        } else { // Creating a new clients.toml if needed
            println!("[WARN] Was detected that you don't have any 'client.toml' ");
            println!("file in your server/computer. Creating a new file...");

            let content: &str = r#"[clients]
default_usr = "/usr/bin/supertuxkart"
default_local = "/usr/local/bin/supertuxkart"
            "#;  

            fs::write(&user_config, content)?;
            config = fs::read_to_string(user_config).map_err(|e| e.to_string())?;
        }    
    }

    let clients_config_read: Config = toml::from_str(&config)?;
    display_menu(clients_config_read);

    Ok(())
}

fn display_menu(toml: Config) {
    // TODO: Create a menu with all client options
    println!("client.toml = {}", toml.clients.default_local);
}