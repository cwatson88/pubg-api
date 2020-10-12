use serde_json::{json, Deserializer, Error, Map, Value};
use std::{convert::Infallible, iter::FromIterator};
use warp::{http::Method, http::StatusCode, Filter};

mod inventory;
mod pubg;

#[tokio::main]
async fn main() -> Result<(), Error> {
    start_server().await;
    Ok(())
}

async fn start_server() {
    let hi = warp::path("hi").map(|| "Hello, World!");
    let guns =
        warp::path!("weaponvsweapon" / String / "vs" / String).map(|gun1: String, gun2: String| {
            let gun = pubg::guns::gun_vs_gun(&gun1, &gun2);
            let gun_json = &json!(&gun);
            println!("{:?}", &gun);
            format!("{:#?}", gun_json)
        });

    let player_route = warp::path!("player" / String).and_then(player_stats);
    let top_guns_route = warp::path!("topguns" / String).and_then(top_x_guns);
    let lifetime_stats_route = warp::path!("lifetime" ).and_then(lifetime_stats);

    let cors = warp::cors()
        .allow_any_origin()
        // .allow_origin("https://pubg-291421.web.app")
        .allow_methods(&[Method::GET]);

    let routes = warp::get()
        .and(hi.or(guns).or(player_route).or(top_guns_route).or(lifetime_stats_route))
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

async fn player_stats(player: String) -> Result<warp::reply::Json, warp::Rejection> {
    if !&player.is_empty() {
        Ok(warp::reply::json(&pubg::get_player(&player).await.unwrap()))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn lifetime_stats () -> Result<warp::reply::Json, warp::Rejection>{
    
    // if !&player.is_empty() {
        Ok(warp::reply::json(&pubg::player_lifetime_stats("account.c7763c41ba4246d497db2b85ff68a897").await.unwrap()))
    // } else {
    //     Err(warp::reject::not_found())
    // }
}

async fn top_x_guns(player: String) -> Result<warp::reply::Json, warp::Rejection> {
    if !&player.is_empty() {
        // let player_name = "Kirri";
        // let player_name = "DaliRafter";
        // let player_name = "Siminious";
        // let player_name = "SeeWats0n";
        let account_id = pubg::get_account_id(&player).await.unwrap();
        let weapons = pubg::weapon_mastery(&account_id).await.unwrap();
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
                "gun":inventory::inventory::item_friendly_name(&weapon_list[index].0),
                "kills":&weapon_list[index].1.as_ref().unwrap().stats_total.kills
            }));
        });
        Ok(warp::reply::json(&top_weapons))
    } else {
        Err(warp::reject::not_found())
    }
}

// use tokio for async function testing
// enable println by using "cargo test -- --nocapture"
#[cfg(test)]
mod tests {
    use crate::pubg;

    // #[test]
    #[tokio::test]
    async fn test_account() {
        let player_name = "SeeWats0n";
        let account_id = pubg::get_account_id(player_name).await.unwrap();
        assert_eq!(
            account_id,
            "account.c7763c41ba4246d497db2b85ff68a897".to_string()
        );
    }
    #[tokio::test]
    async fn test_weapons() {
        let account_id = "account.c7763c41ba4246d497db2b85ff68a897".to_string();
        let weapons = pubg::weapon_mastery(&account_id).await.unwrap();
        assert_eq!(weapons.account_id, account_id);
    }
}
