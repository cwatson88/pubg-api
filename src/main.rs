use serde_json::{json,  Error};
use warp::{http::Method, Filter};
mod pubg;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    let player_route = warp::path!("player" / String).and_then(pubg::get::player_stats);
    let top_guns_route = warp::path!("topguns" / String).and_then(pubg::get::top_x_guns);
    let lifetime_stats_route = warp::path!("lifetime" / String).and_then(pubg::get::lifetime_stats);

    let cors = warp::cors()
        .allow_any_origin()
        // .allow_origin("https://pubg-291421.web.app")
        .allow_methods(&[Method::GET]);

    let routes = warp::get()
        .and(guns.or(player_route).or(top_guns_route).or(lifetime_stats_route))
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}


// use tokio for async function testing
// enable println by using "cargo test -- --nocapture"
// #[cfg(test)]
// mod tests {
//     use crate::pubg;

//     // #[test]
//     #[tokio::test]
//     async fn test_account() {
//         let player_name = "SeeWats0n";
//         let account_id = pubg::get_account_id(player_name).await.unwrap();
//         assert_eq!(
//             account_id,
//             "account.c7763c41ba4246d497db2b85ff68a897".to_string()
//         );
//     }
//     #[tokio::test]
//     async fn test_weapons() {
//         let account_id = "account.c7763c41ba4246d497db2b85ff68a897".to_string();
//         let weapons = pubg::weapon_mastery(&account_id).await.unwrap();
//         assert_eq!(weapons.account_id, account_id);
//     }
// }
