use configparser::ini::Ini;
use std::error::Error;
use std::io::{stdout, Write};
use text_io::read;
pub mod dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();
  let file: &str = "config/overhaul.ini";
  let _overhaul = config.load(file)?;

  dec::main_menu();

  println!("\nPlease make sure the name of file is the same as when you registered it, if you dont remember then select 4 from main menu.");
  print!("\nName of file you want to update: ");
  stdout().flush().ok();
  let section: String = read!();
  let mut config = Ini::new();
  println!("Please wait as we update your file...");

  let _overhaul = config.load("config/overhaul.ini")?;
  let url = config.get(section.as_str().trim(), "url").unwrap();
  let request = reqwest::get(url).await?.text().await?;
  //println!("body = {:?}", body);
  let request = dec::store_request(request);

  //  let newstr: String = read!();
  //  let _url = config.get(newstr.as_str().trim(), "url").unwrap();
  let loc = config.get("parse_args", "loc").unwrap();
  dec::update_configuration_string(loc.as_str(), request);


  println!("File Updated!");
  dec::main_menu();
  Ok(())
}
