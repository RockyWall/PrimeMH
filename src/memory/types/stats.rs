use std::collections::HashSet;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::Regex;

use crate::memory::{process::D2RInstance, structs::{StatValueStruct, StatsList, Unit}};
use convert_case::{Case, Casing};
use super::skills::{get_skill_class, Skill, SkillTree};

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum Immunity {
    Physical,
    Magic,
    Fire,
    Lightning,
    Cold,
    Poison,
    #[default]
    None,
}

impl Immunity {}


pub fn read_stats(d2rprocess: &D2RInstance, unit: &Unit) -> Vec<Stat> {
    let stat_list: StatsList = d2rprocess.read_mem::<StatsList>(unit.p_stats_list_ex);
    let mut stats: Vec<Stat> = vec![];


    let mut last_stat_ptr = stat_list.stat_unit_ptr;
    while last_stat_ptr != 0 {
        let stat_list_flags = d2rprocess.read_mem::<u64>(last_stat_ptr + 0x1C);
        if 0x40 & stat_list_flags & 0xFFFFDFFF != 0 {
            break
        }   
        last_stat_ptr = d2rprocess.read_mem::<u64>(last_stat_ptr + 0x48);
    }
    
    if last_stat_ptr > 0 {
        let last_stat_list_ptr = d2rprocess.read_mem::<u64>(last_stat_ptr + 0x30);
        let last_stat_list_cnt = d2rprocess.read_mem::<u32>(last_stat_ptr + 0x38);
        
        for n in 0..last_stat_list_cnt {
            if last_stat_list_ptr > 0 {
                let mut stat_struct: StatValueStruct = d2rprocess.read_mem::<StatValueStruct>(last_stat_list_ptr + ((n as u64) * 8));
                if stat_struct.stat == 0 && stat_struct.value == 0 {
                    break;
                }
                let this_stat = Stat::new(&mut stat_struct, false, true);
                stats.push(this_stat);
            }
        }
    }
    

    for n in 0..stat_list.stat_count {
        if stat_list.stat_ptr > 0 {
            let mut stat_struct: StatValueStruct = d2rprocess.read_mem::<StatValueStruct>(stat_list.stat_ptr + ((n as u64) * 8));
            if stat_struct.stat == 0 && stat_struct.value == 0 {
                // log::info!("stat_struct.stat {:?} unit {:?}", stat_struct.stat, &unit);
                break;
            }
            let this_stat = Stat::new(&mut stat_struct, false, false);
            stats.push(this_stat);
        }
    }
    for n in 0..stat_list.stat_ex_count {
        if stat_list.stat_ex_ptr > 0 {
            let mut stat_struct: StatValueStruct = d2rprocess.read_mem::<StatValueStruct>(stat_list.stat_ex_ptr + ((n as u64) * 8));
            if stat_struct.stat == 0 && stat_struct.value == 0 {
                // log::info!("stat_struct_ex.stat {:?} unit {:?}", stat_struct.stat, &unit);
                break;
            }
            let this_stat = Stat::new(&mut stat_struct, true, false);
            stats.push(this_stat);
        }
    }

    stats
}



#[allow(dead_code)]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Stat {
    pub layer: u16,
    pub stat: StatEnum,
    pub value: i16,
    pub value2: i16,
    pub ex: bool,
    pub last: bool,
}

impl Stat {
    pub fn new(raw: &mut StatValueStruct, ex: bool, last: bool) -> Self {
        let stat_name: StatEnum = StatEnum::from_u16(raw.stat).unwrap_or_default();
        match stat_name {
            StatEnum::MaxLife => raw.value >>= 8,
            StatEnum::MaxMana => raw.value >>= 8,
            StatEnum::Life => raw.value >>= 8,
            StatEnum::Mana => raw.value >>= 8,
            StatEnum::MaxStamina => raw.value >>= 8,
            StatEnum::LifePerLevel => raw.value /= 2048,
            StatEnum::ManaPerLevel => raw.value /= 2048,
            StatEnum::DeadlyStrikePerLevel => raw.value = (raw.value as f32 / 0.8) as i16,
            StatEnum::MagicFindPerLevel => raw.value /= 8,
            StatEnum::ExtraGoldPerLevel => raw.value /= 8,
            StatEnum::DamageDemonPerLevel => raw.value /= 8,
            StatEnum::DamageUndeadPerLevel => raw.value /= 8,
            StatEnum::DefensePerLevel => raw.value /= 8,
            StatEnum::MaxDamagePerLevel => raw.value /= 8,
            StatEnum::MaxDamagePercentPerLevel => raw.value /= 8,
            StatEnum::AttackRatingUndeadPerLevel => raw.value /= 2,
            StatEnum::HitCausesMonsterToFlee => raw.value = (raw.value as f32 / 1.28) as i16,
            StatEnum::ColdLength => raw.value /= 25,
            StatEnum::PoisonLength => raw.value /= 25,
            StatEnum::StrengthPerLevel => raw.value /= 8,
            StatEnum::DexterityPerLevel => raw.value /= 8,
            StatEnum::VitalityPerLevel => raw.value /= 8,
            StatEnum::EnergyPerLevel => raw.value /= 8,
            StatEnum::ReplenishDurability => raw.value = if raw.value > 0 { 100 / raw.value } else { 0 },
            _ => (),
        }
        // log::info!("{} {} {:?} {}", raw.layer, raw.stat, stat_name, raw.value);
        Stat {
            layer: raw.layer,
            stat: stat_name,
            value: raw.value,
            value2: raw.value2,
            ex: ex,
            last: last,
        }
    }

    fn has_all_res(stats: &Vec<Stat>) -> bool {
        let mut seen = HashSet::new();
        let all_resist: Vec<&Stat> = stats
            .iter()
            .filter(|stat| matches!(
                stat.stat,
                StatEnum::ColdResist | StatEnum::FireResist | StatEnum::LightningResist | StatEnum::PoisonResist
            ))
            .filter(|stat| seen.insert(stat.stat.clone()))
            .collect();
        
        if all_resist.len() != 4 {
            return false
        }
        if all_resist.iter().skip(1).all(|&x| x.value == all_resist[0].value) {
            return true
        }
        return false
    }

    fn has_all_attrib(stats: &Vec<Stat>) -> bool {
        let mut seen = HashSet::new();
        let all_attrib: Vec<&Stat> = stats
            .iter()
            .filter(|stat| matches!(
                stat.stat,
                StatEnum::Strength | StatEnum::Dexterity | StatEnum::Vitality | StatEnum::Energy
            ))
            .filter(|stat| seen.insert(stat.stat.clone()))
            .collect();
        
        if all_attrib.len() != 4 {
            return false
        }
        if all_attrib.iter().skip(1).all(|&x| x.value == all_attrib[0].value) {
            return true
        }
        return false
    }

    pub fn format_stat(&self, player_level: i16, stats: &Vec<Stat>) -> StatFormat {
        match self.stat {
            StatEnum::Strength => {
                if !Self::has_all_attrib(stats) {
                    StatFormat { stat_str: format!("+{} to Strength", self.value), order: 67}
                } else {
                    StatFormat { stat_str: format!("+{} to All Attributes", self.value), order: 36}
                }
            },
            StatEnum::Energy => {
                if !Self::has_all_attrib(stats) {
                StatFormat { stat_str: format!("+{} to Energy", self.value), order: 61}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::Dexterity => {
                if !Self::has_all_attrib(stats) {
                StatFormat { stat_str: format!("+{} to Dexterity", self.value), order: 65}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::Vitality => {
                if !Self::has_all_attrib(stats) {
                    StatFormat { stat_str: format!("+{} to Vitality", self.value), order: 63}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::MaxLife => StatFormat { stat_str: format!("+{} to Life", self.value), order: 59},
            StatEnum::MaxMana => StatFormat { stat_str: format!("+{} to Mana", self.value), order: 55},
            StatEnum::MaxStamina => StatFormat { stat_str: format!("+{} Maximum Stamina", self.value), order: 51},
            StatEnum::EnhancedDefense => StatFormat { stat_str: format!("+{}% Enhanced Defense", self.value), order: 74},
            StatEnum::EnhancedDamage => StatFormat { stat_str: format!("+{}% Enhanced Damage", self.value), order: 129},
            StatEnum::AttackRating => StatFormat { stat_str: format!("+{} to Attack Rating", self.value), order: 115},
            StatEnum::ChanceToBlock => StatFormat { stat_str: format!("{}% Increased Chance of Blocking", self.value), order: 134},
            StatEnum::MinDamage => StatFormat { stat_str: format!("+{} Minimum Damage", self.value), order: 127},
            StatEnum::ManaRecoveryBonus => StatFormat { stat_str: format!("Regenerate Mana {}%", self.value), order: 52},
            StatEnum::StaminaRecoveryBonus => StatFormat { stat_str: format!("Heal Stamina Plus {}%", self.value), order: 48},
            StatEnum::Defense => {
                if self.ex == false && self.last == false {
                    
                    let last_def = stats.iter().find(|stat| stat.stat == StatEnum::EnhancedDefense && stat.last == true);
                    
                    let ed = match last_def {
                        Some(def) => {
                            def.value
                        },
                        None => 0
                    };

                    let other_def = stats.iter().find(|stat| stat.stat == StatEnum::Defense && stat.ex == true);
                    let extra_def = match other_def {
                        Some(def) => {
                            def.value
                        },
                        None => 0,
                    };

                    if ed > 0 {
                        let ed_calc = (self.value as f32 * ((ed as f32 / 100.0) + 1.0)) as i16;
                        if extra_def - ed_calc > 0 {
                            StatFormat { stat_str: format!("+{} Defense", extra_def - ed_calc), order: 71}
                        } else {
                            StatFormat::default()
                        }
                    } else {
                        if extra_def - self.value > 0 {
                            StatFormat { stat_str: format!("+{} Defense", extra_def - self.value), order: 71}
                        } else {
                            StatFormat::default()
                        }
                    }
                } else if self.ex == true && self.last == false {
                    StatFormat { stat_str: format!("{} Total Defense", self.value), order: 0}
                } else {
                    StatFormat::default()
                }
            }
            StatEnum::DefenseVsMissiles => StatFormat { stat_str: format!("+{} Defense vs. Missile", self.value), order: 69},
            StatEnum::DefenseVsHth => StatFormat { stat_str: format!("+{} Defense vs. Melee", self.value), order: 70},
            StatEnum::NormalDamageReduction => StatFormat { stat_str: format!("Damage Reduced by {}", self.value), order: 22},
            StatEnum::MagicDamageReduction => StatFormat { stat_str: format!("Magic Damage Reduced by {}", self.value), order: 21},
            StatEnum::DamageReduced => StatFormat { stat_str: format!("Damage Reduced by {}%", self.value), order: 22},
            StatEnum::MagicResist => StatFormat { stat_str: format!("Magic Resist +{}%", self.value), order: 41},
            StatEnum::MaxMagicResist => StatFormat { stat_str: format!("+{}% to Maximum Magic Resist", self.value), order: 46},
            StatEnum::FireResist => 
            {
                if !Self::has_all_res(stats) {
                    StatFormat { stat_str: format!("Fire Resist +{}%", self.value), order: 36}
                } else {
                    StatFormat { stat_str: format!("All Resistances +{}", self.value), order: 36}
                }
            },
            StatEnum::MaxFireResist => StatFormat { stat_str: format!("+{}% to Maximum Fire Resist", self.value), order: 42},
            StatEnum::LightningResist => 
            {
                if !Self::has_all_res(stats) {
                    StatFormat { stat_str: format!("Lightning Resist +{}%", self.value), order: 38}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::MaxLightningResist => StatFormat { stat_str: format!("+{}% to Maximum Lightning Resist", self.value), order: 43},
            StatEnum::ColdResist => 
            {
                if !Self::has_all_res(stats) {
                    StatFormat { stat_str: format!("Cold Resist +{}%", self.value), order: 40}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::MaxColdResist => StatFormat { stat_str: format!("+{}% to Maximum Cold Resist", self.value), order: 44},
            StatEnum::PoisonResist => {
                if !Self::has_all_res(stats) {
                    StatFormat { stat_str: format!("Poison Resist +{}%", self.value), order: 34}
                } else {
                    StatFormat::default()
                }
            },
            StatEnum::MaxPoisonResist => StatFormat { stat_str: format!("+{}% to Maximum Poison Resist", self.value), order: 45},
            StatEnum::FireMinDamage => StatFormat { stat_str: format!("+{} to Minimum Fire Damage", self.value), order: 102},
            StatEnum::FireMaxDamage => StatFormat { stat_str: format!("+{} to Maximum Fire Damage", self.value), order: 101},
            StatEnum::LightningMinDamage => StatFormat { stat_str: format!("+{} to Minimum Lightning Damage", self.value), order: 99},
            StatEnum::LightningMaxDamage => StatFormat { stat_str: format!("+{} to Maximum Lightning Damage", self.value), order: 98},
            StatEnum::MagicMinDamage => StatFormat { stat_str: format!("+{} Minimum Magic Damage", self.value), order: 104},
            StatEnum::ColdMinDamage => StatFormat { stat_str: format!("+{} to Minimum Cold Damage", self.value), order: 96},
            StatEnum::ColdMaxDamage => StatFormat { stat_str: format!("+{} to Maximum Cold Damage", self.value), order: 95},
            StatEnum::PoisonMinDamage => StatFormat { stat_str: format!("+{} to Minimum Poison Damage", self.value), order: 92},
            StatEnum::PoisonMaxDamage => StatFormat { stat_str: format!("+{} to Maximum Poison Damage", self.value), order: 91},
            StatEnum::LifeSteal => StatFormat { stat_str: format!("{}% Life stolen per hit", self.value), order: 88},
            StatEnum::ManaSteal => StatFormat { stat_str: format!("{}% Mana stolen per hit", self.value), order: 89},
            StatEnum::ReplenishLife => StatFormat { stat_str: format!("Replenish Life +{}", self.value), order: 56},
            StatEnum::MaxDurabilityPercent => StatFormat { stat_str: format!("Increase Maximum Durability {}%", self.value), order: 3},
            StatEnum::MaxLifePercent => StatFormat { stat_str: format!("Increase Maximum Life {}%", self.value), order: 58},
            StatEnum::MaxManaPercent => StatFormat { stat_str: format!("Increase Maximum Mana {}%", self.value), order: 54},
            StatEnum::AttackerTakesDamage => StatFormat { stat_str: format!("Attacker Takes Damage of {}", self.value), order: 13},
            StatEnum::GoldFind => StatFormat { stat_str: format!("{}% Extra Gold from Monsters", self.value), order: 10},
            StatEnum::MagicFind => StatFormat { stat_str: format!("{}% Better Chance of Getting Magic Items", self.value), order: 8},
            StatEnum::Knockback => StatFormat { stat_str: format!("Knockback"), order: 76},
            StatEnum::AddClassSkills => {
                match self.layer {
                    0 => StatFormat { stat_str: format!("+{} to Amazon Skill Levels", self.value), order: 150},
                    1 => StatFormat { stat_str: format!("+{} to Sorceress Skill Levels", self.value), order: 150},
                    2 => StatFormat { stat_str: format!("+{} to Necromancer Skill Levels", self.value), order: 150},
                    3 => StatFormat { stat_str: format!("+{} to Paladin Skill Levels", self.value), order: 150},
                    4 => StatFormat { stat_str: format!("+{} to Barbarian Skill Levels", self.value), order: 150},
                    5 => StatFormat { stat_str: format!("+{} to Druid Skill Levels", self.value), order: 150},
                    6 => StatFormat { stat_str: format!("+{} to Assassin Skill Levels", self.value), order: 150},
                    _ => StatFormat::default()
                }
            },
            StatEnum::AddExperience => StatFormat { stat_str: format!("+{}% to Experience Gained", self.value), order: 11},
            StatEnum::LifeAfterEachKill => StatFormat { stat_str: format!("+{} Life after each Kill", self.value), order: 16},
            StatEnum::ReducePrices => StatFormat { stat_str: format!("Reduces all Vendor Prices {}%", self.value), order: 8},
            StatEnum::LightRadius => StatFormat { stat_str: format!("{} to Light Radius", self.value), order: 6},
            StatEnum::Requirements => StatFormat { stat_str: format!("Requirements {}%", self.value), order: 0},
            StatEnum::IncreasedAttackSpeed => StatFormat { stat_str: format!("+{}% Increased Attack Speed", self.value), order: 145},
            StatEnum::FasterRunWalk => StatFormat { stat_str: format!("+{}% Faster Run/Walk", self.value), order: 148},
            StatEnum::NonClassSkill => {
                let skill_name = format!("{:?}", Skill::from_u16(self.layer).unwrap_or_default()).to_case(Case::Title);
                StatFormat { stat_str: format!("+{} to {}", self.value, skill_name), order: 81}
            },
            StatEnum::FasterHitRecovery => StatFormat { stat_str: format!("+{}% Faster Hit Recovery", self.value), order: 139},
            StatEnum::FasterBlockRate => StatFormat { stat_str: format!("+{}% Faster Block Rate", self.value), order: 136},
            StatEnum::FasterCastRate => StatFormat { stat_str: format!("+{}% Faster Cast Rate", self.value), order: 142},
            StatEnum::SingleSkill => {
                let skill_name = format!("{:?}", Skill::from_u16(self.layer).unwrap_or_default()).to_case(Case::Title);
                StatFormat { stat_str: format!("+{} to {} ({} only)", self.value, skill_name, get_skill_class(self.layer)), order: 81 }
            },
            StatEnum::SlainMonstersRestInPeace => StatFormat { stat_str: format!("Slain Monsters Rest in Peace"), order: 81},
            StatEnum::PoisonLengthReduced => StatFormat { stat_str: format!("Poison Length Reduced by {}%", self.value), order: 18},
            StatEnum::NormalDamage => StatFormat { stat_str: format!("Damage +{}", self.value), order: 122},
            StatEnum::HitCausesMonsterToFlee => StatFormat { stat_str: format!("Hit Causes Monster to Flee {}%", self.value), order: 79},
            StatEnum::HitBlindsTarget => StatFormat { stat_str: format!("Hit Blinds Target +{}", self.value), order: 80},
            StatEnum::DamageTakenGoesToMana => StatFormat { stat_str: format!("{}% Damage Taken Goes To Mana", self.value), order: 11},
            StatEnum::IgnoreTargetsDefense => StatFormat { stat_str: format!("Ignore Target's Defense"), order: 119},
            StatEnum::TargetDefense => StatFormat { stat_str: format!("-{}% Target Defense", self.value), order: 118},
            StatEnum::PreventMonsterHeal => StatFormat { stat_str: format!("Prevent Monster Heal"), order: 81},
            StatEnum::HalfFreezeDuration => StatFormat { stat_str: format!("Half Freeze Duration"), order: 19},
            StatEnum::AttackRatingPercent => StatFormat { stat_str: format!("{}% Bonus to Attack Rating", self.value), order: 117},
            StatEnum::MonsterDefensePerHit => StatFormat { stat_str: format!("-{} to Monster Defense Per Hit", self.value), order: 75},
            StatEnum::DemonDamagePercent => StatFormat { stat_str: format!("+{}% Damage to Demons", self.value), order: 112},
            StatEnum::UndeadDamagePercent => StatFormat { stat_str: format!("+{}% Damage to Undead", self.value), order: 108},
            StatEnum::DemonAttackRating => StatFormat { stat_str: format!("+{} to Attack Rating against Demons", self.value), order: 110},
            StatEnum::UndeadAttackRating => StatFormat { stat_str: format!("+{} to Attack Rating against Undead", self.value), order: 106},
            StatEnum::FireSkills => StatFormat { stat_str: format!("+{} to Fire Skills", self.value), order: 157},
            StatEnum::AllSkills => StatFormat { stat_str: format!("+{} to All Skills", self.value), order: 158},
            StatEnum::AttackerTakesLightDamage => StatFormat { stat_str: format!("Attacker Takes Lightning Damage of {}", self.value), order: 14},
            StatEnum::FreezesTarget => StatFormat { stat_str: format!("Freezes Target +{}", self.value), order: 78},
            StatEnum::OpenWounds => StatFormat { stat_str: format!("{}% Chance of Open Wounds", self.value), order: 83},
            StatEnum::CrushingBlow => StatFormat { stat_str: format!("{}% Chance of Crushing Blow", self.value), order: 87},
            StatEnum::KickDamage => StatFormat { stat_str: format!("+{} Kick Damage", self.value), order: 121},
            StatEnum::ManaAfterKill => StatFormat { stat_str: format!("+{} to Mana after each Kill", self.value), order: 16},
            StatEnum::HealAfterDemonKill => StatFormat { stat_str: format!("+{} Life after each Demon Kill", self.value), order: 15},
            StatEnum::DeadlyStrike => StatFormat { stat_str: format!("{}% Deadly Strike", self.value), order: 85},
            StatEnum::AbsorbFirePercent => StatFormat { stat_str: format!("+{} Fire Absorb", self.value), order: 23},
            StatEnum::AbsorbFire => StatFormat { stat_str: format!("Fire Absorb {}%", self.value), order: 27},
            StatEnum::AbsorbLightningPercent => StatFormat { stat_str: format!("+{} Lightning Absorb", self.value), order: 24},
            StatEnum::AbsorbLightning => StatFormat { stat_str: format!("Lightning Absorb {}%", self.value), order: 29},
            StatEnum::AbsorbMagicPercent => StatFormat { stat_str: format!("Magic Absorb {}%", self.value), order: 26},
            StatEnum::AbsorbMagic => StatFormat { stat_str: format!("+{} Magic Absorb", self.value), order: 33},
            StatEnum::AbsorbColdPercent => StatFormat { stat_str: format!("+{} Cold Absorb", self.value), order: 25},
            StatEnum::AbsorbCold => StatFormat { stat_str: format!("Cold Absorb {}%", self.value), order: 31},
            StatEnum::SlowsTarget => StatFormat { stat_str: format!("Slows Target by {}%", self.value), order: 77},
            StatEnum::Aura => {
                let skill_name = format!("{:?}", Skill::from_u16(self.layer).unwrap_or_default()).to_case(Case::Title);
                StatFormat { stat_str: format!("Level {} {} Aura When Equipped", self.value, skill_name), order: 159 }
            },
            StatEnum::CannotBeFrozen => StatFormat { stat_str: format!("Cannot Be Frozen"), order: 20},
            StatEnum::SlowerStaminaDrain => StatFormat { stat_str: format!("{}% Slower Stamina Drain", self.value), order: 49},
            StatEnum::Reanimate => StatFormat { stat_str: format!("Reanimate As: [Returned]"), order: 17},
            StatEnum::Pierce => StatFormat { stat_str: format!("Piercing Attack"), order: 132},
            StatEnum::MagicArrow => StatFormat { stat_str: format!("Fires Magic Arrows"), order: 131},
            StatEnum::ExplosiveArrow => StatFormat { stat_str: format!("Fires Explosive Arrows or Bolts"), order: 133},
            StatEnum::AttackVsMonType => StatFormat { stat_str: format!("{}% to Attack Rating versus [Monster Type]", self.value), order: 108},
            StatEnum::DamageVsMonType => StatFormat { stat_str: format!("{}% to Damage versus [Monster Type]", self.value), order: 106},
            StatEnum::Unused187 => StatFormat { stat_str: format!("Monster Cold Immunity is Sundered"), order: 100},
            StatEnum::AddSkillTab => {
                let skill_tab = format!("{:?}", SkillTree::from_u16(self.layer).unwrap_or(SkillTree::AmazonBowAndCrossbow)).to_case(Case::Title);
                StatFormat { stat_str: format!("+{} to {}", self.value, skill_tab), order: 151}
            },
            StatEnum::Unused189 => StatFormat { stat_str: format!("Monster Fire Immunity is Sundered"), order: 100},
            StatEnum::Unused190 => StatFormat { stat_str: format!("Monster Lightning Immunity is Sundered"), order: 100},
            StatEnum::Unused191 => StatFormat { stat_str: format!("Monster Poison Immunity is Sundered"), order: 100},
            StatEnum::Unused192 => StatFormat { stat_str: format!("Monster Physical Immunity is Sundered"), order: 100},
            StatEnum::Unused193 => StatFormat { stat_str: format!("Monster Magic Immunity is Sundered"), order: 100},
            StatEnum::SkillOnAttack => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} on attack", self.value, level, skill_name), order: 160}
            },
            StatEnum::SkillOnKill => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} when you Kill an Enemy", self.value, level, skill_name), order: 160}
            },
            StatEnum::SkillOnDeath => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} when you Die", self.value, level, skill_name), order: 160}
            },
            StatEnum::SkillOnHit => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} on striking", self.value, level, skill_name), order: 160}
            },
            StatEnum::SkillOnLevelUp => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} when you Level-Up", self.value, level, skill_name), order: 160}
            },
            StatEnum::SkillOnGetHit => {
                let (skill_name, level, _, _) = self.skill_chance();
                StatFormat { stat_str: format!("{}% Chance to cast level {} {} when struck", self.value, level, skill_name), order: 160}
            },
            StatEnum::ItemChargedSkill => {
                let (skill_name, level, max_charges, charges) = self.skill_chance();
                StatFormat { stat_str: format!("Level {} {} ({}/{} Charges)", level, skill_name, charges, max_charges), order: 1}
            },
            StatEnum::Unused205 => StatFormat { stat_str: format!("+{} to not Consume Quantity", self.value), order: 4},
            StatEnum::DefensePerLevel => StatFormat { stat_str: format!("+{} Defense (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 72},
            StatEnum::ArmorPercentPerLevel => StatFormat { stat_str: format!("+{}% Enhanced Defense (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 73},
            StatEnum::LifePerLevel => {
                // let max_life = ((self.value2 as u16) << 8) ^ (self.value as u8) as u16;
                // log::info!("{} value {} value2 {} max_life {}", ((self.value2 as u16) << 8) ^ (self.value as u8) as u16, self.value, self.value2, max_life);
                StatFormat { stat_str: format!("+{} to Life (Based on Character Level)", (player_level as f32 * 1.5).round()), order: 57}
            },
            StatEnum::ManaPerLevel => StatFormat { stat_str: format!("+{} to Mana (Based on Character Level)", (player_level as f32 * 1.5).round()), order: 53},
            StatEnum::MaxDamagePerLevel => StatFormat { stat_str: format!("+{} to Maximum Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 125},
            StatEnum::MaxDamagePercentPerLevel => StatFormat { stat_str: format!("+{}% Enhanced Maximum Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 128},
            StatEnum::StrengthPerLevel => StatFormat { stat_str: format!("+{} to Strength (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 66},
            StatEnum::DexterityPerLevel => StatFormat { stat_str: format!("+{} to Dexterity (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 64},
            StatEnum::EnergyPerLevel => StatFormat { stat_str: format!("+{} to Energy (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 60},
            StatEnum::VitalityPerLevel => StatFormat { stat_str: format!("+{} to Vitality (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 62},
            StatEnum::AttackRatingPerLevel => StatFormat { stat_str: format!("+{} to Attack Rating (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 114},
            StatEnum::AttackRatingPercentPerLevel => StatFormat { stat_str: format!("{}% Bonus to Attack Rating (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 116},
            StatEnum::ColdDamageMaxPerLevel => StatFormat { stat_str: format!("+{} to Maximum Cold Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 94},
            StatEnum::FireDamageMaxPerLevel => StatFormat { stat_str: format!("+{} to Maximum Fire Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 100},
            StatEnum::LightningDamageMaxPerLevel => StatFormat { stat_str: format!("+{} to Maximum Lightning Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 97},
            StatEnum::PoisonDamageMaxPerLevel => StatFormat { stat_str: format!("+{} to Maximum Poison Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 90},
            StatEnum::ResistColdPerLevel => StatFormat { stat_str: format!("Cold Resist +{}% (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 39},
            StatEnum::ResistFirePerLevel => StatFormat { stat_str: format!("Fire Resist +{}% (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 35},
            StatEnum::ResistLightningPerLevel => StatFormat { stat_str: format!("Lightning Resist +{}% (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 37},
            StatEnum::ResistPoisonPerLevel => StatFormat { stat_str: format!("Poison Resist +{}% (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 33},
            StatEnum::ThornsPerLevel => StatFormat { stat_str: format!("Attacker Takes Damage of {} (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 12},
            StatEnum::ExtraGoldPerLevel => StatFormat { stat_str: format!("{}% Extra Gold from Monsters (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 9},
            StatEnum::MagicFindPerLevel => StatFormat { stat_str: format!("{}% Better Chance of Getting Magic Items (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 7},
            StatEnum::RegenStaminaPerLevel => StatFormat { stat_str: format!("Heal Stamina Plus {}% (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 47},
            StatEnum::StaminaPerLevel => StatFormat { stat_str: format!("+{} Maximum Stamina (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 50},
            StatEnum::DamageDemonPerLevel => StatFormat { stat_str: format!("+{}% Damage to Demons (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 111},
            StatEnum::DamageUndeadPerLevel => StatFormat { stat_str: format!("+{}% Damage to Undead (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 107},
            StatEnum::AttackRatingDemonPerLevel => StatFormat { stat_str: format!("+{} to Attack Rating against Demons (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 109},
            StatEnum::AttackRatingUndeadPerLevel => StatFormat { stat_str: format!("+{} to Attack Rating against Undead (Based on Character Level)", (self.value / 2) * player_level), order: 105},
            StatEnum::CrushingBlowPerLevel => StatFormat { stat_str: format!("{}% Chance of Crushing Blow (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 86},
            StatEnum::OpenWoundsPerLevel => StatFormat { stat_str: format!("{}% Chance of Open Wounds (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 82},
            StatEnum::KickDamagePerLevel => StatFormat { stat_str: format!("+{} Kick Damage (Based on Character Level)", (self.value as f32 / 8.0).round() as i16 * player_level), order: 120},
            StatEnum::DeadlyStrikePerLevel => StatFormat { stat_str: format!("{}% Deadly Strike (Based on Character Level)", (self.value as f32 / 0.8).round()), order: 84},
            StatEnum::ReplenishDurability => StatFormat { stat_str: format!("Repairs 1 durability in {} seconds", self.value), order: 1},
            StatEnum::ReplenishQuantity => StatFormat { stat_str: format!("Replenishes quantity"), order: 2},
            StatEnum::ExtraStack => StatFormat { stat_str: format!("Increased Stack Size"), order: 4},
            StatEnum::FireSkillDamage => StatFormat { stat_str: format!("+{}% to Fire Skill Damage", self.value), order: 88},
            StatEnum::LightningSkillDamage => StatFormat { stat_str: format!("+{}% to Lightning Skill Damage", self.value), order: 88},
            StatEnum::ColdSkillDamage => StatFormat { stat_str: format!("+{}% to Cold Skill Damage", self.value), order: 88},
            StatEnum::PoisonSkillDamage => StatFormat { stat_str: format!("+{}% to Poison Skill Damage", self.value), order: 88},
            StatEnum::EnemyFireResist => StatFormat { stat_str: format!("-{}% to Enemy Fire Resistance", self.value), order: 88},
            StatEnum::EnemyLightningResist => StatFormat { stat_str: format!("-{}% to Enemy Lightning Resistance", self.value), order: 88},
            StatEnum::EnemyColdResist => StatFormat { stat_str: format!("-{}% to Enemy Cold Resistance", self.value), order: 88},
            StatEnum::EnemyPoisonResist => StatFormat { stat_str: format!("-{}% to Enemy Poison Resistance", self.value), order: 88},
            _ => StatFormat::default()
        }
    }

    fn skill_chance(&self) -> (String, u16, i16, i16) {
        let skill_id = self.layer >> 6;
        let level = self.layer % (1 << 6);
        let max_charges = self.value >> 8;
        let charges = self.value % (1 << 8);
        let skill_name = format!("{:?}", Skill::from_u16(skill_id).unwrap_or_default()).to_case(Case::Title);
        (skill_name, level, max_charges, charges)
    }
    
}


#[derive(Default, Hash, PartialEq, Eq)]
pub struct StatFormat {
    stat_str: String,
    order: u32
}

pub fn format_stat_list(stats: &Vec<Stat>, player_level: i16) -> Vec<String> {
    
    let formatted_hs: HashSet<StatFormat> = stats.iter().map(|stat| stat.format_stat(player_level, stats)).collect();
    let mut formatted: Vec<StatFormat> = formatted_hs.into_iter().collect();

    formatted.sort_by(|a, b| {
        if a.order == b.order {
            a.stat_str.cmp(&b.stat_str)
        } else {
            a.order.cmp(&b.order)
        }
    });
    formatted.sort_by_key(|p| p.order);
    formatted.reverse();
    let mut stat_string = vec![];
    formatted.iter().for_each(|stat| stat_string.push(stat.stat_str.clone()));
    stat_string
}

pub fn format_affixes(stat_string: String, ethereal: bool, num_sockets: u8) -> String {
    let pattern = Regex::new(r"\+(\d+) to Minimum (.*?)Damage\s\+(\d+) to Maximum.*?Damage").unwrap();
    let replacement = "Adds $1-$3 to $2 Damage";
    let mut result = pattern.replace_all(&stat_string.as_str(), replacement).clone().into_owned();
    
    // change any 1-1 or 2-2 to +1 or +2
    let pattern = Regex::new(r"Adds (\d+)-(\d+) to").unwrap();
    for captures in pattern.captures_iter(result.clone().as_str()) {
        let group1 = captures.get(1).map_or("", |m| m.as_str());
        let group2 = captures.get(2).map_or("", |m| m.as_str());
        if group1 == group2 {
            let replacement_pattern = format!("Adds {}-({}) to", group1, group2); 
            let pattern = Regex::new(replacement_pattern.as_str()).unwrap();
            let replacement: &str = "+$1 to";
            result = pattern.replace_all(&result.as_str(), replacement).clone().into_owned();
        }
    }
    // get rid of double spaces
    let pattern = Regex::new(r"  +").unwrap();
    result = pattern.replace_all(&result, " ").clone().into_owned();

    // add number of sockets
    if num_sockets > 0 {
        result = format!("{}Socketed ({})", result, num_sockets);
    }
    // add ethereal
    if ethereal {
        result = format!("{}Ethereal", result);
    }
    return result
}


#[allow(dead_code)]
#[derive(FromPrimitive, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum StatEnum {
    Strength,
    Energy,
    Dexterity,
    Vitality,
    StatPoints,
    SkillPoints,
    Life,
    MaxLife,
    Mana,
    MaxMana,
    Stamina,
    MaxStamina,
    Level,
    Experience,
    Gold,
    StashGold,
    EnhancedDefense,
    EnhancedDamageMax,
    EnhancedDamage,
    AttackRating,
    ChanceToBlock,
    MinDamage,
    MaxDamage,
    TwoHandedMinDamage,
    TwoHandedMaxDamage,
    DamagePercent,
    ManaRecovery,
    ManaRecoveryBonus,
    StaminaRecoveryBonus,
    LastExp,
    NextExp,
    Defense,
    DefenseVsMissiles,
    DefenseVsHth,
    NormalDamageReduction,
    MagicDamageReduction,
    DamageReduced,
    MagicResist,
    MaxMagicResist,
    FireResist,
    MaxFireResist,
    LightningResist,
    MaxLightningResist,
    ColdResist,
    MaxColdResist,
    PoisonResist,
    MaxPoisonResist,
    DamageAura,
    FireMinDamage,
    FireMaxDamage,
    LightningMinDamage,
    LightningMaxDamage,
    MagicMinDamage,
    MagicMaxDamage,
    ColdMinDamage,
    ColdMaxDamage,
    ColdLength,
    PoisonMinDamage,
    PoisonMaxDamage,
    PoisonLength,
    LifeSteal,
    LifeStealMax,
    ManaSteal,
    ManaStealMax,
    StaminaDrainMinDamage,
    StaminaDrainMaxDamage,
    StunLength,
    VelocityPercent,
    AttackRate,
    OtherAnimRate,
    Quantity,
    Value,
    Durability,
    MaxDurability,
    ReplenishLife,
    MaxDurabilityPercent,
    MaxLifePercent,
    MaxManaPercent,
    AttackerTakesDamage,
    GoldFind,
    MagicFind,
    Knockback,
    TimeDuration,
    AddClassSkills,
    Unused84,
    AddExperience,
    LifeAfterEachKill,
    ReducePrices,
    DoubleHerbDuration,
    LightRadius,
    LightColor,
    Requirements,
    LevelRequire,
    IncreasedAttackSpeed,
    LevelRequirePercent,
    LastBlockFrame,
    FasterRunWalk,
    NonClassSkill,
    State,
    FasterHitRecovery,
    PlayerCount,
    PoisonOverrideLength,
    FasterBlockRate,
    BypassUndead,
    BypassDemons,
    FasterCastRate,
    BypassBeasts,
    SingleSkill,
    SlainMonstersRestInPeace,
    CurseResistance,
    PoisonLengthReduced,
    NormalDamage,
    HitCausesMonsterToFlee,
    HitBlindsTarget,
    DamageTakenGoesToMana,
    IgnoreTargetsDefense,
    TargetDefense,
    PreventMonsterHeal,
    HalfFreezeDuration,
    AttackRatingPercent,
    MonsterDefensePerHit,
    DemonDamagePercent,
    UndeadDamagePercent,
    DemonAttackRating,
    UndeadAttackRating,
    Throwable,
    FireSkills,
    AllSkills,
    AttackerTakesLightDamage,
    IronMaidenLevel,
    LifeTapLevel,
    ThornsPercent,
    BoneArmor,
    BoneArmorMax,
    FreezesTarget,
    OpenWounds,
    CrushingBlow,
    KickDamage,
    ManaAfterKill,
    HealAfterDemonKill,
    ExtraBlood,
    DeadlyStrike,
    AbsorbFirePercent,
    AbsorbFire,
    AbsorbLightningPercent,
    AbsorbLightning,
    AbsorbMagicPercent,
    AbsorbMagic,
    AbsorbColdPercent,
    AbsorbCold,
    SlowsTarget,
    Aura,
    Indestructible,
    CannotBeFrozen,
    SlowerStaminaDrain,
    Reanimate,
    Pierce,
    MagicArrow,
    ExplosiveArrow,
    ThrowMinDamage,
    ThrowMaxDamage,
    SkillHandofAthena,
    SkillStaminaPercent,
    SkillPassiveStaminaPercent,
    SkillConcentration,
    SkillEnchant,
    SkillPierce,
    SkillConviction,
    SkillChillingArmor,
    SkillFrenzy,
    SkillDecrepify,
    SkillArmorPercent,
    Alignment,
    Target0,
    Target1,
    GoldLost,
    ConverisonLevel,
    ConverisonMaxHP,
    UnitDooverlay,
    AttackVsMonType,
    DamageVsMonType,
    Fade,
    ArmorOverridePercent,
    Unused183,
    Unused184,
    Unused185,
    Unused186,
    Unused187,
    AddSkillTab,
    Unused189,
    Unused190,
    Unused191,
    Unused192,
    Unused193,
    NumSockets,
    SkillOnAttack,
    SkillOnKill,
    SkillOnDeath,
    SkillOnHit,
    SkillOnLevelUp,
    Unused200,
    SkillOnGetHit,
    Unused202,
    Unused203,
    ItemChargedSkill,
    Unused205,
    Unused206,
    Unused207,
    Unused208,
    Unused209,
    Unused210,
    Unused211,
    Unused212,
    Unused213,
    DefensePerLevel,
    ArmorPercentPerLevel,
    LifePerLevel,
    ManaPerLevel,
    MaxDamagePerLevel,
    MaxDamagePercentPerLevel,
    StrengthPerLevel,
    DexterityPerLevel,
    EnergyPerLevel,
    VitalityPerLevel,
    AttackRatingPerLevel,
    AttackRatingPercentPerLevel,
    ColdDamageMaxPerLevel,
    FireDamageMaxPerLevel,
    LightningDamageMaxPerLevel,
    PoisonDamageMaxPerLevel,
    ResistColdPerLevel,
    ResistFirePerLevel,
    ResistLightningPerLevel,
    ResistPoisonPerLevel,
    AbsorbColdPerLevel,
    AbsorbFirePerLevel,
    AbsorbLightningPerLevel,
    AbsorbPoisonPerLevel,
    ThornsPerLevel,
    ExtraGoldPerLevel,
    MagicFindPerLevel,
    RegenStaminaPerLevel,
    StaminaPerLevel,
    DamageDemonPerLevel,
    DamageUndeadPerLevel,
    AttackRatingDemonPerLevel,
    AttackRatingUndeadPerLevel,
    CrushingBlowPerLevel,
    OpenWoundsPerLevel,
    KickDamagePerLevel,
    DeadlyStrikePerLevel,
    FindGemsPerLevel,
    ReplenishDurability,
    ReplenishQuantity,
    ExtraStack,
    FindItem,
    SlashDamage,
    SlashDamagePercent,
    CrushDamage,
    CrushDamagePercent,
    ThrustDamage,
    ThrustDamagePercent,
    AbsorbSlash,
    AbsorbCrush,
    AbsorbThrust,
    AbsorbSlashPercent,
    AbsorbCrushPercent,
    AbsorbThrustPercent,
    ArmorByTime,
    ArmorPercentByTime,
    LifeByTime,
    ManaByTime,
    MaxDamageByTime,
    MaxDamagePercentByTime,
    StrengthByTime,
    DexterityByTime,
    EnergyByTime,
    VitalityByTime,
    AttackRatingByTime,
    AttackRatingPercentByTime,
    ColdDamageMaxByTime,
    FireDamageMaxByTime,
    LightningDamageMaxByTime,
    PoisonDamageMaxByTime,
    ResistColdByTime,
    ResistFireByTime,
    ResistLightningByTime,
    ResistPoisonByTime,
    AbsorbColdByTime,
    AbsorbFireByTime,
    AbsorbLightningByTime,
    AbsorbPoisonByTime,
    FindGoldByTime,
    MagicFindByTime,
    RegenStaminaByTime,
    StaminaByTime,
    DamageDemonByTime,
    DamageUndeadByTime,
    AttackRatingDemonByTime,
    AttackRatingUndeadByTime,
    CrushingBlowByTime,
    OpenWoundsByTime,
    KickDamageByTime,
    DeadlyStrikeByTime,
    FindGemsByTime,
    PierceCold,
    PierceFire,
    PierceLightning,
    PiercePoison,
    DamageVsMonster,
    DamagePercentVsMonster,
    AttackRatingVsMonster,
    AttackRatingPercentVsMonster,
    AcVsMonster,
    AcPercentVsMonster,
    FireLength,
    BurningMin,
    BurningMax,
    ProgressiveDamage,
    ProgressiveSteal,
    ProgressiveOther,
    ProgressiveFire,
    ProgressiveCold,
    ProgressiveLightning,
    ExtraCharges,
    ProgressiveAttackRating,
    PoisonCount,
    DamageFrameRate,
    PierceIdx,
    FireSkillDamage,
    LightningSkillDamage,
    ColdSkillDamage,
    PoisonSkillDamage,
    EnemyFireResist,
    EnemyLightningResist,
    EnemyColdResist,
    EnemyPoisonResist,
    PassiveCriticalStrike,
    PassiveDodge,
    PassiveAvoid,
    PassiveEvade,
    PassiveWarmth,
    PassiveMasteryMeleeAttackRating,
    PassiveMasteryMeleeDamage,
    PassiveMasteryMeleeCritical,
    PassiveMasteryThrowAttackRating,
    PassiveMasteryThrowDamage,
    PassiveMasteryThrowCritical,
    PassiveWeaponBlock,
    SummonResist,
    ModifierListSkill,
    ModifierListLevel,
    LastSentHPPercent,
    SourceUnitType,
    SourceUnitID,
    ShortParam1,
    QuestItemDifficulty,
    PassiveMagicMastery,
    PassiveMagicPierce,
    #[default]
    Unknown,
}


#[cfg(test)]
mod tests {
    use crate::logger::configure_logging;

    use super::format_affixes;
    #[test]
    fn test_format_affixes() {
        configure_logging();
        let affixes = vec!["", "+1 to Minimum Fire Damage", "+2 to Maximum Fire Damage", "+1 to Minimum Damage", "+1 to Maximum Damage"];
        log::info!("{:?}", affixes);
        format_affixes(affixes.join("\n"), true, 2);
    }
}