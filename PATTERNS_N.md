# PATTERNS - batch N

1. **Queue the work, then drain it in a lent call.** A widget press or responder runs inside the game's widget pass, where the screen is NOT lent. It sets a flag, an `AtomicBool` or a queue, and the screen's next `tick`/`init`/`container_tick`/mouse call drains it (`recreate_requested`, `pending_removal`, `recipe_book_toggle_requested`, `pending_search`).
2. **A widget field is an index (`Option<usize>`) or a `WidgetRef`, never a copy.** Reach it with `screen.widget_mut(i)`.
3. **A composite widget's children live in a `Screen` it owns.** Draw them with `context.extract_widgets(&mut screen, ..)`. Dispatch with `compat::children_mouse_clicked`, `released`, `dragged`, `key_pressed` and `char_typed`. Examples: searchable grid, sound popup list.
4. **A callback that writes its own Shared object is a static fn taking `__this: &Shared<Self>`, not `&mut self`.** Calling `upgrade()` from inside `&mut self` re-borrows the RefCell and panics. Example: `on_slot_change`.
5. **During dispatch, take the child widget out of its owner (`Option::take`) and put it back afterwards.** Otherwise the child cannot reach the owner. Example: the status-bars edit widget.
6. **`LayoutSettings` is `Copy`, so a setter's result must be written back:** `*s = s.align_x()`. Or use `compat::adjust_layout_settings`.
7. **Slot listeners use `compat::listen_slot_changes(chest, name)`**, which returns a `SlotChangeQueue` that the screen drains in lent calls.
8. **Private copies of shared helpers are deleted and point at compat.** This covers the jvm reflection shim, the NEU file accessors, `local_of`, `run_async_logging` and `no_op`.
9. **A Java `switch (entity)` with type patterns** becomes `entity.as_any().downcast_ref::<T>()` with `if let .. &&` guards, kept in Java's case order (all glow adders).
