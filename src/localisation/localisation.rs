use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::settings::Locales;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LocalisationEntry {
    #[serde(default = "a_default")]
    id: u32,
    Key: String,
    enUS: String,
    zhTW: String,
    deDE: String,
    esES: String,
    frFR: String,
    itIT: String,
    koKR: String,
    plPL: String,
    #[serde(default = "blank")]
    enBG: String,
}

fn a_default() -> u32{
    1
}

fn blank() -> String {
    String::new()
}

 

pub struct Localisation {
    pub item_gems: HashMap<String, String>,
    pub item_modifiers: HashMap<String, String>,
    pub item_nameaffixes: HashMap<String, String>,
    pub item_names: HashMap<String, String>,
    pub item_runes: HashMap<String, String>,
    pub levels: HashMap<String, String>,
    pub mercenaries: HashMap<String, String>,
    pub monsters: HashMap<String, String>,
    pub npcs: HashMap<String, String>,
    pub objects: HashMap<String, String>,
    pub quests: HashMap<String, String>,
    pub shrines: HashMap<String, String>,
    pub skills: HashMap<String, String>,
    pub primemh: HashMap<String, String>,
}

impl Localisation {
    pub fn get_npc_name(&self, key_name: &String) -> String {
        let new_key_name: String = key_name.chars()
            .filter(|&c| !c.is_digit(10) && !c.is_whitespace())
            .collect();
        let new_string = self.npcs.get(&new_key_name.to_lowercase().replace("-",""));
        return match new_string {
            Some(s) => s.clone(),
            None => String::new(),
        }
    }

    pub fn get(&self, key_name: &'static str) -> String {
        let new_key_name: String = key_name.chars()
            .filter(|&c| !c.is_digit(10) && !c.is_whitespace())
            .collect();
        let new_string = self.primemh.get(&new_key_name.to_lowercase().replace("-",""));
        return match new_string {
            Some(s) => s.clone(),
            None => String::from(key_name),
        }
    }

    pub fn get_level(&self, key_name: &String) -> String {
        let mut new_key_name: String = key_name.chars()
            .filter(|&c| !c.is_whitespace())
            .collect();
        new_key_name = new_key_name.to_lowercase().replace("-","");
        let new_string = self.levels.get(&new_key_name);
        return match new_string {
            Some(s) => s.clone(),
            None => String::from(key_name),
        }
    }
}

fn parse_json_bytes<T: DeserializeOwned>(data: &str) -> Vec<T> {
    serde_json::from_str(data.trim_start_matches("\u{feff}")).expect("Unable to parse JSON")
}

pub fn load_localisation_data(locale: &Locales) -> Localisation {
    let item_gems_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-gems.json"));
    let item_modifiers_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-modifiers.json"));
    let item_nameaffixes_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-nameaffixes.json"));
    let item_names_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-names.json"));
    let item_runes_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/item-runes.json"));
    let levels_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/levels.json"));
    let mercenaries_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/mercenaries.json"));
    let monsters_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/monsters.json"));
    let npcs_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/npcs.json"));
    let objects_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/objects.json"));
    let quests_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/quests.json"));
    let shrines_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/shrines.json"));
    let skills_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/skills.json"));
    let primemh_data: Vec<LocalisationEntry> = parse_json_bytes(include_str!("./reference/primemh.json"));

    let item_gems: HashMap<String, String> = vec_to_hashmap(item_gems_data, locale);
    let item_modifiers: HashMap<String, String> = vec_to_hashmap(item_modifiers_data, locale);
    let item_nameaffixes: HashMap<String, String> = vec_to_hashmap(item_nameaffixes_data, locale);
    let item_names: HashMap<String, String> = vec_to_hashmap(item_names_data, locale);
    let item_runes: HashMap<String, String> = vec_to_hashmap(item_runes_data, locale);
    let levels: HashMap<String, String> = vec_to_hashmap(levels_data, locale);
    let mercenaries: HashMap<String, String> = vec_to_hashmap(mercenaries_data, locale);
    let monsters: HashMap<String, String> = vec_to_hashmap(monsters_data, locale);
    let npcs: HashMap<String, String> = vec_to_hashmap(npcs_data, locale);
    let objects: HashMap<String, String> = vec_to_hashmap(objects_data, locale);
    let quests: HashMap<String, String> = vec_to_hashmap(quests_data, locale);
    let shrines: HashMap<String, String> = vec_to_hashmap(shrines_data, locale);
    let skills: HashMap<String, String> = vec_to_hashmap(skills_data, locale);
    let primemh: HashMap<String, String> = vec_to_hashmap(primemh_data, locale);
    Localisation { item_gems, item_modifiers, item_nameaffixes, item_names, item_runes, levels, mercenaries, monsters, npcs, objects, quests, shrines, skills, primemh }
}

fn vec_to_hashmap(file_data: Vec<LocalisationEntry>, locale: &Locales) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for entry in file_data {
        
        let val: String = match locale {
            Locales::enUS => entry.enUS.clone(),
            Locales::zhTW => entry.zhTW.clone(),
            Locales::deDE => entry.deDE.clone(),
            Locales::esES => entry.esES.clone(),
            Locales::frFR => entry.frFR.clone(),
            Locales::itIT => entry.itIT.clone(),
            Locales::koKR => entry.koKR.clone(),
            Locales::plPL => entry.plPL.clone(),
            Locales::enBG => entry.enUS.clone(),
            Locales::Unknown => entry.enUS.clone(),
        };
        let new_key_name: String = entry.Key.chars()
            .filter(|&c| !c.is_whitespace())
            .collect();
        hashmap.insert(new_key_name.to_lowercase().replace("-",""), val);
    }
    hashmap
}