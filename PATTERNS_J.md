# PATTERNS - burn-down batch J

Converter (fabric2seer) shapes seen three or more times in batch J's packages, and the shape that replaced them.

## Silent behaviour changes (they compiled)

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| S1 | **A whole statement or method body dropped into a comment** ("TODO(fabric2seer) ... java: ...") with no `todo!` and no `// rust:` marker - the method silently does nothing | port the statement | 9 | `ProfileViewerScreen.<init>` (the `fetchPlayerData(..).thenRun(initialisePagesAndWidgets)` that loads the whole screen), `ContainerSolverManager.slotMap` (loop body: every solver got an EMPTY map), `ItemRepository.importItemFiles` after-import tasks, `GuiHelper.nineSliceColored(Color)` / `horizontalGradient`, `MuseumItemCache` / `ProfileViewerScreen` / `GreenhousePaste` command trees (nothing was registered), `SkyShardsLayout.decode` / `parsePalette` (methods missing entirely) |
| S2 | `ClientLifecycleEvents.CLIENT_STARTED.register(..)` left as a comment | `hook::inject("net.minecraft.client.Minecraft#<init>", At::Return, ..)` (as `WidgetManager.init`) | 2 | `museum_item_cache.rs`, `greenhouse_paste.rs` |
| S3 | a value a record accessor answers by COPY mutated in place (`data.collected_item_ids().insert(..)`, `matching_upgrade.clone().unwrap().clear()`) | mutate the field and put the record back (`ProfiledData::put`), or index into the live list under its lock | 3 | `MuseumItemCache.handleClick`, `loadMuseumItems` ordered upgrades |
| S4 | CUSTOM_NAME / LORE values set as `MutableComponent` (`FlexibleItemStack::set(.., custom_name(), TextTransformer::from_legacy(..))`) - the port's components hold `FormattedText`, `ItemStack.getCustomName` casts to it and finds nothing | `FormattedText::from(..)` | 3 | `item_stack_builder.rs` (2), `legacy_item_stack_fixer.rs` |
| S5 | Gson's insertion-ordered `asMap()` / `keySet()` collected into `HashMap` / `HashSet` before a `findFirst` or an ordered UI list | keep `serde_json::Map` (preserve_order) / a `Vec` | 4 | `museum_item_cache.rs` `setExceptions` / `children` / `MUSEUM_CATEGORIES`, `getUpgrades` |
| S6 | `Stream.max(comparator)` as `max_by_key` (keeps the LAST of equals; Java's `maxBy` keeps the first) | a fold keeping the first | 2 | `AccessoriesHelper.calculateFamilyReport` |
| S7 | a mod `MainThread` static or `RenderThread` cell reached from a worker closure (`CompletableFuture.runAsync(.., VIRTUAL_THREAD_EXECUTOR)`) - panics at run time | network / parse on the worker, field writes in ONE `game::execute` hop that also completes a `compat::LocalFuture` | 3 | `ProfileViewerScreen.fetchPlayerData` (both halves), `MuseumItemCache.updateData4ProfileMember` |

## Compile-shape placeholders

| # | Wrong shape emitted | Correct shape | Count | Example |
|---|---|---|---|---|
| 1 | a brigadier tree left as a comment ("a command tree this tool cannot read") | `client_command_registration_callback::literal/argument`, `.then(&child)`, `.executes(Arc::new(..))`, `.suggests(Some(provider))` with the provider bound to a `SuggestionProvider<S>` `let` first (an `Arc<closure>` does not unsize inside `Some`), `dispatcher.dispatcher().register(&root)` | 4 trees | `profile_viewer_screen.rs` (`/pv`), `museum_item_cache.rs`, `greenhouse_paste.rs` |
| 2 | `stack.is(Items.X)` on a `FlexibleItemStack` / generic `T extends ItemInstance` | `Arc::ptr_eq(&item_of(&FlexibleItemStack::type_holder(&s)), &items().get("x"))`; for a generic `T`, a local trait over the two `T`s (`LegacyFixableStack`) | 3 | `item_repository.rs`, `item_stack_builder.rs`, `legacy_item_stack_fixer.rs` |
| 3 | `TriConsumer<T, DataComponentType, Object>` / other functional interfaces as `crate::unresolved` placeholders | `&dyn Fn(&mut T, Arc<DataComponentType>, Obj)` | 1 (+ callers) | `legacy_item_stack_fixer.rs` |
| 4 | `getTranslatedKeyMessage()` on a mod `KeyMapping` static whose initialiser is `todo!` | `key_mapping::get_translated_key_message("key.x", &|k, _| minecraft().key_names.get(&k).cloned())`; `KEY.matches(e)` -> `key_mapping::matches("key.x", e)` | 2 | `donation_button.rs`, `museum_manager.rs` |
| 5 | `new Dynamic<>(ops, value)` (two-argument) | `Dynamic::new_with(ops, Some(obj(value)))` | 3 | `item_stack_componentization_fixer.rs` (2), `legacy_item_stack_fixer.rs` |
| 6 | `CycleButton.builder(stringifier, value).withValues(..).create(x, y, w, h, name, listener)` | `cycle_button::builder(label).with_values(labels).with_tooltip(TooltipSupplier::Constant(..))` + `compat::cycle_button_widget(.., move |index| ..)` mapping the index back to the value | 1 (+2 in the gap file) | `farming_hud_widget.rs` |
| 7 | an `AbstractWidget` subclass whose own widgets are fields it draws and clicks (`searchField.extractRenderState(..)`, `button.mouseClicked(..)`) | `compat::ObjectWidget` with the children in a `Screen` the object owns: `context.extract_widgets(&mut children, ..)`, each child's click tested on the child (`widget.mouse_clicked(..)`), a `Button`'s `onPress` body run where Java's `mouseClicked` runs it, the EditBox driven through `compat::with_edit_box` | 1 class (17 placeholders) | `museum/museum_manager.rs` |
