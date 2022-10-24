use dotenv;
use serenity::model::prelude::*;
use serenity::http::*;
use serde_json::Value;
use crate::greenboard::*;

pub fn init_serenity_http()->Http{
    let token = dotenv::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let http = Http::new(&token);
    http
}

pub async fn init_webhook()->Webhook{
    let token = dotenv::var("DISCORD_TOKEN").expect("WEBHOOK_URL must be set");
    let id:u64 = dotenv::var("DISCORD_ID").expect("WEBHOOK_URL must be set").parse().unwrap();
    let http = init_serenity_http();
    let webhook = http.get_webhook_with_token(id, &token).await.expect("Failed to get webhook");
    webhook
}

pub async fn send_embed(embed:Value){
    let webhook = init_webhook().await;
    let http = init_serenity_http();
    let _ = webhook.execute(&http, false, |w| {
        w.username("Job Sniper");
        w.embeds(vec![embed])
    }).await;
}

// #[tokio::test]
// async fn test_embed_via_webhook(){
//     let j = GreenboardJobOffer::default();
//     let embed = create_embed_job(j);
//     send_embed(embed).await;
// }