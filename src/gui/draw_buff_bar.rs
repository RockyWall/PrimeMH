use std::collections::HashMap;

use notan::draw::*;
use notan::prelude::*;

use crate::memory::gamedata::GameData;
use crate::settings::Settings;
use crate::types::states::State;

pub fn draw_buff_bar(draw: &mut Draw, game_data: &GameData, settings: &Settings, width: &u32, height: &u32, images: &HashMap<String, Texture>) {
    if !settings.buffbar.enabled {
        return;
    }
    let width = *width as f32;
    let height = *height as f32;
    
    let mut state_icons: Vec<BuffIcon> = vec![];
    for state in game_data.player.states.iter() {
        if state != &State::None {
            match get_buff_bar_icon(state) {
                Some(icon) => state_icons.push(icon),
                None => ()
            }
        }
    }
    let icon_size = (1.0 / settings.buffbar.icon_scale) * height;

    if !state_icons.is_empty() {
        let mut x = (width / 2.0) - (state_icons.len() as f32 * icon_size) / 2.0;
        let y = height * settings.buffbar.vertical_pos;
        for state_icon in state_icons.iter() {
            let color = match state_icon.buff_group {
                BuffGroup::Debuff => Color::RED,
                BuffGroup::Buff => Color::GREEN,
                BuffGroup::Aura => Color::YELLOW,
                BuffGroup::Passive => Color::GRAY,
            };
            match images.get(&state_icon.image_name) {
                Some(image) => {
                    draw.rect((x - 1.0, y - 1.0), (icon_size + 2.0, icon_size + 2.0)).color(color);
                    draw.image(image).position(x, y).size(icon_size, icon_size);
                    x = x + icon_size + 3.0;
                }
                None => {
                    println!("{}", &state_icon.image_name);
                },
            };
            
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
            Some(BuffIcon { image_name: state.to_string(), buff_group: BuffGroup::Aura })
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
            Some(BuffIcon { image_name: state.to_string(), buff_group: BuffGroup::Buff })
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
            Some(BuffIcon { image_name: state.to_string(), buff_group: BuffGroup::Debuff })
        },
        _ => None
    }

}


pub struct BuffIcon {
    pub image_name: String,
    pub buff_group: BuffGroup
}

#[allow(dead_code)]
pub enum BuffGroup {
    Debuff,
    Buff,
    Aura,
    Passive
}