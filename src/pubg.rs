use percent_encoding::percent_decode_str;
use serde_json::{json, Result as JSONResult, Value};
use std::collections::HashMap;
use std::error::Error;
extern crate reqwest;

pub mod guns {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io::prelude::*;

    // explanations for data below https://pubg.gamepedia.com/Data_Key
    // Note: Enums can be used with structs
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GunStats {
        BDMG0: String,
        BDMG1: String,
        BDMG2: String,
        BDMG3: String,
        DMG: String, // DMG can be type "null"
        HDMG0: String,
        HDMG1: String,
        HDMG2: String,
        HDMG3: String,
        Image: String,
        Name: String,
        PWR: String,
        SPD: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum GunVsGun {
        Gun(GunStats),
        ErrMessage(String),
    }

    pub fn gun_vs_gun<'a>(gun1: &'a String, gun2: &'a String) -> GunVsGun {
        let gun_list = get_gun_list();
        if gun_list.is_ok() {
            let guns = gun_list.unwrap();
            let get_gun_stats = |gun_list: &Vec<GunStats>, gun_name: String| -> GunStats {
                gun_list
                    .iter()
                    .find(|gun| gun.Name == gun_name)
                    .unwrap()
                    .clone()
            };

            let gun1_stats = get_gun_stats(&guns, gun1.to_string());
            let gun2_stats = get_gun_stats(&guns, gun2.to_string());
            let winner = compare_guns(&gun1_stats, &gun2_stats);
            GunVsGun::Gun(winner)
        } else {
            println!("ERR");
            GunVsGun::ErrMessage("Error".to_string())
        }
    }
    // read the gun list from the file and return as json array
    fn get_gun_list() -> std::io::Result<Vec<GunStats>> {
        // grab the guns json file
        let mut file = File::open("./src/pubg-guns.json")?;
        // read the contents as a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // convert to serde_json format
        let file_contents_to_json: Value = serde_json::from_str(&mut contents).unwrap();
        // convert to an array with the gun objects
        let res: Vec<GunStats> = serde_json::from_value(file_contents_to_json).unwrap();

        Ok(res)
    }

    fn compare_guns(gun1: &GunStats, gun2: &GunStats) -> GunStats {
        if gun1.BDMG1 > gun2.BDMG1 {
            gun1.clone()
        } else {
            gun2.clone()
        }
    }
}

async fn api_get(endpoint: &str) -> Result<Value, reqwest::Error> {
    use reqwest::header;
    use serde_json::{Result, Value};

    const key: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI0YmQ2ZTJmMC1jM2M1LTAxMzgtOTQ0ZS0xOTdlNDVlMjM0OWUiLCJpc3MiOiJnYW1lbG9ja2VyIiwiaWF0IjoxNTk3Nzg1MDg0LCJwdWIiOiJibHVlaG9sZSIsInRpdGxlIjoicHViZyIsImFwcCI6ImN3YXRzb24xOTg4LWdtIn0.Mt8A76L-gEWUvCpcrYAo4Wl1dS0sA23oKZjhdEJSqfA";

    const base_url: &str = "https://api.pubg.com";

    let url = format!("{}{}", base_url, endpoint);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("bearer"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.api+json"),
    );

    // get a client builder
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client
        .get(&url)
        .header(header::AUTHORIZATION, key) // used to pass the key
        .send()
        .await?
        // .text()
        .json()
        .await?;

    // println!("{:#?}", res);
    Ok(res)
}

pub async fn get_player(player: &str) -> Result<Value, reqwest::Error> {
    let player_endpoint = "/shards/stadia/players?filter[playerNames]=";
    let player_search = format!("{}{}", &player_endpoint, &player);
    api_get(&player_search).await
}

pub async fn get_account_id(player: &str) -> String {
    let player = get_player(&player).await.unwrap();
    // as_str and String::From is needed to remove the quotes from a string
    let account_id = String::from(player["data"][0]["id"].as_str().unwrap());
    account_id
}

pub async fn weapon_mastery(player: &str) -> Result<Value, reqwest::Error> {
    let account_id = get_account_id(&player).await;
    let weapon_mastery_url = format!("/shards/stadia/players/{}/weapon_mastery", &account_id);
    api_get(&weapon_mastery_url).await
}

// use tokio for async function testing
#[tokio::test]
// enable println by using "cargo test -- --nocapture"
async fn test_get_account_id() {
    let account_id = get_account_id("SeeWats0n").await;
    println!("{}", account_id);
    assert_eq!(account_id, "account.c7763c41ba4246d497db2b85ff68a897");
}
