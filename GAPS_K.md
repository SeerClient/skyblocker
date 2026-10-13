# Batch K gaps (markers kept on purpose)

Each item says what is missing in the port. These are not things this batch could close inside its own files.

- MapRenderer::extract_decoration_render_state is private, so dungeon_map_texture cannot push decorations.
- networth library: NetworthResult and PetConstants are missing (salvage_helper, pet_cache).
- NEU repo library: NEURepoFile.stream (sack_item_autocomplete) and the NEU id lookup in sack_message_price.
- ModScreen has no mouseMoved slot (radial_menu_screen.mouseMoved). hide() has no port API.
- There is no API to open the vanilla ContainerScreen from a mod screen.
- setTooltipForNextFrame from a ModWidget (same as batch E #1).
- A mod screen cannot draw one of its own widgets from extractRenderState. auction_browser category tabs are now added as renderable widgets, so they draw under the slots, not over them.
- ChestMenu has no public split. Worked around with mem::replace plus containers_for.
- ContainerListener runs inside the player hold. The callbacks are queued and drained in the screen's lent calls, which can delay them by up to 1 tick.
- ALLOW_MOUSE_CLICK lacks the button (wardrobe and loadout keybinds: the mouse half is missing).
- Screens.getWidgets on vanilla container screens (inventory_search showSearchBar).
- java.time.temporal is missing (utils/time SkyblockTimeField: 11 todo).
- java.net.http WebSocket is missing (utils/ws/skyblocker_web_socket).
- SoundManager.play (event_notifications sound; same as batch H #1).
- Integrated server: snapshot_debug get_seed returns 0 when there is no server.
- Debug commands: mixin audit, Biome holder / DIRECT_CODEC (dumpBiome), HudAccessor overlay message (dumpActionBar), ServerPackManager.packs.
- EntityRenderDispatcher has no blockModelResolver (cat_picture item frame).
- EMI/JEI/REI APIs: third-party, left as they are.
- A list object's methods reach the live list only while it is lent (chat_rules_config_list_widget delete_entry works on the detached half).
- sendSystemMessage(Component) is plain text only (D17).
- ClientLifecycleEvents CLIENT_STARTED and ALLOW_GAME are not registered (G L2).
- EditBox.TextFormatter is a closed enum (F).
- CommandUtils.NO_OP is a MainThread static and should be a LazyLock (it is read from command threads).
- auction_browser reset_filters_button is an AbstractWidget copy, so writing `active` to it may not reach the live widget (uncertain).
- radial_menu get_option depends on the config shapes (config merge).
- Not finished (out of budget, not blocked): auction_view_screen, edit_bid_popup, the fancybars files, chat_rule_config_screen (13 todo / 54 rust).
- LayoutSettings mutate-copy sites outside batch K: accessories_helper_widget:132,151; corpse_profit_screen:174; dye_select_popup:217.
