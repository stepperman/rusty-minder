use crate::anilist::searching::*;
use std::thread;
use serenity::{
    framework::{standard::{macros::command, CommandResult, CommandError, Args}},
    model::channel::Message,
    client::Context
};

#[command]
#[aliases(sub)]
pub fn subscribe(context : &mut Context, message : &Message, mut args : Args) -> CommandResult {

    let search_query = args.single::<String>()?;

    let search = match search(search_query) {
        Ok(o) => o,
        _ => return Err(CommandError::from("Searching failed!"))
    };

    

    Ok(())
}