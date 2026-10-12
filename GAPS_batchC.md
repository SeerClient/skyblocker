# GAPS - batch C (`skyblock/tabhud/`, `skyblock/profileviewer2/`, `skyblock/profileviewer/`)

What the game crate (RustSrc) or the mod runtime does not provide, found while fixing the batch C compile errors.
RustSrc was NOT edited. Every RustSrc row was checked by grepping RustSrc for the member. Sites are marked in the
source with `// rust: port gap (GAPS_batchC #id)` and the Java statement kept as `// rust:` lines; a site that
belongs to another batch's row cites that row (`GAPS_batchA #9`, `GAPS_batchB L1`).

## Missing in RustSrc

| # | Java member | What the port has | What is needed | Sites |
|---|---|---|---|---|
| L3 | A widget OBJECT in a layout or container outside a screen (`LinearLayout.addChild(widget)`, `AbstractContainerWidget` subclasses holding child widgets: `SidePanelWidget`, `TopBarWidget`, `PositionRuleWidget`, `AddWidgetWidget`; profileviewer2 tab buttons / pagination) | `LayoutElement::Widget` is an index into a SCREEN's widget list; `compat::WidgetAdapter` covers leaf widgets only; no adapter for a mod composite widget | a layout element that holds a widget value (or a composite-widget adapter) | 13 marked: `tabhud/config/{widgets_configuration_screen, side_panel_widget (3), top_bar_widget (4), position_rule_widget, list/widgets_element_list}.rs`, `profileviewer2/pages/{collections_page, inventory_page (2), skills_page}.rs`, `profileviewer2/utils/tree/hotm_tree_builder.rs`. Effect: the HUD widget editor's side panel / top bar / add-widget popup are not the screen's listeners (they are fields the screen draws and asks itself) |
| C1 | `Screen.rebuildWidgets()` called from a task (`thenRunAsync(this::rebuildWidgets, minecraft)`) | `Screen::rebuild_widgets(&mut self, build)` needs the game's live `Screen` half, which is lent to a mod screen object only inside its own methods | a way to ask the open mod screen to re-`init` | `profileviewer/profile_viewer_screen.rs` (`initialisePagesAndWidgets`): the page buttons join at the next `init` (resize) instead of when the pages finish loading |
| C2 | `widget.extractRenderState(graphics, ..)` on a widget object from mod code | the port's `AbstractWidget` has no `Renderable` impl; a widget is drawn by the screen that holds it | `Renderable` for `AbstractWidget` (or a draw-this-widget call) | 5: `profileviewer/{profile_viewer_screen, collections/collections_page, inventory/inventory_page, inventory/inventory (2)}.rs`. Visual effect small: the same widgets are also `addRenderableWidget`ed, so the game draws them |
| C4 | `new CopyToPopup(screen, ..)` - an `AbstractPopupScreen` over a MOD screen | `AbstractPopupScreen` (utils, batch A) takes its background as a game `Screen` value; a mod screen object is not one (GAPS_batchA #19) | batch A's popup base taking a background handle | `tabhud/config/side_panel_widget.rs` copy button (the button does nothing) |
| C5 | A mod `AbstractSelectionList` subclass whose rows are mod objects (`AddWidgetWidget` + its `Entry`) | the converter built `AddWidgetWidget` on the plain `AbstractSelectionList` (rows are the game's `Entry` values); mod rows attach only through `ModSelectionList` + `compat::EntryAdapter` | rebuilding the class on `ModSelectionList` (`compat::attach_list`) | `tabhud/config/add_widget_widget.rs` `openWith` (`replaceEntries`) |
| C6 | `ScrollableLayout` as a layout element holding content (`new ScrollableLayout(minecraft, content, maxHeight)`) | the port's `ScrollableLayout` is the scroll helper; the element is `ScrollableLayoutElement` (`LayoutElement::Scrollable`) | the element's public constructor from a mod | `tabhud/config/{copy_to_popup, other_options_screen}.rs` |
| - | `children()` lending live child widgets | (GAPS_batchA #9) | - | `tabhud/config/{side_panel_widget (2), top_bar_widget, position_rule_widget}.rs` answer an empty list |
| - | `AbstractWidget.visible` on a selection list | (GAPS_batchA #8) | - | `tabhud/config/add_widget_widget.rs` `mouseClicked` |
| - | Fabric `HudElementRegistry` (attach before/after a vanilla HUD element) | none; `hud_render_callback` only | an element registry | `tabhud/screenbuilder/widget_manager.rs:188` (both layers draw after the vanilla HUD) |
| - | `minecraft.screen instanceof X` for a screen below the top of the stack | `game::current_screen_is::<T>()` (top only) | - | `tabhud/screenbuilder/widget_manager.rs:205` |
| - | `ModScreen.children()` (a mod screen's own event targets) | the screen half's widgets only | - | `profileviewer2/profile_viewer_screen.rs:185` |

## Missing mod runtime / libraries (not RustSrc)

| What | Effect | Sites |
|---|---|---|
| **NEU repo library** (`Rarity`, `PetNumbers`, `NEUItem`, `Leveling`): `crate::unresolved` unit structs with no `Eq`/`Hash` (GAPS_batchB L1) | pet stats / levelling cannot be looked up | `profileviewer2/utils/pet_loader.rs`, `profileviewer/inventory/pet.rs`, `profileviewer2/utils/level_calculator.rs` (converter todos) |
| **GSON object binding** (`LoadedData`, the profile response as Java objects) | `ProfileViewer.loadData` cannot build the model from JSON | `profileviewer2/profile_viewer.rs:111` |
| **lisp library** (`moe.nea.lisp`) | HOTM / HOTF perk formulas | `profileviewer2/utils/tree/*` (types wrapped in `Arc`, bodies are converter todos) |
| **azureaaron networth** (`NetworthCalculator.getItemNetworth`) | profile viewer networth stays 0 | `profileviewer/profile_viewer_text_widget.rs` `addItemNetworth` (converter todo) |
| `utils/http.rs` `send_get_request` (batch A) | collections / profile fetches | `profileviewer/profile_viewer_screen.rs` `fetchCollectionsData`, `profileviewer2/utils/elite_leaderboards.rs` |
| `utils/` `LegacyItemStackFixer`, `CodecUtils.mutable_optional`, `Location::CODEC`, `RangedSliderWidget` constructor / adapter (batch A) | item fixing, config codecs, the option slider is its base widget only | `profileviewer2/utils/item_loader.rs`, `tabhud/screenbuilder/*`, `tabhud/config/option_widget_collector.rs:203` |
| the mod's ASM widget registration (`hud_injector`) | `WidgetManager.instantiateWidgets` has no body | `tabhud/screenbuilder/widget_manager.rs:445` |
| inner classes reaching their enclosing instance (`TopBarWidget.Layout.getX`, `CustomDropdownWidget.CustomEntry`, `AnchorSelectionWidget`) | converter todos remain (GAPS_batchA "Inner classes") | `tabhud/config/{top_bar_widget, custom_dropdown_widget, position_rule_widget}.rs` |
