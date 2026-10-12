# GAPS - batch D (`mixins/`, `skyblock/dwarven/`, `skyblock/waypoint/`)

These are things the game crate (RustSrc) or the mod runtime does not provide. They turned up while fixing the batch D
compile errors. RustSrc was NOT edited. Each site is marked in the source with `rust: port gap`.

Each claim was checked by grep against RustSrc before it was written down.

## Missing or unhookable in RustSrc

| # | What Java uses | What the port has | Mod sites |
|---|---|---|---|
| 1 | slf4j `Logger.warn/error` as a mixin call-site target (`@WrapWithCondition` on a `Logger` call) | no hookable logger: the game's log calls are not `mcrust_hook` sites | `fishing_hook_mixin`, `scoreboard_mixin`, `skin_manager_mixin`, `yggdrasil_*_mixin` (not registered) |
| 2 | `PacketUtils.ensureRunningOnSameThread` as a hook point | the listener's `_borrowed` / `_for` handlers bypass the hooked method | `client_packet_listener_mixin` (HEAD injectors check `render_system::is_on_render_thread()` instead) |
| 3 | `ClientPacketListener.handleTabListCustomisation` -> `setFooter` | `setFooter` is deferred to the client loop, so the INVOKE site does not exist | `client_packet_listener_mixin` footer (an `At::Return` injector on the handler) |
| 4 | `Level.setBlock` HEAD (the old state before the update) | `super_set_block` is not hooked | `client_level_mixin` before-update |
| 5 | `DataComponentHolder.get` as the target `ItemStack.get` dispatches to | `ItemStack::get` is not the hooked `DataComponentHolder#get` | `data_component_holder_mixin` |
| 6 | `AbstractContainerScreen.extractTooltip -> setTooltip` | the tooltip is set in the unhooked `extract_render_state_tail` | `abstract_container_screen_mixin` tooltip wrapper |
| 7 | `AbstractContainerScreen.extractSlot` drawing through `GuiGraphicsExtractor` | the slot is drawn through `gl` / `gfx` with `&ItemStack` arguments | `abstract_container_screen_mixin` display-stack modifiers |
| 8 | `ContainerScreen` background position (`leftPos` / `topPos`) | `background_position` is unhooked | `container_screen_mixin` (xo / yo) |
| 9 | `AbstractSignEditScreen.keyPressed(KeyEvent)` | the port hands booleans, and `on_close` does not close | `abstract_sign_edit_screen_mixin` keyPressed |
| 10 | `Entity.isInvisibleTo(Player)` | value parameters, not hookable by reference | `entity_mixin` |
| 11 | a hook on an `Entity` trait default method seeing a `dyn Entity` | the hook's `this` is the concrete class (no dyn view) | `entity_mixin` onRemoval (tries a class list) |
| 12 | `with_entity` by handle | only by id (`game::with_entity(id)`); `compat::with_entity_handle` wraps `lock_entity` | `client_level_mixin`, `metal_detector` |
| 13 | `LivingEntity.hasLineOfSight(Entity)` | not ported (`compat::has_line_of_sight` transcribes `LivingEntity:3339-3355`) | `corpse_finder` |
| 14 | `CarpetBlock.SHAPE` | private | `carpet_highlighter` |
| 15 | `ClientTooltipComponent` downcast | no `as_any` | `gui_graphics_extractor_mixin` grids |
| 16 | `RecipeBookComponent` subclassing (replace the recipe book) | a concrete struct | `inventory_screen_mixin` (`modify_args`) |
| 17 | a component system message from a mod (`player.sendSystemMessage(Component)`) | only `script_host::send_feedback(&str)`, a text-only message: click / hover events and styles are dropped | `call_mismyla`, `redial_on_bad_signal`, `crystals_locations_manager` |
| 18 | `Item.appendHoverText` builder hook | not hookable | `item_stack_mixin` addDetailsToTooltip |
| 19 | `Options.dataFix`, `SkinTextureDownloader.processLegacySkin` | not ported | `options_mixin`, `skin_texture_downloader_mixin` |
| 20 | `@At("NEW")` | `hook::call_site` cannot target a constructor call | several mixins (noted at each site) |
| 21 | `ScreenEffectRenderer` item-name drawing with the outer method's locals | no locals capture | `screen_effect_renderer_mixin` |
| 22 | `MultiPlayerGameMode.lambda$useItem$..` | no port method | `multi_player_game_mode_mixin` |
| 23 | `ConfirmScreen` from a mod (`new ConfirmScreen(callback, title, msg, yes, no)`) | no constructor for mods and no `game::` request for one | `waypoints_screen.rs` `onClose` (the "discard changes?" prompt) |
| 24 | `HeaderAndFooterLayout.addTitleHeader(Component, Font)` | not ported | `corpse_profit_screen.rs` `init` (its body, `headerFrame.addChild(new StringWidget(..))`, is written at the call; a hook on it cannot see the call) |
| 25 | `NumberFormat.getInstance()` (the default locale) | `java::text` has `getIntegerInstance(Locale)`, and only the ROOT / US locales | `corpse_profit_screen.rs`, `reward_list.rs`, `corpse_list.rs` (`#,##0.###` over US symbols) |
| 26 | `CompletableFuture.thenRunAsync(Runnable)` / `runAsync(.., VIRTUAL_THREAD_EXECUTOR)` on mod objects | the port runs a task on an `Executor` that takes `Send` closures, and the mod's objects are render-thread only | `fairy_souls.rs`, `relics.rs`: the task goes through `game::execute` (the game's task queue, on the render thread) |
| 27 | a Mixin `@Unique` field on a screen (attachment on `Screen`) | `AttachmentTarget` is implemented for entities only (`attach.rs:231`), not `Screen` | `quick_nav_mixin.rs`: a static stands for the field of the screen being initialised |
| 28 | `AbstractSignEditScreen.onDone` HEAD with a mutable `this` (rewrite `messages`) | the port's `on_done(&self)` is `escape_saves()` - not Java's send-and-close, and no mutable receiver | `abstract_sign_edit_screen_mixin.rs` `finishEditing` (returns without rewriting) |

## Libraries Java has that the port does not

| # | Library class | Use | What was done |
|---|---|---|---|
| L1 | Fabric `net.fabricmc.fabric.api.util.TriState` | `MythologicalRitual.GriffinBurrow.confirmed` | the game's `net.minecraft.util.TriState` (the same three constants, compared by identity) |
| L2 | commons-math3 `Vector2D`, `Line`, `SimpleRegression` | the burrow direction estimate | the parts used are written in `mythological_ritual.rs`: `addData`, `getSlope`, `clear`, `Line(p1, p2, tol)`, `intersection`, and `MathArrays.linearCombination` |
| L3 | Gson `toJson` (pretty, `htmlSafe` default) | waypoint / fairy soul / relic files, Skytils export | serde_json: the same structure and indentation, but `<`, `>`, `&`, `=`, `'` are not escaped as `<` etc. The output reads back the same |
| L4 | `java.util.zip.GZIPOutputStream` | the Skyblocker waypoint export | `java::util::zip::gzip` (header identical; the DEFLATE body differs byte-for-byte from zlib's, and every inflater reads it) |
| L5 | Guava `Multimap` (`MultimapBuilder.enumKeys().arrayListValues()`) | the waypoint map | `waypoints::LocationMultimap` (EnumMap order, ArrayList values, Guava's empty-key and `equals` semantics) |
