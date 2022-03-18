use text_io::read;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::io::stdout;
// use colored::*; //to add colors

pub fn update_configuration_raw(file: &str, data: &str){
    let mut write = OpenOptions::new()
          .append(true)
          .open(file)
          .expect("Unable to open file");
     write.write_all(data.as_bytes()).expect("Unable to write data");
  }
  
/// Updates file configuration with the data variable being a string.
pub fn update_configuration_string(file: &str, data: String) {
    let mut write = OpenOptions::new()
          .append(true)
          .open(file)
          .expect("Unable to open file");
     write.write_all(data.as_bytes()).expect("Unable to write data");
}


fn add_new() -> Result<(), std::io::Error> {
    let file: &str = "config/overhaul.ini";
    print!("\nFilename: ");
    stdout().flush().ok();
    let _filename: String = read!();

    print!("File Location: ");
    stdout().flush().ok();
    let _fileloc: String = read!();

    print!("File Url: ");
    stdout().flush().ok();
    let _fileurl: String = read!();

    let filename: String = format!("\n\n[{}]", _filename);
    let fileloc: String = format!("\nloc = {}", _fileloc);
    let fileurl: String = format!("\nurl = {}", _fileurl);
    update_configuration_string(file, filename);
    update_configuration_string(file, fileloc);
    update_configuration_string(file, fileurl);
    Ok(())
}

pub fn read(){
    let ans = read!();
    match ans {
        1 => {
            match add_new() {
                Ok(_) => println!("Configuration Added."),
            _ => println!("Failed to add new file to configuration.")
            }
            main_menu();},
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