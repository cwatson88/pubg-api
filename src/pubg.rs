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

    #[derive(Debug)]
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

pub mod player {
    pub fn hello() {
        println!("wow we hit the plaer meth")
    }
}
