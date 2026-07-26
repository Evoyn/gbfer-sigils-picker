// reloaded-ii integration: find the loader, check deps, autodetect paths.
// same approach as the summon and wrightstone pickers

use std::fs;
use std::path::{Path, PathBuf};

pub struct DepReport {
    pub located: bool,
    pub ok: bool,
    pub items: Vec<(String, bool)>,
}

fn mods_dir() -> Option<PathBuf> {
    let appdata = std::env::var("APPDATA").ok()?;
    let cfg = Path::new(&appdata).join("Reloaded-Mod-Loader-II").join("ReloadedII.json");
    let txt = fs::read_to_string(&cfg).ok()?;
    let v: serde_json::Value = serde_json::from_str(txt.trim_start_matches('\u{feff}')).ok()?;
    v.get("ModConfigDirectory").and_then(|x| x.as_str()).map(PathBuf::from)
}

pub fn check_deps() -> DepReport {
    let mods = mods_dir();
    let has = |id: &str| -> bool { mods.as_ref().map(|m| m.join(id).join("ModConfig.json").exists()).unwrap_or(false) };
    let core = "gbfrelink.utility.manager";
    let mut items = vec![(core.to_string(), has(core))];
    if let Some(m) = &mods {
        if let Ok(txt) = fs::read_to_string(m.join(core).join("ModConfig.json")) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(txt.trim_start_matches('\u{feff}')) {
                if let Some(arr) = v.get("ModDependencies").and_then(|x| x.as_array()) {
                    for d in arr.iter().filter_map(|x| x.as_str()) { items.push((d.to_string(), has(d))); }
                }
            }
        }
    }
    DepReport { ok: items.iter().all(|(_, f)| *f), located: mods.is_some(), items }
}

pub fn autodetect() -> (String, String) {
    let mut reloaded = String::new();
    let mut game = String::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        let cfg = Path::new(&appdata).join("Reloaded-Mod-Loader-II").join("ReloadedII.json");
        if let Ok(txt) = fs::read_to_string(&cfg) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(txt.trim_start_matches('\u{feff}')) {
                if let Some(lp) = v.get("LauncherPath").and_then(|x| x.as_str()) { reloaded = lp.to_string(); }
                if let Some(acd) = v.get("ApplicationConfigDirectory").and_then(|x| x.as_str()) {
                    if let Ok(rd) = fs::read_dir(acd) {
                        for e in rd.flatten() {
                            let ac = e.path().join("AppConfig.json");
                            if let Ok(t) = fs::read_to_string(&ac) {
                                if let Ok(j) = serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}')) {
                                    let loc = j.get("AppLocation").and_then(|x| x.as_str()).unwrap_or("");
                                    let id = j.get("AppId").and_then(|x| x.as_str()).unwrap_or("");
                                    if id.to_lowercase().contains("granblue") || loc.to_lowercase().contains("granblue") {
                                        game = loc.to_string();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if reloaded.is_empty() {
        for c in ["F:\\Reloaded-II\\Reloaded-II.exe", "C:\\Reloaded-II\\Reloaded-II.exe", "D:\\Reloaded-II\\Reloaded-II.exe"] {
            if Path::new(c).exists() { reloaded = c.to_string(); break; }
        }
    }
    (reloaded, game)
}
