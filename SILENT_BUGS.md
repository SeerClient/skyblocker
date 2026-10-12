# Silent-behaviour bugs in the frozen tree

Frozen tree `mods-src/skyblocker`, baseline revision `8dbf913`, compared with a fresh conversion in `C:/Users/y64/AppData/Local/Temp/claude/c--Users-y64-Desktop-McRust/b63c15a5-b680-4c4e-b573-c89b7f994ba8/scratchpad/f2s_round22`.

`line` is the line in the CURRENT frozen file holding the same text as the baseline line; `?` means the line has been edited since (it may already be fixed - check by hand).

| category | sites | still present |
|---|---|---|
| precedence | 50 | 34 |
| format-literal | 20 | 15 |
| format-indexed | 4 | 0 |
| pair-setter-read | 30 | 20 |
| never-null-array | 16 | 11 |
| integer-decode | 4 | 0 |
| parse-trim | 75 | 63 |
| lambda-return | 3 | 2 |
| mutate-copy | 6 | 4 |
| dropped-append | 48 | 34 |
| ptr-eq-fields | 2 | 1 |
| fill-additive | 2 | 0 |
| overload | 12 | 12 |
| arrays-tostring | 8 | 2 |
| never-null | 157 | 142 |

## precedence (50)

parentheses around an `a || b` operand of `&&` were dropped (`(a || b) && c` became `a || b && c`)

- `src/de/hysky/skyblocker/skyblock/bazaar/reorder_helper.rs:155` (baseline :155) `if slot == 11 || slot == 13 && stack.is_item(mcrust::net::minecraft::world::item::items::dyed_terracotta().green()) && input_constants::is_key_down(compat::minecraft().get_window().clone(), mcrust::co`
- `src/de/hysky/skyblocker/skyblock/chat/chat_rule_config_screen.rs:481` (baseline :481) `if only_if_formatted && !(ChatRule::get_include_formatting(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone()))) || ChatRule::get_regex(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone()`
- `src/de/hysky/skyblocker/skyblock/crimson/kuudra/kuudra_waypoints.rs:119` (baseline :119) `if Utils::is_in_kuudra() && compat::read(&config, |__t| __t.supply_waypoints) || compat::read(&config, |__t| __t.fuel_waypoints) || compat::read(&config, |__t| __t.ballista_build_waypoints) && mcrust:`
- `src/de/hysky/skyblocker/skyblock/crimson/kuudra/kuudra_waypoints.rs:123` (baseline :123) `if compat::read(&config, |__t| __t.supply_waypoints) || compat::read(&config, |__t| __t.fuel_waypoints) && mcrust::mcrust::modding::game::level().is_some() {`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_map.rs:?` (baseline :143) `while i < (DungeonPlayerManager::get_players().unwrap().len() as i32) && dungeon_player.is_none() || !(DungeonPlayer::alive(dungeon_player.as_ref().unwrap())) {`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_splits_widget.rs:?` (baseline :478) `} else if !showing_current && compat::read(__this, |__t| __t.running) || compat::read(__this, |__t| __t.start_time) != 0 {`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_one_flow.rs:?` (baseline :353) `if compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) && *lever_type != LeverType::Water && times.is_empty() || times[0] != 0.0 {`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_previewer.rs:?` (baseline :138) `if WaterboardPreviewer::is_water_passable(__this, &left.immutable().below()) && left_steps <= 5 && left_steps < right_steps || !(WaterboardPreviewer::is_water_passable(__this, &right.immutable().below`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_previewer.rs:?` (baseline :141) `} else if WaterboardPreviewer::is_water_passable(__this, &right.immutable().below()) && right_steps <= 5 && right_steps < left_steps || !(WaterboardPreviewer::is_water_passable(__this, &left.immutable`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:?` (baseline :245) `return (x < 13 || x > 17 || z > 2 && z < 28 && z < 13 || z > 17 || x > 2 && x < 28);`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/secret_waypoint.rs:?` (baseline :301) `if compat::read(&SkyblockerConfigManager::get().dungeons.secret_waypoints, |__t| __t.adaptive_box_size) && compat::read(__this, |__t| __t.category.clone()).unwrap() == Category::Wither || compat::read`
- `src/de/hysky/skyblocker/skyblock/dwarven/corpse_finder.rs:146` (baseline :146) `if { let __f = compat::dispatch(&corpse.waypoint, |__t| __t.should_render()); __f(&corpse.waypoint) } && corpse.seen || crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && Skyblocker`
- `src/de/hysky/skyblocker/skyblock/dwarven/profittrackers/powder_mining_tracker.rs:209` (baseline :209) `if !(compat::read(__this, |__t| __t.inside_chest_message)) && message == "  CHEST LOCKPICKED " || SkyblockerConfigManager::get().mining.crystal_hollows.count_natural_chests_in_tracker && message == " `
- `src/de/hysky/skyblocker/skyblock/entity/glow/adder/slayer_glow_adder.rs:98` (baseline :98) `if SkyblockerConfigManager::get().slayers.blaze_slayer.attunement_highlights && SlayerManager::is_fighting_slayer_type(SlayerType::Demonlord) && entity.as_any().unwrap_or(&() as &dyn std::any::Any).is`
- `src/de/hysky/skyblocker/skyblock/experiment/experiment_solver.rs:147` (baseline :147) `if __this.lock().borrow().as_any().is::<compat::Shared<SuperpairsSolver>>() || __this.lock().borrow().as_any().is::<compat::Shared<UltrasequencerSolver>>() && ({ let __f = compat::dispatch(__this, |__`
- `src/de/hysky/skyblocker/skyblock/fancybars/fancy_status_bars.rs:419` (baseline :419) `return (SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars && !(Utils::is_in_the_rift()) || SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars_rift);`
- `src/de/hysky/skyblocker/skyblock/fishing/fishing_helper.rs:86` (baseline :86) `if Utils::is_on_skyblock() && SkyblockerConfigManager::get().helpers.fishing.enable_fishing_helper || SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Hud {`
- `src/de/hysky/skyblocker/skyblock/foraging/galatea/sweep_details_hud_widget.rs:233` (baseline :233) `return (!(Utils::get_location() == Location::Hub) || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Forest)) || compat::read(&`
- `src/de/hysky/skyblocker/skyblock/garden/visitor/visitor_helper.rs:133` (baseline :133) `return (is_helper_enabled && !is_garden_mode || Utils::is_in_garden() || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Bazaar`
- `src/de/hysky/skyblocker/skyblock/garden/visitor/visitor_helper.rs:223` (baseline :223) `if slot_id == 29 || slot_id == 13 || slot_id == 33 && slot.has_item() && ItemUtils::get_lore_line_if(slot.get_item(), Arc::new(move |s| (s == "Click to give!" || s == "Click to refuse!"))).is_some() {`
- `src/de/hysky/skyblocker/skyblock/hunting/floor_drops.rs:141` (baseline :141) `return (SkyblockerConfigManager::get().hunting.floor_drops.highlight_floor_drops && Utils::is_in_galatea() || Utils::is_in_torrhus_canyon() || Utils::is_in_safari());`
- `src/de/hysky/skyblocker/skyblock/hunting/safari/safari_utils.rs:37` (baseline :37) `return (Utils::is_in_safari() && Utils::is_in_biome(sky_block_biomes::ICY.get_mut()) || Utils::is_in_biome(sky_block_biomes::ICY_CAVES.get_mut()));`
- `src/de/hysky/skyblocker/skyblock/item/skyblock_crafting_table_screen.rs:?` (baseline :199) `return (!(compat::read(__this, |__t| __t.narrow)) || !(compat::write(&__o1, |__t| __t.is_visible())) && __this.lock().borrow_mut().base.is_hovering_box(x, y, width, height, point_x, point_y));`
- `src/de/hysky/skyblocker/skyblock/item/slottext/slot_text_manager.rs:?` (baseline :111) `if Utils::is_on_skyblock() || !({ let __f = compat::dispatch(adder, |__t| __t.skyblock_only()); __f(&compat::view(adder, |a| a as compat::Shared<dyn ContainerMatcher>)) }) && { let __f = compat::dispa`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/avg_bin_tooltip.rs:?` (baseline :125) `if r#type == Average::OneDay || r#type == Average::Both && tooltip_info_type::ONE_DAY_AVERAGE.get_mut().lock().borrow().has_or_null_warning(&skyblock_api_id) {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/avg_bin_tooltip.rs:?` (baseline :131) `if r#type == Average::ThreeDay || r#type == Average::Both && tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow().has_or_null_warning(&skyblock_api_id) {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/true_hex_display.rs:?` (baseline :118) `if !item_id.is_empty() && item_id.starts_with("DYE_") || item_id.ends_with("_DYE") {`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipes/centered_recipe.rs:82` (baseline :82) `return (should_split(size) && size.wrapping_rem(2) == 1 || size >= 8);`
- `src/de/hysky/skyblocker/skyblock/radial_menu/radial_button.rs:110` (baseline :110) `if hovered && GuiHelper::has_shift_down() || SkyblockerConfigManager::get().ui_and_visuals.radial_menu.tooltips_without_shift {`
- `src/de/hysky/skyblocker/skyblock/slayers/boss/sven/mute_wolf_sounds.rs:29` (baseline :29) `if SkyblockerConfigManager::get().slayers.wolf_slayer.mute_wolf_sounds && Utils::is_in_park() || Utils::is_in_hub() {`
- `src/de/hysky/skyblocker/skyblock/slayers/boss/tarantula/mute_spider_sounds.rs:50` (baseline :50) `if SkyblockerConfigManager::get().slayers.spider_slayer.mute_spider_sounds && Utils::is_in_spiders_den() || Utils::is_in_crimson() {`
- `src/de/hysky/skyblocker/skyblock/storageoverlay/storage_overlay_screen.rs:171` (baseline :171) `let mut enabled: bool = (SkyblockerConfigManager::get().ui_and_visuals.storage_overlay.enabled && title == "storage" || compat::get_state(&OPEN_STORAGE) != -1 && !(compat::get_state(&DISABLE_ON_NEXT_L`
- `src/de/hysky/skyblocker/skyblock/storageoverlay/storage_overlay_screen.rs:295` (baseline :295) `return (__this.lock().borrow_mut().base.is_hovering_slot(slot, xm, ym) && compat::read(__this, |__t| __t.grid.clone()).is_none() || slot.container.as_any().is::<Inventory>() || (({ let __r = &compat::`
- `src/de/hysky/skyblocker/skyblock/storageoverlay/storage_overlay_screen.rs:604` (baseline :604) `if compat::read(__this, |__t| __t.open_backpack.clone()).is_some() && compat::write(&__o3, |__t| __t.get_bottom()) < (__this.lock().borrow().get_y()).wrapping_add(HEADER_H) || compat::write(&__o4, |__`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/side_panel_widget.rs:187` (baseline :187) `if self.is_open() && open_x != self.target_x || right_side != self.right_side {`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/widgets_configuration_screen.rs:515` (baseline :515) `if click.button() == 1 && !(__this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open()) || !({ let (__a0,) = (__this.lock().borrow().side_panel_widget.as_ref().unwrap().get_positioned_widget`
- `src/de/hysky/skyblocker/skyblock/tabhud/util/player_list_manager.rs:236` (baseline :236) `if !(string.starts_with(" ")) && string.contains(":") && !(hypixel_widget_name.1.starts_with("Mining Event")) || !(string.to_lowercase().starts_with("ends in")) {`
- `src/de/hysky/skyblocker/skyblock/teleport/predictive_smooth_aote.rs:371` (baseline :371) `if PredictiveSmoothAOTE::is_block_floor(&check_pos.below()) || PredictiveSmoothAOTE::is_block_floor(&check_pos.below().offset_vec(x_diagonal_offset.as_vec3i())) && PredictiveSmoothAOTE::is_block_floor`
- `src/de/hysky/skyblocker/skyblock/waypoint/abstract_waypoints_screen.rs:311` (baseline :311) `if !self.is_focused() && mouse_x <= self.get_x().wrapping_sub(50) || mouse_x >= self.get_right().wrapping_add(50) || mouse_y <= self.get_y().wrapping_sub(50) || mouse_y >= self.get_bottom().wrapping_a`
- `src/de/hysky/skyblocker/skyblock/waypoint/fairy_souls.rs:197` (baseline :197) `if fairy_souls_config.enable_fairy_souls_helper && fairy_souls_config.highlight_on_bingo || !(Utils::is_on_bingo()) && FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().is_done() && FAIRY_SOULS.get_mut()`
- `src/de/hysky/skyblocker/utils/command/argumenttypes/color/hex_color_argument_type.rs:?` (baseline :49) `if character < '0' || character > '9' && character < 'a' || character > 'f' && character < 'A' || character > 'F' {`
- `src/de/hysky/skyblocker/utils/container/container_solver_manager.rs:?` (baseline :96) `if Utils::is_on_skyblock() || !({ let __f = compat::dispatch(solver, |__t| __t.skyblock_only()); __f(&compat::view(solver, |a| a as compat::Shared<dyn ContainerMatcher>)) }) && { let __f = compat::dis`
- `src/de/hysky/skyblocker/utils/fun_utils.rs:54` (baseline :54) `return (date.get_month_value() == 12 && date.get_day_of_month() == 24 || date.get_day_of_month() == 25);`
- `src/de/hysky/skyblocker/utils/render/gui/color_picker_widget.rs:128` (baseline :128) `if compat::read(__this, |__t| __t.dragging_h) || compat::read(__this, |__t| __t.dragging_sv) || compat::read(__this, |__t| __t.dragging_a) && compat::read(__this, |__t| __t.on_color_change.clone()).is`
- `src/de/hysky/skyblocker/utils/text_transformer.rs:?` (baseline :59) `} else if style.get_color().is_some() && needs_reset || !(previous.get_color().unwrap() == style.get_color().unwrap()) {`
- `src/de/hysky/skyblocker/utils/text_transformer.rs:?` (baseline :167) `if compat::char_at(&legacy, i) != legacy_prefix && i == 0 || compat::char_at(&legacy, i.wrapping_sub(1)) != legacy_prefix {`
- `src/mixins/abstract_container_screen_mixin.rs:216` (baseline :216) `if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_empty_tooltips && is_title_empty_or_filler && !({ let __r = crate::de::hysky::sky`
- `src/mixins/client_packet_listener_mixin.rs:242` (baseline :242) `if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && options.predictive && !(compat::get_state(&crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::TELEPORT_`
- `src/mixins/local_player_mixin.rs:13` (baseline :13) `if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(<_ as mcrust::net::minec`
- `src/mixins/skin_texture_downloader_mixin.rs:39` (baseline :39) `if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.dont_strip_skin_alpha_values && crate::de::hysky::skyblocker::utils::utils::Utils::is_`

## format-literal (20)

a `String.format` specifier left in a `format!` string: the format is printed literally and its arguments dropped - use `compat::string_format`

- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_splits_widget.rs:?` (baseline :474) `let mut diff_str: String = format!("%+.2fs");`
- `src/de/hysky/skyblocker/skyblock/dungeon/fire_freeze_staff_timer.rs:50` (baseline :53) `message = component::literal("in ").append((component::literal(&format!("{}s", format!("{:.2}", (((compat::get_state(&FIRE_FREEZE_TIMER)) as f32) / (1000 as f32)))))).with_style_format(ChatFormatting:`
- `src/de/hysky/skyblocker/skyblock/dungeon/fire_freeze_staff_timer.rs:50` (baseline :91) `message = component::literal("in ").append((component::literal(&format!("{}s", format!("{:.2}", (((compat::get_state(&FIRE_FREEZE_TIMER)) as f32) / (1000 as f32)))))).with_style_format(ChatFormatting:`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_one_flow.rs:340` (baseline :334) `text = (component::literal(&format!("{:.2}", time_to_show))).with_style_format(ChatFormatting::Yellow);`
- `src/de/hysky/skyblocker/skyblock/events/greatspook/math_teacher_helper.rs:49` (baseline :49) `let mut result: String = format!("{:.0}", Calculator::calculate(expression.clone()));`
- `src/de/hysky/skyblocker/skyblock/garden/farming_hud_widget.rs:316` (baseline :316) `let mut yaw: MutableComponent = (if camera_entity.is_none() { component::translatable("skyblocker.farming.farmingHud.noCameraEntity") } else { component::literal(&format!("{:.2}", mth::wrap_degrees_f3`
- `src/de/hysky/skyblocker/skyblock/garden/farming_hud_widget.rs:317` (baseline :317) `let mut pitch: MutableComponent = (if camera_entity.is_none() { component::translatable("skyblocker.farming.farmingHud.noCameraEntity") } else { component::literal(&format!("{:.2}", mth::wrap_degrees_`
- `src/de/hysky/skyblocker/skyblock/item/slottext/adders/prehistoric_egg_adder.rs:122` (baseline :122) `walked_str = format!("{:.1}k", ((walked as f32) / 1000.0f32));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/evolving_item_progress_tooltip.rs:135` (baseline :135) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Age:").with_style_format(ChatFormatting::Gray)), &vec![(((((component::empty().append((component::literal(&format!("{:3.1}", (((sec`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/motes_tooltip.rs:?` (baseline :58) `let mut total_price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/motes_tooltip.rs:?` (baseline :62) `let mut each_price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/item_tooltip.rs:?` (baseline :60) `let mut price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/item_tooltip.rs:?` (baseline :64) `let mut price_string_total: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/utils/profile_viewer_utils.rs:28` (baseline :28) `return format!("%.4gB");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/utils/profile_viewer_utils.rs:30` (baseline :30) `return format!("%.4gM");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/utils/profile_viewer_utils.rs:32` (baseline :32) `return format!("%.4gK");`
- `src/de/hysky/skyblocker/skyblock/slayers/boss/voidgloom/lazer_timer.rs:57` (baseline :57) `let mut text: MutableComponent = (component::literal(&format!("{:.1}s", compat::get_state(&REMAINING_TIME)))).with_style_format(ChatFormatting::Aqua);`
- `src/de/hysky/skyblocker/skyblock/slayers/features/slayer_timer.rs:164` (baseline :164) `return format!("{:.2}s", ((millis as f64) / 1000.0));`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/element/progress_element.rs:107` (baseline :107) `let __this = Self::new(ico.clone(), description.clone(), Some(component::null_to_empty(Some(&format!("{:.2}%", percent)))), percent, color);`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/fire_sale_widget.rs:173` (baseline :173) `let mut prgress_txt: MutableComponent = component::literal(&format!("{}/{:.0}", avail, total));`

## format-indexed (4)

an indexed `String.format` specifier (`%1$`) printed literally

- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/motes_tooltip.rs:?` (baseline :58) `let mut total_price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/motes_tooltip.rs:?` (baseline :62) `let mut each_price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/item_tooltip.rs:?` (baseline :60) `let mut price_string: String = format!("%1$,.1f");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/item_tooltip.rs:?` (baseline :64) `let mut price_string_total: String = format!("%1$,.1f");`

## pair-setter-read (30)

a mutable pair's setter (`pair.first(v)` / `.left(v)`) emitted as a READ: the value is never stored

- `src/de/hysky/skyblocker/skyblock/end/ender_nodes.rs:99` (baseline :99) `particles.0;`
- `src/de/hysky/skyblocker/skyblock/end/ender_nodes.rs:101` (baseline :101) `particles.1;`
- `src/de/hysky/skyblocker/skyblock/end/ender_nodes.rs:185` (baseline :185) `entry.1.0;`
- `src/de/hysky/skyblocker/skyblock/end/ender_nodes.rs:186` (baseline :186) `entry.1.1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:322` (baseline :322) `compat::read(__this, |__t| __t.resize_hover.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:323` (baseline :323) `compat::read(__this, |__t| __t.resize_hover.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:322` (baseline :329) `compat::read(__this, |__t| __t.resize_hover.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :460) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :461) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :477) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :479) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :481) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :484) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :486) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :488) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :495) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :496) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:461` (baseline :498) `compat::read(__this, |__t| __t.resized_bars.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bars_config_screen.rs:460` (baseline :499) `compat::read(__this, |__t| __t.resized_bars.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :97) `compat::read((&__this_1), |__t| __t.current_color.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :102) `compat::read((&__this_2), |__t| __t.current_color.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :125) `compat::read((&__this_1), |__t| __t.current_color.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :130) `compat::read((&__this_2), |__t| __t.current_color.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :135) `compat::read((&__this_3), |__t| __t.current_color.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/color_popup.rs:?` (baseline :140) `compat::read((&__this_4), |__t| __t.current_color.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/visitor/get_render_width_visitor.rs:?` (baseline :96) `compat::read(__this, |__t| __t.widths.clone()).0;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/visitor/get_render_width_visitor.rs:?` (baseline :100) `compat::read(__this, |__t| __t.widths.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/visitor/get_render_width_visitor.rs:?` (baseline :110) `compat::read(__this, |__t| __t.widths.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/visitor/get_render_width_visitor.rs:?` (baseline :115) `compat::read(__this, |__t| __t.widths.clone()).1;`
- `src/de/hysky/skyblocker/skyblock/museum/price_data.rs:47` (baseline :47) `piece.1;`

## never-null-array (16)

a null test on an array ELEMENT emitted as a constant: Java's object arrays hold nulls (`Vec<Option<T>>`)

- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/party_entry.rs:?` (baseline :239) `if true /* java: __this.lock().borrow().party_entry_base().party_members[(i) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:?` (baseline :132) `if true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ && compat::read(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize], |__t| __t.dirty) {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:?` (baseline :176) `if slot.get_item().is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().red()) || true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:?` (baseline :194) `if true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:?` (baseline :226) `if false /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/collections/collections_page.rs:114` (baseline :114) `if false /* java: __this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/collections/collections_page.rs:130` (baseline :130) `if true /* java: __this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/inventory_page.rs:138` (baseline :138) `if false /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/inventory_page.rs:154` (baseline :154) `if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/inventory_page.rs:154` (baseline :187) `if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/inventory_page.rs:154` (baseline :203) `if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/profile_viewer_screen.rs:232` (baseline :232) `if true /* java: __this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/profile_viewer_screen.rs:232` (baseline :325) `if true /* java: __this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/searchoverlay/overlay_screen.rs:260` (baseline :260) `if (__this.lock().borrow().history_buttons.len() as i32) > 0 && true /* java: __this.lock().borrow().history_buttons[(0) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/searchoverlay/search_over_manager.rs:376` (baseline :376) `if (compat::lock(&SUGGESTIONS_ARRAY).len() as i32) > index && true /* java: compat::lock(&SUGGESTIONS_ARRAY)[(index) as usize] != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:556` (baseline :556) `if cc < (dictionary.len() as i32) && true /* java: dictionary[(cc) as usize] != null: never null here */ {`

## integer-decode (4)

`Integer.decode` as a decimal parse (fails on `#RRGGBB` / `0x..`)

- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/color_tooltip.rs:?` (baseline :65) `lines.insert((1) as usize, (component::literal(&format!("{}{}(", existing_tooltip, compat::jstring(&ChatFormatting::DarkGray)))).append((r#type).get_translated_text()).append(component::literal(" - ")`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/true_hex_display.rs:?` (baseline :126) `text = text.copy().with_color(stringified.trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/true_hex_dye_screen_display.rs:?` (baseline :132) `siblings.push(component::literal(&hex).with_color(hex.trim().parse::<i32>().expect("NumberFormatException")));`
- `src/de/hysky/skyblocker/utils/command/argumenttypes/color/hex_color_argument_type.rs:?` (baseline :53) `return (format!("#{}", input)).trim().parse::<i32>().expect("NumberFormatException");`

## parse-trim (75)

`Integer.parseInt` with a `.trim()` Java does not do (Java throws on surrounding whitespace); when the Java is `Integer.decode`, the `0x` / `#` / octal forms fail too (see integer-decode)

- `src/de/hysky/skyblocker/skyblock/auction/auction_browser_screen.rs:356` (baseline :356) `{ let __v = ((parts[(0) as usize]).replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.current_page = __v); }`
- `src/de/hysky/skyblocker/skyblock/auction/auction_browser_screen.rs:357` (baseline :357) `{ let __v = ((parts[(1) as usize]).replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.total_pages = __v); }`
- `src/de/hysky/skyblocker/skyblock/barn/treasure_hunter.rs:82` (baseline :82) `let mut request_message: MutableComponent = (constants::PREFIX.get_mut())().append((component::translatable_args("skyblocker.waypoints.chat.display", (compat::regex::split(&location, " ")).iter().clon`
- `src/de/hysky/skyblocker/skyblock/bazaar/bazaar_max.rs:66` (baseline :66) `self.last_seen_max = (max_quantity_matcher.as_mut().unwrap().group(1).unwrap().replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/bazaar/bazaar_order_tracker.rs:84` (baseline :84) `let mut amount: i32 = (((matchers[(1) as usize]).group(1).unwrap()).replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/chest_value.rs:198` (baseline :198) `profit = (profit - (found_string.trim().parse::<i32>().expect("NumberFormatException") as f64));`
- `src/de/hysky/skyblocker/skyblock/chocolatefactory/chocolate_factory_solver.rs:239` (baseline :239) `{ let __v = (amount_string.trim().parse::<i64>().expect("NumberFormatException")).wrapping_sub(current_chocolate.clone().unwrap()); compat::write(__this, |__t| __t.required_until_next_prestige = __v);`
- `src/de/hysky/skyblocker/skyblock/chocolatefactory/time_tower_reminder.rs:111` (baseline :111) `time = file.clone().next().unwrap().trim().parse::<i64>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/compact_damage.rs:71` (baseline :71) `let mut prettified_dmg: String = CompactDamage::prettify_damage_number(dmg.trim().parse::<i64>().expect("NumberFormatException"), config.max_precision);`
- `src/de/hysky/skyblocker/skyblock/crimson/kuudra/kuudra.rs:104` (baseline :104) `let mut amount: i32 = found_string.trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_score.rs:371` (baseline :371) `return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("rooms").unwrap().trim().parse::<i32>().expect("NumberFormatException") } else { 0 });`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_score.rs:406` (baseline :406) `return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("count").unwrap().trim().parse::<i32>().expect("NumberFormatException") } else { 0 });`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_score.rs:435` (baseline :435) `return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("crypts").unwrap().trim().parse::<i32>().expect("NumberFormatException") } else { 0 });`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/finder_settings_container.rs:248` (baseline :246) `le_min = (compat::trim(&(min_and_max[(0) as usize]))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/finder_settings_container.rs:254` (baseline :252) `le_max = (compat::trim(&(min_and_max[(1) as usize]))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/party_entry.rs:143` (baseline :140) `{ let __v = matcher.group(0).unwrap().trim().parse::<i32>().expect("NumberFormatException"); compat::write(&__this, |__t| __t.party_entry_base_mut().min_class_level = __v); }`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/party_entry.rs:148` (baseline :145) `{ let __v = matcher.group(0).unwrap().trim().parse::<i32>().expect("NumberFormatException"); compat::write(&__this, |__t| __t.party_entry_base_mut().min_catacombs_level = __v); }`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/party_entry.rs:218` (baseline :215) `class_level = matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/ranged_value_widget.rs:102` (baseline :102) `let mut i: i32 = compat::trim(&string).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/ranged_value_widget.rs:225` (baseline :225) `{ let __v = input_trimmed.trim().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.min = __v); }`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/ranged_value_widget.rs:232` (baseline :232) `{ let __v = input_trimmed.trim().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.max = __v); }`
- `src/de/hysky/skyblocker/skyblock/dungeon/preview/preview_room.rs:75` (baseline :75) `let mut secret_index: i32 = (if secret_index_matcher.find() { secret_index_matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException") } else { 0 });`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/dungeon_blaze.rs:95` (baseline :84) `let mut health: i32 = (compat::regex::replace_all(&(compat::substring2(&blaze_name, (compat::index_of(&blaze_name, "/")).wrapping_add(1), (compat::str_len(&blaze_name)).wrapping_sub(1))), ",", "")).tr`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:771` (baseline :747) `let mut secret_index: i32 = (if secret_index_matcher.find() { secret_index_matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException") } else { 0 });`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:872` (baseline :849) `{ let __v = matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.room_base_mut().secrets_found = __v); }`
- `src/de/hysky/skyblocker/skyblock/dwarven/corpse_finder.rs:241` (baseline :241) `x = matcher.group_named("x").unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dwarven/corpse_finder.rs:242` (baseline :242) `y = matcher.group_named("y").unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dwarven/corpse_finder.rs:243` (baseline :243) `z = matcher.group_named("z").unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:147` (baseline :147) `let mut block_pos: BlockPos = BlockPos::new(matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException"), matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatExcepti`
- `src/de/hysky/skyblocker/skyblock/dwarven/glacite_cold_overlay.rs:74` (baseline :74) `compat::set_state(&COLD, value.trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/skyblock/foraging/galatea/tuner_solver.rs:200` (baseline :200) `let mut current_speed: i32 = (compat::trim(&(parts[(1) as usize]))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/garden/garden_plots_widget.rs:254` (baseline :254) `self.infected_plots.push(GARDEN_PLOT_TO_SLOT.get_mut().get(&s.trim().to_string().trim().parse::<i32>().expect("NumberFormatException")).cloned().unwrap_or_else(|| -1));`
- `src/de/hysky/skyblocker/skyblock/item/slottext/adders/essence_shop_adder.rs:138` (baseline :138) `return SlotText::bottom_right_list(&(component::literal(&formatters::SHORT_FLOAT_NUMBERS.get_mut().format((essence_amount.trim().parse::<i32>().expect("NumberFormatException")) as f64))).with_color(sl`
- `src/de/hysky/skyblocker/skyblock/item/slottext/adders/your_essence_adder.rs:139` (baseline :139) `return SlotText::bottom_right_list(&(component::literal(&formatters::SHORT_FLOAT_NUMBERS.get_mut().format((essence_amount.trim().parse::<i32>().expect("NumberFormatException")) as f64))).with_color(sl`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/bits_helper.rs:256` (baseline :254) `let mut bits_cost: i64 = (bits_matcher.group_named("amount").unwrap().replace(",", "")).trim().parse::<i64>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/bits_helper.rs:256` (baseline :457) `let mut bits_cost: i64 = (bits_matcher.group_named("amount").unwrap().replace(",", "")).trim().parse::<i64>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/color_tooltip.rs:?` (baseline :65) `lines.insert((1) as usize, (component::literal(&format!("{}{}(", existing_tooltip, compat::jstring(&ChatFormatting::DarkGray)))).append((r#type).get_translated_text()).append(component::literal(" - ")`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/color_tooltip.rs:76` (baseline :76) `return Some(format!("{:02X}{:02X}{:02X}", (rgbvalues[(0) as usize]).trim().parse::<i32>().expect("NumberFormatException"), (rgbvalues[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/true_hex_display.rs:?` (baseline :126) `text = text.copy().with_color(stringified.trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/true_hex_dye_screen_display.rs:?` (baseline :132) `siblings.push(component::literal(&hex).with_color(hex.trim().parse::<i32>().expect("NumberFormatException")));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:276` (baseline :255) `return (echest.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_sub(1);`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:280` (baseline :259) `return (backpack.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_add(8);`
- `src/de/hysky/skyblocker/skyblock/itemlist/item_stack_builder.rs:117` (baseline :117) `let rarity = &rarities[(((compat::regex::split(&internal_name, ";"))[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException")) as usize];`
- `src/de/hysky/skyblocker/skyblock/profileviewer/collections/generic_category.rs:162` (baseline :162) `let mut c_tier: i32 = (compat::trim(&(parts[(0) as usize]))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/item_loaders/backpack_item_loader.rs:70` (baseline :70) `let mut key1: i32 = e1.0.trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/item_loaders/backpack_item_loader.rs:71` (baseline :71) `let mut key2: i32 = e2.0.trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/pet.rs:216` (baseline :216) `let mut number_key: i32 = (compat::substring2(&placeholder, 1, (compat::str_len(&placeholder)).wrapping_sub(1))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/item_loader.rs:54` (baseline :54) `backpacks.insert(entry.0.trim().parse::<i32>().expect("NumberFormatException"), Backpack::new(icon.clone(), contents.clone()));`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/pet_loader.rs:194` (baseline :194) `let mut index: i32 = other_number_matcher.group_named("index").unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/slayers/boss/demonlord/fire_pillar_announcer.rs:41` (baseline :41) `let mut seconds: i32 = matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs:157` (baseline :157) `let mut level: i32 = matcher_next_lvl.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs:158` (baseline :158) `let mut xp_remaining: i32 = (compat::trim(&(xp_remaining_str.replace(",", "")))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs:163` (baseline :163) `let mut level: i32 = (compat::regex::replace_all(&message, "(\\d+).+", "$1")).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/speedpreset/speed_preset_list_widget.rs:457` (baseline :457) `return Some((compat::write(&__o1, |__t| __t.filtered_edit_box_base_mut().get_value()), (compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().get_value())).trim().parse::<i32>().expect("NumberFo`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/crop_milestones_widget.rs:187` (baseline :187) `let mut level: i32 = matcher.group_named("level").unwrap().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/waypoint/waypoints.rs:272` (baseline :272) `let mut version: i32 = (compat::substring2(&waypoints_string, 25, compat::index_of(&waypoints_string, ')'))).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/skyblock/waypoint/waypoints_list_widget.rs:1035` (baseline :1035) `return (if value.is_empty() || value == "-" { 0 } else { value.trim().parse::<i32>().expect("NumberFormatException") });`
- `src/de/hysky/skyblocker/utils/command/argumenttypes/color/hex_color_argument_type.rs:?` (baseline :53) `return (format!("#{}", input)).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/utils/config/duration_controller.rs:56` (baseline :56) `result = result.wrapping_add((hours_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_mul(3600));`
- `src/de/hysky/skyblocker/utils/config/duration_controller.rs:59` (baseline :59) `result = result.wrapping_add((minutes_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_mul(60));`
- `src/de/hysky/skyblocker/utils/config/duration_controller.rs:62` (baseline :62) `result = result.wrapping_add(seconds_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_string_nbt_reader.rs:?` (baseline :140) `return byte_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i8>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_string_nbt_reader.rs:?` (baseline :143) `return long_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i64>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_string_nbt_reader.rs:?` (baseline :146) `return short_tag::value_of((compat::substring2(&input, 0, (compat::str_len(&input)).wrapping_sub(1))).trim().parse::<i16>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_string_nbt_reader.rs:?` (baseline :149) `return int_tag::value_of(input.trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/item_utils.rs:422` (baseline :412) `return Some(((drill_fuel_strings[(0) as usize]).trim().parse::<i32>().expect("NumberFormatException"), ((drill_fuel_strings[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException")).wrappi`
- `src/de/hysky/skyblocker/utils/pos_utils.rs:29` (baseline :29) `return BlockPos::new((pos_array[(0) as usize]).trim().parse::<i32>().expect("NumberFormatException"), (pos_array[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException"), (pos_array[(2) as`
- `src/de/hysky/skyblocker/utils/regex_utils.rs:?` (baseline :85) `return Some((s.replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/regex_utils.rs:?` (baseline :96) `return Some((s.replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/regex_utils.rs:?` (baseline :101) `return (matcher.group(group).unwrap().replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/utils/regex_utils.rs:?` (baseline :106) `return (matcher.group_named(group).unwrap().replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/utils/utils.rs:366` (baseline :364) `bits = (compat::regex::replace_all(&bits_string, "[^0-9.]", "")).trim().to_string().trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/utils/utils.rs:668` (baseline :655) `return Some(input.trim().parse::<i32>().expect("NumberFormatException"));`
- `src/de/hysky/skyblocker/utils/waypoint/named_waypoint.rs:525` (baseline :521) `let mut i1: i32 = num1.trim().parse::<i32>().expect("NumberFormatException");`
- `src/de/hysky/skyblocker/utils/waypoint/named_waypoint.rs:526` (baseline :522) `let mut i2: i32 = num2.trim().parse::<i32>().expect("NumberFormatException");`

## lambda-return (3)

`return` inside a forEach / ifPresent lambda returns from the whole METHOD here; Java returns from the lambda (next element)

- `src/de/hysky/skyblocker/skyblock/item/custom/screen/dye_select_popup.rs:?` (baseline :213) `return;`
- `src/de/hysky/skyblocker/skyblock/museum/museum_item_cache.rs:114` (baseline :208) `return;`
- `src/de/hysky/skyblocker/utils/utils.rs:382` (baseline :441) `return;`

## mutate-copy (6)

a loop that mutates each element of a copy of the collection (the originals never change)

- `src/de/hysky/skyblocker/config/screens/powdertracker/item_tick_list.rs:152` (baseline :159) `for child in compat::read(__this, |__t| __t.children.clone()) {`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_one_flow.rs:472` (baseline :383) `for mark in compat::read(__this, |__t| __t.marks.clone()) {`
- `src/de/hysky/skyblocker/skyblock/dungeon/puzzle/waterboard/waterboard_one_flow.rs:472` (baseline :464) `for mark in compat::read(__this, |__t| __t.marks.clone()) {`
- `src/de/hysky/skyblocker/skyblock/dwarven/profittrackers/corpse/reward_list.rs:434` (baseline :434) `for child in compat::read(__this, |__t| __t.children.clone()) {`
- `src/de/hysky/skyblocker/utils/exclusive_grouping_set.rs:?` (baseline :59) `for set in compat::read(__this, |__t| __t.sets.clone()) {`
- `src/de/hysky/skyblocker/utils/exclusive_grouping_set.rs:?` (baseline :67) `for set in compat::read(__this, |__t| __t.sets.clone()) {`

## dropped-append (48)

`component.append(..)` / `withStyle(..)` as a statement: the port's call returns the new component and the statement drops it (Java mutates in place)

- `src/de/hysky/skyblocker/skyblock/barn/call_trevor.rs:100` (baseline :100) `call_message.with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command("/call trevor".to_string())))).with_color_format(Some(ChatFormatting::Aqua)));`
- `src/de/hysky/skyblocker/skyblock/bazaar/bazaar_quick_quantities.rs:45` (baseline :45) `text.with_style_fn(|style| style.with_color_format(Some(ChatFormatting::Aqua)));`
- `src/de/hysky/skyblocker/skyblock/calculators/calculator_command.rs:59` (baseline :59) `text.append(component::literal(calculation).with_style_format(ChatFormatting::Aqua));`
- `src/de/hysky/skyblocker/skyblock/calculators/calculator_command.rs:60` (baseline :60) `text.append_str(" = ");`
- `src/de/hysky/skyblocker/skyblock/calculators/calculator_command.rs:61` (baseline :61) `text.append(component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(Calculator::calculate(calculation.to_string()))).with_style_format(ChatFormatting::Green));`
- `src/de/hysky/skyblocker/skyblock/calculators/calculator_command.rs:64` (baseline :64) `text.append(component::translatable("skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation").with_style_format(ChatFormatting::Red));`
- `src/de/hysky/skyblocker/skyblock/chat/sack_message_price.rs:113` (baseline :113) `copy.append(SackMessagePrice::deep_copy(sibling));`
- `src/de/hysky/skyblocker/skyblock/chocolatefactory/chocolate_factory_solver.rs:698` (baseline :698) `(result.append((component::literal("/")).with_style_format(ChatFormatting::Gray))).append((component::literal(&compat::read(__this, |__t| __t.purchased_hitman_slots).to_string())).with_style_format(Ch`
- `src/de/hysky/skyblocker/skyblock/compact_damage.rs:98` (baseline :98) `prettier_custom_name.append(component::literal(matcher.group(2).as_deref().unwrap()).set_style((siblings[siblings.len() - 1]).get_style().clone()));`
- `src/de/hysky/skyblocker/skyblock/dungeon/terminal_hud.rs:?` (baseline :245) `display_text.append_str(" ").append(component::translatable("skyblocker.dungeons.terminalHud.destroyedStatus").with_style_format(ChatFormatting::Green));`
- `src/de/hysky/skyblocker/skyblock/dungeon/terminal_hud.rs:?` (baseline :247) `display_text.append_str(" ").append(component::translatable("skyblocker.dungeons.terminalHud.incompleteStatus").with_style_format(ChatFormatting::Red));`
- `src/de/hysky/skyblocker/skyblock/dwarven/call_mismyla.rs:105` (baseline :105) `call_message.with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command("/call mismyla".to_string())))).with_color_format(Some(ChatFormatting::Aqua)));`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:256` (baseline :256) `text.append((component::literal(&format!("[{}]", waypoint_location))).with_color(location_color).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker cry`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:269` (baseline :269) `text.append(component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.remove").append(component::literal(": ")));`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:272` (baseline :272) `text.append(component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.share").append(component::literal(": ")));`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:277` (baseline :277) `text.append((component::literal(&format!("[{}]", waypoint_location))).with_color(location_color).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker cry`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:342` (baseline :342) `received_waypoint_names.append(component::literal(&category.get_name()).with_color(category.get_color()));`
- `src/de/hysky/skyblocker/skyblock/dwarven/crystals_locations_manager.rs:344` (baseline :344) `received_waypoint_names.append_str(", ");`
- `src/de/hysky/skyblocker/skyblock/dwarven/redial_on_bad_signal.rs:105` (baseline :105) `call_message.with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/call {}", name))))).with_color_format(Some(ChatFormatting::Aqua)));`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bar.rs:528` (baseline :528) `literal.append_str(&((compat::read(__this, |__t| __t.status_bar_base().to_display.clone()).unwrap()))(compat::read(__this, |__t| __t.status_bar_base().overflow.clone())));`
- `src/de/hysky/skyblocker/skyblock/fancybars/status_bar.rs:529` (baseline :529) `text.append(literal.clone());`
- `src/de/hysky/skyblocker/skyblock/garden/visitor/visitor_helper.rs:287` (baseline :287) `item_text.append((component::literal(" ✔ ")).with_style_format(ChatFormatting::Green));`
- `src/de/hysky/skyblocker/skyblock/health_bars.rs:156` (baseline :156) `cleaned_text.append(parts[(i) as usize].clone());`
- `src/de/hysky/skyblocker/skyblock/health_bars.rs:156` (baseline :197) `cleaned_text.append(parts[(i) as usize].clone());`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/name/visitor/get_clicked_position_visitor.rs:?` (baseline :56) `self.text.append(text1.clone());`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/motes_tooltip.rs:?` (baseline :64) `message.append((component::literal(&format!("({} each)", each_price_string.replace(".0", "")))).with_style_format(ChatFormatting::Gray));`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :294) `text.with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(c`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :508) `empty.append(component::literal(" (").with_style_format(ChatFormatting::Gray));`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :511) `empty.append(component::literal(")").with_style_format(ChatFormatting::Gray));`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :582) `empty.append(self.display_name());`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :587) `empty.append_str(": ").append(apply.clone());`
- `src/de/hysky/skyblocker/skyblock/item/value_breakdown_popup.rs:?` (baseline :651) `empty.append(apply.clone());`
- `src/de/hysky/skyblocker/skyblock/museum/item_filter.rs:69` (baseline :69) `tooltip.append(self.get_filter_text(i, &component::literal(&category_name)));`
- `src/de/hysky/skyblocker/skyblock/museum/item_filter.rs:72` (baseline :72) `(tooltip.append_str("\n")).append(component::translatable("skyblocker.museum.hud.filter.switch").with_style_format(ChatFormatting::Yellow));`
- `src/de/hysky/skyblocker/skyblock/searchoverlay/overlay_screen.rs:212` (baseline :212) `stars.append((component::literal("✪")).with_style_format((if i < 5 { ChatFormatting::Yellow } else { ChatFormatting::Red })));`
- `src/de/hysky/skyblocker/skyblock/searchoverlay/overlay_screen.rs:215` (baseline :215) `stars.append(component::literal("✪"));`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:489` (baseline :489) `overflow_warning.append(component::translatable("skyblocker.config.hud.widgetsList.info.overflowWarning.columnTip")).append_str("\n");`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:491` (baseline :491) `overflow_warning.append(component::translatable("skyblocker.config.hud.widgetsList.info.overflowWarning.wrappingSpacingTip"));`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:492` (baseline :492) `text.append_str("\n");`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:493` (baseline :493) `text.append(overflow_warning.clone());`
- `src/de/hysky/skyblocker/skyblock/tabhud/util/player_list_manager.rs:300` (baseline :300) `out.append(__a0.clone());`
- `src/de/hysky/skyblocker/skyblock/tabhud/util/player_list_manager.rs:410` (baseline :410) `new_text.append(component::literal(&text_to_append).set_style(current.get_style().clone()));`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/jacobs_contest_widget.rs:194` (baseline :194) `crop_text.append((component::literal(&format!(" {}", sky_block_icons::FARMING_FORTUNE))).with_style_format(ChatFormatting::Gold));`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/minion_widget.rs:147` (baseline :147) `mt.append(component::literal(&stat).with_style_format(format));`
- `src/de/hysky/skyblocker/utils/constants.rs:?` (baseline :55) `component.append((component::literal(&compat::substring2(&string, i, i.wrapping_add(1)))).with_color(OkLabColor::interpolate(0x00FF4C, 0x14D0FF, (((i) as f32) / ((compat::str_len(&string)).wrapping_su`
- `src/de/hysky/skyblocker/utils/fun_utils.rs:?` (baseline :76) `component.append((component::literal(&c.to_string()).with_color(argb::srgb_lerp(progress, start_color, end_color))).with_style_fn(|style| style.with_obfuscated(Some(obfuscated))));`
- `src/de/hysky/skyblocker/utils/text_transformer.rs:?` (baseline :133) `new_text.append(component::literal(&builder.clone()).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(formatting.clone()).with_bold(Some(bold)).with_italic(Some(i`
- `src/de/hysky/skyblocker/utils/text_transformer.rs:?` (baseline :171) `new_text.append(component::literal(&builder.clone()).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(formatting.clone()).with_bold(Some(bold)).with_italic(Some(i`

## ptr-eq-fields (2)

`std::ptr::eq` between values held by two places: always false

- `src/de/hysky/skyblocker/skyblock/compact_damage.rs:89` (baseline :89) `if text_color.is_none() || std::ptr::eq(text_color.as_ref().unwrap(), &TextColor::from_legacy_format(ChatFormatting::Gray).unwrap()) {`
- `src/de/hysky/skyblocker/utils/render/texture/identifier_texture.rs:?` (baseline :78) `return std::ptr::eq(&self.current_texture, &self.fallback);`

## fill-additive (2)

`fill(RenderPipelines.GUI, ..)` resolved to `fill_additive` (additive blending): it is the plain `fill`

- `src/de/hysky/skyblocker/utils/render/gui/sound_selection_popup.rs:?` (baseline :457) `graphics.fill_additive(&mcrust::net::minecraft::client::renderer::render_pipelines::gui(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), 0x3000FF00);`
- `src/de/hysky/skyblocker/utils/render/gui/sound_selection_popup.rs:?` (baseline :459) `graphics.fill_additive(&mcrust::net::minecraft::client::renderer::render_pipelines::gui(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), 0x20FFFFFF);`

## overload (12)

a call resolved to the wrong overload (`of(String, Component...)` for `of(title, value)`)

- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/accessory_tooltip.rs:144` (baseline :144) `lines.push(GridComponent::of_2(&compat::jstring(&title), &vec![state_text.clone()]));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/bazaar_price_tooltip.rs:127` (baseline :127) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Bazaar Buy Price:").with_style_format(ChatFormatting::Gold)), &vec![(if product.buy_price().is_none() { component::literal("No data`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/bazaar_price_tooltip.rs:128` (baseline :128) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Bazaar Sell Price:").with_style_format(ChatFormatting::Gold)), &vec![(if product.sell_price().is_none() { component::literal("No da`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/craft_price_tooltip.rs:230` (baseline :218) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Crafting Price:").with_style_format(ChatFormatting::Gold)), &vec![ItemTooltip::get_coins_message(craft_price, count)]));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/dungeon_quality_tooltip.rs:147` (baseline :147) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Item Quality:").with_style_formats(&style)), &vec![(component::literal(&format!("{}/50", base_stat_boost_percentage))).with_style_f`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/dungeon_quality_tooltip.rs:150` (baseline :150) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Floor Tier:").with_style_formats(&style)), &vec![(component::literal(&format!("{} ({})", item_tier, DungeonQualityTooltip::get_item`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/essence_shop_price.rs:151` (baseline :151) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Essence Cost:").with_style_format(ChatFormatting::Aqua)), &vec![(((component::empty().append((component::literal(&format!("{} coins`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/george_price_tooltip.rs:135` (baseline :135) `lines.push(GridComponent::of_2(&compat::jstring(&(component::literal(&format!("{:<21}", "Pet Sell Price:"))).with_style_format(ChatFormatting::Yellow)), &vec![ItemTooltip::get_coins_message(price, 1)]`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/hunting_box_price_tooltip.rs:135` (baseline :135) `lines.push(GridComponent::of_2(&compat::jstring(&(component::literal(&format!("{} Sell Price:", shard_text))).with_style_format(ChatFormatting::Gold)), &vec![(if product.sell_price().is_none() { compo`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/museum_tooltip.rs:129` (baseline :129) `lines.push(GridComponent::of_2(&compat::jstring(&(component::literal(&format!("Museum ({}):", item_category))).with_style_format(ChatFormatting::LightPurple)), &vec![(component::literal((if is_in_muse`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/npc_price_tooltip.rs:132` (baseline :132) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("NPC Sell Price:").with_style_format(ChatFormatting::Yellow)), &vec![ItemTooltip::get_coins_message(price, count)]));`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/obtained_date_tooltip.rs:165` (baseline :162) `lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Obtained:").with_style_format(ChatFormatting::LightPurple)), &vec![component::literal(&timestamp).with_style_format(ChatFormatting:`

## arrays-tostring (8)

`Arrays.toString` as `{:?}` (quotes strings, Rust syntax): Java prints `[a, b]`

- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_map.rs:?` (baseline :181) `logger().error(&format!("[Skyblocker Dungeon Map] Dungeon player for map decoration '{}' {}. Player list index (zero-indexed): {}. Player list: {}. Map decorations: {}", decoration_id, reason, i, form`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/dungeon_map_utils.rs:300` (baseline :295) `logger().debug(&format!("[Skyblocker] Found dungeon room segments: {}", format!("{:?}", segments.iter().cloned().collect::<Vec<_>>())));`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/dungeon_player_manager.rs:?` (baseline :115) `logger().error(&format!("[Skyblocker Dungeon Player Manager] Received ghost message for player '{}' but player was not found in the player list: {}", compat::opt_str(&matcher.group_named("name")), for`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:?` (baseline :507) `return format!("Room{{type={}, segments={}, shape={}, matchState={}, name={}, direction={}, physicalCornerPos={}}}", compat::read(__this, |__t| __t.room_base().r#type), format!("{:?}", __this.lock().b`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:?` (baseline :540) `panic!("IllegalArgumentException: {}", format!("Shape {} does not match segments: {}", __this.lock().borrow().room_base().shape.as_ref().unwrap().fields().shape, format!("{:?}", __this.lock().borrow()`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/room.rs:?` (baseline :553) `panic!("IllegalArgumentException: {}", format!("Shape {} does not match segments: {}", __this.lock().borrow().room_base().shape.as_ref().unwrap().fields().shape, format!("{:?}", __this.lock().borrow()`
- `src/de/hysky/skyblocker/skyblock/shortcut/shortcuts_config_list_widget.rs:1201` (baseline :1201) `return format!("{:?}", ShortcutKeyBinding::get_bound_keys_translation_key(self.key_binding.as_ref().unwrap()));`
- `src/de/hysky/skyblocker/utils/datafixer/item_stack_componentization_fixer.rs:?` (baseline :68) `return Some(format!("{:?}", components.entry_set().iter().cloned().map(|entry| {`

## never-null (157)

a null test emitted as a constant (`true`/`false`): check that the value can really never be null

- `src/de/hysky/skyblocker/compatibility/rei/recipe/skyblock_recipe_category.rs:95` (baseline :95) `if true /* java: arrow_location != null: never null here */ {`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:445` (baseline :445) `if false /* java: backup == null: never null here */ {`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:451` (baseline :451) `if compat::json::is_object(&backup) && true /* java: current != null: never null here */ && compat::json::is_object(&current) {`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:456` (baseline :456) `} else if compat::json::is_array(&backup) && true /* java: current != null: never null here */ && compat::json::is_array(&current) {`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:471` (baseline :471) `let mut new_path: String = (if false /* java: key == null: never null here */ { path.to_string() } else { (if path.is_empty() { key.to_string() } else { format!("{}.{}", path, key) }) });`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:473` (baseline :473) `let mut line: String = (if false /* java: key == null: never null here */ { format!("{}{{", ind) } else { format!("{}\"{}\": {{", ind, key) });`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:485` (baseline :485) `let mut line: String = (if false /* java: key == null: never null here */ { format!("{}[", ind) } else { format!("{}\"{}\": [", ind, key) });`
- `src/de/hysky/skyblocker/config/backup/config_backup_screen.rs:494` (baseline :494) `let mut line: String = (if false /* java: key == null: never null here */ { format!("{}{}", ind, value) } else { format!("{}\"{}\": {}", ind, key, value) });`
- `src/de/hysky/skyblocker/skyblock/calculators/calculator_command.rs:68` (baseline :68) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/chat/chat_rule_config_screen.rs:492` (baseline :492) `if true /* java: formatting != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/chest_value.rs:189` (baseline :189) `if false /* java: matcher == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/chest_value.rs:308` (baseline :308) `if screen_type == ScreenType::Minion && slot.index == 28 && stack.is_item(&items::items().get("hopper")) && true /* java: { coins_line = ItemUtils::get_lore_line_if(&stack, Arc::new(move |s| s.contain`
- `src/de/hysky/skyblocker/skyblock/chocolatefactory/egg_finder.rs:220` (baseline :220) `if true /* java: egg_type.fields().egg != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/discipline_test_helper.rs:30` (baseline :30) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/discipline_test_helper.rs:39` (baseline :39) `if compat::get_state(&dojo_manager::CURRENT_CHALLENGE) != DojoChallenges::Discipline || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_|`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/dojo_manager.rs:197` (baseline :197) `if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/mastery_test_helper.rs:45` (baseline :45) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/stamina_test_helper.rs:75` (baseline :75) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/stamina_test_helper.rs:75` (baseline :161) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/stamina_test_helper.rs:231` (baseline :231) `if WALL_HOLES.get_mut().is_empty() || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/dojo/tenacity_test_helper.rs:58` (baseline :58) `if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::level().is_none() {`
- `src/de/hysky/skyblocker/skyblock/crimson/kuudra/kuudra.rs:97` (baseline :97) `if true /* java: matcher != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/dungeon_map.rs:?` (baseline :244) `let mut deg: f32 = (if true /* java: player_entity != null: never null here */ { player_entity.get_y_rot() } else { (((map_decoration.rot() as i32).wrapping_mul(360) as f32) / 16.0f32) });`
- `src/de/hysky/skyblocker/skyblock/dungeon/goldor_waypoints_manager.rs:?` (baseline :193) `if true /* java: { player_name = GoldorWaypointsManager::get_player_name(&TERMINAL_ACTIVATED.matcher(&message)).unwrap(); player_name.clone() } != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/goldor_waypoints_manager.rs:?` (baseline :195) `} else if true /* java: { player_name = GoldorWaypointsManager::get_player_name(&DEVICE_ACTIVATED.matcher(&message)).unwrap(); player_name.clone() } != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/goldor_waypoints_manager.rs:?` (baseline :197) `} else if true /* java: { player_name = GoldorWaypointsManager::get_player_name(&LEVER_ACTIVATED.matcher(&message)).unwrap(); player_name.clone() } != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/guardian_health.rs:71` (baseline :71) `if false /* java: compat::component_of(armor_stand.get_display_name()) == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/guardian_health.rs:111` (baseline :111) `if false /* java: compat::component_of(entity.get_display_name()) == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/option_dropdown_widget.rs:?` (baseline :419) `if false /* java: o == null: never null here */ || self.get_class() != compat::type_name_of(&o) {`
- `src/de/hysky/skyblocker/skyblock/dungeon/partyfinder/party_finder_screen.rs:616` (baseline :619) `debug_assert!(true /* java: compat::minecraft().game_mode != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/dungeon/preview/preview_room.rs:95` (baseline :95) `if false /* java: server == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/preview/room_preview.rs:47` (baseline :47) `if compat::minecraft().has_singleplayer_server() && true /* java: compat::minecraft().get_singleplayer_server() != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/preview/room_preview_server.rs:153` (baseline :148) `if false /* java: server == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/preview/room_preview_server.rs:153` (baseline :185) `if false /* java: server == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/dungeon_manager.rs:?` (baseline :997) `if true /* java: on_checkmark_found != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/dungeon_map_utils.rs:81` (baseline :81) `while true /* java: { map_pos = pos_to_check.pop_front().unwrap(); map_pos.clone() } != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/secrets/secret_waypoint.rs:110` (baseline :111) `if false /* java: color == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/terminal/same_color_terminal.rs:?` (baseline :90) `if true /* java: idx != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/dungeon/terminal/starts_with_terminal.rs:177` (baseline :177) `if clicked || false /* java: stack == null: never null here */ || stack.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/dwarven/block_break_prediction.rs:84` (baseline :84) `if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || false /* java: compat::minecraft().game_mode == null: never null here */ || !(SkyblockerConfigManager::get().mining.block_break_predi`
- `src/de/hysky/skyblocker/skyblock/dwarven/fossil/structures.rs:232` (baseline :232) `if true /* java: percentage != null: never null here */ && !(percentage == (self.r#type).fields().percentage) {`
- `src/de/hysky/skyblocker/skyblock/dwarven/wishing_compass_solver.rs:129` (baseline :129) `return (mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items().iter().cloned().a`
- `src/de/hysky/skyblocker/skyblock/events/event_notifications.rs:141` (baseline :141) `if false /* java: response == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/events/event_notifications.rs:177` (baseline :177) `if false /* java: skyblock_event == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/events/event_notifications.rs:177` (baseline :185) `if false /* java: skyblock_event == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/fancybars/fancy_status_bars.rs:203` (baseline :203) `if true /* java: object != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/foraging/galatea/tuner_solver.rs:350` (baseline :350) `if true /* java: stack != null: never null here */ && TunerSolver::is_stained_glass_pane(&stack.get_item()) {`
- `src/de/hysky/skyblocker/skyblock/garden/farming_hud.rs:86` (baseline :86) `if false /* java: old_state == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/garden/garden_plots.rs:138` (baseline :138) `if false /* java: garden_plot == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/garden/garden_plots_widget.rs:147` (baseline :147) `if false /* java: garden_plot == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/garden/greenhouse_paste.rs:308` (baseline :308) `if false /* java: layout == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/garden/greenhouse_paste.rs:576` (baseline :576) `if false /* java: model == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/garden/stereo_harmony_helper.rs:140` (baseline :140) `if true /* java: matcher != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/background/adders/jacob_medal_background.rs:69` (baseline :69) `if false /* java: stack == null: never null here */ || stack.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/item/background/adders/jacob_medal_background.rs:?` (baseline :81) `if true /* java: color != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/background/adders/legacy_attribute_background.rs:66` (baseline :66) `if false /* java: stack == null: never null here */ || stack.is_empty() || stack.get_skyblock_id() == "ATTRIBUTE_SHARD" {`
- `src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_animated_dyes.rs:78` (baseline :76) `if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ && !held_item.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_dye_colors.rs:51` (baseline :51) `if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_trims.rs:110` (baseline :112) `if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_trims.rs:115` (baseline :117) `if false /* java: material == null: never null here */ && false /* java: pattern == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/custom/screen/animated_dye_timeline_widget.rs:83` (baseline :83) `debug_assert!(true /* java: __this.lock().borrow().gradient_texture.as_ref().unwrap().get_pixels() != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/item/custom/skyblock_item_models.rs:67` (baseline :67) `while true /* java: next_model != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/player_head_hash_cache.rs:35` (baseline :35) `for __a0 in api_item_textures.chain(override_textures).map(|__a0| PlayerHeadHashCache::get_skin_hash_from_base64(&__a0)).filter(|hash| (true /* java: hash != null: never null here */ && !hash.is_empty`
- `src/de/hysky/skyblocker/skyblock/item/player_head_hash_cache.rs:67` (baseline :67) `if true /* java: url != null: never null here */ && url == "ETF pre test, skin check" {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/bits_helper.rs:223` (baseline :221) `if false /* java: slots == null: never null here */ || slots.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/color_tooltip.rs:63` (baseline :63) `if true /* java: expected_hex != null: never null here */ && !(compat::equals_ignore_case(&color_hex, expected_hex)) && !(ColorTooltip::is_exception(internal_id, color_hex)) && !(ColorTooltip::intende`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/adders/color_tooltip.rs:74` (baseline :74) `if true /* java: color != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/tooltip/backpack_preview.rs:?` (baseline :200) `if true /* java: size != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/item/wikilookup/wiki_lookup_manager.rs:51` (baseline :51) `if false /* java: independent == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_crafting_tab.rs:110` (baseline :110) `debug_assert!(true /* java: compat::read(&self.recipe_book, |__t| __t.search_box.clone()) != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_crafting_tab.rs:152` (baseline :152) `if true /* java: compat::read(&self.recipe_book, |__t| __t.search_box.clone()) != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_recipe_book_component.rs:100` (baseline :100) `if false /* java: compat::read(__this, |__t| __t.search_box.clone()) == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_recipe_book_component.rs:272` (baseline :272) `debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_recipe_book_component.rs:272` (baseline :284) `debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_recipe_book_component.rs:272` (baseline :303) `debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/skyblock_recipe_tab_button.rs:45` (baseline :45) `if true /* java: compat::read(__this, |__t| __t.sprites.clone()) != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/itemlist/recipebook/upcoming_events_tab.rs:242` (baseline :242) `if false /* java: event == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/itemlist/stack_overlays.rs:44` (baseline :44) `if true /* java: overlay_file != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/loadout_keybinds.rs:126` (baseline :126) `if false /* java: minecraft.game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/museum/museum_item_cache.rs:302` (baseline :302) `if false /* java: parent_item == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/museum/museum_manager.rs:220` (baseline :220) `if true /* java: Donation::get_set(item) != null: never null here */ && !(Donation::get_set(item).is_empty()) {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/collections/collections_page.rs:149` (baseline :149) `if true /* java: page != null: never null here */ && ({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).is_some() {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/collections/generic_category.rs:80` (baseline :80) `let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = (if false /* java: flexible == null: never null here */ { FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut())`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/inventory_page.rs:173` (baseline :173) `if true /* java: page != null: never null here */ && ({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).is_some() {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/pet.rs:145` (baseline :145) `if false /* java: item == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/pet.rs:151` (baseline :151) `if false /* java: pet_stack == null: never null here */ || FlexibleItemStack::is_empty(&pet_stack) {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/inventory/pet.rs:224` (baseline :224) `if true /* java: held_item != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/profile_viewer_screen.rs:341` (baseline :341) `if true /* java: profile_viewer_page != null: never null here */ && ({ let __f = compat::dispatch(&profile_viewer_page, |__t| __t.get_buttons()); __f(&profile_viewer_page) }).is_some() {`
- `src/de/hysky/skyblocker/skyblock/profileviewer/profile_viewer_screen.rs:343` (baseline :343) `if true /* java: button != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/model/generic_catacombs.rs:356` (baseline :356) `return (if true /* java: d != null: never null here */ { d } else { ((0) as f64) as f64 });`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/accessory_power_calculator.rs:142` (baseline :142) `return (if true /* java: number != null: never null here */ { number } else { ((0) as f64) as f64 });`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/level_calculator.rs:37` (baseline :37) `let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/level_calculator.rs:37` (baseline :102) `let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/level_calculator.rs:37` (baseline :150) `let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/pet_loader.rs:232` (baseline :232) `if held_item.is_some() && true /* java: FlexibleItemStack::get(held_item.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::custom_name()) != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/profileviewer2/utils/pet_loader.rs:258` (baseline :258) `if pet_skin.is_some() && true /* java: FlexibleItemStack::get(pet_skin.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::profile()) != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/quicknav/quick_nav_button.rs:96` (baseline :96) `if false /* java: tooltip == null: never null here */ || tooltip.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/radial_menu/radial_menu_screen.rs:278` (baseline :278) `if false /* java: compat::minecraft().game_mode == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/searchoverlay/overlay_screen.rs:285` (baseline :285) `if false /* java: flexible == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/shortcut/shortcuts.rs:311` (baseline :311) `if false /* java: command == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/shortcut/shortcuts.rs:319` (baseline :319) `if false /* java: command == null: never null here */ || (compat::get_state(&LAST_KEY_BINDING_COMMAND_TIME)).wrapping_add(KEY_BINDING_COOLDOWN) > compat::millis() {`
- `src/de/hysky/skyblocker/skyblock/slayers/boss/voidgloom/beacon_highlighter.rs:109` (baseline :109) `if true /* java: entity != null: never null here */ && !(beacon_entity_path.1[beacon_entity_path.1.len() - 1] == entity.position()) {`
- `src/de/hysky/skyblocker/skyblock/slayers/features/slayer_timer.rs:149` (baseline :149) `return (if true /* java: current_best != null: never null here */ { current_best.best_time_millis() } else { ((-1) as i64) as i64 });`
- `src/de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs:229` (baseline :229) `if true /* java: boss != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/storageoverlay/storage_overlay_screen.rs:400` (baseline :400) `if compat::minecraft().options.borrow().key_inventory.matches(event) && true /* java: focus_path != null: never null here */ && focus_path.leaf_component().as_any().is::<EditBox>() {`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:261` (baseline :261) `if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/tabhud/config/list/widgets_list_screen.rs:261` (baseline :275) `if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/element_based_widget.rs:85` (baseline :85) `let __init_color = (0xFF000000u32 as i32 | (if false /* java: color_value == null: never null here */ { 0 } else { SkyBlockColors::from_vanilla(color_value) }));`
- `src/de/hysky/skyblocker/skyblock/tabhud/widget/element_based_widget.rs:184` (baseline :184) `if false /* java: compat::panic_message(&e) == null: never null here */ || !(Some(&*(compat::panic_message(&e))) == compat::read(__this, |__t| __t.element_based_widget_base().last_error.clone()).as_de`
- `src/de/hysky/skyblocker/skyblock/teleport/predictive_smooth_aote.rs:112` (baseline :112) `return (if true /* java: custom_data != null: never null here */ && custom_data.contains("tuned_transmission") { base_range.wrapping_add(custom_data.get_int_or("tuned_transmission", 0)) } else { base_`
- `src/de/hysky/skyblocker/skyblock/teleport/predictive_smooth_aote.rs:272` (baseline :272) `if false /* java: start_pos == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/teleport/predictive_smooth_aote.rs:289` (baseline :289) `if false /* java: entity_hit == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/teleport/predictive_smooth_aote.rs:334` (baseline :334) `if mcrust::mcrust::modding::game::level().is_none() || false /* java: direction == null: never null here */ || false /* java: start_pos == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/teleport/teleport_overlay.rs:95` (baseline :95) `TeleportOverlay::extract_rendering_3(collector, (if true /* java: custom_data != null: never null here */ && custom_data.contains("tuned_transmission") { base_range.wrapping_add(custom_data.get_int_or`
- `src/de/hysky/skyblocker/skyblock/wardrobe_keybinds.rs:77` (baseline :77) `if false /* java: minecraft.game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/skyblock/waypoint/abstract_waypoints_screen.rs:137` (baseline :137) `if false /* java: w == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/waypoint/individual_waypoint.rs:81` (baseline :81) `if true /* java: area != null: never null here */ && !area.is_empty() {`
- `src/de/hysky/skyblocker/skyblock/waypoint/individual_waypoint.rs:92` (baseline :92) `let mut waypoint_name: String = (if true /* java: area != null: never null here */ && !area.is_empty() { area.to_string() } else { "Chat Waypoint".to_string() });`
- `src/de/hysky/skyblocker/skyblock/waypoint/mythological_ritual.rs:214` (baseline :214) `if false /* java: intersection == null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/waypoint/waypoints.rs:390` (baseline :390) `if true /* java: group != null: never null here */ {`
- `src/de/hysky/skyblocker/skyblock/waypoint/waypoints.rs:390` (baseline :398) `if true /* java: group != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/json_helper.rs:20` (baseline :20) `if false /* java: root == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/json_helper.rs:20` (baseline :43) `if false /* java: root == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/json_helper.rs:20` (baseline :66) `if false /* java: root == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_item_stack_fixer.rs:50` (baseline :50) `if true /* java: name != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_item_stack_fixer.rs:57` (baseline :57) `if true /* java: lore != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/datafixer/legacy_item_stack_fixer.rs:71` (baseline :71) `if true /* java: custom_data != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/item_utils.rs:269` (baseline :259) `if false /* java: stack == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/item_utils.rs:?` (baseline :659) `if true /* java: tooltip_style != null: never null here */ && tooltip_style.namespace() == utils::HYPIXEL_SKYBLOCK_NAMESPACE {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:44` (baseline :44) `if false /* java: input == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:71` (baseline :71) `if false /* java: input_str == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:44` (baseline :84) `if false /* java: input == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:94` (baseline :94) `if false /* java: compressed_str == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:44` (baseline :107) `if false /* java: input == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:71` (baseline :117) `if false /* java: input_str == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:138` (baseline :138) `if false /* java: uncompressed_str == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/lz_string.rs:405` (baseline :405) `if false /* java: compressed == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/regex_utils.rs:?` (baseline :93) `if false /* java: s == null: never null here */ {`
- `src/de/hysky/skyblocker/utils/registry_utils.rs:?` (baseline :28) `return (if client.is_some() && minecraft::get_connection(client.as_ref().unwrap()).is_some() && true /* java: minecraft::get_connection(client.as_ref().unwrap()).unwrap().registry_access() != null: ne`
- `src/de/hysky/skyblocker/utils/render/gui/abstract_custom_hypixel_gui.rs:62` (baseline :59) `if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {`
- `src/de/hysky/skyblocker/utils/render/primitive/primitive_feature_renderer.rs:145` (baseline :139) `if true /* java: info != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/render/state/gui/gui_outlined_text_render_state.rs:104` (baseline :103) `if true /* java: screen_rect != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/render/texture_pool.rs:?` (baseline :130) `if true /* java: texture != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/render/texture_pool.rs:?` (baseline :135) `if true /* java: texture_view != null: never null here */ {`
- `src/de/hysky/skyblocker/utils/text_transformer.rs:?` (baseline :61) `if false /* java: code == null: never null here */ {`
- `src/mixins/abstract_container_screen_mixin.rs:29` (baseline :29) `if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && true /* java: hovered_slot != null: never null here */ && !input.is_escape() && !(compat::minecraft().options.borrow().key_inventory.`
- `src/mixins/abstract_container_screen_mixin.rs:267` (baseline :267) `if true /* java: current_solver != null: never null here */ {`
- `src/mixins/abstract_sign_edit_screen_mixin.rs:49` (baseline :49) `if true /* java: button != null: never null here */ {`
- `src/mixins/camera_mixin.rs:16` (baseline :16) `if true /* java: pos != null: never null here */ {`
- `src/mixins/camera_mixin.rs:16` (baseline :22) `if true /* java: pos != null: never null here */ {`
- `src/mixins/entity_mixin.rs:79` (baseline :79) `if true /* java: slayer_quest != null: never null here */ && compat::read(&slayer_quest, |__t| __t.boss_spawned) && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_f`
- `src/mixins/entity_renderer_mixin.rs:43` (baseline :43) `if true /* java: pos != null: never null here */ {`
- `src/mixins/entity_renderer_mixin.rs:60` (baseline :60) `ci.set_return_value((if true /* java: custom_name != null: never null here */ { custom_name.clone() } else { original.clone() }));`
- `src/mixins/hud_mixin.rs:26` (baseline :26) `if false /* java: component == null: never null here */ {`
- `src/mixins/item_stack_mixin.rs:42` (baseline :42) `return (!(true /* java: compat::minecraft() != null: never null here */ && true /* java: compat::minecraft().gui != null: never null here */ && mcrust::mcrust::modding::game::current_screen_is::<compa`
- `src/mixins/mouse_handler_mixin.rs:44` (baseline :44) `if true /* java: position != null: never null here */ {`
- `src/mixins/options_mixin.rs:31` (baseline :31) `if true /* java: element != null: never null here */ {`
