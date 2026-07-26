// state and the actions (apply, run). gui lives in ui.rs

use std::fs;
use std::path::{Path, PathBuf};

use crate::data::{flatten, Kind, Sigil, Subs, GROUPS, NO_TRAIT, SIGILS, TRAIT_COUNT};
use crate::patch::{build_gacha_tables, build_trait_tables, patched_gacha, patched_gem, patched_gem_mix_success, patched_gem_rare, Grant};
use crate::reloaded::{autodetect, check_deps, DepReport};

pub const MODCONFIG_JSON: &str = include_str!("../dist/ModConfig.json");

#[derive(Clone)]
pub struct Pick {
    pub sigil: usize,
    // for lot sigils: 0 = random, n = flattened trait n-1
    pub sub: usize,
    pub editing: bool,
}

pub fn sigil(p: &Pick) -> &'static Sigil { &SIGILS[p.sigil] }

// pinnable traits of a sigil, none when the 2nd slot isn't rollable
pub fn sub_traits(s: &Sigil) -> Option<Vec<crate::data::Trait>> {
    match s.subs {
        Subs::Lot(groups) => Some(flatten(groups).collect()),
        _ => None,
    }
}

// first sigil of the kind whose gem key no other pick uses, same-key variants
// share one gem row so only one can be active
pub fn first_free(picks: &[Pick], kind: Kind) -> Option<usize> {
    (0..SIGILS.len()).find(|i| SIGILS[*i].kind == kind
        && !picks.iter().any(|p| SIGILS[p.sigil].key == SIGILS[*i].key))
}

// gacha mode: every checked eligible sigil enters the pool, subs stay random.
// same-key dupes: first entry wins
pub fn pool_grants(pool: &[bool]) -> Vec<Grant> {
    let mut out: Vec<Grant> = Vec::new();
    for (i, s) in SIGILS.iter().enumerate() {
        if !s.gacha || !pool.get(i).copied().unwrap_or(false) { continue }
        if out.iter().any(|g| g.key == s.key) { continue }
        out.push(Grant { key: s.key, level: s.level, pin: strip_pin(s) });
    }
    out
}

// strip gems always get their leftover 2nd trait cleared
fn strip_pin(s: &Sigil) -> Option<u32> {
    if s.subs == Subs::Strip { Some(NO_TRAIT) } else { None }
}

// display name of the pinned sub, none while random
pub fn sub_name(p: &Pick) -> Option<&'static str> {
    match (sigil(p).subs, p.sub) {
        (Subs::Lot(groups), n) if n > 0 => flatten(groups).nth(n - 1).map(|t| t.0),
        _ => None,
    }
}

// one grant per gem key, first pick wins on dupes
pub fn grants(picks: &[Pick]) -> Vec<Grant> {
    let mut out: Vec<Grant> = Vec::new();
    for p in picks {
        let s = sigil(p);
        if out.iter().any(|g| g.key == s.key) { continue }
        let pin = match (s.subs, p.sub) {
            (Subs::Lot(groups), n) if n > 0 => flatten(groups).nth(n - 1).map(|t| t.1),
            _ => strip_pin(s),
        };
        out.push(Grant { key: s.key, level: s.level, pin });
    }
    out
}

pub struct App {
    pub picks: Vec<Pick>,
    pub gacha_mode: bool,
    // gacha mode: checked state per SIGILS index
    pub pool: Vec<bool>,
    // gacha mode: allowed random 2nd traits, indexed in GROUPS flatten order
    pub traits_allowed: Vec<bool>,
    pub sigil_only: bool,
    pub reloaded_path: String,
    pub game_path: String,
    pub status: String,
    pub exe_dir: PathBuf,
    pub dep: DepReport,
    pub logo: Option<eframe::egui::TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        let exe_dir = std::env::current_exe().ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        let (mut reloaded_path, mut game_path) = autodetect();
        let mut picks: Vec<Pick> = Vec::new();
        let mut gacha_mode = false;
        let mut pool = vec![false; SIGILS.len()];
        let mut traits_allowed = vec![true; TRAIT_COUNT];
        let mut sigil_only = true; // default: skip wrightstones, always give a sigil

        let cfg = exe_dir.join("picker_settings.json");
        if let Ok(txt) = fs::read_to_string(&cfg) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(txt.trim_start_matches('\u{feff}')) {
                if let Some(p) = v.get("reloaded_path").and_then(|x| x.as_str()) { if !p.is_empty() { reloaded_path = p.to_string(); } }
                if let Some(p) = v.get("game_path").and_then(|x| x.as_str()) { if !p.is_empty() { game_path = p.to_string(); } }
                if let Some(b) = v.get("sigil_only").and_then(|x| x.as_bool()) { sigil_only = b; }
                if let Some(b) = v.get("gacha_mode").and_then(|x| x.as_bool()) { gacha_mode = b; }
                if let Some(arr) = v.get("gacha_pool").and_then(|x| x.as_array()) {
                    for nm in arr.iter().filter_map(|x| x.as_str()) {
                        if let Some(i) = SIGILS.iter().position(|s| s.name == nm) { pool[i] = true; }
                    }
                }
                if let Some(arr) = v.get("gacha_traits_excluded").and_then(|x| x.as_array()) {
                    for nm in arr.iter().filter_map(|x| x.as_str()) {
                        if let Some(i) = GROUPS.iter().flat_map(|(_, g)| g.iter()).position(|t| t.0 == nm) {
                            traits_allowed[i] = false;
                        }
                    }
                }
                if let Some(arr) = v.get("picks").and_then(|x| x.as_array()) {
                    for e in arr {
                        let Some(si) = e.get("sigil").and_then(|x| x.as_str())
                            .and_then(|nm| SIGILS.iter().position(|s| s.name == nm)) else { continue };
                        if picks.iter().any(|p| p.sigil == si) { continue }
                        let sub = e.get("sub").and_then(|x| x.as_str())
                            .and_then(|nm| sub_traits(&SIGILS[si])
                                .and_then(|ts| ts.iter().position(|t| t.0 == nm).map(|i| i + 1)))
                            .unwrap_or(0);
                        picks.push(Pick { sigil: si, sub, editing: false });
                    }
                }
            }
        }
        if picks.is_empty() { picks.push(Pick { sigil: 0, sub: 0, editing: true }); } // default: War Elemental+
        let g = if gacha_mode { pool_grants(&pool) } else { grants(&picks) };
        ensure_installed(&exe_dir, &g, sigil_only);
        Self {
            picks, gacha_mode, pool, traits_allowed, sigil_only, reloaded_path, game_path,
            status: "Add the sigil(s) you want from Transmarvel, then Apply.".into(),
            exe_dir, dep: check_deps(), logo: None,
        }
    }

    pub fn save_config(&self) {
        let picks: Vec<serde_json::Value> = self.picks.iter().map(|p| {
            serde_json::json!({ "sigil": sigil(p).name, "sub": sub_name(p).unwrap_or("Random") })
        }).collect();
        let gacha_pool: Vec<&str> = SIGILS.iter().enumerate()
            .filter(|(i, _)| self.pool[*i]).map(|(_, s)| s.name).collect();
        let excluded: Vec<&str> = GROUPS.iter().flat_map(|(_, g)| g.iter()).enumerate()
            .filter(|(i, _)| !self.traits_allowed[*i]).map(|(_, t)| t.0).collect();
        let v = serde_json::json!({
            "reloaded_path": self.reloaded_path, "game_path": self.game_path,
            "sigil_only": self.sigil_only, "picks": picks,
            "gacha_mode": self.gacha_mode, "gacha_pool": gacha_pool,
            "gacha_traits_excluded": excluded,
        });
        let _ = fs::write(self.exe_dir.join("picker_settings.json"), serde_json::to_string_pretty(&v).unwrap_or_default());
    }

    // gacha mode grants the curated pool, otherwise the exact picks
    pub fn active_grants(&self) -> Vec<Grant> {
        if self.gacha_mode { pool_grants(&self.pool) } else { grants(&self.picks) }
    }

    pub fn apply(&mut self) -> std::io::Result<()> {
        let dir = self.exe_dir.join("GBFR").join("data").join("system").join("table");
        // trait filter only acts in gacha mode, exact picks write vanilla lots
        let ta = if self.gacha_mode { self.traits_allowed.clone() } else { vec![true; TRAIT_COUNT] };
        write_tables(&dir, &self.active_grants(), self.sigil_only, &ta)?;
        self.save_config();
        Ok(())
    }

    pub fn do_apply(&mut self) {
        match self.apply() {
            Ok(()) => self.status = "Applied. Enable ONLY this mod in Reloaded-II, then (re)launch the game.".into(),
            Err(e) => self.status = format!("Could not write tables: {e}. Is the game running / folder read-only?"),
        }
    }

    pub fn run_game(&mut self) {
        if let Err(e) = self.apply() { self.status = format!("Could not write tables: {e}"); return; }
        let reloaded_ok = !self.reloaded_path.is_empty() && Path::new(&self.reloaded_path).exists();
        if !reloaded_ok {
            self.status = "Picks applied & saved. Reloaded-II.exe not found - set its path below (Browse), or launch the game through Reloaded-II yourself.".into();
            return;
        }
        let mut cmd = std::process::Command::new(&self.reloaded_path);
        if !self.game_path.is_empty() && Path::new(&self.game_path).exists() {
            cmd.arg("--launch").arg(&self.game_path);
        }
        match cmd.spawn() {
            Ok(_) => self.status = if self.game_path.is_empty() {
                "Applied. Opened Reloaded-II - press Launch there.".into()
            } else {
                "Applied & launching the game through Reloaded-II...".into()
            },
            Err(e) => self.status = format!("Applied, but couldn't start Reloaded-II: {e}. Launch it yourself."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::NO_TRAIT;

    #[test]
    fn same_key_variants_dedupe() {
        let a = SIGILS.iter().position(|s| s.name == "Immortal Shell+").unwrap();
        let b = SIGILS.iter().position(|s| s.name == "Immortal Shell+ (with Crabvestment Returns)").unwrap();
        assert_eq!(SIGILS[a].key, SIGILS[b].key);
        // exact picks: first wins, here the stripped variant
        let g = grants(&[
            Pick { sigil: a, sub: 0, editing: false },
            Pick { sigil: b, sub: 0, editing: false },
        ]);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].pin, Some(NO_TRAIT));
        // keep-trait variant alone leaves the gem row vanilla
        let g = grants(&[Pick { sigil: b, sub: 0, editing: false }]);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].pin, None);
        assert_eq!(g[0].level, 20);
        // gacha pool: event sigils are not gacha-eligible at all
        let mut pool = vec![false; SIGILS.len()];
        pool[a] = true;
        pool[b] = true;
        assert!(pool_grants(&pool).is_empty());
    }

    #[test]
    fn exact_pick_only_sigils_not_in_gacha() {
        let excluded = [
            "Immortal Shell", "Immortal Shell+", "Immortal Shell+ (with Crabvestment Returns)",
            "In a Pinch", "Sumo Force", "Crabby Resonance", "Crabs Are Forever+",
            "Crabvestment Returns", "Alpha+", "Beta+", "Gamma+", "Stout Heart",
            "Natural Defenses",
        ];
        for s in SIGILS {
            assert_eq!(s.gacha, !excluded.contains(&s.name), "{}", s.name);
        }
        let pool = vec![true; SIGILS.len()];
        assert_eq!(pool_grants(&pool).len(), SIGILS.iter().filter(|s| s.gacha).count());
    }
}

// all eight tables in one pass, shared by apply and the self-install
fn write_tables(dir: &Path, g: &[Grant], sigil_only: bool, traits_allowed: &[bool]) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    // grants above 15 need the r5 cap raised, the locks come with it
    let unlock = g.iter().any(|x| x.level > 15);
    let (gl, grg) = build_gacha_tables(g);
    fs::write(dir.join("gacha_lot.tbl"), gl)?;
    fs::write(dir.join("gacha_rate_group.tbl"), grg)?;
    fs::write(dir.join("gem.tbl"), patched_gem(g, unlock))?;
    fs::write(dir.join("gacha.tbl"), patched_gacha(sigil_only))?;
    let (sl, stl) = build_trait_tables(traits_allowed);
    fs::write(dir.join("skill_lot.tbl"), sl)?;
    fs::write(dir.join("skill_type_lot.tbl"), stl)?;
    fs::write(dir.join("gem_rare.tbl"), patched_gem_rare(unlock))?;
    fs::write(dir.join("gem_mix_success.tbl"), patched_gem_mix_success(unlock))
}

// run as a lone exe: write modconfig + default tables so reloaded-ii sees a mod
fn ensure_installed(exe_dir: &Path, g: &[Grant], sigil_only: bool) {
    let mc = exe_dir.join("ModConfig.json");
    if !mc.exists() { let _ = fs::write(&mc, MODCONFIG_JSON); }
    let dir = exe_dir.join("GBFR").join("data").join("system").join("table");
    if !dir.join("gacha_lot.tbl").exists() {
        let _ = write_tables(&dir, g, sigil_only, &vec![true; TRAIT_COUNT]);
    }
}
