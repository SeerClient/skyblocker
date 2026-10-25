//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.GuiGraphicsExtractorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.GuiGraphicsExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue skyblocker$modifyItemCooldown
///   java: @ModifyExpressionValue(method = "itemCooldown", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/item/ItemCooldowns;getCooldownPercent(Lnet/minecraft/world/item/ItemStack;F)F"))
/// target: net.minecraft.world.item.ItemCooldowns#getCooldownPercent  [exists]
/// within: net.minecraft.client.gui.GuiGraphicsExtractor#itemCooldown
/// Rust: net/minecraft/world/item/item_cooldowns.rs:140 fn get_cooldown_percent(&self, item: &ItemStack, a: f32)
fn skyblocker_modify_item_cooldown(ci: &mut CallbackInfo<'_>) {
    let Some(cooldown_progress) = ci.return_value::<f32>().copied() else { return; };
    // `@Local(name = "itemStack") ItemStack stack` is the stack `itemCooldown` hands the call (argument 0).
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(0)) else { return; };
    ci.set_return_value(if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::is_on_cooldown(stack) {
        crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::get_item_cooldown_entry(stack).get_remaining_cooldown_percent()
    } else {
        cooldown_progress
    });
}

/// `line instanceof GridTooltipComponent component`: the game's `ClientTooltipComponent.as_any` answers the line's
/// object, and a grid line is the mod's `GridTooltipComponentLine` (`ClientTooltipComponentMixin.create`).
fn as_grid_tooltip_component(
    line: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent,
) -> Option<compat::Shared<crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent>> {
    line.as_any()
        .and_then(|any| any.downcast_ref::<crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponentLine>())
        .map(|line| line.0.clone())
}

/// @Inject initializeGrids
///   java: @Inject(method = "tooltip", at = @At("HEAD"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#tooltip  [exists]
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:6607 fn tooltip(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, font: &dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont, lines: &[Box< dyn crate::net::minecraft::client
fn initialize_grids(ci: &mut CallbackInfo<'_>) {
    // `List<ClientTooltipComponent> lines` is argument 2, a slice whose type carries a lifetime.
    let Some(lines) = compat::hook_arg::<[Box<dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent>]>(ci, 2) else { return; };
    let xo: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let yo: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    // null initially to not create instances needlessly, might be premature optimization but oh well
    let mut manager: Option<compat::Shared<crate::de::hysky::skyblocker::utils::render::text::grid_component_manager::GridComponentManager>> = None;
    for line in lines.iter() {
        if let Some(component) = as_grid_tooltip_component(&**line) {
            if manager.is_none() {
                manager = Some(crate::de::hysky::skyblocker::utils::render::text::grid_component_manager::GridComponentManager::new());
            }
            crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent::set_manager(&component, manager.clone());
        }
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // @ModifyExpressionValue: runs after the call (it reads the call's `itemStack` argument and replaces its value).
    report(hook::call_site("net.minecraft.client.gui.GuiGraphicsExtractor#itemCooldown", "net.minecraft.world.item.ItemCooldowns#getCooldownPercent").priority(1000).inject(Shift::After, skyblocker_modify_item_cooldown));
    report(hook::register_injector("net.minecraft.client.gui.GuiGraphicsExtractor#tooltip", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(initialize_grids)));
}
