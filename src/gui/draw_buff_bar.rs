use std::collections::HashMap;
use std::time::Instant;
use std::time::SystemTime;

use notan::draw::*;
use notan::prelude::*;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::buffs::BuffTimer;
use crate::types::buffs::BuffTimers;
use crate::types::states::State;

use super::Fonts;

pub fn draw_buff_bar(draw: &mut Draw, game_data: &GameData, settings: &Settings, all_fonts: &Fonts, buff_bar_animation: &mut BuffBarAnimationState, skill_popover_visible: bool, buff_timers: &BuffTimers, width: &u32, height: &u32, images: &HashMap<String, Texture>) {
    if !settings.buffbar.enabled {
        return;
    }
    if skill_popover_visible {
        return;
    }
    let width = *width as f32;
    let height = *height as f32;
    let icon_size = (1.0 / settings.buffbar.icon_scale) * height;

    buff_bar_animation.update(&game_data.player.states);

    if !buff_bar_animation.buff_icons.is_empty() {
        let mut x = ((width / 2.0) - (buff_bar_animation.buff_icons.len() as f32 * icon_size) / 2.0) * (settings.buffbar.horizontal_pos * 2.0);
        let y = height * settings.buffbar.vertical_pos;
        for state_icon in buff_bar_animation.buff_icons.iter() {
            let color = match state_icon.buff_group {
                BuffGroup::Debuff => Color::RED,
                BuffGroup::Buff => Color::GREEN,
                BuffGroup::Aura => Color::YELLOW,
                BuffGroup::Passive => Color::GRAY,
            };
            match images.get(&state_icon.image_name) {
                Some(image) => {
                    if state_icon.removing == true && state_icon.buff_group == BuffGroup::Buff {
                        // do the removal animation here
                        if state_icon.animation_frame % 2 == 0 {
                            draw.rect((x - 2.5, y - 2.5), (icon_size + 5.0, icon_size + 5.0)).color(Color::RED);
                        }
                        draw.image(image).position(x, y).size(icon_size, icon_size).alpha(0.8);
                        // draw.text(&all_fonts.formal_font, &state_icon.animation_frame.to_string()).position(x + icon_size - 20.0, y + icon_size - 20.0).size(20.0).color(Color::WHITE);

                    } else if state_icon.removing == false {
                        draw.rect((x - 1.0, y - 1.0), (icon_size + 2.0, icon_size + 2.0)).color(color);
                        draw.image(image).position(x, y).size(icon_size, icon_size);
                        if state_icon.state == State::BattleOrders {
                            draw_timer_text(draw, x, y, icon_size, all_fonts, &buff_timers.battle_orders);
                        }
                        if state_icon.state == State::BattleCommand {
                            draw_timer_text(draw, x, y, icon_size, all_fonts, &buff_timers.battle_command);
                        }
                    }
                    x = x + icon_size + 3.0;
                }
                None => {
                    log::info!("Found unknown state icon {}", &state_icon.image_name);
                },
            };
            
        }
    }
    
}

pub fn draw_timer_text(draw: &mut Draw, x: f32, y: f32, icon_size: f32, all_fonts: &Fonts, buff_timer: &BuffTimer) {
    if buff_timer.expiration > Instant::now() {
        let seconds_remaining = buff_timer.expiration.duration_since(Instant::now()).as_secs_f32().round();
        if seconds_remaining > 0.0 {
            draw.text(&all_fonts.formal_font, &seconds_remaining.to_string()).position(x + (icon_size / 2.0), y - 22.0).size(20.0).h_align_center().color(Color::WHITE);
        }
    }
    
}


#[derive(Default)]
pub struct BuffBarAnimationState {
    pub buff_icons: Vec<BuffIcon>
}

impl BuffBarAnimationState {
    pub fn update(&mut self, states: &[State; 192]) {
        for state in states.iter() {
            if state != &State::None {
                match get_buff_bar_icon(state) {
                    Some(new_icon) => {
                        if self.buff_icons.iter_mut().find(|icon| icon.state == *state).is_none() {
                            // add new one
                            self.buff_icons.push(new_icon);
                        }
                    },
                    None => ()
                }
            }
        }
        self.buff_icons.sort();
        self.buff_icons.dedup();
        for buff_icon in &mut self.buff_icons {
            if buff_icon.removing {
                buff_icon.animation_frame += 1;
            }
        }
        self.buff_icons.retain(|buff_icon| buff_icon.animation_frame <= 100);
        
        
        for buff_icon in self.buff_icons.iter_mut() {
            if states.iter().find(|state| *state == &buff_icon.state).is_none() {
                if buff_icon.removing == false {
                    // state disappeared so set removing state to true and begin animation
                    // log::info!("Removing state icon {}", &buff_icon.image_name);
                    buff_icon.removing = true;
                    buff_icon.started = SystemTime::now();
                }
            } else {
                // state has come back during animation so reset animation
                if buff_icon.removing == true {
                    // log::info!("Aborting removal of state icon {}", &buff_icon.image_name);
                    buff_icon.removing = false;
                    buff_icon.animation_frame = 0;
                    buff_icon.started = SystemTime::now();
                }
            }
        }
        // non buff icons are removed immediately
        self.buff_icons.retain(|buff_icon| (buff_icon.buff_group != BuffGroup::Buff && buff_icon.removing == false) || buff_icon.buff_group == BuffGroup::Buff);
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BuffIcon {
    pub image_name: String,
    pub buff_group: BuffGroup,
    pub removing: bool,
    pub state: State,
    pub animation_frame: u32,
    pub started: SystemTime,
}

impl BuffIcon {
    pub fn new(state: State, buff_group: BuffGroup, removing: bool) -> Self {
        Self {
            image_name: state.to_string(),
            buff_group,
            removing,
            state,
            animation_frame: 0,
            started: SystemTime::now(),
        }
    }
}

pub fn get_buff_bar_icon(state: &State) -> Option<BuffIcon> {
    match state {
        State::ResistFire |
        State::ResistCold |
        State::ResistLight |
        State::ResistAll |
        State::Conviction |  // this is a buff and debuff
        State::Might |
        State::Prayer |
        State::HolyFire |
        State::Thorns |
        State::Defiance |
        State::BlessedAim |
        State::Stamina |
        State::Concentration |
        State::HolyWind |
        State::Cleansing |
        State::HolyShock |
        State::Sanctuary |
        State::Meditation |
        State::Fanaticism |
        State::Redemption |
        State::Barbs |
        State::Wolverine |
        State::OakSage => {
            Some(BuffIcon::new(state.clone(), BuffGroup::Aura, false))
        },
        State::FrozenArmor |
        State::Inferno |
        State::Blaze |
        State::BoneArmor |
        State::Enchant |
        State::InnerSight |
        State::ChillingArmor |
        State::Shout |
        State::EnergyShield |
        State::VenomClaws |
        State::BattleOrders |
        State::Thunderstorm |
        State::BattleCommand |
        State::SlowMissiles |
        State::ShiverArmor |
        State::Valkyrie |
        State::Frenzy |
        State::Berserk |
        State::HolyShield |
        State::ShadowWarrior |
        State::FeralRage |
        State::Wolf |
        State::Bear |
        State::Hurricane |
        State::Armageddon |
        State::CycloneArmor |
        State::CloakOfShadows |
        State::Cloaked |
        State::Quickness |
        State::Bladeshield |
        State::Fade => {
            Some(BuffIcon::new(state.clone(), BuffGroup::Buff, false))
        },
        State::Poison |
        State::AmplifyDamage |
        State::Cold |
        State::Weaken |
        State::DimVision |
        State::Slowed |
        // State::Conviction |
        State::Convicted |
        State::Conversion |
        State::IronMaiden |
        State::Terror |
        State::Attract |
        State::LifeTap |
        State::Confuse |
        State::Decrepify |
        State::LowerResist |
        State::DefenseCurse |
        State::BloodMana => {
            Some(BuffIcon::new(state.clone(), BuffGroup::Debuff, false))
        },
        _ => None
    }

}


#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[allow(dead_code)]
pub enum BuffGroup {
    Debuff,
    Buff,
    Aura,
    Passive
}