use std::error::Error;
use std::fs;
pub mod dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  while 1==1 {
    dec::main_menu();
    let _loc = String::from_utf8(fs::read(dec::LOC)?).unwrap(); 
    let _url = String::from_utf8(fs::read(dec::URL)?).unwrap();
    for (urls, loc) in _url.lines().zip(_loc.lines()) { 
      println!("Updating {}...", urls);
        let request = reqwest::get(urls).await?.text().await?; 
        let request = dec::store_request(request); 
        dec::write_to_file(loc, request);
    }
  }
  Ok(())
}