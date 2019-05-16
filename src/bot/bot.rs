use crate::Config;
use serenity::{client::{Client, EventHandler}, framework::StandardFramework};
use std::sync::{Arc, Mutex};

struct Handler;

impl EventHandler for Handler {
}

pub fn start_bot(config: Config) -> Bot {
    Bot::new(config)
}

pub struct Bot {
    client : Arc<Mutex<Client>>
}

impl Bot {
    pub fn new(config : Config) -> Bot {
        let client = Client::new(&config.token, Handler)
            .expect("Could not create client.");

        

        Bot {
            client : Arc::new(Mutex::new(client))
        }
    }
}