# GBFRER Transmarvel Sigil Picker

Source code for the GUI app that comes with the Transmarvel Sigil Picker mod for
*Granblue Fantasy: Relink* (Endless Ragnarok). The source is up so you don't have to
trust a random unsigned exe from the internet - you can read what it does and build the
same thing yourself.

The mod itself is a Reloaded-II data mod. In vanilla, Transmarvel (the tier 4
transmutation for 150 Transmarvel) gives a random sigil 75% of the time and a random
wrightstone 25% of the time, and the sigil roll is almost always junk. This app edits
the mod's copies of eight game tables (gacha, gacha_lot, gacha_rate_group, gem,
skill_lot, skill_type_lot, gem_rare, gem_mix_success) so the sigil roll can only land
on the sigil(s) you picked. It can also set the tier to
100% sigil so wrightstones never steal a roll.

## What you can pick

Three pools, one per Add button:

Unique sigils, the ones sigil synthesis can never create:

- Specials: War Elemental+, Untouchable+, Flight over Fight+, Potent Greens+,
  Auto Potion+, Roll of the Die+, Natural Defenses, Stout Heart
- Echo sigils: Berserker Echo+, Spartan Echo+, Super Ultimate Perfect Dodge+
- Lucilius sigils: Alpha+, Beta+, Gamma+ (fixed DMG Cap 2nd trait)
- One-of-a-kind event sigils: Immortal Shell, In a Pinch, Sumo Force,
  Crabby Resonance, Crabs Are Forever+, Crabvestment Returns. The game blocks
  owning duplicates of these (and of every Awakening+); the app warns in red,
  because rolling one you already own gives nothing and the Transmarvel is
  wasted. Their level comes from the save's crab-event progress, not from the
  drop.
- Immortal Shell+, in two flavors, both flagged as cheats in the app. The
  stripped one grants at Lv20 with its stray hidden 2nd trait removed, same
  stats as the legit Lv20 Immortal Shell. The keep-trait one keeps that hidden
  Crabvestment Returns, which then runs at Lv20, past its natural Lv15 cap, a
  stat line vanilla never produces. Both are the same gem row, so only one can
  be picked at a time.
- Immortal Shell+ and Natural Defenses are granted sellable. Vanilla flags both
  as unsellable, and since duplicates of them are allowed here, re-rolled
  copies would be permanent junk. The sell flag stays cleared even after the
  pick is removed, so old copies can always be sold.

Character sigils, four per character for all 28 characters: the three "+"
sigils (each with a pinnable random 2nd trait) and the Awakening+ (fixed trait
pair). All of these are items vanilla Transmarvel rolls.

Transmarvel sigils, the 34 V+ sigils the vanilla pool rolls: the standard ones
(Damage Cap V+, Supplementary Damage V+, Attack Power V+ and the rest) plus the
six Celestials V+, Divergence V+ and Fatebreaker V+. All native rolls with
pinnable random 2nd traits, deduplicated against the other two pools.

For sigils with a random 2nd trait you can either leave it random or pin a
specific trait. The pin list is always that sigil's own vanilla trait pool, so a
pinned sigil is still a combination the game itself defines. With several picks
set, Transmarvel splits evenly between them.

Honesty note: most of the unique pool is real game sigils that vanilla
Transmarvel never rolls; this mod adds them to its pool and the GUI marks them
in amber. War Elemental+, the echoes and Super Ultimate Perfect Dodge+ became
genuine Transmarvel rolls with the DLC, so they carry no mark. Character and
Transmarvel sigils are all genuine vanilla rolls.

## The Lv20 pick

Grants clamp to the rarity-5 level cap, so picking an Immortal Shell+ also
raises that cap (gem_rare) from 15 to 20. To close the loopholes that opens,
the same Apply turns off azurite leveling for rarity-5 sigils and zeroes the
synthesis grand success weights (a grand success grants a gem's max level;
synthesis still works, results just stay at the default level). Nothing can
reach Lv20 except the picked grant. All of it exists only while such a pick is
set; remove it and Apply to restore vanilla.

## Gacha mode

Prefer randomness? Gacha mode replaces exact picks with three checklists, one
per pool. Every checked sigil enters the pool at even odds and 2nd traits stay
random, so it still feels like a gacha, minus the junk you unchecked. The
one-of-a-kind event sigils, both Immortal Shell+ flavors, Alpha+/Beta+/Gamma+,
Stout Heart and Natural Defenses are exact picks only and stay out of the
checklists.

Gacha mode also has a 2nd-trait checklist. Unchecked traits can no longer roll
as the random 2nd trait, and the allowed ones become equally likely. Fair
warning: this edits the shared trait pools, so it applies to every sigil in the
game that rolls a random 2nd trait, not only the ones from Transmarvel. Leaving
all (or none) checked keeps the vanilla pools untouched. Sigils with fixed
traits never roll from these pools, so the filter cannot change what they come
with.

## Is it safe?

- 100% offline. No networking anywhere in this code - no HTTP client, no sockets, no
  telemetry, no auto-update. The only dependencies are eframe/egui (the GUI), rfd (the
  native file dialog), serde_json (reads a local settings file) and winresource (embeds
  the icon at build time).
- What it actually touches: it reads and writes files only inside its own mod folder
  (the eight .tbl files plus a small picker_settings.json), and - only when you press
  Apply & Run Game - it starts Reloaded-II.exe to launch the game. No memory editing,
  no code injection, no background processes.
- Nothing to install. The Visual C++ runtime is statically linked (see
  .cargo/config.toml), so the .exe runs on a clean Windows 10/11.
- Unsigned binary. Windows SmartScreen may warn on first run. Build it yourself if
  you'd rather not trust a prebuilt exe.

## Antivirus

Defender's behavior monitor may flag the exe (seen once as
Behavior:Win32/DefenseEvasion.A!ml). It's a machine-learning heuristic, not a
signature match: an unsigned exe with no reputation that writes files next to
itself and starts another program (Reloaded-II, only when you press Apply & Run
Game) looks suspicious to it. There is no networking, no code injection and no
command execution beyond that one launch; the whole source is in this repo, so
build it yourself if in doubt. If Defender quarantines the exe: Windows
Security > Protection history > Allow, then re-extract the mod. False positives
can be reported to Microsoft's file submission page, which usually clears the
specific build.

## Build it yourself

Install a [Rust toolchain](https://rustup.rs/) (built and tested with 1.96, MSVC
target on Windows), then:

```sh
cargo build --release
```

The binary lands at `target/release/gbfrer-sigilis-picker.exe` (distributed in the mod
as `GBFRER Sigil Picker.exe`). Exact crate versions are pinned in Cargo.lock.

## How it works

Transmarvel is one row of `gacha.tbl` (75/25 sigil/wrightstone split). Its
sigil side points at a rate group in `gacha_rate_group.tbl`, which weights nine item
pools in `gacha_lot.tbl`. The app writes the picked gem ids over the first pool rows
(weight 50 each, quest and Endless Ragnarok gates cleared), zero-weights every other
row, and sets each pool's rate weight to 5000 per pick it holds, so N picks come out at
exactly 1/N each. Pinning a 2nd trait edits that gem's row in `gem.tbl`: the trait is
written as the explicit 2nd skill and the random lot is disabled, the same shape the
game uses for its own fixed-trait variants.

The 2nd-trait filter zero-weights unchecked traits in `skill_lot` and sets each
governed pool's group weight to its allowed-trait count in `skill_type_lot` (those
columns are weights, not percents - the vanilla lot 16 row sums to 120). The
skill_lot weight column is labeled "maybe weight" by the community table tools; the
vanilla data is consistent with a weight, but treat this one feature as needing an
in-game sanity check.

The app can also install itself: run as a lone .exe, it writes its own ModConfig.json
and default tables next to itself so Reloaded-II recognises it as a mod.

## What's in here

| Path | What it is |
|------|------------|
| `src/data.rs` | The sigil and trait tables (names, hashes, pools). |
| `src/patch.rs` | The embedded .tbl bytes and the byte patcher. |
| `src/reloaded.rs` | Reloaded-II detection, dependency check, launching. |
| `src/app.rs` | State and the actions (apply, run). |
| `src/ui.rs` | The egui layout and theme. |
| `base/*.tbl` | The vanilla game tables, embedded into the exe. |
| `dist/ModConfig.json` | The Reloaded-II mod manifest (also embedded, for self-install). |
| `build.rs`, `assets/` | The window and executable icon. |

## Credits

- **Nenkai** and all contributors - [GBFRDataTools](https://github.com/Nenkai/GBFRDataTools), the table tools and hashing.
- **Nenkai** and **WistfulHopes** - `gbfrelink.utility.manager`, the mod loader this mod depends on.
- **Sewer56 / Reloaded-Project** - Reloaded-II.
- Built with **egui / eframe**.

## License

MIT. Do what you want with the code, no warranty. The `base/*.tbl` files are
*Granblue Fantasy: Relink* data, included only for interoperability; all game assets
remain (c) their respective owners.
