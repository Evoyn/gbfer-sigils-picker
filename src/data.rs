// generated from the vanilla er 2.0.2 tables, regenerate on game updates
// instead of hand-editing

// (display name, hash id as stored in the tbl)
pub type Trait = (&'static str, u32);

// the six skill_lot groups the random 2nd-trait pools draw from
pub const GROUP_A: &[Trait] = &[
    ("ATK", 0x50079A1C),
    ("Critical Hit Rate", 0x8D78A19B),
    ("HP", 0xF372F096),
    ("Stun Power", 0xCEB700EE),
];

pub const GROUP_B: &[Trait] = &[
    ("Break Assassin", 0xAC9674C1),
    ("Charged Attack DMG", 0x1C360C63),
    ("Combo Booster", 0xF17850B9),
    ("Combo Finisher DMG", 0xA7A45F28),
    ("Concentrated Fire", 0xB360801D),
    ("Critical Hit DMG", 0xC0979A17),
    ("DMG Cap", 0xDC584F60),
    ("Dodge Payback", 0x7C2E4D64),
    ("Enmity", 0x3F488339),
    ("Guard Payback", 0x3C2B57B0),
    ("Injury to Insult", 0x4F1A3683),
    ("Life on the Line", 0x8F502F0D),
    ("Linked Together", 0x3FEC5F80),
    ("Lucky Charge", 0xC35B111B),
    ("Overdrive Assassin", 0xA9D17F55),
    ("Quick Charge", 0x84078CB0),
    ("Skilled Assault", 0xEAE321EB),
    ("Stamina", 0x2FC8FBFF),
    ("Throw DMG", 0x8D078597),
    ("Tyranny", 0x71F11A9B),
    ("Weak Point DMG", 0x6B694D6D),
];

pub const GROUP_C: &[Trait] = &[
    ("Aegis", 0xE0ABFDFE),
    ("ATK Down Resistance", 0x4BF2E191),
    ("Blight Resistance", 0x9702860F),
    ("Burn Resistance", 0x7C84A6B3),
    ("Darkflame Resistance", 0xDD4A701E),
    ("DEF Down Resistance", 0x66DE60B1),
    ("Dizzy Resistance", 0x3759A5B9),
    ("Firm Stance", 0xB6E31F76),
    ("Garrison", 0xE6CDBA9C),
    ("Glaciate Resistance", 0xFB572681),
    ("Held Under Resistance", 0x1DC9D7E7),
    ("Improved Dodge", 0x8B3BF60C),
    ("Improved Guard", 0x0AA20846),
    ("Nimble Defense", 0x09AA7DB5),
    ("Paralysis Resistance", 0x2242921F),
    ("Poison Resistance", 0x973B49AF),
    ("Precise Resilience", 0x29B292A8),
    ("Sandtomb Resistance", 0xD54F8CA7),
    ("SBA Sealed Resistance", 0xCFB48782),
    ("Skill Sealed Resistance", 0x50B453DD),
    ("Slow Resistance", 0xA2FA9685),
    ("Steel Nerves", 0x1470F860),
];

pub const GROUP_D: &[Trait] = &[
    ("Cascade", 0x05F2ECDC),
    ("Drain", 0x7CCFF74F),
    ("Improved Healing", 0x9389CC06),
    ("Nimble Onslaught", 0xD2C8E10A),
    ("Precise Wrath", 0x7EDD69D0),
    ("Quick Cooldown", 0x318D12E9),
    ("Regen", 0x6085DA25),
    ("Uplift", 0xB5FF9FD3),
];

pub const GROUP_E: &[Trait] = &[
    ("Autorevive", 0x95F3FA86),
    ("Fast Learner", 0xF687C5EF),
    ("Guts", 0xE69A4694),
    ("Low Profile", 0xDC607D75),
    ("Potion Hoarder", 0x24883AF3),
    ("Provoke", 0x6018372B),
    ("Rupie Tycoon", 0xC86F3082),
    ("Steady Focus", 0x0053599E),
];

pub const GROUP_F: &[Trait] = &[
    ("Berserker", 0x70395731),
    ("Glass Cannon", 0xA8A3163B),
    ("Greater Aegis", 0x48A95B8D),
    ("Head Start", 0x1568E0E4),
    ("Less Is More", 0x82CE278D),
    ("Path to Mastery", 0x5E422AE5),
    ("Power Hungry", 0xDC225C96),
    ("Stronghold", 0x74AA75D6),
    ("Supplementary DMG", 0x57AB5B10),
];

// skill_type_lot pools as group unions, in flatten order
pub const LOT5: &[&[Trait]] = &[GROUP_B, GROUP_C, GROUP_D, GROUP_E];
pub const LOT6: &[&[Trait]] = &[GROUP_C, GROUP_D, GROUP_E];
pub const LOT7: &[&[Trait]] = &[GROUP_D, GROUP_E];
pub const LOT15: &[&[Trait]] = &[GROUP_A, GROUP_B, GROUP_C, GROUP_D, GROUP_E];
pub const LOT16: &[&[Trait]] = &[GROUP_A, GROUP_B, GROUP_C, GROUP_D, GROUP_E, GROUP_F];
pub const LOT26: &[&[Trait]] = &[GROUP_A, GROUP_B, GROUP_C, GROUP_E];
pub const LOT27: &[&[Trait]] = &[GROUP_A, GROUP_B, GROUP_C, GROUP_D];

// lot 4 uses old-gen clones of groups d and e, same skills under different hashes
pub const GROUP_ALIASES: &[(u32, usize)] = &[(0xED723E06, 3), (0x5B7C309C, 4)];

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Unique,
    Character,
    Transmarvel,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Subs {
    // single trait, no 2nd slot
    None,
    // fixed 2nd trait, display only
    Fixed(&'static str),
    // random 2nd trait from these groups, pinnable to any of them
    Lot(&'static [&'static [Trait]]),
    // leftover 2nd trait, stripped to NO_TRAIT on grant
    Strip,
}

// what the game's own single-trait gems carry in the 2nd slot
pub const NO_TRAIT: u32 = 0x887AE0B0;

pub struct Sigil {
    pub name: &'static str,
    // gem.tbl key == gacha_lot itemid hash
    pub key: u32,
    // gacha_lot traitlevel to write (15 = maxed, 0 = game default)
    pub level: i32,
    pub subs: Subs,
    // vanilla transmarvel can roll this exact item
    pub native: bool,
    pub kind: Kind,
    // canonlyholdone: the game blocks duplicates and eats the roll
    pub once: bool,
    // red warning text when the grant is some kind of cheat
    pub cheat: Option<&'static str>,
    // false = exact picks only, hidden from gacha mode
    pub gacha: bool,
}

pub const SIGILS: &[Sigil] = &[
    Sigil { name: "War Elemental+",                 key: 0x2D85102A, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true }, // GEEN_146_34
    Sigil { name: "Untouchable+",                   key: 0x49EBEBEB, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Flight over Fight+",             key: 0x8E20B20C, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Potent Greens+",                 key: 0x9A60FBF0, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Auto Potion+",                   key: 0x3C0F5461, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Roll of the Die+",               key: 0x6DB307D5, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Natural Defenses",               key: 0xF2C84D4F, level: 15, subs: Subs::None, native: false, kind: Kind::Unique, once: false, cheat: None, gacha: false }, // GEEN_103_04
    Sigil { name: "Stout Heart",                    key: 0xCB5F29C1, level: 15, subs: Subs::None, native: false, kind: Kind::Unique, once: false, cheat: None, gacha: false }, // GEEN_044_04
    Sigil { name: "Berserker Echo+",                key: 0x99E8B892, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true }, // GEEN_233_34
    Sigil { name: "Spartan Echo+",                  key: 0x4AE72C9E, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true }, // GEEN_234_34
    Sigil { name: "Super Ultimate Perfect Dodge+",  key: 0x113035D8, level: 15, subs: Subs::Lot(LOT16), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: true },
    Sigil { name: "Alpha+",                         key: 0x921D90D8, level: 15, subs: Subs::Fixed("DMG Cap"), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: false }, // GEEN_160_04
    Sigil { name: "Beta+",                          key: 0xEE337FE3, level: 15, subs: Subs::Fixed("DMG Cap"), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: false }, // GEEN_161_04
    Sigil { name: "Gamma+",                         key: 0x4438676E, level: 15, subs: Subs::Fixed("DMG Cap"), native: false, kind: Kind::Unique, once: false, cheat: None, gacha: false }, // GEEN_162_04
    Sigil { name: "Immortal Shell",                 key: 0x49434696, level: 0, subs: Subs::None, native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_301_00
    Sigil { name: "Immortal Shell+",                key: 0x66CB28BA, level: 20, subs: Subs::Strip, native: false, kind: Kind::Unique, once: false, cheat: Some("Considered a cheat sigil, but it has the same stats as vanilla."), gacha: false }, // GEEN_301_10, stripped; needs the gem_rare r5 cap raise to land above 15
    Sigil { name: "Immortal Shell+ (with Crabvestment Returns)", key: 0x66CB28BA, level: 20, subs: Subs::Fixed("Crabvestment Returns"), native: false, kind: Kind::Unique, once: false, cheat: Some("Cheat sigil: this combination and its stats do not exist in vanilla."), gacha: false }, // GEEN_301_10 unstripped, 2nd trait runs past its Lv15 cap
    Sigil { name: "In a Pinch",                     key: 0x65F0420A, level: 0, subs: Subs::None, native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_302_00
    Sigil { name: "Sumo Force",                     key: 0xB289A9AD, level: 0, subs: Subs::None, native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_303_00
    Sigil { name: "Crabby Resonance",               key: 0x1C4D37E4, level: 0, subs: Subs::None, native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_140_00
    Sigil { name: "Crabs Are Forever+",             key: 0x426AD20E, level: 0, subs: Subs::Fixed("Crabby Resonance + Crabmiration"), native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_300_11
    Sigil { name: "Crabvestment Returns",           key: 0xF8FEF304, level: 0, subs: Subs::None, native: false, kind: Kind::Unique, once: true, cheat: None, gacha: false }, // GEEN_141_04
    Sigil { name: "Aegis V+",                       key: 0x9C2399DA, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_085_24
    Sigil { name: "Attack Power V+",                key: 0x2D7F2E70, level: 15, subs: Subs::Lot(LOT5), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_000_24
    Sigil { name: "Autorevive V+",                  key: 0xD340651C, level: 15, subs: Subs::Lot(LOT27), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Cascade V+",                     key: 0x6CBA6B0D, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Celestial Aqua V+",              key: 0xE14E1598, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_324_24
    Sigil { name: "Celestial Incendo V+",           key: 0x74061B0C, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_323_24
    Sigil { name: "Celestial Lumen V+",             key: 0x20492635, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_321_24
    Sigil { name: "Celestial Nyx V+",               key: 0x8B8085C0, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_320_24
    Sigil { name: "Celestial Terra V+",             key: 0xD29CD8E0, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_322_24
    Sigil { name: "Celestial Ventus V+",            key: 0x9300FADB, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_326_24
    Sigil { name: "Critical Hit Rate V+",           key: 0x0BB9C188, level: 15, subs: Subs::Lot(LOT5), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_003_24
    Sigil { name: "Damage Cap V+",                  key: 0x54D8EA04, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_020_24
    Sigil { name: "Divergence V+",                  key: 0x7B4AAB30, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_327_24
    Sigil { name: "Drain V+",                       key: 0x3BA37635, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Enmity V+",                      key: 0xA3021B5E, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_005_24
    Sigil { name: "Fatebreaker V+",                 key: 0x5BF84FD1, level: 15, subs: Subs::Lot(LOT27), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_325_24
    Sigil { name: "Garrison V+",                    key: 0x381BBE64, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_036_24, gem lot 4 = same D+E pools under old hashes
    Sigil { name: "Greater Aegis V+",               key: 0x6F1D0870, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_166_24
    Sigil { name: "Guts V+",                        key: 0xBB49C8F6, level: 15, subs: Subs::Lot(LOT27), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Health V+",                      key: 0xE92EE838, level: 15, subs: Subs::Lot(LOT5), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_001_24
    Sigil { name: "Improved Dodge+",                key: 0xE89224A1, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_063_24
    Sigil { name: "Improved Guard V+",              key: 0xB832E7A7, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_060_24
    Sigil { name: "Nimble Defense V+",              key: 0xA4B849A0, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_104_24
    Sigil { name: "Nimble Onslaught V+",            key: 0x2679A4F0, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Potion Hoarder V+",              key: 0x3ED16FB2, level: 15, subs: Subs::Lot(LOT27), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Quick Cooldown V+",              key: 0xAB70208C, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Regen V+",                       key: 0x837B3D64, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Stamina V+",                     key: 0xE8454459, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_006_24
    Sigil { name: "Steel Nerves V+",                key: 0xEDA2116A, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_096_24
    Sigil { name: "Stronghold V+",                  key: 0xD4EBB836, level: 15, subs: Subs::Lot(LOT7), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_144_24
    Sigil { name: "Stun Power V+",                  key: 0x791DA8ED, level: 15, subs: Subs::Lot(LOT5), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_004_24
    Sigil { name: "Supplementary Damage V+",        key: 0x035A4DDD, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_151_24
    Sigil { name: "Tyranny V+",                     key: 0xA492FFAC, level: 15, subs: Subs::Lot(LOT6), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true }, // GEEN_027_24
    Sigil { name: "Uplift V+",                      key: 0x04AC2281, level: 15, subs: Subs::Lot(LOT26), native: true, kind: Kind::Transmarvel, once: false, cheat: None, gacha: true },
    Sigil { name: "Fearless Drive+ (Protagonist)", key: 0x33F01810, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_114_91
    Sigil { name: "Fearless Spirit+ (Protagonist)", key: 0x380A3CA8, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_114_92
    Sigil { name: "Fearless Heart+ (Protagonist)", key: 0x0713D928, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_114_93
    Sigil { name: "Fearless Soul+ (Protagonist)", key: 0x52A6E299, level: 15, subs: Subs::Fixed("Drive + Spirit"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_114_90
    Sigil { name: "Guardian's Conviction+ (Katalina)", key: 0x522004AB, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_115_91
    Sigil { name: "Guardian's Honor+ (Katalina)", key: 0x30A3F2EA, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_115_92
    Sigil { name: "Guardian's Warpath+ (Katalina)", key: 0xAC175924, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_115_93
    Sigil { name: "Guardian's Awakening+ (Katalina)", key: 0x9ADA3E00, level: 15, subs: Subs::Fixed("Conviction + Honor"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_115_90
    Sigil { name: "Helmsman's Navigation+ (Rackam)", key: 0x9F08F697, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_116_91
    Sigil { name: "Helmsman's Tenacity+ (Rackam)", key: 0xD48ABDDA, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_116_92
    Sigil { name: "Helmsman's Warpath+ (Rackam)", key: 0xBC53CE24, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_116_93
    Sigil { name: "Helmsman's Awakening+ (Rackam)", key: 0x98A6D249, level: 15, subs: Subs::Fixed("Navigation + Tenacity"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_116_90
    Sigil { name: "Mage's Aspiration+ (Io)", key: 0x9D88DEA1, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_117_91
    Sigil { name: "Mage's Savvy+ (Io)", key: 0xF6C0FCA5, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_117_92
    Sigil { name: "Mage's Warpath+ (Io)", key: 0x43F26A91, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_117_93
    Sigil { name: "Mage's Awakening+ (Io)", key: 0xE2B380E5, level: 15, subs: Subs::Fixed("Aspiration + Savvy"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_117_90
    Sigil { name: "Veteran's Insight+ (Eugen)", key: 0x64D63823, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_118_91
    Sigil { name: "Veteran's Vision+ (Eugen)", key: 0x05ACA892, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_118_92
    Sigil { name: "Veteran's Warpath+ (Eugen)", key: 0xCAAE3F9C, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_118_93
    Sigil { name: "Veteran's Awakening+ (Eugen)", key: 0x1BBE919C, level: 15, subs: Subs::Fixed("Insight + Vision"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_118_90
    Sigil { name: "Rose's Blooming+ (Rosetta)", key: 0x01D1A6CE, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_119_91
    Sigil { name: "Rose's Profusion+ (Rosetta)", key: 0x21E10EB7, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_119_92
    Sigil { name: "Rose's Warpath+ (Rosetta)", key: 0x515E693C, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_119_93
    Sigil { name: "Rose's Awakening+ (Rosetta)", key: 0x9BD1CC24, level: 15, subs: Subs::Fixed("Blooming + Profusion"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_119_90
    Sigil { name: "Phantasm's Concord+ (Ferry)", key: 0xE073EA65, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_120_91
    Sigil { name: "Phantasm's Harmony+ (Ferry)", key: 0xBF714A8A, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_120_92
    Sigil { name: "Phantasm's Warpath+ (Ferry)", key: 0xE496D882, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_120_93
    Sigil { name: "Phantasm's Awakening+ (Ferry)", key: 0xB441275D, level: 15, subs: Subs::Fixed("Concord + Harmony"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_120_90
    Sigil { name: "White Dragon's Oath+ (Lancelot)", key: 0x85D7B335, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_121_91
    Sigil { name: "White Dragon's Glory+ (Lancelot)", key: 0xB5DA3E80, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_121_92
    Sigil { name: "White Dragon's Warpath+ (Lancelot)", key: 0x8A3819C0, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_121_93
    Sigil { name: "White Dragon's Awakening+ (Lancelot)", key: 0xE19B1965, level: 15, subs: Subs::Fixed("Oath + Glory"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_121_90
    Sigil { name: "Hero's Creed+ (Vane)", key: 0x9D5BC5BF, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_122_91
    Sigil { name: "Hero's Will+ (Vane)", key: 0xFB9B6DD5, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_122_92
    Sigil { name: "Hero's Warpath+ (Vane)", key: 0xA490BADF, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_122_93
    Sigil { name: "Hero's Awakening+ (Vane)", key: 0x673C5D8F, level: 15, subs: Subs::Fixed("Creed + Will"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_122_90
    Sigil { name: "Lord's Procession+ (Percival)", key: 0xB5725272, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_123_91
    Sigil { name: "Lord's Ambition+ (Percival)", key: 0xC06F4708, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_123_92
    Sigil { name: "Lord's Warpath+ (Percival)", key: 0x4CDCE25B, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_123_93
    Sigil { name: "Lord's Awakening+ (Percival)", key: 0x02472C43, level: 15, subs: Subs::Fixed("Procession + Ambition"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_123_90
    Sigil { name: "Dragonslayer's Dominance+ (Siegfried)", key: 0xC0B5128E, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_124_91
    Sigil { name: "Dragonslayer's Ingenuity+ (Siegfried)", key: 0xBCEDF060, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_124_92
    Sigil { name: "Dragonslayer's Warpath+ (Siegfried)", key: 0xE21A4170, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_124_93
    Sigil { name: "Dragonslayer's Awakening+ (Siegfried)", key: 0xAB835493, level: 15, subs: Subs::Fixed("Dominance + Ingenuity"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_124_90
    Sigil { name: "Holy Knight's Luster+ (Charlotta)", key: 0xA0F94F69, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_125_91
    Sigil { name: "Holy Knight's Grandeur+ (Charlotta)", key: 0x7C8580CA, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_125_92
    Sigil { name: "Holy Knight's Warpath+ (Charlotta)", key: 0x4C28585A, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_125_93
    Sigil { name: "Holy Knight's Awakening+ (Charlotta)", key: 0xAEEF8343, level: 15, subs: Subs::Fixed("Luster + Grandeur"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_125_90
    Sigil { name: "Swordmaster's Prowess+ (Yodarha)", key: 0xE7624711, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_126_91
    Sigil { name: "Swordmaster's Art+ (Yodarha)", key: 0x49651C89, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_126_92
    Sigil { name: "Swordmaster's Warpath+ (Yodarha)", key: 0x76D4716B, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_126_93
    Sigil { name: "Swordmaster's Awakening+ (Yodarha)", key: 0x119B24A8, level: 15, subs: Subs::Fixed("Prowess + Art"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_126_90
    Sigil { name: "Butterfly's Grace+ (Narmaya)", key: 0xB143DAE6, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_127_91
    Sigil { name: "Butterfly's Valor+ (Narmaya)", key: 0xA879208F, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_127_92
    Sigil { name: "Butterfly's Warpath+ (Narmaya)", key: 0xCEF31894, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_127_93
    Sigil { name: "Butterfly's Awakening+ (Narmaya)", key: 0x1A57AEF1, level: 15, subs: Subs::Fixed("Grace + Valor"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_127_90
    Sigil { name: "Eternal Rage's Mettle+ (Ghandagoza)", key: 0x7D318FF7, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_128_91
    Sigil { name: "Eternal Rage's Ethos+ (Ghandagoza)", key: 0x6CCA1FF7, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_128_92
    Sigil { name: "Eternal Rage's Warpath+ (Ghandagoza)", key: 0x3069C2FE, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_128_93
    Sigil { name: "Eternal Rage's Awakening+ (Ghandagoza)", key: 0xCE16D68B, level: 15, subs: Subs::Fixed("Mettle + Ethos"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_128_90
    Sigil { name: "Founder's Strategy+ (Cagliostro)", key: 0x14C58BF1, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_129_91
    Sigil { name: "Founder's Truth+ (Cagliostro)", key: 0x147DA58B, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_129_92
    Sigil { name: "Founder's Warpath+ (Cagliostro)", key: 0x66F1B128, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_129_93
    Sigil { name: "Founder's Awakening+ (Cagliostro)", key: 0x02B1F8C0, level: 15, subs: Subs::Fixed("Strategy + Truth"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_129_90
    Sigil { name: "Versalis Foundation+ (Id)", key: 0xB98A0F22, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_130_91
    Sigil { name: "Versalis Ignition+ (Id)", key: 0xEAA911B2, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_130_92
    Sigil { name: "Versalis Heart+ (Id)", key: 0x98E9E6EF, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_130_93
    Sigil { name: "Versalis Soul+ (Id)", key: 0xAF8E7E7E, level: 15, subs: Subs::Fixed("Foundation + Ignition"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_130_90
    Sigil { name: "Crimson's Clout+ (Zeta)", key: 0xB74C207B, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_131_91
    Sigil { name: "Crimson's Flight+ (Zeta)", key: 0x44D48479, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_131_92
    Sigil { name: "Crimson's Warpath+ (Zeta)", key: 0xBFDF838C, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_131_93
    Sigil { name: "Crimson's Awakening+ (Zeta)", key: 0x4F01D6CA, level: 15, subs: Subs::Fixed("Clout + Flight"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_131_90
    Sigil { name: "Ebony's Presence+ (Vaseraga)", key: 0xFB0F9037, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_132_91
    Sigil { name: "Ebony's Poise+ (Vaseraga)", key: 0xA59C9613, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_132_92
    Sigil { name: "Ebony's Warpath+ (Vaseraga)", key: 0xB3AB43F3, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_132_93
    Sigil { name: "Ebony's Awakening+ (Vaseraga)", key: 0xE4F986D9, level: 15, subs: Subs::Fixed("Presence + Poise"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_132_90
    Sigil { name: "Spirit Edge's Rally+ (Seofon)", key: 0x12DFD310, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_170_91
    Sigil { name: "Spirit Edge's Fury+ (Seofon)", key: 0xAE9D89DF, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_170_92
    Sigil { name: "Spirit Edge's Warpath+ (Seofon)", key: 0x9F72BAE0, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_170_93
    Sigil { name: "Spirit Edge's Awakening+ (Seofon)", key: 0x6AAE4B8F, level: 15, subs: Subs::Fixed("Rally + Fury"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_170_90
    Sigil { name: "Dark Huntress's Volley+ (Tweyen)", key: 0xEB4AD96D, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_171_91
    Sigil { name: "Dark Huntress's Surge+ (Tweyen)", key: 0xDBE503C7, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_171_92
    Sigil { name: "Dark Huntress's Warpath+ (Tweyen)", key: 0xAD8CAEFB, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_171_93
    Sigil { name: "Dark Huntress's Awakening+ (Tweyen)", key: 0x8ECBB0A3, level: 15, subs: Subs::Fixed("Volley + Surge"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_171_90
    Sigil { name: "Supreme Primarch's Awe+ (Sandalphon)", key: 0x3EA4134B, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_172_91
    Sigil { name: "Supreme Primarch's Nimbus+ (Sandalphon)", key: 0x7E3A52A3, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_172_92
    Sigil { name: "Supreme Primarch's Warpath+ (Sandalphon)", key: 0x5D592FDD, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_172_93
    Sigil { name: "Supreme Primarch's Awakening+ (Sandalphon)", key: 0xB8C44D5E, level: 15, subs: Subs::Fixed("Awe + Nimbus"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_172_90
    Sigil { name: "Gladiator's Frenzy+ (Gallanza)", key: 0x282DBFF0, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_173_91
    Sigil { name: "Gladiator's Top+ (Gallanza)", key: 0xF21404B1, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_173_92
    Sigil { name: "Gladiator's Warpath+ (Gallanza)", key: 0x41AC1082, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_173_93
    Sigil { name: "Gladiator's Awakening+ (Gallanza)", key: 0x895ABBF6, level: 15, subs: Subs::Fixed("Frenzy + Top"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_173_90
    Sigil { name: "Bladequeen's Serenade+ (Maglielle)", key: 0xEC9FFE77, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_174_91
    Sigil { name: "Bladequeen's Circuit+ (Maglielle)", key: 0x96D6FE5E, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_174_92
    Sigil { name: "Bladequeen's Warpath+ (Maglielle)", key: 0xEB766D87, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_174_93
    Sigil { name: "Bladequeen's Awakening+ (Maglielle)", key: 0xD8A464F1, level: 15, subs: Subs::Fixed("Serenade + Circuit"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_174_90
    Sigil { name: "Ultramarine's Flash+ (Beatrix)", key: 0x9EC6C56D, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_175_91
    Sigil { name: "Ultramarine's Adversity+ (Beatrix)", key: 0xD4117FF3, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_175_92
    Sigil { name: "Ultramarine's Warpath+ (Beatrix)", key: 0x51E98A7C, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_175_93
    Sigil { name: "Ultramarine's Awakening+ (Beatrix)", key: 0x95CC3CB8, level: 15, subs: Subs::Fixed("Flash + Adversity"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_175_90
    Sigil { name: "Thunderwolf's Recharge+ (Eustace)", key: 0xF964A4CA, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_176_91
    Sigil { name: "Thunderwolf's Acuity+ (Eustace)", key: 0x1A359B67, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_176_92
    Sigil { name: "Thunderwolf's Warpath+ (Eustace)", key: 0xD8C61507, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_176_93
    Sigil { name: "Thunderwolf's Awakening+ (Eustace)", key: 0x23953FD4, level: 15, subs: Subs::Fixed("Recharge + Acuity"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_176_90
    Sigil { name: "Enchantress's Blessing+ (Fraux)", key: 0x64301E91, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_177_91
    Sigil { name: "Enchantress's Rhythm+ (Fraux)", key: 0xBA28C81C, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_177_92
    Sigil { name: "Enchantress's Warpath+ (Fraux)", key: 0x2D70C37D, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_177_93
    Sigil { name: "Enchantress's Awakening+ (Fraux)", key: 0x5A360EA8, level: 15, subs: Subs::Fixed("Blessing + Rhythm"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_177_90
    Sigil { name: "The Black's Mark+ (Fediel)", key: 0x0523A202, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_178_91
    Sigil { name: "The Black's Impulse+ (Fediel)", key: 0x0723F7EC, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_178_92
    Sigil { name: "The Black's Warpath+ (Fediel)", key: 0x9ABD2DA5, level: 15, subs: Subs::Lot(LOT15), native: true, kind: Kind::Character, once: false, cheat: None, gacha: true }, // GEEN_178_93
    Sigil { name: "The Black's Awakening+ (Fediel)", key: 0xA8A0CBFF, level: 15, subs: Subs::Fixed("Mark + Impulse"), native: true, kind: Kind::Character, once: true, cheat: None, gacha: true }, // GEEN_178_90
];

// flatten a lot's groups in stable order
pub fn flatten(groups: &'static [&'static [Trait]]) -> impl Iterator<Item = Trait> {
    groups.iter().flat_map(|g| g.iter().copied())
}

// the six groups keyed by their skill_lot group hash, in flatten order; this is
// the index space of the allowed-2nd-traits mask
pub const GROUPS: &[(u32, &[Trait])] = &[
    (0x4CE7152C, GROUP_A),
    (0xF865A223, GROUP_B),
    (0x8F952AC1, GROUP_C),
    (0x46D6DFDE, GROUP_D),
    (0xD4078C7D, GROUP_E),
    (0xA2C50078, GROUP_F),
];
pub const TRAIT_COUNT: usize = 72;

// skill_type_lot rows the trait filter governs. lots 2 and 3 stay vanilla,
// their gems aren't offered here
pub const TRAIT_LOTS: &[u32] = &[4, 5, 6, 7, 15, 16, 26, 27];

// gacha.tbl key of tier-4 Transmarvel (75/25 gem/wrightstone in vanilla)
pub const TRANSMARVEL_GACHA: u32 = 0xFA21E311;
// Transmarvel's gem-side rate group and its nine pools
pub const GEM_RATE_GROUP: u32 = 0x27509C51;
pub const GEM_POOLS: &[u32] = &[
    0x9092654F, 0xB3976B98, 0x090F0E91, 0xF527EF32, 0x5AD4ADAD,
    0x81216A95, 0x6E52A69A, 0x36879ED7, 0x1F44C95D,
];
