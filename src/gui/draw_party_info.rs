use notan::draw::*;
use notan::prelude::*;

use crate::mapgeneration::jsondata::LevelName;
use crate::memory::gamedata::GameData;

pub fn draw_party_info(draw: &mut Draw, game_data: &GameData, formal_font: &Font, party_portaits: bool, width: &u32, height: &u32) {
    if game_data.menus.is_left_panel_open() {
        return;
    }
    if !party_portaits {
        return;
    }
    let width = *width as f32;
    let height = *height as f32;
    
    let (left_margin, top_margin, spacing) = if width / height > 2.0 {
        //if ultrawide
        let left_margin = (width / 2.0) - (1.034 * height);
        let top_margin = height / 53.0;
        let spacing = height / 10.59;
        (left_margin, top_margin, spacing)
    } else {
        // if not ultrawide
        let left_margin = height / 46.0;
        let top_margin = height / 51.5;
        let spacing = height / 10.6;
        (left_margin, top_margin, spacing)
    };

    let yoffset = top_margin + (spacing * 1.63);
    let xoffset = left_margin - 1.0;
    let current_player = game_data.roster_items.get(0).unwrap();
    log::info!("{:?}", game_data.roster_items);
    
    let font_size_plvl = spacing / 5.0;
    let font_size_area = spacing / 7.0;
    for (n, roster) in game_data.roster_items.iter().enumerate() {
        let is_hostile = match roster.hostile_info.get(0) {
            Some(h) => h.hostile_flag > 0,
            None => false,
        };
        if roster.party_id == current_player.party_id && is_hostile == false && roster.party_id != u16::MAX {
            if n > 0 {
                let plvl = format!("{}", roster.player_level);
                
                draw.text(formal_font, &plvl)
                    .position(xoffset + (spacing * 0.06), yoffset + ((n-1) as f32 * spacing))
                    .size(font_size_plvl)
                    .color(Color::from_hex(0xDDDDDDDD))
                    .h_align_left()
                    .v_align_bottom();

                if roster.area != LevelName::None {
                    
                    draw.text(formal_font, &roster.area.to_string())
                        .position(xoffset, yoffset + ((n-1) as f32 * spacing) + (spacing * 0.37))
                        .size(font_size_area)
                        .color(Color::from_hex(0xC6B276FF))
                        .h_align_left()
                        .v_align_bottom();
                }
            }
        }
    };
}