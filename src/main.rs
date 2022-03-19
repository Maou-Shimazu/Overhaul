use configparser::ini::Ini;
use std::error::Error;
use std::io::{stdout, Write};
use text_io::read;
pub mod dec;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();
  let _overhaul = config.load(dec::PATH)?;
  
  while 1==1 {
    dec::main_menu();
    println!("\nPlease make sure the name of file is the same as when you registered it, if you dont remember then select 4 from main menu.");
    print!("\nName of file you want to update: ");
    stdout().flush().ok();
    let section: String = read!();
    let mut config = Ini::new();
    println!("Please wait as we update your file...");

    let _overhaul = config.load(dec::PATH)?;
    let url = config.get(section.as_str().trim(), "url").unwrap();
    let request = reqwest::get(url).await?.text().await?;
    let request = dec::store_request(request);
    let loc = config.get(section.as_str().trim(), "loc").unwrap();
    dec::write_to_file(loc.as_str(), request);
    println!("File Updated!");
  }
  Ok(())
}
