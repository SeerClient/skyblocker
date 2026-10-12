# GAPS - batch G

Packages: `skyblock/` (direct files), `skyblock/{garden, slayers, crimson, accessories, events, entity, rift, barn, special}/`,
`compatibility/rei/`, `config/` (direct files), `src/entry/`. RustSrc was NOT edited. Every RustSrc row was checked by
grepping RustSrc. Sites are marked `rust: port gap` (or `rust: YACL port pending`) in the source.

## Missing in RustSrc

| # | Java member | What the port has | What is needed | Sites |
|---|---|---|---|---|
| 1 | The vanilla `ContainerScreen` / `AbstractContainerScreen` object (its `leftPos`/`topPos`/`imageWidth` through `AbstractContainerScreenAccessor`, `getMenu()`, `Screens.getWidgets(screen)`) lent to a `ScreenEvents.AFTER_INIT` listener | the listener gets the `Screen` half only; `instanceof ContainerScreen` is answered from `gui.screen_kind` (`ScreenKind::Container`), the title from `screen.get_title()` | a lend of the live container screen (or its base fields + widget list) to AFTER_INIT listeners | `accessories/accessories_helper.rs` (`attachToScreen` not called), `accessories/accessories_helper_widget.rs` `attach_to_screen` (`screen.leftPos = x` in the tab button), `chest_value.rs`, `crimson/kuudra/kuudra.rs`, `pet_cache.rs`, `inventory_search.rs`, `garden/garden_plots.rs` (the whole AFTER_INIT body), `compatibility/rei/*` `checkScreen` (kind check used) |
| 2 | `ScreenEvents.afterTick(screen)` / `ScreenEvents.remove(screen)` (per-screen Fabric events) | `screen_events` has `BEFORE_INIT`/`AFTER_INIT` | the two events | `accessories_helper.rs`, `chest_value.rs`, `kuudra.rs`, `pet_cache.rs`, `garden/visitor/visitor_helper.rs`, `garden/garden_plots.rs` (converter TODOs remain) |
| 3 | A mod subclass of `EditBox` (`SearchTextFieldWidget extends EditBox`) | `EditBox` is the `WidgetKind::EditBox` of a game `AbstractWidget`; focus, hover and the click pass live on that `AbstractWidget` | a ModWidget form for EditBox subclasses | `inventory_search.rs` `SearchTextFieldWidget.mouseClicked` (returns `false`, documented; the class's constructor is the converter's todo) |
| 4 | Running a child `OnPress` from a mod composite widget (`ContainerEventHandler.mouseClicked` over `ItemButtonWidget` children) | `OnPress::Lambda` takes `&mut Screen`; a mod container widget is not lent one | the screen, or a press path for widgets outside a screen | `garden/garden_plots_widget.rs` `mouse_clicked` (child found, its press not run) |
| 5 | `Minecraft.getEntityModels()`, `Minecraft.playerSkinRenderCache()` | not on `Minecraft` (`game_renderer.player_skin_render_cache` is a field) | the accessors (batch B #15 also) | `garden/greenhouse_paste.rs` skull preview (converter todos remain) |
| 6 | `instanceof CropBlock` | `Block` carries no class identity | a class tag | `garden/greenhouse_paste.rs` (answered by `has_property(AGE_7)`, exact for the blocks `CROP_ID_MAP` names) |
| 7 | `Base64.getUrlDecoder()`, `new Inflater(true)` (raw deflate) | `java::util::base64` has no URL flag; `Inflater::new()` is zlib-wrapped only | the URL decoder and the `nowrap` inflater | `garden/sky_shards_layout.rs` `decompress` (converter todos remain) |
| 8 | `DecimalFormat.parse` | `DecimalFormat` has `new` + `format` only | `parse` | `accessories/newyearcakes/new_year_cakes_helper.rs` (`\d+` parsed directly), `utils/formatters.rs` `parseNumber` (batch A) is still a todo, which `item_pickup_widget.rs`, `health_bars.rs`, `visitor_helper.rs` call |
| 9 | `minecraft.gui.screen().onClose()` from a mod (the OPEN screen's own `onClose`) | no game API | e.g. `game::close_current_screen()` | `garden/garden_plots_widget.rs` `onClick` |
| 10 | Java reflection (`getDeclaredFields`) | none (Rust) | n/a - a design decision | `config/config_null_fields_fix.rs` `fixNullFields`, `config/config_commands.rs` `registerConfigEntries` (parameters are now `&dyn Any`) |

## Missing mod runtime / libraries (not RustSrc)

| # | What | Effect | Sites |
|---|---|---|---|
| L1 | **YACL** (`ConfigManager`/`ConfigClassHandler`): construction, serializer, load/save of `config/skyblocker.json`, patched vs unpatched instance | `SkyblockerConfigManager::INSTANCE` holds the ONE live config (starts as `SkyblockerConfig::new()`); `get()` lends it, `get_unpatched()`/`update`/`update_only` mutate it in place; load/save/patches marked `rust: YACL port pending` | `config/skyblocker_config_manager.rs` |
| L2 | `ClientReceiveMessageEvents.ALLOW_GAME`, `AttackEntityCallback`, `ClientEntityEvents.ENTITY_UNLOAD`, `ClientPlayerBlockBreakEvents.AFTER`, `ClientLifecycleEvents.CLIENT_STOPPING`, `END_LEVEL_TICK`, `CustomGlowCallback` | handlers exist, never registered | `slayers/slayer_manager.rs` (`onChatMessage`, `onEntityAttack`), `slayers/boss/voidgloom/beacon_highlighter.rs`, `item_pickup_widget.rs`, `crimson/kuudra/kuudra.rs`, `health_bars.rs`, `garden/farming_hud.rs`, `garden/garden_plots.rs`, `entity/mob_glow.rs` |
| L3 | **renderchest** `GlowConstants.NO_GLOW` / `REMOVE_GLOW` | `MobGlow::NO_GLOW` initialiser and `applyCustomGlow`'s returns stay the converter's todos | `entity/mob_glow.rs` |
| L4 | **NEU repo parser** (`NEUItem.getSkyblockItemId`, `getDisplayName`) - batch B L1 | the item found by name cannot be resolved: the placeholder / the id stands in | `garden/visitor/visitor_helper.rs` `getCachedItem`, `item_pickup_widget.rs` `getItem`, `sack_item_autocomplete.rs` |
| L5 | **REI** (`EntryStack`, `Widget`/`Slot`, `CompoundEventResult`, transfer `Context`) | placeholders; `widgets.add(slot)` and the scheduled `checkScreen` cannot be expressed | `compatibility/rei/*` |
| L6 | **commons-lang** `NumberUtils.isParsable` / `isCreatable`, `StringUtils.join` | `compact_damage.rs` answered exactly (digits-only input); `kuudra.rs` `isCreatable` stays a todo | as listed |
| L7 | A **mod `Toast`** adapter | added `event_toast.rs` `EventToastAdapter` (dispatches each `Toast` method to the class); the body draws through the `gui_graphics_extractor` functions with `GuiGraphicsExtractor::new()`, not the toast's `TooltipGraphics` | `events/event_toast.rs`, `events/event_notifications.rs` |
| L8 | Mod composite widgets (`extends AbstractContainerWidget`) - batch B L3 | `GardenPlotsWidget`, `AccessoriesHelperWidget` compile with their Java bodies but are not ModWidgets, so nothing adds them to a screen | as listed |
