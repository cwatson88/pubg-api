use serde_json::{json, Error, Result as JSONResult, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{ iter::FromIterator};
extern crate reqwest;
pub mod weapon_structs;

pub mod guns {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io::prelude::*;


    #[derive(Debug, Serialize, Deserialize)]
    pub enum GunVsGun {
        Gun(GunStats),
        ErrMessage(String),
    }
    // explanations for data below https://pubg.gamepedia.com/Data_Key
    // Note: Enums can be used with structs
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GunStats {
        #[serde(rename = "BDMG0")]
        bdgm0: String,
        #[serde(rename = "BDMG1")]
        bdmg1: String,
        #[serde(rename = "BDMG2")]
        bdmg2: String,
        #[serde(rename = "BDMG3")]
        bdmg3: String,
        #[serde(rename = "DMG")]
        dmg: String, // DMG can be type "null"
        #[serde(rename = "HDMG0")]
        hdmg0: String,
        #[serde(rename = "HDMG1")]
        hdmg1: String,
        #[serde(rename = "HDMG2")]
        hdmg2: String,
        #[serde(rename = "HDMG3")]
        hdmg3: String,
        #[serde(rename = "IMAGE")]
        image: String,
        #[serde(rename = "NAME")]
        name: String,
        #[serde(rename = "PWR")]
        pwr: String,
        #[serde(rename = "SPD")]
        spd: String,
    }

    pub fn gun_vs_gun<'a>(gun1: &'a String, gun2: &'a String) -> GunVsGun {
        let gun_list = get_gun_list();
        if gun_list.is_ok() {
            let guns = gun_list.unwrap();
            let get_gun_stats = |gun_list: &Vec<GunStats>, gun_name: String| -> GunStats {
                gun_list
                    .iter()
                    .find(|gun| gun.name == gun_name)
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
        if gun1.bdmg1 > gun2.bdmg1 {
            gun1.clone()
        } else {
            gun2.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct GameModes {
    #[serde(rename = "duo")]
    duo:GameModeStats,
    #[serde(rename = "duo-fpp")]
    duo_fpp:GameModeStats,
    #[serde(rename = "solo")]
    solo:GameModeStats,
    #[serde(rename = "solo-fpp")]
    solo_fpp:GameModeStats,
    #[serde(rename = "squad")]
    squad:GameModeStats,
    #[serde(rename = "squad-fpp")]
    squad_fpp:GameModeStats,
   
}
/// Game Mode stats objects contain a player's aggregated stats for a game mode in the
/// context of a season.
#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct GameModeStats {
    /// Number of enemy players this player damaged that were killed by teammates
    #[serde(rename = "assists")]
    pub assists: Option<i64>,

    /// Number of boost items used
    #[serde(rename = "boosts")]
    pub boosts: Option<i64>,

    /// Number of kills during the most recent day played.
    #[serde(rename = "dailyKills")]
    pub daily_kills: Option<i64>,

    /// Number of wins during the most recent day played.
    #[serde(rename = "dailyWins")]
    pub daily_wins: Option<i64>,

    /// Total damage dealt. Note: Self inflicted damage is subtracted
    #[serde(rename = "damageDealt")]
    pub damage_dealt: Option<f64>,

    #[serde(rename = "days")]
    pub days: Option<i64>,

    /// Number of enemy players knocked
    #[serde(rename = "dBNOs")]
    pub d_bn_os: Option<i64>,

    /// Number of enemy players killed with headshots
    #[serde(rename = "headshotKills")]
    pub headshot_kills: Option<i64>,

    /// Number of healing items used
    #[serde(rename = "heals")]
    pub heals: Option<i64>,

    /// N/A
    #[serde(rename = "killPoints")]
    pub kill_points: Option<f64>,

    /// Number of enemy players killed
    #[serde(rename = "kills")]
    pub kills: Option<i64>,

    #[serde(rename = "longestKill")]
    pub longest_kill: Option<f64>,

    /// Longest time survived in a match
    #[serde(rename = "longestTimeSurvived")]
    pub longest_time_survived: Option<f64>,

    /// Number of matches lost
    #[serde(rename = "losses")]
    pub losses: Option<i64>,

    #[serde(rename = "maxKillStreaks")]
    pub max_kill_streaks: Option<i64>,

    /// Longest time survived in a match
    #[serde(rename = "mostSurvivalTime")]
    pub most_survival_time: Option<f64>,

    /// Number of rank points the player was awarded. This value will be 0 when roundsPlayed < 10
    #[serde(rename = "rankPoints")]
    pub rank_points: Option<f64>,

    /// Rank title in the form title-level
    #[serde(rename = "rankPointsTitle")]
    pub rank_points_title: Option<String>,

    /// Number of times this player revived teammates
    #[serde(rename = "revives")]
    pub revives: Option<i64>,

    /// Total distance traveled in vehicles measured in meters
    #[serde(rename = "rideDistance")]
    pub ride_distance: Option<f64>,

    /// Number of kills while in a vehicle
    #[serde(rename = "roadKills")]
    pub road_kills: Option<i64>,

    /// Highest number of kills in a single match
    #[serde(rename = "roundMostKills")]
    pub round_most_kills: Option<i64>,

    /// Number of matches played
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: Option<i64>,

    /// Number of self-inflicted deaths
    #[serde(rename = "suicides")]
    pub suicides: Option<i64>,

    /// Total distance traveled while swimming measured in meters
    #[serde(rename = "swimDistance")]
    pub swim_distance: Option<f64>,

    /// Number of times this player killed a teammate
    #[serde(rename = "teamKills")]
    pub team_kills: Option<i64>,

    /// Total time survived
    #[serde(rename = "timeSurvived")]
    pub time_survived: Option<f64>,

    /// Number of times this player made it to the top 10 in a match
    #[serde(rename = "top10s")]
    pub top10_s: Option<i64>,

    /// Number of vehicles destroyed
    #[serde(rename = "vehicleDestroys")]
    pub vehicle_destroys: Option<i64>,

    /// Total distance traveled on foot measured in meters
    #[serde(rename = "walkDistance")]
    pub walk_distance: Option<f64>,

    /// Number of weapons picked up
    #[serde(rename = "weaponsAcquired")]
    pub weapons_acquired: Option<i64>,

    /// Number of kills during the most recent week played
    #[serde(rename = "weeklyKills")]
    pub weekly_kills: Option<i64>,

    /// Number of wins during the most recent week played.
    #[serde(rename = "weeklyWins")]
    pub weekly_wins: Option<i64>,

    /// N/A
    #[serde(rename = "winPoints")]
    pub win_points: Option<f64>,

    /// Number of matches won
    #[serde(rename = "wins")]
    pub wins: Option<i64>,
}


async fn api_get(endpoint: &str) -> Result<Value, reqwest::Error> {
    use reqwest::header;
    // use serde_json::{Result, Value};

    const KEY: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI0YmQ2ZTJmMC1jM2M1LTAxMzgtOTQ0ZS0xOTdlNDVlMjM0OWUiLCJpc3MiOiJnYW1lbG9ja2VyIiwiaWF0IjoxNTk3Nzg1MDg0LCJwdWIiOiJibHVlaG9sZSIsInRpdGxlIjoicHViZyIsImFwcCI6ImN3YXRzb24xOTg4LWdtIn0.Mt8A76L-gEWUvCpcrYAo4Wl1dS0sA23oKZjhdEJSqfA";

    const BASE_URL: &str = "https://api.pubg.com";

    let url = format!("{}{}", BASE_URL, endpoint);

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
        .header(header::AUTHORIZATION, KEY) // used to pass the key
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

///  Return all the stats on a player including the account id using the gamer name
/// `/shards/stadia/players?filter[playerNames]=`
pub async fn get_player(player: &str) -> Result<Value, reqwest::Error> {
    // using format to concatenate strings
    let player_search = format!("/shards/stadia/players?filter[playerNames]={}", &player);
    api_get(&player_search).await
}

/// Return the player account - this is needed for most api calls - it is NOT the gamer tag
pub async fn get_account_id(player: &str) -> Result<String, Error> {
    let player = get_player(&player).await.unwrap();
    // as_str and String::From is needed to remove the quotes from a string
    let account_id = String::from(player["data"][0]["id"].as_str().unwrap());
    Ok(account_id)
}

/// get the weapon mastery stats for a player
/// `/shards/stadia/players/{}/weapon_mastery`
pub async fn weapon_mastery(
    account_id: &str
) -> Result<weapon_structs::WeaponMasterySummary, Error> {
    let weapon_mastery_url = format!("/shards/stadia/players/{}/weapon_mastery", &account_id);
    serde_json::from_str(&api_get(&weapon_mastery_url).await.unwrap()["data"].to_string())
}



pub async fn player_lifetime_stats ( account_id: &str) 
-> Result<GameModes ,Error> 
{
    let stat_search = format!("/shards/stadia/players/{}/seasons/lifetime?filter[gamepad]=true", &account_id);
    let data:Result<GameModes,Error> = serde_json::from_str(&api_get(&stat_search).await.unwrap()["data"]["attributes"]["gameModeStats"].to_string());

    let json_data = 
    &api_get(&stat_search)
    .await
    .unwrap()["data"]["attributes"]["gameModeStats"].as_object()
    .unwrap()
    .into_iter()
    .fold(GameModeStats::default(),|acc,curr|{
        let (key, value) = curr;
        let stats:GameModeStats = serde_json::from_value(value.clone()).unwrap();
        
        println!("{:#?}",stats);

        acc.assists.unwrap() += stats.assists.unwrap();
        acc
    });
    
    data
   
}