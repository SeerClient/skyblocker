# PATTERNS - batch C (converter defects seen 3+ times in tabhud / profileviewer2 / profileviewer)

Each is a fix the translator could make mechanically. Counts are sites fixed in this batch.

1. **Java array of nullable elements read as never-null** (`new ProfileViewerPage[5]`, `GenericCategory[]`,
   `inventorySubPages`): the converter types the array `Option<Vec<T>>`, fills it with `Default::default()` and
   writes every `arr[i] != null` as `true`. Right shape: `Vec<Option<T>>` filled with `None`. ~12 sites
   (pv1 screen, CollectionsPage, InventoryPage, their `getButtons`).
2. **`Collection` (a mod record) read as `java.util.Collection`**: `Map<String, List<Collection>>` became
   `HashMap<String, Vec<Vec<_>>>` and every `collection.id()` a TODO. 5 signatures (pv1 `Collection.parse`,
   `ProfileViewerScreen`, `GenericCategory`).
3. **Temporary borrow of a `Shared` field held across a statement** (`let x = &__this.lock().borrow().list[i];`):
   E0716. Right shape: bind the guard (`let g = __this.lock(); let r = g.borrow(); &r.list[i]`) when nothing in the
   scope reaches the object mutably, else `compat::read` a clone. 12 sites.
4. **`.0` / `.1` on an `Option<(A, B)>` field** (Java `Pair` field assigned later): `.as_ref().expect("NullPointerException").0`. 6 sites.
5. **`handleCursor(graphics)` called from `extractWidgetRenderState`**: the port asks each widget for its cursor
   after the frame; the right port is a `ModWidget::handle_cursor` override returning `widget.handle_cursor(cursors)`.
   3 sites (PageTabWidget, ProfileViewerNavButton, SubPageSelectButton).
6. **`setX` / `setWidth` on a mod widget's own fields while iterating them** (`for w in self.ws.iter_mut() { w.set_width(self.get_width()) }`) and setters typed `&self`: E0502/E0596. 6 sites.
7. **`String.format` with a non-literal or `%g` format left as `format!("%.4gB")`**: `compat::string_format`. 4 sites
   (`profile_viewer_utils` x3, `dungeon_floor_runs_widget`).
8. **`Component.append` result dropped** (Java mutates, the port's `append(self)` answers a new value): 9 sites
   (`widgets_list_screen` x4, `player_list_manager` x3, `jacobs_contest_widget`, `minion_widget`).
9. **`ItemStack.set(component, value)` / `FlexibleItemStack.set` with the Java value type**: CUSTOM_NAME / LORE values must be
   `FormattedText` (`FormattedText::from(..)`, `ItemLore::new(Vec<FormattedText>)`), and `ItemStack::set` takes
   `Some(obj(value))`. ~12 sites.
10. **`Integer.parseInt(s)` written `s.trim().parse()`** (Java does not trim): 8 sites.
11. **`ScreenDirection.values()` / `.ordinal()`, `new Matrix3x2f()`, `getRectangle()` on a `LayoutElement` implementor**:
    TODOs whose port is one line (`[Up, Down, Left, Right]`, `matrix3x2f::IDENTITY`, `ScreenRectangle::new_at(x, y, w, h)`).
12. **A lambda stored in an object's own widget capturing `self`** (`PressLambda::new(move |..| self.config_screen ..)`):
    must capture a `RenderThread<Weak<..>>` of the handle. 6 sites (side panel, top bar, position rule widget).
13. **Overload confusion between a class's private method and an inherited one of the same name**: `Screen.addWidget(listener)`
    turned into `WidgetsConfigurationScreen.addWidget(HudWidget)`; `extractIconAndText(graphics, y, x, h)` vs the
    base's 6-argument one (E0034 x4); `new PositionRule(Optional, ..)` routed to the `String` constructor (x4).
