use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use reqwest;

pub fn write_file_hashmap(file_name: &str, map: &HashMap<String, String>) {
    let mut file = File::create(file_name).unwrap();
    for (key, value) in map {
        file.write_all(format!("{}={}\n", key, value).as_bytes()).unwrap();
    }
}

pub fn load_file_hashmap(file_name: &str) -> HashMap<String, String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut map = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split('=');
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        map.insert(key, value);
    }
    map
}

pub fn init_reqwest_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"),
    );
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn go_to(url: &str) -> Result<String, reqwest::Error> {
    let client = init_reqwest_client();
    let res = client.get(url).send().await.expect("can't go to url");
    let text = res.text().await.expect("can't get text");
    Ok(text)
}

pub async fn get_json_resp(url:&str)->Result<serde_json::Value,reqwest::Error>{
    let client = init_reqwest_client();
    let res = client.get(url).send().await.expect("can't go to url");
    let json = res.json::<serde_json::Value>().await.expect("can't get json");
    Ok(json)
}

pub fn wait_for(seconds: u64) {
    std::thread::sleep(std::time::Duration::from_secs(seconds));
}