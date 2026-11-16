/*
STK Wizard - A management tool for create/edit/modify/etc SuperTuxKart servers, written in Rust.
Copyright (C) 2026 Gustavo Barreira
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
*/

// main.rs -> Main file of this project. Handles the main menu screen.

use std::io;

mod utils;
mod linkserver;
mod tutorial;
mod about;

fn handle_menu(input: &String) {
    let converted_input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("[ERROR] You didn't type a valid character, try again."); // If the user types anything, unless a integer number
            main(); // Returns to main menu
            0
        }
    };

    match converted_input {
        1 => {  // Link a STK server to Wizard
                utils::clear_screen();
                println!("[SELECTED] 1 - Link a STK server to Wizard");
                linkserver::init_screen();
             },
        2 => {  // Read the tutorial
                utils::clear_screen();
                println!("[SELECTED] 2 - Read the tutorial");
                tutorial::init_screen();
             },
        3 => {  // About us
                utils::clear_screen();
                println!("[SELECTED] 3 - About this project");
                about::init_screen();
             },
        4 => { // Exit
                utils::clear_screen();
                println!("[SELECTED] 4 - Exit");
                return;
             },
        _ => { // Default / User typed a worng number option
                utils::clear_screen();
                println!("[ERROR] You didn't type a valid number, try again.\n");
                main();
             },
    }
}

fn main() {
    println!("Welcome to STK Wizard!");
    println!("-> Options menu <-");
    println!("1 - Link a STK server to Wizard");
    println!("2 - Read the tutorial");
    println!("3 - About this project");
    println!("4 - Exit");

    println!("\nChoose a option to start: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[FATAL] I/O failure"); // If the system fins a I/O error

    handle_menu(&input);
}
