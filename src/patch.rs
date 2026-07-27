// rewrites the gacha tables so the transmarvel gem roll only lands on the picks.
// picked itemids overwrite the first rows of transmarvel's own pools, everything
// else is zero-weighted. rate weight per pool = picks in it * 5000, one shared
// lot weight, so n picks = exactly 1/n each.

use crate::data::{FORCE_SELLABLE, GEM_POOLS, GEM_RATE_GROUP, GROUPS, GROUP_ALIASES, TRAIT_LOTS, TRANSMARVEL_GACHA};

pub const BASE_GACHA: &[u8] = include_bytes!("../base/gacha.tbl");
pub const BASE_GACHA_LOT: &[u8] = include_bytes!("../base/gacha_lot.tbl");
pub const BASE_GACHA_RATE_GROUP: &[u8] = include_bytes!("../base/gacha_rate_group.tbl");
pub const BASE_GEM: &[u8] = include_bytes!("../base/gem.tbl");
pub const BASE_SKILL_LOT: &[u8] = include_bytes!("../base/skill_lot.tbl");
pub const BASE_SKILL_TYPE_LOT: &[u8] = include_bytes!("../base/skill_type_lot.tbl");
pub const BASE_GEM_RARE: &[u8] = include_bytes!("../base/gem_rare.tbl");
pub const BASE_GEM_MIX_SUCCESS: &[u8] = include_bytes!("../base/gem_mix_success.tbl");

// one forced roll: gem, lot level, optional pinned 2nd trait
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Grant {
    pub key: u32,
    pub level: i32,
    pub pin: Option<u32>,
}

// tbl format: 8-byte row count (u64 LE), then fixed-size rows of 4-byte fields
#[inline]
pub fn u32le(b: &[u8], o: usize) -> u32 { u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }
#[cfg(test)]
pub fn i32le(b: &[u8], o: usize) -> i32 { i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }
#[inline]
fn put_u32(b: &mut [u8], o: usize, v: u32) { b[o..o + 4].copy_from_slice(&v.to_le_bytes()); }
#[inline]
fn put_i32(b: &mut [u8], o: usize, v: i32) { b[o..o + 4].copy_from_slice(&v.to_le_bytes()); }
#[inline]
pub fn row_count(b: &[u8]) -> usize { u32le(b, 0) as usize } // low 32 bits of the u64 count

// gacha_lot row = 28 bytes.
//  +0 QuestIDMin  +4 QuestIDMax  +8 Key(pool)  +12 ItemId  +16 Weight
//  +20 TraitLevel  +24 NeedsEndlessRagnarokToDrop
// gacha_rate_group row = 16 bytes.  +0 Key(group) +4 GachaLotId(pool) +8 Weight +12 ER gate
pub fn build_gacha_tables(grants: &[Grant]) -> (Vec<u8>, Vec<u8>) {
    let mut gl = BASE_GACHA_LOT.to_vec();
    let mut grg = BASE_GACHA_RATE_GROUP.to_vec();

    // picks over the first pool rows in file order, rest zeroed
    let mut next = 0usize;
    let mut per_pool = [0i32; 16];
    let n = row_count(&gl);
    for r in 0..n {
        let o = 8 + r * 28;
        let pool = u32le(&gl, o + 8);
        let Some(pidx) = GEM_POOLS.iter().position(|&p| p == pool) else { continue };
        if next < grants.len() {
            let g = grants[next];
            next += 1;
            put_u32(&mut gl, o, 0); // drop quest gates some rows carry
            put_u32(&mut gl, o + 4, 0);
            put_u32(&mut gl, o + 12, g.key);
            put_i32(&mut gl, o + 16, 50);
            put_i32(&mut gl, o + 20, g.level);
            put_i32(&mut gl, o + 24, 0);
            per_pool[pidx] += 1;
        } else {
            put_i32(&mut gl, o + 16, 0);
        }
    }

    let n = row_count(&grg);
    for r in 0..n {
        let o = 8 + r * 16;
        if u32le(&grg, o) != GEM_RATE_GROUP { continue }
        let pool = u32le(&grg, o + 4);
        if let Some(pidx) = GEM_POOLS.iter().position(|&p| p == pool) {
            put_i32(&mut grg, o + 8, per_pool[pidx] * 5000);
            put_i32(&mut grg, o + 12, 0); // drop the er gate on the pool
        }
    }

    (gl, grg)
}

// gem.tbl row = 64 bytes: +0 skill1 +4 skill2 +8 key ... +36 random-2nd lot
// +48 rarity +60 canuseazurite.
// pinning writes the trait as the explicit 2nd skill and kills the random lot,
// same shape the game uses for its own fixed-sub variants. lock_overlevel turns
// azurite off for rarity 5 so nothing gets hand-leveled past 15 while the cap
// is 20. synthesis lives in gem_mix_success (zeroing cangemmix here was tested
// in game and does nothing).
pub fn patched_gem(grants: &[Grant], lock_overlevel: bool) -> Vec<u8> {
    let mut gem = BASE_GEM.to_vec();
    let n = row_count(&gem);
    for r in 0..n {
        let o = 8 + r * 64;
        if FORCE_SELLABLE.contains(&u32le(&gem, o + 8)) { gem[o + 57] = 0; }
        if lock_overlevel && u32le(&gem, o + 48) == 5 { gem[o + 60] = 0; }
    }
    for g in grants {
        let Some(pin) = g.pin else { continue };
        for r in 0..n {
            let o = 8 + r * 64;
            if u32le(&gem, o + 8) == g.key {
                put_u32(&mut gem, o + 4, pin);
                put_i32(&mut gem, o + 36, -1);
                break;
            }
        }
    }
    gem
}

// groups index for a group hash, old-gen aliases included
fn group_index(hash: u32) -> Option<usize> {
    GROUPS.iter().position(|(h, _)| *h == hash)
        .or_else(|| GROUP_ALIASES.iter().find(|(h, _)| *h == hash).map(|(_, i)| *i))
}

// flatten offsets into the allowed mask + allowed count per group
fn group_stats(allowed: &[bool]) -> ([usize; 6], [i32; 6]) {
    let mut base = [0usize; 6];
    let mut counts = [0i32; 6];
    let mut off = 0usize;
    for (gi, (_, g)) in GROUPS.iter().enumerate() {
        base[gi] = off;
        counts[gi] = (0..g.len()).filter(|i| allowed.get(off + i).copied().unwrap_or(false)).count() as i32;
        off += g.len();
    }
    (base, counts)
}

// skill_lot row = 12 bytes: +0 key(group) +4 skillid +8 weight
// skill_type_lot row = 52 bytes: +0..+20 group ids, +24..+44 their weights, +48 key
// the "chancepercent" columns are weights, not percents (vanilla lot 16 sums to
// 120), so group weight = allowed-trait count makes every allowed trait equally
// likely. all or none checked = vanilla bytes back.
pub fn build_trait_tables(allowed: &[bool]) -> (Vec<u8>, Vec<u8>) {
    let sl = BASE_SKILL_LOT.to_vec();
    let stl = BASE_SKILL_TYPE_LOT.to_vec();
    if allowed.iter().all(|&a| a) || allowed.iter().all(|&a| !a) { return (sl, stl); }
    let mut sl = sl;
    let mut stl = stl;
    let (base, counts) = group_stats(allowed);

    let n = row_count(&sl);
    for r in 0..n {
        let o = 8 + r * 12;
        let Some(gi) = group_index(u32le(&sl, o)) else { continue };
        let skill = u32le(&sl, o + 4);
        if let Some(ti) = GROUPS[gi].1.iter().position(|t| t.1 == skill) {
            put_i32(&mut sl, o + 8, allowed[base[gi] + ti] as i32);
        }
    }

    let n = row_count(&stl);
    for r in 0..n {
        let o = 8 + r * 52;
        if !TRAIT_LOTS.contains(&u32le(&stl, o + 48)) { continue }
        for slot in 0..6 {
            let gh = u32le(&stl, o + slot * 4);
            if gh == 0 { continue }
            if let Some(gi) = group_index(gh) {
                put_i32(&mut stl, o + 24 + slot * 4, counts[gi]);
            }
        }
    }
    (sl, stl)
}

// governed lots left with no allowed trait; a sigil rolling from one would break
pub fn uncovered_lots(allowed: &[bool]) -> Vec<u32> {
    if allowed.iter().all(|&a| a) || allowed.iter().all(|&a| !a) { return Vec::new(); }
    let (_, counts) = group_stats(allowed);
    let mut out = Vec::new();
    let n = row_count(BASE_SKILL_TYPE_LOT);
    for r in 0..n {
        let o = 8 + r * 52;
        let key = u32le(BASE_SKILL_TYPE_LOT, o + 48);
        if !TRAIT_LOTS.contains(&key) { continue }
        let total: i32 = (0..6).map(|slot| {
            let gh = u32le(BASE_SKILL_TYPE_LOT, o + slot * 4);
            group_index(gh).map(|gi| counts[gi]).unwrap_or(0)
        }).sum();
        if total == 0 { out.push(key); }
    }
    out
}

// gem_rare row = 12 bytes: +0 key(rarity) +4 default +8 max.
// grants clamp to the rarity max, so a lv20 grant needs the r5 cap at 20.
// vanilla drops all sit at 15 or below, nothing else moves on its own.
pub fn patched_gem_rare(unlock: bool) -> Vec<u8> {
    let mut g = BASE_GEM_RARE.to_vec();
    if unlock {
        let n = row_count(&g);
        for r in 0..n {
            let o = 8 + r * 12;
            if u32le(&g, o) == 5 {
                put_i32(&mut g, o + 8, 20);
                break;
            }
        }
    }
    g
}

// how many grants fit at once, the row count of transmarvel's own gem pools
pub fn pool_capacity() -> usize {
    (0..row_count(BASE_GACHA_LOT))
        .filter(|r| GEM_POOLS.contains(&u32le(BASE_GACHA_LOT, 8 + r * 28 + 8)))
        .count()
}

// gem_mix_success row = 12 bytes: +0 great weight +4 grand weight +8 key
// (combined skill level). grand success grants the gem's max level, aka the
// raised cap, so lock zeroes every grand weight: synthesis still works, just
// always lands at the default level.
pub fn patched_gem_mix_success(lock: bool) -> Vec<u8> {
    let mut g = BASE_GEM_MIX_SUCCESS.to_vec();
    if lock {
        let n = row_count(&g);
        for r in 0..n {
            put_u32(&mut g, 8 + r * 12 + 4, 0);
        }
    }
    g
}

// gacha.tbl row = 48 bytes: +0 gem chance +4 wrightstone chance +8 key.
// sigil_only sets the transmarvel row to 100% gem, other tiers untouched.
pub fn patched_gacha(sigil_only: bool) -> Vec<u8> {
    let mut g = BASE_GACHA.to_vec();
    if sigil_only {
        let n = row_count(&g);
        for r in 0..n {
            let o = 8 + r * 48;
            if u32le(&g, o + 8) == TRANSMARVEL_GACHA {
                put_u32(&mut g, o, 100);
                put_u32(&mut g, o + 4, 0);
                break;
            }
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{SIGILS, Subs, GROUP_F};

    fn pool_rows(gl: &[u8]) -> Vec<usize> {
        (0..row_count(gl)).map(|r| 8 + r * 28)
            .filter(|&o| GEM_POOLS.contains(&u32le(gl, o + 8)))
            .collect()
    }
    fn rate_weight(grg: &[u8], pool: u32) -> Option<i32> {
        (0..row_count(grg)).map(|r| 8 + r * 16)
            .find(|&o| u32le(grg, o) == GEM_RATE_GROUP && u32le(grg, o + 4) == pool)
            .map(|o| i32le(grg, o + 8))
    }
    fn gem_row(gem: &[u8], key: u32) -> Option<usize> {
        (0..row_count(gem)).map(|r| 8 + r * 64).find(|&o| u32le(gem, o + 8) == key)
    }

    #[test]
    fn base_tables_well_formed() {
        assert_eq!(8 + row_count(BASE_GACHA) * 48, BASE_GACHA.len());
        assert_eq!(8 + row_count(BASE_GACHA_LOT) * 28, BASE_GACHA_LOT.len());
        assert_eq!(8 + row_count(BASE_GACHA_RATE_GROUP) * 16, BASE_GACHA_RATE_GROUP.len());
        assert_eq!(8 + row_count(BASE_GEM) * 64, BASE_GEM.len());
        assert_eq!(8 + row_count(BASE_SKILL_LOT) * 12, BASE_SKILL_LOT.len());
        assert_eq!(8 + row_count(BASE_SKILL_TYPE_LOT) * 52, BASE_SKILL_TYPE_LOT.len());
        // the allowed mask covers exactly the six groups
        assert_eq!(crate::data::TRAIT_COUNT, GROUPS.iter().map(|(_, g)| g.len()).sum::<usize>());
        // every gacha-eligible sigil fits at once, exact picks are ui-capped
        assert!(SIGILS.iter().filter(|s| s.gacha).count() <= pool_capacity());
        // every sigil key exists in gem.tbl
        for s in SIGILS { assert!(gem_row(BASE_GEM, s.key).is_some(), "{} missing from gem.tbl", s.name); }
    }

    #[test]
    fn single_grant_full_rate() {
        let g = Grant { key: SIGILS[0].key, level: 15, pin: None };
        let (gl, grg) = build_gacha_tables(&[g]);
        let rows = pool_rows(&gl);
        // first pool row carries the grant, gates cleared
        let o = rows[0];
        assert_eq!(u32le(&gl, o + 12), g.key);
        assert_eq!(i32le(&gl, o + 16), 50);
        assert_eq!(i32le(&gl, o + 20), 15);
        assert_eq!(u32le(&gl, o), 0);
        assert_eq!(u32le(&gl, o + 4), 0);
        assert_eq!(i32le(&gl, o + 24), 0);
        // every other pool row is dead
        for &o in &rows[1..] { assert_eq!(i32le(&gl, o + 16), 0); }
        // only the pool holding row 0 has rate weight
        let pool0 = u32le(&gl, rows[0] + 8);
        for &p in GEM_POOLS {
            assert_eq!(rate_weight(&grg, p), Some(if p == pool0 { 5000 } else { 0 }));
        }
    }

    #[test]
    fn grants_split_evenly() {
        let grants: Vec<Grant> = SIGILS.iter().take(5).map(|s| Grant { key: s.key, level: s.level, pin: None }).collect();
        let (gl, grg) = build_gacha_tables(&grants);
        let rows = pool_rows(&gl);
        for (i, g) in grants.iter().enumerate() {
            assert_eq!(u32le(&gl, rows[i] + 12), g.key);
            assert_eq!(i32le(&gl, rows[i] + 16), 50);
        }
        // total rate weight = grants * 5000, split per pool by row count
        let total: i32 = GEM_POOLS.iter().map(|&p| rate_weight(&grg, p).unwrap()).sum();
        assert_eq!(total, grants.len() as i32 * 5000);
    }

    #[test]
    fn capacity_of_sigils_at_once() {
        let cap = pool_capacity();
        let grants: Vec<Grant> = SIGILS.iter().take(cap).map(|s| Grant { key: s.key, level: s.level, pin: None }).collect();
        let (gl, grg) = build_gacha_tables(&grants);
        let active = pool_rows(&gl).iter().filter(|&&o| i32le(&gl, o + 16) > 0).count();
        assert_eq!(active, grants.len());
        let total: i32 = GEM_POOLS.iter().map(|&p| rate_weight(&grg, p).unwrap()).sum();
        assert_eq!(total, grants.len() as i32 * 5000);
    }

    #[test]
    fn pin_rewrites_gem_row() {
        // War Elemental+ pinned to Supplementary DMG
        let war = &SIGILS[0];
        let supp = GROUP_F.iter().find(|t| t.0 == "Supplementary DMG").unwrap().1;
        let gem = patched_gem(&[Grant { key: war.key, level: 15, pin: Some(supp) }], false);
        let o = gem_row(&gem, war.key).unwrap();
        assert_eq!(u32le(&gem, o + 4), supp);              // SkillId2 = pinned trait
        assert_eq!(i32le(&gem, o + 36), -1);               // random lot disabled
        // unpinned adds nothing beyond the sell-fix baseline
        assert_eq!(patched_gem(&[Grant { key: war.key, level: 15, pin: None }], false), patched_gem(&[], false));
    }

    #[test]
    fn sigil_only_toggle() {
        let g = patched_gacha(true);
        let o = (0..row_count(&g)).map(|r| 8 + r * 48)
            .find(|&o| u32le(&g, o + 8) == TRANSMARVEL_GACHA).unwrap();
        assert_eq!(u32le(&g, o), 100);    // gem/sigil chance
        assert_eq!(u32le(&g, o + 4), 0);  // wrightstone chance
        assert_eq!(patched_gacha(false), BASE_GACHA); // disabled -> untouched vanilla table
        // vanilla Transmarvel really is 75/25, catches base-table mixups
        assert_eq!(u32le(BASE_GACHA, o), 75);
        assert_eq!(u32le(BASE_GACHA, o + 4), 25);
    }

    #[test]
    fn native_flags_match_vanilla_pools() {
        // native must match presence in the embedded 2.0.2 pools, except the
        // four the dlc added to transmarvel after our base tables were dumped
        const DLC_NATIVE: &[u32] = &[
            0x2D85102A, // War Elemental+
            0x99E8B892, // Berserker Echo+
            0x4AE72C9E, // Spartan Echo+
            0x113035D8, // Super Ultimate Perfect Dodge+
        ];
        let rows = pool_rows(BASE_GACHA_LOT);
        for s in SIGILS {
            let present = rows.iter().any(|&o| u32le(BASE_GACHA_LOT, o + 12) == s.key);
            assert_eq!(present || DLC_NATIVE.contains(&s.key), s.native, "{}", s.name);
        }
    }

    fn trait_idx(name: &str) -> usize {
        GROUPS.iter().flat_map(|(_, g)| g.iter()).position(|t| t.0 == name).unwrap()
    }
    fn lot_row(stl: &[u8], key: u32) -> usize {
        (0..row_count(stl)).map(|r| 8 + r * 52).find(|&o| u32le(stl, o + 48) == key).unwrap()
    }

    #[test]
    fn trait_filter_all_or_none_is_vanilla() {
        use crate::data::TRAIT_COUNT;
        assert_eq!(build_trait_tables(&vec![true; TRAIT_COUNT]), (BASE_SKILL_LOT.to_vec(), BASE_SKILL_TYPE_LOT.to_vec()));
        assert_eq!(build_trait_tables(&vec![false; TRAIT_COUNT]), (BASE_SKILL_LOT.to_vec(), BASE_SKILL_TYPE_LOT.to_vec()));
        assert!(uncovered_lots(&vec![false; TRAIT_COUNT]).is_empty());
    }

    #[test]
    fn trait_filter_excludes_one() {
        use crate::data::TRAIT_COUNT;
        let mut allowed = vec![true; TRAIT_COUNT];
        allowed[trait_idx("Stamina")] = false; // group B
        let (sl, stl) = build_trait_tables(&allowed);
        // Stamina's group-B row is dead, every other governed row still weight 1
        let stamina = GROUPS[1].1.iter().find(|t| t.0 == "Stamina").unwrap().1;
        for r in 0..row_count(&sl) {
            let o = 8 + r * 12;
            if GROUPS.iter().any(|(h, _)| *h == u32le(&sl, o)) {
                let expect = if u32le(&sl, o + 4) == stamina { 0 } else { 1 };
                assert_eq!(i32le(&sl, o + 8), expect);
            }
        }
        // lot 16 group weights become allowed counts: A4 B20 C22 D8 E8 F9
        let o = lot_row(&stl, 16);
        let w: Vec<i32> = (0..6).map(|s| i32le(&stl, o + 24 + s * 4)).collect();
        assert_eq!(w, vec![4, 20, 22, 8, 8, 9]);
        // lot 5 (B C D E) and lot 4 (aliased D E) recompute too
        let o = lot_row(&stl, 5);
        let w: Vec<i32> = (0..4).map(|s| i32le(&stl, o + 24 + s * 4)).collect();
        assert_eq!(w, vec![20, 22, 8, 8]);
        let o = lot_row(&stl, 4);
        let w: Vec<i32> = (0..2).map(|s| i32le(&stl, o + 24 + s * 4)).collect();
        assert_eq!(w, vec![8, 8]);
        // ungoverned lots stay byte-identical to vanilla
        let o2 = lot_row(&stl, 2);
        assert_eq!(&stl[o2..o2 + 52], &BASE_SKILL_TYPE_LOT[o2..o2 + 52]);
        assert!(uncovered_lots(&allowed).is_empty());
    }

    #[test]
    fn trait_filter_uncovered_lots() {
        use crate::data::TRAIT_COUNT;
        // allow a single group-B trait: lots 4 (aliased D/E), 6 (C/D/E) and
        // 7 (D/E) lose all options
        let mut allowed = vec![false; TRAIT_COUNT];
        allowed[trait_idx("DMG Cap")] = true;
        let mut unc = uncovered_lots(&allowed);
        unc.sort_unstable();
        assert_eq!(unc, vec![4, 6, 7]);
    }

    #[test]
    fn subs_match_gem_table() {
        // every subs claim must match the real gem.tbl row. lot -1 means the
        // trait filter can never touch it (fixed and single-trait sigils),
        // rollable ones must reference the exact lot their trait list came from.
        // skill2 only checked on fixed gems, lot rows carry a placeholder there.
        use crate::data::{LOT15, LOT16, LOT26, LOT27, LOT5, LOT6, LOT7};
        for s in SIGILS {
            let o = gem_row(BASE_GEM, s.key).unwrap();
            let lot = i32le(BASE_GEM, o + 36);
            match s.subs {
                Subs::None => assert_eq!(lot, -1, "{}", s.name),
                Subs::Strip => {
                    // only meaningful on fixed rows that really carry a leftover trait
                    assert_eq!(lot, -1, "{}", s.name);
                    assert_ne!(u32le(BASE_GEM, o + 4), crate::data::NO_TRAIT, "{}", s.name);
                }
                Subs::Fixed(_) => {
                    assert_eq!(lot, -1, "{}", s.name);
                    assert_ne!(u32le(BASE_GEM, o + 4), 0, "{}", s.name);
                }
                Subs::Lot(g) if g == LOT7 => {
                    // lot 4 duplicates lot 7's pools under old group hashes
                    assert!(lot == 7 || lot == 4, "{}: lot {}", s.name, lot);
                }
                Subs::Lot(g) => {
                    let expect = if g == LOT5 { 5 } else if g == LOT6 { 6 } else if g == LOT15 { 15 }
                        else if g == LOT16 { 16 } else if g == LOT26 { 26 } else if g == LOT27 { 27 }
                        else { panic!("{}: unknown lot", s.name) };
                    assert_eq!(lot, expect, "{}", s.name);
                }
            }
        }
    }

    #[test]
    fn overlevel_lockout() {
        let base = patched_gem(&[], false);
        let locked = patched_gem(&[], true);
        let mut azurite = 0;
        for r in 0..row_count(&base) {
            let o = 8 + r * 64;
            if u32le(&base, o + 48) == 5 {
                assert_eq!(locked[o + 60], 0); // azurite off on rarity 5
                if base[o + 60] == 1 { azurite += 1; }
                // only the azurite byte may differ on a rarity-5 row
                assert_eq!(&locked[o..o + 60], &base[o..o + 60]);
                assert_eq!(&locked[o + 61..o + 64], &base[o + 61..o + 64]);
            } else {
                assert_eq!(&locked[o..o + 64], &base[o..o + 64]);
            }
        }
        assert_eq!(azurite, 533); // rarity-5 gems with azurite in the vanilla table
    }

    #[test]
    fn force_sellable_rows() {
        let g = patched_gem(&[], false);
        let mut hits = 0;
        for r in 0..row_count(BASE_GEM) {
            let o = 8 + r * 64;
            if FORCE_SELLABLE.contains(&u32le(BASE_GEM, o + 8)) {
                assert_eq!(BASE_GEM[o + 57], 1); // vanilla really marks them cantsell
                assert_eq!(g[o + 57], 0);
                assert_eq!(&g[o..o + 57], &BASE_GEM[o..o + 57]);
                assert_eq!(&g[o + 58..o + 64], &BASE_GEM[o + 58..o + 64]);
                hits += 1;
            } else {
                assert_eq!(&g[o..o + 64], &BASE_GEM[o..o + 64]);
            }
        }
        assert_eq!(hits, FORCE_SELLABLE.len());
    }

    #[test]
    fn synthesis_grand_success_lockout() {
        assert_eq!(8 + row_count(BASE_GEM_MIX_SUCCESS) * 12, BASE_GEM_MIX_SUCCESS.len());
        // vanilla sanity: combined level 44 rolls 5500 great / 4500 grand
        let o = (0..row_count(BASE_GEM_MIX_SUCCESS)).map(|r| 8 + r * 12)
            .find(|&o| u32le(BASE_GEM_MIX_SUCCESS, o + 8) == 44).unwrap();
        assert_eq!(u32le(BASE_GEM_MIX_SUCCESS, o), 5500);
        assert_eq!(u32le(BASE_GEM_MIX_SUCCESS, o + 4), 4500);
        assert_eq!(patched_gem_mix_success(false), BASE_GEM_MIX_SUCCESS);
        let locked = patched_gem_mix_success(true);
        for r in 0..row_count(&locked) {
            let o = 8 + r * 12;
            assert_eq!(u32le(&locked, o + 4), 0); // no grand success anywhere
            assert_eq!(u32le(&locked, o), u32le(BASE_GEM_MIX_SUCCESS, o)); // great untouched
            assert_eq!(u32le(&locked, o + 8), u32le(BASE_GEM_MIX_SUCCESS, o + 8));
        }
    }

    #[test]
    fn gem_rare_unlock() {
        assert_eq!(8 + row_count(BASE_GEM_RARE) * 12, BASE_GEM_RARE.len());
        // vanilla rarity-5 row is 11/15; only its max may change, and only on unlock
        let o = (0..row_count(BASE_GEM_RARE)).map(|r| 8 + r * 12)
            .find(|&o| u32le(BASE_GEM_RARE, o) == 5).unwrap();
        assert_eq!(i32le(BASE_GEM_RARE, o + 4), 11);
        assert_eq!(i32le(BASE_GEM_RARE, o + 8), 15);
        assert_eq!(patched_gem_rare(false), BASE_GEM_RARE);
        let g = patched_gem_rare(true);
        assert_eq!(i32le(&g, o + 4), 11);
        assert_eq!(i32le(&g, o + 8), 20);
        let mut expect = BASE_GEM_RARE.to_vec();
        expect[o + 8] = 20;
        assert_eq!(g, expect); // nothing else touched
    }

    #[test]
    fn strip_writes_no_trait_sentinel() {
        // a stripped grant must look exactly like the game's own single-trait gems
        use crate::data::NO_TRAIT;
        let s = SIGILS.iter().find(|s| s.subs == Subs::Strip).unwrap();
        let gem = patched_gem(&[Grant { key: s.key, level: s.level, pin: Some(NO_TRAIT) }], false);
        let o = gem_row(&gem, s.key).unwrap();
        assert_eq!(u32le(&gem, o + 4), NO_TRAIT);
        assert_eq!(i32le(&gem, o + 36), -1);
        // reference: Immortal Shell (the real single-trait gem) has the same slot value
        let base = SIGILS.iter().find(|s| s.name == "Immortal Shell").unwrap();
        let ob = gem_row(BASE_GEM, base.key).unwrap();
        assert_eq!(u32le(BASE_GEM, ob + 4), NO_TRAIT);
    }

    #[test]
    fn once_flags_match_gem_table() {
        // once must mirror the canonlyholdone byte (offset 59) for every sigil
        for s in SIGILS {
            let o = gem_row(BASE_GEM, s.key).unwrap();
            assert_eq!(BASE_GEM[o + 59] == 1, s.once, "{}", s.name);
        }
    }

    #[test]
    fn sub_pools_have_no_duplicates() {
        for s in SIGILS {
            if let Subs::Lot(groups) = s.subs {
                let all: Vec<u32> = crate::data::flatten(groups).map(|t| t.1).collect();
                let mut dedup = all.clone();
                dedup.sort_unstable();
                dedup.dedup();
                assert_eq!(all.len(), dedup.len(), "{} sub pool has duplicates", s.name);
            }
        }
    }
}
