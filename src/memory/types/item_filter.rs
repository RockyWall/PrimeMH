use crate::ITEM_FILTER_FILE;
use serde::{de, Deserialize, Deserializer};
use serde_yaml::Error;
use std::{collections::HashMap, fs::File, str::FromStr};

use super::item::{BaseItem, ItemUnit, Quality};
use convert_case::{Case, Casing};

#[derive(Clone, PartialEq)]
pub struct ItemFilters {
    pub filters: HashMap<BaseItem, Vec<ItemFilter>>,
}

impl ItemFilters {
    pub fn load() -> Option<Self> {
        let mut path = std::env::current_dir().unwrap();
        path.push(ITEM_FILTER_FILE);
        if let Ok(file) = File::open(path.as_path()) {
            let results: Result<HashMap<BaseItem, Vec<ItemFilter>>, Error> = serde_yaml::from_reader(file);
            if results.is_ok() {
                return Some(ItemFilters {
                    filters: results.unwrap(),
                });
            } else {
                log::error!("Could not load itemfilter {:?}", results.err().unwrap());
                None
            }
        } else {
            log::error!("Error opening item filter file {:?}", path);
            None
        }
    }

    pub fn match_filter(&self, item: &ItemUnit) -> bool {
        let filters = match self.filters.get(&item.txt_file_no) {
            None => return false, // base item not in filter
            Some(filters) => filters,
        };

        if filters.is_empty() {
            // no filters of base item, so match immediately
            return true;
        }

        for filter in filters.iter() {
            if (match &filter.ethereal {
                Some(eth) => &item.is_ethereal() == eth,
                None => true,
            } && match &filter.quality {
                Some(qualities) => qualities.contains(&item.quality),
                None => true,
            } && match &filter.sockets {
                Some(socket_vec) => socket_vec.contains(&item.num_sockets),
                None => true,
            }) {
                return true;
            }
        }
        false
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ItemFilter {
    pub quality: Option<Vec<Quality>>,
    pub ethereal: Option<bool>,
    pub play_sound_on_drop: Option<bool>,
    pub sockets: Option<Vec<u8>>,
    pub identified: Option<bool>,
}

impl<'de> Deserialize<'de> for Quality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let original = String::deserialize(deserializer)?;
        let s = original.to_case(Case::Pascal);
        let qualitity = match Quality::from_str(&s) {
            Ok(qualitity) => qualitity,
            Err(e) => {
                return Err(de::Error::custom(format!("Invalid item quality : '{}' Error: '{}'", original, e)));
            }
        };
        Ok(qualitity)
    }
}

impl<'de> Deserialize<'de> for BaseItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let original = String::deserialize(deserializer)?;
        let s = original.to_case(Case::Pascal);
        let item = match BaseItem::from_str(&s) {
            Ok(item) => item,
            Err(e) => {
                return Err(de::Error::custom(format!("Invalid item name : '{}' Error: '{}'", original, e)));
            }
        };
        Ok(item)
    }
}
