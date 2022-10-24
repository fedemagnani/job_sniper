use serde_json::Value;
use serenity::model::prelude::*;
#[derive(Debug)]
pub struct GreenboardJobOffer{
    pub employer: String,
    pub absolute_url: String,
    pub location: String,
    pub updated_at: String,
    pub title: String,
}
impl GreenboardJobOffer{
    pub fn default()->GreenboardJobOffer{
        GreenboardJobOffer{
            employer: String::from(""),
            absolute_url: String::from(""),
            location: String::from(""),
            updated_at: String::from(""),
            title: String::from(""),
        }
    }
    pub fn from_json(response:&Value)->GreenboardJobOffer{
        let employer = response["employer"].as_str().expect("No employer provided").to_string();
        let absolute_url = response["absolute_url"].as_str().expect("No url provided").to_string();
        let location = response["location"]["name"].as_str().expect("No location provided").to_string();
        let updated_at = response["updated_at"].as_str().expect("No 'updated at' provided").to_string();
        let title = response["title"].as_str().expect("No title provided").to_string();
        GreenboardJobOffer{
            employer,
            absolute_url,
            location,
            updated_at,
            title,
        }
    }

    pub fn create_embed(&self)->Value{
    let title = format!("{} - {}",&self.employer,&self.title);
    let embed = Embed::fake(|e| {
        e.title(title);
        e.field("Location", &self.location, true);
        e.field("Updated at", &self.updated_at, true);
        e.url(&self.absolute_url);
        e
    });
    embed
}
}