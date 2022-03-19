use configparser::ini::Ini;
use std::error::Error;
use text_io::read;
use reqwest;

pub mod dec;

#[allow(dead_code)]
/// Requests content from a URL.
fn get_request(){}


fn get_all_sections(file: std::collections::HashMap<std::string::String, std::collections::HashMap<std::string::String, std::option::Option<std::string::String>>>){
  println!("\n{:?}\n", file);
}

fn get_specific_section(section_name: String){
  println!("{}", section_name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  // Creating a new configuration for configparser
  let mut config = Ini::new();
  // File location
  let file: &str = "config/overhaul.ini";

  // Load file for cp
  let overhaul = config.load(file)?;

  get_all_sections(overhaul);

  //dec::main_menu();
  
  // let newstr: String = read!();
  // let url = config.get(newstr.as_str().trim(), "url").unwrap();
  // let loc = config.get("parse_args", "loc").unwrap();

  // get_specific_section(url);

  let body = reqwest::get("https://github.com/Maou-Shimazu/Parse-Args/blob/main/include/parse_args.h")
    .await?
    .text()
    .await?;

println!("body = {:?}", body);

    Ok(())
}