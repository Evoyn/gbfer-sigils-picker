GBFRER Transmarvel Sigil Picker
===============================

Pick exactly which sigil(s) Transmarvel gives you: the unique sigils (War Elemental+,
Untouchable+, the echoes, Alpha/Beta/Gamma+, the Celestials and more) and every
character's own sigils.


WHAT IT DOES
------------
Normally Transmarvel gives a random sigil ~75% of the time, and the roll is almost
always junk. This mod replaces that roll with your own list. If you add one sigil you
get it 100% of the time; if you add several, Transmarvel splits evenly between them.

Three pools to pick from:
- Unique sigils: War Elemental+, Untouchable+, Flight over Fight+, Potent Greens+,
  Auto Potion+, Roll of the Die+, Natural Defenses, Stout Heart, the echo sigils
  (Berserker Echo+, Spartan Echo+, Super Ultimate Perfect Dodge+), Alpha+/Beta+/
  Gamma+, the one-of-a-kind event sigils (Immortal Shell, In a Pinch, Sumo Force,
  the crab set), and Immortal Shell+ (leftover data the game never hands out:
  granted at Lv20 with its stray 2nd trait removed; picking it raises the game's
  rarity-5 level cap from 15 to 20 while the mod is applied, see NOTES). A second
  Immortal Shell+ variant keeps the hidden Crabvestment Returns 2nd trait - a
  cheat sigil, its Lv20 stat line does not exist in vanilla. The two variants are
  the same gem, only one can be picked.
- Character sigils: for all 28 characters, the three "+" sigils (random 2nd trait)
  and the Awakening+ (fixed trait pair).
- Transmarvel sigils: the 34 V+ sigils the vanilla pool rolls - the standard ones
  (Damage Cap V+, Supplementary Damage V+, Attack Power V+ and the rest) plus the
  six Celestials V+, Divergence V+ and Fatebreaker V+. All genuine rolls, 2nd
  traits pinnable.

Sigils with a random 2nd trait can be left random or pinned to the trait you want.
The pin list is always that sigil's own trait pool, so a pinned sigil is still a
combination the game itself defines. Fixed sigils (the Awakening+ pairs,
Alpha/Beta/Gamma+ with their DMG Cap, single-trait sigils) always come as-is.

ONE OF A KIND: the game blocks owning duplicates of some sigils (the event sigils
above and every Awakening+). The app shows a red warning on those picks: rolling one
you already own gives NOTHING - the Transmarvel is spent for no item.

PREFER GACHA? There's a mode for that: instead of forcing exact picks, check every
sigil you want in the pool and Transmarvel rolls randomly among them, at even odds.
The one-of-a-kind event sigils, Alpha+/Beta+/Gamma+, Stout Heart and Natural
Defenses are exact picks only and never appear in gacha mode's checklists.
A separate checklist limits which traits can roll as the random 2nd trait; the
allowed ones become equally likely. Careful: that trait filter applies to every
sigil in the game that rolls a random 2nd trait, not only the ones from Transmarvel.

By default the mod also makes Transmarvel give a SIGIL EVERY TIME (no wrightstones).
Untick "Sigils only" to keep the vanilla 75% sigil / 25% wrightstone split. Lower
Transmutation tiers are never touched.

HONESTY NOTE: nothing in the unique pool is a roll vanilla Transmarvel gives
(War Elemental+ and the echoes appear in vanilla only as close variants). They are
real game sigils that this mod adds to the Transmarvel pool - the app marks them
in amber. Character and Transmarvel sigils are all genuine vanilla rolls.


HOW TO USE
----------
1. Make sure you have Reloaded-II with "gbfrelink.utility.manager" installed
   (the app checks this for you and shows a green banner when it's ready).
2. Run  "GBFRER Sigil Picker.exe".
3. Click  + Add unique sigils  or  + Add character sigils , choose the sigil and its
   2nd trait, press Save. Or tick "Prefer gacha" and curate the pool instead.
4. Press  Apply Picks  (or  Apply & Run Game  to launch straight away).
5. In Reloaded-II, enable ONLY this mod, launch Granblue Fantasy: Relink, and use
   Transmarvel. You'll only ever get the sigil(s) you picked.

Your choices are saved next to the .exe in picker_settings.json.


NOTES
-----
- Offline and self-contained. No network access, no installer. The .exe is also the
  mod - it writes its own tables next to itself.
- Only the Transmarvel (tier-4) roll is changed, plus - only in gacha mode with a
  trait filter set - the shared random-2nd-trait pools. Lower Transmutation tiers are
  untouched.
- Granted levels: the specials, the quest exclusives (Celestials, Divergence,
  Fatebreaker) and the character sigils all come at Lv15. The event sigils
  (Immortal Shell and friends) level from your save's event progress (crabs caught),
  not from the drop - a save with no crabs gets them at Lv1, and no table mod can
  raise that.
- Rarity-5 level cap: only while an Immortal Shell+ pick is set, the cap is raised
  from 15 to 20 so the grant can land at Lv20. To close the loopholes that opens,
  the same Apply turns off azurite leveling for all rarity-5 sigils AND removes
  Sigil Synthesis grand successes (a grand success grants the raised max; results
  now always come at the default level), so nothing can reach Lv20 except the
  picked grant. Remove the pick and Apply to restore everything. Vanilla drops
  sit at 15 or below either way.
- Immortal Shell+ and Natural Defenses are made sellable. Vanilla flags them
  unsellable, and re-rolled duplicates would be stuck in the inventory forever.
- Solo / offline use only, like every table mod.

ANTIVIRUS
---------
Defender's behavior monitor may flag the exe (Behavior:Win32/DefenseEvasion.A!ml
or similar). That's a machine-learning false positive: an unsigned exe that
writes its tables next to itself and starts Reloaded-II on request looks
suspicious to it. Nothing here touches the network or other processes. If it
gets quarantined: Windows Security > Protection history > Allow, then re-extract
the mod. The full source is public - build the exe yourself if in doubt.


Made for personal use. Values come straight from the game's own data files.
