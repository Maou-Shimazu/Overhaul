use configparser::ini::Ini;
use std::env;
use std::error::Error;
use std::io::{stdout, Write};
use text_io::read;
use std::fs;
pub mod dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => (),
        2 => match args[1].as_str() {
            "-config" => {
                let base: String = format!("{}\\.overhaul\\config", dec::configdir().to_str().unwrap());
                if !std::path::Path::new(base.as_str()).is_dir(){
                    let url: String = format!("{}\\url.ini", base);
                    let loc: String = format!("{}\\loc.ini", base);
                    let overhaul: String = format!("{}\\overhaul.ini", base);
                    fs::create_dir_all(base)?;
                    fs::File::create(url)?;
                    fs::File::create(loc)?;
                    fs::File::create(overhaul)?;
                    println!("All configs created, type `overhaul` to get started.");
                }
                else {
                    println!("Directory already exists.");
                }
               
                std::process::exit(0);
            }
            _ => {
                println!("Please type `overhaul -config` to set up configuration.");
                std::process::exit(0);
            }
        },
        _ => println!("Invalid argument, type `overhaul -config` then `overhaul` to get started."),
    }

    while 1 == 1 {
        dec::main_menu();

        println!("\nUpdate Options.");
        println!("----------------------");
        println!("[0] Main Menu.");
        println!("[1] Update File.");
        println!("[2] Update all files.");
        print!("What would you like to do?: ");
        stdout().flush().ok();

        let input: i32 = read!();
        match input {
            0 => dec::main_menu(),
            1 => {
                println!("\nPlease make sure the name of file is the same as when you registered it, if you dont remember then select 4 from main menu.");
                print!("\nName of file you want to update: ");
                stdout().flush().ok();
                let section: String = read!();
                let mut config = Ini::new();
                //println!("Please wait as we update your file...");
                let path: String = format!(
                    "{}\\.overhaul\\config\\overhaul.ini",
                    dec::configdir().to_str().unwrap()
                );
                let _overhaul = config.load(path)?;
                let url = config.get(section.as_str().trim(), "url").unwrap();
                println!("Fetching {}", url.clone());
                let request = reqwest::get(url).await?.text().await?;

                let request = dec::store_request(request);
                let loc = config.get(section.as_str().trim(), "loc").unwrap();
                println!("Updating {}", loc.clone());
                dec::write_to_file(loc.as_str(), request);
            }
            2 => {
                let _url: String = format!(
                    "{}/.overhaul/config/url.ini",
                    dec::configdir().to_str().unwrap()
                );
                let _loc: String = format!(
                    "{}/.overhaul/config/loc.ini",
                    dec::configdir().to_str().unwrap()
                );

                let loc: String = String::from_utf8(std::fs::read(_loc)?).unwrap();
                let url: String = String::from_utf8(std::fs::read(_url)?).unwrap();

                for (urls, locs) in url.lines().zip(loc.lines()) {
                    println!("\nFetching {}", urls);
                    println!("Updating {}\n", locs);
                    let request = reqwest::get(urls).await?.text().await?;
                    let request = dec::store_request(request);
                    dec::write_to_file(locs, request);
                }
            }
            _ => println!("option {} not available.", input),
        }
    }
    Ok(())
}
