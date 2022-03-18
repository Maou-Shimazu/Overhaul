use configparser::ini::Ini;
use std::error::Error;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::stdin;

#[allow(dead_code)]
fn get_request(){}

#[allow(dead_code)]
fn main_menu(){

}

#[allow(dead_code)]
fn update_configuration(file: &str, data: &str){
  let mut write = OpenOptions::new()
        .append(true)
        .open(file)
        .expect("Unable to open file");
   write.write_all(data.as_bytes()).expect("Unable to write data");
}

#[allow(dead_code)]
fn get_all_sections(file: std::collections::HashMap<std::string::String, std::collections::HashMap<std::string::String, std::option::Option<std::string::String>>>){
  println!("\n{:?}\n", file);
}

fn get_specific_section(section_name: String){
  println!("{}", section_name)
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {

  let mut config = Ini::new();
  let file: &str = "config/overhaul.ini";

  let overhaul = config.load(file)?;

  get_all_sections(overhaul);
  update_configuration(file, "");

  let mut section = String::new();
  std::io::stdin().read_line(&mut section)?;
  println!("{}", section);
  
  let newstr: &str = "parse_args";

  let url = config.get(newstr, "url").unwrap();
  let loc = config.get("parse_args", "loc").unwrap();

  get_specific_section(url);

  // assert_eq!(url, "https://github.com/Maou-Shimazu/Parse-Args/blob/main/include/parse_args.h");
  // assert_eq!(loc, r#"C:\Users\User5\Documents\Github Projects\Parse-Args\include\parse_args.h"#);

    Ok(())
}
