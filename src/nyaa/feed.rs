use reqwest::{Client, Request};
use serde::de::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Feed {
    channel : Channel
}

#[derive(Deserialize, Debug)]
struct Channel {
    item : Vec<Item>
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub title : String,
    pub link : String,
    pub guid : String,

    #[serde(rename = "infoHash", default)]
    pub info_hash : String,

    #[serde(rename = "size", default)]
    pub size : String,

    #[serde(rename = "categoryId", default)]
    pub category_id : String
}

impl Feed {
    pub fn feed(&self) -> &Vec<Item> {
        &self.channel.item
    }
}

static link : &'static str = "https://nyaa.si/?page=rss";

pub fn fetch() -> Result<Feed, Box<dyn Error>> {
    let feed = fetch_feed()?;
    let feed : Feed = serde_xml_rs::from_str(&feed)?;

    Ok(feed)
}

fn fetch_feed() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(&link[..]).send()?;

    let text = response.text()?;

    Ok(text)
}