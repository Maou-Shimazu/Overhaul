use std::error::Error;
use std::fs;
pub mod dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  #[allow(while_true)] // kek
  while true {
    dec::main_menu();
    let _url = String::from_utf8(fs::read(dec::URL)?).unwrap(); // gets url from line in url.ini
    for urls in _url.lines() { // loops through each line in url.ini
      println!("Updating {}...", urls); //outputs urls

      let mut file: &str; // location of file to be wirtten to, unassigned
      let _loc = String::from_utf8(fs::read(dec::LOC)?).unwrap(); // location feched from loc.ini

      for _location in _loc.lines() { // iterate through loc.ini
        file = _location; // assigns location to line in loc.ini
        let request = reqwest::get(urls).await?.text().await?; // requests the url currently in loop
        let request = dec::store_request(request); // stores that request in some function i made idk
        dec::write_to_file(file, request); // writes request data to file with another weird function i made somewhere kek
      }
    }
  }
  Ok(())
}
/*
  Planning on adding a configuration section to create the config dir and all its files, fetch current path and put it in environment variables. But thats for a later date. This project has been fun.
*/