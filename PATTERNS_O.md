# PATTERNS - batch O

1. **Inner-class outer reference.** Give the inner class a field `outer: compat::WeakShared<Outer>`. Its constructor takes `outer` (Java's `outer.new Inner(..)`), and the outer's code passes `Arc::downgrade(__this)`. When the outer is built by value first, its constructor sets the field after `compat::shared`.
   - Examples: `option_dropdown_widget.rs` Header / Option, `waypoints_list_widget.rs` `AbstractWaypointEntryBase.outer`, `model_selection_popup.rs` Widget (as `item_selection_popup.rs` ItemWidget).
2. **`getEntitiesOfClass(Foo.class, box, pred)`** becomes `level.get_entities_of_class(EntityClass::Entity, except, &box, &|e| e.as_any().is_some_and(|a| a.is::<Foo>()) && pred(e))`.
   - When the caller holds an entity (the player in `with_player`), pass its id as `except`.
   - Read each hit under `game::lock_entity(&**e, site)`.
   - Examples: `creeper_beams.rs`, `ice_path.rs`, `three_weirdos.rs`.
3. **`StringRepresentableArgument` subclass `super(CODEC, X::values)`** becomes `StringRepresentableArgument::new(X::values(), |v| v.get_serialized_name(), |id| id.to_string())`. Type the base as `StringRepresentableArgument<X>`, not `<compat::Object>`.
   - Examples: `waterboard.rs` LeverType, `room.rs` Direction, as `corpse_type.rs`.
4. **`updateConfig(Consumer<Conf>)` mutated a CLONE** (silent bug, mutate-copy). The consumer is `Arc<dyn Fn(&mut Conf)>` and is handed `&mut config.section`.
   - Examples: `other_options_screen.rs`, `waypoints_option_screen.rs`.
5. **`CycleButton.booleanBuilder(..).create(name, (_, v) -> ..)`** becomes `compat::cycle_button_widget(0, 0, 150, 20, &name, cycle_button::boolean_builder(yes, no, init), move |i| { let v = i == 0; .. })`. An enum cycle maps the index back through `X::values()`.
6. **Popup `ScrollableLayout`** becomes `ScrollableLayoutElement::new(LayoutElement::Grid(content.wrapped.clone()), maxHeight, ReserveStrategy::Both)`. After `arrange_elements` / `set_x` / `set_y`, copy `visit_widgets` positions onto the screen's widgets.
   - When a layout holds it by value, read its bounds back via `visit_children` (`copy_to_popup.rs` `scrollable_bounds`).
