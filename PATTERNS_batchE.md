# PATTERNS - batch E

Converter mistakes and port shapes seen three or more times in batch E's files, with the fix used.

| # | Pattern | Fix | Sites |
|---|---|---|---|
| 1 | `Color` / `Identifier` / non-`Default` fields initialised with `Default::default()` | the Java initialiser (`Color::from_rgba(..)`, `sky_block_colors::GOLD ..`) | ~22 (`config/configs`) |
| 2 | `T::to_string(*self)` in a `Display` impl of a non-`Copy` enum | `T::to_string(self)` | 20 (also outside batch E: accessories, chocolatefactory, fancybars, gyro_overlay, update_notifications) |
| 3 | enum `toString()` left as `todo!` | `i18n::get(&format!("..{}", self.name()), &[])` / `self.name().to_lowercase()` | ~20 |
| 4 | a game entity held by value (`ArmorStand`, `Entity`) | its id (`i32`) and `game::with_entity` / `compat::with_entity_handle` | ~8 |
| 5 | `a && b \|\| c` from `a && (b \|\| c)` (precedence) | parentheses back | 9 |
| 6 | `layoutSettings.alignX()` / `p.paddingX(n)` as a statement (mutate-copy: the port's setters return a new value) | `*p = p.clone().align_x()` / `*settings = settings.clone()..` | 10+ (also accessories_helper_widget, item_tab, armor_tab, waypoints_list_widget) |
| 7 | an `AbstractContainerWidget` subclass whose widgets other objects toggle (`visible`, `setCurrentItem`) | `compat::ObjectWidget` (the object keeps Java's `visible`/`active` and box; the game's widget keeps the box the layouts move; `setX`/`setWidth` overrides run on a move) + its children in a `Screen` the object owns (`compat::children_*`) | 12 classes (customization screen, shortcut rows, event-time rows, greenhouse rows, powder filter rows) |
| 8 | a child's call-back into its parent while the parent hands it the input (`this::onX`, `list.removeEntry(this)`) | the call is recorded (`ColorEvent`, `TrimPress`, `NameEvent`, `remove_requested`) and run as the parent's call returns | 7 |
| 9 | a layout built once, its widgets resized later (`setWidth` then `arrangeElements`) | the port's `LayoutElement::Widget` keeps the size it was given: the layout is rebuilt from the live sizes before it is arranged (`build_grid`, `build_layout`, `arrange`) | 6 |
| 10 | an inner (non-static) class reading its outer object | a `WeakShared` of the outer (`Arc::new_cyclic` when the outer builds the inner) | 9 |
| 11 | `parent.clone()` of a `Screen` / the parent kept as a field | the game holds the parent (`set_screen_keeping_parent`, `set_screen_to_parent`) | 6 |
| 12 | a popup handed `minecraft.screen` | an empty stand-in `Screen` (batch A GAPS #19) | 5 |
| 13 | `MutableComponent::get_string()` unresolved (two traits define it: `Component`, brigadier `Message`) | `use ..component::Component as _;` in the file (not in the prelude: three files import `Message`) | 6 |
| 14 | `Arc::new(closure)` coerced to `Arc<dyn Fn(..)>` with unannotated parameters | annotate the closure's parameters | 10+ |
| 15 | `x.trim().parse::<i32>()` for `Integer.parseInt(x)` (parse-trim) | no trim (Java throws on surrounding blanks); `Double.parseDouble` and `.strip()` keep theirs | 12 |
| 16 | `GridComponent.of(title, value)` routed to the `(String, Component...)` overload | `GridComponent::of(&vec![title, value])` | 13 |
| 17 | `String.format` as `format!("{:.1}")` (half-even, no width semantics) | `compat::string_format("%.1f", ..)` | 4 |
| 18 | a field read inside a `compat::write` of the same object that also calls out (RefCell re-borrow) | take what is needed out first, call out with no borrow held (`with_screen`, `update_widgets(this)`) | 6 |
| 19 | identity-keyed `HashMap<Shared<..>, ..>` | `Vec` + `Arc::ptr_eq` | 3 |
| 20 | `world.getBlockState` / particle `==` | `get_block_state_typed` / `compat::particle_type_is` | 4 |
