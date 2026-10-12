//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.GuiGraphicsExtractorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.GuiGraphicsExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue skyblocker$modifyItemCooldown
///   java: @ModifyExpressionValue(method = "itemCooldown", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/item/ItemCooldowns;getCooldownPercent(Lnet/minecraft/world/item/ItemStack;F)F"))
/// target: net.minecraft.world.item.ItemCooldowns#getCooldownPercent  [exists]
/// within: net.minecraft.client.gui.GuiGraphicsExtractor#itemCooldown
/// Rust: net/minecraft/world/item/item_cooldowns.rs:140 fn get_cooldown_percent(&self, item: &ItemStack, a: f32)
fn skyblocker_modify_item_cooldown(v: f32) -> f32 {
    let mut cooldown_progress = v;
    // TODO(fabric2seer): `stack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: return Utils.isOnSkyblock() && ItemCooldowns.isOnCooldown(stack) ? ItemCooldowns.getItemCooldownEntry(stack).getRemainingCooldownPercent() : cooldownProgress;
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @Inject initializeGrids
///   java: @Inject(method = "tooltip", at = @At("HEAD"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#tooltip  [exists]
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:6607 fn tooltip(graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, font: &dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont, lines: &[Box< dyn crate::net::minecraft::client
fn initialize_grids(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `font` is Rust argument 1: `&dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(1) }`
    // TODO(fabric2seer): `lines` is Rust argument 2: `&[Box< dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent, >]` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(2) }`
    let xo: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let yo: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    // TODO(fabric2seer): `positioner` is Rust argument 5: `&dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_positioner::ClientTooltipPositioner` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(5) }`
    // TODO(fabric2seer): `style` is Rust argument 6: `Option<&crate::net::minecraft::resources::identifier::Identifier>` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(6) }`
    let mut manager: Option<compat::Shared<crate::de::hysky::skyblocker::utils::render::text::grid_component_manager::GridComponentManager>> = None;
    for line in lines.iter() {
        if let Some(component) = line.as_any().downcast_ref::<compat::Shared<crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent>>() {
            if manager.is_none() {
                manager = Some(crate::de::hysky::skyblocker::utils::render::text::grid_component_manager::GridComponentManager::new());
            }
            crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent::set_manager(component, manager.clone());
        }
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.gui.GuiGraphicsExtractor#itemCooldown", "net.minecraft.world.item.ItemCooldowns#getCooldownPercent").priority(1000).modify_expression_value::<f32>(skyblocker_modify_item_cooldown));
    report(hook::register_injector("net.minecraft.client.gui.GuiGraphicsExtractor#tooltip", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(initialize_grids)));
}
