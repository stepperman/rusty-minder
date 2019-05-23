use crate::anilist::searching::*;
use crate::response_module::waiter::*;
use std::sync::{Arc, Mutex};
use serenity::{
    framework::{standard::{macros::command, CommandResult, CommandError, Args}},
    model::channel::Message,
    client::Context
};

struct SubscriptionWaiter {
    results : Vec<AnilistResult>,
    temporary_messages : Vec<Arc<Message>>
}

#[command]
#[aliases(sub)]
pub fn subscribe(context : &mut Context, message : &Message, mut args : Args) -> CommandResult {
    use std::sync::{Arc, Mutex};

    // error if message not in guild
    let guild_id = match message.guild_id {
        None => return Err(CommandError::from("Subscribing can only be done in guilds!")),
        Some(s) => s
    };

    let search_query = args.single::<String>()?;

    let mut search = match search(search_query) {
        Ok(o) => o,
        _ => return Err(CommandError::from("Searching failed!"))
    };

    if search.len() > 10 {
        search.split_off(9);
    }

    let mut content : String = String::from("```\n");

    for (i, result) in search.iter().enumerate() {
        content = format!("{}{} : {}\n", content, i + 1, result.title.get_title()); 
    }

    content = format!("{}```", content);

    let new_message = message.channel_id.say(&context, content)?;
    let new_message = Arc::new(new_message);

    let data = Arc::new(Mutex::new(SubscriptionWaiter {
        results : search,
        temporary_messages : vec![new_message.clone()]
    }));

    let waiter = Waiter::new(guild_id, message.channel_id, message.author.id, data, subscribe_callback);

    match add_waiter(context, waiter) {
        Err(s) => return Err(CommandError::from(s)),
        _ => ()
    };

    Ok(())
}

pub fn subscribe_callback(waiter : Arc<Mutex<Waiter>>, context : Context, message : std::sync::Arc<Message>) {
    println!("entered callback");
    let waiter = waiter.lock().unwrap();
    println!("entered callback");
    let content = waiter.content.lock().unwrap();
    println!("entered callback");
    let content = match content.downcast_ref::<SubscriptionWaiter>() {
        Some(x) => x,
        _ => return
    };

    let value = message.content.parse::<i32>();
    let value = match value {
        Ok(e) => e,
        _ => {
            message.channel_id.say(&context, "Could not parse message as number.");
            return;
        }
    };

    if value < 1 {
        message.channel_id.say(&context, "Value cannot be below 1!");
        return;
    }
    else if value > content.results.len() as i32 {
        message.channel_id.say(&context, format!("Value cannot be higher than {}!", content.results.len()));
        return;
    }

    let msg_content = format!("You selected {:#?}", content.results[value as usize]);
    message.reply(&context, &msg_content[..]);
}