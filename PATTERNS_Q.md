# PATTERNS - batch Q

- **A Java library Skyblocker depends on is ported under its own package path** (like hm-api): `src/net/azureaaron/networth/`
  mirrors `net.azureaaron.networth`. Java `interface` constant holders -> module `static`s (`LazyLock<HashMap<&'static str, _>>`
  for `Object2IntMap`/`Map`, `&[i32]` for `IntList`). `Object2IntMap.getInt(k)` / `getOrDefault(k, 0)` -> `*map.get(k).unwrap_or(&0)`.
  `IntList.subList(a, b)` -> `&LIST[a as usize..b as usize]` (panics where Java throws IndexOutOfBounds).
- **`IntIntPair` -> `(i32, i32)`**: `.leftInt()` is `.0`, `.rightInt()` is `.1`.
- **`EnumMap<E, V>` -> `BTreeMap<E, V>`** with `E: Ord` derived in declaration order, so iteration is ordinal order like Java's
  (a `HashMap` iterates randomly - `ValueBreakdownPopup.map`).
- **`Options.keyX.getTranslatedKeyMessage()`** -> `key_mapping::get_translated_key_message("key.x", &|key, _| compat::minecraft().key_names.get(&key).cloned())`
  then `compat::component_of(..)`; a mod `KeyMapping` static is looked up by its name the same way.
- **`TooltipInfoType.X.getData() != null && getData().containsKey(k)`** -> `tooltip_info_type::X.get_mut().lock().borrow().get_data().is_some_and(|d| d.contains_key(&k))`.
- **Calling a trait default from inside a record's `&self` impl** (`appendCountAndPrice(..)` in `BasicSingleAppender.appendTo`):
  wrap `self.clone()` in `compat::shared(..) as Shared<dyn Trait>` and dispatch; acceptable off the per-frame path only.
