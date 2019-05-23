use crate::Config;
use crate::response_module::waiter;
use serenity::{
    client::{Client, EventHandler, Context}, 
    framework::standard::{
        StandardFramework,
        macros::{group}
    },
    model::channel::Message,
};
use std::sync::{Arc, Mutex};

use crate::commands::subscribe::{subscribe, SUBSCRIBE_COMMAND};

struct Handler;
 
impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        let message = Arc::new(message);
        Handler::check_waiters(ctx, message.clone());
    }
}

impl Handler {
    pub fn check_waiters(ctx: Context, message: Arc<Message>) {
        waiter::check_waiters(ctx, message);
    }
}

pub fn start_bot(config: Config) -> Bot {
    Bot::new(config)
}

pub struct Bot {
    client : Arc<Mutex<Client>>
}

group!({
    name: "general",
    options: {},
    commands: [subscribe]
});

impl Bot {
    pub fn new(config : Config) -> Bot {
        let mut client = Client::new(&config.token, Handler)
            .expect("Could not create client.");

        client.with_framework(
            StandardFramework::new()
            .configure(|c|
                c.prefix("$"))
            .group(&GENERAL_GROUP)
        );

        if let Err(e) = client.start() {
            println!("Could not start bot {:?}", e);
        }

        Bot {
            client : Arc::new(Mutex::new(client))
        }
    }
}