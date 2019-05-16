use chrono::{DateTime, Utc};
use reqwest::{Client, Request, Response};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnilistResult {
    pub title : AnilistTitle,

    #[serde(rename = "startDate")]
    pub first_air : Date,

    #[serde(rename = "nextAiringEpisode")]
    pub next_air : Option<NextEpisode>,
    pub genres : Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnilistTitle {
    #[serde (rename = "english")]
    pub english_name : Option<String>,

    #[serde (rename = "romaji")]
    pub romaji_name : Option<String>,

    #[serde (rename = "native")]
    pub native_name : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextEpisode {
    pub episode : u16,
    pub date : Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    pub year : u16,
    pub month : u16,
    pub day : u16
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "data")]
struct _Data {
    #[serde(rename="Page")]
    page : _Page
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Page")]
struct _Page {
    media : Vec<AnilistResult>
}

#[derive(Serialize, Deserialize, Debug)]
struct All {
    data : _Data
}

pub fn search(search_query:String) -> Result<Vec<AnilistResult>, ()> {
    let result = search_raw(search_query)?;

    println!("{:?}", result);

    let parsed : All = serde_json::from_str(&result).expect("FUCK!");

    Ok(parsed.data.page.media)
}

fn search_raw(search_query:String) -> Result<String, ()> {

    let query = r##"query ($search:String, $perPage:Int, $page:Int) {
    Page(page:$page, perPage:$perPage) {
        pageInfo {
            perPage 
            total
        }
    media(search:$search, type: ANIME) {
        id
        title {
            romaji
            english
            native
        }
        nextAiringEpisode {
            airingAt
            episode
        }
        genres
        startDate {
            year
            month
            day
        }
    }
}}"##.replace("\"", "\\\"");

    let variables = format!(r##"
    {{
    "search" : "{}",
    "perPage" : "{}",
    "page" : "{}" }}
    "##, search_query, 10, 1)
        .replace("\"", "\\\"");

    let content = format!(r##"{{ "query" : "{}", "variables" : "{}" }}"##, query.replace('\n', ""), variables.replace('\n', ""));

    let http_client = Client::new();
    let request = http_client.post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .body(content)
        .send();

    let response = match request {
        Ok(mut v) => v.text(),
        _ => return Err(())
    };

    match response {
        Ok(text) => Ok(text),
        _ => Err(())
    }
}