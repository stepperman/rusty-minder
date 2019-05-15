mod commands;
mod anilist;

use std::io::BufRead;

fn main() {
    let result = anilist::searching::search(String::from("Attack on Titan"));    

    println!("{:?}", result);
}

fn get_conf() -> Config {
    let token = std::env::var("token");

    let token : String = match token {
        Ok(t) => t,
        _ => request_token()
    }; 

   Config {
        token : token
    }
}

fn request_token() -> String {
    println!("No token has been found in the environment variables. Please fill in the token:");

    let mut token = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut token).expect("Not a correct token!");

    std::env::set_var("token", token.clone());
    token
}

struct Config {
    token : String,

}