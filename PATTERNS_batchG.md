# PATTERNS - batch G

Converter (fabric2seer) mistakes fixed three or more times in batch G. Paths relative to `src/de/hysky/skyblocker/`.

## Silent behaviour changes

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| S1 | parentheses dropped around `a \|\| b` inside `&&` | restore them | 8 | `skyblock/entity/glow/adder/slayer_glow_adder.rs` attunement check, `skyblock/garden/visitor/visitor_helper.rs` `shouldRender` / `onSlotClick`, `skyblock/crimson/kuudra/kuudra_waypoints.rs` x2, `skyblock/slayers/boss/{sven/mute_wolf_sounds, tarantula/mute_spider_sounds}.rs`, `garden_plots_widget.rs` `isValidClickButton` |
| S2 | `String.format("%.Nf", v)` as `format!("{:.N}")` (half-even, not HALF_UP) | `compat::string_format` | 5 | `slayers/boss/voidgloom/lazer_timer.rs`, `slayers/features/slayer_timer.rs`, `garden/farming_hud_widget.rs` x2, `events/greatspook/math_teacher_helper.rs` |
| S3 | `Integer.parseInt(s)` emitted with `.trim()` | no trim | 8 | `slayers/slayer_manager.rs` x3, `chest_value.rs`, `compact_damage.rs`, `crimson/kuudra/kuudra.rs`, `barn/treasure_hunter.rs`, `slayers/boss/demonlord/fire_pillar_announcer.rs` |
| S4 | `MutableComponent.append(..)` / `withStyle(..)` statement whose result is dropped | assign back | 6 | `health_bars.rs` x2, `compact_damage.rs` x2, `garden/visitor/visitor_helper.rs`, `barn/call_trevor.rs` |
| S5 | a value a Java `computeIfAbsent` / record accessor answers by reference, mutated on the port's COPY (`data.pages().insert(..)`) | mutate the fields, then `ProfiledData::put` back | 3 | `accessories/accessories_helper.rs` `collectAccessories`, `slayers/slayer_manager.rs` `SlayerQuest.save` |
| S6 | `@Nullable T[]` as `Vec<T>` (+ "never null here") | `Vec<Option<T>>` | 3 | `garden/garden_plots.rs` `GARDEN_PLOTS`, `garden/garden_plots_widget.rs` `items` / `CUSTOM_ICON_OPTIONS` |
| S7 | `instanceof X` over a class with no Rust type emitted as `as_any().is::<Item>()` / `is::<Block>()` (always true) | the registered object's identity (`ptr_eq` with `items().get("bow")`) | 2 | `crimson/kuudra/arrow_poison_warning.rs`, `garden/greenhouse_paste.rs` |

## Compile errors

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | a game entity kept by value (`ArmorStand`, `Box<dyn Entity>`, `HashMap<Zombie, ..>`, `HashSet<Box<dyn Entity>>`) | entity id + `game::with_entity` / `get_entities` + `lock_entity`; an entity the caller holds is passed down as `&dyn Entity` and is the sweep's `except` | ~60 sites in 14 files | `slayers/slayer_manager.rs` (`BossFight`, `SlayerQuest`, `ENTITIES_CACHE`), `entity/mob_glow.rs` `CACHE`, `health_bars.rs`, `crimson/dojo/{force,tenacity}_test_helper.rs`, `crimson/kuudra/kuudra_waypoints.rs` |
| 2 | `screen.as_any().downcast_ref::<ContainerScreen>()` in an AFTER_INIT listener | `matches!(minecraft().gui.screen_kind, ScreenKind::Container)` (+ `SignEdit` for `AbstractSignEditScreen`), title from `screen.get_title()` | 8 | `chest_value.rs`, `pet_cache.rs`, `kuudra.rs`, `accessories_helper.rs`, `visitor_helper.rs`, `rei/*` x2 |
| 3 | `slot.getItem()` / `hasItem()` with no `Containers` | `compat::slot_item` / `slot_has_item` / `with_open_menu` | 7 | `chest_value.rs`, `inventory_search.rs`, `visitor_helper.rs`, `accessories_helper.rs`, `new_year_cake_bag_helper.rs` |
| 4 | `Codec.unboundedMap` typed `HashMap` | `codec::xmap(unbounded_map(..), collect, to_vec)` | 4 | `accessories_helper.rs` x2, `kuudra_profile_data.rs`, `slayer_manager.rs` |
| 5 | async loaders touching render-thread statics / `&mut Minecraft` inside `runAsync` | IO on `compat::executor`, objects built through `game::execute`; the `'static` resource manager is captured | 6 | `rift/enigma_souls.rs`, `rift/mirrorverse_waypoints.rs`, `crimson/kuudra/kuudra_waypoints.rs`, `garden/garden_plots.rs`, `events/event_notifications.rs` |
| 6 | a record's canonical constructor and a convenience one both `new` | canonical `new_N` | 4 | `garden/garden_plots.rs`, `garden/greenhouse_crops.rs`, `garden/visitor/visitor.rs`, `slayers/slayer_tier.rs` (`value_of_2`) |
| 7 | `fn x_base(&self) -> &XBase { &self }` | `self` | 3 | `events/event_toast.rs`, `sky_block_equipment_updater.rs` |
| 8 | `Display` calling `T::to_string(*self)` | `T::to_string(self)` | 2 | `gyro_overlay.rs`, `accessories_helper_widget.rs` |
| 9 | `update(Arc::new(move \|config\| config.x = ..))` | `move \|mut config\|` (now `&mut SkyblockerConfig`) | 7 | `tips.rs` x2, `night_vision_command.rs`, `mc_grubber_updater.rs`, `status_bar_tracker.rs`, `garden_plots_widget.rs` (values read first) |
| 10 | `set_parent(self.clone())` in a HUD widget | `self.hud_widget_base().as_parent()` | 3 | `slayer_hud_widget.rs`, `farming_hud_widget.rs`, `item_pickup_widget.rs` |
| 11 | `match s.as_str()` on a `&str` (unstable `str_as_str`) | `match s` | 4 | `special/*_special_effects.rs` x3, `slayers/slayer_type.rs` |
| 12 | `Font.split` result stored as `Vec<FormattedCharSequence>` | `language::get_instance().get_visual_order_list(&font.split(..))`, or keep `FormattedText` where the port's draw takes it | 2 | `accessories_helper_widget.rs`, `events/event_toast.rs` |
