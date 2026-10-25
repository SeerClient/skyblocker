# GAPS - batch U

What is still marked `rust: port gap` in batch U's packages after the burn-down. Each was checked against RustSrc
(read-only for this batch) or against the owning package.

| # | Java | What blocks it (verified) | Kind | Site |
|---|---|---|---|---|
| U1 | `getFixer().update(TypeReferences.LEGACY_ITEM_STACK, ..)` (Legacy Item DFU) | `net.azureaaron.legacyitemdfu` has no Rust port; no file under `src/net/azureaaron/` (only `hmapi`, `networth`). A new library package belongs to the library worker | library | `utils/datafixer/legacy_item_stack_fixer.rs` (the one `todo!` left; every NEU item built by `ItemStackBuilder` reaches it) |
| U2 | `GSON.fromJson(json, ApiProfileResponse.class)` | Gson's reflective binder over ~60 classes in `profileviewer2/model/` (outside batch U, ~700 fields, polymorphic adapters); no reflective binder in the port. Feature is debug-only (`Debug.debugEnabled()`) | subsystem | `profileviewer2/profile_viewer.rs` (2) |
| U3 | `DisableAll.disableEntries` reflection (`getDeclaredFields`, `@EnumDisabledValue`) | the config classes are being replaced by the generated merge (do-not-touch package); needs a field visitor there | cross-package | `disable_all.rs` |
| U4 | `getMusicManager().tick()` / `getSoundManager().tick(false)` / `stopPlayingMusic` | `minecraft::sound_manager_tick` / `music_manager_tick` take the frame loop's `audio` (OpenAL) and options, not lent to mods (GAPS_L G4) | game | `skyblocker_credits_screen.rs` (2) |
| U5 | `RadialMenuScreen.mouseMoved` | `ModScreen` has no `mouse_moved` slot (`modding/screens.rs`) | game | `radial_menu/radial_menu_screen.rs` |
| U6 | `setScreen(new ContainerScreen(handler, inventory, parentName))` | a mod can only open a `ModScreen`; `screens::open_over_menu` is loop-internal | game | `radial_menu/radial_menu_screen.rs` `hide` (2) |
| U7 | anonymous `RemotePlayer` subclass (`getSkin`, `isModelPartShown`, `isInvisibleTo`) | entity classes take no mod subclass (GAPS_batchB #9) | game | `profileviewer/profile_viewer_screen.rs` |
| U8 | `ProfileViewerScreen.children()` override | `ModScreen` has no `children()` slot | game | `profileviewer2/profile_viewer_screen.rs` (doc note) |
| U9 | Fabric Loader `McVersionLookup` snapshot table | the loader's year/week table is not in the crate; unreachable for this client's release id | library | `update_notifications.rs` (doc note) |

Closed in this batch (were marked gaps): the deferred tooltip from mod draw code (`compat::batch_s_set_tooltip_for_next_frame`,
3 sites), `PopupScreen.Builder` for a mod (`compat::BatchUPopupScreenBuilder`), `rebuildWidgets` from a task
(profileviewer), `NetworthCalculator.getItemNetworth(..).price()` (the library is ported), the REI transfer handler's
`Scheduler.schedule(checkScreen)`, `ScreenEvents.afterTick` for the experiment solvers and Chronomatron's
`addSlotListener` / `removeSlotListener`.
