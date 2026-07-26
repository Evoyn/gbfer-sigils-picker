// egui layout and theme, same crystal palette as the summon and wrightstone pickers

use eframe::egui;

use crate::app::{first_free, sigil, sub_name, sub_traits, App, Pick};
use crate::data::{Kind, Subs, GROUPS, SIGILS, TRAIT_COUNT};
use crate::patch::{pool_capacity, uncovered_lots};

pub const WINDOW_ICON: &[u8] = include_bytes!("../assets/icon_256.rgba");

const ACCENT: egui::Color32 = egui::Color32::from_rgb(0x8b, 0x7c, 0xf0);
const MUTED: egui::Color32 = egui::Color32::from_rgb(0x8b, 0x92, 0xa0);
const CARD: egui::Color32 = egui::Color32::from_rgb(0x1e, 0x20, 0x2a);
const LINE: egui::Color32 = egui::Color32::from_rgb(0x2d, 0x32, 0x41);
const WARN: egui::Color32 = egui::Color32::from_rgb(0xd9, 0xb4, 0x6a);
const RED: egui::Color32 = egui::Color32::from_rgb(0xe8, 0x92, 0x92);

pub fn setup_style(ctx: &egui::Context) {
    use egui::{Color32, CornerRadius, Stroke};
    let cr = CornerRadius::same(7);
    ctx.set_theme(egui::ThemePreference::Dark);
    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 7.0);
        style.spacing.interact_size.y = 30.0;
        style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(21.0, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(14.5, egui::FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(14.5, egui::FontFamily::Proportional));

        let mut v = egui::Visuals::dark();
        v.panel_fill = Color32::from_rgb(0x14, 0x15, 0x1c);
        v.window_fill = v.panel_fill;
        v.override_text_color = Some(Color32::from_rgb(0xdf, 0xe3, 0xea));
        v.faint_bg_color = CARD;
        v.extreme_bg_color = Color32::from_rgb(0x0e, 0x0f, 0x15);
        v.window_corner_radius = CornerRadius::same(10);
        v.selection.bg_fill = Color32::from_rgba_unmultiplied(0x8b, 0x7c, 0xf0, 96);
        v.selection.stroke = Stroke::new(1.0, ACCENT);
        v.hyperlink_color = ACCENT;
        let widget = Color32::from_rgb(0x28, 0x2c, 0x38);
        let hover = Color32::from_rgb(0x32, 0x37, 0x46);
        let active = Color32::from_rgb(0x3b, 0x41, 0x53);
        let text = Color32::from_rgb(0xdf, 0xe3, 0xea);
        for (w, fill) in [
            (&mut v.widgets.noninteractive, CARD),
            (&mut v.widgets.inactive, widget),
            (&mut v.widgets.hovered, hover),
            (&mut v.widgets.active, active),
            (&mut v.widgets.open, hover),
        ] {
            w.bg_fill = fill;
            w.weak_bg_fill = fill;
            w.bg_stroke = Stroke::new(1.0, LINE);
            w.corner_radius = cr;
            w.fg_stroke = Stroke::new(1.0, text);
        }
        v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0x6b, 0x5f, 0xc0));
        v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        v.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
        v.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        style.visuals = v;
    });
}

// combo-box style trigger button, label left, chevron right
fn combo_trigger(ui: &mut egui::Ui, width: f32, label: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, 28.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let vis = ui.style().interact(&resp);
        ui.painter().rect(rect, vis.corner_radius, vis.bg_fill, vis.bg_stroke, egui::StrokeKind::Inside);
        let c = vis.text_color();
        ui.painter().text(egui::pos2(rect.left() + 10.0, rect.center().y), egui::Align2::LEFT_CENTER, label, egui::FontId::proportional(14.5), c);
        // chevron is painted since the font may not have the glyph
        let cx = rect.right() - 13.0;
        let cy = rect.center().y;
        ui.painter().add(egui::Shape::convex_polygon(
            vec![egui::pos2(cx - 4.5, cy - 2.5), egui::pos2(cx + 4.5, cy - 2.5), egui::pos2(cx, cy + 3.0)],
            c, egui::Stroke::NONE,
        ));
    }
    resp
}

// dropdown with type-to-filter, egui's combobox has no search. items = (index, label).
// lists over 8 entries get a filter box pinned on top, the list scrolls under it
fn searchable_combo(ui: &mut egui::Ui, id_salt: (&'static str, usize), width: f32, items: &[(usize, &'static str)], selected: &mut usize, cur_label: &str) {
    let popup_id = ui.make_persistent_id(id_salt);
    let q_id = popup_id.with("q");
    let focus_id = popup_id.with("focus");

    let resp = combo_trigger(ui, width, cur_label);
    if resp.clicked() {
        // fresh filter + keyboard focus every time it opens
        ui.ctx().data_mut(|d| { d.insert_temp(q_id, String::new()); d.insert_temp(focus_id, true); });
    }

    egui::Popup::menu(&resp)
        .id(popup_id)
        .width(width)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            let mut q: String = ui.ctx().data(|d| d.get_temp(q_id).unwrap_or_default());
            if items.len() > 8 {
                let te = ui.add(egui::TextEdit::singleline(&mut q).hint_text("type to filter\u{2026}").desired_width(width));
                let want = ui.ctx().data_mut(|d| { let f = d.get_temp::<bool>(focus_id).unwrap_or(false); if f { d.insert_temp(focus_id, false); } f });
                if want { te.request_focus(); }
                ui.separator();
            }
            let needle = q.trim().to_lowercase();
            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                let mut shown = 0;
                for &(idx, label) in items {
                    if needle.is_empty() || label.to_lowercase().contains(&needle) {
                        shown += 1;
                        if ui.selectable_label(*selected == idx, label).clicked() {
                            *selected = idx;
                            egui::Popup::close_id(ui.ctx(), popup_id);
                        }
                    }
                }
                if shown == 0 { ui.weak("(no match)"); }
            });
            ui.ctx().data_mut(|d| d.insert_temp(q_id, q));
        });
}

// multi-select dropdown, stays open until click-outside. all/none act on the
// filtered list
fn checklist_combo(ui: &mut egui::Ui, id_salt: (&'static str, usize), width: f32, items: &[(usize, &'static str)], pool: &mut [bool], label: &str) -> bool {
    let popup_id = ui.make_persistent_id(id_salt);
    let q_id = popup_id.with("q");
    let focus_id = popup_id.with("focus");
    let mut changed = false;

    let resp = combo_trigger(ui, width, label);
    if resp.clicked() {
        ui.ctx().data_mut(|d| { d.insert_temp(q_id, String::new()); d.insert_temp(focus_id, true); });
    }

    egui::Popup::menu(&resp)
        .id(popup_id)
        .width(width)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            let mut q: String = ui.ctx().data(|d| d.get_temp(q_id).unwrap_or_default());
            let te = ui.add(egui::TextEdit::singleline(&mut q).hint_text("type to filter\u{2026}").desired_width(width));
            let want = ui.ctx().data_mut(|d| { let f = d.get_temp::<bool>(focus_id).unwrap_or(false); if f { d.insert_temp(focus_id, false); } f });
            if want { te.request_focus(); }
            let needle = q.trim().to_lowercase();
            ui.horizontal(|ui| {
                if ui.button("All").clicked() {
                    for &(idx, label) in items {
                        if (needle.is_empty() || label.to_lowercase().contains(&needle)) && !pool[idx] { pool[idx] = true; changed = true; }
                    }
                }
                if ui.button("None").clicked() {
                    for &(idx, label) in items {
                        if (needle.is_empty() || label.to_lowercase().contains(&needle)) && pool[idx] { pool[idx] = false; changed = true; }
                    }
                }
            });
            ui.separator();
            egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                let mut shown = 0;
                for &(idx, label) in items {
                    if needle.is_empty() || label.to_lowercase().contains(&needle) {
                        shown += 1;
                        if ui.checkbox(&mut pool[idx], label).changed() { changed = true; }
                    }
                }
                if shown == 0 { ui.weak("(no match)"); }
            });
            ui.ctx().data_mut(|d| d.insert_temp(q_id, q));
        });
    changed
}

// details line drops the v+/+ marker, the card title already shows it
fn short_name(name: &str) -> String {
    fn trim_plus(n: &str) -> &str {
        let n = n.strip_suffix('+').unwrap_or(n);
        n.strip_suffix(" V").unwrap_or(n)
    }
    match name.split_once(" (") {
        Some((base, rest)) => format!("{} ({}", trim_plus(base), rest),
        None => trim_plus(name).to_string(),
    }
}

fn summary(p: &Pick) -> String {
    let s = sigil(p);
    let name = short_name(s.name);
    let lv = if s.level > 0 { format!(" Lv{}", s.level) } else { String::new() };
    match s.subs {
        Subs::None => format!("{}{}   (single trait)", name, lv),
        Subs::Strip => format!("{}{}   (leftover 2nd trait removed)", name, lv),
        Subs::Fixed(t) => format!("{}{}   +   {}{} (fixed)", name, lv, t, lv),
        Subs::Lot(_) => match sub_name(p) {
            Some(t) => format!("{}{}   +   {}{}", name, lv, t, lv),
            None => format!("{}{}   +   random 2nd trait", name, lv),
        },
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        use egui::{Color32, CornerRadius, Margin, RichText, Stroke};
        enum Act { None, Apply, Run, Save, Recheck, Add(Kind), Remove(usize), Edit(usize), SavePick(usize) }
        let mut act = Act::None;

        if self.logo.is_none() {
            let img = egui::ColorImage::from_rgba_unmultiplied([256, 256], WINDOW_ICON);
            self.logo = Some(ui.ctx().load_texture("logo", img, egui::TextureOptions::LINEAR));
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
        // margin so nothing sits right on the window edge
        egui::Frame::NONE.inner_margin(Margin { left: 24, right: 18, top: 6, bottom: 10 }).show(ui, |ui| {
            ui.add_space(12.0);

            // header
            ui.horizontal(|ui| {
                if let Some(t) = &self.logo {
                    ui.add(egui::Image::new(egui::load::SizedTexture::new(t.id(), egui::vec2(48.0, 48.0))));
                }
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.heading("GBFRER Transmarvel Sigil Picker");
                    ui.label(RichText::new("Choose exactly which sigil(s) Transmarvel gives.").color(MUTED));
                });
            });
            ui.add_space(10.0);

            // dependency banner
            let (fill, msg) = if !self.dep.located {
                (Color32::from_rgb(0x40, 0x36, 0x1c), "Couldn't find Reloaded-II's Mods folder to check requirements.".to_string())
            } else if self.dep.ok {
                (Color32::from_rgb(0x17, 0x32, 0x26), "Loader ready - gbfrelink.utility.manager and its dependencies are installed.".to_string())
            } else {
                let miss: Vec<_> = self.dep.items.iter().filter(|(_, f)| !*f).map(|(n, _)| n.clone()).collect();
                (Color32::from_rgb(0x44, 0x22, 0x24), format!("Missing: {}  -  install it or the mod won't load.", miss.join(", ")))
            };
            egui::Frame::group(ui.style()).fill(fill).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(8)).inner_margin(Margin::same(10)).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| { ui.label(RichText::new(msg).strong()); });
                ui.collapsing("Requirement details", |ui| {
                    for (n, f) in &self.dep.items {
                        ui.horizontal(|ui| {
                            let (c, m) = if *f { (Color32::from_rgb(0x7b, 0xd6, 0x9a), "installed") } else { (RED, "MISSING ") };
                            ui.label(RichText::new(m).color(c).strong());
                            ui.label(RichText::new(n).monospace().color(MUTED));
                        });
                    }
                    if ui.button("Re-check").clicked() { act = Act::Recheck; }
                });
            });
            ui.add_space(12.0);

            // gacha preference: curate a pool instead of forcing exact picks
            egui::Frame::group(ui.style()).fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(8)).inner_margin(Margin::same(10)).show(ui, |ui| {
                if ui.checkbox(&mut self.gacha_mode, RichText::new("Prefer gacha (curate the pool, rolls stay random)").strong()).changed() { act = Act::Save; }
                ui.label(RichText::new("Instead of forcing exact picks, check the sigils allowed in the pool. Transmarvel rolls one of them at random (even split) and 2nd traits stay random.").color(MUTED).size(12.5));
            });
            ui.add_space(8.0);

            // gacha mode: pool checklists instead of the pick cards
            let mut trait_gap: Vec<u32> = Vec::new();
            if self.gacha_mode {
                for (bi, (title, kind)) in [("Unique sigils", Kind::Unique), ("Character sigils", Kind::Character), ("Transmarvel sigils", Kind::Transmarvel)].into_iter().enumerate() {
                    let items: Vec<(usize, &str)> = SIGILS.iter().enumerate()
                        .filter(|(_, s)| s.kind == kind && s.gacha).map(|(k, s)| (k, s.name)).collect();
                    let n = items.iter().filter(|(k, _)| self.pool[*k]).count();
                    egui::Frame::group(ui.style()).fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(9)).inner_margin(Margin::same(11)).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(title).size(15.5).strong().color(ACCENT));
                            ui.label(RichText::new(format!("{n} of {} in the pool", items.len())).color(MUTED));
                        });
                        ui.add_space(4.0);
                        let label = format!("{n} selected, click to edit");
                        if checklist_combo(ui, ("pool", bi), 380.0, &items, &mut self.pool, &label) { act = Act::Save; }
                    });
                    ui.add_space(8.0);
                }
                // 2nd trait filter
                let titems: Vec<(usize, &str)> = GROUPS.iter().flat_map(|(_, g)| g.iter()).enumerate().map(|(i, t)| (i, t.0)).collect();
                let tn = self.traits_allowed.iter().filter(|&&a| a).count();
                egui::Frame::group(ui.style()).fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(9)).inner_margin(Margin::same(11)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("2nd traits").size(15.5).strong().color(ACCENT));
                        ui.label(RichText::new(format!("{tn} of {TRAIT_COUNT} allowed")).color(MUTED));
                    });
                    ui.add_space(4.0);
                    let label = format!("{tn} allowed, click to edit");
                    if checklist_combo(ui, ("pooltraits", 0), 380.0, &titems, &mut self.traits_allowed, &label) { act = Act::Save; }
                    ui.label(RichText::new("Restricts the random 2nd trait of every sigil that rolls one, not only Transmarvel drops. Allowed traits become equally likely. All or none checked = vanilla rolls. Sigils with fixed traits (Awakening+, Alpha/Beta/Gamma+, single-trait ones) are never affected.").color(MUTED).size(12.5));
                });
                ui.add_space(8.0);
                trait_gap = uncovered_lots(&self.traits_allowed);
                if !trait_gap.is_empty() {
                    let pools: Vec<&str> = trait_gap.iter().map(|k| match *k {
                        6 => "echo / Celestial pool",
                        7 => "Untouchable / Flight over Fight pool",
                        15 => "character pool",
                        16 => "special pool",
                        26 => "Divergence pool",
                        27 => "Fatebreaker pool",
                        _ => "unknown pool",
                    }).collect();
                    ui.label(RichText::new(format!("No allowed 2nd trait left for: {}. Allow at least one trait from each affected pool.", pools.join(", "))).color(RED).size(12.5));
                    ui.add_space(8.0);
                }
                ui.label(RichText::new("Checked unique sigils that vanilla Transmarvel does not roll are added to its pool.").color(WARN).size(12.5));
                ui.add_space(8.0);
                if SIGILS.iter().enumerate().any(|(i, s)| s.once && self.pool[i]) {
                    ui.label(RichText::new("Some checked sigils are one of a kind: rolls that land on one you already own give nothing, the Transmarvel is wasted.").color(RED).size(12.5));
                    ui.add_space(8.0);
                }
                if SIGILS.iter().enumerate().any(|(i, s)| s.level > 15 && self.pool[i]) {
                    ui.label(RichText::new("A checked pick grants above Lv15: you can't upgrade using azurite while this is on, and Sigil Synthesis can't grand succeed.").color(RED).size(12.5));
                    ui.add_space(8.0);
                }
            }

            // add buttons (exact-pick mode)
            if !self.gacha_mode { ui.horizontal_wrapped(|ui| {
                // more picks than pool rows would silently never roll
                let room = self.active_grants().len() < pool_capacity();
                let free_unique = room && first_free(&self.picks, Kind::Unique).is_some();
                let free_char = room && first_free(&self.picks, Kind::Character).is_some();
                let free_tm = room && first_free(&self.picks, Kind::Transmarvel).is_some();
                let add_u = egui::Button::new(RichText::new("  +  Add unique sigils  ").color(Color32::WHITE).strong()).fill(ACCENT).min_size(egui::vec2(190.0, 32.0));
                if ui.add_enabled(free_unique, add_u).clicked() { act = Act::Add(Kind::Unique); }
                let add_c = egui::Button::new(RichText::new("  +  Add character sigils  ").color(Color32::WHITE).strong()).fill(ACCENT).min_size(egui::vec2(190.0, 32.0));
                if ui.add_enabled(free_char, add_c).clicked() { act = Act::Add(Kind::Character); }
                let add_t = egui::Button::new(RichText::new("  +  Add Transmarvel sigils  ").color(Color32::WHITE).strong()).fill(ACCENT).min_size(egui::vec2(190.0, 32.0));
                if ui.add_enabled(free_tm, add_t).clicked() { act = Act::Add(Kind::Transmarvel); }
                if !free_unique && !free_char && !free_tm {
                    ui.label(RichText::new("every sigil is already picked").color(MUTED));
                }
            }); ui.add_space(8.0); }

            // pick cards (exact-pick mode)
            let taken: Vec<usize> = self.picks.iter().map(|p| p.sigil).collect();
            if !self.gacha_mode { for i in 0..self.picks.len() {
                let p = &mut self.picks[i];
                egui::Frame::group(ui.style()).fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(9)).inner_margin(Margin::same(11)).show(ui, |ui| {
                    if p.editing {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(SIGILS[p.sigil].name).size(15.5).strong().color(ACCENT));
                            ui.label(RichText::new("(editing)").color(MUTED));
                        });
                        ui.add_space(6.0);
                        egui::Grid::new(("edit", i)).num_columns(2).spacing([12.0, 10.0]).min_col_width(70.0).show(ui, |ui| {
                            // this card's kind only, minus gems on other cards
                            // (same key = same gem)
                            ui.label(RichText::new("Sigil").color(MUTED));
                            let kind = SIGILS[p.sigil].kind;
                            let items: Vec<(usize, &str)> = SIGILS.iter().enumerate()
                                .filter(|(k, s)| s.kind == kind
                                    && (*k == p.sigil || !taken.iter().enumerate().any(|(ti, t)| ti != i && SIGILS[*t].key == s.key)))
                                .map(|(k, s)| (k, s.name)).collect();
                            let before = p.sigil;
                            let label = SIGILS[p.sigil].name;
                            searchable_combo(ui, ("sigil", i), 380.0, &items, &mut p.sigil, label);
                            if p.sigil != before { p.sub = 0; } // sub pool changed with the sigil
                            ui.end_row();

                            // 2nd trait
                            let s = sigil(p);
                            match s.subs {
                                Subs::Lot(_) => {
                                    ui.label(RichText::new("2nd trait").color(MUTED));
                                    let ts = sub_traits(s).unwrap_or_default();
                                    let mut items: Vec<(usize, &str)> = vec![(0, "Random (rolls from its normal pool)")];
                                    items.extend(ts.iter().enumerate().map(|(k, t)| (k + 1, t.0)));
                                    let label = sub_name(p).unwrap_or("Random (rolls from its normal pool)");
                                    searchable_combo(ui, ("sub", i), 340.0, &items, &mut p.sub, label);
                                    ui.end_row();
                                }
                                Subs::Fixed(t) => {
                                    ui.label(RichText::new("2nd trait").color(MUTED));
                                    ui.label(RichText::new(format!("{t} (fixed on this sigil)")).color(MUTED));
                                    ui.end_row();
                                }
                                Subs::None => {
                                    ui.label(RichText::new("2nd trait").color(MUTED));
                                    ui.label(RichText::new("none (single-trait sigil)").color(MUTED));
                                    ui.end_row();
                                }
                                Subs::Strip => {
                                    ui.label(RichText::new("2nd trait").color(MUTED));
                                    ui.label(RichText::new("removed by this mod (leftover data pairs it with Crabvestment Returns)").color(MUTED));
                                    ui.end_row();
                                }
                            }
                        });
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            let save = egui::Button::new(RichText::new("Save").strong()).min_size(egui::vec2(90.0, 30.0));
                            if ui.add(save).clicked() { act = Act::SavePick(i); }
                            if ui.button("Remove").clicked() { act = Act::Remove(i); }
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(SIGILS[p.sigil].name).color(ACCENT).strong().size(15.5));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Remove").clicked() { act = Act::Remove(i); }
                                if ui.button("Edit").clicked() { act = Act::Edit(i); }
                            });
                        });
                        ui.add_space(3.0);
                        ui.label(RichText::new(summary(p)).size(15.0).strong());
                    }
                    if !sigil(p).native {
                        ui.add_space(2.0);
                        ui.label(RichText::new("Not a vanilla Transmarvel roll - this sigil is added to the Transmarvel pool.").color(WARN).size(12.5));
                    }
                    if sigil(p).once {
                        ui.add_space(2.0);
                        ui.label(RichText::new("One of a kind: if you already own it, this roll gives nothing and the Transmarvel is wasted.").color(RED).size(12.5));
                    }
                    if sigil(p).level > 15 {
                        ui.add_space(2.0);
                        ui.label(RichText::new("You can't upgrade using azurite while this is on, and Sigil Synthesis can't grand succeed (results stay at the default level).").color(RED).size(12.5));
                    }
                    if let Some(w) = sigil(p).cheat {
                        ui.add_space(2.0);
                        ui.label(RichText::new(w).color(RED).size(12.5));
                    }
                });
                ui.add_space(8.0);
            } }

            // distribution message
            let total = self.active_grants().len();
            ui.add_space(2.0);
            egui::Frame::group(ui.style()).fill(Color32::from_rgb(0x17, 0x22, 0x32)).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(8)).inner_margin(Margin::same(10)).show(ui, |ui| {
                let txt = if total == 0 {
                    if self.gacha_mode { "Check at least one sigil above.".to_string() }
                    else { "Add at least one sigil above.".to_string() }
                } else if total == 1 {
                    "Every Transmarvel sigil roll will give this sigil (it's the only one).".to_string()
                } else {
                    format!("Each sigil roll gives {:.1}% of each pick (split evenly across {} sigils).", 100.0 / total as f32, total)
                };
                ui.label(RichText::new(txt).strong());
            });
            ui.add_space(10.0);

            // sigil-only toggle
            egui::Frame::group(ui.style()).fill(CARD).stroke(Stroke::new(1.0, LINE)).corner_radius(CornerRadius::same(8)).inner_margin(Margin::same(10)).show(ui, |ui| {
                if ui.checkbox(&mut self.sigil_only, RichText::new("Sigils only (skip wrightstones)").strong()).changed() { act = Act::Save; }
                ui.label(RichText::new("Transmarvel normally gives a wrightstone 25% of the time. With this on, every Transmarvel gives one of your picked sigils instead.").color(MUTED).size(12.5));
            });
            ui.add_space(12.0);

            // settings
            ui.collapsing("Reloaded-II / game paths (auto-detected)", |ui| {
                egui::Grid::new("paths").num_columns(3).spacing([8.0, 8.0]).show(ui, |ui| {
                    ui.label("Reloaded-II.exe");
                    ui.add(egui::TextEdit::singleline(&mut self.reloaded_path).desired_width(320.0));
                    if ui.button("Browse").clicked() {
                        if let Some(p) = rfd::FileDialog::new().add_filter("exe", &["exe"]).pick_file() { self.reloaded_path = p.display().to_string(); act = Act::Save; }
                    }
                    ui.end_row();
                    ui.label("Game .exe");
                    ui.add(egui::TextEdit::singleline(&mut self.game_path).desired_width(320.0));
                    if ui.button("Browse").clicked() {
                        if let Some(p) = rfd::FileDialog::new().add_filter("exe", &["exe"]).pick_file() { self.game_path = p.display().to_string(); act = Act::Save; }
                    }
                    ui.end_row();
                });
                ui.label(RichText::new("Leave the game path empty to just open Reloaded-II and press Launch there.").color(MUTED));
            });
            ui.add_space(14.0);

            // actions
            let have = total > 0 && trait_gap.is_empty();
            ui.horizontal(|ui| {
                if ui.add_enabled(have, egui::Button::new("Apply Picks").min_size(egui::vec2(150.0, 34.0))).clicked() { act = Act::Apply; }
                let run = egui::Button::new(RichText::new("Apply & Run Game").color(Color32::WHITE).strong()).fill(ACCENT).min_size(egui::vec2(220.0, 34.0));
                if ui.add_enabled(have, run).clicked() { act = Act::Run; }
            });
            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Status:").color(MUTED));
                ui.label(RichText::new(&self.status).strong());
            });
            ui.add_space(8.0);
        });
        }); // inner margin wrapper

        match act {
            Act::Apply => self.do_apply(),
            Act::Run => self.run_game(),
            Act::Save => self.save_config(),
            Act::Recheck => { self.dep = crate::reloaded::check_deps(); self.status = "Re-checked requirements.".into(); }
            Act::Add(kind) => {
                if let Some(free) = first_free(&self.picks, kind) {
                    self.picks.push(Pick { sigil: free, sub: 0, editing: true });
                }
            }
            Act::Edit(i) => { if let Some(p) = self.picks.get_mut(i) { p.editing = true; } }
            Act::SavePick(i) => { if let Some(p) = self.picks.get_mut(i) { p.editing = false; } self.save_config(); }
            Act::Remove(i) => { if i < self.picks.len() { self.picks.remove(i); } self.save_config(); }
            Act::None => {}
        }
    }
}
