use std::error::Error;
use std::fs;
pub mod dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  //dec::main_menu();
  let contents = String::from_utf8(fs::read(dec::LOG)?).unwrap();
  for line in contents.lines() {
    println!("{}", line);
    let request = reqwest::get("https://swapi.dev/api/people").await?.text().await?;
  }
  Ok(())
}
