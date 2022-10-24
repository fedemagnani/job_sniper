
pub mod constants;
pub mod discord;
pub mod utils;
pub mod greenboard;
use discord::{send_embed};
use utils::*;
use greenboard::*;
use serde_json::Value;

static FILE_NAME:&str ="Jobs"; 
static REST_FOR:u64 = 5;

pub fn add_board_token(employer_id: &str, board_token: &str)->Result<(),&'static str> {
    let mut map  = load_file_hashmap(format!("{FILE_NAME}.txt").as_str());
    map.insert(employer_id.to_string(), board_token.to_string());
    let res = write_file_hashmap(format!("{}.txt",FILE_NAME).as_str(), &map);
    Ok(res)
}

pub fn remove_board_token(employer_id: &str)->Result<(),&'static str> {
    let mut map  = load_file_hashmap(format!("{FILE_NAME}.txt").as_str());
    map.remove(employer_id);
    let res = write_file_hashmap(format!("{}.txt",FILE_NAME).as_str(), &map);
    Ok(res)
}

#[tokio::test]
pub async fn get_job_offers()->Result<(),&'static str> {
    let employer:&str = "Flashbots";
    let map  = load_file_hashmap(format!("{FILE_NAME}.txt").as_str());
    let board_token = map.get(employer).expect("Employer not found");
    let url = format!("{}{}/jobs",constants::GREENHOUSE_BASE_API_URL,board_token);
    let res = get_json_resp(url.as_str()).await.expect("Failed to parse response");
    // println!("{:#?}",res);
    for j in res["jobs"].as_array().unwrap(){
        // We shadow the immutable reference with a mutable one
        let mut j = j.clone();
        j["employer"] = Value::String(employer.to_string());
        let joboff = GreenboardJobOffer::from_json(&j);
        // println!("{:?}",joboff);
        let embed = joboff.create_embed(); 
        send_embed(embed).await;
        wait_for(REST_FOR);
    };
    Ok(())
}