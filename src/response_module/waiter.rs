use serenity::prelude::TypeMapKey;
use serenity::client::Context;
use serenity::model::{channel::Message, id::{GuildId, ChannelId, UserId}};
use std::sync::{RwLock, Arc, Mutex};

/// time out in minutes
pub const WAITER_TIME_OUT : f64 = 5.0; 

pub type Content = Arc<Mutex<dyn std::any::Any + Send + Sync>>;
pub type Callback = fn(Arc<Mutex<Waiter>>, Context, Arc<Message>);


pub struct Waiter {
    pub guild_id : GuildId,
    pub channel_id : ChannelId,
    pub user_id : UserId,
    pub content : Content,
    pub callback : Arc<Callback>
}

unsafe impl Send for Waiter { }

impl Waiter {
    pub fn new(guild_id: GuildId, channel_id : ChannelId, user_id : UserId,
                content : Content, callback : Callback) -> Waiter {
        Waiter {
            guild_id : guild_id,
            channel_id : channel_id,
            user_id : user_id,
            content : content,
            callback : Arc::new(callback)
        }
    }
}

pub struct Waiters {
    pub waiters : Vec<Arc<Mutex<Waiter>>>
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
        let waiter = Arc::new(Mutex::new(waiter));
        self.waiters.push(waiter);
    }

    pub fn remove_index(&mut self, index : usize) {
        self.waiters.remove(index);
    }
}

pub fn add_waiter(context : &mut Context, waiter : Waiter) 
        -> Result<(), Box<String>> {
    let waiters = get_waiters(context);
    let mut waiters = waiters.write();

    let mut waiters = match waiters {
        Err(e) => return Err(Box::new(e.to_string())),
        Ok(o) => o
    };

    waiters.add_waiter(waiter);

    Ok(())
}

pub fn check_waiters(ctx : Context, msg : Arc<Message>) {

    println!("in function check_waiters");

    std::thread::spawn(move || {
        println!("thread spawned!");
        let waiters = get_waiters(&ctx);
        let mut waiters = waiters.write().unwrap();
        println!("started writing waiters");
        let mut last_index : Option<usize> = None;
        {
            println!("going to iterate check_waiters, amuont {}", waiters.waiters.len());
            let iterators = waiters.waiters.iter().enumerate();
            for (i, mutex_waiter) in iterators {
                let waiter = mutex_waiter.lock().unwrap();

                if !should_invoke_waiter(&waiter, &msg) {
                    continue;
                }

                let wt = Arc::clone(&mutex_waiter);
                let message = Arc::clone(&msg);
                let callback = Arc::clone(&waiter.callback);
                drop(waiter);

                std::thread::spawn(move || {
                    (callback)(wt, ctx.clone(), message)
                });

                last_index = Some(i);
                break;
            }
        }
        
        if let Some(i) = last_index {
            println!("found! removing");
            waiters.remove_index(i);
        };

        drop(waiters);
    });
}

fn should_invoke_waiter(waiter : &Waiter, msg : &Message) -> bool {
    println!("waiter values: \n{:?} \n{:?}\nnew values: \n{:?} \n{:?}", 
        waiter.channel_id, waiter.user_id, msg.channel_id, msg.author.id);
    waiter.channel_id.0 == msg.channel_id.0 && waiter.user_id.0 == msg.author.id.0
}

pub fn get_waiters(mut context : &Context) -> Arc<RwLock<Waiters>> {
    let read = context.data.read();
    let waiters = read.get::<WaitersKey>();

    let waiters = match waiters {
        None => { 
            drop(read);
            create_waiters(&mut context);
            get_waiters(&mut context)
        },
        Some(w) => Arc::clone(&w)
    };

    return waiters;
}

fn create_waiters(mut context : &Context)
{
    let mut write = context.data.write();
    let waiters = Arc::new(RwLock::new(Waiters::new()));

    let result = write.insert::<WaitersKey>(waiters);
    drop(write);
}