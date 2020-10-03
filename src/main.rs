use serde_json::{json, Deserializer, Map, Result, Value};
use std::error::Error;
use std::iter::FromIterator;
use warp::{http::StatusCode, Filter};

mod inventory;
mod pubg;

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await;
    Ok(())
}

async fn start_server() {
    let guns =
        warp::path!("weaponvsweapon" / String / "vs" / String).map(|gun1: String, gun2: String| {
            let gun = pubg::guns::gun_vs_gun(&gun1, &gun2);
            let gun_json = &json!(&gun);
            println!("{:?}", &gun);
            format!("{:#?}", gun_json)
        });

    // async fn player_stats(player: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    //     if !&player.is_empty() {
    //         Ok(Box::new(pubg::get_player(&player).await.unwrap()))
    //     } else {
    //         Ok(Box::new(StatusCode::BAD_REQUEST))
    //     }
    // }

    // let player_route = warp::path!("player" / String).and_then(player_stats);

    let routes = warp::get().and(guns); //.or(player_route));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

async fn pbg() {
    let player_name = "SeeWats0n";
    // let player_name = "Kirri";
    // let player_name = "DaliRafter";
    // let player_name = "Siminious";
    let account_id = pubg::get_account_id(player_name).await.unwrap();
    let weapons = pubg::weapon_mastery(&account_id).await.unwrap();
    let weapon_summaries = weapons.attributes.weapon_summaries;

    let mut weapon_list = Vec::from_iter(weapon_summaries);
    weapon_list.sort_by(|(_, val_a), (_, val_b)| {
        let weapon_a_stat = &val_a.as_ref().unwrap().stats_total;
        let weapon_b_stat = &val_b.as_ref().unwrap().stats_total;

        weapon_b_stat.kills.cmp(&weapon_a_stat.kills)
    });

    // gat top x weapons of kills
    (0..5).for_each(|index| {
        println!(
            "{:#?} -- {:#?}",
            // TODO: change this to a better module
            inventory::inventory::item_friendly_name(&weapon_list[index].0),
            &weapon_list[index].1.as_ref().unwrap().stats_total.kills
        );
    });
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
