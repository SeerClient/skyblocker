# GAPS - batch I (cross-package cleanup)

Game / platform gaps met while fixing the finished batches' files. Each was checked against `RustSrc` first.

| # | Java member | What the port has | What is needed | Sites |
|---|---|---|---|---|
| I1 | `NumberFormat.getNumberInstance(Locale)` + `DecimalFormat.parse(String)` | `java/text/number_format.rs` has `getIntegerInstance` only; `DecimalFormat` has `new` + `format` (`parse` is listed as library surface not ported) | `parse` (longest-prefix number, grouping, exponent, `ParseException` at index 0) | `skyblock/chest_value.rs` (the JDK body reproduced as the private `number_instance_us_parse`); `utils/formatters.rs` `parseNumber` (batch E, compact-number parse); `accessories/newyearcakes` (batch G, noted in GAPS_batchG #8) |
| I2 | `Component.hashCode()` as the JVM's `int` | `MutableComponent: Hash` (a Rust hash that agrees with `==`; its header says Java's int is not reproduced) | Java's `MutableComponent.hashCode:112-117` over contents / style / siblings | `skyblock/dungeon/secrets/secret_waypoint.rs` `hashCode` - the value is SENT to other players (`DungeonRoomHideWaypointMessage.waypointHash`) and matched against their Java `hashCode()`. Reproduced exactly for the shape a secret waypoint's name has (`Component.nullToEmpty(name)`: literal, `Style.EMPTY`, no siblings); any other component falls back to the port's hash (`// rust: port gap`) |
| I3 | `Object.toString()` of a mod object held as a trait object (`getClass().getName()`) | no runtime Java class name for a `dyn HudWidget` / `dyn NamedWaypoint` | the concrete class's Java name | `tabhud/screenbuilder/positioned_widget.rs` `@GenToString` (the widget's internal id stands in, as `ElementBasedWidget`'s log already does); `utils/waypoint/waypoint_group.rs` `@GenToString` (the three `utils.waypoint` classes are told apart by `dyn_any`; any other subclass prints as `NamedWaypoint`) |
| I4 | `Enum.hashCode()` (identity hash) | - | not reproducible by any port (the JVM's own number) | `WaypointGroup.hashCode` (`island`, `waypointType`), `ChatRule.hashCode` (`validLocations`) - the port's value stands in, consistent with `equals` |
| I5 | `ClientPacketListener.send(Packet)` from mod code | no send path a mod reaches (`// rust:` lines across the tree) | a packet-send API | blocks the two `local_player_mixin.rs` `TODO(batchD)` sites (Search Overlay / EditBidPopup open over the sign editor): even with the sign handed over, the overlay could never write the sign back (`SearchOverManager.pushSign`, `EditBidPopup`), so the vanilla editor is left to open |
| I6 | `Gson` (default, HTML-safe) `toJson` | serde_json | - | `utils/ws/ws_message_handler.rs` `send`: serde_json writes the same document; Gson's `<`-style escapes of `< > & = '` are the only spelling difference, and they decode to the same JSON |
| I7 | `@GenToString` / `@GenEquals` / `@GenHashCode` in `utils/test/ObjectMethodGenerationTestUtils` | the class is not in the crate (no `utils/test` package was converted) | nothing - it is a fixture only `src/test` (`ObjectMethodGenerationTest`) uses | - |

## Not gaps (verified)

- `ImageButton` for `SideTabButtonWidget`: the port's `ImageButton` is `AbstractWidget` of `WidgetKind::ImageButton(WidgetSprites)`; the constructor, its sprites and `RecipeBookTabButton.SPRITES` (`recipe_book_tab_button::tab_sprite` / `tab_selected_sprite`) all exist - ported.
- `StringRepresentable.fromEnum` (`Waypoint.Type.CODEC`): `string_representable::from_enum(values, serialized_name)` - ported.
- `HitResult` returned by `BlockGetter.clip` (`TenacityTestHelper.raycast`): the port's polymorphic hit result is `hit_result::AnyHitResult` - used.
