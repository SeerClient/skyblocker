# GAPS - batch E

Things the game crate (RustSrc), the mod runtime or another batch's file does not provide, found while fixing
batch E's packages (`skyblock/item/**`, `config/configs`, `config/screens`, `config/backup`, `config/categories`,
`skyblock/{foraging,shortcut,fishing,hunting,quicknav,radial_menu}`, `utils/scheduler`, `utils/*.rs`, `src/*.rs`).
RustSrc was NOT edited. Each site is marked in the source with `rust: port gap`.

## Missing in RustSrc / the modding API

| # | What Java uses | What the port has | What is needed | Mod sites |
|---|---|---|---|---|
| 1 | `graphics.setTooltipForNextFrame(..)` from a widget's / a row's draw | the frame's `DeferredTooltip` lives in the loop and is not reachable from a `ModWidget` / `ModListEntry` | a `WidgetRenderContext` hook that sets the frame's tooltip | `item/custom/screen/item_tab.rs` (ItemSelector), shortcut rows, event-times rows (4) |
| 2 | `EditBox.addFormatter((string, pos) -> ..)` | `TextFormatter` is a closed enum of the vanilla formatters | a `TextFormatter::Lambda` arm | `config/screens/eventnotifications` `TimeEditBox`, `item/custom/screen/identifier_text_field.rs` (2) |
| 3 | `AbstractWidget.createNarrationMessage` override / a `CreateNarration` supplier | the supplier is data only | a lambda arm | `skyblock/shortcut/keybind_widget.rs` (1) |
| 4 | `BeehiveBlock.MAX_HONEY_LEVELS` | not a public constant | the constant | `foraging` honeyhive (1) |
| 5 | `ClientChunkEvents.CHUNK_LOAD` / `CHUNK_UNLOAD` | no such events | the two events | `foraging/abstract_block_highlighter.rs`, `sea_lumies.rs` (2) |
| 6 | `RecipeBookComponent` as a widget (`addWidget`, `setInitialFocus`, `slotClicked`) | not a `ModWidget`; no `slot_clicked` | the component as a screen child | `item/skyblock_crafting_table_screen.rs` (4) |
| 7 | `MultiLineTextWidget.setComponentClickHandler` | not ported | the setter | `item/value_breakdown_popup.rs` (1) |
| 8 | log4j `LoggerContext` / file appender | the game logs without log4j | - (the fallback arm is taken) | `utils/logs_folder_finder.rs` (1) |
| 9 | `ExtraCodecs.JSON`, `MapCodec.assumeMapUnsafe` | not ported | the two codecs | `utils/codec_utils.rs` (2, `todo!` kept) |
| 10 | gzip `Content-Encoding` | no gzip decoder | a decoder (the request asks for `identity`) | `utils/http.rs` (1) |
| 11 | `KeyMapping.ALL` with mod key mappings | the conflict list holds the vanilla mappings only | mod mappings in the list | `skyblock/shortcut` keybind conflict check (1) |
| 12 | `playDownSound(SoundManager)` / `SimpleSoundInstance.forUI` from mod code | a mod has no sound manager; the screen's click sweep plays only for the screen's own widgets | a `game::play_ui_sound` | children of mod container widgets (`compat::children_*`), `CustomizeScreen.select_tab` (tab click), `KeybindWidget` key press (≈3) |
| 13 | `Screen.mouseClicked`'s code AFTER `super.mouseClicked(..)` | `ModScreen::mouse_clicked` runs BEFORE the generic pass | an after-pass hook | `CustomizeScreen.mouseClicked` (`setFocused(null)` decided from the child under the cursor), `ShortcutsConfigScreen` (2) |
| 14 | the background screen of a popup (`new XPopup(minecraft.screen, ..)`) | the port's `Screen` is a husk; the live screen cannot be handed on | (batch A's GAPS #19) - an empty stand-in is passed | `ColorSelectionWidget` dye popup, `ItemTab` item / model popups, `CustomizeNameWidget` colour popups (5) |
| 15 | anonymous `RemotePlayer` subclass (`isInvisibleTo`, `onEquipItem`, `isModelPartShown`, `getId`) and `Minecraft.getGameProfile()` | entity classes take no mod subclass; no `getGameProfile` | entity subclassing | `item/custom/screen/armor_tab.rs` (1) |
| 16 | `Minecraft.getEntityModels()` (for `ArmorModelSet.bake`) | not lent to mods | the accessor | `item/custom/screen/trim_element_button.rs` (the trimmed armour piece is not drawn) (1) |
| 17 | IME: `preeditUpdated` / `Minecraft.onTextInputFocusChange` for a mod widget | no IME hook for mod widgets | the hook | `item/custom/screen/name/customize_name_widget.rs` (2) |
| 18 | `ScreenEvents.remove(screen)`, `ScreenKeyboardEvents.afterKeyRelease(screen)` | only `AFTER_INIT` / `AFTER_KEY_PRESS` exist | the two events | `item/slottext/slot_text_manager.rs` (2; `keyHeld` is never reset, so "hold to show/hide" sticks) |
| 19 | `ZoneId.of("UTC")`, ICU `DateTimePatternGenerator` | `ZoneId` is the system default only; no locale data | `ZoneId.of`, a locale's time pattern | `utils/formatters.rs` `getTimeZone` (test environment), `is12HourClock` (Linux / other OS) (2) |
| 20 | `DateTimeFormatter.withZone(..)` / `ofPattern(.., Locale)` | a pattern only | - (the zone is applied by the caller, as `ObtainedDateTooltip` does) | `utils/formatters.rs` `DATE_FORMATTER` (1) |
| 21 | `Gui.setScreen(parent)` running the parent's `added()` | the pop to a held parent runs `init(w, h)` (reposition) only | `added()` on pop | `CustomizeScreen.added` - the timeline gradient is not re-created after a popup closes (1) |
| 22 | `NumberFormat.getCompactNumberInstance`, `DecimalFormat.setPositivePrefix`, `DecimalFormat.parse` | `DecimalFormat` is `#0.,` patterns + `format` only | those three | `utils/formatters.rs` `SHORT_*_NUMBERS`, `DIFF_NUMBERS`, `parseNumber` (`todo!` kept) (5) |
| 23 | `OnPress::SelectTab` for a mod screen's `MenuTabBar` | `ScreenAction::SelectTab` is `Navigation::Stay` for a mod screen | - (the buttons' presses are replaced by lambdas that run the tab switch) | `CustomizeScreen` (handled, no loss) |

## Blocked on another batch's file

| # | What | Owner file | Effect on batch E |
|---|---|---|---|
| B1 | `SearchableGridWidget` constructor, `children()`, `mouseScrolled` are `todo!` | `utils/render/gui/searchable_grid_widget.rs` (batch A) | `HeadSelectionWidget::new` panics, so `ArmorTab` (and the whole `CustomizeScreen`) cannot open |
| B2 | `AbstractPopupScreenBase::new_in`'s `backgroundScreen` field is `todo!` | `utils/render/gui/abstract_popup_screen.rs` (batch A) | every popup of `item/custom/screen` panics when opened (`DyeSelectPopup`, `ItemSelectPopup`, `ColorPopup`; their own constructors are now written) |
| B3 | `AbstractSelectionPopupBase` / `ModelSelectionPopup.Widget` constructor | `utils/render/gui/abstract_selection_popup.rs` (batch A) | `ModelSelectionPopup::new` keeps its `todo!` |
| B4 | `GuiHelper.equipment` is `todo!` | `utils/render/gui_helper.rs` | trim pattern buttons draw no armour piece (also gap 16) |
| B5 | `ColorPickerWidget.setARGBColor` is `todo!` (`Color.RGBtoHSB`) | `utils/render/gui/color_picker_widget.rs` | the colour widgets panic when a colour is set |
| B6 | `ConfigUtils.FORMATTING_FORMATTER` is `todo!` | `config/config_utils.rs` | `CustomizeNameWidget` (its formatting / colour buttons' tooltips) panics |
| B7 | `GridComponent.of` is `todo!` (batch A GAPS #1) | `utils/render/text/grid_component.rs` | every price tooltip adder (the overload fix below now calls `GridComponent::of`) |

## Mod-wide

- `serde_json` without the `preserve_order` feature: Gson keeps insertion order, `serde_json::Map` sorts keys
  (config backups, every JSON round trip). Add `features = ["preserve_order"]` to the crate's `Cargo.toml`
  (not a batch E file).
