use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::memory::{
    decrypt::decrypt_seed,
    process::D2RInstance,
    structs::{Act, ActMisc, Level, Path, Room, Room2},
};

use super::player::PlayerUnit;

#[derive(Debug, Default)]
pub struct SeedValues {
    pub map_seed: u32,
    pub difficulty: Difficulty,
    pub level: u32,
    pub dw_init_seed_hash: u64,
}

impl SeedValues {
    pub fn new(d2rprocess: &D2RInstance, player: &PlayerUnit) -> Self {
        let player_act: Act = d2rprocess.read_mem::<Act>(player.raw.p_act);
        let player_act_misc: ActMisc = d2rprocess.read_mem::<ActMisc>(player_act.p_act_misc);
        let map_seed = decrypt_seed(player_act_misc.dw_init_seed_hash, player_act_misc.dw_end_seed_hash);
        let difficulty: Difficulty = Difficulty::from_u32(player_act_misc.game_difficulty).unwrap_or_default();
        let player_path: Path = d2rprocess.read_mem::<Path>(player.raw.p_path);
        let player_room: Room = d2rprocess.read_mem::<Room>(player_path.p_room);
        let player_room2: Room2 = d2rprocess.read_mem::<Room2>(player_room.p_room_2);
        let level: Level = d2rprocess.read_mem::<Level>(player_room2.p_level);
        
        SeedValues {
            map_seed,
            difficulty,
            level: level.level_id,
            dw_init_seed_hash: player_act_misc.dw_init_seed_hash,
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Default)]
pub enum Difficulty {
    Normal,
    Nightmare,
    Hell,
    #[default]
    Invalid,
}
