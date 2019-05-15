use crate::anilist::searching::*;
use std::thread;
use serenity::{
    framework::{standard::{macros::command, CommandResult, Args}},
    model::channel::Message,
    client::Context
};

#[command]
#[aliases(sub)]
pub fn subscribe(context : &mut Context, message : &Message, mut args : Args) -> CommandResult {
    // start a new thread that searches the anime
    let search_query = args.single::<String>()?;

    search(search_query);    

    Ok(())
}