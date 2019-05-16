use reqwest::{Client, Request};
use serde::{Deserialize, Serialize, self};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Feed {
    channel : Channel
}

#[derive(Deserialize, Serialize, Debug)]
struct Channel {
    item : Vec<Item>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub title : String,
    pub link : String,
    pub guid : String,

    #[serde(rename = "nyaa:infoHash")]
    pub info_hash : String,

    #[serde(rename = "nyaa:size")]
    pub size : String,

    #[serde(rename = "nyaa:categoryId")]
    pub category_id : String
}

impl Feed {
    pub fn feed(&self) -> Vec<Item> {
        self.channel.item
    }
}

static link : String = String::from("https://nyaa.si/?page=rss");

pub fn fetch() -> Result<Feed, Box<dyn Error>> {
    let feed = fetch_feed()?;
    let feed : Feed = serde_xml::from_str(&feed)?;

    feed
}

fn fetch_feed() -> Result<String, Box<dyn Error>> {
    let mut client = reqwest::Client::new();
    let mut response = client.get(link).send()?;

    response.text()?
}