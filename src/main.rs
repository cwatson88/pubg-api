use pubg::weapon_structs::StatsTotal;
use serde_json::{json, Deserializer, Map, Result, Value};
use std::error::Error;
use std::iter::FromIterator;
use warp::{http::StatusCode, Filter};

mod pubg;

#[tokio::main]
async fn main() -> Result<()> {
    // start_server().await;
    // let player_name = "SeeWats0n";
    let player_name = "Kirri";
    // let player_name = "DaliRafter";
    // let player_name = "Siminious";
    let account_id = pubg::get_account_id(player_name).await.unwrap();
    let weapons = pubg::weapon_mastery(&account_id).await.unwrap();
    let weap = weapons.attributes.weapon_summaries;


    let mut a = Vec::from_iter(weap);
    a.sort_by(|(key_a, val_a), (key_b,val_b)| {
        let weapon_a = val_a.as_ref().unwrap();
        // let StatsTotal{kills:kills_a, ..} = &weapon_a.stats_total.as_ref().unwrap();
        let kills_a = &weapon_a.stats_total.as_ref().unwrap().kills;

        let weapon_b = &val_b.as_ref().unwrap();
        let kills_b = &weapon_b.stats_total.as_ref().unwrap().kills;
        
        kills_b.cmp(&kills_a)
    });

    // gat top x weapons of kills 
    (0..5).for_each(|index|println!("{:#?} -- {:#?}",&a[index].0,&a[index].1.as_ref().unwrap().stats_total.as_ref().unwrap()));


    Ok(())
}


// {(
//     ("Item_Ammo_12Guage_C": "12 Gauge Ammo"),
//     ("Item_Ammo_300Magnum_C": "300 Magnum Ammo"),
//     ("Item_Ammo_45ACP_C": ".45 ACP Ammo"),
//     ("Item_Ammo_556mm_C": "5.56mm Ammo"),
//     ("Item_Ammo_762mm_C": "7.62mm Ammo"),
//     ("Item_Ammo_9mm_C": "9mm Ammo"),
//     ("Item_Ammo_Bolt_C": "Crossbow Bolt"),
//     ("Item_Ammo_Flare_C": "Flare Gun Ammo"),
//     ("Item_Armor_C_01_Lv3_C": "Military Vest (Level 3)"),
//     ("Item_Armor_D_01_Lv2_C": "Police Vest (Level 2)"),
//     ("Item_Armor_E_01_Lv1_C": "Police Vest (Level 1)"),
//     ("Item_Attach_Weapon_Lower_AngledForeGrip_C": "Angled Foregrip"),
//     ("Item_Attach_Weapon_Lower_Foregrip_C": "Vertical Foregrip"),
//     ("Item_Attach_Weapon_Lower_HalfGrip_C": "Half Grip"),
//     ("Item_Attach_Weapon_Lower_LaserPointer_C": "Laser Sight"),
//     ("Item_Attach_Weapon_Lower_LightweightForeGrip_C": "Light Grip"),
//     ("Item_Attach_Weapon_Lower_QuickDraw_Large_Crossbow_C": "QuickDraw Crossbow Quiver"),
//     ("Item_Attach_Weapon_Lower_ThumbGrip_C": "Thumb Grip"),
//     ("Item_Attach_Weapon_Magazine_Extended_Large_C": "Extended Mag (AR, DMR, M249, S12K)"),
//     ("Item_Attach_Weapon_Magazine_Extended_Medium_C": "Extended Mag (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Magazine_Extended_Small_C": "Extended Mag (Handgun)"),
//     ("Item_Attach_Weapon_Magazine_Extended_SniperRifle_C": "Extended Mag (DMR, SR)"),
//     ("Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Large_C": "Extended QuickDraw Mag (AR, DMR, M249, S12K)"),
//     ("Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Medium_C": "Extended QuickDraw Mag (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Magazine_ExtendedQuickDraw_Small_C": "Extended QuickDraw Mag (Handgun)"),
//     ("Item_Attach_Weapon_Magazine_ExtendedQuickDraw_SniperRifle_C": "Extended QuickDraw Mag (DMR, SR)"),
//     ("Item_Attach_Weapon_Magazine_QuickDraw_Large_C": "QuickDraw Mag (AR, DMR, M249, S12K)"),
//     ("Item_Attach_Weapon_Magazine_QuickDraw_Medium_C": "Quickdraw Mag (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Magazine_QuickDraw_Small_C": "Quickdraw Mag (Handgun)"),
//     ("Item_Attach_Weapon_Magazine_QuickDraw_SniperRifle_C": "Quickdraw Mag (DMR, SR)"),
//     ("Item_Attach_Weapon_Muzzle_Choke_C": "Choke"),
//     ("Item_Attach_Weapon_Muzzle_Compensator_Large_C": "Compensator (AR, DMR, S12K)"),
//     ("Item_Attach_Weapon_Muzzle_Compensator_Medium_C": "Compensator (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Muzzle_Compensator_SniperRifle_C": "Compensator (DMR, SR)"),
//     ("Item_Attach_Weapon_Muzzle_Duckbill_C": "Duckbill"),
//     ("Item_Attach_Weapon_Muzzle_FlashHider_Large_C": "Flash Hider (AR, DMR, S12K)"),
//     ("Item_Attach_Weapon_Muzzle_FlashHider_Medium_C": "Flash Hider (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Muzzle_FlashHider_SniperRifle_C": "Flash Hider (DMR, SR)"),
//     ("Item_Attach_Weapon_Muzzle_Suppressor_Large_C": "Supressor (AR, DMR, S12K)"),
//     ("Item_Attach_Weapon_Muzzle_Suppressor_Medium_C": "Supressor (Handgun, SMG)"),
//     ("Item_Attach_Weapon_Muzzle_Suppressor_Small_C": "Supressor (Handgun)"),
//     ("Item_Attach_Weapon_Muzzle_Suppressor_SniperRifle_C": "Supressor (DMR, SR)"),
//     ("Item_Attach_Weapon_SideRail_DotSight_RMR_C": "Canted Sight"),
//     ("Item_Attach_Weapon_Stock_AR_Composite_C": "Tactical Stock"),
//     ("Item_Attach_Weapon_Stock_Shotgun_BulletLoops_C": "Shotgun Bullet Loops"),
//     ("Item_Attach_Weapon_Stock_SniperRifle_BulletLoops_C": "Sniper Rifle Bullet Loops"),
//     ("Item_Attach_Weapon_Stock_SniperRifle_CheekPad_C": "Sniper Rifle Cheek Pad"),
//     ("Item_Attach_Weapon_Stock_UZI_C": "Uzi Stock"),
//     ("Item_Attach_Weapon_Upper_ACOG_01_C": "4x ACOG Scope"),
//     ("Item_Attach_Weapon_Upper_Aimpoint_C": "2x Aimpoint Scope"),
//     ("Item_Attach_Weapon_Upper_CQBSS_C": "8x CQBSS Scope"),
//     ("Item_Attach_Weapon_Upper_DotSight_01_C": "Red Dot Sight"),
//     ("Item_Attach_Weapon_Upper_Holosight_C": "Holographic Sight"),
//     ("Item_Attach_Weapon_Upper_PM2_01_C": "15x PM II Scope"),
//     ("Item_Attach_Weapon_Upper_Scope3x_C": "3x Scope"),
//     ("Item_Attach_Weapon_Upper_Scope6x_C": "6x Scope"),
//     ("Item_Back_B_01_StartParachutePack_C": "Parachute"),
//     ("Item_Back_BlueBlocker": "Jammer Pack"),
//     ("Item_Back_C_01_Lv3_C": "Backpack (Level 3)"),
//     ("Item_Back_C_02_Lv3_C": "Backpack (Level 3)"),
//     ("Item_Back_E_01_Lv1_C": "Backpack (Level 1)"),
//     ("Item_Back_E_02_Lv1_C": "Backpack (Level 1)"),
//     ("Item_Back_F_01_Lv2_C": "Backpack (Level 2)"),
//     ("Item_Back_F_02_Lv2_C": "Backpack (Level 2)"),
//     ("Item_Boost_AdrenalineSyringe_C": "Adrenaline Syringe"),
//     ("Item_Boost_EnergyDrink_C": "Energy Drink"),
//     ("Item_Boost_PainKiller_C": "Painkiller"),
//     ("Item_Ghillie_01_C": "Ghillie Suit"),
//     ("Item_Ghillie_02_C": "Ghillie Suit"),
//     ("Item_Ghillie_03_C": "Ghillie Suit"),
//     ("Item_Ghillie_04_C": "Ghillie Suit"),
//     ("Item_Head_E_01_Lv1_C": "Motorcycle Helmet (Level 1)"),
//     ("Item_Head_E_02_Lv1_C": "Motorcycle Helmet (Level 1)"),
//     ("Item_Head_F_01_Lv2_C": "Military Helmet (Level 2)"),
//     ("Item_Head_F_02_Lv2_C": "Military Helmet (Level 2)"),
//     ("Item_Head_G_01_Lv3_C": "Spetsnaz Helmet (Level 3)"),
//     ("Item_Heal_Bandage_C": "Bandage"),
//     ("Item_Heal_FirstAid_C": "First Aid Kit"),
//     ("Item_Heal_MedKit_C": "Med kit"),
//     ("Item_JerryCan_C": "Gas Can"),
//     ("Item_Weapon_AK47_C": "AKM"),
//     ("Item_Weapon_Apple_C": "Apple"),
//     ("Item_Weapon_AUG_C": "AUG A3"),
//     ("Item_Weapon_AWM_C": "AWM"),
//     ("Item_Weapon_Berreta686_C": "S686"),
//     ("Item_Weapon_BerylM762_C": "Beryl"),
//     ("Item_Weapon_BizonPP19_C": "Bizon"),
//     ("Item_Weapon_C4_C": "C4"),
//     ("Item_Weapon_Cowbar_C": "Crowbar"),
//     ("Item_Weapon_Crossbow_C": "Crossbow"),
//     ("Item_Weapon_DecoyGrenade_C": "Decoy Grenade"),
//     ("Item_Weapon_DesertEagle_C": "Deagle"),
//     ("Item_Weapon_DP12_C": "DBS"),
//     ("Item_Weapon_DP28_C": "DP-28"),
//     ("Item_Weapon_FlareGun_C": "Flare Gun"),
//     ("Item_Weapon_FlashBang_C": "Flashbang"),
//     ("Item_Weapon_FNFal_C": "SLR"),
//     ("Item_Weapon_G18_C": "P18C"),
//     ("Item_Weapon_G36C_C": "G36C"),
//     ("Item_Weapon_Grenade_C": "Frag Grenade"),
//     ("Item_Weapon_Grenade_Warmode_C": "Frag Grenade"),
//     ("Item_Weapon_Groza_C": "Groza"),
//     ("Item_Weapon_HK416_C": "M416"),
//     ("Item_Weapon_Kar98k_C": "Kar98k"),
//     ("Item_Weapon_M16A4_C": "M16A4"),
//     ("Item_Weapon_M1911_C": "P1911"),
//     ("Item_Weapon_M249_C": "M249"),
//     ("Item_Weapon_M24_C": "M24"),
//     ("Item_Weapon_M9_C": "P92"),
//     ("Item_Weapon_Machete_C": "Machete"),
//     ("Item_Weapon_MG3_C": "MG3"),
//     ("Item_Weapon_Mini14_C": "Mini 14"),
//     ("Item_Weapon_Mk14_C": "Mk14 EBR"),
//     ("Item_Weapon_Mk47Mutant_C": "Mk47 Mutant"),
//     ("Item_Weapon_Molotov_C": "Molotov Cocktail"),
//     ("Item_Weapon_Mosin_C": "Mosin-Nagant"),
//     ("Item_Weapon_MP5K_C": "MP5K"),
//     ("Item_Weapon_NagantM1895_C": "R1895"),
//     ("Item_Weapon_Pan_C": "Pan"),
//     ("Item_Weapon_PanzerFaust100M_C": "Panzerfaust"),
//     ("Item_Weapon_QBU88_C": "QBU88"),
//     ("Item_Weapon_QBZ95_C": "QBZ95"),
//     ("Item_Weapon_Rhino_C": "R45"),
//     ("Item_Weapon_Rock_C": "Rock"),
//     ("Item_Weapon_Saiga12_C": "S12K"),
//     ("Item_Weapon_Sawnoff_C": "Sawed-off"),
//     ("Item_Weapon_SCAR-L_C": "SCAR-L"),
//     ("Item_Weapon_Sickle_C": "Sickle"),
//     ("Item_Weapon_SKS_C": "SKS"),
//     ("Item_Weapon_SmokeBomb_C": "Smoke Grenade"),
//     ("Item_Weapon_Snowball_C": "Snowball"),
//     ("Item_Weapon_SpikeTrap_C": "Spike Trap"),
//     ("Item_Weapon_StickyGrenade_C": "Sticky Bomb"),
//     ("Item_Weapon_Thompson_C": "Tommy Gun"),
//     ("Item_Weapon_UMP_C": "UMP9"),
//     ("Item_Weapon_UZI_C": "Micro Uzi"),
//     ("Item_Weapon_Vector_C": "Vector"),
//     ("Item_Weapon_VSS_C": "VSS"),
//     ("Item_Weapon_vz61Skorpion_C": "Skorpion"),
//     ("Item_Weapon_Win1894_C": "Win94"),
//     ("Item_Weapon_Winchester_C": "S1897"),
//     ("WarModeStartParachutePack_C": "Parachute"),
//     ("SP6_EventItem_DVD_01_C": "Event Item"),
//     ("SP6_EventItem_DVD_02_C": "Event Item"),
//     ("SP6_EventItem_DVD_03_C": "Event Item")
//   }





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
