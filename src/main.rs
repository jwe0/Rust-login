use serde_json::Value;
use std::fs;
use std::io::{self, Write};

fn main() {

    print!("[USER] > ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    match io::stdin().read_line(&mut username) {
      Ok(_) => {},
      Err(error) => {
        println!("FUCKING SHIT: {}", error);
        return;
      }
    }


  
    print!("[PASS] > ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    match io::stdin().read_line(&mut password) {
      Ok(_) => {},
      Err(error) => {
        println!("FUCKING SHIT: {}", error);
      }
    }



    let contents = fs::read_to_string("test.json").expect("FUCK");

    let v: Value = serde_json::from_str(&contents).expect("FUCK SHIT");


    let username = username.trim();
    let password = password.trim();


    if &v[username]["password"] == password {
      println!("[SUCCESS] : Succesfully authenticated as {}", username);
    }

}