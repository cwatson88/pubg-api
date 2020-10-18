use serde_json::{ Error, Value};
use serde::{Deserialize, Serialize};
extern crate reqwest;


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
    #[serde(rename = "total-stats")]
    total_stats:Option<TotalStats>,
   
}
/// Game Mode stats objects contain a player's aggregated stats for a game mode in the
/// context of a season.
#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct GameModeStats {
    /// Number of enemy players this player damaged that were killed by teammates
    #[serde(rename = "assists")]
    pub assists: i64,

    /// Number of boost items used
    #[serde(rename = "boosts")]
    pub boosts: i64,

    /// Number of kills during the most recent day played.
    #[serde(rename = "dailyKills")]
    pub daily_kills: i64,

    /// Number of wins during the most recent day played.
    #[serde(rename = "dailyWins")]
    pub daily_wins: i64,

    /// Total damage dealt. Note: Self inflicted damage is subtracted
    #[serde(rename = "damageDealt")]
    pub damage_dealt: f64,

    #[serde(rename = "days")]
    pub days: i64,

    /// Number of enemy players knocked - dBNO (down but not out)
    #[serde(rename = "dBNOs")]
    pub d_bn_os: i64,

    /// Number of enemy players killed with headshots
    #[serde(rename = "headshotKills")]
    pub headshot_kills: i64,

    /// Number of healing items used
    #[serde(rename = "heals")]
    pub heals: i64,

    /// N/A
    #[serde(rename = "killPoints")]
    pub kill_points: f64,

    /// Number of enemy players killed
    #[serde(rename = "kills")]
    pub kills: i64,

    #[serde(rename = "longestKill")]
    pub longest_kill: f64,

    /// Longest time survived in a match
    #[serde(rename = "longestTimeSurvived")]
    pub longest_time_survived: f64,

    /// Number of matches lost
    #[serde(rename = "losses")]
    pub losses: i64,

    #[serde(rename = "maxKillStreaks")]
    pub max_kill_streaks: i64,

    /// Longest time survived in a match
    #[serde(rename = "mostSurvivalTime")]
    pub most_survival_time: f64,

    /// Number of rank points the player was awarded. This value will be 0 when roundsPlayed < 10
    #[serde(rename = "rankPoints")]
    pub rank_points: f64,

    /// Rank title in the form title-level
    #[serde(rename = "rankPointsTitle")]
    pub rank_points_title: String,

    /// Number of times this player revived teammates
    #[serde(rename = "revives")]
    pub revives: i64,

    /// Total distance traveled in vehicles measured in meters
    #[serde(rename = "rideDistance")]
    pub ride_distance: f64,

    /// Number of kills while in a vehicle
    #[serde(rename = "roadKills")]
    pub road_kills: i64,

    /// Highest number of kills in a single match
    #[serde(rename = "roundMostKills")]
    pub round_most_kills: i64,

    /// Number of matches played
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i64,

    /// Number of self-inflicted deaths
    #[serde(rename = "suicides")]
    pub suicides: i64,

    /// Total distance traveled while swimming measured in meters
    #[serde(rename = "swimDistance")]
    pub swim_distance: f64,

    /// Number of times this player killed a teammate
    #[serde(rename = "teamKills")]
    pub team_kills: i64,

    /// Total time survived
    #[serde(rename = "timeSurvived")]
    pub time_survived: f64,

    /// Number of times this player made it to the top 10 in a match
    #[serde(rename = "top10s")]
    pub top10_s: i64,

    /// Number of vehicles destroyed
    #[serde(rename = "vehicleDestroys")]
    pub vehicle_destroys: i64,

    /// Total distance traveled on foot measured in meters
    #[serde(rename = "walkDistance")]
    pub walk_distance: f64,

    /// Number of weapons picked up
    #[serde(rename = "weaponsAcquired")]
    pub weapons_acquired: i64,

    /// Number of kills during the most recent week played
    #[serde(rename = "weeklyKills")]
    pub weekly_kills: i64,

    /// Number of wins during the most recent week played.
    #[serde(rename = "weeklyWins")]
    pub weekly_wins: i64,

    /// N/A
    #[serde(rename = "winPoints")]
    pub win_points: f64,

    /// Number of matches won
    #[serde(rename = "wins")]
    pub wins: i64,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
enum StatType {
    KILL,
    FUN,
    MATCH
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct TotalStatItem { 
    pub friendly_name: String,
    pub value:i64,
    pub category:StatType,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
struct TotalStats {
      /// Number of enemy players this player damaged that were killed by teammates
      #[serde(rename = "assists")]
      pub assists: TotalStatItem,
  
      /// Number of kills during the most recent day played.
      #[serde(rename = "dailyKills")]
      pub daily_kills: TotalStatItem,
  
      /// Number of wins during the most recent day played.
      #[serde(rename = "dailyWins")]
      pub daily_wins: TotalStatItem,
  
      /// Total damage dealt. Note: Self inflicted damage is subtracted
      #[serde(rename = "damageDealt")]
      pub damage_dealt: TotalStatItem,
  
      #[serde(rename = "days")]
      pub days: TotalStatItem,
  
      /// Number of enemy players knocked - dBNO (down but not out)
      #[serde(rename = "dBNOs")]
      pub d_bn_os: TotalStatItem,
  
      /// Number of enemy players killed with headshots
      #[serde(rename = "headshotKills")]
      pub headshot_kills: TotalStatItem,
  
      /// Number of healing items used
      #[serde(rename = "heals")]
      pub heals: TotalStatItem,
  
      /// Number of enemy players killed
      #[serde(rename = "kills")]
      pub kills: TotalStatItem,
  
      #[serde(rename = "longestKill")]
      pub longest_kill: TotalStatItem,
  
      /// Longest time survived in a match
      #[serde(rename = "longestTimeSurvived")]
      pub longest_time_survived: TotalStatItem,
  
      /// Number of matches lost
      #[serde(rename = "losses")]
      pub losses: TotalStatItem,
  
      #[serde(rename = "maxKillStreaks")]
      pub max_kill_streaks: TotalStatItem,
  
      /// Number of times this player revived teammates
      #[serde(rename = "revives")]
      pub revives: TotalStatItem,
  
      /// Total distance traveled in vehicles measured in meters
      #[serde(rename = "rideDistance")]
      pub ride_distance: TotalStatItem,
  
      /// Number of kills while in a vehicle
      #[serde(rename = "roadKills")]
      pub road_kills: TotalStatItem,
  
      /// Highest number of kills in a single match
      #[serde(rename = "roundMostKills")]
      pub round_most_kills: TotalStatItem,
  
      /// Number of matches played
      #[serde(rename = "roundsPlayed")]
      pub rounds_played: TotalStatItem,
  
      /// Number of self-inflicted deaths
      #[serde(rename = "suicides")]
      pub suicides: TotalStatItem,
  
      /// Total distance traveled while swimming measured in meters
      #[serde(rename = "swimDistance")]
      pub swim_distance: TotalStatItem,
  
      /// Number of times this player killed a teammate
      #[serde(rename = "teamKills")]
      pub team_kills: TotalStatItem,
  
      /// Total time survived
      #[serde(rename = "timeSurvived")]
      pub time_survived: TotalStatItem,
  
      /// Number of times this player made it to the top 10 in a match
      #[serde(rename = "top10s")]
      pub top10_s: TotalStatItem,
  
      /// Number of vehicles destroyed
      #[serde(rename = "vehicleDestroys")]
      pub vehicle_destroys: TotalStatItem,
  
      /// Total distance traveled on foot measured in meters
      #[serde(rename = "walkDistance")]
      pub walk_distance: TotalStatItem,
  
      /// Number of weapons picked up
      #[serde(rename = "weaponsAcquired")]
      pub weapons_acquired: TotalStatItem,
  
      /// Number of kills during the most recent week played
      #[serde(rename = "weeklyKills")]
      pub weekly_kills: TotalStatItem,
  
      /// Number of wins during the most recent week played.
      #[serde(rename = "weeklyWins")]
      pub weekly_wins: TotalStatItem,
  
  
      /// Number of matches won
      #[serde(rename = "wins")]
      pub wins: TotalStatItem,
}



///  Return all the stats on a player including the account id using the gamer name
/// `/shards/stadia/players?filter[playerNames]=`
pub async fn get_player(player: &str) -> Result<Value, reqwest::Error> {
    // using format to concatenate strings
    let player_search = format!("/shards/stadia/players?filter[playerNames]={}", &player);
    super::pubg_api::get(&player_search).await
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
) -> Result<super::weapon_structs::WeaponMasterySummary, Error> {
    let weapon_mastery_url = format!("/shards/stadia/players/{}/weapon_mastery", &account_id);
    serde_json::from_str(&super::pubg_api::get(&weapon_mastery_url).await.unwrap()["data"].to_string())
}



pub async fn player_lifetime_stats ( account_id: &str) 
-> Result<GameModes ,Error> 
{
    let stat_search = format!("/shards/stadia/players/{}/seasons/lifetime?filter[gamepad]=true", &account_id);
    let mut data:GameModes = serde_json::from_str(&super::pubg_api::get(&stat_search).await.unwrap()["data"]["attributes"]["gameModeStats"].to_string()).unwrap();

    // loop through all GameStats for all game modes and return a total object(similar to using reduce in javascript)
    let total_stats = 
    &super::pubg_api::get(&stat_search)
    .await
    .unwrap()["data"]["attributes"]["gameModeStats"].as_object()
    .unwrap()
    .into_iter()
    .fold(TotalStats{
        assists:TotalStatItem{friendly_name:"Assists".to_string(),value:0, category:StatType::FUN},
daily_kills:TotalStatItem{friendly_name:"Daily Kills".to_string(),value:0, category: StatType::KILL},
daily_wins:TotalStatItem{friendly_name:"Daily Wins".to_string(),value:0, category: StatType::MATCH},
damage_dealt:TotalStatItem{friendly_name:"Damage Dealt".to_string(),value:0, category: StatType::FUN},
days:TotalStatItem{friendly_name:"Days Played".to_string(),value:0, category: StatType::FUN},
d_bn_os:TotalStatItem{friendly_name:"Knock-outs".to_string(),value:0, category: StatType::FUN},
headshot_kills:TotalStatItem{friendly_name:"Headshot Kills".to_string(),value:0, category: StatType::KILL},
heals:TotalStatItem{friendly_name:"Healing Items Used".to_string(),value:0, category: StatType::FUN},
kills:TotalStatItem{friendly_name:"Total Kills".to_string(),value:0, category: StatType::KILL},
longest_kill:TotalStatItem{friendly_name:"Longest Distance Kill".to_string(),value:0, category: StatType::FUN},
longest_time_survived:TotalStatItem{friendly_name:"Longest time survived (match)".to_string(),value:0, category: StatType::MATCH},
losses:TotalStatItem{friendly_name:"Losses".to_string(),value:0, category: StatType::MATCH},
max_kill_streaks:TotalStatItem{friendly_name:"Kill Streak".to_string(),value:0, category: StatType::KILL},
revives:TotalStatItem{friendly_name:"Revives".to_string(),value:0, category: StatType::FUN},
ride_distance:TotalStatItem{friendly_name:"Vehicle Distance (meters)".to_string(),value:0, category: StatType::FUN},
road_kills:TotalStatItem{friendly_name:"Vehicle Kills".to_string(),value:0, category: StatType::FUN},
round_most_kills:TotalStatItem{friendly_name:"Most Kills In a Match".to_string(),value:0, category: StatType::KILL},
rounds_played:TotalStatItem{friendly_name:"Matches Played".to_string(),value:0, category: StatType::MATCH},
suicides:TotalStatItem{friendly_name:"Killed Yourself".to_string(),value:0, category: StatType::KILL},
swim_distance:TotalStatItem{friendly_name:"Swimming Distance (meters)".to_string(),value:0, category: StatType::FUN},
team_kills:TotalStatItem{friendly_name:"Friendly Fire Kills".to_string(),value:0, category: StatType::KILL},
time_survived:TotalStatItem{friendly_name:"Survival Time".to_string(),value:0, category: StatType::FUN},
top10_s:TotalStatItem{friendly_name:"Top 10 places ever".to_string(),value:0, category: StatType::MATCH},
vehicle_destroys:TotalStatItem{friendly_name:"Vehicles destroyed".to_string(),value:0, category: StatType::FUN},
walk_distance:TotalStatItem{friendly_name:"Distance Walked (meters)".to_string(),value:0, category: StatType::FUN},
weapons_acquired:TotalStatItem{friendly_name:"Weapons Picked Up".to_string(),value:0, category: StatType::FUN},
weekly_kills:TotalStatItem{friendly_name:"This weeks kills".to_string(),value:0, category: StatType::KILL},
weekly_wins:TotalStatItem{friendly_name:"This weeks wins".to_string(),value:0, category: StatType::MATCH},
wins:TotalStatItem{friendly_name:"Matches Won".to_string(),value:0, category: StatType::MATCH},
    },|mut acc,curr|{
        let (_, value) = curr;
        let stats:GameModeStats = serde_json::from_value(value.clone()).unwrap();
        
    // find a better way to loop through both sets of Struct fields to add them together
    
    acc.assists.value += stats.assists;
    acc.daily_kills.value+=stats.daily_kills ;
    acc.daily_wins.value+=stats.daily_wins ;
    acc.damage_dealt.value+=stats.damage_dealt.floor() as i64;
    acc.days.value+=stats.days ;
    acc.d_bn_os.value+=stats.d_bn_os ;
    acc.headshot_kills.value+=stats.headshot_kills ;
    acc.heals.value+=stats.heals ;
    acc.kills.value+=stats.kills ;
    acc.longest_kill.value+=stats.longest_kill.floor() as i64;
    acc.longest_time_survived.value+=stats.longest_time_survived.floor() as i64;
    acc.losses.value+=stats.losses ;
    acc.max_kill_streaks.value+=stats.max_kill_streaks ;
    acc.revives.value+=stats.revives ;
    acc.ride_distance.value+=stats.ride_distance.floor() as i64;
    acc.road_kills.value+=stats.road_kills ;
    acc.round_most_kills.value+=stats.round_most_kills ;
    acc.rounds_played.value+=stats.rounds_played ;
    acc.suicides.value+=stats.suicides ;
    acc.swim_distance.value+=stats.swim_distance.floor() as i64;
    acc.team_kills.value+=stats.team_kills ;
    acc.time_survived.value+=stats.time_survived.floor() as i64;
    acc.top10_s.value+=stats.top10_s ;
    acc.vehicle_destroys.value+=stats.vehicle_destroys ;
    acc.walk_distance.value+=stats.walk_distance.floor() as i64;
    acc.weapons_acquired.value+=stats.weapons_acquired ;
    acc.weekly_kills.value+=stats.weekly_kills ;
    acc.weekly_wins.value+=stats.weekly_wins ;
    acc.wins.value+=stats.wins;

    // return the accumulated values - this will be the overall total
    acc
    });
    
    data.total_stats = Some(total_stats.clone());
    Ok(data)
   
}