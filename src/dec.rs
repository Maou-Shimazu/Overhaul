use text_io::read;

fn add_new(){
    
}

#[allow(dead_code)]
/// The main menu 
pub fn main_menu(){
    println!("Welcome to OverHaul.");
    println!("[1] Add New File.");
    println!("[2] Update File.");
    println!("[3] Update all files.");
    println!("[4] -------------------");
    println!("[5] Exit.");
    print!("What would you like to do?: ");
    let ans:i32 = read!();
    while ans != 5 {
        match ans {
                1 => add_new();
        }
    }
    
    
}