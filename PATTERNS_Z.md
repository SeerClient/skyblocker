# PATTERNS - sweep Z

| # | Wrong shape | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | `with_player(\|p\| script_host::send_feedback(&component.to_string()))` - `player.sendSystemMessage(Component)` as text: click / hover events lost | `compat::batch_w_send_feedback(&component)` | 44 | `dwarven/wishing_compass_solver.rs`, `dwarven/corpse_finder.rs` |
| 2 | `// TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet` (listener never registered) | `compat::register_allow_game(\|text, overlay\| X::on_message(text, overlay))` | 8 | `dwarven/metal_detector.rs` |
| 3 | `// TODO(fabric2seer): a command tree this tool cannot read` (command missing) | build the tree with `client_command_registration_callback::{literal, argument}`, `executes` / `then` / `suggests` (import `ArgumentBuilder as _`), `dispatcher.register(&root)` | 7 | `dwarven/profittrackers/powder_mining_tracker.rs` |
| 4 | a widget object in a layout outside a screen (`// rust: layout.add_child(widget)`) in a profile viewer page | `profile_viewer_page::layout_widget(compat::WidgetAdapter::widget(w))` inside the page's `collect_widgets`; the page keeps its widgets in a `Screen` | ~40 | `profileviewer2/pages/skills_page.rs` |
| 5 | Gson reflective `fromJson(json, X.class)` | `X::from_json(&Value)` = `X::new()` + `read_json_fields` (member per `@SerializedName`, default kept when absent) | 129 structs | `profileviewer2/model/*.rs` |
| 6 | `super.onClick/onDrag/onRelease(..)` of `AbstractWidget` left as `// rust:` | the base bodies are empty: a `// java:` note | 6 | `utils/render/gui/color_picker_widget.rs` |
| 7 | `graphics.requestCursor(CursorTypes.X)` left as `// rust:` | `compat::batch_w_request_cursor(compat::BatchWCursor::X)` | 6 | `utils/render/gui/dropdown_widget.rs` |
| 8 | `keyboardHandler.setClipboard/getClipboard` left as `// rust:` | `GlfwClipboard::published()` else a `MemoryClipboard` | 3 | `utils/render/gui/argb_text_input.rs` |
| 9 | an inner-class `Entry` reading `Outer.this` (no outer reference) | a `Weak` handle to a small shared link object the outer owns; a call the row makes INTO the outer while the outer is mid-call (its own `mouseClicked`) is parked on the link and applied when the outer's child pass returns | 1 | `utils/render/gui/dropdown_widget.rs` |
