use std::collections::HashSet;

use notan::draw::*;
use notan::math::Rect;
use notan::prelude::*;


use crate::gui::internationalization::{load_translations, get_translation, load_settings_language, Language};
use crate::memory::{gamedata::GameData};
use crate::settings::Settings;
use crate::types::{
    missile::{MissileType, MissileUnit},
    npc::{MonsterFlag, NPC, NPCMode, NPCType, NPCUnit},
    player::PlayerUnit,
    roster::RosterItem,
    states::State,
    stats::Immunity,
};

pub fn draw_units(draw: &mut Draw, game_data: &GameData, settings: &Settings, width: &f32, height: &f32, formal_font: &Font, korean_font: &Font, taiwan_font: &Font, exocet_font: &Font, blizzard_font: &Font) {
    let player_pos = (game_data.player.pos_x, game_data.player.pos_y);

    // draw player dot at the centre
    draw_player(&game_data.player, player_pos, draw, settings.visual.scale, width, height);

    // draw npcs
    game_data.npcs.iter().for_each(|npc| match npc.npc_type {
        NPCType::Monster => { draw_monster(npc, player_pos, draw, settings, width, height); }
        NPCType::Town => { draw_town_npc(npc, player_pos, draw, settings.visual.scale, formal_font, korean_font, taiwan_font, width, height); }
        NPCType::Pet => { draw_pet(npc, player_pos, draw, settings.visual.scale, width, height);}
        _ => (),
    });

    // draw bosses separately to ensure they draw on top
    game_data.npcs.iter().for_each(|npc| {
        if let NPCType::Boss = npc.npc_type {
            draw_boss(npc, player_pos, draw, settings, exocet_font, width, height);
        }
    });

    // draw other players
    // get players that are in the roster, but not the unit table
    let roster_players: Vec<&RosterItem> = game_data
        .roster_items
        .iter()
        .filter(|other_player| {
            !game_data
                .players
                .iter()
                .any(|player| player.unit_id == other_player.unit_id)
        })
        .collect();

    // if other players don't exist on unit tables, use roster data to draw them on map
    roster_players.iter().for_each(|player| {
        if player.area as u32 <= game_data.seed_values.level {
            if game_data.player.unit_id != player.unit_id && game_data.seed_values.level - (player.area as u32) < 4 {
                draw_other_player(
                    (player.pos_x as f32, player.pos_y as f32),
                    &player.name,
                    player_pos,
                    false,
                    draw,
                    settings.visual.scale,
                    formal_font,
                    blizzard_font,
                    width, 
                    height
                );
            }
        }
    });

    // draw other players which are on the unit table
    game_data.players.iter().for_each(|player| {
        if game_data.player.unit_id != player.unit_id {
            draw_other_player(
                (player.pos_x, player.pos_y),
                &player.player_name,
                player_pos,
                player.is_corpse,
                draw,
                settings.visual.scale,
                formal_font,
                blizzard_font,
                width, 
                height
            );
        }
    });

    // draw missiles
    if settings.missiles.enabled {
        game_data.missiles.iter().for_each(|missile: &MissileUnit| {
            draw_missle_type(missile, player_pos, draw, settings, width, height);
        });
    }
}

fn draw_monster(npc: &NPCUnit, player_pos: (f32, f32), draw: &mut Draw, settings: &Settings, width: &f32, height: &f32) {
    if !(npc.mode != NPCMode::Dead && npc.mode != NPCMode::Death) | npc.states.contains(&State::Revive) {
        return;
    }
    let scale = settings.visual.scale;
    let unique_size: f32 = settings.monsters.unique_mobs_size * scale;
    let champion_size: f32 = settings.monsters.champions_mobs_size * scale;
    let minion_mob_size: f32 = settings.monsters.minions_mobs_size * scale;
    let normal_mob_size: f32 = settings.monsters.normal_mobs_size * scale;

    // should precalculate this somewhere
    let unique_color: Color = convert_color(settings.monsters.unique_mob_color);
    let champion_color: Color = convert_color(settings.monsters.champions_mob_color);
    let minion_color: Color = convert_color(settings.monsters.minions_mob_color);
    let normal_color: Color = convert_color(settings.monsters.normal_mob_color);

    let immunities = npc.get_immunities();
    let unit_pos = (npc.pos_x, npc.pos_y);
    let (size, npc_pos, mob_color) = match npc.monster_flag {
        MonsterFlag::SuperUnique | MonsterFlag::Unique => {
            let size = (unique_size, unique_size / 1.0);
            let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
            (size, npc_pos, unique_color)
        }
        MonsterFlag::Champion => {
            let size = (champion_size, champion_size / 1.0);
            let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
            (size, npc_pos, champion_color)
        }
        MonsterFlag::Minion => {
            let size = (minion_mob_size, minion_mob_size / 1.0);
            let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
            (size, npc_pos, minion_color)
        }
        _ => {
            let size = (normal_mob_size, normal_mob_size / 1.0);
            let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
            (size, npc_pos, normal_color)
        }
    };
    if immunities.is_empty() {
        draw.ellipse(npc_pos, size).color(mob_color).scale_from(npc_pos, (1.0, 0.5));
    } else {
        draw_monster_with_immunities(draw, immunities, npc_pos, size, mob_color);
    }
}

fn draw_monster_with_immunities(
    draw: &mut Draw,
    immunities: HashSet<Immunity>,
    npc_pos: (f32, f32),
    size: (f32, f32),
    mob_color: Color
) {
    let mut colors = vec![];
    immunities.iter().for_each(|immunity| match immunity {
        Immunity::Physical => colors.push(Color::from_hex(0xCD853FFF)),
        Immunity::Magic => colors.push(Color::from_hex(0xFF8800FF)),
        Immunity::Fire => colors.push(Color::from_hex(0xFF0000FF)),
        Immunity::Lightning => colors.push(Color::from_hex(0xE0E000FF)),
        Immunity::Cold => colors.push(Color::from_hex(0x0000FFFF)),
        Immunity::Poison => colors.push(Color::from_hex(0x32CD32FF)),
        Immunity::None => colors.push(Color::from_hex(0x00000000)),
    });
    colors.sort_by_key(|c| c.hex());

    draw_immunities(npc_pos, size, colors, draw);

    draw.ellipse(npc_pos, size)
        .stroke(size.0 / 3.0)
        .color(mob_color)
        .fill()
        .scale_from(npc_pos, (1.0, 0.5));
    
}

fn draw_player(player: &PlayerUnit, player_pos: (f32, f32), draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    let size = (1.8, 0.5);
    let unit_pos = (player.pos_x, player.pos_y);
    let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    let player_color = Color::from_hex(0x2087FDFF);
    draw_cross(npc_pos, size.0 * scale, player_color, 0.4 * scale, draw);
}

fn draw_other_player(
    unit_pos: (f32, f32),
    player_name: &str,
    player_pos: (f32, f32),
    is_corpse: bool,
    draw: &mut Draw,
    scale: f32,
    formal_font: &Font,
    _blizzard_font: &Font,
    width: &f32, height: &f32
) {
    let size = (1.8, 0.5);
    let other_player_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    let color: Color = if is_corpse { Color::MAGENTA } else { Color::GREEN };

    draw_cross(other_player_pos, size.0 * scale, color, 0.4 * scale, draw);
    
    if player_name.chars().all(|x| x.is_ascii()) {

        // there is a bug drawing non-english chars, it doesn't seem to be encoding though
        let text_pos = (other_player_pos.0, (other_player_pos.1 - (7.0 * scale)));
        draw.text(formal_font, player_name)
            .position(text_pos.0 + 1.5, text_pos.1 + 1.5)
            .size(5.0 * scale)
            .color(Color::BLACK)
            .h_align_center()
            .v_align_top();
        draw.text(formal_font, player_name)
            .position(text_pos.0, text_pos.1)
            .size(5.0 * scale)
            .color(color)
            .h_align_center()
            .v_align_top();
    }
}

fn draw_town_npc(npc: &NPCUnit, player_pos: (f32, f32), draw: &mut Draw, scale: f32, formal_font: &Font, korean_font: &Font, taiwan_font: &Font,  width: &f32, height: &f32) {
    let size = (1.8, 0.5);
    let unit_pos = (npc.pos_x, npc.pos_y);
    let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    let color = Color::WHITE;
    draw_cross(npc_pos, size.0 * scale, color, 0.4 * scale, draw);
    let npc_name = format!("{:?}", npc.txt_file_no);
    draw_npc_name(npc_pos, size.1, npc_name, draw, scale, formal_font, korean_font, taiwan_font);
}

fn draw_boss(npc: &NPCUnit, player_pos: (f32, f32), draw: &mut Draw, settings: &Settings, exocet_font: &Font, width: &f32, height: &f32) {
    if npc.mode != NPCMode::Dead && npc.mode != NPCMode::Death {
        let scale = settings.visual.scale;
        let boss_color: Color = convert_color(settings.monsters.boss_mob_color);
        let size = (settings.monsters.boss_mobs_size * scale, settings.monsters.boss_mobs_size * scale / 2.0);
        let unit_pos = (npc.pos_x, npc.pos_y);
        let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
        draw.ellipse(npc_pos, size).color(boss_color);

        match npc.get_health() {
            Some((health, max_health)) => {
                let hp_percent = health as f32 / max_health as f32;
                let boss_text = format!("{:?}", npc.txt_file_no);
                draw_health_bar(npc_pos, size.1, hp_percent, boss_text, draw, settings, exocet_font);
            },
            None => (),
        }
    }
}

fn draw_npc_name(
    npc_pos: (f32, f32),
    size: f32,
    text: String,
    draw: &mut Draw,
    scale: f32,
    formal_font: &Font,
    korean_font: &Font,
    taiwan_font: &Font,    
) {
    let font_size = 4.5;
    let npc_name_pos = (npc_pos.0, npc_pos.1 - ((size + 1.0) * scale * 3.2));

    let translations = load_translations().unwrap_or_else(|_| {
        eprintln!("Failed to load translations. Falling back to English.");
        get_translation(&Language::English) 
    });

    let current_language = load_settings_language().unwrap_or(Language::English); 
    let font_to_use = match current_language {
        Language::Korean => korean_font,
        Language::TraditionalChinese => taiwan_font,
        _ => formal_font,
    };

    let mut npc_name = String::from(text);
        
    if npc_name.contains("Cain") {
        npc_name = translations.deckardcain.to_string();
    } else if npc_name.contains("Drehya") {
        npc_name = translations.drehya.to_string();
    } else if npc_name.contains("Charsi") {
        npc_name = translations.charsi.to_string();
    } else if npc_name.contains("Gheed") {
        npc_name = translations.gheed.to_string();
    } else if npc_name.contains("Kashya") {
        npc_name = translations.kashya.to_string();
    } else if npc_name.contains("Warriv") {
        npc_name = translations.warriv.to_string();
    } else if npc_name.contains("Akara") {
        npc_name = translations.akara.to_string();
    } else if npc_name.contains("Meshif") {
        npc_name = translations.meshif.to_string();
    } else if npc_name.contains("Elzix") {
        npc_name = translations.elzix.to_string();
    } else if npc_name.contains("Greiz") {
        npc_name = translations.greiz.to_string();
    } else if npc_name.contains("Drognan") {
        npc_name = translations.drognan.to_string();
    } else if npc_name.contains("Fara") {
        npc_name = translations.fara.to_string();
    } else if npc_name.contains("Lysander") {
        npc_name = translations.lysander.to_string();
    } else if npc_name.contains("Atma") {
        npc_name = translations.atma.to_string();
    } else if npc_name.contains("Geglash") {
        npc_name = translations.geglash.to_string();
    } else if npc_name.contains("Kaelan") {
        npc_name = translations.kaelan.to_string();
    } else if npc_name.contains("Jerhyn") {
        npc_name = translations.jerhyn.to_string();
    } else if npc_name.contains("Asheara") {
        npc_name = translations.asheara.to_string();
    } else if npc_name.contains("Natalya") {
        npc_name = translations.natalya.to_string();
    } else if npc_name.contains("Alkor") {
        npc_name = translations.alkor.to_string();
    } else if npc_name.contains("Ormus") {
        npc_name = translations.ormus.to_string();
    } else if npc_name.contains("Hratli") {
        npc_name = translations.hratli.to_string();
    } else if npc_name.contains("Halbu") {
        npc_name = translations.halbu.to_string();
    } else if npc_name.contains("Jamella") {
        npc_name = translations.jamella.to_string();
    } else if npc_name.contains("Tyrael") {
        npc_name = translations.tyrael.to_string();
    } else if npc_name.contains("Larzuk") {
        npc_name = translations.larzuk.to_string();
    } else if npc_name.contains("Malah") {
        npc_name = translations.malah.to_string();
    } else if npc_name.contains("Nihlathak") {
        npc_name = translations.nihlathak.to_string();
    } else if npc_name.contains("QualKehk") {
        npc_name = translations.qualkehk.to_string();
    } else if npc_name.contains("Izual") {
        npc_name = translations.izual.to_string();
    } else if npc_name.contains("Flavie") {
        npc_name = translations.navi.to_string(); // Assuming "navi" is the key for Flavie
    } else if npc_name.contains("Hadriel") {
        npc_name = translations.malachai.to_string(); // Assuming "malachai" is the key for Hadriel
    }
    

    draw.text(font_to_use, &npc_name)
        .position(npc_name_pos.0, npc_name_pos.1)
        .size(font_size * scale)
        .color(Color::TRANSPARENT)
        .h_align_center()
        .v_align_middle();

    let bounds: Rect = draw.last_text_bounds();
    let pad = 1.0;

    draw.rect(
        (bounds.x - (pad * scale), bounds.y - pad),
        (bounds.width + (pad * scale * 2.0), bounds.height + pad + pad),
    )
    .color(Color::from_hex(0x00000088));

    draw.text(font_to_use, &npc_name)
        .position(npc_name_pos.0 + 1.0, npc_name_pos.1 + 1.0)
        .size(font_size * scale)
        .color(Color::BLACK)
        .h_align_center()
        .v_align_middle();
    draw.text(font_to_use, &npc_name)
        .position(npc_name_pos.0, npc_name_pos.1)
        .size(font_size * scale)
        .color(Color::from_hex(0xc6b276FF))
        .h_align_center()
        .v_align_middle();
}

fn draw_health_bar(
    npc_pos: (f32, f32),
    size: f32,
    hp_percent: f32,
    text: String,
    draw: &mut Draw,
    settings: &Settings,
    exocet_font: &Font
) {
    let font_size = 4.5;
    // draw boss health bar
    let scale = settings.visual.scale;
    let health_bar_pos = (npc_pos.0, npc_pos.1 - (size * scale));

    draw.text(exocet_font, &text)
        .position(health_bar_pos.0, health_bar_pos.1)
        .size(font_size * scale)
        .color(Color::TRANSPARENT)
        .h_align_center()
        .v_align_middle();

    let bounds: Rect = draw.last_text_bounds();
    let pad = 1.0;

    draw.rect(
        (bounds.x - (pad * scale), bounds.y - pad),
        (bounds.width + (pad * scale * 2.0), bounds.height + pad + pad),
    )
    .color(Color::from_hex(0x00000088));
    draw.rect(
        (bounds.x - (pad * scale), bounds.y - pad),
        ((bounds.width + (pad * scale * 2.0)) * hp_percent, bounds.height + pad + pad),
    )
    .color(Color::from_hex(0xaa000088));

    draw.text(exocet_font, &text)
        .position(health_bar_pos.0 + 1.0, health_bar_pos.1 + 1.0)
        .size(font_size * scale)
        .color(Color::BLACK)
        .h_align_center()
        .v_align_middle();
    draw.text(exocet_font, &text)
        .position(health_bar_pos.0, health_bar_pos.1)
        .size(font_size * scale)
        .color(Color::from_hex(0xD4AF37FF))
        .h_align_center()
        .v_align_middle();
}

fn draw_pet(npc: &NPCUnit, player_pos: (f32, f32), draw: &mut Draw, scale: f32, width: &f32, height: &f32) {
    if !(npc.mode != NPCMode::Dead && npc.mode != NPCMode::Death) {
        return;
    }
    let size = (3.0, 1.75);
    let unit_pos = (npc.pos_x, npc.pos_y);
    let npc_pos = transform_position(unit_pos, size, player_pos, scale, width, height);
    match &npc.txt_file_no {
        NPC::Hydra | NPC::Hydra2 | NPC::Hydra3 => {
            draw.ellipse(npc_pos, size).color(Color::RED);
        }
        _ => {
            draw_cross(npc_pos, 1.8 * scale, Color::from_hex(0x436f73ff), 0.2 * scale, draw);
        }
    };
}

fn draw_missle_type(missile: &MissileUnit, player_pos: (f32, f32), draw: &mut Draw, settings: &Settings, width: &f32, height: &f32) {
    let scale = settings.visual.scale;
    let (size, color) = match missile.missile_type {
        MissileType::Fire => (settings.missiles.fire_size, convert_color(settings.missiles.fire_color)),
        MissileType::Ice => (settings.missiles.cold_size, convert_color(settings.missiles.cold_color)),
        MissileType::Lightning => (settings.missiles.lightning_size, convert_color(settings.missiles.lightning_color)),
        MissileType::Poison => (settings.missiles.poison_size, convert_color(settings.missiles.poison_color)),
        MissileType::Magic => (settings.missiles.magic_size, convert_color(settings.missiles.magic_color)),
        MissileType::Physical => (settings.missiles.physical_size, convert_color(settings.missiles.physical_color)),
        _ => (0.0, Color::TRANSPARENT),
    };
    let unit_pos = (missile.pos_x, missile.pos_y);
    let missile_pos = transform_position(unit_pos, (size, size / 2.0), player_pos, scale, width, height);
    draw.ellipse(missile_pos, (size, size / 2.0)).color(color);
    //missile overlay tint
    // draw.ellipse(missile_pos, size).color(Color::from_hex(missile.missile_color));
}

fn transform_position(
    unit_pos: (f32, f32),
    size: (f32, f32),
    player_pos: (f32, f32),
    scale: f32,
    width: &f32, 
    height: &f32
) -> (f32, f32) {
    let xdiff = unit_pos.0 - player_pos.0;
    let ydiff = unit_pos.1 - player_pos.1;

    let center_x = *width as f32 / 2.0;
    let center_y = *height as f32 / 2.0;
    let angle: f32 = std::f32::consts::FRAC_PI_4;
    let x = xdiff * angle.cos() - ydiff * angle.sin();
    let y = xdiff * angle.sin() + ydiff * angle.cos();

    let new_pos_x = center_x + (x * scale) - (size.0 / 2.0);
    let new_pos_y = center_y + (y * scale * 0.5) - (size.1 / 2.0);

    (new_pos_x, new_pos_y)
}

fn draw_cross(pos: (f32, f32), cross_size: f32, color: Color, stroke: f32, draw: &mut Draw) {
    let pos_x = pos.0;
    let pos_y = pos.1;
    draw.path()
        .move_to(pos_x, pos_y + cross_size)
        .line_to(pos_x + cross_size, pos_y + cross_size + cross_size)
        .line_to(pos_x + cross_size + cross_size, pos_y + cross_size)
        .line_to(pos_x + cross_size, pos_y)
        .line_to(pos_x + cross_size + cross_size, pos_y - cross_size)
        .line_to(pos_x + cross_size, pos_y - cross_size - cross_size)
        .line_to(pos_x, pos_y - cross_size)
        .line_to(pos_x - cross_size, pos_y - cross_size - cross_size)
        .line_to(pos_x - cross_size - cross_size, pos_y - cross_size)
        .line_to(pos_x - cross_size, pos_y)
        .line_to(pos_x - cross_size - cross_size, pos_y + cross_size)
        .line_to(pos_x - cross_size, pos_y + cross_size + cross_size)
        .line_to(pos_x, pos_y + cross_size)
        .color(color)
        .stroke(stroke)
        .scale_from((pos_x, pos_y), (1.0, 0.5));
}

fn convert_color(color_arr: [u8; 4]) -> Color {
    Color::from_bytes(color_arr[0], color_arr[1], color_arr[2], color_arr[3])
}


fn draw_immunities(npc_pos: (f32, f32), size: (f32, f32), colors: Vec<Color>, draw: &mut Draw) {

    let degrees = 360 / colors.len();
    for (im, color) in colors.iter().enumerate() {
        let x: f32 = npc_pos.0;  // x-coordinate of center of arc
        let y: f32 = npc_pos.1;  // y-coordinate of center of arc
        let radius: f32 = size.0 * 1.6;  // radius of arc
        let start_angle: f32 = 270.0 + (im * degrees) as f32;
        let end_angle: f32 = 270.0 + ((im + 1) * degrees) as f32;
        {
            let start_x = x + radius * (start_angle as f32 * std::f32::consts::PI / 180.0).cos();
            let start_y = y + radius * (start_angle as f32 * std::f32::consts::PI / 180.0).sin();
            let mut binding = draw.path();
            let path = binding.move_to(x,y);
            path.line_to(start_x, start_y);
            for degrees in ((start_angle as u32)..(end_angle as u32)).step_by(8) {
                let x1 = x + radius * (degrees as f32 * std::f32::consts::PI / 180.0).cos();
                let y1 = y + radius * (degrees as f32 * std::f32::consts::PI / 180.0).sin();
                path.line_to(x1, y1);
                // draw.rect((x1, y1), (2.0, 2.0)).color(Color::GREEN);
            }
            let end_x = x + radius * (end_angle as f32 * std::f32::consts::PI / 180.0).cos();
            let end_y = y + radius * (end_angle as f32 * std::f32::consts::PI / 180.0).sin();
            path.line_to(end_x, end_y);
            path.line_to(x, y);
            path.fill();
            path.color(*color).stroke(1.0);
            path.scale_from(npc_pos, (1.0, 0.5));
        }
    }
    
}
