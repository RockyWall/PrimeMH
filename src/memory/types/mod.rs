use item::ItemUnit;
use notan::egui::epaint::ahash::HashSet;

use crate::memory::{process::D2RInstance, structs::{Unit, UIWidget, UIPanelManager}};

use self::{player::PlayerUnit, missile::MissileUnit, object::GameObjectUnit, npc::NPCUnit};

pub mod item;
pub mod item_filter;
pub mod menus;
pub mod missile;
pub mod npc;
pub mod object;
pub mod player;
pub mod roster;
pub mod seedvalues;
pub mod stats;
pub mod states;
pub mod last_hovered;
pub mod skills;
pub mod affixes;

pub fn get_players(d2rprocess: &D2RInstance, unit_ptrs: [u64; 128]) -> Vec<PlayerUnit> {
    let units: Vec<Unit> = get_raw_units(d2rprocess, unit_ptrs);
    units.iter().map(|unit| PlayerUnit::new(d2rprocess, *unit)).collect()
}

pub fn get_npcs(d2rprocess: &D2RInstance, unit_ptrs: [u64; 128]) -> Vec<NPCUnit> {
    let units: Vec<Unit> = get_raw_units(d2rprocess, unit_ptrs);
    units.iter().map(|unit| NPCUnit::new(d2rprocess, *unit)).collect()
}


pub fn get_missiles(d2rprocess: &D2RInstance, missile_ptrs: [u64; 128], server_missile_ptrs: [u64; 128]) -> Vec<MissileUnit> {
    let mut missile_units: Vec<Unit> = get_raw_units(d2rprocess, missile_ptrs);
    let mut server_missile_units: Vec<Unit> = get_raw_units(d2rprocess, server_missile_ptrs);
    missile_units.append(&mut server_missile_units);
    missile_units.iter().map(|unit| MissileUnit::new(d2rprocess, *unit)).collect()
}

pub fn get_items(d2rprocess: &D2RInstance, item_ptrs: [u64; 128]) -> Vec<ItemUnit> {
    let units: Vec<Unit> = get_raw_units(d2rprocess, item_ptrs);
    let mut item_units: Vec<ItemUnit> = vec![];
    for unit in units.iter() {
        match ItemUnit::new(d2rprocess, *unit) {
            Some(item) => item_units.push(item),
            None => break
        }
    }
    item_units
}

pub fn get_objects(d2rprocess: &D2RInstance, object_ptrs: [u64; 128]) -> Vec<GameObjectUnit> {
    let units: Vec<Unit> = get_raw_units(d2rprocess, object_ptrs);
    units.iter().map(|unit| GameObjectUnit::new(d2rprocess, *unit)).collect()
}



pub fn get_raw_units(d2rprocess: &D2RInstance, unit_ptrs: [u64; 128]) -> Vec<Unit> {
    let mut units: Vec<Unit> = vec![];
    unit_ptrs.iter().for_each(|u| {
        if u > &0 {
            follow_p_next(d2rprocess, *u, &mut units);
        }
    });
    units
}

fn follow_p_next(d2rprocess: &D2RInstance, memory_address: u64, units: &mut Vec<Unit>) {
    let unit: Unit = d2rprocess.read_mem::<Unit>(memory_address);
    units.push(unit);
    if unit.p_next > 0 {
        follow_p_next(d2rprocess, unit.p_next, units);
    }
}

#[allow(dead_code)]
pub fn get_panels(d2rprocess: &D2RInstance) -> HashSet<String> {
    let mut widgets: Vec<UIWidget> = vec![];
    let p_panelmanager: u64 = d2rprocess.read_mem_offset::<u64>(d2rprocess.offsets.panels);
    let panelmanager: UIPanelManager = d2rprocess.read_mem::<UIPanelManager>(p_panelmanager);
    follow_p_next_panel(d2rprocess, panelmanager.children, panelmanager.num_children, &mut widgets);
    let mut panel_names: HashSet<String> = HashSet::default();
    for widget in widgets.iter() {
        let name_arr = d2rprocess.read_mem::<[u8; 16]>(widget.p_name);
        let panel_name = d2rprocess.parse_arr_to_string(&name_arr);
        panel_names.insert(panel_name);
    }
    panel_names
}

#[allow(dead_code)]
fn follow_p_next_panel(d2rprocess: &D2RInstance, memory_address: u64, num_children: u64, widgets: &mut Vec<UIWidget>) {
    for i in 0..num_children {
        let p_widget: u64 = d2rprocess.read_mem::<u64>(memory_address + (i * 8));
        let widget: UIWidget = d2rprocess.read_mem::<UIWidget>(p_widget);
        widgets.push(widget);
        if widget.children > 0 {
            follow_p_next_panel(d2rprocess, widget.children, widget.num_children, widgets);
        }
    }
}

