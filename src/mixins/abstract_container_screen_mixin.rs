//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.AbstractContainerScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): state `GENERIC_CONTAINER_TEXTURE` of type `Identifier` - give it a Rust type (`static GENERIC_CONTAINER_TEXTURE: LazyLock<Mutex<T>>`)
pub static OUT_OF_BOUNDS_SLOT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-999));
// TODO(fabric2seer): state `FILLER_ITEMS` of type `Set<String>` - give it a Rust type (`static FILLER_ITEMS: LazyLock<Mutex<T>>`)

/// @Unique
fn skyblocker_modify_display_stack(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, slot: &mut mcrust::net::minecraft::world::inventory::slot::Slot, stack: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack, solver: /* TODO: ContainerSolver */ ()) -> /* TODO: ItemStack */ () {
    // TODO(fabric2seer): `.isSolverSlot(..)` on a ContainerSolver
    //   java: if (solver instanceof StackDisplayModifier modifier && solver.isSolverSlot(slot, this)) {
    //   java: 			return modifier.modifyDisplayStack(slot.getContainerSlot(), stack);
    //   java: 		}
    return stack;
}

/// @Inject skyblocker$keyPressed
///   java: @Inject(at = @At("HEAD"), method = "keyPressed")
/// target: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#keyPressed  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_container_screen.rs:2185 fn key_pressed(&mut self, host: &mut dyn ContainerScreenHost, event: &crate::net::minecraft::client::input::key_event::KeyEvent)
fn skyblocker_key_pressed(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>()) else { return; };
    let Some(input) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::client::input::key_event::KeyEvent>(1)) else { return; };
    let mut hovered_slot: mcrust::net::minecraft::world::inventory::slot::Slot = this.hovered_slot;
    // TODO(fabric2seer): `Options.keyInventory` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
    if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && true /* java: hovered_slot != null: never null here */ && !input.is_escape() && !(compat::minecraft().options.borrow().key_inventory.matches(&input)) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut config: crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        mcrust::mcrust::modding::game::with_player(|player| { crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager::handle_wiki_lookup_4(Some(this.screen.get_title().to_string()), &mcrust::com::mojang::datafixers::util::either::Either::left(hovered_slot.clone()), player, &input); });
        if crate::de::hysky::skyblocker::skyblock::item::item_protection::ITEM_PROTECTION.get_mut().as_ref().unwrap().matches(&input) {
            // TODO(fabric2seer): `consume_click` takes 1 argument(s) in the port (`name: &str`), Java passes 0
            // rust: mcrust::net::minecraft::client::key_mapping::consume_click();
            // TODO(fabric2seer): the mod keeps the game's `Inventory` (Java holds a reference to the live object); the port's `Inventory` is not a value to copy and the port lends the mod no handle to it
            // rust: let mut own_item: bool = (std::ptr::eq(&hovered_slot.container, &mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().clone()).unwrap()) || crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_personal_storage(&this.screen.get_title().to_string()));
            let mut own_item: bool = todo!("fabric2seer: the initialiser of `ownItem` (TODO above)");
            if own_item {
                // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
                // rust: crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::handle_key_pressed(hovered_slot.get_item());
            }
        }
        if config.helpers.item_price.enable_item_price_lookup && crate::de::hysky::skyblocker::skyblock::item::item_price::ITEM_PRICE_LOOKUP.get_mut().matches(&input) {
            // TODO(fabric2seer): unknown name `minecraft`
            //   java: ItemPrice.itemPriceLookup(minecraft.player, hoveredSlot);
        }
    }
}

/// @ModifyExpressionValue skyblocker$passThroughSearchFieldUnfocusedClicks
///   java: @ModifyExpressionValue(method = "mouseClicked", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z"))
/// target: net.minecraft.client.gui.screens.Screen#mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#mouseClicked
/// note: not a method of that class in JavaSrc: `mouseClicked` is not declared in net.minecraft.client.gui.screens.Screen; it is inherited from net.minecraft.client.gui.components.events.ContainerEventHandler - hook `net.minecraft.client.gui.components.events.ContainerEventHandler#mouseClicked` (it runs for every subclass) and test `ci.getThis()`'s type
fn skyblocker_pass_through_search_field_unfocused_clicks(v: bool) -> bool {
    let mut super_clicked = v;
    if !super_clicked {
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Optional<AbstractWidget> searchField = Screens.getWidgets(this).stream()
        //   java: 					.filter(InventorySearch.SearchTextFieldWidget.class::isInstance)
        //   java: 					.findFirst();
        let mut search_field: Option<mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget> = todo!("fabric2seer: the initialiser of `searchField` (TODO above)");
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        // TODO(fabric2seer): the condition: `click`: this argument is not visible to the injector (see the notes above the handler)
        //   java: searchField.isPresent() && searchField.get().mouseClicked(click, doubled)
        if todo!("fabric2seer: the condition (TODO above)") {
            return true;
        }
    }
    return super_clicked;
}

/// @Inject skyblocker$beforeTooltipExtracted
///   java: @Inject(method = "extractTooltip", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_container_screen.rs:3577 fn extract_tooltip(state: &AbstractContainerScreen, menu: &AbstractContainerMenu, containers: &Containers<'_>, carried: &crate::net::minecraft::world::item::item_stack::ItemStack)
fn skyblocker_before_tooltip_extracted(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>(0)) else { return; };
    // `this`: this port's extractTooltip is a free function; its `state` argument (index 0) is the AbstractContainerScreen
    // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
    //   java: ContainerSolverManager.onExtract(graphics, (AbstractContainerScreen<?>) (Object) this, this.menu.slots);
}

/// @WrapOperation skyblocker$extractMouseOverTooltip
///   java: @SuppressWarnings("DataFlowIssue")
///   java: @WrapOperation(method = "extractTooltip", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:6570 fn set_tooltip_for_next_frame_with_image(tooltip_for_next_frame: &mut Option<DeferredTooltip>, texts: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>, optional_image: Option< Box<dyn crate::net::minecraft::world::inventory::tooltip::tooltip_component::TooltipComponent>, >)
fn skyblocker_extract_mouse_over_tooltip(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    // TODO(fabric2seer): `textRenderer` is Rust argument 1: `Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(1) }`
    // TODO(fabric2seer): `text` is Rust argument 2: `Option< Box<dyn crate::net::minecraft::world::inventory::tooltip::tooltip_component::TooltipComponent>, >` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(2) }`
    // TODO(fabric2seer): the condition: `stack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: ContainerSolverManager.getCurrentSolver() instanceof StackDisplayModifier && stack.isEmpty()
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    if !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()) || text.is_empty() {
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: original.call(graphics, textRenderer, text, data, x, y, texture);
        return;
    }
    // TODO(fabric2seer): `.getString(..)` on a value of unknown type
    //   java: var name = text.getFirst().getString();
    // TODO(fabric2seer): the condition: `.isBlank(..)` on a value of unknown type
    //   java: SkyblockerConfigManager.get().uiAndVisuals.hideEmptyTooltips && name.isBlank()
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    let mut shift_down: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.backpack_preview_without_shift ^ crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper::has_shift_down());
    // TODO(fabric2seer): the condition: call to `getTitle(..)` (not a method of this class)
    //   java: shiftDown && getTitle().getString().equals("Storage") && hoveredSlot.container != minecraft.player.getInventory() && BackpackPreview.extractPreview(graphics, this, hoveredSlot.getContainerSlot(), x, y)
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.compactor_deletor_preview {
        // TODO(fabric2seer): `stack`: this argument is not visible to the injector (see the notes above the handler)
        //   java: Matcher matcher = CompactorDeletorPreview.NAME.matcher(stack.getSkyblockId());
        let mut matcher: compat::regex::Matcher = todo!("fabric2seer: the initialiser of `matcher` (TODO above)");
        // TODO(fabric2seer): the condition: `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: matcher.matches() && CompactorDeletorPreview.extractPreview(graphics, stack, getTooltipFromContainerItem(stack), matcher.group("type"), matcher.group("size"), x, y)
        if todo!("fabric2seer: the condition (TODO above)") {
            return;
        }
    }
    // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
    //   java: original.call(graphics, textRenderer, text, data, x, y, texture);
}

/// @ModifyVariable skyblocker$modifyTooltipDisplayStack
///   java: @ModifyVariable(method = "extractTooltip", at = @At(value = "STORE"), name = "item")
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
fn skyblocker_modify_tooltip_display_stack(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyVariable is not expressible - the handler is kept for reference
    //   java: @ModifyVariable(method = "extractTooltip", at = @At(value = "STORE"), name = "item")
    //   java: 	private ItemStack skyblocker$modifyTooltipDisplayStack(ItemStack stack) {
    //   java: 		// hoveredSlot can not be null here.
    //   java: 		return skyblocker$modifyDisplayStack(hoveredSlot, stack, ContainerSolverManager.getCurrentSolver());
    //   java: 	}
}

/// @ModifyArg skyblocker$modifyDisplayStackFake
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;fakeItem(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#fakeItem(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4294 fn fake_item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_modify_display_stack_fake(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    let mut stack = v.clone();
    // TODO(fabric2seer): unknown name `slot`
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @ModifyArg skyblocker$modifyDisplayStackReal
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;item(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4323 fn item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_modify_display_stack_real(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    let mut stack = v.clone();
    // TODO(fabric2seer): unknown name `slot`
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @ModifyArg skyblocker$modifyDisplayStackDecorations
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:178 fn item_decorations_with_count(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, count_override: Option<&str>, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
fn skyblocker_modify_display_stack_decorations(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    let mut stack = v.clone();
    // TODO(fabric2seer): unknown name `slot`
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @Inject skyblocker$onSlotClick
///   java: @Inject(method = "slotClicked(Lnet/minecraft/world/inventory/Slot;IILnet/minecraft/world/inventory/ContainerInput;)V", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;handleContainerInput(IIILnet/minecraft/world/inventory/ContainerInput;Lnet/minecraft/world/entity/player/Player;)V"), cancellable = true)
/// target: net.minecraft.client.multiplayer.MultiPlayerGameMode#handleContainerInput  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#slotClicked
/// Rust: net/minecraft/client/multiplayer/multi_player_game_mode.rs:1964 fn handle_container_input(&mut self, container_id: i32, slot_num: i32, button_num: i32, container_input: crate::net::minecraft::world::inventory::container_input::ContainerInput, menu: &mut crate::net::minecraft::world::entity::player::player::ContainerMenu, inventory_menu: &mut crate::net::minecraf
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_slot_click(ci: &mut CallbackInfo<'_>) {
    if !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()) {
        return;
    }
    // TODO(fabric2seer): the condition: `slotId`: this argument is not visible to the injector (see the notes above the handler)
    //   java: slotId == OUT_OF_BOUNDS_SLOT && ItemProtection.isItemProtected(this.menu.getCarried())
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
        return;
    }
    // TODO(fabric2seer): the condition: `slot`: this argument is not visible to the injector (see the notes above the handler)
    //   java: slot == null
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    // TODO(fabric2seer): call to `getTitle(..)` (not a method of this class)
    //   java: String title = getTitle().getString();
    let mut title: String = todo!("fabric2seer: the initialiser of `title` (TODO above)");
    let mut current_solver: compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver> = crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::get_current_solver().unwrap();
    // TODO(fabric2seer): `slot`: this argument is not visible to the injector (see the notes above the handler)
    //   java: ItemStack stack = skyblocker$modifyDisplayStack(slot, slot.getItem(), currentSolver);
    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
    let mut is_title_empty_or_filler: bool = compat::get_state(&FILLER_ITEMS).contains(&<_ as mcrust::net::minecraft::network::chat::component::Component>::get_string(&compat::component_of(stack.get_hover_name())));
    if is_title_empty_or_filler {
        // TODO(fabric2seer): field `EMPTY` of a value of unknown type
        //   java: var tooltip = stack.getTooltipLines(Item.TooltipContext.EMPTY, Minecraft.getInstance().player, TooltipFlag.NORMAL).stream().map(Component::getString).toList();
        let mut lore: String = [compat::jstring(&tooltip)].join("\n");
        // TODO(fabric2seer): `.getFirst(..)` on a value of unknown type
        //   java: isTitleEmptyOrFiller = lore.isBlank() || FILLER_ITEMS.contains(tooltip.getFirst());
    }
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_empty_tooltips && is_title_empty_or_filler && !({ let __r = crate::de::hysky::skyblocker::skyblock::experiment::ultrasequencer_solver::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.test_string()); __f(&compat::view(__r, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher>), &title) }) || crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().helpers.experiments.enable_ultrasequencer_solver {
        ci.cancel();
        return;
    }
    // TODO(fabric2seer): the condition: `containerInput`: this argument is not visible to the injector (see the notes above the handler)
    //   java: containerInput == ContainerInput.THROW && ItemProtection.isItemProtected(stack)
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
        return;
    }
    if title == "Salvage Items" && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        ci.cancel();
        return;
    }
    if title.starts_with("You  ") && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        ci.cancel();
        return;
    }
    let mut is_in_auction_gui: bool = (title.ends_with("Auction House") || title == "Create Auction" || title == "Create BIN Auction");
    if is_in_auction_gui && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        ci.cancel();
        return;
    }
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons() && title == "Chest" && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        ci.cancel();
        return;
    }
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: this.menu instanceof ChestMenu && ItemProtection.isItemProtected(stack)
    //   java: 				&& !ItemProtection.isNpcSellButton(slot) && ItemProtection.isNpcSellMenu(this.menu)
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
        return;
    }
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: switch (this.menu) {
    //   java: 			case ChestMenu genericContainerScreenHandler when genericContainerScreenHandler.getRowCount() == 6 -> {
    //   java: 				VisitorHelper.onSlotClick(slot, slotId, title, genericContainerScreenHandler.getSlot(13));
    //   java: 			}
    //   java:
    //   java: 			case ChestMenu genericContainerScreenHandler when title.equals(MuseumItemCache.DONATION_CONFIRMATION_SCREEN_TITLE) -> //Museum Item Cache donation tracking
    //   java: 					MuseumItemCache.handleClick(slot, slotId, genericContainerScreenHandler.slots);
    //   java:
    //   java: 			default -> {}
    //   java: 		}
    // TODO(fabric2seer): the condition: `button`: this argument is not visible to the injector (see the notes above the handler)
    //   java: button == InputConstants.MOUSE_BUTTON_LEFT && title.startsWith("Pets")
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `slot`: this argument is not visible to the injector (see the notes above the handler)
        //   java: PetCache.handlePetEquip(slot, slotId);
    }
    if true /* java: current_solver != null: never null here */ {
        // TODO(fabric2seer): `slotId`: this argument is not visible to the injector (see the notes above the handler)
        //   java: boolean disallowed = ContainerSolverManager.onSlotClick(slotId, stack, button);
        let mut disallowed: bool = todo!("fabric2seer: the initialiser of `disallowed` (TODO above)");
        if disallowed {
            ci.cancel();
        }
    }
    // TODO(fabric2seer): `slot`: this argument is not visible to the injector (see the notes above the handler)
    //   java: BazaarMax.INSTANCE.checkMaxValue(slot);
}

/// @Inject skyblocker$drawOnItem
///   java: @Inject(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;item(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4323 fn item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_draw_on_item(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // TODO(fabric2seer): `slot`: this argument is not visible to the injector (see the notes above the handler)
        //   java: ItemBackgroundManager.drawBackgrounds(slot.getItem(), graphics, slot.x, slot.y);
    }
    // TODO(fabric2seer): the condition: `slot`: this argument is not visible to the injector (see the notes above the handler)
    //   java: ItemProtection.isItemProtected(..)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: ItemProtection.drawSlotIcon(graphics, slot.x, slot.y);
    }
    // TODO(fabric2seer): the condition: `slot`: this argument is not visible to the injector (see the notes above the handler)
    //   java: InventorySearch.isSearching() && !InventorySearch.slotMatches(slot)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: graphics.fill(slot.x, slot.y, slot.x + 16, slot.y + 16, 0xB0_000000);
    }
}

/// @Inject skyblocker$drawSlotText
///   java: @Inject(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:178 fn item_decorations_with_count(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, count_override: Option<&str>, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_draw_slot_text(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: SlotTextManager.extractSlotText(graphics, font, slot);
    }
}

/// @WrapWithCondition skyblocker$hideChestName
///   java: @WrapWithCondition(method = "extractLabels", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V", ordinal = 0))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractLabels
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:469 fn draw_formatted_text_with_drop_shadow(gl: &Gl, gfx: &Minecraft, text: &crate::net::minecraft::network::chat::formatted_text::FormattedText, x: i32, y: i32, color: i32, drop_shadow: bool)
fn skyblocker_hide_chest_name(ci: &CallbackInfo<'_>) -> bool {
    let x: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let y: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    let colour: i32 = ci.arg::<i32>(5).copied().unwrap_or_default();
    let shadow: bool = ci.arg::<bool>(6).copied().unwrap_or_default();
    return !(compat::get_state(&crate::de::hysky::skyblocker::skyblock::chest_value::HIDE_CHEST_NAME_LABEL));
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#keyPressed", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_key_pressed)));
    // TODO(fabric2seer): not registered - not a method of that class in JavaSrc: `mouseClicked` is not declared in net.minecraft.client.gui.screens.Screen; it is inherited from net.minecraft.client.gui.components.events.ContainerEventHandler - hook `net.minecraft.client.gui.components.events.ContainerEventHandler#mouseClicked` (it runs for every subclass) and test `ci.getThis()`'s type
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#mouseClicked`, `net.minecraft.client.gui.screens.Screen#mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z`).priority(1000).modify_expression_value::<bool>(skyblocker_pass_through_search_field_unfocused_clicks));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_before_tooltip_extracted)));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip", "net.minecraft.client.gui.GuiGraphicsExtractor#setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V").priority(1000).wrap_operation(skyblocker_extract_mouse_over_tooltip));
    // TODO(fabric2seer): @ModifyVariable skyblocker$modifyTooltipDisplayStack: nothing to register - @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot`, `net.minecraft.client.gui.GuiGraphicsExtractor#fakeItem(Lnet/minecraft/world/item/ItemStack;III)V`).priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(1, skyblocker_modify_display_stack_fake));
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot`, `net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V`).priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(1, skyblocker_modify_display_stack_real));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V").priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(2, skyblocker_modify_display_stack_decorations));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#slotClicked", "net.minecraft.client.multiplayer.MultiPlayerGameMode#handleContainerInput").priority(1000).inject(Shift::Before, skyblocker_on_slot_click));
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot`, `net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V`).priority(1000).inject(Shift::Before, skyblocker_draw_on_item));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V").priority(1000).inject(Shift::Before, skyblocker_draw_slot_text));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractLabels", "net.minecraft.client.gui.GuiGraphicsExtractor#text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V").ordinal(0).priority(1000).wrap_with_condition(skyblocker_hide_chest_name));
}
