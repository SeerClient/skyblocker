# PATTERNS - batch M

Shapes seen three or more times in batch M (mixins, profileviewer2, dwarven, storageoverlay, compatibility, ...).

| # | Wrong shape | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | Mixin handler needing an argument of the CALLER (`@Local`, the target method's own parameter) at a call-site injector, left as "not visible to the injector" | record it in a `thread_local!` from a HEAD injector on the caller (cheap: a bool, a String, a cloned `Slot`) and read it in the call-site handler | 5 | `dyed_item_color_mixin.rs`, `gl_command_encoder_mixin.rs`, `abstract_container_screen_mixin.rs` (`EXTRACT_SLOT`) |
| 2 | `@Local` that is actually the call's RECEIVER (`decoration.renderOnFrame()`) handled with `modify_expression_value` (no access to it) | `inject(Shift::After, ..)` + `ci.this::<T>()` + `return_value` / `set_return_value` | 2 (+hud batch D) | `map_renderer_mixin.rs` |
| 3 | `@At("NEW")` / `@At("CONSTANT")` / `@At("FIELD")` declared inexpressible although the port has a hooked function there (record constructor `X#<init>`, `GuiRenderer#render` after `push("gui")`, the `return fog` after the PUTFIELD) | register at that hooked function (call site / HEAD / RETURN) | 3 | `level_extractor_mixin.rs`, `game_renderer_mixin.rs`, `fog_renderer_mixin.rs` |
| 4 | `@ModifyVariable(argsOnly = true, at = HEAD)` called inexpressible | it is a HEAD argument write (`hook_arg_mut` on the parameter) | 1 | `equipment_layer_renderer_mixin.rs` |
| 5 | `WrapWithCondition` on a call that is a method's WHOLE body (`Logger.warn` in `handleUnknownCustomPayload`) left unregistered | HEAD injector that cancels | 1 | `client_packet_listener_mixin.rs` |
| 6 | `"graphics not visible"` in a handler whose port target takes `gl`/`gfx` | `GuiGraphicsExtractor::new()` (the port's draws go through the frame's global state) / `hook_arg::<Gl>`, `hook_arg::<Minecraft>` | 5 | `boss_health_overlay_mixin.rs`, `abstract_container_screen_mixin.rs` (3) |
| 7 | getter answering a COPY (`get_player_data`, `get_slots`) mutated (`insert`, `remove`) | write through the owner (`with_player_data`, `experiment_solver_base_mut().slots`) | 5 | `utils/data/profiled_data.rs` (4), `experiment/superpairs_solver.rs` |
| 8 | inner (non-static) class reading `Outer.this` fields left as TODO | `outer: WeakShared<Outer>` field passed by the outer's constructor call | 9 sites | `storageoverlay/storage_overlay_screen.rs` (BackpackGridWidget, BackpackWidget) |
| 9 | static import resolved as a method of the class (`getItemStack(id)` -> `Screen.getItemStack`) | the imported class (`ItemRepository::get_item_stack`) | 1 | `searchoverlay/overlay_screen.rs` |
| 10 | unlinked library calls (NEU repo, Gson binding, REI, networth, JVM reflection) as scattered `todo!` | one private `// rust: port gap` helper per accessor with Java's "absent" answer, the surrounding logic ported for real | 20 helpers | `block_break_prediction.rs`, `attributes.rs`, `search_over_manager.rs`, `skyblock_transfer_handler.rs`, `*_compatibility.rs` |
| 11 | `LinearLayout.spacing(5);` as a statement (by-value builder, result dropped) | `layout = layout.spacing(5)` | 1 | `storage_overlay_screen.rs` |
| 12 | `(a && b || c || d)` from Java `a && (b || c || d)` | parentheses back | 2 | `storage_overlay_screen.rs` `hasClickedOutside`, `hunting/floor_drops.rs` `shouldProcess` |
| 13 | `.unwrap()` on a `@Nullable` field read before `!= null` (`grid`, `previousMousePosition`) | match on the `Option` | 2 | `storage_overlay_screen.rs` |
| 14 | `Map.putIfAbsent` translated as `entry().or_insert()` returning the stored value | answer the previous value, `None` when absent | 1 | `profiled_data.rs` |

Helpers that belong in `compat.rs` (batch L): `local_of` (worker `CompletableFuture<T>` -> render-thread `LocalFuture<T>`,
`profile_viewer.rs`), the NEU repo file accessors (`neu_repo_file_is_file` / `neu_repo_file_read_all`, now in two
files), and the JVM reflection shim (`class_for_name` etc., three files).
