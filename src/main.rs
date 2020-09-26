use serde_json::{json, Result, Value};
use std::error::Error;
use warp::{http::StatusCode, Filter};

mod pubg;

#[tokio::main]
async fn main() -> Result<()> {
    // start_server().await;
    let player_name = "SeeWats0n";
    let player = pubg::get_player(&player_name).await.unwrap();
    let weapons = pubg::weapon_mastery(&player_name).await.unwrap();
    println!("{:?}", weapons);
    Ok(())
}

// async fn start_server() {
//     // GET /hi
//     let hi = warp::path("hi").map(|| "Hello, World!");

//     // How about multiple segments? First, we could use the `path!` macro:
//     //
//     // GET /hello/from/warp
//     let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

//     let times =
//         warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

//     let guns =
//         warp::path!("weaponvsweapon" / String / "vs" / String).map(|gun1: String, gun2: String| {
//             let gun = pubg::guns::gun_vs_gun(&gun1, &gun2);
//             let gun_json = &json!(&gun);
//             println!("{:?}", &gun);
//             format!("{:#?}", gun_json)
//         });

//     async fn player_stats(player: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
//         if !&player.is_empty() {
//             Ok(Box::new(pubg::get_player(&player).await.unwrap()))
//         } else {
//             Ok(Box::new(StatusCode::BAD_REQUEST))
//         }
//     }

//     let player_route = warp::path!("player" / String).and_then(player_stats);

//     let routes = warp::get().and(hi.or(hello_from_warp).or(times).or(guns).or(player_route));

//     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
// }

// _________ NOTES:

// map will not work unless you return a value.
// Using for_each will allow you to loop over and use the values without a return
// r.iter().for_each(|item| {
//     let (x, y) = item;
//     println!("{:?} - {:?}", x, y);
// });
