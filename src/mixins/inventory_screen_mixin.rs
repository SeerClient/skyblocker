//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.InventoryScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.InventoryScreen)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::injected::recipe_book_holder::RecipeBookHolder;

/// java: @Unique private final List<Runnable> recipeBookToggleCallbacks = new ArrayList<>();
/// A field of the open `InventoryScreen`. The port's `onRecipeBookButtonClick` is a free function with no screen to
/// read an attachment from, and there is one inventory screen at a time (`init` clears the list as each one opens),
/// so the field is this mixin's own list. `Runnable`s run on the render thread only.
pub static RECIPE_BOOK_TOGGLE_CALLBACKS: compat::MainThread<Vec<Arc<dyn Fn()>>> = compat::MainThread::new(|| Vec::new());

/// @Override
fn register_recipe_book_toggle_callback(runnable: Arc<dyn Fn()>) {
    RECIPE_BOOK_TOGGLE_CALLBACKS.get_mut().push(runnable);
}

/// @ModifyArg skyblocker$replaceRecipeBook
///   java: @ModifyArg(method = "<init>", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen;<init>(Lnet/minecraft/world/inventory/RecipeBookMenu;Lnet/minecraft/client/gui/screens/recipebook/RecipeBookComponent;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/network/chat/Component;)V"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen#<init>  [exists]
/// within: net.minecraft.client.gui.screens.inventory.InventoryScreen#<init>
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_recipe_book_screen.rs:458 fn new(recipe_book_component: RecipeBookComponent, player_inventory_title: String, title: String, recipe_book_button_position: fn(i32, i32) -> ScreenPosition,)
///
/// rust: port gap - the port's `RecipeBookComponent` is a concrete struct (recipe_book_component.rs:828), so the mod's
/// `SkyblockRecipeBookComponent extends RecipeBookComponent<?>` (a mod object) cannot be handed to
/// `AbstractRecipeBookScreen`'s constructor in its place (GAPS_batchD). Registered as `@ModifyArgs`, which leaves the
/// argument as it is.
fn skyblocker_replace_recipe_book(ci: &mut CallbackInfo<'_>) {
    //   java: return SkyblockerConfigManager.get().general.itemList.enableRecipeBook && Utils.isOnSkyblock() ? new SkyblockRecipeBookComponent(player.inventoryMenu) : original;
    let _ = ci;
}

/// @ModifyArg skyblocker$moveButton
///   java: @ModifyArg(method = "getRecipeBookButtonPosition", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/navigation/ScreenPosition;<init>(II)V"), index = 0)
/// target: net.minecraft.client.gui.navigation.ScreenPosition#<init>  [exists]
/// within: net.minecraft.client.gui.screens.inventory.InventoryScreen#getRecipeBookButtonPosition
/// Rust: net/minecraft/client/gui/navigation/screen_position.rs:19 fn new(x: i32, y: i32)
fn skyblocker_move_button(v: &i32) -> i32 {
    let mut x = v.clone();
    return (if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.show_equipment_in_inventory { x.wrapping_add(21) } else { x });
}

/// @WrapWithCondition skyblocker$dontExtractStatusEffects
///   java: @WrapWithCondition(method = "extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/inventory/EffectsInInventory;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V"))
/// target: net.minecraft.client.gui.screens.inventory.EffectsInInventory#extractRenderState  [exists]
/// within: net.minecraft.client.gui.screens.inventory.InventoryScreen#extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
/// Rust: net/minecraft/client/gui/screens/inventory/effects_in_inventory.rs:313 fn extract_render_state(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, screen_width: i32, left_pos: i32, image_width: i32, top_pos: i32, active_effects: &[crate::net::minecraft::world::effect::mob_effect_instance::MobEffectInstance], tickrate: f32, mouse: (
fn skyblocker_dont_extract_status_effects(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_status_effect_overlay || crate::de::hysky::skyblocker::utils::utils::Utils::is_in_garden() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().farming.plots_widget.enabled);
}

/// @ModifyReturnValue skyblocker$markStatusEffectsHidden
///   java: @ModifyReturnValue(method = "showsActiveEffects", at = @At("RETURN"))
/// target: net.minecraft.client.gui.screens.inventory.InventoryScreen#showsActiveEffects  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/inventory_screen.rs:205 fn shows_active_effects(screen_width: i32, left_pos: i32, image_width: i32)
fn skyblocker_mark_status_effects_hidden(ci: &mut CallbackInfo<'_>) {
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_garden() {
        ci.set_return_value((original && !(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_status_effect_overlay) && !(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().farming.plots_widget.enabled)));
        return;
    }
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        ci.set_return_value((original && !(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_status_effect_overlay)));
        return;
    }
    ci.set_return_value(original);
    return;
}

/// @ModifyExpressionValue skyblocker$getBackground
///   java: @ModifyExpressionValue(method = "extractBackground", at = @At(value = "FIELD", target = "Lnet/minecraft/client/gui/screens/inventory/InventoryScreen;INVENTORY_LOCATION:Lnet/minecraft/resources/Identifier;", opcode = Opcodes.GETSTATIC))
/// note: @ModifyExpressionValue at @At("FIELD"): only a CALL (@At("INVOKE")) has a call-site injector (`hook::call_site`); a local, a field, a constant or `new` has no injection point
fn skyblocker_get_background(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyExpressionValue is not expressible - the handler is kept for reference
    //   java: @ModifyExpressionValue(method = "extractBackground", at = @At(value = "FIELD", target = "Lnet/minecraft/client/gui/screens/inventory/InventoryScreen;INVENTORY_LOCATION:Lnet/minecraft/resources/Identifier;", opcode = Opcodes.GETSTATIC))
    //   java: 	private Identifier skyblocker$getBackground(Identifier original) {
    //   java: 		// gotta do this, if I don't call super in SkyblockInventoryScreen quick nav doesn't get rendered
    //   java: 		if (Utils.isOnSkyblock() && SkyblockerConfigManager.get().uiAndVisuals.showEquipmentInInventory) return SkyblockInventoryScreen.BACKGROUND.get();
    //   java: 		return original;
    //   java: 	}
}

/// @Inject skyblocker$callRecipeToggleCallbacks
///   java: @Inject(method = "onRecipeBookButtonClick", at = @At("TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.InventoryScreen#onRecipeBookButtonClick  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/inventory_screen.rs:349 fn on_recipe_book_button_click()
fn skyblocker_call_recipe_toggle_callbacks(ci: &mut CallbackInfo<'_>) {
    // A snapshot of the handles, so a callback may register another without re-entering the list.
    let callbacks: Vec<Arc<dyn Fn()>> = RECIPE_BOOK_TOGGLE_CALLBACKS.get().clone();
    for callback in callbacks.iter() {
        (callback)();
    }
}

/// @Inject skyblocker$clearRecipeToggleCallbacks
///   java: @Inject(method = "init", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.inventory.InventoryScreen#init  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/inventory_screen.rs:495 fn init(&mut self, width: i32, height: i32, gfx: &crate::net::minecraft::client::minecraft::Minecraft)
fn skyblocker_clear_recipe_toggle_callbacks(ci: &mut CallbackInfo<'_>) {
    RECIPE_BOOK_TOGGLE_CALLBACKS.get_mut().clear();
}

/// InventoryScreenMixin makes every InventoryScreen implement RecipeBookHolder (an interface added by a Mixin): an
/// extension trait of the mod, implemented for the port's type.
impl RecipeBookHolder for mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn register_recipe_book_toggle_callback(&self, callback: Arc<dyn Fn()>) {
        register_recipe_book_toggle_callback(callback);
    }

}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.InventoryScreen#<init>", "net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen#<init>").priority(1000).modify_args(skyblocker_replace_recipe_book));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.InventoryScreen#getRecipeBookButtonPosition", "net.minecraft.client.gui.navigation.ScreenPosition#<init>").priority(1000).modify_arg::<i32>(0, skyblocker_move_button));
    report(hook::call_site("net.minecraft.client.gui.screens.inventory.InventoryScreen#extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V", "net.minecraft.client.gui.screens.inventory.EffectsInInventory#extractRenderState").priority(1000).wrap_with_condition(skyblocker_dont_extract_status_effects));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.InventoryScreen#showsActiveEffects", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_mark_status_effects_hidden)));
    // TODO(fabric2seer): @ModifyExpressionValue skyblocker$getBackground: nothing to register - @ModifyExpressionValue at @At("FIELD"): only a CALL (@At("INVOKE")) has a call-site injector (`hook::call_site`); a local, a field, a constant or `new` has no injection point
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.InventoryScreen#onRecipeBookButtonClick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_call_recipe_toggle_callbacks)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.InventoryScreen#init", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_clear_recipe_toggle_callbacks)));
}
