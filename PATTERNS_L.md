# PATTERNS - burn-down batch L

Converter placeholders / defects seen three or more times in batch L, with the fix used. Paths are relative to
`src/de/hysky/skyblocker/`.

## Silent behaviour changes

| # | Wrong shape | Correct shape | Count | Example |
|---|---|---|---|---|
| S1 | a negative float literal as a method receiver: `-2.42f32.mul_add(..)` parses as `-(2.42.mul_add(..))` (introduced when `Math.fma` todos are filled mechanically) | `(-2.42f32).mul_add(..)` | 5 | `utils/ok_lab_color.rs` |
| S2 | an async body (`CompletableFuture.runAsync / supplyAsync`, a worker) reading `compat::MainThread` statics (codecs, paths) - a panic on the worker | read the statics on the render thread, move the values in (`*_in(..)` helpers) | 4 | `config/image_repo_loader.rs`, `utils/api_authentication.rs` (codecs), `skyblock/item/skyblock_item_data.rs` |
| S3 | a Java `switch` / `if` body dropped as a comment block with only a `TODO(fabric2seer)` header (no `todo!`, compiles, does nothing) | port the body | 4 big ones | `utils/item_utils.rs` `getSkyblockApiId` (the whole id switch), `utils/utils.rs` `tickProfileId`, `utils/utils.rs` `onPacket`, `utils/api_authentication.rs` token POST |
| S4 | a cache map put into its static BEFORE it is filled, filled through the local copy | fill, then store (or write through the static) | 1 | `utils/lz_string.rs` `getBaseValue` |
| S5 | `std::env::var(..).ok().unwrap()` for `System.getProperty(key, default)` - panics when unset | `.unwrap_or_else(|_| default)` | 1 (+3 in the hm-api port) | `utils/fun_utils.rs` `FUN_DISABLED` |
| S6 | `Files.delete(dir)` as `remove_file` (fails on a directory) | `remove_dir` for a directory | 1 | `utils/file_utils.rs` |
| S7 | `Holder::is_identifier_direct` (the `Direct` arm's constant `false`) for `holder.is(id)` | `holder.is_identifier(id)` | 1 | `utils/utils.rs` `isInBiome` |

## Placeholder shapes (todo! / rust:) and their fills

| # | Placeholder | Fill | Count | Example |
|---|---|---|---|---|
| 1 | `slot.getItem()` ("`get_item` takes 1 argument") | `compat::slot_item(slot)`; a menu slot by index: `compat::with_open_menu(|m, c| m.slots[i].get_item(c).duplicate())` | 9 | `skyblock/item/item_price.rs`, `item_protection.rs` (2), `bazaar/bazaar_helper.rs`, `skyblock_crafting_table_screen.rs` (3) |
| 2 | `Http.sendGetRequest / sendPostRequest / sendHypixelRequest / downloadContent` ("not a static method of Http") | `crate::..::utils::http::Http::send_*` (they are ported; the converter missed the path) | 8 | `update_notifications.rs`, `config/config_patch_loader.rs`, `utils/profile_utils.rs` |
| 3 | `KeyMappingHelper.registerKeyMapping(new KeyMapping(..))` static | `key_binding_helper::register_key_binding(KeyMapping::new(name, glfw_key, *KEYBINDING_CATEGORY.get()))`, and the class's `init` touches the static (Java's class initialisation) | 2 | `skyblock/item/item_price.rs`, `value_breakdown_popup.rs` |
| 4 | `StringRepresentable.fromEnum(X::values)` ("takes 2 arguments") | `string_representable::from_enum(X::values(), |e: &X| e.get_serialized_name())` | 2 | `utils/location.rs`, `skyblock/item/skyblock_item_rarity.rs` |
| 5 | `new ConfirmScreen(callback, ..)` + `setScreen` | `game::set_screen_keeping_parent(Box::new(compat::ConfirmScreen::new(..)))`; `setScreen(this)` = one `set_screen_to_parent()`, `setScreen(parent)` = two | 2 | `speedpreset/speed_presets_screen.rs`, `config/backup/config_backup_screen.rs` |
| 6 | `NumberUtils.toInt / isCreatable`, `FilenameUtils.getBaseName / getExtension`, `URI.getPath` | the new `compat::number_utils_*`, `compat::filename_get_*`, `compat::uri_get_path` | 7 | `end/end_stats_bestiary_updater.rs`, `bazaar/bazaar_helper.rs`, `config/backup/config_backup_manager.rs` |
| 7 | an inner (non-static) class reading / writing a field of its enclosing screen ("a field of the enclosing instance") | the field is a shared cell (`Arc<Mutex<..>>`) handed to the inner objects at construction; `field = x` replaces its contents | 4 | `config/backup/config_backup_screen.rs` `changedPaths` |
| 8 | `Math.fma(a, b, c)` | `a.mul_add(b, c)` (fused, as Java's) - mind S1 | 13 | `utils/ok_lab_color.rs`, `utils/color_utils.rs` |
| 9 | a mod screen's layout built once from widgets whose size later changes (`tip.setMessage; layout.arrangeElements()`) | keep the widget indices, rebuild the layout from the LIVE sizes in one `arrange` fn (batch E pattern 9) | 1 | `skyblocker_screen.rs` |
| 10 | `ConfirmLinkScreen.confirmLink(this, url)` as a button's press | `compat::confirm_link(url)` (parks the game's own trusted link screen, `Minecraft.pending_confirm_link`) | 8 | `skyblocker_screen.rs` |
