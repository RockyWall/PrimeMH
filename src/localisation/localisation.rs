use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::settings::Locales;

use super::ui_text::Translations;

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
    enBG: String,
}

fn a_default() -> u32{
    1
}

 

pub struct Localisation {
    pub item_gems: HashMap<String, LocalisationEntry>,
    pub item_modifiers: HashMap<String, LocalisationEntry>,
    pub item_nameaffixes: HashMap<String, LocalisationEntry>,
    pub item_names: HashMap<String, LocalisationEntry>,
    pub item_runes: HashMap<String, LocalisationEntry>,
    pub levels: HashMap<String, LocalisationEntry>,
    pub mercenaries: HashMap<String, LocalisationEntry>,
    pub monsters: HashMap<String, LocalisationEntry>,
    pub npcs: HashMap<String, LocalisationEntry>,
    pub objects: HashMap<String, LocalisationEntry>,
    pub quests: HashMap<String, LocalisationEntry>,
    pub shrines: HashMap<String, LocalisationEntry>,
    pub skills: HashMap<String, LocalisationEntry>,
    pub primemh: HashMap<String, LocalisationEntry>,
}

impl Localisation {
    pub fn get_npc_name(&self, key_name: &String, locale: &Locales) -> String {

        let new_key_name: String = key_name.chars()
            .filter(|&c| !c.is_digit(10) && !c.is_whitespace())
            .collect();
        
        match self.npcs.get(&new_key_name.to_lowercase()) {
            Some(npc) => {
                match locale {
                    Locales::enUS => return npc.enUS.clone(),
                    Locales::zhTW => return npc.zhTW.clone(),
                    Locales::deDE => return npc.deDE.clone(),
                    Locales::esES => return npc.esES.clone(),
                    Locales::frFR => return npc.frFR.clone(),
                    Locales::itIT => return npc.itIT.clone(),
                    Locales::koKR => return npc.koKR.clone(),
                    Locales::plPL => return npc.plPL.clone(),
                    Locales::enBG => return npc.enUS.clone(),
                    Locales::Unknown => return npc.enUS.clone(),
                }
            }
            None => {
                log::error!("key_name {:?}", key_name);
                return String::new();
            }
        };
        
    }

    // pub fn get_ui_text(&self, locale: &Locales) -> HashMap<String, String> {
    //     let mut prime_text: HashMap<String, String> = HashMap::new();
    //     self.primemh.into_iter().for_each(|(key, val)| {
    //         match locale {
    //             Locales::enUS => prime_text.insert(key.clone(), val.enUS.clone()),
    //             Locales::zhTW => prime_text.insert(key.clone(), val.zhTW.clone()),
    //             Locales::deDE => prime_text.insert(key.clone(), val.deDE.clone()),
    //             Locales::esES => prime_text.insert(key.clone(), val.esES.clone()),
    //             Locales::frFR => prime_text.insert(key.clone(), val.frFR.clone()),
    //             Locales::itIT => prime_text.insert(key.clone(), val.itIT.clone()),
    //             Locales::koKR => prime_text.insert(key.clone(), val.koKR.clone()),
    //             Locales::plPL => prime_text.insert(key.clone(), val.plPL.clone()),
    //             Locales::enBG => prime_text.insert(key.clone(), val.enBG.clone()),
    //             Locales::Unknown => prime_text.insert(key.clone(), val.enUS.clone()),
    //         };
    //     });
    //     prime_text
    // }
}


fn parse_json_bytes<T: DeserializeOwned>(data: &str) -> Vec<T> {
    serde_json::from_str(data.trim_start_matches("\u{feff}")).expect("Unable to parse JSON")
}

pub fn load_localisation_data() -> Localisation {
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

    let item_gems: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_gems_data);
    let item_modifiers: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_modifiers_data);
    let item_nameaffixes: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_nameaffixes_data);
    let item_names: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_names_data);
    let item_runes: HashMap<String, LocalisationEntry> = vec_to_hashmap(item_runes_data);
    let levels: HashMap<String, LocalisationEntry> = vec_to_hashmap(levels_data);
    let mercenaries: HashMap<String, LocalisationEntry> = vec_to_hashmap(mercenaries_data);
    let monsters: HashMap<String, LocalisationEntry> = vec_to_hashmap(monsters_data);
    let npcs: HashMap<String, LocalisationEntry> = vec_to_hashmap(npcs_data);
    let objects: HashMap<String, LocalisationEntry> = vec_to_hashmap(objects_data);
    let quests: HashMap<String, LocalisationEntry> = vec_to_hashmap(quests_data);
    let shrines: HashMap<String, LocalisationEntry> = vec_to_hashmap(shrines_data);
    let skills: HashMap<String, LocalisationEntry> = vec_to_hashmap(skills_data);
    let primemh: HashMap<String, LocalisationEntry> = vec_to_hashmap(primemh_data);
    Localisation { item_gems, item_modifiers, item_nameaffixes, item_names, item_runes, levels, mercenaries, monsters, npcs, objects, quests, shrines, skills, primemh }
}

fn vec_to_hashmap(file_data: Vec<LocalisationEntry>) -> HashMap<String, LocalisationEntry> {
    let mut hashmap: HashMap<String, LocalisationEntry> = HashMap::new();
    for entry in file_data {
        hashmap.insert(entry.Key.clone().to_lowercase().replace("-",""), entry);
    }
    hashmap
}