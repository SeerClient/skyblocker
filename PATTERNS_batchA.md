# PATTERNS - batch A (`utils/`)

Converter (fabric2seer) mistakes fixed three or more times in `utils/`. Paths are relative to
`src/de/hysky/skyblocker/utils/` unless they say otherwise. Counts are the sites fixed in this batch.
Patterns that only change behaviour, not compilation, are listed separately at the end.

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | `fn x_base_mut(&mut self) -> &mut XBase { &mut self }` in the root class's own impl (E0596) | body `self` | 11 | `render/gui/abstract_popup_screen.rs:150` |
| 2 | record with a convenience constructor plus the canonical one, both emitted as `new` (E0592 / E0061); a delegating `this(..)` left as a TODO over `Default::default()` fields | canonical constructor named `new_canonical`; the convenience one ends in `Self::new_canonical(..)` | 12 | `ws/message/dungeon_bat_killed_message.rs`, `render/text/grid_formatted_char_sequence.rs:35` |
| 3 | `Default::default()` for a field whose type has no `Default` (the converter's "field value in a TODO") | the Java initialiser's value (`Visibility::Show`, a lambda, a computed value) | 8 | `render/gui/basic_toast.rs:49`, `render/gui/cycling_texture_widget.rs:64` |
| 4 | unqualified enum constants inside the enum's own methods (`return TALISMAN_OF_COINS;`) (E0425) | `Self::Variant` / `Type::Variant` | 10 | `purse/purse_change_cause.rs`, `location.rs` |
| 5 | `impl Display { write!(f, "{}", T::to_string(*self)) }` where `to_string` takes `&self` | `T::to_string(self)` | 5 | `chat/chat_filter_result.rs:60`, `location.rs:187` |
| 6 | `.clone()` on a generic `T` / `Option<T>` with no `Clone` bound (E0599) | add `T: Clone` (or `where T: Clone` on the one method), or move the value | 12 | `render/gui/dropdown_widget.rs:191-195`, `data/json_data.rs:116` |
| 7 | a generic class whose type parameter appears only in methods (E0392) | `PhantomData<fn() -> T>` field | 4 | `render/gui/abstract_custom_hypixel_gui.rs:13`, `render/gui/pip/gui_equipment_renderer.rs:13` |
| 8 | a generic trait named without its arguments (`dyn AbstractSelectionPopup`, `<dyn X>::f_from::<Self>`) (E0107) | `dyn AbstractSelectionPopup<W>` | 6 | `render/gui/item_selection_popup.rs:116` |
| 9 | Java `Set<X>` of objects that keep `Object.equals` emitted as `HashSet<compat::Shared<X>>` (no `Hash`/`Eq`) | identity set: `Vec<Shared<X>>` plus `Arc::ptr_eq` membership (insertion order kept, as `LinkedHashSet` keeps it) | 7 | `render/title/title_container.rs:57`, `render/text/grid_component_manager.rs:35` |
| 10 | `super.setX(x)` emitted as `self.x_base().set_x(x)` through `&` (E0596); the `&mut` form would autoderef into the trait's own `set_x` and recurse | `self.x_base_mut().base.set_x(x)` | 6 | `render/gui/dropdown_widget.rs:201` |
| 11 | `&__this.lock().borrow().field[i]` held in a `let` (temporary dropped, E0716) | `compat::read(__this, \|t\| t.field[i].clone())` for a handle or value | 4 | `render/gui/cycling_icon_button_widget.rs:92`, `render/title/title_container_config_screen.rs:85` |
| 12 | `catch (NumberFormatException)` around a `todo!` parse inside `catch_unwind`, or `.expect("NumberFormatException")` | match on the `Result` of `from_str_radix` / `parse` (the `Err` arm is the catch block) | 4 | `render/gui/argb_text_input.rs:96,270`, `datafixer/legacy_string_nbt_reader.rs` |
| 13 | brigadier exception statics typed `MainThread<SimpleCommandExceptionType>` while `::new` returns `Arc` | `LazyLock<Arc<..ExceptionType>>`, message `Arc::new(c) as Arc<dyn Message>` | 5 | `command/argumenttypes/color/rgb_color_argument_type.rs:14` |
| 14 | `do { .. } while (true)` as `loop { ..; if !(true) { break; } }` (the loop's value becomes `()`) | plain `loop { .. }` | 4 | `block_pos_set.rs:99` |
| 15 | `byte[]` fields as `Vec<i8>` where the port's codecs and APIs use `Vec<u8>` | `Vec<u8>` | 3 | `api_authentication.rs:39` |
| 16 | a functional interface placeholder from `crate::unresolved` (`Double2ObjectFunction`, `DoubleConsumer`, `Visibility`, `EditBoxAccessor`, `Class`) | `Arc<dyn Fn(f64) -> MutableComponent>`, `Arc<dyn Fn(f64)>`, the port's `toast::Visibility`, the object itself, `TypeId` | 9 | `render/gui/ranged_slider_widget.rs:21-24` |
| 17 | a builder's fluent setters `fn x(&mut self) -> Builder { ..; self.clone() }` (no `Clone`; a copy where Java returns `this`) | `fn x(mut self) -> Builder { ..; self }` | 9 | `render/gui/ranged_slider_widget.rs:158` |
| 18 | `RenderPipeline` by value where the port hands `Rc<RenderPipeline>` | `Rc<RenderPipeline>` throughout (statics, records, `last_pipeline`); identity via `Rc::ptr_eq` | 8 | `render/primitive/quad_feature_renderer.rs:68`, `render/primitive/primitive_feature_renderer.rs` |
| 19 | `"is the field reachable"` getters on `crate::unresolved` statics as `.get_mut().clone()` (no `Clone`) | return `&'static` | 4 | `utils.rs` `get_environment` / `get_rank`, `networth/networth_data_suppliers.rs` |
| 20 | `Rc` used without `use std::rc::Rc` (`ProblemReporter` is `Rc<dyn ..>`) | import it | 5 | `json_value_input.rs`, `json_value_output.rs` |
| 21 | `compat::Object` substituted for a class's type argument in a cast (`as Shared<dyn FallbackedTexture<compat::Object>>`, `TypeId::of::<X<compat::Object>>()`) | the real argument (`Identifier`, `WidgetSprites`, `S`) | 3 | `render/texture/fallbacked_texture.rs:27,55` |

## Semantic patterns (they compile but behave differently from Java)

| Wrong shape | Correct shape | Count | Example |
|---|---|---|---|
| parentheses dropped in `&&` / `\|\|` mixes: `(a \|\| b) && c` emitted as `a \|\| b && c` | keep the Java grouping | 2 | `command/argumenttypes/color/hex_color_argument_type.rs:49`, `container/container_solver_manager.rs` |
| a mutation applied to a clone taken for iteration (`for s in read(this, \|t\| t.sets.clone()) { s.remove(..) }`) | mutate in place inside `write` | 3 | `exclusive_grouping_set.rs`, `block_pos_set.rs` retainAll / removeAll |
| `std::ptr::eq(&self.a, &self.b)` for Java `a == b` on two fields (always false) | value equality (or `Rc::ptr_eq` on handles) | 2 | `render/texture/identifier_texture.rs` `isUsingFallback` |
| `Integer.parseInt(s)` emitted with `.trim()` | no trim (Java throws on whitespace) | 3 or more | `regex_utils.rs` |
| `Arrays.toString(..)` as `format!("{:?}", vec)` (quotes each string) | `[a, b]` built by joining | 2 | `datafixer/item_stack_componentization_fixer.rs:68`, `logs_folder_finder.rs` |

## Found outside `utils/` (not fixed here; reported for the other owners)

- `GridComponent.of(title, value)` was resolved to the `of(String group, Component...)` overload, with the title passed as `jstring(&title)`. Java's two-argument call is `of(Component...)`, so it should be `of(&vec![title, value])`. About 12 sites in `skyblock/item/tooltip/adders/*`. This is a semantic bug: each line becomes its own group with one column.
- HUD callbacks pass the `ScriptGraphics` session where a `&GuiGraphicsExtractor` is expected. Fixed once in `render/title/title_container.rs:43`; the same shape appears at 6 other sites (`dungeon_score_hud`, `glacite_cold_overlay`, `fancy_status_bars`, `vanilla_style_mana_bar` x2, `widget_manager` x2). The fix is to keep the session alive and pass `&GuiGraphicsExtractor::new()`.
- `Box<dyn NarratableEntry>`: about 25 sites. The port trait is not dyn-compatible (GAPS #2).
