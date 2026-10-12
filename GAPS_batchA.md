# GAPS - batch A (`utils/`)

Things the game crate (RustSrc) or the mod runtime does not provide, found while fixing the
`utils/` compile errors. RustSrc was NOT edited. Each site is marked in the source with
`rust: port gap` (and, where the Java method has no other body, `todo!("rust: port gap - ..")`).

## Missing or private in RustSrc

| # | What Java uses | What the port has | Mod sites |
|---|---|---|---|
| 1 | `ComponentContents` as an open interface (a mod record `implements ComponentContents`) | `ComponentContents` is a closed enum of the 7 vanilla contents types | `render/text/grid_component.rs` `GridComponent.of` (todo). **High impact:** every price/info tooltip adder in `skyblock/item/tooltip/adders/*` calls `GridComponent.of`, so it panics when a tooltip is built |
| 2 | `NarratableEntry` as a list element type (`List<? extends NarratableEntry>`) | `NarratableEntry::get_narratables -> Vec<&Self>` has no `where Self: Sized`, so `dyn NarratableEntry` is not dyn-compatible | `dropdown_widget.rs` narratables (typed `Box<dyn NarrationSupplier>` instead). About 25 `Box<dyn NarratableEntry>` sites in the mod hit the same wall |
| 3 | `EditBox.keyPressed(KeyEvent)` from a mod subclass | `key_pressed(splitter, event, focused, active, &mut dyn ClipboardAccess)`; the focus and active state live in the game's widget tree, and the only clipboard implementation is `GlfwClipboard` (needs `Glfw` + a window) | `abstract_popup_screen.rs` `EnterConfirmTextFieldWidget.keyPressed` (todo) |
| 4 | `EditBoxAccessor` (`getHighlightPos`, `getMaxLength`, `invokeOnValueChange`) | `EditBox.highlight_pos` and `on_value_change` are private | `filtered_edit_box.rs` `insertText` / `deleteCharsToPos` (the converter's TODOs remain) |
| 5 | `BlockEntityRenderState.blockState = ..` from a mod | the field is private; only `extract_base` writes it | `primitive_collector_impl.rs` beacon state |
| 6 | `AbstractContainerScreenAccessor.setHandler(menu)` | the port's `AbstractContainerScreen` has no `menu` field | `abstract_custom_hypixel_gui.rs` `changeHandler` |
| 7 | `AbstractContainerWidget.mouseClicked` as `super` | takes the scrollbar pass (`updateScrolling`) and the children pass as closures from its caller | `dropdown_widget.rs` `mouseClicked` (todo) |
| 8 | `AbstractWidget.visible` on a selection list (`DropdownList`) | `AbstractSelectionList` / `ModSelectionList` hold no `visible` | `dropdown_widget.rs` DropdownList mouse handlers (4, todo) |
| 9 | `children()` lending live child widgets (`List.of(searchField, container)`) | `children()` answers owned `Box<dyn GuiEventListener>`; a mod container widget's children sit inside its lock | `searchable_grid_widget.rs`, `dropdown_widget.rs` (todo) |
| 10 | `LayoutElement` holding an arbitrary widget object | `LayoutElement::Widget` is an index into a SCREEN's widget list | `searchable_grid_widget.rs` (`layoutWidget.addChild(searchField / widgetsContainer)`) |
| 11 | `List<AbstractWidget>` holding a subclass (`SoundWidget extends StringWidget`) | the port's `AbstractWidget` is a concrete struct with no subclass dispatch | `sound_selection_popup.rs` `filterSounds` (pushes the widget's `base`; its overrides do not travel) |
| 12 | `ScrollableLayout.removeChildren` | no port on `ScrollableLayoutElement` | `toggleable_layout_widget.rs` `removeChildren` |
| 13 | `Minecraft.getAtlasManager()` | no accessor for the live `AtlasManager` | `identifier_texture.rs` `update` (converter TODO remains) |
| 14 | `Toast.getToken()` answering any `Object` | `Token` is a closed enum (`NoToken`, `SystemToastId`) | `basic_toast.rs` token typed as `Token` |
| 15 | `SystemToastId(long displayTime)` | `SystemToastId::new()` takes no display time | `data/json_data.rs` `ERROR_TOAST_ID` (10 000 ms) |
| 16 | `VanillaRegistries.createLookup()` | no counterpart (it is data generation) | `registry_utils.rs` `LOOKUP` |
| 17 | `KeyPairFuture.thenAcceptAsync` | not ported | `api_authentication.rs` `updateToken` (polls on a worker) |
| 18 | HTTP response headers `CF-Cache-Status`, `Local-Cache-Status`, `Age`, `Content-Encoding`, `Etag`, `Last-Modified` | the port's http client returns 4 fixed headers | `http.rs` |
| 19 | `Screen` objects reachable from a mod (`instanceof` a mod interface; keeping the background screen) | the port's `Screen` is a husk with no `as_any`, and the object around it cannot be reached | `hovered_item_stack_utils.rs` `getProvider` (returns None); `abstract_popup_screen.rs` / `abstract_selection_popup.rs` constructors (todo) |
| 20 | `DataComponentGetter` downcast (`stack instanceof ItemStack`) | the port trait has no `as_any` | `item_utils.rs` `getPetInfo` |

## Missing mod runtime or libraries (not RustSrc)

- **YACL / Dandelion** config libraries: `config/duration_controller.rs` `controller` and `provideWidget` are todo.
- **Fabric `ResourceLoader` reload listeners**: `render/texture/fallbacked_texture.rs` `init` (the `UPDATE_CALLBACKS` list now holds every texture's `update`, but nothing runs it on a resource reload).
- **Fabric `PictureInPictureRendererRegistry`**: `render/gui/pip/gui_equipment_renderer.rs` `init`.
- **Multithreaded scheduler tasks**: `Arc<dyn Fn()>` over render-thread objects is not `Send`, so multithreaded tasks run on the render thread.
- **Async `JsonData` load/save**: the object and its codec are not `Send`, so these run synchronously.
- **Inner (non-static) classes with an enclosing-instance handle**: `AbstractSelectionPopup.ItemList`, `SoundSelectionPopup.ListContainer` / `SoundWidget`, `DropdownWidget.DropdownList` / `Entry`. They read the outer object's fields and call its methods, and the converted classes have no handle to it. `AbstractSelectionPopup.init` is a todo because of this.
- **Generic `T extends Enum<T>`**: `EnumUtils.cycle` / `cycleBackwards` are still the converter's todos. I added a `JavaEnum` trait to `enum_utils.rs` (name / ordinal / values) and implemented it for `FilterOption`. Implementing it for every converted enum is the translator's job, and then `cycle` can be written on it.
- **log4j, the JDK `WebSocket`, `java.time` `Temporal`, `ClassDesc`**: unresolved placeholders remain wherever the converter left them.
