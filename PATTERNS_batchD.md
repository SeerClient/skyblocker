# PATTERNS - batch D (`mixins/`, `skyblock/dwarven/`, `skyblock/waypoint/`)

These converter defects repeated three or more times in the batch. For each one, the list gives the form the fix took.

1. **`this` in a mixin handler.** The converter wrote `this_mut()` / `self` for Java's `this` inside the injector.
   - The fix: `ci.this::<C>()`.
   - At a call site, `ci.this` is the call's RECEIVER, not the method's owner. The caller's object comes from a HEAD
     injector on the caller, recorded in a static (`SLOT_CLICKED_SCREEN_TITLE`, `PLAYER_BEFORE_TELEPORT_BLOCK_POS`).

2. **By-value arguments read at RETURN.** The hook moves a by-value argument into the call, so it is gone by RETURN.
   - Guards that read such arguments after the call were removed.
   - The value is now read at HEAD, or from the return value (`take_return_value`).
   - Arguments that carry a lifetime use `compat::hook_arg` / `hook_arg_mut`.

3. **Lowercase block names.** The converter wrote `blocks::blocks().get("chest")`. `BlocksTable` keys are the
   UPPERCASE Java constant names.
   - Every batch site was uppercased.
   - This affects 91 sites mod-wide; any site outside the batch panics at runtime.

4. **Identity sets turned into `HashSet<Arc<..>>`.** Java's `Set<Block>`, `ReferenceOpenHashSet` and `HashSet` of a
   mod object do not compile as `HashSet<Arc<..>>`, because the shared handle has no `Hash`/`Eq`.
   - The fix: a `Vec` asked with `Arc::ptr_eq` (`contains_block`, `contains_group`) where Java compares by identity.
   - Where Java's `equals` is structural, the `Vec` is asked with that equality instead (`named_waypoint_equals`,
     `waypoint_group_equals`).

5. **Dropped `append`.** `MutableComponent.append` returns the new component, and the converter discarded it.
   - The fix: `x = x.append(..)` (`call_mismyla`, `redial_on_bad_signal`, `crystals_locations_manager` ×6).

6. **`Display` written as `to_string(*self)`.** The fix: `Enum::to_string(self)`.

7. **`Default::default()` for fields with no `Default`.** This hit `TriState`, `SimpleRegression`, `MutableComponent`
   keys, `BlockState` arrays and `TileGrid` cells.
   - The fix: the Java initial value.
   - For a Java array that starts full of nulls and is written before it is read, the fix is a placeholder that is
     never observed (`TileGrid::of_size`, `PickobulusHelper.BLOCKS`).

8. **`NumberUtils.toInt` / `Integer.parseInt` / `parseInt(..).trim()`.**
   - `Integer.parseInt` does not trim: the silent-bug rows were fixed by parsing the raw text, with Java's
     `NumberFormatException` message.
   - `NumberUtils.toInt(s, d)` became `parse().unwrap_or(d)`.

9. **`HitResult` casts.** The fix: match on `AnyHitResult::Block(..)`.

10. **`ParticleTypes.X.equals(type)`.** `getType()` is erased, so the fix is `options.get_type().java_equals(&**X())`
    (identity). The forms are `compat::particle_type_is` and a local `particle_type_is` for typed `ParticleType`s.

11. **Mutating a copy.** Getters that answer a copy were mutated in place:
    - `WaypointGroup::waypoints(..)`, `ProfiledData::compute_if_absent`, `HashMap::get(..).cloned()` followed by
      `entry(..)`, and a cloned `GridLayout` handed to a layout before its rows were added.
    - The fix: write through the owner (`compat::write(&group, |g| g.waypoints...)`), put the copy back
      (`ProfiledData::put`), or add the grid once it is filled.

12. **Precedence.** Java's `a && (b || c)` was flattened to `a && b || c`.
    - Fixed in `powder_mining_tracker:209`, `fairy_souls:197`, `abstract_waypoints_screen:311`, `corpse_finder:146`
      and the mixins.

13. **`children()` / `narratables()` typed `Vec<Box<dyn GuiEventListener>>`.** The port's widgets implement neither
    trait, and the game reaches a row's widgets through `ModListEntry::children`.
    - The fix: these answer the widgets (or their `WidgetRef`s).
    - Row widgets now join the row's `__row` list through `set_children` (`reward_list.rs`), so the game draws and
      clicks them.

14. **Mod selection lists placed in a layout as values.** The fix follows the converter's established pattern:
    `WidgetRef::value(compat::attach_list(..))`, then `__widgets.add_to_layout`, and the list's own methods through
    `lend_list`.
