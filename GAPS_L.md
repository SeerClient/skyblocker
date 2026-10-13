# GAPS - burn-down batch L

Genuine gaps found while replacing placeholders in batch L's packages (`utils/`, `skyblock/item/`, top-level
`skyblocker/`, `config/` (direct files), `config/backup/`, `skyblock/speedpreset/`, `utils/config/`,
`skyblock/{bazaar,foraging,end,slayers,special,fishing}/`, `src/`). Each was checked against `RustSrc` before it was
listed. The site keeps a `// rust: port gap ..` marker.

## Whole libraries (Fabric loads them beside the mod; none is in the crate)

| # | Library | What is missing | What it costs | Sites |
|---|---|---|---|---|
| L1 | **NEU repo parser** (`io.github.moulberry.repo`) + **JGit** (`org.eclipse.jgit`) | `NEURepository.of/reload`, `NEUItem`, `NEUConstants.getBazaarStocks`, `NEURecipeCache`, `ItemOverlays`; JGit clone / fetch / reset / stash | The NEU item repo never loads. `REPO_LOADING` is now a pending future (Java's "still loading"), `runAsyncAfterLoad` answers a pending future (the task waits, as it would in Java until the load completes) and the getters answer an empty repo instead of panicking. Every after-load task (item list, recipes, museum, fairy souls, pet numbers, autocompletes, enchanted-book ids, ...) never runs. | `utils/neu_repo_manager.rs` (4 todos: Git, `runner`), `utils/enchanted_book_utils.rs` (4 todos, NEUItem), `skyblock/item/skyblock_item_rarity.rs` (`Rarity` now a real enum in `unresolved.rs`) |
| L2 | **Dandelion / YACL** (config library) | controllers (`BooleanController`, `ColourController`, `EnumController`), `ConfigPatch.PATCH_LIST_CODEC`, `ControllerBuilder`, `StringControllerElement`; the config GUI (`SkyblockerConfigManager.createGUI`) | No config screen; config patches cannot parse. The config merge is meant to replace these. | `config/config_utils.rs` (5), `config/config_patch_loader.rs` (1), `utils/config/duration_controller*.rs` (11), `skyblocker_screen.rs` `openConfig`, `config_backup_screen.rs` restore, `disable_all_popup.rs` |
| L3 | **azureaaron networth** (`net.azureaaron.networth`) | `NetworthCalculator`, `Calculation`, `PetCalculator.calculatePetLevel` | Value breakdown popup and `PetInfo.level()` cannot compute. | `skyblock/item/value_breakdown_popup.rs` (5), `skyblock/item/pet_info.rs` `level` |
| ~~L4~~ | ~~**hm-api** (Hypixel Mod API client)~~ | **CLOSED in this batch**: ported to `src/net/azureaaron/hmapi/` (43 files, the library's source from github.com/AzureAaron/hm-api). Its codec/listener mixins are the port's `networking::register_global_receiver` on `hypixel:hello`, `hypixel:player_info`, `hypixel:party_info`, `hyevent:location` (+ `hypixel:register` for sending); `HMApi.onInitializeClient` runs before Skyblocker's. `Utils.onPacket` and the event registrations are ported, so location / skyblock detection, `Utils.getEnvironment()` and `getRank()` are live. | | |

## Game / modding-API gaps

| # | Java member | What the port has | What is needed | Sites |
|---|---|---|---|---|
| G1 | `ClientReceiveMessageEvents.ALLOW_GAME` / `MODIFY_GAME` (Fabric API) | no event; `ChatListener.handleSystemMessage` runs on the CONNECTION thread here, so a hook cannot run mod code that touches render-thread statics | an `events::client_receive_message_events` module dispatching on the client thread (and able to cancel / modify) | `utils/utils.rs` `init` (`onChatMessage` - the `/profileid` + legacy locraw path), plus GAPS_batchG L2's sites |
| G2 | `UseItemCallback` (Fabric API) | none | an event | `skyblock/item/item_cooldowns.rs` `init` (the cooldown handler is ported but never called) |
| G3 | `PopupScreen.Builder` opened by a mod | the game builds its popups itself (`gui::PopupState`) | a mod-facing popup builder (or `compat` wrapper as for `ConfirmScreen`) | `disable_all_popup.rs` `open` (1 todo) |
| G4 | `Minecraft.getMusicManager().tick()` / `getSoundManager().tick(false)` / `stopPlayingMusic(music)` from a mod screen | they need the frame loop's `MusicContext` / OpenAL library | a modding hook | `skyblocker_credits_screen.rs` `tick`, `removed` |
| G5 | a list ROW asking for a cursor (`context.requestCursor` inside `Entry.extractContent`) | `ModListEntry` has no `handle_cursor`; the list answers only its scrollbar's | a `ModListEntry::handle_cursor` slot | `config/backup/config_backup_screen.rs` `BackupEntry` |
| G6 | a mod list overriding `extractScrollbar` | `ModList` has no `extract_scrollbar` slot; the game draws the scrollbar | the slot | `config/backup/config_backup_screen.rs` `SettingsListWidget.extractScrollbar` (the changed-path markers) |
| G7 | `screen.onClose()` of the vanilla sign screen from a mod button | `on_close_for_interrupt` needs the frame's state | a modding hook (`game::close_screen`) | `skyblock/bazaar/bazaar_quick_quantities.rs` |
| G8 | `case AbstractPopupScreen` in `ScreenUtils.getUnderlyingScreen` | the mod popup's `backgroundScreen` is a stand-in (GAPS_batchA #19) | the object behind a mod screen reachable from its `Screen` | `utils/screen_utils.rs` (the vanilla `PopupScreen` case is ported via `Gui.screen_extra`) |
| G9 | `VanillaRegistries.createLookup()` | the built-in half (`RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY)`); the data-generator bootstraps are datagen-only | - | `utils/registry_utils.rs` `LOOKUP` (built-in registries only) |
| G10 | `ZoneId.of`, ICU `DateTimePatternGenerator` | system zone only | - | `utils/formatters.rs` (kept from batch E) |
| G11 | log4j `LoggerContext` | the game does not log through log4j | - | `utils/logs_folder_finder.rs` (the fallback folder is taken; the todo after the `None` is unreachable) |
| G12 | Fabric loader `McVersionLookup` snapshot table | - | - | `update_notifications.rs` `normalize_minecraft_version` (releases, `-preN`, `-rcN` handled; a snapshot id fails the semver parse, which the update check catches) |
| G13 | brigadier command trees built outside `dispatcher.register` (`LiteralArgumentBuilder` returned by a method) and reflection over config fields (`getDeclaredFields`) | the port's client commands take one literal + words | - | `config/config_commands.rs` (3), `utils/item_utils.rs` dump commands (2), `skyblock/speedpreset/speed_presets.rs` `getCommandNode` |
| G14 | mod-owned `Slot` over a mod `SimpleContainer`; a mod `InventoryScreen` subclass | `SkyblockInventoryScreen` has no `ModContainerScreen` impl, so its overrides are never dispatched | - | `skyblock/item/skyblock_inventory_screen.rs` (the bodies are now ported; `return super.x()` written as the `ModContainerScreen` "answer false" convention) |
| G15 | the recipe book of a mod container screen (`RecipeBookComponent.slotClicked / tick / extractGhostRecipe / setFocused`) | GAPS_batchE: not a widget of the screen | - | `skyblock/item/skyblock_crafting_table_screen.rs` |
| G16 | `java.text.CompactNumberFormat`, `DecimalFormat.setPositivePrefix`, `java.net.URI.getPath`, commons-io `FilenameUtils`, commons-lang `NumberUtils` | not in the port's JDK surface | written in the mod (`utils/formatters.rs` `CompactNumberFormat` / `PositivePrefixDecimalFormat`; `compat::uri_get_path`, `filename_get_*`, `number_utils_*`) | - |

## Known divergences kept (documented at the site)

- `FlexibleItemStack`'s memo getters (`getSkyblockId`, `getLoreStrings`, ...) recompute instead of caching: the
  getter is handed `&self` inside the object's cell, so the cache field cannot be written (same answers).
- `CompoundTag.keySet().stream().findFirst()` (enchanted book / rune ids): the port's compound keeps keys SORTED,
  Java's is a hash map; one-key compounds (the normal case) agree.
- The hm-api channels are announced in `minecraft:register` (the port's receivers are announced); Fabric's hm-api hijacks
  the codec and announces nothing.
