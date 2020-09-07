use warp::Filter;
mod pubg;

#[tokio::main]
async fn main() {
    start_server().await;
}

async fn start_server() {
    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    let guns =
        warp::path!("weaponvsweapon" / String / "vs" / String).map(|gun1: String, gun2: String| {
            let res = pubg::guns::gun_vs_gun(gun1, gun2);
            format!("{:#?}", res)
        });

    let routes = warp::get().and(hi.or(hello_from_warp).or(times).or(guns));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// _________ NOTES:

// map will not work unless you return a value.
// Using for_each will allow you to loop over and use the values without a return
// r.iter().for_each(|item| {
//     let (x, y) = item;
//     println!("{:?} - {:?}", x, y);
// });
