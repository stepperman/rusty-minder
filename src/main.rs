#[macro_use] extern crate serde_derive;

mod commands;
mod anilist;
mod bot;
mod nyaa;
mod response_module;

use std::io::BufRead;
use bot::Bot;

fn main() {

    let feed = nyaa::feed::fetch().expect("WHAT THE FUCK DUDE."); 
    println!("{:#?}", feed.feed());
    return;
    let config = get_conf();

    Bot::new(config);
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

#[derive(Clone)]
pub struct Config {
    token : String,
}