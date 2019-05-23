use serenity::prelude::TypeMapKey;
use serenity::client::Context;
use serenity::model::id::{GuildId, ChannelId, UserId};
use std::sync::{RwLock, Arc, Mutex};

/// time out in minutes
pub const WAITER_TIME_OUT : f64 = 5.0; 

pub type Content = Arc<Mutex<dyn std::any::Any + Send>>;

pub struct Waiter {
    pub guild_id : GuildId,
    pub channel_id : ChannelId,
    pub user_id : UserId,
    pub content : Content,
    pub callback : fn(Waiter, Content)
}

impl Waiter {
    pub fn new(guild_id: GuildId, channel_id : ChannelId, user_id : UserId,
                content : Content, callback : fn(Waiter, Content)) -> Waiter {
        Waiter {
            guild_id : guild_id,
            channel_id : channel_id,
            user_id : user_id,
            content : content,
            callback : callback
        }
    }
}

pub struct Waiters {
    waiters : Vec<Waiter>
}

pub struct WaitersKey;

impl TypeMapKey for WaitersKey {
    type Value = Arc<RwLock<Waiters>>;
}

impl Waiters {
    fn new() -> Waiters {
        Waiters {
            waiters : Vec::new()
        }
    }

    pub fn add_waiter(&mut self, waiter:Waiter) {
        self.waiters.push(waiter);
    }
}

pub fn add_waiter(mut context : &Context, waiter : Waiter) 
        -> Result<(), Box<String>> {
    let waiters = get_waiters(&mut context);
    let mut waiters = waiters.write();

    let mut waiters = match waiters {
        Err(e) => return Err(Box::new(e.to_string())),
        Ok(o) => o
    };

    waiters.add_waiter(waiter);

    Ok(())
}

pub fn  get_waiters(mut context : &Context) -> Arc<RwLock<Waiters>> {
    let read = context.data.read();
    let waiters = read.get::<WaitersKey>();

    match waiters {
        None => { 
            create_waiters(&mut context);
            get_waiters(&mut context)
        },
        Some(w) => Arc::clone(&w)
    }
}

fn create_waiters(mut context : &Context)
{
    let mut write = context.data.write();
    let waiters = Arc::new(RwLock::new(Waiters::new()));

    write.insert::<WaitersKey>(waiters).expect("Could not create a waiter");
}