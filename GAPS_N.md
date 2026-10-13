# GAPS - batch N (fix-up / burn-down)

Gaps found while closing batch K's files and the step-4 sweep. Each was checked against RustSrc by grepping the function, hook or API. RustSrc was not edited. Sites keep a `// rust: port gap` marker.

| # | Java member / mechanism | What the port has | What is needed | Sites |
|---|---|---|---|---|
| 1 | `PopupScreen` / `AbstractPopupScreen` draws the parent screen under itself | a popup is opened with `game::set_screen_keeping_parent`, but the held parent is never drawn | a `game::` call that draws the held parent | `utils/render/gui/abstract_popup_screen.rs` `extract_background_body`, and every popup |
| 2 | `EditBox.setFilter`, and mod `EditBox` subclass overrides (`insertText`, `deleteCharsToPos`, `setValue`) | the game's EditBox has no filter slot and does not dispatch to the mod object | a filter or override slot | `filtered_edit_box.rs`, `abstract_popup_screen.rs` `EnterConfirmTextFieldWidget` (bodies ported, never dispatched) |
| 3 | `enableScissor` around children that the GAME draws (ContainerWidget content) | scissor applies only to the mod's own draws | a scissored widget pass | `chat_rule_config_screen.rs` ContentContainer: out-of-bounds widgets are hidden, but partly visible ones overflow |
| 4 | `Screen.handleClickEvent` outcomes other than OpenUrl, from a mod screen | only OpenUrl is reached (`confirm_link_now` / `open_uri`) | a general click-event dispatch | `chat_rule_config_screen.rs` `mouse_clicked` |
| 5 | `StringWidget` `TextOverflow.SCROLLING` | no scrolling overflow | the overflow mode | chat rule screens |
| 6 | `PopupScreen.Builder` | `compat::ConfirmScreen` stand-in (GAPS G3) | the builder | `auction_view_screen.rs` `get_confirm_purchase_popup` (signature now returns ConfirmScreen), `status_bars_config_screen.rs` |
| 7 | `setTooltipForNextFrame` from a mod draw | as GAPS_M #15 / E#1 | - | `auction_view_screen.rs` price hover |
| 8 | outlined text render state (`GuiTextRenderState` with outline) | no outline flag | a render-state field | `utils/render/state/gui/*` |
| 9 | world-render submit dispatch (`LevelRenderContext` is unresolved) | primitives are collected but never dispatched | the world-render event | `utils/render/primitive_collector_impl.rs`, `vanilla_renderer.rs` |
| 10 | Sound playback from a mod (`SoundManager.play` of an arbitrary id) | as GAPS_batchH #1 | - | `sound_selection_popup.rs`, `chat_rule_config_screen.rs` preview |
| 11 | NEU repo contacts for `/call` autocomplete | NEU library gap (GAPS_M #10) | the library | `call_autocomplete.rs` `load_call_names` |
| 12 | `clearAndInit` run synchronously from inside a widget press | the press runs in the game's widget pass (not lent), so it is queued and run on the next lent call | none (one-tick delay, documented) | `chat_rule_config_screen.rs` `recreate_requested`, `skyblock_crafting_table_screen.rs` recipe-book toggle |

Library gaps that are still marked and were left alone: NEU repo, networth, Legacy Item DFU, JEI/REI/EMI, Render Chest, java.time temporal, WebSocket client, Discord IPC.
