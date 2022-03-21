use configparser::ini::Ini;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::io::{stdout, Write};
use std::path::Path;
use text_io::read;
// use colored::*; //to add colors

pub fn configdir() -> std::path::PathBuf {
    dirs::home_dir().unwrap()
}

/// Get specific key from overhaul.ini. takes a String which is assigned config.get() and returns the key for usage;
pub fn get_specific_key(key: String) -> String {
    key
}
/// Updates file configuration with the data variable being a string.
pub fn update_configuration_string(file: &str, data: String) {
    let mut write = OpenOptions::new()
        .append(true)
        .open(file)
        .expect("Unable to open file");
    write
        .write_all(data.as_bytes())
        .expect("Unable to write data");
}

/// Add new configuration file to overhaul.ini
pub fn add_new() -> Result<(), std::io::Error> {
    let path: String = format!("{}\\.overhaul\\config\\overhaul.ini", configdir().to_str().unwrap());
    let url: String = format!("{}\\.overhaul\\config\\url.ini", configdir().to_str().unwrap());
    let loc: String = format!("{}\\.overhaul\\config\\loc.ini", configdir().to_str().unwrap());

    print!("\nFilename: ");
    stdout().flush().ok();
    let _filename: String = read!();

    print!("File Url: ");
    stdout().flush().ok();
    let _fileurl: String = read!();
    
    print!("File Location: ");
    stdout().flush().ok();
    let _fileloc: String = read!();
    

    let filename: String = format!("[{}]\n", _filename);
    let fileloc: String = format!("loc = {}\n", _fileloc);
    let fileurl: String = format!("url = {}\n", _fileurl);

    let _fileurl = format!("{}\n", _fileurl);
    let _fileloc = format!("{}\n", _fileloc);

    update_configuration_string(path.as_str(), filename); 
    update_configuration_string(path.as_str(), fileurl); 
    update_configuration_string(path.as_str(), fileloc);

    update_configuration_string(url.as_str(), _fileurl);
    update_configuration_string(loc.as_str(), _fileloc);

    Ok(())
}

/// Read config/overhaul.ini to stdout
pub fn get_config() {
    let path: String = format!("{}/.overhaul/config/overhaul.ini", configdir().to_str().unwrap());
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    println!("\n{}", contents);
}
/// stores get_request
pub fn store_request(request: String) -> String {
    request
}

/// Returns Location from provided section.
pub fn get_location(section: &str) -> String {
    return Ini::new().get(section.trim(), "loc").unwrap();
}
/// Takes filename as str and data to be writen to the file as String, to be used with store_request()
pub fn write_to_file(file: &str, data: String) {
    let mut write = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .expect("Unable to open file");
    write
        .write_all(data.as_bytes())
        .expect("Unable to write data");
}

pub fn update_file() -> Result<(), Box<dyn Error>> {
    
    Ok(())
}


/// Read input for main menu.
pub fn read_input_main() {
    let ans: i8 = read!();
    match ans {
        1 => {
            // Adding new file
            match add_new() {
                Ok(_) => println!("Configuration Added."),
                _ => println!("Failed to add new file to configuration."),
            }
            main_menu();
        }

        2 => (), //move to main

        3 => {
            // Getting all information from overhaul.ini
            get_config();
            main_menu();
        }

        0 => {
            // Exit process
            println!("Thank you for using Overhaul.");
            std::process::exit(0)
        }

        _ => {
            println!(
                "option {} is not available, please try another option.",
                ans
            );
            main_menu();
        }
    }
}

/// The main menu
pub fn main_menu() {
    println!("\nWelcome to OverHaul.");
    println!("----------------------");
    println!("[0] Exit Overhaul");
    println!("[1] Add New File.");
    println!("[2] Update Options.");
    println!("[3] Show all stored files.");
    print!("What would you like to do?: ");
    stdout().flush().ok();
    read_input_main();
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
