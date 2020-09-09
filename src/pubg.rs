pub mod guns {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io::prelude::*;

    // explanations for data below https://pubg.gamepedia.com/Data_Key
    // Note: Enums can be used with structs
    #[derive(Serialize, Deserialize, Debug)]
    struct GunStats {
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
    enum GunVsGun<'a> {
        Gun(&'a GunStats),
        ErrMessage(&'a str),
    }

    pub fn gun_vs_gun(gun1: String, gun2: String) -> GunVsGun<'static> {
        let gun_list = get_gun_list();
        if gun_list.is_ok() {
            let guns = gun_list.unwrap();
            let winner = compare_guns(
                get_gun_stats(&guns, gun1.to_string()),
                get_gun_stats(&guns, gun2.to_string()),
            );
            GunVsGun::Gun(winner)
        } else {
            println!("ERR");
            GunVsGun::ErrMessage("Error")
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
        let file_contents_to_json: Value = serde_json::from_str(&contents).unwrap();
        // convert to an array with the gun objects
        let res: Vec<GunStats> = serde_json::from_value(file_contents_to_json).unwrap();

        Ok(res)
    }

    // get a specific gun from the list
    fn get_gun_stats<'a>(gun_list: &'a Vec<GunStats>, gun_name: String) -> &'a GunStats {
        gun_list.iter().find(|&gun| gun.Name == gun_name).unwrap()
    }

    fn compare_guns<'a>(gun1: &'a GunStats, gun2: &'a GunStats) -> &'a GunStats {
        if gun1.BDMG1 > gun2.BDMG1 {
            gun1
        } else {
            gun2
        }
    }
}

pub mod player {
    pub fn hello() {
        println!("wow we hit the plaer meth")
    }
}
