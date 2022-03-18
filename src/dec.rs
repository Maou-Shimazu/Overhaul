use text_io::read;
use std::io::{self, Write};

fn add_new(){
    println!("adding new yeet");
}

pub fn read(){
    let ans = read!();
    match ans {
        1 => {add_new(); main_menu();},
        5 => std::process::exit(0),
        _=> {println!("option {} is not available, please try another option.", ans); main_menu();},
    }
}

/// The main menu 
pub fn main_menu(){
    println!("\nWelcome to OverHaul.");
    println!("----------------------");
    println!("[1] Add New File.");
    println!("[2] Update File.");
    println!("[3] Update all files.");
    println!("[4] -------------------");
    print!("[5] Exit.\nWhat would you like to do?: ");
    let _ = io::stdout().flush();
    read();
}