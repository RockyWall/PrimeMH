use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::memory::{
    process::D2RInstance,
    structs::{MissileData, Unit},
};

use super::get_position;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MissileUnit {
    pub unit_id: u32,
    pub txt_file_no: Missile,
    pub pos_x: f32,
    pub pos_y: f32,
    pub p_path: u64,
    pub missile_type: MissileType,
    pub missile_color: u32,
    pub missile_data: MissileData,
    pub collided: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub enum MissileType {
    Fire,
    Ice,
    Lightning,
    Poison,
    Magic,
    Physical,
    Sfx,
    FxTrigger,
    #[default]
    Dummy,
}

impl MissileUnit {
    pub fn new(d2rprocess: &D2RInstance, unit: Unit, player_pos_x: f32, player_pos_y: f32, unit_id: u32) -> Self {
        let txt_file_no: Missile = Missile::from_u32(unit.txt_file_no).unwrap_or(Missile::Unknown);
        let (pos_x, pos_y) = get_position(d2rprocess, unit);
        let missile_type = get_missile_type(&txt_file_no);
        let missile_color = get_missile_color(&txt_file_no);
        let missile_data: MissileData = d2rprocess.read_mem::<MissileData>(unit.p_unit_data);
        let mut collided = false;
        if (txt_file_no == Missile::Battleorders || txt_file_no == Missile::Battlecommand) && missile_data.base_skill_level < 100 && missile_data.base_skill_level > 0 {
            collided = (unit_id == missile_data.dw_owner_id) || ((pos_x - player_pos_x).abs() < 2.5 && (pos_y - player_pos_y).abs() < 2.5);
        }
        
        MissileUnit {
            unit_id: unit.unit_id,
            txt_file_no,
            pos_x,
            pos_y,
            p_path: unit.p_path,
            missile_type,
            missile_color,
            missile_data,
            collided
        }
    }
}


#[derive(FromPrimitive, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Missile {
    Arrow,
    Javelin,
    Bighead1,
    Bighead2,
    Bighead3,
    Bighead4,
    Bighead5,
    Spike1,
    Spike2,
    Spike3,
    Spike4,
    Spike5,
    Firearrow,
    CrArrow1,
    CrArrow2,
    CrArrow3,
    CrArrow4,
    CrArrow5,
    Blood1,
    Blood2,
    Bigblood1,
    Bigblood2,
    Shafire1,
    Shafire2,
    Shafire3,
    Shafire4,
    Shafire5,
    Magicarrow,
    Icearrow,
    Fireexplode,
    Iceexplode,
    Bolt,
    Andarielspray,
    Bigheadexp,
    Shamanexp,
    Throwaxe,
    Throwknife,
    Glaive,
    Poisonjav,
    Poisonjavcloud,
    Coldarrow,
    Explodingarrow,
    Explodingarrowexp,
    Plaguejavelin,
    Oilpotion,
    Explosivepotion,
    Fulminatingpotion,
    Rancidgasepotion,
    Chokinggaspoition,
    Stranglinggaspotion,
    Notused50,
    Explosivepotionexp,
    Explosivepotiondebris1,
    Explosivepotiondebris2,
    Explosivepotiondebris3,
    Holybolt,
    Chargedbolt,
    Sanctuarybolt,
    Firebolt,
    Icebolt,
    Infernoflame1,
    Infernoflame2,
    Fireball,
    Mummy1,
    Mummy2,
    Mummy3,
    Mummy4,
    Blaze,
    Firewallmaker,
    Firewall,
    Goospit1,
    Goospit2,
    Goospit3,
    Goospit4,
    Goospit5,
    Goosplat,
    SandPile,
    Unholybolt1,
    Unholybolt2,
    Unholybolt3,
    Unholybolt4,
    Sanctuarycenter,
    Fireexplosion,
    Stuckarrow,
    Footprint,
    Immolationarrow,
    Guidedarrow,
    Freezingarrow,
    Freezingarrowexp1,
    Freezingarrowexp2,
    Nova,
    Iceblast,
    Blessedhammer,
    Chainlightning,
    Fistofares,
    Chillblood,
    Glacialspike,
    Teleport,
    Lightningbolt,
    Lightninghit,
    Meteor,
    Meteorcenter,
    Meteortail,
    Meteorexplode,
    Firesmall,
    Firemedium,
    Monblizcenter,
    Monbliz1,
    Monbliz2,
    Monbliz3,
    Monbliz4,
    Monblizexplode1,
    Monblizexplode2,
    Monblizexplode3,
    Teeth,
    Corpseexplosion,
    Poisoncorpseexplosion,
    Monstercorpseexplode,
    Poisonnova,
    Frostnova,
    Rogue1,
    Rogue2,
    Rogue3,
    BatLightningBolt,
    BatLightningTrail,
    Skmage1,
    Skmage2,
    Skmage3,
    Skmage4,
    Vampirefireball,
    Vampirefirewallmaker,
    Vampirefirewall,
    Vampiremeteor,
    Vampiremeteorcenter,
    Vampiremeteorexp,
    Raven1,
    Raven2,
    Amphibiangoo1,
    Amphibiangoo2,
    Tentaclegoo,
    Amphibianexplode,
    Poisonpuff,
    Curseeffectred,
    Spidergoolay,
    Fetishinferno1,
    Fetishinferno2,
    Spidergoo,
    Cursecast,
    Howl,
    Shout,
    Dust,
    Redlightmissile,
    Greenlightmissile,
    Bluelightmissile,
    Whitelightmissile,
    Corpsepoisoncloud,
    Chillbloodcloud,
    Chillbloodpuff,
    Blizzardcenter,
    Blizzard1,
    Blizzard2,
    Blizzard3,
    Blizzard4,
    Blizzardexplode1,
    Blizzardexplode2,
    Blizzardexplode3,
    Thunderstorm1,
    Thunderstorm2,
    Thunderstorm3,
    Thunderstorm4,
    Monsterlight,
    Poisonball,
    Diablight,
    Redemption,
    Redemptionfail,
    Handofgod,
    Diabfire,
    Fingermagespider,
    ElectricThrowaxe,
    Diabwallmaker,
    Diabwall,
    Curseamplifydamage,
    Cursedimvision,
    Curseweaken,
    Curseironmaiden,
    Curseterror,
    Curseattract,
    Cursereversevampire,
    Curseconfuse,
    Cursedecrepify,
    Curselowerresist,
    Cursecenter,
    Bonespear,
    Bonespirit,
    Coldunique,
    Lightunique,
    Skbowarrow1,
    Skbowarrow2,
    Skbowarrow3,
    Skbowarrow4,
    Skbowarrow5,
    Nova1,
    Nova2,
    Andypoisonbolt,
    Teethexplode,
    Lightningjavelin,
    Lightningfury,
    Bonewallmaker,
    Necromage1,
    Necromage2,
    Necromage3,
    Necromage4,
    Sparkle,
    Multipleshotarrow,
    Multipleshotbolt,
    Chargedstrikebolt,
    Bonespearexplode,
    Poisonexplosioncloud,
    Bonecast,
    Battlecry,
    Primepoisoncloud,
    Plaguejavcloud,
    Rancidgascloud,
    Chokinggascloud,
    Stranglinggascloud,
    Buglightning,
    Pantherjav1,
    Pantherjav2,
    Pantherjav3,
    Pantherjav4,
    Immolationfire,
    Furylightning,
    Lightningstrike,
    Fistoftheheavensdelay,
    Fistoftheheavensbolt,
    Warcry,
    Battlecommand,
    Battleorders,
    Pantherpotorange,
    Pantherpotgreen,
    Meteorfire,
    TrapSpikeRight,
    TrapSpikeLeft,
    TrapCursedSkullRight,
    TrapCursedSkullLeft,
    TrapPoisonBallRight,
    TrapPoisonBallLeft,
    Hydra,
    Bonespeartrail,
    Grimwardsmallstart,
    Grimwardsmall,
    Grimwardsmallstop,
    Grimwardmediumstart,
    Grimwardmedium,
    Grimwardmediumstop,
    Grimwardlargestart,
    Grimwardlarge,
    Grimwardlargestop,
    Zakarumlight,
    Grimwardscare,
    Frozenorb,
    Frozenorbbolt,
    Frozenorbnova,
    Frozenorbexplode,
    Chillingarmorbolt,
    Fireexplosion2,
    Blowgun,
    Chainlightning2,
    Revivesmall,
    Revivemedium,
    Revivelarge,
    Monglacialspike,
    Icebreaksmall,
    Icebreakmedium,
    Icebreaklarge,
    Icebreaksmoke,
    Mephisto,
    Firehead,
    Whilrwind,
    Arcanelightningbolt,
    Frogfire,
    Frogcold,
    Frogpois,
    Desertfireball,
    Brdeathcontrol,
    Brdeathlightningbolt,
    Brdeathlightninghit,
    Denofevillight,
    Cairnstones,
    Cairnstonessky,
    Cairnstonesground,
    Towermist,
    Towermisttrail,
    Brdeathsmokes1,
    Brdeathsmokenu,
    Brdeathsmokedt,
    Brdeathspirits1,
    Brdeathspiritnu,
    Brdeathspiritdt,
    Mephistodeathcontrol,
    Mephistofirewallmaker,
    Mephistofirewall,
    Mephistoflyingrocksbig,
    Mephistoexplosionbig,
    Mephistoflyingrockssmall,
    Mephistoexplosionsmall,
    Mephistodonotdraw,
    Andycontrol0,
    Andyfirewallmaker,
    Andyfirewall,
    Andycolumnfirebase,
    Andycolumnfire,
    Andyfallingdebris1,
    Andyfallingdebris2,
    Andyfallingdebris3,
    Andydebrisexplosion1,
    Andydebrisexplosion2,
    Andydebrisexplosion3,
    Andydebrisexplosion4,
    Andydebrisexplosion5,
    Willowisplightningbolt,
    Queenpoisoncloud,
    DirtPile,
    Undeadmissile1,
    Undeadmissile2,
    Undeadmissile3,
    Undeadmissile4,
    Bonespiritexplode,
    Dopplezonexplode,
    Monbonespirit,
    Towermistfade,
    Countessfirewall,
    Towerchestspawner,
    Hellmeteorlaunch1,
    Hellmeteorlaunch2,
    Hellmeteorup,
    Hellmeteordown,
    Hellmeteorball,
    Horadricstaff,
    Horadriclightning,
    Horadriclight,
    Regurgitatorcorpse,
    Regurgitatorcorpseexpl,
    Highpriestlightning,
    Icebreaksmallmelt,
    Icebreaklargemelt,
    Leapknockback,
    Radamentdeath,
    Radamenthandofgod,
    Radamentholybolt,
    Taintedsuncontrol,
    Taintedsunflash,
    Taintedsunball,
    Queendeathcenter,
    Queendeathglob,
    Queendeathsplat1,
    Queendeathsplat2,
    Healingbolt,
    Mephistoholedelay,
    Mephistoholebirth,
    Mephistoholeneutral,
    Mephistoholedeath,
    Mephistoholedead,
    Durieldeathcontrol,
    Durieldeathrock,
    Durieldeathdebris,
    Durieldeathsmoke,
    Mephistoexplosion,
    Orbmist,
    Orbmisttrail,
    Orbmistfade,
    Pilum,
    DiabloAppears,
    Hfcontrol,
    Hffragment1,
    Hffragment2,
    Hffragment3,
    Hfspirit1,
    Hfreserved3,
    IzualControl,
    IzualMistLoop,
    IzualMistFade,
    IzualLightning,
    IzualLightningTrail,
    CairnStonesBolt,
    BombInAir,
    BombOnGround,
    BombExplosion,
    ShockFieldInAir,
    ShockFieldOnGround,
    Throwingstar,
    Acidspray,
    BladeCreeper,
    Distraction,
    DistractionFog,
    DistractionPuff,
    DistractionStart,
    DistractionEnd,
    Impinfernoflame1,
    Impinfernoflame2,
    Baallightningbolt,
    Baallightningtrail,
    Baallightningbolt2,
    Baallightningtrail2,
    Impfireball,
    Impfireballexplode,
    CatapultchargedballOn,
    Catapultchargedball,
    Catapultchargedballbolt,
    ImpSpawnMonsters,
    CatapultSpikeBallOn,
    CatapultSpikeBall,
    CatapultSpikeInAir,
    CatapultSpikeOnGround,
    CatapultSpikeExplosion,
    CatapultColdBallOn,
    CatapultColdBall,
    CatapultColdExplosion,
    CatapultPlagueBallOn,
    CatapultPlagueBall,
    CatapultPlagueCloud,
    CatapultMeteorBallOn,
    CatapultMeteorBall,
    CatapultMeteorFire,
    TowerDeath,
    HealingVortex,
    Suicidecorpseexplode,
    Suicidefireexplode,
    Suicideiceexplode,
    Explodingjavalin,
    Explodingjavalinexp,
    Lightingtrailingjavalin,
    Lightjavalintrail,
    Lightjavalinexplosion,
    Icejavalin,
    Icejavalinexplode,
    Plaguejavelin2,
    Plaguejavlinexplode,
    Advlighttrailingjav,
    Advlighttrailingjav2,
    Advlightjavexplode,
    Sucfireball,
    Sucfireballexplode,
    Sucfireballtrail,
    Sucshockfieldmissile,
    Sucshockfieldmissileexp,
    Sucshockfield,
    Hellfiremissile,
    Hellfireexa,
    Hellfireexb,
    ImpChargedBolt,
    ImpTeleport,
    Moltenboulder,
    Moltenboulderemerge,
    Moltenboulderexplode,
    Moltenboulderfirepath,
    MoltenboulderFlyingrocks,
    Firestorm,
    Firestormmaker,
    Arcticblast1,
    Arcticblast2,
    ErruptionCenter,
    ErruptionCrack1,
    ErruptionCrack2,
    ErruptionSmoke1,
    ErruptionSmoke2,
    VineBeastWalk1,
    VineBeastWalk2,
    VineBeastNeutral,
    VineBeastAttack,
    VineBeastDeath,
    Vines,
    VinesTrail,
    VinesWither,
    PlagueVines,
    PlagueVinesTrail,
    PlagueVinesWither,
    Twister,
    Tornado,
    Volcano,
    VolcanoOverlayFire,
    VolcanoDebris2,
    VolcanoExplosion,
    VolcanoSmallFire,
    DragonbreathMissile,
    Lureprojectile,
    Lurecenter,
    Lurecloud,
    Impmiss1,
    Impmiss2,
    Impmiss3,
    Impmiss4,
    Impmiss5,
    FrozenhorrorArcticblast1,
    FrozenhorrorArcticblast2,
    Sentrychargedbolt,
    Sentryspikeinair,
    Sentryspikeonground,
    RecyclerDelay,
    RecyclerVine,
    RecyclerFade,
    RecyclerExplosion,
    DeathMauler,
    DeathMaulerTrail,
    DeathMaulerTrailFade,
    Bladefury1,
    Bladefragment1,
    Bladefury2,
    Bladefragment2,
    Bladefury3,
    Bladefragment3,
    Shockwave,
    Lightningtalons,
    Lightningtalonstrail,
    Phoenixtrail,
    Rabiesplague,
    Rabiescontagion,
    WakeOfDestructionMaker,
    WakeOfDestruction,
    Deathsentryexplode,
    Tigerfury,
    Tigerfurytrail,
    Tigerfurytrail2,
    InfernoSentry1,
    InfernoSentry2,
    AncientThrowingAxe,
    Sentrylightningbolt,
    Sentrylightninghit,
    AnyaCenter,
    AnyaIcicle,
    AnyaIceimpact,
    AnyaIcesteam,
    AnyaIcemagic,
    DragontailMissile,
    Dragonflight,
    Dragonflightmaker,
    ProgressiveRadiusDamage,
    VineBeastWalk1Fade,
    VineBeastWalk2Fade,
    VineBeastNeutralFade,
    VineRecyclerDelay,
    AncientDeathCenter,
    AncientDeathCloud,
    LightningChargeUpNova,
    ChainlightningchargeUp,
    PainWormAppear,
    BaalTauntControl,
    BaalTauntLightning,
    BaalTauntLightningTrail,
    BaalTauntPoison,
    BaalSpawnMonsters,
    MindblastHit,
    BladeShieldMissile,
    BladeShieldAttachment,
    BaalInferno,
    BaalNova,
    Fistsoffireexplode,
    Fistsoffirefirewall,
    Clawsofthunderbolt,
    Clawsofthundernova,
    Bladesoficeexplode,
    Bladesoficecubes,
    Bladesoficecubesmelt,
    Royalstrikemeteor,
    Royalstrikemeteorcenter,
    Royalstrikemeteortail,
    Royalstrikemeteorexplode,
    Royalstrikemeteorfire,
    Royalstrikechainlightning,
    Royalstrikechaosice,
    WorldStoneChip1,
    WorldStoneChip2,
    WorldStoneChip3,
    WorldStoneChip4,
    Highpriestlightning2,
    Infernoflame3,
    MindblastCenter,
    Armageddoncontrol,
    Armageddonrock,
    Armageddontail,
    Armageddonexplosion,
    Hurricaneswoosh,
    Hurricanecart,
    Hurricanerock,
    Hurricanesack,
    Hurricanetree,
    Hurricanevase,
    Baalcorpseexplodedelay,
    Baalcorpseexplodeexpl,
    BaalColdMaker,
    BaalColdTrail,
    BaalSpawnMonstersExp,
    Impmiss21,
    Impmiss22,
    Impmiss23,
    Impmiss24,
    Impmiss25,
    Anyasteam1,
    Anyasteam2,
    Ancientsguide,
    Ancientsmarker,
    Ancientscontrol,
    Overseercontrol,
    Nihlithak1,
    Nihlithak2,
    Nihlithak3,
    Nihlathakcontrol,
    Nihlathakswoosh,
    Nihlathakdebris1,
    Nihlathakdebris2,
    Nihlathakdebris3,
    Nihlathakdebris4,
    Nihlathakglow,
    Baalteleport,
    Baalclonedeath,
    Anyasteamvent,
    Anyasteam,
    Nihlathakhole,
    Nihlathakholelight,
    Volcanofiretrail,
    Nihlathakglow2,
    Nihlathakbonechips,
    Baalcorpseexplodefade,
    Armageddonfire,
    Icesparkle,
    BaalfxControl,
    BaalfxSpirit1,
    BaalfxSpirit2,
    BaalfxSpirit3,
    BaalfxSpirit4,
    BaalfxSpirit5,
    BaalfxBaalHeadAppear,
    BaalfxBaalHead1,
    BaalfxBaalHead2,
    BaalfxBaalHead3,
    BaalfxTyrealDebris1,
    BaalfxTyrealDebris2,
    BaalfxTyrealDebris3,
    BaalfxTyrealDebrisBreak,
    WorldstoneShake,
    Blessedhammerex,
    Sentrylightningbolt2,
    Sentrylightninghit2,
    Lightningtowernova,
    Skbowarrow6,
    Skbowarrow7,
    Skbowarrow8,
    Bighead6,
    ViperPoisjav,
    ViperPoisjavcloud,
    ViperFire,
    ViperFirecloud,
    ViperBonespear,
    Countessfirewallmaker,
    BaalTauntLightningControl,
    BaalTauntPoisonControl,
    Explodingarrowexp2,
    Freezingarrowexp3,
    Pantherjav5,
    Spike6,
    CrArrow6,
    Skmagepois,
    Skmagecold,
    Skmagefire,
    Skmageltng,
    Succubusmiss,
    Willowisplightningbolt2,
    Mummyex,
    Goospitex,
    Impmissex,
    Diablogeddoncontrol,
    Diablogeddonrock,
    Diablogeddontail,
    Diablogeddonexplosion,
    Diablogeddonfire,
    Megademoninferno,
    Trapfirebolt,
    Trappoisonjavcloud,
    Trapnova,
    Mephfrostnova,
    Mephlight,
    Vampiremeteorfire,
    Strafearrow,
    Strafebolt,
    Recklessattacksmissile,
    Unknown,
}

fn get_missile_color(txt_file_no: &Missile) -> u32 {
    let arrow_color = 0xFFFFFF54;
    let javelin_color = 0xFFFFFF54;
    let bighead1_color = 0x8F8FFF54;
    let bighead2_color = 0x8F8FFF54;
    let bighead3_color = 0x8F8FFF54;
    let bighead4_color = 0x8F8FFF54;
    let bighead5_color = 0x8F8FFF54;
    let spike1_color = 0xFFFFFF54;
    let spike2_color = 0xFFFFFF54;
    let spike3_color = 0xFFFFFF54;
    let spike4_color = 0xFFFFFF54;
    let spike5_color = 0xFFFFFF54;
    let firearrow_color = 0xFFB24054;
    let crarrow1_color = 0xFFFFFF54;
    let crarrow2_color = 0xFFFFFF54;
    let crarrow3_color = 0xFFFFFF54;
    let crarrow4_color = 0xFFFFFF54;
    let crarrow5_color = 0xFFFFFF54;
    let blood1_color = 0xFFFFFF54;
    let blood2_color = 0xFFFFFF54;
    let bigblood1_color = 0xFFFFFF54;
    let bigblood2_color = 0xFFFFFF54;
    let shafire1_color = 0xFFB24054;
    let shafire2_color = 0xFFB24054;
    let shafire3_color = 0xFFB24054;
    let shafire4_color = 0xFFB24054;
    let shafire5_color = 0xFFB24054;
    let magicarrow_color = 0xFFFFFF54;
    let icearrow_color = 0x5151FF54;
    let fireexplode_color = 0xFFB24054;
    let iceexplode_color = 0x5151FF54;
    let bolt_color = 0xFFFFFF54;
    let andarielspray_color = 0xFFFFFF54;
    let bigheadexp_color = 0x8F8FFF54;
    let shamanexp_color = 0xFFB24054;
    let throwaxe_color = 0xFFFFFF54;
    let throwknife_color = 0xFFFFFF54;
    let glaive_color = 0xFFFFFF54;
    let poisonjav_color = 0x80FF8054;
    let poisonjavcloud_color = 0x80FF8054;
    let coldarrow_color = 0x5151FF54;
    let explodingarrow_color = 0xFFB24054;
    let explodingarrowexp_color = 0xFFB24054;
    let plaguejavelin_color = 0x80FF8054;
    let oilpotion_color = 0xFF808054;
    let explosivepotion_color = 0xFF808054;
    let fulminatingpotion_color = 0xFF808054;
    let rancidgasepotion_color = 0x80FF8054;
    let chokinggaspoition_color = 0x80FF8054;
    let stranglinggaspotion_color = 0x80FF8054;
    let notused50_color = 0xFFFFFF54;
    let explosivepotionexp_color = 0xFFB24054;
    let explosivepotiondebris1_color = 0xFFFFFF54;
    let explosivepotiondebris2_color = 0xFFFFFF54;
    let explosivepotiondebris3_color = 0xFFFFFF54;
    let holybolt_color = 0xDEDEFF54;
    let chargedbolt_color = 0xFFFFFF54;
    let sanctuarybolt_color = 0xDEDEFF54;
    let firebolt_color = 0xFFB24054;
    let icebolt_color = 0x5151FF54;
    let infernoflame1_color = 0xFFB24054;
    let infernoflame2_color = 0xFFB24054;
    let fireball_color = 0xFFB24054;
    let mummy1_color = 0xFFFFFF54;
    let mummy2_color = 0xFFFFFF54;
    let mummy3_color = 0xFFFFFF54;
    let mummy4_color = 0xFFFFFF54;
    let blaze_color = 0xFFB24054;
    let firewallmaker_color = 0xFFB24054;
    let firewall_color = 0xFFB24054;
    let goospit1_color = 0xFFFFFF54;
    let goospit2_color = 0xFFFFFF54;
    let goospit3_color = 0xFFFFFF54;
    let goospit4_color = 0xFFFFFF54;
    let goospit5_color = 0xFFFFFF54;
    let goosplat_color = 0xFFFFFF54;
    let sandpile_color = 0xFFFFFF54;
    let unholybolt1_color = 0x00000054;
    let unholybolt2_color = 0x00000054;
    let unholybolt3_color = 0x00000054;
    let unholybolt4_color = 0x00000054;
    let sanctuarycenter_color = 0x00000054;
    let fireexplosion_color = 0xFFB24054;
    let stuckarrow_color = 0x00000054;
    let footprint_color = 0x00000054;
    let immolationarrow_color = 0xFFB24054;
    let guidedarrow_color = 0xFFFFFF54;
    let freezingarrow_color = 0x5151FF54;
    let freezingarrowexp1_color = 0x5151FF54;
    let freezingarrowexp2_color = 0x5151FF54;
    let nova_color = 0xFFFFFF54;
    let iceblast_color = 0x5151FF54;
    let blessedhammer_color = 0xDEDEFF54;
    let chainlightning_color = 0xFFFFFF54;
    let fistofares_color = 0xFFFFFF54;
    let chillblood_color = 0x5151FF54;
    let glacialspike_color = 0x5151FF54;
    let teleport_color = 0xFFFFFF54;
    let lightningbolt_color = 0xFFFFFF54;
    let lightninghit_color = 0xFFFFFF54;
    let meteor_color = 0x00000054;
    let meteorcenter_color = 0xFF404054;
    let meteortail_color = 0x00000054;
    let meteorexplode_color = 0xFFB24054;
    let firesmall_color = 0xFFB24054;
    let firemedium_color = 0xFFB24054;
    let monblizcenter_color = 0x5151FF54;
    let monbliz1_color = 0x00000054;
    let monbliz2_color = 0x00000054;
    let monbliz3_color = 0x00000054;
    let monbliz4_color = 0x00000054;
    let monblizexplode1_color = 0x00000054;
    let monblizexplode2_color = 0x00000054;
    let monblizexplode3_color = 0x00000054;
    let teeth_color = 0xFFFFFF54;
    let corpseexplosion_color = 0xFFB24054;
    let poisoncorpseexplosion_color = 0xFFB24054;
    let monstercorpseexplode_color = 0xFFB24054;
    let poisonnova_color = 0x00000054;
    let frostnova_color = 0x5151FF54;
    let rogue1_color = 0xFFFFFF54;
    let rogue2_color = 0xFFB24054;
    let rogue3_color = 0x5151FF54;
    let batlightningbolt_color = 0xFFFFFF54;
    let batlightningtrail_color = 0xFFFFFF54;
    let skmage1_color = 0xFFFFFF54;
    let skmage2_color = 0xFFFFFF54;
    let skmage3_color = 0xFFFFFF54;
    let skmage4_color = 0xFFFFFF54;
    let vampirefireball_color = 0xFFB24054;
    let vampirefirewallmaker_color = 0xFFB24054;
    let vampirefirewall_color = 0xFFB24054;
    let vampiremeteor_color = 0x00000054;
    let vampiremeteorcenter_color = 0xFF404054;
    let vampiremeteorexp_color = 0xFFB24054;
    let raven1_color = 0xFFFFFF54;
    let raven2_color = 0xFFB24054;
    let amphibiangoo1_color = 0x40FF4054;
    let amphibiangoo2_color = 0x40FF4054;
    let tentaclegoo_color = 0x40FF4054;
    let amphibianexplode_color = 0xFFFFFF54;
    let poisonpuff_color = 0xFFFFFF54;
    let curseeffectred_color = 0xFF000054;
    let spidergoolay_color = 0x00000054;
    let fetishinferno1_color = 0xFFB24054;
    let fetishinferno2_color = 0xFFB24054;
    let spidergoo_color = 0x00000054;
    let cursecast_color = 0xFF000054;
    let howl_color = 0xC0C0C054;
    let shout_color = 0xC0C0C054;
    let dust_color = 0xFFFFFF54;
    let redlightmissile_color = 0xFFB24054;
    let greenlightmissile_color = 0x80FF8054;
    let bluelightmissile_color = 0x5151FF54;
    let whitelightmissile_color = 0xFFFFFF54;
    let corpsepoisoncloud_color = 0x80FF8054;
    let chillbloodcloud_color = 0x5151FF54;
    let chillbloodpuff_color = 0x5151FF54;
    let blizzardcenter_color = 0x5151FF54;
    let blizzard1_color = 0x00000054;
    let blizzard2_color = 0x00000054;
    let blizzard3_color = 0x00000054;
    let blizzard4_color = 0x00000054;
    let blizzardexplode1_color = 0x00000054;
    let blizzardexplode2_color = 0x00000054;
    let blizzardexplode3_color = 0x00000054;
    let thunderstorm1_color = 0xFFFFFF54;
    let thunderstorm2_color = 0xFFFFFF54;
    let thunderstorm3_color = 0xFFFFFF54;
    let thunderstorm4_color = 0xFFFFFF54;
    let monsterlight_color = 0xFFFFFF54;
    let poisonball_color = 0xFFFFFF54;
    let diablight_color = 0xFFFFFF54;
    let redemption_color = 0xFFFFFF54;
    let redemptionfail_color = 0xFFB24054;
    let handofgod_color = 0xFFFFFF54;
    let diabfire_color = 0xFFFFFF54;
    let fingermagespider_color = 0xFFB2B254;
    let electricthrowaxe_color = 0xFFFFFF54;
    let diabwallmaker_color = 0xFFB24054;
    let diabwall_color = 0xFFB24054;
    let curseamplifydamage_color = 0xFF000054;
    let cursedimvision_color = 0xFF000054;
    let curseweaken_color = 0xFF000054;
    let curseironmaiden_color = 0xFF000054;
    let curseterror_color = 0xFF000054;
    let curseattract_color = 0xFF000054;
    let cursereversevampire_color = 0xFF000054;
    let curseconfuse_color = 0xFF000054;
    let cursedecrepify_color = 0xFF000054;
    let curselowerresist_color = 0xFF000054;
    let cursecenter_color = 0xFF000054;
    let bonespear_color = 0xFFFFFF54;
    let bonespirit_color = 0xFFFFFF54;
    let coldunique_color = 0x5151FF54;
    let lightunique_color = 0xFFFFFF54;
    let skbowarrow1_color = 0xFFFFFF54;
    let skbowarrow2_color = 0xFFFFFF54;
    let skbowarrow3_color = 0xFFFFFF54;
    let skbowarrow4_color = 0xFFB24054;
    let skbowarrow5_color = 0xFFFFFF54;
    let nova1_color = 0xFFFFFF54;
    let nova2_color = 0xFFFFFF54;
    let andypoisonbolt_color = 0xFFFFFF54;
    let teethexplode_color = 0x00000054;
    let lightningjavelin_color = 0xFFFFFF54;
    let lightningfury_color = 0xFFFFFF54;
    let bonewallmaker_color = 0xFFFFFF54;
    let necromage1_color = 0xFFFFFF54;
    let necromage2_color = 0xFFFFFF54;
    let necromage3_color = 0xFFFFFF54;
    let necromage4_color = 0xFFFFFF54;
    let sparkle_color = 0xFFFFFF54;
    let multipleshotarrow_color = 0xFFFFFF54;
    let multipleshotbolt_color = 0xFFFFFF54;
    let chargedstrikebolt_color = 0xFFFFFF54;
    let bonespearexplode_color = 0x00000054;
    let poisonexplosioncloud_color = 0x80FF8054;
    let bonecast_color = 0xFFFFFF54;
    let battlecry_color = 0xC0C0C054;
    let primepoisoncloud_color = 0xFFFFFF54;
    let plaguejavcloud_color = 0xFFFFFF54;
    let rancidgascloud_color = 0xFFFFFF54;
    let chokinggascloud_color = 0xFFFFFF54;
    let stranglinggascloud_color = 0xFFFFFF54;
    let buglightning_color = 0xFFFFFF54;
    let pantherjav1_color = 0xFFFFFF54;
    let pantherjav2_color = 0xFFFFFF54;
    let pantherjav3_color = 0xFFFFFF54;
    let pantherjav4_color = 0xFFFFFF54;
    let immolationfire_color = 0xFFB24054;
    let furylightning_color = 0xFFFFFF54;
    let lightningstrike_color = 0xFFFFFF54;
    let fistoftheheavensdelay_color = 0x00000054;
    let fistoftheheavensbolt_color = 0xDEDEFF54;
    let warcry_color = 0xC0C0C054;
    let battlecommand_color = 0xC0C0C054;
    let battleorders_color = 0xC0C0C054;
    let pantherpotorange_color = 0xFF808054;
    let pantherpotgreen_color = 0x80FF8054;
    let meteorfire_color = 0xFFB24054;
    let trapspikeright_color = 0xFFFFFF54;
    let trapspikeleft_color = 0xFFFFFF54;
    let trapcursedskullright_color = 0xFFFFFF54;
    let trapcursedskullleft_color = 0xFFFFFF54;
    let trappoisonballright_color = 0xFFFFFF54;
    let trappoisonballleft_color = 0xFFFFFF54;
    let hydra_color = 0xFFFFFF54;
    let bonespeartrail_color = 0xFFFFFF54;
    let grimwardsmallstart_color = 0x00000054;
    let grimwardsmall_color = 0x00000054;
    let grimwardsmallstop_color = 0x00000054;
    let grimwardmediumstart_color = 0x00000054;
    let grimwardmedium_color = 0x00000054;
    let grimwardmediumstop_color = 0x00000054;
    let grimwardlargestart_color = 0x00000054;
    let grimwardlarge_color = 0x00000054;
    let grimwardlargestop_color = 0x00000054;
    let zakarumlight_color = 0xFFFFFF54;
    let grimwardscare_color = 0x00000054;
    let frozenorb_color = 0x5151FF54;
    let frozenorbbolt_color = 0x5151FF54;
    let frozenorbnova_color = 0x5151FF54;
    let frozenorbexplode_color = 0x5151FF54;
    let chillingarmorbolt_color = 0x5151FF54;
    let fireexplosion2_color = 0x00000054;
    let blowgun_color = 0xFFFFFF54;
    let chainlightning2_color = 0xFFFFFF54;
    let revivesmall_color = 0xFFB24054;
    let revivemedium_color = 0xFFB24054;
    let revivelarge_color = 0xFFB24054;
    let monglacialspike_color = 0x5151FF54;
    let icebreaksmall_color = 0xFFFFFF54;
    let icebreakmedium_color = 0xFFFFFF54;
    let icebreaklarge_color = 0xFFFFFF54;
    let icebreaksmoke_color = 0xFFFFFF54;
    let mephisto_color = 0xFFFFFF54;
    let firehead_color = 0xFFB24054;
    let whilrwind_color = 0x00000054;
    let arcanelightningbolt_color = 0xB2B2FF54;
    let frogfire_color = 0xFF404054;
    let frogcold_color = 0x4040FF54;
    let frogpois_color = 0x40FF4054;
    let desertfireball_color = 0xFF404054;
    let brdeathcontrol_color = 0xFFFFFF54;
    let brdeathlightningbolt_color = 0xFFFFFF54;
    let brdeathlightninghit_color = 0xFFFFFF54;
    let denofevillight_color = 0xFFFFFF54;
    let cairnstones_color = 0xFFFFFF54;
    let cairnstonessky_color = 0xFFFFFF54;
    let cairnstonesground_color = 0xFFFFFF54;
    let towermist_color = 0xFFFFFF54;
    let towermisttrail_color = 0xFFFFFF54;
    let brdeathsmokes1_color = 0x00000054;
    let brdeathsmokenu_color = 0x00000054;
    let brdeathsmokedt_color = 0x00000054;
    let brdeathspirits1_color = 0xFFFFFF54;
    let brdeathspiritnu_color = 0xFFFFFF54;
    let brdeathspiritdt_color = 0xFFFFFF54;
    let mephistodeathcontrol_color = 0xFFFFFF54;
    let mephistofirewallmaker_color = 0xFFB24054;
    let mephistofirewall_color = 0xFFB24054;
    let mephistoflyingrocksbig_color = 0x00000054;
    let mephistoexplosionbig_color = 0xFFB24054;
    let mephistoflyingrockssmall_color = 0x00000054;
    let mephistoexplosionsmall_color = 0xFFB24054;
    let mephistodonotdraw_color = 0x00000054;
    let andycontrol0_color = 0x00000054;
    let andyfirewallmaker_color = 0xFFB24054;
    let andyfirewall_color = 0xFFB24054;
    let andycolumnfirebase_color = 0xFFB24054;
    let andycolumnfire_color = 0xC0808054;
    let andyfallingdebris1_color = 0x00000054;
    let andyfallingdebris2_color = 0x00000054;
    let andyfallingdebris3_color = 0x00000054;
    let andydebrisexplosion1_color = 0x00000054;
    let andydebrisexplosion2_color = 0x00000054;
    let andydebrisexplosion3_color = 0x00000054;
    let andydebrisexplosion4_color = 0x00000054;
    let andydebrisexplosion5_color = 0x00000054;
    let willowisplightningbolt_color = 0xB2B2FF54;
    let queenpoisoncloud_color = 0x40FF4054;
    let dirtpile_color = 0xFFFFFF54;
    let undeadmissile1_color = 0xFFFFFF54;
    let undeadmissile2_color = 0xFFFFFF54;
    let undeadmissile3_color = 0xFFFFFF54;
    let undeadmissile4_color = 0xFFFFFF54;
    let bonespiritexplode_color = 0xFFFFFF54;
    let dopplezonexplode_color = 0xFFFFFF54;
    let monbonespirit_color = 0xFFFFFF54;
    let towermistfade_color = 0xFFFFFF54;
    let countessfirewall_color = 0xFFB24054;
    let towerchestspawner_color = 0xC0C0FF54;
    let hellmeteorlaunch1_color = 0xFFFFFF54;
    let hellmeteorlaunch2_color = 0xFFFFFF54;
    let hellmeteorup_color = 0xFFFFFF54;
    let hellmeteordown_color = 0xFFFFFF54;
    let hellmeteorball_color = 0xFFFFFF54;
    let horadricstaff_color = 0xFFFFFF54;
    let horadriclightning_color = 0xFFFFFF54;
    let horadriclight_color = 0xFFFFFF54;
    let regurgitatorcorpse_color = 0xFF404054;
    let regurgitatorcorpseexpl_color = 0xFF404054;
    let highpriestlightning_color = 0xFF404054;
    let icebreaksmallmelt_color = 0xFFFFFF54;
    let icebreaklargemelt_color = 0xFFFFFF54;
    let leapknockback_color = 0x00000054;
    let radamentdeath_color = 0xFFFFFF54;
    let radamenthandofgod_color = 0x00000054;
    let radamentholybolt_color = 0xFFFFFF54;
    let taintedsuncontrol_color = 0xFFFFFF54;
    let taintedsunflash_color = 0xFFFFFF54;
    let taintedsunball_color = 0xFFFFFF54;
    let queendeathcenter_color = 0xFFFFFF54;
    let queendeathglob_color = 0xFFFFFF54;
    let queendeathsplat1_color = 0xFFFFFF54;
    let queendeathsplat2_color = 0xFFFFFF54;
    let healingbolt_color = 0xDEDEFF54;
    let mephistoholedelay_color = 0x00000054;
    let mephistoholebirth_color = 0x00000054;
    let mephistoholeneutral_color = 0x00000054;
    let mephistoholedeath_color = 0x00000054;
    let mephistoholedead_color = 0x00000054;
    let durieldeathcontrol_color = 0x00000054;
    let durieldeathrock_color = 0x00000054;
    let durieldeathdebris_color = 0x00000054;
    let durieldeathsmoke_color = 0x00000054;
    let mephistoexplosion_color = 0xFFFFFF54;
    let orbmist_color = 0xFF7D7D54;
    let orbmisttrail_color = 0xFF7D7D54;
    let orbmistfade_color = 0xFF7D7D54;
    let pilum_color = 0xFFFFFF54;
    let diabloappears_color = 0xFFFFFF54;
    let hfcontrol_color = 0xFFFFFF54;
    let hffragment1_color = 0xFFFFFF54;
    let hffragment2_color = 0xFFFFFF54;
    let hffragment3_color = 0xFFFFFF54;
    let hfspirit1_color = 0xFFFFFF54;
    let hfreserved3_color = 0xFFFFFF54;
    let izualcontrol_color = 0xFFFFFF54;
    let izualmistloop_color = 0xFFFFFF54;
    let izualmistfade_color = 0xFFFFFF54;
    let izuallightning_color = 0xFFFFFF54;
    let izuallightningtrail_color = 0xFFFFFF54;
    let cairnstonesbolt_color = 0xFFFFFF54;
    let bombinair_color = 0xFFFFFF54;
    let bombonground_color = 0xFFFFFF54;
    let bombexplosion_color = 0xFFB24054;
    let shockfieldinair_color = 0xFFFFFF54;
    let shockfieldonground_color = 0xFFFFFF54;
    let throwingstar_color = 0xFFFFFF54;
    let acidspray_color = 0xFFFFFF54;
    let bladecreeper_color = 0xFFFFFF54;
    let distraction_color = 0xFFFFFF54;
    let distractionfog_color = 0xFFFFFF54;
    let distractionpuff_color = 0xFFFFFF54;
    let distractionstart_color = 0xFFFFFF54;
    let distractionend_color = 0xFFFFFF54;
    let impinfernoflame1_color = 0xFFB24054;
    let impinfernoflame2_color = 0xFFB24054;
    let baallightningbolt_color = 0xFFFFFF54;
    let baallightningtrail_color = 0xFFFFFF54;
    let baallightningbolt2_color = 0xFFFFFF54;
    let baallightningtrail2_color = 0xFFFFFF54;
    let impfireball_color = 0xFFB24054;
    let impfireballexplode_color = 0xFFB24054;
    let catapultchargedballon_color = 0xFFFFFF54;
    let catapultchargedball_color = 0x8F8FFF54;
    let catapultchargedballbolt_color = 0xFFFFFF54;
    let impspawnmonsters_color = 0xFFB24054;
    let catapultspikeballon_color = 0xFFFFFF54;
    let catapultspikeball_color = 0xFFFFFF54;
    let catapultspikeinair_color = 0xFFFFFF54;
    let catapultspikeonground_color = 0xFFFFFF54;
    let catapultspikeexplosion_color = 0xFFFFFF54;
    let catapultcoldballon_color = 0xFFFFFF54;
    let catapultcoldball_color = 0x8F8FFF54;
    let catapultcoldexplosion_color = 0xFFFFFF54;
    let catapultplagueballon_color = 0xFFFFFF54;
    let catapultplagueball_color = 0x8F8FFF54;
    let catapultplaguecloud_color = 0x80FF8054;
    let catapultmeteorballon_color = 0xFFFFFF54;
    let catapultmeteorball_color = 0x8F8FFF54;
    let catapultmeteorfire_color = 0xFFB24054;
    let towerdeath_color = 0xFFFFFF54;
    let healingvortex_color = 0xFFB24054;
    let suicidecorpseexplode_color = 0xFFB24054;
    let suicidefireexplode_color = 0xFFB24054;
    let suicideiceexplode_color = 0xFFB24054;
    let explodingjavalin_color = 0xFFFFFF54;
    let explodingjavalinexp_color = 0xFFB24054;
    let lightingtrailingjavalin_color = 0xFFFFFF54;
    let lightjavalintrail_color = 0xFFFFFF54;
    let lightjavalinexplosion_color = 0xFFFFFF54;
    let icejavalin_color = 0xFFFFFF54;
    let icejavalinexplode_color = 0x5151FF54;
    let plaguejavelin2_color = 0xFFFFFF54;
    let plaguejavlinexplode_color = 0xFFFFFF54;
    let advlighttrailingjav_color = 0xFFFFFF54;
    let advlighttrailingjav2_color = 0xFFFFFF54;
    let advlightjavexplode_color = 0xFFFFFF54;
    let sucfireball_color = 0xFFB24054;
    let sucfireballexplode_color = 0xFFB24054;
    let sucfireballtrail_color = 0xFFB24054;
    let sucshockfieldmissile_color = 0xFFFFFF54;
    let sucshockfieldmissileexp_color = 0x00000054;
    let sucshockfield_color = 0xFFB24054;
    let hellfiremissile_color = 0x00000054;
    let hellfireexa_color = 0xFFFFFF54;
    let hellfireexb_color = 0xFFFFFF54;
    let impchargedbolt_color = 0xFFFFFF54;
    let impteleport_color = 0xFFFFFF54;
    let moltenboulder_color = 0xFFB24054;
    let moltenboulderemerge_color = 0xFFB24054;
    let moltenboulderexplode_color = 0xFFB24054;
    let moltenboulderfirepath_color = 0xFFB24054;
    let moltenboulderflyingrocks_color = 0xFFB24054;
    let firestorm_color = 0xFFB24054;
    let firestormmaker_color = 0xFFFFFF54;
    let arcticblast1_color = 0xFFFFFF54;
    let arcticblast2_color = 0xFFFFFF54;
    let erruptioncenter_color = 0xFFFFFF54;
    let erruptioncrack1_color = 0xFFB24054;
    let erruptioncrack2_color = 0xFFB24054;
    let erruptionsmoke1_color = 0xFFFFFF54;
    let erruptionsmoke2_color = 0xFFFFFF54;
    let vinebeastwalk1_color = 0xFFFFFF54;
    let vinebeastwalk2_color = 0xFFFFFF54;
    let vinebeastneutral_color = 0xFFFFFF54;
    let vinebeastattack_color = 0xFFFFFF54;
    let vinebeastdeath_color = 0xFFFFFF54;
    let vines_color = 0xFFFFFF54;
    let vinestrail_color = 0xFFFFFF54;
    let vineswither_color = 0xFFFFFF54;
    let plaguevines_color = 0xFFFFFF54;
    let plaguevinestrail_color = 0xFFFFFF54;
    let plaguevineswither_color = 0xFFFFFF54;
    let twister_color = 0xFFFFFF54;
    let tornado_color = 0xFFFFFF54;
    let volcano_color = 0x00000054;
    let volcanooverlayfire_color = 0xFF404054;
    let volcanodebris2_color = 0x00000054;
    let volcanoexplosion_color = 0x00000054;
    let volcanosmallfire_color = 0x00000054;
    let dragonbreathmissile_color = 0xFFFFFF54;
    let lureprojectile_color = 0xFFFFFF54;
    let lurecenter_color = 0x5151FF54;
    let lurecloud_color = 0x00000054;
    let impmiss1_color = 0xFF404054;
    let impmiss2_color = 0xFF404054;
    let impmiss3_color = 0xFF404054;
    let impmiss4_color = 0xFF404054;
    let impmiss5_color = 0xFF404054;
    let frozenhorrorarcticblast1_color = 0xFFFFFF54;
    let frozenhorrorarcticblast2_color = 0xFFFFFF54;
    let sentrychargedbolt_color = 0xFFFFFF54;
    let sentryspikeinair_color = 0xFFFFFF54;
    let sentryspikeonground_color = 0xFFFFFF54;
    let recyclerdelay_color = 0x00000054;
    let recyclervine_color = 0xFFFFFF54;
    let recyclerfade_color = 0xFFFFFF54;
    let recyclerexplosion_color = 0xFFB24054;
    let deathmauler_color = 0xFFFFFF54;
    let deathmaulertrail_color = 0xFFFFFF54;
    let deathmaulertrailfade_color = 0xFFFFFF54;
    let bladefury1_color = 0x00000054;
    let bladefragment1_color = 0x00000054;
    let bladefury2_color = 0x00000054;
    let bladefragment2_color = 0x00000054;
    let bladefury3_color = 0x00000054;
    let bladefragment3_color = 0x00000054;
    let shockwave_color = 0xFFFFFF54;
    let lightningtalons_color = 0xFFFFFF54;
    let lightningtalonstrail_color = 0xFFFFFF54;
    let phoenixtrail_color = 0xFFB24054;
    let rabiesplague_color = 0x00000054;
    let rabiescontagion_color = 0x00000054;
    let wakeofdestructionmaker_color = 0xFFB24054;
    let wakeofdestruction_color = 0xFFB24054;
    let deathsentryexplode_color = 0x00000054;
    let tigerfury_color = 0xFFFFFF54;
    let tigerfurytrail_color = 0xFFB24054;
    let tigerfurytrail2_color = 0xFFFFFF54;
    let infernosentry1_color = 0xFFB24054;
    let infernosentry2_color = 0xFFB24054;
    let ancientthrowingaxe_color = 0xFFFFFF54;
    let sentrylightningbolt_color = 0xFFFFFF54;
    let sentrylightninghit_color = 0xFFFFFF54;
    let anyacenter_color = 0x00000054;
    let anyaicicle_color = 0x00000054;
    let anyaiceimpact_color = 0x00000054;
    let anyaicesteam_color = 0x00000054;
    let anyaicemagic_color = 0x00000054;
    let dragontailmissile_color = 0xFFB24054;
    let dragonflight_color = 0xFFFFFF54;
    let dragonflightmaker_color = 0xFFFFFF54;
    let progressiveradiusdamage_color = 0xFFFFFF54;
    let vinebeastwalk1fade_color = 0xFFFFFF54;
    let vinebeastwalk2fade_color = 0xFFFFFF54;
    let vinebeastneutralfade_color = 0xFFFFFF54;
    let vinerecyclerdelay_color = 0x00000054;
    let ancientdeathcenter_color = 0xFF7D7D54;
    let ancientdeathcloud_color = 0xFF7D7D54;
    let lightningchargeupnova_color = 0xFFFFFF54;
    let chainlightningchargeup_color = 0xFFFFFF54;
    let painwormappear_color = 0xFFB24054;
    let baaltauntcontrol_color = 0xFFFFFF54;
    let baaltauntlightning_color = 0xFFFFFF54;
    let baaltauntlightningtrail_color = 0xFFFFFF54;
    let baaltauntpoison_color = 0xFFFFFF54;
    let baalspawnmonsters_color = 0x5151FF54;
    let mindblasthit_color = 0xFFFFFF54;
    let bladeshieldmissile_color = 0xFFFFFF54;
    let bladeshieldattachment_color = 0x00000054;
    let baalinferno_color = 0xFFFFFF54;
    let baalnova_color = 0xFFFFFF54;
    let fistsoffireexplode_color = 0xFFB24054;
    let fistsoffirefirewall_color = 0xFFB24054;
    let clawsofthunderbolt_color = 0xFFFFFF54;
    let clawsofthundernova_color = 0xFFFFFF54;
    let bladesoficeexplode_color = 0x5151FF54;
    let bladesoficecubes_color = 0xFFFFFF54;
    let bladesoficecubesmelt_color = 0xFFFFFF54;
    let royalstrikemeteor_color = 0x00000054;
    let royalstrikemeteorcenter_color = 0xFF404054;
    let royalstrikemeteortail_color = 0x00000054;
    let royalstrikemeteorexplode_color = 0xFFB24054;
    let royalstrikemeteorfire_color = 0xFFB24054;
    let royalstrikechainlightning_color = 0xFFFFFF54;
    let royalstrikechaosice_color = 0x5151FF54;
    let worldstonechip1_color = 0xFFFFFF54;
    let worldstonechip2_color = 0xFFFFFF54;
    let worldstonechip3_color = 0xFFFFFF54;
    let worldstonechip4_color = 0xFFFFFF54;
    let highpriestlightning2_color = 0xFF404054;
    let infernoflame3_color = 0xFFB24054;
    let mindblastcenter_color = 0xFFFFFF54;
    let armageddoncontrol_color = 0x00000054;
    let armageddonrock_color = 0x00000054;
    let armageddontail_color = 0x00000054;
    let armageddonexplosion_color = 0xFFFFFF54;
    let hurricaneswoosh_color = 0x00000054;
    let hurricanecart_color = 0x00000054;
    let hurricanerock_color = 0x00000054;
    let hurricanesack_color = 0x00000054;
    let hurricanetree_color = 0x00000054;
    let hurricanevase_color = 0x00000054;
    let baalcorpseexplodedelay_color = 0xFFB24054;
    let baalcorpseexplodeexpl_color = 0xFFB24054;
    let baalcoldmaker_color = 0x8F8FFF54;
    let baalcoldtrail_color = 0x8F8FFF54;
    let baalspawnmonstersexp_color = 0x5151FF54;
    let impmiss21_color = 0xFF404054;
    let impmiss22_color = 0xFF404054;
    let impmiss23_color = 0xFF404054;
    let impmiss24_color = 0xFF404054;
    let impmiss25_color = 0xFF404054;
    let anyasteam1_color = 0x00000054;
    let anyasteam2_color = 0x00000054;
    let ancientsguide_color = 0x00000054;
    let ancientsmarker_color = 0x00000054;
    let ancientscontrol_color = 0x00000054;
    let overseercontrol_color = 0x00000054;
    let nihlithak1_color = 0x00000054;
    let nihlithak2_color = 0x00000054;
    let nihlithak3_color = 0x00000054;
    let nihlathakcontrol_color = 0x00000054;
    let nihlathakswoosh_color = 0x00000054;
    let nihlathakdebris1_color = 0x00000054;
    let nihlathakdebris2_color = 0x00000054;
    let nihlathakdebris3_color = 0x00000054;
    let nihlathakdebris4_color = 0x00000054;
    let nihlathakglow_color = 0x00000054;
    let baalteleport_color = 0x80404054;
    let baalclonedeath_color = 0x80404054;
    let anyasteamvent_color = 0x00000054;
    let anyasteam_color = 0x00000054;
    let nihlathakhole_color = 0x00000054;
    let nihlathakholelight_color = 0x00000054;
    let volcanofiretrail_color = 0x00000054;
    let nihlathakglow2_color = 0x00000054;
    let nihlathakbonechips_color = 0x00000054;
    let baalcorpseexplodefade_color = 0x00000054;
    let armageddonfire_color = 0xFFB24054;
    let icesparkle_color = 0xFFB24054;
    let baalfxcontrol_color = 0xFFB24054;
    let baalfxspirit1_color = 0x00000054;
    let baalfxspirit2_color = 0x00000054;
    let baalfxspirit3_color = 0x00000054;
    let baalfxspirit4_color = 0x00000054;
    let baalfxspirit5_color = 0x00000054;
    let baalfxbaalheadappear_color = 0x00000054;
    let baalfxbaalhead1_color = 0xFFB24054;
    let baalfxbaalhead2_color = 0xFFB24054;
    let baalfxbaalhead3_color = 0xFFB24054;
    let baalfxtyrealdebris1_color = 0x00000054;
    let baalfxtyrealdebris2_color = 0x00000054;
    let baalfxtyrealdebris3_color = 0x00000054;
    let baalfxtyrealdebrisbreak_color = 0x00000054;
    let worldstoneshake_color = 0x00000054;
    let blessedhammerex_color = 0xDEDEFF54;
    let sentrylightningbolt2_color = 0xFFFFFF54;
    let sentrylightninghit2_color = 0xFFFFFF54;
    let lightningtowernova_color = 0xFFFFFF54;
    let skbowarrow6_color = 0xFFB24054;
    let skbowarrow7_color = 0x5151FF54;
    let skbowarrow8_color = 0xFFB24054;
    let bighead6_color = 0x8F8FFF54;
    let viperpoisjav_color = 0xFFFFFF54;
    let viperpoisjavcloud_color = 0x80FF8054;
    let viperfire_color = 0xFFFFFF54;
    let viperfirecloud_color = 0xFFB24054;
    let viperbonespear_color = 0xFFFFFF54;
    let countessfirewallmaker_color = 0xFFB24054;
    let baaltauntlightningcontrol_color = 0x00000054;
    let baaltauntpoisoncontrol_color = 0x00000054;
    let explodingarrowexp2_color = 0x00000054;
    let freezingarrowexp3_color = 0x00000054;
    let pantherjav5_color = 0xFFFFFF54;
    let spike6_color = 0xFFFFFF54;
    let crarrow6_color = 0xFFFFFF54;
    let skmagepois_color = 0xFFFFFF54;
    let skmagecold_color = 0xFFFFFF54;
    let skmagefire_color = 0xFFFFFF54;
    let skmageltng_color = 0xFFFFFF54;
    let succubusmiss_color = 0xFF404054;
    let willowisplightningbolt2_color = 0xB2B2FF54;
    let mummyex_color = 0xFFFFFF54;
    let goospitex_color = 0xFFFFFF54;
    let impmissex_color = 0xFF404054;
    let diablogeddoncontrol_color = 0x00000054;
    let diablogeddonrock_color = 0x00000054;
    let diablogeddontail_color = 0x00000054;
    let diablogeddonexplosion_color = 0xFFFFFF54;
    let diablogeddonfire_color = 0xFFB24054;
    let megademoninferno_color = 0xFFB24054;
    let trapfirebolt_color = 0xFFB24054;
    let trappoisonjavcloud_color = 0x80FF8054;
    let trapnova_color = 0xFFFFFF54;
    let mephfrostnova_color = 0x5151FF54;
    let mephlight_color = 0xFFFFFF54;
    let vampiremeteorfire_color = 0xFFB24054;
    let strafearrow_color = 0xFFFFFF54;
    let strafebolt_color = 0xFFFFFF54;
    let recklessattacksmissile_color = 0xFFFFFF54;
    let unknown_color = 0x00000000;

    match txt_file_no {
        Missile::Arrow => arrow_color,
        Missile::Javelin => javelin_color,
        Missile::Bighead1 => bighead1_color,
        Missile::Bighead2 => bighead2_color,
        Missile::Bighead3 => bighead3_color,
        Missile::Bighead4 => bighead4_color,
        Missile::Bighead5 => bighead5_color,
        Missile::Spike1 => spike1_color,
        Missile::Spike2 => spike2_color,
        Missile::Spike3 => spike3_color,
        Missile::Spike4 => spike4_color,
        Missile::Spike5 => spike5_color,
        Missile::Firearrow => firearrow_color,
        Missile::CrArrow1 => crarrow1_color,
        Missile::CrArrow2 => crarrow2_color,
        Missile::CrArrow3 => crarrow3_color,
        Missile::CrArrow4 => crarrow4_color,
        Missile::CrArrow5 => crarrow5_color,
        Missile::Blood1 => blood1_color,
        Missile::Blood2 => blood2_color,
        Missile::Bigblood1 => bigblood1_color,
        Missile::Bigblood2 => bigblood2_color,
        Missile::Shafire1 => shafire1_color,
        Missile::Shafire2 => shafire2_color,
        Missile::Shafire3 => shafire3_color,
        Missile::Shafire4 => shafire4_color,
        Missile::Shafire5 => shafire5_color,
        Missile::Magicarrow => magicarrow_color,
        Missile::Icearrow => icearrow_color,
        Missile::Fireexplode => fireexplode_color,
        Missile::Iceexplode => iceexplode_color,
        Missile::Bolt => bolt_color,
        Missile::Andarielspray => andarielspray_color,
        Missile::Bigheadexp => bigheadexp_color,
        Missile::Shamanexp => shamanexp_color,
        Missile::Throwaxe => throwaxe_color,
        Missile::Throwknife => throwknife_color,
        Missile::Glaive => glaive_color,
        Missile::Poisonjav => poisonjav_color,
        Missile::Poisonjavcloud => poisonjavcloud_color,
        Missile::Coldarrow => coldarrow_color,
        Missile::Explodingarrow => explodingarrow_color,
        Missile::Explodingarrowexp => explodingarrowexp_color,
        Missile::Plaguejavelin => plaguejavelin_color,
        Missile::Oilpotion => oilpotion_color,
        Missile::Explosivepotion => explosivepotion_color,
        Missile::Fulminatingpotion => fulminatingpotion_color,
        Missile::Rancidgasepotion => rancidgasepotion_color,
        Missile::Chokinggaspoition => chokinggaspoition_color,
        Missile::Stranglinggaspotion => stranglinggaspotion_color,
        Missile::Notused50 => notused50_color,
        Missile::Explosivepotionexp => explosivepotionexp_color,
        Missile::Explosivepotiondebris1 => explosivepotiondebris1_color,
        Missile::Explosivepotiondebris2 => explosivepotiondebris2_color,
        Missile::Explosivepotiondebris3 => explosivepotiondebris3_color,
        Missile::Holybolt => holybolt_color,
        Missile::Chargedbolt => chargedbolt_color,
        Missile::Sanctuarybolt => sanctuarybolt_color,
        Missile::Firebolt => firebolt_color,
        Missile::Icebolt => icebolt_color,
        Missile::Infernoflame1 => infernoflame1_color,
        Missile::Infernoflame2 => infernoflame2_color,
        Missile::Fireball => fireball_color,
        Missile::Mummy1 => mummy1_color,
        Missile::Mummy2 => mummy2_color,
        Missile::Mummy3 => mummy3_color,
        Missile::Mummy4 => mummy4_color,
        Missile::Blaze => blaze_color,
        Missile::Firewallmaker => firewallmaker_color,
        Missile::Firewall => firewall_color,
        Missile::Goospit1 => goospit1_color,
        Missile::Goospit2 => goospit2_color,
        Missile::Goospit3 => goospit3_color,
        Missile::Goospit4 => goospit4_color,
        Missile::Goospit5 => goospit5_color,
        Missile::Goosplat => goosplat_color,
        Missile::SandPile => sandpile_color,
        Missile::Unholybolt1 => unholybolt1_color,
        Missile::Unholybolt2 => unholybolt2_color,
        Missile::Unholybolt3 => unholybolt3_color,
        Missile::Unholybolt4 => unholybolt4_color,
        Missile::Sanctuarycenter => sanctuarycenter_color,
        Missile::Fireexplosion => fireexplosion_color,
        Missile::Stuckarrow => stuckarrow_color,
        Missile::Footprint => footprint_color,
        Missile::Immolationarrow => immolationarrow_color,
        Missile::Guidedarrow => guidedarrow_color,
        Missile::Freezingarrow => freezingarrow_color,
        Missile::Freezingarrowexp1 => freezingarrowexp1_color,
        Missile::Freezingarrowexp2 => freezingarrowexp2_color,
        Missile::Nova => nova_color,
        Missile::Iceblast => iceblast_color,
        Missile::Blessedhammer => blessedhammer_color,
        Missile::Chainlightning => chainlightning_color,
        Missile::Fistofares => fistofares_color,
        Missile::Chillblood => chillblood_color,
        Missile::Glacialspike => glacialspike_color,
        Missile::Teleport => teleport_color,
        Missile::Lightningbolt => lightningbolt_color,
        Missile::Lightninghit => lightninghit_color,
        Missile::Meteor => meteor_color,
        Missile::Meteorcenter => meteorcenter_color,
        Missile::Meteortail => meteortail_color,
        Missile::Meteorexplode => meteorexplode_color,
        Missile::Firesmall => firesmall_color,
        Missile::Firemedium => firemedium_color,
        Missile::Monblizcenter => monblizcenter_color,
        Missile::Monbliz1 => monbliz1_color,
        Missile::Monbliz2 => monbliz2_color,
        Missile::Monbliz3 => monbliz3_color,
        Missile::Monbliz4 => monbliz4_color,
        Missile::Monblizexplode1 => monblizexplode1_color,
        Missile::Monblizexplode2 => monblizexplode2_color,
        Missile::Monblizexplode3 => monblizexplode3_color,
        Missile::Teeth => teeth_color,
        Missile::Corpseexplosion => corpseexplosion_color,
        Missile::Poisoncorpseexplosion => poisoncorpseexplosion_color,
        Missile::Monstercorpseexplode => monstercorpseexplode_color,
        Missile::Poisonnova => poisonnova_color,
        Missile::Frostnova => frostnova_color,
        Missile::Rogue1 => rogue1_color,
        Missile::Rogue2 => rogue2_color,
        Missile::Rogue3 => rogue3_color,
        Missile::BatLightningBolt => batlightningbolt_color,
        Missile::BatLightningTrail => batlightningtrail_color,
        Missile::Skmage1 => skmage1_color,
        Missile::Skmage2 => skmage2_color,
        Missile::Skmage3 => skmage3_color,
        Missile::Skmage4 => skmage4_color,
        Missile::Vampirefireball => vampirefireball_color,
        Missile::Vampirefirewallmaker => vampirefirewallmaker_color,
        Missile::Vampirefirewall => vampirefirewall_color,
        Missile::Vampiremeteor => vampiremeteor_color,
        Missile::Vampiremeteorcenter => vampiremeteorcenter_color,
        Missile::Vampiremeteorexp => vampiremeteorexp_color,
        Missile::Raven1 => raven1_color,
        Missile::Raven2 => raven2_color,
        Missile::Amphibiangoo1 => amphibiangoo1_color,
        Missile::Amphibiangoo2 => amphibiangoo2_color,
        Missile::Tentaclegoo => tentaclegoo_color,
        Missile::Amphibianexplode => amphibianexplode_color,
        Missile::Poisonpuff => poisonpuff_color,
        Missile::Curseeffectred => curseeffectred_color,
        Missile::Spidergoolay => spidergoolay_color,
        Missile::Fetishinferno1 => fetishinferno1_color,
        Missile::Fetishinferno2 => fetishinferno2_color,
        Missile::Spidergoo => spidergoo_color,
        Missile::Cursecast => cursecast_color,
        Missile::Howl => howl_color,
        Missile::Shout => shout_color,
        Missile::Dust => dust_color,
        Missile::Redlightmissile => redlightmissile_color,
        Missile::Greenlightmissile => greenlightmissile_color,
        Missile::Bluelightmissile => bluelightmissile_color,
        Missile::Whitelightmissile => whitelightmissile_color,
        Missile::Corpsepoisoncloud => corpsepoisoncloud_color,
        Missile::Chillbloodcloud => chillbloodcloud_color,
        Missile::Chillbloodpuff => chillbloodpuff_color,
        Missile::Blizzardcenter => blizzardcenter_color,
        Missile::Blizzard1 => blizzard1_color,
        Missile::Blizzard2 => blizzard2_color,
        Missile::Blizzard3 => blizzard3_color,
        Missile::Blizzard4 => blizzard4_color,
        Missile::Blizzardexplode1 => blizzardexplode1_color,
        Missile::Blizzardexplode2 => blizzardexplode2_color,
        Missile::Blizzardexplode3 => blizzardexplode3_color,
        Missile::Thunderstorm1 => thunderstorm1_color,
        Missile::Thunderstorm2 => thunderstorm2_color,
        Missile::Thunderstorm3 => thunderstorm3_color,
        Missile::Thunderstorm4 => thunderstorm4_color,
        Missile::Monsterlight => monsterlight_color,
        Missile::Poisonball => poisonball_color,
        Missile::Diablight => diablight_color,
        Missile::Redemption => redemption_color,
        Missile::Redemptionfail => redemptionfail_color,
        Missile::Handofgod => handofgod_color,
        Missile::Diabfire => diabfire_color,
        Missile::Fingermagespider => fingermagespider_color,
        Missile::ElectricThrowaxe => electricthrowaxe_color,
        Missile::Diabwallmaker => diabwallmaker_color,
        Missile::Diabwall => diabwall_color,
        Missile::Curseamplifydamage => curseamplifydamage_color,
        Missile::Cursedimvision => cursedimvision_color,
        Missile::Curseweaken => curseweaken_color,
        Missile::Curseironmaiden => curseironmaiden_color,
        Missile::Curseterror => curseterror_color,
        Missile::Curseattract => curseattract_color,
        Missile::Cursereversevampire => cursereversevampire_color,
        Missile::Curseconfuse => curseconfuse_color,
        Missile::Cursedecrepify => cursedecrepify_color,
        Missile::Curselowerresist => curselowerresist_color,
        Missile::Cursecenter => cursecenter_color,
        Missile::Bonespear => bonespear_color,
        Missile::Bonespirit => bonespirit_color,
        Missile::Coldunique => coldunique_color,
        Missile::Lightunique => lightunique_color,
        Missile::Skbowarrow1 => skbowarrow1_color,
        Missile::Skbowarrow2 => skbowarrow2_color,
        Missile::Skbowarrow3 => skbowarrow3_color,
        Missile::Skbowarrow4 => skbowarrow4_color,
        Missile::Skbowarrow5 => skbowarrow5_color,
        Missile::Nova1 => nova1_color,
        Missile::Nova2 => nova2_color,
        Missile::Andypoisonbolt => andypoisonbolt_color,
        Missile::Teethexplode => teethexplode_color,
        Missile::Lightningjavelin => lightningjavelin_color,
        Missile::Lightningfury => lightningfury_color,
        Missile::Bonewallmaker => bonewallmaker_color,
        Missile::Necromage1 => necromage1_color,
        Missile::Necromage2 => necromage2_color,
        Missile::Necromage3 => necromage3_color,
        Missile::Necromage4 => necromage4_color,
        Missile::Sparkle => sparkle_color,
        Missile::Multipleshotarrow => multipleshotarrow_color,
        Missile::Multipleshotbolt => multipleshotbolt_color,
        Missile::Chargedstrikebolt => chargedstrikebolt_color,
        Missile::Bonespearexplode => bonespearexplode_color,
        Missile::Poisonexplosioncloud => poisonexplosioncloud_color,
        Missile::Bonecast => bonecast_color,
        Missile::Battlecry => battlecry_color,
        Missile::Primepoisoncloud => primepoisoncloud_color,
        Missile::Plaguejavcloud => plaguejavcloud_color,
        Missile::Rancidgascloud => rancidgascloud_color,
        Missile::Chokinggascloud => chokinggascloud_color,
        Missile::Stranglinggascloud => stranglinggascloud_color,
        Missile::Buglightning => buglightning_color,
        Missile::Pantherjav1 => pantherjav1_color,
        Missile::Pantherjav2 => pantherjav2_color,
        Missile::Pantherjav3 => pantherjav3_color,
        Missile::Pantherjav4 => pantherjav4_color,
        Missile::Immolationfire => immolationfire_color,
        Missile::Furylightning => furylightning_color,
        Missile::Lightningstrike => lightningstrike_color,
        Missile::Fistoftheheavensdelay => fistoftheheavensdelay_color,
        Missile::Fistoftheheavensbolt => fistoftheheavensbolt_color,
        Missile::Warcry => warcry_color,
        Missile::Battlecommand => battlecommand_color,
        Missile::Battleorders => battleorders_color,
        Missile::Pantherpotorange => pantherpotorange_color,
        Missile::Pantherpotgreen => pantherpotgreen_color,
        Missile::Meteorfire => meteorfire_color,
        Missile::TrapSpikeRight => trapspikeright_color,
        Missile::TrapSpikeLeft => trapspikeleft_color,
        Missile::TrapCursedSkullRight => trapcursedskullright_color,
        Missile::TrapCursedSkullLeft => trapcursedskullleft_color,
        Missile::TrapPoisonBallRight => trappoisonballright_color,
        Missile::TrapPoisonBallLeft => trappoisonballleft_color,
        Missile::Hydra => hydra_color,
        Missile::Bonespeartrail => bonespeartrail_color,
        Missile::Grimwardsmallstart => grimwardsmallstart_color,
        Missile::Grimwardsmall => grimwardsmall_color,
        Missile::Grimwardsmallstop => grimwardsmallstop_color,
        Missile::Grimwardmediumstart => grimwardmediumstart_color,
        Missile::Grimwardmedium => grimwardmedium_color,
        Missile::Grimwardmediumstop => grimwardmediumstop_color,
        Missile::Grimwardlargestart => grimwardlargestart_color,
        Missile::Grimwardlarge => grimwardlarge_color,
        Missile::Grimwardlargestop => grimwardlargestop_color,
        Missile::Zakarumlight => zakarumlight_color,
        Missile::Grimwardscare => grimwardscare_color,
        Missile::Frozenorb => frozenorb_color,
        Missile::Frozenorbbolt => frozenorbbolt_color,
        Missile::Frozenorbnova => frozenorbnova_color,
        Missile::Frozenorbexplode => frozenorbexplode_color,
        Missile::Chillingarmorbolt => chillingarmorbolt_color,
        Missile::Fireexplosion2 => fireexplosion2_color,
        Missile::Blowgun => blowgun_color,
        Missile::Chainlightning2 => chainlightning2_color,
        Missile::Revivesmall => revivesmall_color,
        Missile::Revivemedium => revivemedium_color,
        Missile::Revivelarge => revivelarge_color,
        Missile::Monglacialspike => monglacialspike_color,
        Missile::Icebreaksmall => icebreaksmall_color,
        Missile::Icebreakmedium => icebreakmedium_color,
        Missile::Icebreaklarge => icebreaklarge_color,
        Missile::Icebreaksmoke => icebreaksmoke_color,
        Missile::Mephisto => mephisto_color,
        Missile::Firehead => firehead_color,
        Missile::Whilrwind => whilrwind_color,
        Missile::Arcanelightningbolt => arcanelightningbolt_color,
        Missile::Frogfire => frogfire_color,
        Missile::Frogcold => frogcold_color,
        Missile::Frogpois => frogpois_color,
        Missile::Desertfireball => desertfireball_color,
        Missile::Brdeathcontrol => brdeathcontrol_color,
        Missile::Brdeathlightningbolt => brdeathlightningbolt_color,
        Missile::Brdeathlightninghit => brdeathlightninghit_color,
        Missile::Denofevillight => denofevillight_color,
        Missile::Cairnstones => cairnstones_color,
        Missile::Cairnstonessky => cairnstonessky_color,
        Missile::Cairnstonesground => cairnstonesground_color,
        Missile::Towermist => towermist_color,
        Missile::Towermisttrail => towermisttrail_color,
        Missile::Brdeathsmokes1 => brdeathsmokes1_color,
        Missile::Brdeathsmokenu => brdeathsmokenu_color,
        Missile::Brdeathsmokedt => brdeathsmokedt_color,
        Missile::Brdeathspirits1 => brdeathspirits1_color,
        Missile::Brdeathspiritnu => brdeathspiritnu_color,
        Missile::Brdeathspiritdt => brdeathspiritdt_color,
        Missile::Mephistodeathcontrol => mephistodeathcontrol_color,
        Missile::Mephistofirewallmaker => mephistofirewallmaker_color,
        Missile::Mephistofirewall => mephistofirewall_color,
        Missile::Mephistoflyingrocksbig => mephistoflyingrocksbig_color,
        Missile::Mephistoexplosionbig => mephistoexplosionbig_color,
        Missile::Mephistoflyingrockssmall => mephistoflyingrockssmall_color,
        Missile::Mephistoexplosionsmall => mephistoexplosionsmall_color,
        Missile::Mephistodonotdraw => mephistodonotdraw_color,
        Missile::Andycontrol0 => andycontrol0_color,
        Missile::Andyfirewallmaker => andyfirewallmaker_color,
        Missile::Andyfirewall => andyfirewall_color,
        Missile::Andycolumnfirebase => andycolumnfirebase_color,
        Missile::Andycolumnfire => andycolumnfire_color,
        Missile::Andyfallingdebris1 => andyfallingdebris1_color,
        Missile::Andyfallingdebris2 => andyfallingdebris2_color,
        Missile::Andyfallingdebris3 => andyfallingdebris3_color,
        Missile::Andydebrisexplosion1 => andydebrisexplosion1_color,
        Missile::Andydebrisexplosion2 => andydebrisexplosion2_color,
        Missile::Andydebrisexplosion3 => andydebrisexplosion3_color,
        Missile::Andydebrisexplosion4 => andydebrisexplosion4_color,
        Missile::Andydebrisexplosion5 => andydebrisexplosion5_color,
        Missile::Willowisplightningbolt => willowisplightningbolt_color,
        Missile::Queenpoisoncloud => queenpoisoncloud_color,
        Missile::DirtPile => dirtpile_color,
        Missile::Undeadmissile1 => undeadmissile1_color,
        Missile::Undeadmissile2 => undeadmissile2_color,
        Missile::Undeadmissile3 => undeadmissile3_color,
        Missile::Undeadmissile4 => undeadmissile4_color,
        Missile::Bonespiritexplode => bonespiritexplode_color,
        Missile::Dopplezonexplode => dopplezonexplode_color,
        Missile::Monbonespirit => monbonespirit_color,
        Missile::Towermistfade => towermistfade_color,
        Missile::Countessfirewall => countessfirewall_color,
        Missile::Towerchestspawner => towerchestspawner_color,
        Missile::Hellmeteorlaunch1 => hellmeteorlaunch1_color,
        Missile::Hellmeteorlaunch2 => hellmeteorlaunch2_color,
        Missile::Hellmeteorup => hellmeteorup_color,
        Missile::Hellmeteordown => hellmeteordown_color,
        Missile::Hellmeteorball => hellmeteorball_color,
        Missile::Horadricstaff => horadricstaff_color,
        Missile::Horadriclightning => horadriclightning_color,
        Missile::Horadriclight => horadriclight_color,
        Missile::Regurgitatorcorpse => regurgitatorcorpse_color,
        Missile::Regurgitatorcorpseexpl => regurgitatorcorpseexpl_color,
        Missile::Highpriestlightning => highpriestlightning_color,
        Missile::Icebreaksmallmelt => icebreaksmallmelt_color,
        Missile::Icebreaklargemelt => icebreaklargemelt_color,
        Missile::Leapknockback => leapknockback_color,
        Missile::Radamentdeath => radamentdeath_color,
        Missile::Radamenthandofgod => radamenthandofgod_color,
        Missile::Radamentholybolt => radamentholybolt_color,
        Missile::Taintedsuncontrol => taintedsuncontrol_color,
        Missile::Taintedsunflash => taintedsunflash_color,
        Missile::Taintedsunball => taintedsunball_color,
        Missile::Queendeathcenter => queendeathcenter_color,
        Missile::Queendeathglob => queendeathglob_color,
        Missile::Queendeathsplat1 => queendeathsplat1_color,
        Missile::Queendeathsplat2 => queendeathsplat2_color,
        Missile::Healingbolt => healingbolt_color,
        Missile::Mephistoholedelay => mephistoholedelay_color,
        Missile::Mephistoholebirth => mephistoholebirth_color,
        Missile::Mephistoholeneutral => mephistoholeneutral_color,
        Missile::Mephistoholedeath => mephistoholedeath_color,
        Missile::Mephistoholedead => mephistoholedead_color,
        Missile::Durieldeathcontrol => durieldeathcontrol_color,
        Missile::Durieldeathrock => durieldeathrock_color,
        Missile::Durieldeathdebris => durieldeathdebris_color,
        Missile::Durieldeathsmoke => durieldeathsmoke_color,
        Missile::Mephistoexplosion => mephistoexplosion_color,
        Missile::Orbmist => orbmist_color,
        Missile::Orbmisttrail => orbmisttrail_color,
        Missile::Orbmistfade => orbmistfade_color,
        Missile::Pilum => pilum_color,
        Missile::DiabloAppears => diabloappears_color,
        Missile::Hfcontrol => hfcontrol_color,
        Missile::Hffragment1 => hffragment1_color,
        Missile::Hffragment2 => hffragment2_color,
        Missile::Hffragment3 => hffragment3_color,
        Missile::Hfspirit1 => hfspirit1_color,
        Missile::Hfreserved3 => hfreserved3_color,
        Missile::IzualControl => izualcontrol_color,
        Missile::IzualMistLoop => izualmistloop_color,
        Missile::IzualMistFade => izualmistfade_color,
        Missile::IzualLightning => izuallightning_color,
        Missile::IzualLightningTrail => izuallightningtrail_color,
        Missile::CairnStonesBolt => cairnstonesbolt_color,
        Missile::BombInAir => bombinair_color,
        Missile::BombOnGround => bombonground_color,
        Missile::BombExplosion => bombexplosion_color,
        Missile::ShockFieldInAir => shockfieldinair_color,
        Missile::ShockFieldOnGround => shockfieldonground_color,
        Missile::Throwingstar => throwingstar_color,
        Missile::Acidspray => acidspray_color,
        Missile::BladeCreeper => bladecreeper_color,
        Missile::Distraction => distraction_color,
        Missile::DistractionFog => distractionfog_color,
        Missile::DistractionPuff => distractionpuff_color,
        Missile::DistractionStart => distractionstart_color,
        Missile::DistractionEnd => distractionend_color,
        Missile::Impinfernoflame1 => impinfernoflame1_color,
        Missile::Impinfernoflame2 => impinfernoflame2_color,
        Missile::Baallightningbolt => baallightningbolt_color,
        Missile::Baallightningtrail => baallightningtrail_color,
        Missile::Baallightningbolt2 => baallightningbolt2_color,
        Missile::Baallightningtrail2 => baallightningtrail2_color,
        Missile::Impfireball => impfireball_color,
        Missile::Impfireballexplode => impfireballexplode_color,
        Missile::CatapultchargedballOn => catapultchargedballon_color,
        Missile::Catapultchargedball => catapultchargedball_color,
        Missile::Catapultchargedballbolt => catapultchargedballbolt_color,
        Missile::ImpSpawnMonsters => impspawnmonsters_color,
        Missile::CatapultSpikeBallOn => catapultspikeballon_color,
        Missile::CatapultSpikeBall => catapultspikeball_color,
        Missile::CatapultSpikeInAir => catapultspikeinair_color,
        Missile::CatapultSpikeOnGround => catapultspikeonground_color,
        Missile::CatapultSpikeExplosion => catapultspikeexplosion_color,
        Missile::CatapultColdBallOn => catapultcoldballon_color,
        Missile::CatapultColdBall => catapultcoldball_color,
        Missile::CatapultColdExplosion => catapultcoldexplosion_color,
        Missile::CatapultPlagueBallOn => catapultplagueballon_color,
        Missile::CatapultPlagueBall => catapultplagueball_color,
        Missile::CatapultPlagueCloud => catapultplaguecloud_color,
        Missile::CatapultMeteorBallOn => catapultmeteorballon_color,
        Missile::CatapultMeteorBall => catapultmeteorball_color,
        Missile::CatapultMeteorFire => catapultmeteorfire_color,
        Missile::TowerDeath => towerdeath_color,
        Missile::HealingVortex => healingvortex_color,
        Missile::Suicidecorpseexplode => suicidecorpseexplode_color,
        Missile::Suicidefireexplode => suicidefireexplode_color,
        Missile::Suicideiceexplode => suicideiceexplode_color,
        Missile::Explodingjavalin => explodingjavalin_color,
        Missile::Explodingjavalinexp => explodingjavalinexp_color,
        Missile::Lightingtrailingjavalin => lightingtrailingjavalin_color,
        Missile::Lightjavalintrail => lightjavalintrail_color,
        Missile::Lightjavalinexplosion => lightjavalinexplosion_color,
        Missile::Icejavalin => icejavalin_color,
        Missile::Icejavalinexplode => icejavalinexplode_color,
        Missile::Plaguejavelin2 => plaguejavelin2_color,
        Missile::Plaguejavlinexplode => plaguejavlinexplode_color,
        Missile::Advlighttrailingjav => advlighttrailingjav_color,
        Missile::Advlighttrailingjav2 => advlighttrailingjav2_color,
        Missile::Advlightjavexplode => advlightjavexplode_color,
        Missile::Sucfireball => sucfireball_color,
        Missile::Sucfireballexplode => sucfireballexplode_color,
        Missile::Sucfireballtrail => sucfireballtrail_color,
        Missile::Sucshockfieldmissile => sucshockfieldmissile_color,
        Missile::Sucshockfieldmissileexp => sucshockfieldmissileexp_color,
        Missile::Sucshockfield => sucshockfield_color,
        Missile::Hellfiremissile => hellfiremissile_color,
        Missile::Hellfireexa => hellfireexa_color,
        Missile::Hellfireexb => hellfireexb_color,
        Missile::ImpChargedBolt => impchargedbolt_color,
        Missile::ImpTeleport => impteleport_color,
        Missile::Moltenboulder => moltenboulder_color,
        Missile::Moltenboulderemerge => moltenboulderemerge_color,
        Missile::Moltenboulderexplode => moltenboulderexplode_color,
        Missile::Moltenboulderfirepath => moltenboulderfirepath_color,
        Missile::MoltenboulderFlyingrocks => moltenboulderflyingrocks_color,
        Missile::Firestorm => firestorm_color,
        Missile::Firestormmaker => firestormmaker_color,
        Missile::Arcticblast1 => arcticblast1_color,
        Missile::Arcticblast2 => arcticblast2_color,
        Missile::ErruptionCenter => erruptioncenter_color,
        Missile::ErruptionCrack1 => erruptioncrack1_color,
        Missile::ErruptionCrack2 => erruptioncrack2_color,
        Missile::ErruptionSmoke1 => erruptionsmoke1_color,
        Missile::ErruptionSmoke2 => erruptionsmoke2_color,
        Missile::VineBeastWalk1 => vinebeastwalk1_color,
        Missile::VineBeastWalk2 => vinebeastwalk2_color,
        Missile::VineBeastNeutral => vinebeastneutral_color,
        Missile::VineBeastAttack => vinebeastattack_color,
        Missile::VineBeastDeath => vinebeastdeath_color,
        Missile::Vines => vines_color,
        Missile::VinesTrail => vinestrail_color,
        Missile::VinesWither => vineswither_color,
        Missile::PlagueVines => plaguevines_color,
        Missile::PlagueVinesTrail => plaguevinestrail_color,
        Missile::PlagueVinesWither => plaguevineswither_color,
        Missile::Twister => twister_color,
        Missile::Tornado => tornado_color,
        Missile::Volcano => volcano_color,
        Missile::VolcanoOverlayFire => volcanooverlayfire_color,
        Missile::VolcanoDebris2 => volcanodebris2_color,
        Missile::VolcanoExplosion => volcanoexplosion_color,
        Missile::VolcanoSmallFire => volcanosmallfire_color,
        Missile::DragonbreathMissile => dragonbreathmissile_color,
        Missile::Lureprojectile => lureprojectile_color,
        Missile::Lurecenter => lurecenter_color,
        Missile::Lurecloud => lurecloud_color,
        Missile::Impmiss1 => impmiss1_color,
        Missile::Impmiss2 => impmiss2_color,
        Missile::Impmiss3 => impmiss3_color,
        Missile::Impmiss4 => impmiss4_color,
        Missile::Impmiss5 => impmiss5_color,
        Missile::FrozenhorrorArcticblast1 => frozenhorrorarcticblast1_color,
        Missile::FrozenhorrorArcticblast2 => frozenhorrorarcticblast2_color,
        Missile::Sentrychargedbolt => sentrychargedbolt_color,
        Missile::Sentryspikeinair => sentryspikeinair_color,
        Missile::Sentryspikeonground => sentryspikeonground_color,
        Missile::RecyclerDelay => recyclerdelay_color,
        Missile::RecyclerVine => recyclervine_color,
        Missile::RecyclerFade => recyclerfade_color,
        Missile::RecyclerExplosion => recyclerexplosion_color,
        Missile::DeathMauler => deathmauler_color,
        Missile::DeathMaulerTrail => deathmaulertrail_color,
        Missile::DeathMaulerTrailFade => deathmaulertrailfade_color,
        Missile::Bladefury1 => bladefury1_color,
        Missile::Bladefragment1 => bladefragment1_color,
        Missile::Bladefury2 => bladefury2_color,
        Missile::Bladefragment2 => bladefragment2_color,
        Missile::Bladefury3 => bladefury3_color,
        Missile::Bladefragment3 => bladefragment3_color,
        Missile::Shockwave => shockwave_color,
        Missile::Lightningtalons => lightningtalons_color,
        Missile::Lightningtalonstrail => lightningtalonstrail_color,
        Missile::Phoenixtrail => phoenixtrail_color,
        Missile::Rabiesplague => rabiesplague_color,
        Missile::Rabiescontagion => rabiescontagion_color,
        Missile::WakeOfDestructionMaker => wakeofdestructionmaker_color,
        Missile::WakeOfDestruction => wakeofdestruction_color,
        Missile::Deathsentryexplode => deathsentryexplode_color,
        Missile::Tigerfury => tigerfury_color,
        Missile::Tigerfurytrail => tigerfurytrail_color,
        Missile::Tigerfurytrail2 => tigerfurytrail2_color,
        Missile::InfernoSentry1 => infernosentry1_color,
        Missile::InfernoSentry2 => infernosentry2_color,
        Missile::AncientThrowingAxe => ancientthrowingaxe_color,
        Missile::Sentrylightningbolt => sentrylightningbolt_color,
        Missile::Sentrylightninghit => sentrylightninghit_color,
        Missile::AnyaCenter => anyacenter_color,
        Missile::AnyaIcicle => anyaicicle_color,
        Missile::AnyaIceimpact => anyaiceimpact_color,
        Missile::AnyaIcesteam => anyaicesteam_color,
        Missile::AnyaIcemagic => anyaicemagic_color,
        Missile::DragontailMissile => dragontailmissile_color,
        Missile::Dragonflight => dragonflight_color,
        Missile::Dragonflightmaker => dragonflightmaker_color,
        Missile::ProgressiveRadiusDamage => progressiveradiusdamage_color,
        Missile::VineBeastWalk1Fade => vinebeastwalk1fade_color,
        Missile::VineBeastWalk2Fade => vinebeastwalk2fade_color,
        Missile::VineBeastNeutralFade => vinebeastneutralfade_color,
        Missile::VineRecyclerDelay => vinerecyclerdelay_color,
        Missile::AncientDeathCenter => ancientdeathcenter_color,
        Missile::AncientDeathCloud => ancientdeathcloud_color,
        Missile::LightningChargeUpNova => lightningchargeupnova_color,
        Missile::ChainlightningchargeUp => chainlightningchargeup_color,
        Missile::PainWormAppear => painwormappear_color,
        Missile::BaalTauntControl => baaltauntcontrol_color,
        Missile::BaalTauntLightning => baaltauntlightning_color,
        Missile::BaalTauntLightningTrail => baaltauntlightningtrail_color,
        Missile::BaalTauntPoison => baaltauntpoison_color,
        Missile::BaalSpawnMonsters => baalspawnmonsters_color,
        Missile::MindblastHit => mindblasthit_color,
        Missile::BladeShieldMissile => bladeshieldmissile_color,
        Missile::BladeShieldAttachment => bladeshieldattachment_color,
        Missile::BaalInferno => baalinferno_color,
        Missile::BaalNova => baalnova_color,
        Missile::Fistsoffireexplode => fistsoffireexplode_color,
        Missile::Fistsoffirefirewall => fistsoffirefirewall_color,
        Missile::Clawsofthunderbolt => clawsofthunderbolt_color,
        Missile::Clawsofthundernova => clawsofthundernova_color,
        Missile::Bladesoficeexplode => bladesoficeexplode_color,
        Missile::Bladesoficecubes => bladesoficecubes_color,
        Missile::Bladesoficecubesmelt => bladesoficecubesmelt_color,
        Missile::Royalstrikemeteor => royalstrikemeteor_color,
        Missile::Royalstrikemeteorcenter => royalstrikemeteorcenter_color,
        Missile::Royalstrikemeteortail => royalstrikemeteortail_color,
        Missile::Royalstrikemeteorexplode => royalstrikemeteorexplode_color,
        Missile::Royalstrikemeteorfire => royalstrikemeteorfire_color,
        Missile::Royalstrikechainlightning => royalstrikechainlightning_color,
        Missile::Royalstrikechaosice => royalstrikechaosice_color,
        Missile::WorldStoneChip1 => worldstonechip1_color,
        Missile::WorldStoneChip2 => worldstonechip2_color,
        Missile::WorldStoneChip3 => worldstonechip3_color,
        Missile::WorldStoneChip4 => worldstonechip4_color,
        Missile::Highpriestlightning2 => highpriestlightning2_color,
        Missile::Infernoflame3 => infernoflame3_color,
        Missile::MindblastCenter => mindblastcenter_color,
        Missile::Armageddoncontrol => armageddoncontrol_color,
        Missile::Armageddonrock => armageddonrock_color,
        Missile::Armageddontail => armageddontail_color,
        Missile::Armageddonexplosion => armageddonexplosion_color,
        Missile::Hurricaneswoosh => hurricaneswoosh_color,
        Missile::Hurricanecart => hurricanecart_color,
        Missile::Hurricanerock => hurricanerock_color,
        Missile::Hurricanesack => hurricanesack_color,
        Missile::Hurricanetree => hurricanetree_color,
        Missile::Hurricanevase => hurricanevase_color,
        Missile::Baalcorpseexplodedelay => baalcorpseexplodedelay_color,
        Missile::Baalcorpseexplodeexpl => baalcorpseexplodeexpl_color,
        Missile::BaalColdMaker => baalcoldmaker_color,
        Missile::BaalColdTrail => baalcoldtrail_color,
        Missile::BaalSpawnMonstersExp => baalspawnmonstersexp_color,
        Missile::Impmiss21 => impmiss21_color,
        Missile::Impmiss22 => impmiss22_color,
        Missile::Impmiss23 => impmiss23_color,
        Missile::Impmiss24 => impmiss24_color,
        Missile::Impmiss25 => impmiss25_color,
        Missile::Anyasteam1 => anyasteam1_color,
        Missile::Anyasteam2 => anyasteam2_color,
        Missile::Ancientsguide => ancientsguide_color,
        Missile::Ancientsmarker => ancientsmarker_color,
        Missile::Ancientscontrol => ancientscontrol_color,
        Missile::Overseercontrol => overseercontrol_color,
        Missile::Nihlithak1 => nihlithak1_color,
        Missile::Nihlithak2 => nihlithak2_color,
        Missile::Nihlithak3 => nihlithak3_color,
        Missile::Nihlathakcontrol => nihlathakcontrol_color,
        Missile::Nihlathakswoosh => nihlathakswoosh_color,
        Missile::Nihlathakdebris1 => nihlathakdebris1_color,
        Missile::Nihlathakdebris2 => nihlathakdebris2_color,
        Missile::Nihlathakdebris3 => nihlathakdebris3_color,
        Missile::Nihlathakdebris4 => nihlathakdebris4_color,
        Missile::Nihlathakglow => nihlathakglow_color,
        Missile::Baalteleport => baalteleport_color,
        Missile::Baalclonedeath => baalclonedeath_color,
        Missile::Anyasteamvent => anyasteamvent_color,
        Missile::Anyasteam => anyasteam_color,
        Missile::Nihlathakhole => nihlathakhole_color,
        Missile::Nihlathakholelight => nihlathakholelight_color,
        Missile::Volcanofiretrail => volcanofiretrail_color,
        Missile::Nihlathakglow2 => nihlathakglow2_color,
        Missile::Nihlathakbonechips => nihlathakbonechips_color,
        Missile::Baalcorpseexplodefade => baalcorpseexplodefade_color,
        Missile::Armageddonfire => armageddonfire_color,
        Missile::Icesparkle => icesparkle_color,
        Missile::BaalfxControl => baalfxcontrol_color,
        Missile::BaalfxSpirit1 => baalfxspirit1_color,
        Missile::BaalfxSpirit2 => baalfxspirit2_color,
        Missile::BaalfxSpirit3 => baalfxspirit3_color,
        Missile::BaalfxSpirit4 => baalfxspirit4_color,
        Missile::BaalfxSpirit5 => baalfxspirit5_color,
        Missile::BaalfxBaalHeadAppear => baalfxbaalheadappear_color,
        Missile::BaalfxBaalHead1 => baalfxbaalhead1_color,
        Missile::BaalfxBaalHead2 => baalfxbaalhead2_color,
        Missile::BaalfxBaalHead3 => baalfxbaalhead3_color,
        Missile::BaalfxTyrealDebris1 => baalfxtyrealdebris1_color,
        Missile::BaalfxTyrealDebris2 => baalfxtyrealdebris2_color,
        Missile::BaalfxTyrealDebris3 => baalfxtyrealdebris3_color,
        Missile::BaalfxTyrealDebrisBreak => baalfxtyrealdebrisbreak_color,
        Missile::WorldstoneShake => worldstoneshake_color,
        Missile::Blessedhammerex => blessedhammerex_color,
        Missile::Sentrylightningbolt2 => sentrylightningbolt2_color,
        Missile::Sentrylightninghit2 => sentrylightninghit2_color,
        Missile::Lightningtowernova => lightningtowernova_color,
        Missile::Skbowarrow6 => skbowarrow6_color,
        Missile::Skbowarrow7 => skbowarrow7_color,
        Missile::Skbowarrow8 => skbowarrow8_color,
        Missile::Bighead6 => bighead6_color,
        Missile::ViperPoisjav => viperpoisjav_color,
        Missile::ViperPoisjavcloud => viperpoisjavcloud_color,
        Missile::ViperFire => viperfire_color,
        Missile::ViperFirecloud => viperfirecloud_color,
        Missile::ViperBonespear => viperbonespear_color,
        Missile::Countessfirewallmaker => countessfirewallmaker_color,
        Missile::BaalTauntLightningControl => baaltauntlightningcontrol_color,
        Missile::BaalTauntPoisonControl => baaltauntpoisoncontrol_color,
        Missile::Explodingarrowexp2 => explodingarrowexp2_color,
        Missile::Freezingarrowexp3 => freezingarrowexp3_color,
        Missile::Pantherjav5 => pantherjav5_color,
        Missile::Spike6 => spike6_color,
        Missile::CrArrow6 => crarrow6_color,
        Missile::Skmagepois => skmagepois_color,
        Missile::Skmagecold => skmagecold_color,
        Missile::Skmagefire => skmagefire_color,
        Missile::Skmageltng => skmageltng_color,
        Missile::Succubusmiss => succubusmiss_color,
        Missile::Willowisplightningbolt2 => willowisplightningbolt2_color,
        Missile::Mummyex => mummyex_color,
        Missile::Goospitex => goospitex_color,
        Missile::Impmissex => impmissex_color,
        Missile::Diablogeddoncontrol => diablogeddoncontrol_color,
        Missile::Diablogeddonrock => diablogeddonrock_color,
        Missile::Diablogeddontail => diablogeddontail_color,
        Missile::Diablogeddonexplosion => diablogeddonexplosion_color,
        Missile::Diablogeddonfire => diablogeddonfire_color,
        Missile::Megademoninferno => megademoninferno_color,
        Missile::Trapfirebolt => trapfirebolt_color,
        Missile::Trappoisonjavcloud => trappoisonjavcloud_color,
        Missile::Trapnova => trapnova_color,
        Missile::Mephfrostnova => mephfrostnova_color,
        Missile::Mephlight => mephlight_color,
        Missile::Vampiremeteorfire => vampiremeteorfire_color,
        Missile::Strafearrow => strafearrow_color,
        Missile::Strafebolt => strafebolt_color,
        Missile::Recklessattacksmissile => recklessattacksmissile_color,
        Missile::Unknown => unknown_color,
    }
}

fn get_missile_type(txt_file_no: &Missile) -> MissileType {
    match txt_file_no {
        Missile::Arrow => MissileType::Physical,
        Missile::Javelin => MissileType::Physical,
        Missile::Bighead1 => MissileType::Lightning,
        Missile::Bighead2 => MissileType::Lightning,
        Missile::Bighead3 => MissileType::Lightning,
        Missile::Bighead4 => MissileType::Lightning,
        Missile::Bighead5 => MissileType::Lightning,
        Missile::Spike1 => MissileType::Physical,
        Missile::Spike2 => MissileType::Physical,
        Missile::Spike3 => MissileType::Physical,
        Missile::Spike4 => MissileType::Physical,
        Missile::Spike5 => MissileType::Physical,
        Missile::Firearrow => MissileType::Fire,
        Missile::CrArrow1 => MissileType::Physical,
        Missile::CrArrow2 => MissileType::Physical,
        Missile::CrArrow3 => MissileType::Physical,
        Missile::CrArrow4 => MissileType::Physical,
        Missile::CrArrow5 => MissileType::Physical,
        Missile::Blood1 => MissileType::Sfx,
        Missile::Blood2 => MissileType::Sfx,
        Missile::Bigblood1 => MissileType::Sfx,
        Missile::Bigblood2 => MissileType::Sfx,
        Missile::Shafire1 => MissileType::Fire,
        Missile::Shafire2 => MissileType::Fire,
        Missile::Shafire3 => MissileType::Fire,
        Missile::Shafire4 => MissileType::Fire,
        Missile::Shafire5 => MissileType::Fire,
        Missile::Magicarrow => MissileType::Magic,
        Missile::Icearrow => MissileType::Ice,
        Missile::Fireexplode => MissileType::Fire,
        Missile::Iceexplode => MissileType::Ice,
        Missile::Bolt => MissileType::Physical,
        Missile::Andarielspray => MissileType::Poison,
        Missile::Bigheadexp => MissileType::Sfx,
        Missile::Shamanexp => MissileType::Fire,
        Missile::Throwaxe => MissileType::Physical,
        Missile::Throwknife => MissileType::Physical,
        Missile::Glaive => MissileType::Physical,
        Missile::Poisonjav => MissileType::Poison,
        Missile::Poisonjavcloud => MissileType::Poison,
        Missile::Coldarrow => MissileType::Ice,
        Missile::Explodingarrow => MissileType::Fire,
        Missile::Explodingarrowexp => MissileType::Fire,
        Missile::Plaguejavelin => MissileType::Poison,
        Missile::Oilpotion => MissileType::Fire,
        Missile::Explosivepotion => MissileType::Fire,
        Missile::Fulminatingpotion => MissileType::Fire,
        Missile::Rancidgasepotion => MissileType::Poison,
        Missile::Chokinggaspoition => MissileType::Poison,
        Missile::Stranglinggaspotion => MissileType::Poison,
        Missile::Notused50 => MissileType::FxTrigger,
        Missile::Explosivepotionexp => MissileType::Fire,
        Missile::Explosivepotiondebris1 => MissileType::Fire,
        Missile::Explosivepotiondebris2 => MissileType::Fire,
        Missile::Explosivepotiondebris3 => MissileType::Fire,
        Missile::Holybolt => MissileType::Magic,
        Missile::Chargedbolt => MissileType::Lightning,
        Missile::Sanctuarybolt => MissileType::Lightning,
        Missile::Firebolt => MissileType::Fire,
        Missile::Icebolt => MissileType::Ice,
        Missile::Infernoflame1 => MissileType::Fire,
        Missile::Infernoflame2 => MissileType::Fire,
        Missile::Fireball => MissileType::Fire,
        Missile::Mummy1 => MissileType::Poison,
        Missile::Mummy2 => MissileType::Poison,
        Missile::Mummy3 => MissileType::Poison,
        Missile::Mummy4 => MissileType::Poison,
        Missile::Blaze => MissileType::Fire,
        Missile::Firewallmaker => MissileType::Fire,
        Missile::Firewall => MissileType::Fire,
        Missile::Goospit1 => MissileType::Poison,
        Missile::Goospit2 => MissileType::Poison,
        Missile::Goospit3 => MissileType::Poison,
        Missile::Goospit4 => MissileType::Poison,
        Missile::Goospit5 => MissileType::Poison,
        Missile::Goosplat => MissileType::Poison,
        Missile::SandPile => MissileType::Sfx,
        Missile::Unholybolt1 => MissileType::Magic,
        Missile::Unholybolt2 => MissileType::Magic,
        Missile::Unholybolt3 => MissileType::Magic,
        Missile::Unholybolt4 => MissileType::Magic,
        Missile::Sanctuarycenter => MissileType::Magic,
        Missile::Fireexplosion => MissileType::Fire,
        Missile::Stuckarrow => MissileType::Sfx,
        Missile::Footprint => MissileType::Sfx,
        Missile::Immolationarrow => MissileType::Fire,
        Missile::Guidedarrow => MissileType::Magic,
        Missile::Freezingarrow => MissileType::Ice,
        Missile::Freezingarrowexp1 => MissileType::Ice,
        Missile::Freezingarrowexp2 => MissileType::Ice,
        Missile::Nova => MissileType::Lightning,
        Missile::Iceblast => MissileType::Ice,
        Missile::Blessedhammer => MissileType::Magic,
        Missile::Chainlightning => MissileType::Lightning,
        Missile::Fistofares => MissileType::FxTrigger,
        Missile::Chillblood => MissileType::Sfx,
        Missile::Glacialspike => MissileType::Ice,
        Missile::Teleport => MissileType::Magic,
        Missile::Lightningbolt => MissileType::Lightning,
        Missile::Lightninghit => MissileType::Lightning,
        Missile::Meteor => MissileType::Fire,
        Missile::Meteorcenter => MissileType::Fire,
        Missile::Meteortail => MissileType::Fire,
        Missile::Meteorexplode => MissileType::Fire,
        Missile::Firesmall => MissileType::Fire,
        Missile::Firemedium => MissileType::Fire,
        Missile::Monblizcenter => MissileType::Ice,
        Missile::Monbliz1 => MissileType::Ice,
        Missile::Monbliz2 => MissileType::Ice,
        Missile::Monbliz3 => MissileType::Ice,
        Missile::Monbliz4 => MissileType::Ice,
        Missile::Monblizexplode1 => MissileType::Ice,
        Missile::Monblizexplode2 => MissileType::Ice,
        Missile::Monblizexplode3 => MissileType::Ice,
        Missile::Teeth => MissileType::Magic,
        Missile::Corpseexplosion => MissileType::Physical,
        Missile::Poisoncorpseexplosion => MissileType::Poison,
        Missile::Monstercorpseexplode => MissileType::Physical,
        Missile::Poisonnova => MissileType::Poison,
        Missile::Frostnova => MissileType::Ice,
        Missile::Rogue1 => MissileType::Physical,
        Missile::Rogue2 => MissileType::Fire,
        Missile::Rogue3 => MissileType::Ice,
        Missile::BatLightningBolt => MissileType::Lightning,
        Missile::BatLightningTrail => MissileType::Lightning,
        Missile::Skmage1 => MissileType::Physical,
        Missile::Skmage2 => MissileType::Physical,
        Missile::Skmage3 => MissileType::Physical,
        Missile::Skmage4 => MissileType::Physical,
        Missile::Vampirefireball => MissileType::Fire,
        Missile::Vampirefirewallmaker => MissileType::Fire,
        Missile::Vampirefirewall => MissileType::Fire,
        Missile::Vampiremeteor => MissileType::Fire,
        Missile::Vampiremeteorcenter => MissileType::Fire,
        Missile::Vampiremeteorexp => MissileType::Fire,
        Missile::Raven1 => MissileType::Physical,
        Missile::Raven2 => MissileType::Fire,
        Missile::Amphibiangoo1 => MissileType::Fire,
        Missile::Amphibiangoo2 => MissileType::Fire,
        Missile::Tentaclegoo => MissileType::Poison,
        Missile::Amphibianexplode => MissileType::Sfx,
        Missile::Poisonpuff => MissileType::Poison,
        Missile::Curseeffectred => MissileType::Magic,
        Missile::Spidergoolay => MissileType::FxTrigger,
        Missile::Fetishinferno1 => MissileType::Fire,
        Missile::Fetishinferno2 => MissileType::Fire,
        Missile::Spidergoo => MissileType::Poison,
        Missile::Cursecast => MissileType::Magic,
        Missile::Howl => MissileType::Magic,
        Missile::Shout => MissileType::Magic,
        Missile::Dust => MissileType::Sfx,
        Missile::Redlightmissile => MissileType::Fire,
        Missile::Greenlightmissile => MissileType::Poison,
        Missile::Bluelightmissile => MissileType::Ice,
        Missile::Whitelightmissile => MissileType::Physical,
        Missile::Corpsepoisoncloud => MissileType::Poison,
        Missile::Chillbloodcloud => MissileType::Sfx,
        Missile::Chillbloodpuff => MissileType::Sfx,
        Missile::Blizzardcenter => MissileType::Ice,
        Missile::Blizzard1 => MissileType::Ice,
        Missile::Blizzard2 => MissileType::Ice,
        Missile::Blizzard3 => MissileType::Ice,
        Missile::Blizzard4 => MissileType::Ice,
        Missile::Blizzardexplode1 => MissileType::Ice,
        Missile::Blizzardexplode2 => MissileType::Ice,
        Missile::Blizzardexplode3 => MissileType::Ice,
        Missile::Thunderstorm1 => MissileType::Lightning,
        Missile::Thunderstorm2 => MissileType::Lightning,
        Missile::Thunderstorm3 => MissileType::Lightning,
        Missile::Thunderstorm4 => MissileType::Lightning,
        Missile::Monsterlight => MissileType::Lightning,
        Missile::Poisonball => MissileType::Poison,
        Missile::Diablight => MissileType::Lightning,
        Missile::Redemption => MissileType::Magic,
        Missile::Redemptionfail => MissileType::Sfx,
        Missile::Handofgod => MissileType::Lightning,
        Missile::Diabfire => MissileType::Fire,
        Missile::Fingermagespider => MissileType::Fire,
        Missile::ElectricThrowaxe => MissileType::Lightning,
        Missile::Diabwallmaker => MissileType::Fire,
        Missile::Diabwall => MissileType::Fire,
        Missile::Curseamplifydamage => MissileType::Magic,
        Missile::Cursedimvision => MissileType::Magic,
        Missile::Curseweaken => MissileType::Magic,
        Missile::Curseironmaiden => MissileType::Magic,
        Missile::Curseterror => MissileType::Magic,
        Missile::Curseattract => MissileType::Magic,
        Missile::Cursereversevampire => MissileType::Magic,
        Missile::Curseconfuse => MissileType::Magic,
        Missile::Cursedecrepify => MissileType::Magic,
        Missile::Curselowerresist => MissileType::Magic,
        Missile::Cursecenter => MissileType::Magic,
        Missile::Bonespear => MissileType::Magic,
        Missile::Bonespirit => MissileType::Magic,
        Missile::Coldunique => MissileType::Ice,
        Missile::Lightunique => MissileType::Lightning,
        Missile::Skbowarrow1 => MissileType::Physical,
        Missile::Skbowarrow2 => MissileType::Physical,
        Missile::Skbowarrow3 => MissileType::Physical,
        Missile::Skbowarrow4 => MissileType::Physical,
        Missile::Skbowarrow5 => MissileType::Physical,
        Missile::Nova1 => MissileType::Lightning,
        Missile::Nova2 => MissileType::Lightning,
        Missile::Andypoisonbolt => MissileType::Poison,
        Missile::Teethexplode => MissileType::Magic,
        Missile::Lightningjavelin => MissileType::Lightning,
        Missile::Lightningfury => MissileType::Lightning,
        Missile::Bonewallmaker => MissileType::Magic,
        Missile::Necromage1 => MissileType::Poison,
        Missile::Necromage2 => MissileType::Ice,
        Missile::Necromage3 => MissileType::Fire,
        Missile::Necromage4 => MissileType::Lightning,
        Missile::Sparkle => MissileType::Poison,
        Missile::Multipleshotarrow => MissileType::Physical,
        Missile::Multipleshotbolt => MissileType::Physical,
        Missile::Chargedstrikebolt => MissileType::Lightning,
        Missile::Bonespearexplode => MissileType::Magic,
        Missile::Poisonexplosioncloud => MissileType::Poison,
        Missile::Bonecast => MissileType::Magic,
        Missile::Battlecry => MissileType::Magic,
        Missile::Primepoisoncloud => MissileType::Poison,
        Missile::Plaguejavcloud => MissileType::Poison,
        Missile::Rancidgascloud => MissileType::Poison,
        Missile::Chokinggascloud => MissileType::Poison,
        Missile::Stranglinggascloud => MissileType::Poison,
        Missile::Buglightning => MissileType::Lightning,
        Missile::Pantherjav1 => MissileType::Physical,
        Missile::Pantherjav2 => MissileType::Physical,
        Missile::Pantherjav3 => MissileType::Physical,
        Missile::Pantherjav4 => MissileType::Physical,
        Missile::Immolationfire => MissileType::Fire,
        Missile::Furylightning => MissileType::Lightning,
        Missile::Lightningstrike => MissileType::Lightning,
        Missile::Fistoftheheavensdelay => MissileType::FxTrigger,
        Missile::Fistoftheheavensbolt => MissileType::Magic,
        Missile::Warcry => MissileType::Magic,
        Missile::Battlecommand => MissileType::Magic,
        Missile::Battleorders => MissileType::Magic,
        Missile::Pantherpotorange => MissileType::Fire,
        Missile::Pantherpotgreen => MissileType::Poison,
        Missile::Meteorfire => MissileType::Fire,
        Missile::TrapSpikeRight => MissileType::Physical,
        Missile::TrapSpikeLeft => MissileType::Physical,
        Missile::TrapCursedSkullRight => MissileType::Magic,
        Missile::TrapCursedSkullLeft => MissileType::Magic,
        Missile::TrapPoisonBallRight => MissileType::Poison,
        Missile::TrapPoisonBallLeft => MissileType::Poison,
        Missile::Hydra => MissileType::Fire,
        Missile::Bonespeartrail => MissileType::Magic,
        Missile::Grimwardsmallstart => MissileType::Sfx,
        Missile::Grimwardsmall => MissileType::Sfx,
        Missile::Grimwardsmallstop => MissileType::Sfx,
        Missile::Grimwardmediumstart => MissileType::Sfx,
        Missile::Grimwardmedium => MissileType::Sfx,
        Missile::Grimwardmediumstop => MissileType::Sfx,
        Missile::Grimwardlargestart => MissileType::Sfx,
        Missile::Grimwardlarge => MissileType::Sfx,
        Missile::Grimwardlargestop => MissileType::Sfx,
        Missile::Zakarumlight => MissileType::Lightning,
        Missile::Grimwardscare => MissileType::Magic,
        Missile::Frozenorb => MissileType::Ice,
        Missile::Frozenorbbolt => MissileType::Ice,
        Missile::Frozenorbnova => MissileType::Ice,
        Missile::Frozenorbexplode => MissileType::Ice,
        Missile::Chillingarmorbolt => MissileType::Ice,
        Missile::Fireexplosion2 => MissileType::Fire,
        Missile::Blowgun => MissileType::Physical,
        Missile::Chainlightning2 => MissileType::Lightning,
        Missile::Revivesmall => MissileType::Magic,
        Missile::Revivemedium => MissileType::Magic,
        Missile::Revivelarge => MissileType::Magic,
        Missile::Monglacialspike => MissileType::Ice,
        Missile::Icebreaksmall => MissileType::Ice,
        Missile::Icebreakmedium => MissileType::Ice,
        Missile::Icebreaklarge => MissileType::Ice,
        Missile::Icebreaksmoke => MissileType::Ice,
        Missile::Mephisto => MissileType::Ice,
        Missile::Firehead => MissileType::Fire,
        Missile::Whilrwind => MissileType::Magic,
        Missile::Arcanelightningbolt => MissileType::Lightning,
        Missile::Frogfire => MissileType::Fire,
        Missile::Frogcold => MissileType::Ice,
        Missile::Frogpois => MissileType::Poison,
        Missile::Desertfireball => MissileType::Fire,
        Missile::Brdeathcontrol => MissileType::FxTrigger,
        Missile::Brdeathlightningbolt => MissileType::Lightning,
        Missile::Brdeathlightninghit => MissileType::Lightning,
        Missile::Denofevillight => MissileType::Magic,
        Missile::Cairnstones => MissileType::Magic,
        Missile::Cairnstonessky => MissileType::Lightning,
        Missile::Cairnstonesground => MissileType::Magic,
        Missile::Towermist => MissileType::Magic,
        Missile::Towermisttrail => MissileType::Sfx,
        Missile::Brdeathsmokes1 => MissileType::Sfx,
        Missile::Brdeathsmokenu => MissileType::Sfx,
        Missile::Brdeathsmokedt => MissileType::Sfx,
        Missile::Brdeathspirits1 => MissileType::Sfx,
        Missile::Brdeathspiritnu => MissileType::Sfx,
        Missile::Brdeathspiritdt => MissileType::Sfx,
        Missile::Mephistodeathcontrol => MissileType::FxTrigger,
        Missile::Mephistofirewallmaker => MissileType::Fire,
        Missile::Mephistofirewall => MissileType::Fire,
        Missile::Mephistoflyingrocksbig => MissileType::Sfx,
        Missile::Mephistoexplosionbig => MissileType::Sfx,
        Missile::Mephistoflyingrockssmall => MissileType::Sfx,
        Missile::Mephistoexplosionsmall => MissileType::Sfx,
        Missile::Mephistodonotdraw => MissileType::FxTrigger,
        Missile::Andycontrol0 => MissileType::FxTrigger,
        Missile::Andyfirewallmaker => MissileType::Fire,
        Missile::Andyfirewall => MissileType::Fire,
        Missile::Andycolumnfirebase => MissileType::Fire,
        Missile::Andycolumnfire => MissileType::Fire,
        Missile::Andyfallingdebris1 => MissileType::Sfx,
        Missile::Andyfallingdebris2 => MissileType::Sfx,
        Missile::Andyfallingdebris3 => MissileType::Sfx,
        Missile::Andydebrisexplosion1 => MissileType::Sfx,
        Missile::Andydebrisexplosion2 => MissileType::Sfx,
        Missile::Andydebrisexplosion3 => MissileType::Sfx,
        Missile::Andydebrisexplosion4 => MissileType::Sfx,
        Missile::Andydebrisexplosion5 => MissileType::Sfx,
        Missile::Willowisplightningbolt => MissileType::Lightning,
        Missile::Queenpoisoncloud => MissileType::Poison,
        Missile::DirtPile => MissileType::Sfx,
        Missile::Undeadmissile1 => MissileType::Poison,
        Missile::Undeadmissile2 => MissileType::Fire,
        Missile::Undeadmissile3 => MissileType::Ice,
        Missile::Undeadmissile4 => MissileType::Dummy,
        Missile::Bonespiritexplode => MissileType::Magic,
        Missile::Dopplezonexplode => MissileType::Sfx,
        Missile::Monbonespirit => MissileType::Magic,
        Missile::Towermistfade => MissileType::Sfx,
        Missile::Countessfirewall => MissileType::Fire,
        Missile::Towerchestspawner => MissileType::Sfx,
        Missile::Hellmeteorlaunch1 => MissileType::Sfx,
        Missile::Hellmeteorlaunch2 => MissileType::Sfx,
        Missile::Hellmeteorup => MissileType::Sfx,
        Missile::Hellmeteordown => MissileType::Fire,
        Missile::Hellmeteorball => MissileType::Sfx,
        Missile::Horadricstaff => MissileType::Sfx,
        Missile::Horadriclightning => MissileType::Lightning,
        Missile::Horadriclight => MissileType::Lightning,
        Missile::Regurgitatorcorpse => MissileType::Physical,
        Missile::Regurgitatorcorpseexpl => MissileType::Physical,
        Missile::Highpriestlightning => MissileType::Lightning,
        Missile::Icebreaksmallmelt => MissileType::Ice,
        Missile::Icebreaklargemelt => MissileType::Ice,
        Missile::Leapknockback => MissileType::Physical,
        Missile::Radamentdeath => MissileType::Sfx,
        Missile::Radamenthandofgod => MissileType::Sfx,
        Missile::Radamentholybolt => MissileType::Sfx,
        Missile::Taintedsuncontrol => MissileType::FxTrigger,
        Missile::Taintedsunflash => MissileType::Lightning,
        Missile::Taintedsunball => MissileType::Lightning,
        Missile::Queendeathcenter => MissileType::Sfx,
        Missile::Queendeathglob => MissileType::Sfx,
        Missile::Queendeathsplat1 => MissileType::Sfx,
        Missile::Queendeathsplat2 => MissileType::Sfx,
        Missile::Healingbolt => MissileType::Magic,
        Missile::Mephistoholedelay => MissileType::FxTrigger,
        Missile::Mephistoholebirth => MissileType::FxTrigger,
        Missile::Mephistoholeneutral => MissileType::FxTrigger,
        Missile::Mephistoholedeath => MissileType::FxTrigger,
        Missile::Mephistoholedead => MissileType::FxTrigger,
        Missile::Durieldeathcontrol => MissileType::FxTrigger,
        Missile::Durieldeathrock => MissileType::Sfx,
        Missile::Durieldeathdebris => MissileType::Sfx,
        Missile::Durieldeathsmoke => MissileType::Sfx,
        Missile::Mephistoexplosion => MissileType::Sfx,
        Missile::Orbmist => MissileType::Sfx,
        Missile::Orbmisttrail => MissileType::Sfx,
        Missile::Orbmistfade => MissileType::Sfx,
        Missile::Pilum => MissileType::Physical,
        Missile::DiabloAppears => MissileType::FxTrigger,
        Missile::Hfcontrol => MissileType::FxTrigger,
        Missile::Hffragment1 => MissileType::Sfx,
        Missile::Hffragment2 => MissileType::Sfx,
        Missile::Hffragment3 => MissileType::Sfx,
        Missile::Hfspirit1 => MissileType::Sfx,
        Missile::Hfreserved3 => MissileType::FxTrigger,
        Missile::IzualControl => MissileType::FxTrigger,
        Missile::IzualMistLoop => MissileType::Sfx,
        Missile::IzualMistFade => MissileType::Sfx,
        Missile::IzualLightning => MissileType::Sfx,
        Missile::IzualLightningTrail => MissileType::Sfx,
        Missile::CairnStonesBolt => MissileType::Sfx,
        Missile::BombInAir => MissileType::Sfx,
        Missile::BombOnGround => MissileType::Sfx,
        Missile::BombExplosion => MissileType::Sfx,
        Missile::ShockFieldInAir => MissileType::Sfx,
        Missile::ShockFieldOnGround => MissileType::Sfx,
        Missile::Throwingstar => MissileType::Physical,
        Missile::Acidspray => MissileType::Poison,
        Missile::BladeCreeper => MissileType::Physical,
        Missile::Distraction => MissileType::Sfx,
        Missile::DistractionFog => MissileType::Sfx,
        Missile::DistractionPuff => MissileType::Sfx,
        Missile::DistractionStart => MissileType::FxTrigger,
        Missile::DistractionEnd => MissileType::FxTrigger,
        Missile::Impinfernoflame1 => MissileType::Fire,
        Missile::Impinfernoflame2 => MissileType::Fire,
        Missile::Baallightningbolt => MissileType::Lightning,
        Missile::Baallightningtrail => MissileType::Lightning,
        Missile::Baallightningbolt2 => MissileType::Lightning,
        Missile::Baallightningtrail2 => MissileType::Lightning,
        Missile::Impfireball => MissileType::Fire,
        Missile::Impfireballexplode => MissileType::Fire,
        Missile::CatapultchargedballOn => MissileType::Lightning,
        Missile::Catapultchargedball => MissileType::Lightning,
        Missile::Catapultchargedballbolt => MissileType::Lightning,
        Missile::ImpSpawnMonsters => MissileType::Sfx,
        Missile::CatapultSpikeBallOn => MissileType::Physical,
        Missile::CatapultSpikeBall => MissileType::Physical,
        Missile::CatapultSpikeInAir => MissileType::Physical,
        Missile::CatapultSpikeOnGround => MissileType::Fire,
        Missile::CatapultSpikeExplosion => MissileType::Physical,
        Missile::CatapultColdBallOn => MissileType::Ice,
        Missile::CatapultColdBall => MissileType::Ice,
        Missile::CatapultColdExplosion => MissileType::Ice,
        Missile::CatapultPlagueBallOn => MissileType::Poison,
        Missile::CatapultPlagueBall => MissileType::Poison,
        Missile::CatapultPlagueCloud => MissileType::Poison,
        Missile::CatapultMeteorBallOn => MissileType::Fire,
        Missile::CatapultMeteorBall => MissileType::Fire,
        Missile::CatapultMeteorFire => MissileType::Fire,
        Missile::TowerDeath => MissileType::Sfx,
        Missile::HealingVortex => MissileType::Sfx,
        Missile::Suicidecorpseexplode => MissileType::Sfx,
        Missile::Suicidefireexplode => MissileType::Fire,
        Missile::Suicideiceexplode => MissileType::Ice,
        Missile::Explodingjavalin => MissileType::Fire,
        Missile::Explodingjavalinexp => MissileType::Sfx,
        Missile::Lightingtrailingjavalin => MissileType::Lightning,
        Missile::Lightjavalintrail => MissileType::Lightning,
        Missile::Lightjavalinexplosion => MissileType::Lightning,
        Missile::Icejavalin => MissileType::Ice,
        Missile::Icejavalinexplode => MissileType::Ice,
        Missile::Plaguejavelin2 => MissileType::Poison,
        Missile::Plaguejavlinexplode => MissileType::Poison,
        Missile::Advlighttrailingjav => MissileType::Lightning,
        Missile::Advlighttrailingjav2 => MissileType::Lightning,
        Missile::Advlightjavexplode => MissileType::Lightning,
        Missile::Sucfireball => MissileType::Fire,
        Missile::Sucfireballexplode => MissileType::Fire,
        Missile::Sucfireballtrail => MissileType::Fire,
        Missile::Sucshockfieldmissile => MissileType::Physical,
        Missile::Sucshockfieldmissileexp => MissileType::Physical,
        Missile::Sucshockfield => MissileType::Physical,
        Missile::Hellfiremissile => MissileType::Fire,
        Missile::Hellfireexa => MissileType::Fire,
        Missile::Hellfireexb => MissileType::Fire,
        Missile::ImpChargedBolt => MissileType::Lightning,
        Missile::ImpTeleport => MissileType::Magic,
        Missile::Moltenboulder => MissileType::Fire,
        Missile::Moltenboulderemerge => MissileType::Fire,
        Missile::Moltenboulderexplode => MissileType::Fire,
        Missile::Moltenboulderfirepath => MissileType::Fire,
        Missile::MoltenboulderFlyingrocks => MissileType::Fire,
        Missile::Firestorm => MissileType::Fire,
        Missile::Firestormmaker => MissileType::Fire,
        Missile::Arcticblast1 => MissileType::Ice,
        Missile::Arcticblast2 => MissileType::Ice,
        Missile::ErruptionCenter => MissileType::Fire,
        Missile::ErruptionCrack1 => MissileType::Fire,
        Missile::ErruptionCrack2 => MissileType::Fire,
        Missile::ErruptionSmoke1 => MissileType::Fire,
        Missile::ErruptionSmoke2 => MissileType::Fire,
        Missile::VineBeastWalk1 => MissileType::Physical,
        Missile::VineBeastWalk2 => MissileType::Physical,
        Missile::VineBeastNeutral => MissileType::Physical,
        Missile::VineBeastAttack => MissileType::Physical,
        Missile::VineBeastDeath => MissileType::Sfx,
        Missile::Vines => MissileType::Physical,
        Missile::VinesTrail => MissileType::Physical,
        Missile::VinesWither => MissileType::Sfx,
        Missile::PlagueVines => MissileType::Poison,
        Missile::PlagueVinesTrail => MissileType::Poison,
        Missile::PlagueVinesWither => MissileType::Sfx,
        Missile::Twister => MissileType::Physical,
        Missile::Tornado => MissileType::Physical,
        Missile::Volcano => MissileType::Fire,
        Missile::VolcanoOverlayFire => MissileType::Fire,
        Missile::VolcanoDebris2 => MissileType::Fire,
        Missile::VolcanoExplosion => MissileType::Fire,
        Missile::VolcanoSmallFire => MissileType::Fire,
        Missile::DragonbreathMissile => MissileType::Fire,
        Missile::Lureprojectile => MissileType::Physical,
        Missile::Lurecenter => MissileType::Physical,
        Missile::Lurecloud => MissileType::Sfx,
        Missile::Impmiss1 => MissileType::Fire,
        Missile::Impmiss2 => MissileType::Fire,
        Missile::Impmiss3 => MissileType::Fire,
        Missile::Impmiss4 => MissileType::Fire,
        Missile::Impmiss5 => MissileType::Fire,
        Missile::FrozenhorrorArcticblast1 => MissileType::Ice,
        Missile::FrozenhorrorArcticblast2 => MissileType::Ice,
        Missile::Sentrychargedbolt => MissileType::Lightning,
        Missile::Sentryspikeinair => MissileType::Physical,
        Missile::Sentryspikeonground => MissileType::Physical,
        Missile::RecyclerDelay => MissileType::FxTrigger,
        Missile::RecyclerVine => MissileType::Physical,
        Missile::RecyclerFade => MissileType::Physical,
        Missile::RecyclerExplosion => MissileType::Physical,
        Missile::DeathMauler => MissileType::Physical,
        Missile::DeathMaulerTrail => MissileType::Physical,
        Missile::DeathMaulerTrailFade => MissileType::Physical,
        Missile::Bladefury1 => MissileType::Physical,
        Missile::Bladefragment1 => MissileType::Physical,
        Missile::Bladefury2 => MissileType::Physical,
        Missile::Bladefragment2 => MissileType::Physical,
        Missile::Bladefury3 => MissileType::Physical,
        Missile::Bladefragment3 => MissileType::Physical,
        Missile::Shockwave => MissileType::Physical,
        Missile::Lightningtalons => MissileType::Lightning,
        Missile::Lightningtalonstrail => MissileType::Lightning,
        Missile::Phoenixtrail => MissileType::Magic,
        Missile::Rabiesplague => MissileType::Poison,
        Missile::Rabiescontagion => MissileType::Poison,
        Missile::WakeOfDestructionMaker => MissileType::Magic,
        Missile::WakeOfDestruction => MissileType::Magic,
        Missile::Deathsentryexplode => MissileType::Sfx,
        Missile::Tigerfury => MissileType::Physical,
        Missile::Tigerfurytrail => MissileType::Fire,
        Missile::Tigerfurytrail2 => MissileType::Physical,
        Missile::InfernoSentry1 => MissileType::Fire,
        Missile::InfernoSentry2 => MissileType::Fire,
        Missile::AncientThrowingAxe => MissileType::Physical,
        Missile::Sentrylightningbolt => MissileType::Lightning,
        Missile::Sentrylightninghit => MissileType::Lightning,
        Missile::AnyaCenter => MissileType::Sfx,
        Missile::AnyaIcicle => MissileType::Sfx,
        Missile::AnyaIceimpact => MissileType::Sfx,
        Missile::AnyaIcesteam => MissileType::Sfx,
        Missile::AnyaIcemagic => MissileType::Magic,
        Missile::DragontailMissile => MissileType::Physical,
        Missile::Dragonflight => MissileType::Physical,
        Missile::Dragonflightmaker => MissileType::Fire,
        Missile::ProgressiveRadiusDamage => MissileType::Sfx,
        Missile::VineBeastWalk1Fade => MissileType::Sfx,
        Missile::VineBeastWalk2Fade => MissileType::Sfx,
        Missile::VineBeastNeutralFade => MissileType::Sfx,
        Missile::VineRecyclerDelay => MissileType::FxTrigger,
        Missile::AncientDeathCenter => MissileType::Sfx,
        Missile::AncientDeathCloud => MissileType::Sfx,
        Missile::LightningChargeUpNova => MissileType::Lightning,
        Missile::ChainlightningchargeUp => MissileType::Lightning,
        Missile::PainWormAppear => MissileType::Physical,
        Missile::BaalTauntControl => MissileType::FxTrigger,
        Missile::BaalTauntLightning => MissileType::Lightning,
        Missile::BaalTauntLightningTrail => MissileType::Lightning,
        Missile::BaalTauntPoison => MissileType::Poison,
        Missile::BaalSpawnMonsters => MissileType::Magic,
        Missile::MindblastHit => MissileType::Physical,
        Missile::BladeShieldMissile => MissileType::Physical,
        Missile::BladeShieldAttachment => MissileType::Physical,
        Missile::BaalInferno => MissileType::Fire,
        Missile::BaalNova => MissileType::Lightning,
        Missile::Fistsoffireexplode => MissileType::Fire,
        Missile::Fistsoffirefirewall => MissileType::Fire,
        Missile::Clawsofthunderbolt => MissileType::Lightning,
        Missile::Clawsofthundernova => MissileType::Lightning,
        Missile::Bladesoficeexplode => MissileType::Ice,
        Missile::Bladesoficecubes => MissileType::Ice,
        Missile::Bladesoficecubesmelt => MissileType::Ice,
        Missile::Royalstrikemeteor => MissileType::Fire,
        Missile::Royalstrikemeteorcenter => MissileType::Fire,
        Missile::Royalstrikemeteortail => MissileType::Fire,
        Missile::Royalstrikemeteorexplode => MissileType::Fire,
        Missile::Royalstrikemeteorfire => MissileType::Fire,
        Missile::Royalstrikechainlightning => MissileType::Lightning,
        Missile::Royalstrikechaosice => MissileType::Ice,
        Missile::WorldStoneChip1 => MissileType::Sfx,
        Missile::WorldStoneChip2 => MissileType::Sfx,
        Missile::WorldStoneChip3 => MissileType::Sfx,
        Missile::WorldStoneChip4 => MissileType::Sfx,
        Missile::Highpriestlightning2 => MissileType::Lightning,
        Missile::Infernoflame3 => MissileType::Fire,
        Missile::MindblastCenter => MissileType::Physical,
        Missile::Armageddoncontrol => MissileType::Fire,
        Missile::Armageddonrock => MissileType::Sfx,
        Missile::Armageddontail => MissileType::Sfx,
        Missile::Armageddonexplosion => MissileType::Sfx,
        Missile::Hurricaneswoosh => MissileType::Ice,
        Missile::Hurricanecart => MissileType::Ice,
        Missile::Hurricanerock => MissileType::Ice,
        Missile::Hurricanesack => MissileType::Ice,
        Missile::Hurricanetree => MissileType::Ice,
        Missile::Hurricanevase => MissileType::Ice,
        Missile::Baalcorpseexplodedelay => MissileType::Physical,
        Missile::Baalcorpseexplodeexpl => MissileType::Physical,
        Missile::BaalColdMaker => MissileType::Ice,
        Missile::BaalColdTrail => MissileType::Ice,
        Missile::BaalSpawnMonstersExp => MissileType::Magic,
        Missile::Impmiss21 => MissileType::Magic,
        Missile::Impmiss22 => MissileType::Magic,
        Missile::Impmiss23 => MissileType::Magic,
        Missile::Impmiss24 => MissileType::Magic,
        Missile::Impmiss25 => MissileType::Magic,
        Missile::Anyasteam1 => MissileType::Sfx,
        Missile::Anyasteam2 => MissileType::Sfx,
        Missile::Ancientsguide => MissileType::Sfx,
        Missile::Ancientsmarker => MissileType::Sfx,
        Missile::Ancientscontrol => MissileType::FxTrigger,
        Missile::Overseercontrol => MissileType::FxTrigger,
        Missile::Nihlithak1 => MissileType::Magic,
        Missile::Nihlithak2 => MissileType::Magic,
        Missile::Nihlithak3 => MissileType::Magic,
        Missile::Nihlathakcontrol => MissileType::FxTrigger,
        Missile::Nihlathakswoosh => MissileType::Sfx,
        Missile::Nihlathakdebris1 => MissileType::Sfx,
        Missile::Nihlathakdebris2 => MissileType::Sfx,
        Missile::Nihlathakdebris3 => MissileType::Sfx,
        Missile::Nihlathakdebris4 => MissileType::Sfx,
        Missile::Nihlathakglow => MissileType::Sfx,
        Missile::Baalteleport => MissileType::Magic,
        Missile::Baalclonedeath => MissileType::Magic,
        Missile::Anyasteamvent => MissileType::Magic,
        Missile::Anyasteam => MissileType::Magic,
        Missile::Nihlathakhole => MissileType::Magic,
        Missile::Nihlathakholelight => MissileType::Magic,
        Missile::Volcanofiretrail => MissileType::Fire,
        Missile::Nihlathakglow2 => MissileType::Magic,
        Missile::Nihlathakbonechips => MissileType::Magic,
        Missile::Baalcorpseexplodefade => MissileType::Magic,
        Missile::Armageddonfire => MissileType::Fire,
        Missile::Icesparkle => MissileType::Ice,
        Missile::BaalfxControl => MissileType::Sfx,
        Missile::BaalfxSpirit1 => MissileType::Sfx,
        Missile::BaalfxSpirit2 => MissileType::Sfx,
        Missile::BaalfxSpirit3 => MissileType::Sfx,
        Missile::BaalfxSpirit4 => MissileType::Sfx,
        Missile::BaalfxSpirit5 => MissileType::Sfx,
        Missile::BaalfxBaalHeadAppear => MissileType::Sfx,
        Missile::BaalfxBaalHead1 => MissileType::Sfx,
        Missile::BaalfxBaalHead2 => MissileType::Sfx,
        Missile::BaalfxBaalHead3 => MissileType::Sfx,
        Missile::BaalfxTyrealDebris1 => MissileType::Sfx,
        Missile::BaalfxTyrealDebris2 => MissileType::Sfx,
        Missile::BaalfxTyrealDebris3 => MissileType::Sfx,
        Missile::BaalfxTyrealDebrisBreak => MissileType::Sfx,
        Missile::WorldstoneShake => MissileType::Sfx,
        Missile::Blessedhammerex => MissileType::Magic,
        Missile::Sentrylightningbolt2 => MissileType::Lightning,
        Missile::Sentrylightninghit2 => MissileType::Lightning,
        Missile::Lightningtowernova => MissileType::Lightning,
        Missile::Skbowarrow6 => MissileType::Physical,
        Missile::Skbowarrow7 => MissileType::Physical,
        Missile::Skbowarrow8 => MissileType::Physical,
        Missile::Bighead6 => MissileType::Physical,
        Missile::ViperPoisjav => MissileType::Poison,
        Missile::ViperPoisjavcloud => MissileType::Poison,
        Missile::ViperFire => MissileType::Fire,
        Missile::ViperFirecloud => MissileType::Fire,
        Missile::ViperBonespear => MissileType::Magic,
        Missile::Countessfirewallmaker => MissileType::Fire,
        Missile::BaalTauntLightningControl => MissileType::Lightning,
        Missile::BaalTauntPoisonControl => MissileType::Poison,
        Missile::Explodingarrowexp2 => MissileType::Fire,
        Missile::Freezingarrowexp3 => MissileType::Ice,
        Missile::Pantherjav5 => MissileType::Physical,
        Missile::Spike6 => MissileType::Physical,
        Missile::CrArrow6 => MissileType::Physical,
        Missile::Skmagepois => MissileType::Poison,
        Missile::Skmagecold => MissileType::Ice,
        Missile::Skmagefire => MissileType::Fire,
        Missile::Skmageltng => MissileType::Lightning,
        Missile::Succubusmiss => MissileType::Magic,
        Missile::Willowisplightningbolt2 => MissileType::Lightning,
        Missile::Mummyex => MissileType::Poison,
        Missile::Goospitex => MissileType::Poison,
        Missile::Impmissex => MissileType::Fire,
        Missile::Diablogeddoncontrol => MissileType::Fire,
        Missile::Diablogeddonrock => MissileType::Fire,
        Missile::Diablogeddontail => MissileType::Fire,
        Missile::Diablogeddonexplosion => MissileType::Fire,
        Missile::Diablogeddonfire => MissileType::Fire,
        Missile::Megademoninferno => MissileType::Fire,
        Missile::Trapfirebolt => MissileType::Fire,
        Missile::Trappoisonjavcloud => MissileType::Poison,
        Missile::Trapnova => MissileType::Lightning,
        Missile::Mephfrostnova => MissileType::Ice,
        Missile::Mephlight => MissileType::Lightning,
        Missile::Vampiremeteorfire => MissileType::Fire,
        Missile::Strafearrow => MissileType::Physical,
        Missile::Strafebolt => MissileType::Physical,
        Missile::Recklessattacksmissile => MissileType::Physical,
        Missile::Unknown => MissileType::FxTrigger,
        //_=> MissileType::Dummy,
    }
}
