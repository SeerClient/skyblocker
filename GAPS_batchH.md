# GAPS - batch H (fancybars, config/datafixer, chocolatefactory, museum, auction, teleport, end, speedpreset, compatibility/jei, compatibility/emi, debug, utils/render, utils/time)

What the game crate (RustSrc) or the mod runtime does not provide, found while fixing the batch H errors. RustSrc was
NOT edited. Every RustSrc row was checked by grepping RustSrc. Sites are marked `// rust: port gap (GAPS_batchH #n)` or
cite an earlier batch's row (`GAPS_batchA #3`, `GAPS_batchB L3`).

## Missing in RustSrc

| # | Java member | What the port has | What is needed | Sites |
|---|---|---|---|---|
| 1 | `client.getSoundManager().play(SimpleSoundInstance.forUI(..))` from a mod tick callback | `SoundManager::play(al, library, options, instance)` / `sound_engine::play_ui_sound(audio, ..)`: the audio device + library are locals of the frame loop (`minecraft.rs:2151`), not reachable from `&mut Minecraft` | a mod-side "play this UI sound" request (queued for the frame loop) | `chocolatefactory/chocolate_factory_solver.rs` `onTick` (the stray-rabbit ding: tick logic ported, the play call is the gap) |
| 2 | A mod object that is `Renderable + GuiEventListener + LayoutElement` but NOT an `AbstractWidget` added to a screen (`addRenderableWidget(statusBar)`) | screens hold only `AbstractWidget`s; `compat::WidgetAdapter` needs an `AbstractWidget` half | an adapter for a non-widget renderable/listener | `fancybars/status_bars_config_screen.rs` `setup` (the bars are not drawn / clicked by the config screen) |
| 3 | `java.time.temporal.TemporalAccessor` / `TemporalUnit` / `ChronoField` (the `SkyblockTimeField` / `SkyblockQueries` custom fields) | `crate::unresolved` placeholders | `java.time` temporal API | `utils/time/skyblock_time_field.rs` `getFrom` (Java's "unsupported" branch is the one taken), `utils/time/skyblock_queries.rs` (converter todos) |
| 4 | `Schema.registerType(true, ..)` (a recursive type) | `schema.rs:178` panics: no `RecursiveTypeFamily` | the recursive family | `config/datafixer/config_schema.rs`: the three `DSL.remainder()` types are registered non-recursively (identical type for a template that references nothing) |
| 5 | `block instanceof CarpetBlock / FlowerPotBlock / WebBlock` | `Block` carries no class; `BlocksTable` has `carpet()` but no pot/web class list; `flower_pot_block::potted_field` is private (`get_potted` is public) | a class tag on `Block` | `teleport/predictive_smooth_aote.rs` `canTeleportThrough` (answered by identity: CARPET[16] + MOSS_CARPET, FLOWER_POT or `get_potted != AIR`, COBWEB) |
| 6 | `Hud.FOOD_*_SPRITE` (`HudAccessor.getFOOD_EMPTY_SPRITE()` ..) | `hud.rs:3716-3724` inlines the six names | the constants | `fancybars/vanilla_style_mana_bar.rs` (built from the same `hud/food_*` names) |
| 7 | `RecipeBookTabButton.SPRITES` | `recipe_book_tab_button::tab_sprite()` / `tab_selected_sprite()` | the record | `auction/widgets/category_tab_widget.rs` (rebuilt with `WidgetSprites::with_focused`) |
| 8 | `player.sendSystemMessage(Component)` with click / hover styles | `script_host::send_feedback(&str)` (plain text) | a styled client chat message | `chocolatefactory/egg_finder.rs` `sendEggMessage` (the share-location click event is lost) |
| - | AbstractPopupScreen background over a MOD screen | GAPS_batchA #19 | - | `auction/edit_bid_popup.rs` (an empty `Screen` stands in), `fancybars/edit_bar_color_popup.rs` (constructor todo) |
| - | EditBox subclass (`FilteredEditBox`, `EnterConfirmTextFieldWidget`) as a widget / in a layout / focus target | GAPS_batchA #3 / #4 | - | `auction/edit_bid_popup.rs` (layout + initial focus), `speedpreset/speed_preset_list_widget.rs` (both inputs), `museum/museum_manager.rs` (search field click / key / char) |
| - | Mod composite widgets (`AbstractContainerWidget` with children) | GAPS_batchB L3, GAPS_batchA #7 / #9 | - | `fancybars/edit_bar_widget.rs` (constructor todo, `children`, `mouseClicked`), `museum/museum_manager.rs` (the sort / filter buttons' onPress cannot run: they are fields, not screen widgets), `fancybars/status_bars_config_screen.rs` (`addWidget(editBarWidget)`) |
| - | The open container screen's hovered slot | GAPS_batchB #2 | - | `debug/debug.rs` dump-hovered-item key |
| - | `ScreenKeyboardEvents.afterKeyPress(screen)` per screen | GAPS_batchB #5 (one global event) | - | `debug/debug.rs` (registered once, checks a container screen is open) |
| - | Fabric `HudElementRegistry.replaceElement` | GAPS_batchC row | - | `fancybars/fancy_status_bars.rs`, `fancybars/vanilla_style_mana_bar.rs` (the `hideIf..` lambdas are typed `Fn(HudElement) -> HudElement` with `HudElement = Arc<dyn Fn(&GuiGraphicsExtractor, f32)>`; nothing consumes them) |
| - | Button subclass draw override (`ScaledTextButtonWidget.extractContents`) | GAPS_batchB L4 | - | `auction/auction_browser_screen.rs` (drawn as a plain Button) |

## Missing mod runtime / libraries (not RustSrc)

| What | Effect | Sites |
|---|---|---|
| **JEI / EMI** (`IGuiHelper`, `IRecipeType`, `EmiRecipeCategory`, ..) - `crate::unresolved` unit structs | the integrations compile but cannot run (constructors are converter todos) | `compatibility/jei/*`, `compatibility/emi/*` |
| **NEU repo file reader** (`NEURepoFile.getFsPath`) | `MuseumItemCache.loadMuseumItems` cannot read the museum JSON (the parse after it is ported) | `museum/museum_item_cache.rs` |
| **commons-lang** `NumberUtils.isParsable`, `WordUtils.capitalizeFully`, `ToStringBuilder` | private copies / compat (`compat::capitalize_fully`) | `chocolate_factory_solver.rs` `is_parsable`, `status_bar.rs` `status_bar_to_string` |
| `utils/ws` `WsMessageHandler.sendLocationMessage(Service, Message<? extends Message<?>>)` typed `&Shared<dyn Message<Shared<dyn Message<Object>>>>` | no message implements that; cross-package (utils/ws) | `chocolatefactory/egg_finder.rs:~258` passes the message as `Shared<dyn Message<EggWaypointMessage>>` (stays an error until the handler is generic over the message type) |
| `config/hud_config_screen.rs` `HudConfigScreen.savePos(SkyblockerConfig, ..)` typed `&SkyblockerConfig` | Java mutates the config | `utils/render/title/title_container_config_screen.rs:258` (E0596 until the trait takes `&mut`) |
| `utils/waypoint` `Waypoint.equals(Object)` takes `compat::Object` (`Send + Sync`) | a non-`Send` waypoint (its type supplier) cannot be passed | `egg_finder.rs` uses a private `waypoint_equals` copy of Java's body |
| `config/skyblocker_config_manager.rs` `get()` is a todo | every config read panics at runtime | everywhere (other batch) |
