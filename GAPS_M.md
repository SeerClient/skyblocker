# GAPS - batch M (burn-down)

Genuine gaps found while replacing placeholders in batch M's packages. Each was checked against RustSrc (grep of the
function / hook / API) before being written down. Sites keep a `// rust: port gap` marker naming the number here.
RustSrc was not edited.

| # | Java member / mechanism | What the port has | What is needed | Sites |
|---|---|---|---|---|
| 1 | `BossHealthOverlay.extractBar` called from a mixin (`@Shadow`) | `boss_health_overlay.rs:431 fn extract_bar` is private (`fn`, not `pub fn`) | make it `pub` | `mixins/boss_health_overlay_mixin.rs` (1) |
| 2 | `LevelLoadingScreen.extractBackground` HEAD | no Rust body; the loading screen's background is drawn inline by the client loop (`minecraft.rs:17098`) | a hooked `extract_background` | `mixins/level_loading_screen_mixin.rs` (1, unregistered) |
| 3 | `Screen.extractRenderState` HEAD (cancel) | no Rust body; `ServerReconfigScreen` is drawn by the loop's widget pass | a hooked body | `mixins/screen_mixin.rs` (1, unregistered) |
| 4 | `LevelLoadingScreen.tick` (`super.tick()` -> `Screen.tick`) | `tick_progress` is hooked but has no caller (the port holds no screen object, DIVERGENCES chunk 362) | call it from the loop | `mixins/screen_mixin.rs` (registered, never fires) |
| 5 | authlib `YggdrasilMinecraftSessionService.unpackTextures` `LOGGER.error(String, Throwable)` | an `eprintln!` (no hookable logger call, no `Throwable`) | as GAPS_batchD #1 | `mixins/yggdrasil_minecraft_session_service_mixin.rs` |
| 6 | `Screen.extractBackground`'s `Hud.extractDeferredSubtitles()` call (between the backdrop and a container panel) | each `X#extractBackground` draws gradient + panel in one body through unhooked helpers | a hook point after the gradient | `mixins/quick_nav_screen_mixin.rs` (unselected quick-nav tabs drawn over the panel, at `AbstractContainerScreen#extractRenderState` HEAD) |
| 7 | `AbstractContainerScreen.mouseClicked`'s `super.mouseClicked(..)` answer | decided outside the hooked body (`MouseClickedOutcome::SuperHandled`) | a hooked call | `mixins/abstract_container_screen_mixin.rs` (unregistered) |
| 8 | `GuiGraphicsExtractor.item(ItemStack, III)` inside `extractSlot` | the item is drawn by the unhooked `hud::extract_item` | a hooked call | `mixins/abstract_container_screen_mixin.rs` `drawOnItem` (registered at `extractSlot` HEAD with the call's conditions instead) |
| 9 | Gson reflective binding (`GSON.fromJson(json, ApiProfileResponse.class)`, `@SerializedName`, UUID hierarchy adapter) | `crate::unresolved::Gson` (a name); the ~40 `profileviewer2.model` classes have no JSON binder | a binder (or codecs) for the model | `skyblock/profileviewer2/profile_viewer.rs` (profile always "Failed to load Skyblock profiles.") |
| 10 | NEU repo library (`NEURepoFile.isFile/stream`, `NEUItem.getLore/getDisplayName/getSkyblockItemId`, `NEUConstants.getEssenceCost`) | `crate::unresolved` placeholders; `NEURepoManager.tree/file` answer nothing (GAPS_L) | the library | `dwarven/block_break_prediction.rs`, `hunting/attributes.rs`, `searchoverlay/search_over_manager.rs` (private gap helpers) |
| 11 | JVM reflection into other mods (`Class.forName`, `MethodHandles.Lookup`, `Field.get`, `MethodHandle.invoke`) | no JVM | none possible; the lookup fails as `ClassNotFoundException` | `compatibility/{caxton,iris,modern_ui}_compatibility.rs` |
| 12 | REI API (`TransferHandler.Context`, `ApplicabilityResult`, `CompoundEventResult`, ..) | `crate::unresolved` placeholders, no REI | REI | `compatibility/rei/*.rs` |
| 13 | detach a `ModMenu` (`player.containerMenu = defaultHandler`) and open the vanilla `ContainerScreen` over the same menu in place of a mod container screen | `container_screens` offers screen/menu swaps only from `fromPacket` | a `container_screens` API | `storageoverlay/storage_overlay_screen.rs` `hide` |
| 14 | `AbstractContainerScreen.onClose` override (`super.onClose()`) | `ModContainerScreen` has no `on_close` | an override slot | `storageoverlay/storage_overlay_screen.rs` `onClose` (never reached) |
| 15 | `GuiGraphicsExtractor.setTooltipForNextFrame(Font, ItemStack, x, y)` from a mod draw | the deferred-tooltip slot is the screen pass's `Option<DeferredTooltip>`; no mod access | a `game::`/graphics API | `storage_overlay_screen.rs`, `searchoverlay/overlay_screen.rs` |
| 16 | `ClientReceiveMessageEvents.ALLOW_GAME` | no event; `client_packet_listener::apply_system_chat` is unhooked; `ChatListener#handleSystemMessage` is hooked but has no production caller | an event or a hooked apply | `utils/chat/chat_message_listener.rs` (every chat filter), GAPS_batchG L2 |
| 17 | `ModScreen` `extractBackground` override / `Screen.extractBackground` super | not in `ModScreen` (DIVERGENCES chunk 371) | an override slot | `searchoverlay/overlay_screen.rs` |
| 18 | networth library (`net.azureaaron.networth`) | `crate::unresolved` placeholders | the library | `utils/networth/networth_calculator.rs` |
| 19 | Discord IPC (`meteordevelopment.discordipc.RichPresence`) | placeholder | the library | `utils/discord/discord_rpc_manager.rs` |
| 20 | `FogRenderer.setupFog` as the per-frame fog | the frame uses the unhooked `setup_fog_with_fluid_in_camera` (port optimisation) | hook it or route through `setupFog` | `mixins/fog_renderer_mixin.rs` (registered at `setupFog` RETURN) |
| 21 | `AbstractContainerMenu.addSlotListener(this)` with a mod object | `ContainerListener: Send + Sync`; the solver is a render-thread `Shared` | a render-thread listener slot | `experiment/chronomatron_solver.rs` |
| 22 | `@At("NEW", IllegalStateException)` in `SynchedEntityData.assignValue` | not a call | - | `mixins/synched_entity_data_mixin.rs` (unregistered, as before) |
| 23 | client commands with arguments built by other packages (`SpeedPresets.getCommandNode`, `CallAutocomplete.commandNode`) | still typed `CommandNode<unresolved::FabricClientCommandSource>` / `LiteralCommandNode` | retype to `Arc<CommandNode<FabricClientCommandSource>>` | `mixins/clientbound_commands_packet_mixin.rs` (cross-package) |
| 24 | `PartyFinderScreen::new(.., Inventory, ..)` | takes the port's `Inventory` by value (file in no batch) | `compat::PlayerInventory` | `mixins/menu_screens_constructor_mixin.rs` (cross-package) |

Earlier batches' gaps still marked in these files (GAPS_batchD #1-#22, GAPS_batchA json-data async) were re-checked
and stand.
