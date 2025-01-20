use crate::memory::{process::D2RInstance, structs::MenuStates};

impl MenuStates {
    pub fn get_menu_states(d2rprocess: &D2RInstance) -> Self {
        d2rprocess.read_mem_offset::<MenuStates>(d2rprocess.offsets.ui_offset)
    }
    
    #[allow(unused)]
    pub fn is_left_panel_open(&self) -> bool {
        self.stats_visible
            || self.stash_visible
            || self.cube_visible
            || self.vendor_inventory_visible
            || self.merc_inventory_visible
            || self.quest_menu_visible
            || self.waypoint_menu_visible
            || self.party_visible
            || self.pause_menu_visible
    }

    pub fn is_panel_open(&self) -> bool {
        self.inventory_visible
            || self.stash_visible
            || self.cube_visible
            || self.vendor_inventory_visible
            || self.merc_inventory_visible
            || self.quest_menu_visible
            || self.stats_visible
            || self.skill_tree_visible
            || self.waypoint_menu_visible
            || self.party_visible
            || self.pause_menu_visible
            || self.npc_dialog_visible
            || self.skill_popover_visible
    }
}
