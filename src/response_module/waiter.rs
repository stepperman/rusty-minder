use serenity::model::id::ChannelId;
use std::sync::{Mutex, Arc};

pub struct Waiter {
    channel_id : ChannelId,
    callback : fn()
}

pub fn add_waiter(&mut context : Context, waiter : Waiter) {

}