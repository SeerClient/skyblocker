# PATTERNS - batch P

1. **commons-lang3 `NumberUtils`**: `isCreatable(s)` -> `compat::number_utils_is_creatable(&s)`; `toInt(s, d)` -> `compat::number_utils_to_int(&s, d)` (`toInt(s)` is `d = 0`; a null `s` is `""`). `isDigits(s)` -> `!s.is_empty() && s.chars().all(char::is_numeric)` (no compat helper; Java's `Character.isDigit` is Nd only, `is_numeric` also accepts No/Nl - harmless on SkyBlock text). 12 sites. Ex: `slottext/adders/pet_level_adder.rs:123`.
2. **`WordUtils.capitalizeFully(x)`** -> `compat::capitalize_fully(&x)`. 4 sites. Ex: `profileviewer2/pages/collections_page.rs:115`.
3. **`Math.TAU`** -> `std::f64::consts::TAU`, with `i as f64 / segments as f64` (Java's int-to-double promotion). 5 sites. Ex: `primitive/outlined_circle_feature_renderer.rs:74`.
4. **`ObjectLists.singleton(x)`** -> `vec![x]`. 4 sites (`slot_text.rs`).
5. **`Http.sendGetRequest(url)` "not static"** - it is: `crate::de::hysky::skyblocker::utils::http::Http::send_get_request(url)`. 4 sites.
6. **`Codec.unboundedMap(K, V).xmap(Map::new, Function.identity())`** -> `codec::xmap(codec::unbounded_map(k, v), |e: Vec<(K, V)>| e.into_iter().collect::<HashMap<_, _>>(), |m: &HashMap<K, V>| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>())`. An `EnumMap` sorts the encoded entries by key. 3 sites. Ex: `slayers/features/slayer_timer.rs:41`, `dungeon/secrets/dungeon_manager.rs:126`.
7. **A record codec over a `compat::Shared` waypoint** -> a tuple-`Fields` record codec `xmap`ped to the handle (as `NamedWaypoint.CODEC`). `secret_waypoint.rs:43`, `kuudra_waypoints.rs:51`.
8. **`Instant` in a codec** (`Codec.LONG.xmap(Instant::ofEpochMilli, Instant::toEpochMilli)`) and **`Duration.between(a, b).toMillis()`** -> `SystemTime` +/- `Duration::from_millis` and `duration_since` (negative on `Err`). `corpse_loot.rs:25`, `slayer_timer.rs:123`.
9. **`handler.getSlot(i).getItem()` with no `Containers`** -> `compat::slot_item(handler.get_slot(i))`. `visitor_helper.rs`.
10. **`++field % n == 0` on a `Mutex` state** -> `{ let m = compat::get_state(&F) + 1; compat::set_state(&F, m); m % n == 0 }`. `item_tooltip.rs`.
