use std::sync::Mutex;
use std::time::Instant;

use derivative::Derivative;
use notan::draw::*;
use notan::prelude::*;
use lazy_static::lazy_static;
use linked_hash_set::LinkedHashSet;
use sapi_lite::tts::SyncSynthesizer;
use std::thread;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::item::ItemMode;
use crate::types::item::ItemUnit;
use crate::types::item::Quality;
use crate::types::item_filter::ItemFilters;

use super::util::draw_text;
use super::util::draw_text_left;

lazy_static! {
    static ref ITEM_LOG: Mutex<LinkedHashSet<ItemLogEntry>> = Mutex::new(LinkedHashSet::with_capacity(40));
}

#[derive(Derivative)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ItemLogEntry {
    unit_id: u32,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    time_stamp: Instant,
}

pub fn draw_item_log(
    draw: &mut Draw,
    game_data: &GameData,
    settings: &Settings,
    width: &f32, 
    height: &f32,
    exocet_font: &Font,
    item_frame: i32,
    item_filters: &ItemFilters
) {
    let mut item_log = ITEM_LOG.lock().unwrap();

    // apply filter
    game_data.items.iter().for_each(|item| {
        if !item_log.iter().any(|log| log.unit_id == item.unit_id) {
            if item_filters.match_filter(item) {
                let this_item = ItemLogEntry {
                    unit_id: item.unit_id,
                    time_stamp: Instant::now(),
                };
                match item.mode {
                    ItemMode::OnGround | ItemMode::Dropping => {
                        item_log.insert(this_item);

                        // text to speech needs to be async
                        if settings.item_log.voice_enabled {
                            let item_text = item.get_tts_description().clone();
                            let speech_volume = settings.item_log.voice_volume.clone();
                            let speech_rate = settings.item_log.voice_speed.clone();
                                
                            thread::spawn(move || {
                                sapi_lite::initialize().unwrap_or_default();
                                match SyncSynthesizer::new() {
                                    Ok(synth) => {
                                        synth.set_volume(speech_volume).unwrap_or_default();
                                        synth.set_rate(speech_rate).unwrap_or_default();
                                        match synth.speak(item_text, None) {
                                            Ok(_) => (),
                                            Err(e) => log::debug!("Text to speech error: {:?}", e)
                                        }
                                        sapi_lite::finalize();
                                    },
                                    Err(e) => {
                                        log::debug!("Text to speech error: {:?}", e);
                                        sapi_lite::finalize();
                                    }
                                };
                            });
                        }
                    }
                    _ => (),
                }
            }
        }
    });
    let player_pos = (game_data.player.pos_x, game_data.player.pos_y);
    let scale = settings.visual.scale;
    let itemx: f32 = 10.0;
    let mut itemy: f32 = 50.0;
    for item in game_data.items.iter() {
        let item_log_entry = item_log.iter().find(|p| p.unit_id == item.unit_id);
        match item_log_entry {
            Some(item_log_entry) => {
                // draw item log at top left
                if settings.item_log.enabled {
                    if item_log_entry.time_stamp.elapsed().as_secs() < settings.item_log.text_duration as u64 {
                        let text_color = get_quality_color(&item);

                        let item_text = item.get_item_log_name(settings.item_log.ground_alerts_show_suffix_prefix);
                        draw_text_left(
                            draw,
                            exocet_font,
                            &item_text,
                            itemx,
                            itemy,
                            settings.item_log.text_size,
                            text_color,
                            true,
                            true,
                        );
                        itemy += settings.item_log.text_size + 3.0
                    }
                }
                // draw ground alert with animated dot
                if settings.item_log.ground_alerts {
                    match item.mode {
                        ItemMode::OnGround | ItemMode::Dropping => {
                            let item_pos = transform_position((item.pos_x, item.pos_y), player_pos, scale, width, height);
                            let item_text_pos = (item_pos.0, item_pos.1 - (2.0 * scale));
                            let item_text = item.get_item_ground_alert_name(settings.item_log.ground_alerts_show_suffix_prefix);
                            let text_color = get_quality_color(item);
                            let (dot_size, dot_trans) = {
                                let alpha = 1.0 - (item_frame as f32 * (100.0 / 20.0) / 100.0);
                                let size = item_frame as f32 / 5.0;
                                (size, alpha)
                            };

                            draw.circle(dot_size as f32 * scale)
                                .position(item_pos.0, item_pos.1)
                                .color(text_color)
                                .alpha(dot_trans);
                            draw.circle(0.5 * scale)
                                .position(item_pos.0, item_pos.1)
                                .color(text_color);

                            let font_size = settings.item_log.ground_alerts_text_size * scale;
                            draw_text(
                                draw,
                                exocet_font,
                                &item_text,
                                item_text_pos.0,
                                item_text_pos.1,
                                font_size,
                                text_color,
                                true,
                                true,
                            );
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
    }
}

fn get_quality_color(item: &ItemUnit) -> Color {
    
    match item.quality {
        Quality::Magic => Color::from_hex(0x6D6DFFFF),
        Quality::Set => Color::from_hex(0x00FC00FF),
        Quality::Rare => Color::from_hex(0xFFDD00FF),
        Quality::Unique => Color::from_hex(0xBBA45BFF),
        Quality::Crafted => Color::from_hex(0xFFFFFFFF),
        _ => {
            if item.is_ethereal() {
                Color::from_hex(0xAAAAAAFF)
            } else if item.is_rune() || item.is_key() || item.is_essence() {
                Color::from_hex(0xFFA700FF)
            } else {
                Color::from_hex(0xFFFFFFFF)
            }
        }
    }
}

fn transform_position(unit_pos: (u32, u32), player_pos: (f32, f32), scale: f32, width: &f32, height: &f32) -> (f32, f32) {
    let xdiff = unit_pos.0 as f32 - player_pos.0;
    let ydiff = unit_pos.1 as f32 - player_pos.1;

    let center_x = *width as f32 / 2.0;
    let center_y = *height as f32 / 2.0;
    let angle: f32 = std::f32::consts::FRAC_PI_4;
    let x = xdiff * angle.cos() - ydiff * angle.sin();
    let y = xdiff * angle.sin() + ydiff * angle.cos();

    let new_pos_x = center_x + (x * scale);
    let new_pos_y = center_y + (y * scale * 0.5);

    (new_pos_x, new_pos_y)
}
