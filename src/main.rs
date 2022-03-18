use configparser::ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let overhaul = config.load("config/overhaul.ini")?;
  println!("{:?}", map);
  // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

  // If you want to access the value, then you can simply do:
  let url = config.get("parse_args", "url").unwrap();
  let loc = config.get("parse_args", "loc").unwrap();
  // Notice how get() can access indexes case-insensitively.

  assert_eq!(url, "https://github.com/Maou-Shimazu/Parse-Args/blob/main/include/parse_args.h");
  assert_eq!(loc, r#"C:\Users\User5\Documents\Github Projects\Parse-Args\include\parse_args.h"#); // value accessible!

  // What if you want remove KFC's secret recipe? Just use set():
  config.set("topsecret", "kfc", None);

  assert_eq!(config.get("TOPSECRET", "KFC"), None); // as expected!

  // What if you want to get an unsigned integer?
  let my_number = config.getuint("values", "Uint")?.unwrap();
  assert_eq!(my_number, 31415); // and we got it!
  // The Ini struct provides more getters for primitive datatypes.

  // You can also access it like a normal hashmap:
  let innermap = map["topsecret"].clone();
  // Remember that all indexes are stored in lowercase!

  // You can easily write the currently stored configuration to a file like:
  config.write("output.ini");

  // If you want to simply mutate the stored hashmap, you can use get_mut_map()
  let map = config.get_mut_map();
  // You can then use normal HashMap functions on this map at your convenience.
  // Remember that functions which rely on standard formatting might stop working
  // if it's mutated differently.

  // If you want a case-sensitive map, just do:
  let mut config = Ini::new_cs();
  // This automatically changes the behaviour of every function and parses the file as case-sensitive.

    Ok(())
}
