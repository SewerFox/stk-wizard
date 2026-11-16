/*
STK Wizard - A management tool for create/edit/modify/etc SuperTuxKart servers, written in Rust.
Copyright (C) 2026 Gustavo Barreira
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
*/

// linkservers.rs -> Handles the server linking mian menu and server listing system

// I don't want to write all "server links"-related functions here because the file would be stupidly long and hard to read/edit if I do this

use std::io;

use crate::utils;
use crate::main;
use crate::server_linking::create;

pub fn init_screen() {
    println!("What you want to do?");
    println!("-> Options menu <-");
    println!("1 - List linked servers");
    println!("2 - Link a existent STK server to Wizard");
    println!("3 - Create and link a STK server to Wizard");
    println!("4 - Go back");

    println!("\nChoose a option to start: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[FATAL] I/O failure"); // If the system fins a I/O error

    handle_menu(&input);
}

// The function below should be removed later
fn handle_menu(input: &String) {
    let converted_input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("[ERROR] You didn't type a valid character, try again."); // If the user types anything, unless a integer number
            0
        }
    };

    match converted_input {
        1 => {  // List linked servers
                utils::clear_screen();
                list_servers();
             },
        2 => {  // Link a existent STK server to Wizard
                utils::clear_screen();
             },
        3 => {  // Create and link a STK server to Wizard
                utils::clear_screen();
                create::init();
             },
        4 => { // Go back
                utils::clear_screen();
                main();
             },
        _ => { // Default / User typed a wrong number option
                utils::clear_screen();
                println!("[ERROR] You didn't type a valid number, try again.\n");
                init_screen();
             },
    }
}

// list_servers -> List all linked servers (WIP)
fn list_servers() {
    println!("WIP")
}