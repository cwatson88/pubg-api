// use super::player::*;
use crate::pubg::player::*;
use serde_json::{Value,json};
use std::iter::FromIterator;


pub async fn player_stats(player: String) -> Result<warp::reply::Json, warp::Rejection> {
    if !&player.is_empty() {
        Ok(warp::reply::json(&get_player(&player).await.unwrap()))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn lifetime_stats (player: String) -> Result<warp::reply::Json, warp::Rejection>{
    
    if !&player.is_empty() {
        let account_id = get_account_id(&player).await.unwrap();
        let res =&player_lifetime_stats(&account_id).await.unwrap(); 
        Ok(warp::reply::json(&res))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn top_x_guns(player: String) -> Result<warp::reply::Json, warp::Rejection> {
    if !&player.is_empty() {
        let account_id = get_account_id(&player).await.unwrap();
        let weapons = weapon_mastery(&account_id).await.unwrap();
        let weapon_summaries = weapons.attributes.weapon_summaries;

        let mut weapon_list = Vec::from_iter(weapon_summaries);
        weapon_list.sort_by(|(_, val_a), (_, val_b)| {
            let weapon_a_stat = &val_a.as_ref().unwrap().stats_total;
            let weapon_b_stat = &val_b.as_ref().unwrap().stats_total;

            weapon_b_stat.kills.cmp(&weapon_a_stat.kills)
        });

        let mut top_weapons: Vec<Value> = vec![];
        // gat top x weapons of kills
        (0..5).for_each(|index| {
            top_weapons.push(json!({
                // TODO: change this to a better module
                "gun":super::inventory::item_friendly_name(&weapon_list[index].0),
                "kills":&weapon_list[index].1.as_ref().unwrap().stats_total.kills
            }));
        });
        Ok(warp::reply::json(&top_weapons))
    } else {
        Err(warp::reject::not_found())
    }
}
