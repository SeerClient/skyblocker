# PATTERNS - batch H

Converter (fabric2seer) mistakes fixed three or more times in batch H. Paths are relative to
`src/de/hysky/skyblocker/`. The first table changes BEHAVIOUR silently.

## Silent behaviour changes

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| S1 | **`Blocks.X` looked up by its registry PATH**: `blocks::blocks().get("snow")`; the port's `BlocksTable::get` is keyed by the Java CONSTANT name (`"SNOW"`) - every such lookup panics (`items().get` IS keyed by path) | `blocks().get("SNOW")` | 9 here; **69 sites mod-wide** (`grep 'blocks().get("[a-z]'`) | `skyblock/teleport/predictive_smooth_aote.rs:140`, `skyblock/end/the_end.rs:151`, `debug/snapshot_debug.rs:62` |
| S2 | a Java method that mutates a list ARGUMENT in place (`List.sort`, a `Consumer<List<..>>` sort function, `applyFilter(items, filtered)`) emitted with the list passed as `compat::read(.., \|t\| t.list.clone())` or a `Vec` by value | `&mut Vec` parameter, called inside `compat::write(h, \|t\| f(&mut t.list))` | 6 | `skyblock/museum/item_sorter.rs` SORT_BY_* + `apply_sort`, `museum_manager.rs` sort/filter buttons |
| S3 | `BarPositioner`-style helpers mutating `self.map.get(&k).cloned().unwrap()` (a COPY of the row list) | `self.map.get_mut(&k)` / `&mut self` | 8 methods | `skyblock/fancybars/bar_positioner.rs` |
| S4 | fastutil pair setters (`pair.left(v)`, `resizedBars.right(null)`, `IntIntPair.left(x + 1)`) emitted as a READ statement `compat::read(..).0;` | `compat::write(h, \|t\| t.pair.0 = v)` | 22 | `fancybars/status_bars_config_screen.rs` (mouseClicked / mouseReleased / resize hover), `end/ender_nodes.rs`, `museum/price_data.rs` |
| S5 | a `@Nullable Integer` / `@Nullable T` parameter typed as the bare value, the call site writing `(if c { Some(x) } else { None }).unwrap()` (panics on the null case) | `Option<i32>` parameter | 4 | `fancybars/status_bar.rs` `updateValues` (every vitality/defence/experience update) |
| S6 | `Component.append(..)` / `MutableComponent.append(..)` result dropped (Java appends in place) | `x = x.append(..)` | 6 | `fancybars/status_bar.rs:528-529`, `museum/item_filter.rs:69,72`, `chocolatefactory/chocolate_factory_solver.rs:698` |
| S7 | `Integer.parseInt` / `Long.parseLong` emitted with `.trim()` | no trim | 5 | `auction/auction_browser_screen.rs:356-357`, `speedpreset/speed_preset_list_widget.rs:457`, `chocolate_factory_solver.rs:239`, `chocolatefactory/time_tower_reminder.rs` |
| S8 | `(a \|\| b) && c` emitted without the parentheses | restore them | 3 | `fancybars/fancy_status_bars.rs` `isEnabled`, `teleport/predictive_smooth_aote.rs:371`, `utils/render/gui/color_picker_widget.rs:128` |
| S9 | `x != null` on a nullable field written `if true /* never null */` | `if let Some(..)` | 3 | `chocolatefactory/egg_finder.rs` (egg already known), `fancybars/fancy_status_bars.rs` (loaded config), `predictive_smooth_aote.rs` (entity hit) |
| S10 | `blitSprite(.., int argb)` emitted with the colour cast `as f32` (a numeric conversion of an ARGB int) | pass the `i32` | 3 | `fancybars/status_bar.rs` extractBar |
| S11 | a method reference to an OVERLOADED method resolved to the wrong arity (`this::clickSlot` for a 2-arg functional interface → `click_slot_1`) | the overload matching the interface | 4 | `auction/auction_browser_screen.rs` slider / rarity / category widgets |
| S12 | `forEach` lambda `return` emitted as a method `return` | `continue` | 1 | `museum/museum_item_cache.rs` (category "special") |

## Compile errors

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | a Java subclass of a DFU class (`extends DataFix`) emitted as a `Shared` object with a base struct, the `DataFix` trait never implemented | the class implements the port's trait (`DataFix` with `data_fix()` -> fields, `make_rule(&self)`); the rule's lambdas capture a `Weak` from `Arc::new_cyclic` | 12 classes | `config/datafixer/config_fix*.rs` |
| 2 | generic `<T: 'static>` kept on methods where Java's `<T>` is only `Dynamic<T>`'s parameter (the port's `Dynamic` is untyped) - every call site E0282 | drop the type parameter | ~40 | `config/datafixer/*` |
| 3 | `Map.Entry.mapSecond` / `Optional.renameField` / `JsonObject.asMap` / `Matcher.hasMatch` left as todos although the port has the shape (`(k, f(v))` on the `update_map_values` tuple, `Dynamic::rename_field`, `get_as_json_object(..).into_iter().collect()`, `Matcher::has_match` + `end(0)`) | the one-line equivalent | 14 | `config_fix1.rs`, `config_fix2_quick_nav.rs`, `museum_item_cache.rs`, `chocolate_factory_solver.rs` |
| 4 | an enum's per-constant data in a `LazyLock<Vec<Fields>>` while the fields hold non-`Sync` values (`Shared`, `Arc<dyn Fn>`, `ItemStack`) or are MUTATED (`EggType.collected/egg`) | `compat::MainThread<Vec<Fields>>` (`get_mut` for mutable constants); or add `Send + Sync` to the closure aliases | 4 | `egg_finder.rs` `EggType::fields`, `museum/item_sorter.rs` `SortMode::fields`, `fancybars/bar_positioner.rs` (closure types) |
| 5 | constructor data built as `Self { f: Default::default(), .. }` then assigned, for types with no `Default` (`Identifier`, `BarAnchor`, `MutableComponent`, `ItemStack`, `Arc<dyn Fn>`) | build the struct directly | 6 | `fancybars/status_bar_type.rs`, `auction/widgets/{sort,auction_type}_widget.rs`, `museum/item_sorter.rs` |
| 6 | `&mut self` / `&self` in a root class's own `_base_mut()` body | `self` | 4 | `fancybars/status_bar.rs:805`, `auction/widgets/slider_widget.rs`, `speedpreset/speed_preset_list_widget.rs:230` |
| 7 | a `@FunctionalInterface` emitted as a trait with a dispatch slot (`SlotClickHandler`, `OptionInfo` on enum constants) while callers pass closures / enum values | closure type alias (`Arc<dyn Fn(i32, i32)>`) / value methods on the enum | 2 interfaces, 12 sites | `auction/slot_click_handler.rs`, `auction/widgets/slider_widget.rs` |
| 8 | a `LinearLayout` method chain on a borrowed field (`t.layout.spacing(8).default_cell_setting()..`; `spacing(self)` consumes) and `center_in_rectangle(&LayoutElement::Grid(copy))` on a COPY (positions never applied) | `mem::replace` + reassign; `LayoutBack::grid(&mut layout.wrapped)` then apply the widgets' positions | 5 | `fancybars/edit_bar_color_popup.rs`, `auction/edit_bid_popup.rs`, `auction/auction_view_screen.rs` |
| 9 | `layout.addChild(child)` used as a value (Java answers the child) with the port's `add_child` returning `()` | fill the child layout first, then add it | 1 (+ batch-wide shape) | `fancybars/edit_bar_color_popup.rs` colorLayout |
| 10 | `Display` impls calling `T::to_string(*self)` where `to_string(&self)` | `T::to_string(self)` | 4 | `fancybars/status_bar.rs` (3), `egg_finder.rs` |
| 11 | enum `getSerializedName()` / `toString()` left as todos "call to `name(..)` (not a method of this class)" | `self.name()` / `i18n::get(..)` | 6 | `fancybars/status_bar.rs` IconPosition / TextPosition / Direction |
| 12 | `Color.getRGB()` / `new Color(int)` left as todos | `Color::get_rgb()` / `Color::from_rgb(int)` | 15 | `fancybars/status_bar.rs` |
| 13 | `KeyMapping.consumeClick()` / `isDown()` / `options.keyShift` emitted with no key | `key_mapping::consume_click("key.x")` / `is_down("key.sneak")` | 5 | `debug/debug.rs`, `teleport/*` |
| 14 | `world.getBlockState(pos)` answering the erased `JavaObject` | `get_block_state_typed` | 1 | `predictive_smooth_aote.rs` |
| 15 | a value-typed placeholder (`unresolved::FabricClientCommandSource`, JEI `IGuiHelper`) `.clone()`d | move it / pass a reference | 7 | `compatibility/jei/*`, `museum/museum_item_cache.rs` |
