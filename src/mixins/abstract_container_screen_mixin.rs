//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.AbstractContainerScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private static final Identifier GENERIC_CONTAINER_TEXTURE
pub static GENERIC_CONTAINER_TEXTURE: LazyLock<mcrust::net::minecraft::resources::identifier::Identifier> = LazyLock::new(|| mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("textures/gui/container/generic_54.png"));
pub static OUT_OF_BOUNDS_SLOT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-999));
/// java: @Unique private static final Set<String> FILLER_ITEMS = Set.of(..)
pub static FILLER_ITEMS: LazyLock<std::collections::HashSet<&'static str>> = LazyLock::new(|| {
    std::collections::HashSet::from([
        "", // Empty menu item
        "Locked Page",
        "Quick Crafting Slot",
        "Locked Backpack Slot 2", //Regular expressions won't be utilized here since the search by contains is based on plain text rather than regex syntax
        "Locked Backpack Slot 3",
        "Locked Backpack Slot 4",
        "Locked Backpack Slot 5",
        "Locked Backpack Slot 6",
        "Locked Backpack Slot 7",
        "Locked Backpack Slot 8",
        "Locked Backpack Slot 9",
        "Locked Backpack Slot 10",
        "Locked Backpack Slot 11",
        "Locked Backpack Slot 12",
        "Locked Backpack Slot 13",
        "Locked Backpack Slot 14",
        "Locked Backpack Slot 15",
        "Locked Backpack Slot 16",
        "Locked Backpack Slot 17",
        "Locked Backpack Slot 18",
        "Preparing",
    ])
});

/// `solver instanceof StackDisplayModifier modifier`: the classes that implement it (`ColorTerminal`,
/// `StartsWithTerminal`, and `ExperimentSolver`'s three subclasses), each seen as the interface.
fn as_stack_display_modifier(
    solver: &compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver>,
) -> Option<compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier>> {
    use crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier;
    fn of<C: StackDisplayModifier + 'static>(
        solver: &compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver>,
    ) -> Option<compat::Shared<dyn StackDisplayModifier>> {
        compat::read(solver, |s| s.dyn_any().is::<C>()).then(|| compat::handle_as::<C, _>(solver) as compat::Shared<dyn StackDisplayModifier>)
    }
    of::<crate::de::hysky::skyblocker::skyblock::dungeon::terminal::color_terminal::ColorTerminal>(solver)
        .or_else(|| of::<crate::de::hysky::skyblocker::skyblock::dungeon::terminal::starts_with_terminal::StartsWithTerminal>(solver))
        .or_else(|| of::<crate::de::hysky::skyblocker::skyblock::experiment::chronomatron_solver::ChronomatronSolver>(solver))
        .or_else(|| of::<crate::de::hysky::skyblocker::skyblock::experiment::superpairs_solver::SuperpairsSolver>(solver))
        .or_else(|| of::<crate::de::hysky::skyblocker::skyblock::experiment::ultrasequencer_solver::UltrasequencerSolver>(solver))
}

/// @Unique
fn skyblocker_modify_display_stack(this: &mcrust::net::minecraft::client::gui::screens::screen::Screen, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, stack: mcrust::net::minecraft::world::item::item_stack::ItemStack, solver: Option<compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver>>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    if let Some(solver) = &solver
        && let Some(modifier) = as_stack_display_modifier(solver)
        && { let __f = compat::dispatch(solver, |__t| __t.is_solver_slot()); __f(solver, slot, this) }
    {
        let __f = compat::dispatch(&modifier, |__t| __t.modify_display_stack());
        return __f(&modifier, slot.get_container_slot(), &stack);
    }
    return stack;
}

/// @Inject skyblocker$keyPressed
///   java: @Inject(at = @At("HEAD"), method = "keyPressed")
/// target: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#keyPressed  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_container_screen.rs:2185 fn key_pressed(&mut self, host: &mut dyn ContainerScreenHost, event: &crate::net::minecraft::client::input::key_event::KeyEvent)
fn skyblocker_key_pressed(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>()) else { return; };
    let Some(input) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::client::input::key_event::KeyEvent>(1)) else { return; };
    // `Slot hoveredSlot = this.hoveredSlot`: the port's `hoveredSlot` is the slot's index in `this.menu.slots`; the
    // slot itself is read from the menu (`this.menu` is `player.containerMenu`, and at HEAD the host has not taken
    // the player yet).
    let hovered_slot: Option<mcrust::net::minecraft::world::inventory::slot::Slot> = this.hovered_slot.and_then(|index| {
        compat::with_open_menu(|menu, _| (index >= 0).then(|| menu.slots.get(index as usize).cloned()).flatten()).flatten()
    });
    // `this.minecraft.options.keyInventory.matches(input)` - the binding by name (`key_mapping::matches`).
    if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && let Some(hovered_slot) = hovered_slot && !input.is_escape() && !(mcrust::net::minecraft::client::key_mapping::matches("key.inventory", &input)) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let config: &crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        let title: String = this.screen.get_title().to_string();
        // Wiki lookup
        mcrust::mcrust::modding::game::with_player(|player| { crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager::handle_wiki_lookup_4(Some(title.clone()), &mcrust::com::mojang::datafixers::util::either::Either::left(hovered_slot.clone()), player, &input); });
        //item protection
        if let Some(item_protection) = crate::de::hysky::skyblocker::skyblock::item::item_protection::ITEM_PROTECTION.get().as_ref() && item_protection.matches(&input) {
            mcrust::net::minecraft::client::key_mapping::consume_click(item_protection.get_name());
            // `hoveredSlot.container == this.minecraft.player.getInventory()`: the slot's container tag.
            let mut own_item: bool = (hovered_slot.container == mcrust::net::minecraft::world::inventory::slot::SlotContainer::Inventory || crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_personal_storage(&title));
            if own_item {
                crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::handle_key_pressed(&compat::slot_item(&hovered_slot));
            }
        }
        //Item Price Lookup
        if config.helpers.item_price.enable_item_price_lookup && crate::de::hysky::skyblocker::skyblock::item::item_price::ITEM_PRICE_LOOKUP.get().matches(&input) {
            mcrust::mcrust::modding::game::with_player(|player| crate::de::hysky::skyblocker::skyblock::item::item_price::ItemPrice::item_price_lookup(player, &hovered_slot));
        }
    }
}

/// @ModifyExpressionValue skyblocker$passThroughSearchFieldUnfocusedClicks
///   java: @ModifyExpressionValue(method = "mouseClicked", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z"))
/// target: net.minecraft.client.gui.screens.Screen#mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#mouseClicked
/// note: not a method of that class in JavaSrc: `mouseClicked` is not declared in net.minecraft.client.gui.screens.Screen; it is inherited from net.minecraft.client.gui.components.events.ContainerEventHandler - hook `net.minecraft.client.gui.components.events.ContainerEventHandler#mouseClicked` (it runs for every subclass) and test `ci.getThis()`'s type
fn skyblocker_pass_through_search_field_unfocused_clicks(v: bool) -> bool {
    let super_clicked = v;
    if !super_clicked {
        //   java: Optional<AbstractWidget> searchField = Screens.getWidgets(this).stream()
        //   java: 					.filter(InventorySearch.SearchTextFieldWidget.class::isInstance)
        //   java: 					.findFirst();
        //   java: if (searchField.isPresent() && searchField.get().mouseClicked(click, doubled)) return true;
        // rust: port gap - the port's `AbstractContainerScreen.mouseClicked` decides `super.mouseClicked(..)` (the
        // widget pass) outside the hooked body, so there is no call whose answer this could modify, and the handler
        // is handed neither `this` nor `click` (GAPS_M #7). InventorySearch's `SearchTextFieldWidget` is not added to
        // the screen either (inventory_search.rs:99), so there is no search field to pass the click to; not
        // registered.
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
    // and `this.menu` its `menu` argument (index 1); `graphics` is the frame's extractor (the port's draws go through
    // the frame's global state).
    let Some(menu) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>(1)) else { return; };
    // `onExtract`'s first statement returns with no solver; the port's takes the slot list by value, so it is not
    // copied for nothing.
    if crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::get_current_solver().is_none() {
        return;
    }
    //   java: ContainerSolverManager.onExtract(graphics, (AbstractContainerScreen<?>) (Object) this, this.menu.slots);
    crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::on_extract(&mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new(), this, menu.slots.clone());
}

/// @WrapOperation skyblocker$extractMouseOverTooltip
///   java: @SuppressWarnings("DataFlowIssue")
///   java: @WrapOperation(method = "extractTooltip", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:6570 fn set_tooltip_for_next_frame_with_image(tooltip_for_next_frame: &mut Option<DeferredTooltip>, texts: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>, optional_image: Option< Box<dyn crate::net::minecraft::world::inventory::tooltip::tooltip_component::TooltipComponent>, >)
/// rust: port gap - the port's `extractTooltip` stops at `:169-171` and answers the stack; `:172-174`'s
/// `setTooltipForNextFrame` runs in `extract_render_state_tail`, which is not a hooked method, so this wrapper never
/// sees a call made inside `extractTooltip` (GAPS_batchD). Its body is the Java's, over what the call hands it:
/// `stack` is the item `extractTooltip` answered (recorded by `skyblocker_modify_tooltip_display_stack`).
fn skyblocker_extract_mouse_over_tooltip(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    let stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::get_state(&TOOLTIP_ITEM).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
    // Hide tooltips from items that have been visually replaced by a container solver with air (since the Slot#hasStack still passes)
    if crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::get_current_solver().as_ref().and_then(as_stack_display_modifier).is_some() && stack.is_empty() {
        return;
    }
    // `List<Component> text` is the call's `texts` (argument 1).
    let Some(text) = compat::hook_arg::<Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>>(ci, 1) else {
        original.call(ci);
        return;
    };
    if !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()) || text.is_empty() {
        original.call(ci);
        return;
    }
    let name: String = text[0].get_string();
    // Hide Empty Tooltips
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_empty_tooltips && compat::is_blank(&name) {
        return;
    }
    // Backpack Preview
    let mut shift_down: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.backpack_preview_without_shift ^ crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper::has_shift_down());
    // rust: port gap - `BackpackPreview.extractPreview(graphics, this, hoveredSlot.getContainerSlot(), x, y)` and
    // `CompactorDeletorPreview.extractPreview(graphics, ..)` draw into the call's `GuiGraphicsExtractor`, and the
    // port's `setTooltipForNextFrame` takes the deferred-tooltip slot instead of one (GAPS_batchD); neither preview
    // can be drawn from here, so the vanilla tooltip is kept.
    let _ = shift_down;
    original.call(ci);
}

/// @ModifyVariable skyblocker$modifyTooltipDisplayStack
///   java: @ModifyVariable(method = "extractTooltip", at = @At(value = "STORE"), name = "item")
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
/// The stack `extractTooltip` answered this frame (`@Local(name = "item")` of the tooltip wrapper above).
static TOOLTIP_ITEM: LazyLock<Mutex<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>>> = LazyLock::new(|| Mutex::new(None));

/// `@ModifyVariable(at = STORE, name = "item")`: the port's `extractTooltip` (`:169-171`) ANSWERS `item` rather
/// than storing it, so the store is its return value - modified here at RETURN, with `hoveredSlot` read from the
/// screen it is handed (`state`, argument 0) and the slot from `menu` (argument 1).
fn skyblocker_modify_tooltip_display_stack(ci: &mut CallbackInfo<'_>) {
    let Some(state) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>(0)) else { return; };
    let Some(menu) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>(1)) else { return; };
    let Some(Some(stack)) = ci.take_return_value::<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>>() else {
        ci.set_return_value::<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>>(None);
        compat::set_state(&TOOLTIP_ITEM, None);
        return;
    };
    // hoveredSlot can not be null here.
    let stack = match state.hovered_slot.and_then(|index| (index >= 0).then(|| menu.slots.get(index as usize)).flatten()) {
        Some(hovered_slot) => skyblocker_modify_display_stack(&state.screen, hovered_slot, stack, crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::get_current_solver()),
        None => stack,
    };
    compat::set_state(&TOOLTIP_ITEM, Some(stack.duplicate()));
    ci.set_return_value(Some(stack));
}

/// @ModifyArg skyblocker$modifyDisplayStackFake
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;fakeItem(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#fakeItem(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4294 fn fake_item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_modify_display_stack_fake(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    // rust: port gap - `@Local(name = "slot") Slot slot` is `extractSlot`'s own local, and the port's
    // `extract_slot` draws the item through `hud::extract_item` and `item_decorations_with_count(.., &ItemStack, ..)`,
    // whose stack is a borrow `@ModifyArg` cannot replace (GAPS_batchD): the stack is handed on unchanged.
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    v.duplicate()
}

/// @ModifyArg skyblocker$modifyDisplayStackReal
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;item(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4323 fn item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_modify_display_stack_real(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    // rust: port gap - `@Local(name = "slot") Slot slot` is `extractSlot`'s own local, and the port's
    // `extract_slot` draws the item through `hud::extract_item` and `item_decorations_with_count(.., &ItemStack, ..)`,
    // whose stack is a borrow `@ModifyArg` cannot replace (GAPS_batchD): the stack is handed on unchanged.
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    v.duplicate()
}

/// @ModifyArg skyblocker$modifyDisplayStackDecorations
///   java: @ModifyArg(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:178 fn item_decorations_with_count(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, count_override: Option<&str>, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
fn skyblocker_modify_display_stack_decorations(v: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    // rust: port gap - `@Local(name = "slot") Slot slot` is `extractSlot`'s own local, and the port's
    // `extract_slot` draws the item through `hud::extract_item` and `item_decorations_with_count(.., &ItemStack, ..)`,
    // whose stack is a borrow `@ModifyArg` cannot replace (GAPS_batchD): the stack is handed on unchanged.
    //   java: return skyblocker$modifyDisplayStack(slot, stack, ContainerSolverManager.getCurrentSolver());
    v.duplicate()
}

/// @Inject skyblocker$onSlotClick
///   java: @Inject(method = "slotClicked(Lnet/minecraft/world/inventory/Slot;IILnet/minecraft/world/inventory/ContainerInput;)V", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;handleContainerInput(IIILnet/minecraft/world/inventory/ContainerInput;Lnet/minecraft/world/entity/player/Player;)V"), cancellable = true)
/// target: net.minecraft.client.multiplayer.MultiPlayerGameMode#handleContainerInput  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#slotClicked
/// Rust: net/minecraft/client/multiplayer/multi_player_game_mode.rs:1964 fn handle_container_input(&mut self, container_id: i32, slot_num: i32, button_num: i32, container_input: crate::net::minecraft::world::inventory::container_input::ContainerInput, menu: &mut crate::net::minecraft::world::entity::player::player::ContainerMenu, inventory_menu: &mut crate::net::minecraf
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// `this.getTitle()` of the screen whose `slotClicked` is running, recorded at its HEAD
/// (`skyblocker_record_slot_clicked_screen`): the call-site injector below sees the `handleContainerInput` CALL's
/// arguments, not `slotClicked`'s receiver.
static SLOT_CLICKED_SCREEN_TITLE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

/// Not a Java injector: `skyblocker$onSlotClick` reads `getTitle()`, and this is how its call-site handler gets it.
fn skyblocker_record_slot_clicked_screen(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>()) else { return; };
    compat::set_state(&SLOT_CLICKED_SCREEN_TITLE, this.screen.get_title().to_string());
}

/// `ci.cancel()` of this `@Inject`: the injection point is `slotClicked:470`, its LAST statement, so skipping the
/// `handleContainerInput` call is cancelling the rest of `slotClicked`. The call answers the click packet; skipped,
/// it sends none.
fn cancel_slot_click(ci: &mut CallbackInfo<'_>) {
    ci.set_return_value::<Option<mcrust::net::minecraft::network::protocol::game::serverbound_container_click_packet::ServerboundContainerClickPacket>>(None);
}

fn skyblocker_on_slot_click(ci: &mut CallbackInfo<'_>) {
    if !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()) {
        return;
    }
    // `slotClicked(Slot slot, int slotId, int button, ContainerInput containerInput)` reach the call as its
    // `slot_num` (`:465-467`: `slot.index` when the slot is not null), `button_num` and `container_input`, and
    // `this.menu` as its `menu` - all of the call's own arguments.
    let slot_id: i32 = ci.arg::<i32>(1).copied().unwrap_or_default();
    let button: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    let Some(container_input) = ci.arg::<mcrust::net::minecraft::world::inventory::container_input::ContainerInput>(3).copied() else { return; };
    let Some(menu) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::world::entity::player::player::ContainerMenu>(4)) else { return; };
    let Some(inventory_menu) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::world::inventory::inventory_menu::InventoryMenu>(5)) else { return; };
    let Some(inventory) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::world::entity::player::inventory::Inventory>(9)) else { return; };
    let is_chest_menu = matches!(menu, mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(_));
    let chest_row_count: Option<i32> = match &*menu {
        mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(chest) => Some(chest.get_row_count()),
        _ => None,
    };
    let (this_menu, containers) = menu.slots_and_containers(inventory_menu, inventory);
    // Item Protection
    // When you try and drop the item by picking it up then clicking outside the screen
    if slot_id == compat::get_state(&OUT_OF_BOUNDS_SLOT) && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(this_menu.get_carried().clone())) {
        cancel_slot_click(ci);
        return;
    }
    // `slot == null`: a click with no slot reaches the call with a negative `slotId`.
    let Some(slot) = (slot_id >= 0).then(|| this_menu.slots.get(slot_id as usize)).flatten() else {
        return;
    };
    let mut title: String = compat::get_state(&SLOT_CLICKED_SCREEN_TITLE);
    let mut current_solver: Option<compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver>> = crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::get_current_solver();
    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = {
        // `isSolverSlot(slot, this)`: the solvers read only the menu (`with_chest_menu`), never this `Screen` (the port's is the
        // title-and-widgets husk), so the screen handed on is a husk with the clicked screen's title.
        let screen = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.clone());
        skyblocker_modify_display_stack(&screen, slot, slot.get_item(&containers).duplicate(), current_solver.clone())
    };
    let mut is_title_empty_or_filler: bool = FILLER_ITEMS.contains(stack.get_hover_name().get_string().as_str());
    if is_title_empty_or_filler {
        // `Minecraft.getInstance().player`: the game holds the player for this click, so it is `null` to a mod here.
        let tooltip: Vec<String> = mcrust::mcrust::modding::game::with_player(|player| stack.get_tooltip_lines(&mcrust::net::minecraft::world::item::item::TooltipContext::empty(), Some(&*player as &dyn mcrust::net::minecraft::world::entity::player::player::Player), mcrust::net::minecraft::world::item::tooltip_flag::NORMAL))
            .unwrap_or_else(|| stack.get_tooltip_lines(&mcrust::net::minecraft::world::item::item::TooltipContext::empty(), None, mcrust::net::minecraft::world::item::tooltip_flag::NORMAL))
            .iter()
            .map(|line| line.get_string())
            .collect();
        let mut lore: String = tooltip.join("\n");
        is_title_empty_or_filler = compat::is_blank(&lore) || FILLER_ITEMS.contains(tooltip.first().expect("NoSuchElementException").as_str());
    }
    // Prevent clicks on filler items
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_empty_tooltips && is_title_empty_or_filler && (!({ let __r = crate::de::hysky::skyblocker::skyblock::experiment::ultrasequencer_solver::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.test_string()); __f(&compat::view(__r, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher>), &title) }) || crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().helpers.experiments.enable_ultrasequencer_solver) {
        cancel_slot_click(ci);
        return;
    }
    // Item Protection
    // When you click your drop key while hovering over an item
    if container_input == mcrust::net::minecraft::world::inventory::container_input::ContainerInput::Throw && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        cancel_slot_click(ci);
        return;
    }
    if title == "Salvage Items" && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        cancel_slot_click(ci);
        return;
    }
    if title.starts_with("You  ") && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        cancel_slot_click(ci);
        return;
    }
    let mut is_in_auction_gui: bool = (title.ends_with("Auction House") || title == "Create Auction" || title == "Create BIN Auction");
    if is_in_auction_gui && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        cancel_slot_click(ci);
        return;
    }
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons() && title == "Chest" && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone())) {
        cancel_slot_click(ci);
        return;
    }
    // Prevent selling protected items to NPC shops
    if is_chest_menu && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.clone()))
        && !crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_npc_sell_button(slot) && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_npc_sell_menu(this_menu) {
        cancel_slot_click(ci);
        return;
    }
    match chest_row_count {
        Some(6) => {
            crate::de::hysky::skyblocker::skyblock::garden::visitor::visitor_helper::VisitorHelper::on_slot_click(slot, slot_id, &title, this_menu.get_slot(13));
        }
        Some(_) if title == crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache::DONATION_CONFIRMATION_SCREEN_TITLE => {
            //Museum Item Cache donation tracking
            // java: MuseumItemCache.handleClick(slot, slotId, genericContainerScreenHandler.slots);
            crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache::MuseumItemCache::handle_click(slot, slot_id, &this_menu.slots);
        }
        _ => {}
    }
    //Pet Caching
    if button == mcrust::com::mojang::blaze3d::platform::input_constants::MOUSE_BUTTON_LEFT && title.starts_with("Pets") {
        crate::de::hysky::skyblocker::skyblock::pet_cache::PetCache::handle_pet_equip(slot, slot_id);
    }
    if current_solver.is_some() {
        let mut disallowed: bool = crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::on_slot_click(slot_id, &stack, button);
        if disallowed {
            cancel_slot_click(ci);
        }
    }
    // Bazaar Max Tracking
    // java: BazaarMax.INSTANCE.checkMaxValue(slot);
    crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_max::INSTANCE.get_mut().check_max_value(Some(slot.clone()));
}

/// @Inject skyblocker$drawOnItem
///   java: @Inject(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;item(Lnet/minecraft/world/item/ItemStack;III)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V  [exists with a different descriptor]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:4323 fn item(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
///
/// The port draws the item through the unhooked `hud::extract_item` (abstract_container_screen.rs:2853), so there is
/// no `item(..)` call to inject before (GAPS_M #8). The handler runs at `extractSlot`'s HEAD instead - also before the
/// item, so the backgrounds, the lock icon and the search shade layer under it as in Java - with `:227-245`'s
/// conditions for reaching the non-fake `item(..)` call (`slot_reaches_item_call`). `@Local(name = "slot")` is
/// `menu.getSlot(index)` (arguments 3 and 5) and `graphics` is the frame's extractor (the port's draws go through the
/// frame's global state).
fn skyblocker_draw_on_item(ci: &mut CallbackInfo<'_>) {
    let Some(menu) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>(3)) else { return; };
    let Some(containers) = compat::hook_arg::<mcrust::net::minecraft::world::inventory::slot::Containers<'_>>(ci, 4) else { return; };
    let index: i32 = ci.arg::<i32>(5).copied().unwrap_or(-1);
    let Some(slot) = (index >= 0).then(|| menu.slots.get(index as usize)).flatten() else { return; };
    let item = slot.get_item(containers);
    if !slot_reaches_item_call(slot, item) {
        return;
    }
    let graphics = mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        //   java: ItemBackgroundManager.drawBackgrounds(slot.getItem(), graphics, slot.x, slot.y);
        crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager::draw_backgrounds(item, &graphics, slot.x, slot.y);
    }
    // Item Protection
    if crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(item.duplicate())) {
        //   java: ItemProtection.drawSlotIcon(graphics, slot.x, slot.y);
        crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::draw_slot_icon(&graphics, slot.x, slot.y);
    }
    // Search - darken non-matching slots
    if crate::de::hysky::skyblocker::skyblock::inventory_search::InventorySearch::is_searching() && !crate::de::hysky::skyblocker::skyblock::inventory_search::InventorySearch::slot_matches(slot) {
        //   java: graphics.fill(slot.x, slot.y, slot.x + 16, slot.y + 16, 0xB0_000000);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), slot.x, slot.y, slot.x + 16, slot.y + 16, 0xB0_000000u32 as i32);
    }
}

/// Whether `extractSlot` reaches its non-fake `graphics.item(..)` call (`:227-245`) for this slot: not the empty-slot
/// icon (`:227-233`) and not a fake slot (`:241-245`'s `fakeItem` arm). (`:206-208`'s "draws nothing" case is an
/// empty single-slot drag preview, which the slot's own stack does not show.)
fn slot_reaches_item_call(slot: &mcrust::net::minecraft::world::inventory::slot::Slot, item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    if item.is_empty() && slot.is_active() && slot.get_no_item_icon().is_some() {
        return false;
    }
    !slot.is_fake()
}

thread_local! {
    /// `@Local(name = "slot") Slot slot` of the `extractSlot` call in flight, for `skyblocker$drawSlotText` (a
    /// call-site handler sees only the `itemDecorations` call's arguments). Recorded at `extractSlot`'s HEAD.
    static EXTRACT_SLOT: std::cell::RefCell<Option<mcrust::net::minecraft::world::inventory::slot::Slot>> = const { std::cell::RefCell::new(None) };
}

/// Not a Java injector: records `extractSlot`'s `slot` local (`menu.getSlot(index)`, arguments 3 and 5) while on
/// SkyBlock, the only case `skyblocker$drawSlotText` reads it.
fn skyblocker_record_extract_slot(ci: &mut CallbackInfo<'_>) {
    let slot = if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let index: i32 = ci.arg::<i32>(5).copied().unwrap_or(-1);
        ci.arg::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>(3).and_then(|menu| (index >= 0).then(|| menu.slots.get(index as usize).cloned()).flatten())
    } else {
        None
    };
    EXTRACT_SLOT.with(|cell| *cell.borrow_mut() = slot);
}

/// @Inject skyblocker$drawSlotText
///   java: @Inject(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V  [exists]
/// within: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:178 fn item_decorations_with_count(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, count_override: Option<&str>, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
///
/// `slot` is `extractSlot`'s local, recorded at its HEAD (`skyblocker_record_extract_slot`). Java's three-argument
/// `extractSlotText(graphics, font, slot)` is `extractSlotText(graphics, font, slot, slot.getItem(), slot.id, slot.x,
/// slot.y)`; the port's three-argument form cannot read `slot.getItem()` (it needs the menu's `Containers`), so the
/// seven-argument one is called with the stack this `itemDecorations` call is handed (argument 2) - the slot's item
/// except while a quick-craft drag previews the carried stack in it.
fn skyblocker_draw_slot_text(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let Some(stack) = compat::hook_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(ci, 2) else { return; };
        let Some(slot) = EXTRACT_SLOT.with(|cell| cell.borrow().clone()) else { return; };
        //   java: SlotTextManager.extractSlotText(graphics, font, slot);
        crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager::extract_slot_text_7(&mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new(), &compat::font(), Some(slot.clone()), stack, slot.index, slot.x, slot.y);
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
    // @ModifyVariable(at = STORE, name = "item"): the port's `extractTooltip` answers `item` - see the handler.
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractTooltip", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_modify_tooltip_display_stack)));
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot`, `net.minecraft.client.gui.GuiGraphicsExtractor#fakeItem(Lnet/minecraft/world/item/ItemStack;III)V`).priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(1, skyblocker_modify_display_stack_fake));
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/world/item/ItemStack;II)V; the mod wants (Lnet/minecraft/world/item/ItemStack;III)V; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::call_site(`net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot`, `net.minecraft.client.gui.GuiGraphicsExtractor#item(Lnet/minecraft/world/item/ItemStack;III)V`).priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(1, skyblocker_modify_display_stack_real));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V").priority(1000).modify_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(2, skyblocker_modify_display_stack_decorations));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#slotClicked", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_record_slot_clicked_screen)));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#slotClicked", "net.minecraft.client.multiplayer.MultiPlayerGameMode#handleContainerInput").priority(1000).inject(Shift::Before, skyblocker_on_slot_click));
    // java: @At(INVOKE GuiGraphicsExtractor.item(ItemStack;III)V) in extractSlot - the port's item draw is the unhooked
    // `hud::extract_item`; registered at extractSlot's HEAD with the call's conditions (see the handler).
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_draw_on_item)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_record_extract_slot)));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V").priority(1000).inject(Shift::Before, skyblocker_draw_slot_text));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractLabels", "net.minecraft.client.gui.GuiGraphicsExtractor#text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V").ordinal(0).priority(1000).wrap_with_condition(skyblocker_hide_chest_name));
}
