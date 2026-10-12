//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DyedItemColorMixin by fabric2seer.
//! @Mixin(net.minecraft.world.item.component.DyedItemColor)

#[allow(unused_imports)]
use crate::prelude::*;

thread_local! {
    /// `@Local(name = "itemStack") ItemStack stack` - the port's `getOrDefault` is handed the stack's `DYED_COLOR`
    /// component, not the stack (dyed_item_color.rs:60), so its two callers (`Dye.calculate:19`,
    /// `EquipmentLayerRenderer.renderLayers:69`) record the stack's `getUuid()` at their HEAD, and the
    /// `getOrDefault` call each makes next consumes it.
    static STACK_UUID: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// HEAD of a `getOrDefault` caller: records `stack.getUuid()` of the caller's `itemStack` (argument `index`).
fn record_stack(ci: &CallbackInfo<'_>, index: usize) {
    let uuid = if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(index).map(|stack| stack.get_uuid())
    } else {
        None
    };
    STACK_UUID.with(|cell| *cell.borrow_mut() = uuid);
}

fn record_dye_stack(ci: &mut CallbackInfo<'_>) {
    record_stack(ci, 0);
}

fn record_equipment_stack(ci: &mut CallbackInfo<'_>) {
    record_stack(ci, 4);
}

/// @ModifyReturnValue skyblocker$customDyeColor
///   java: @ModifyReturnValue(method = "getOrDefault", at = @At("RETURN"))
/// target: net.minecraft.world.item.component.DyedItemColor#getOrDefault  [exists]
/// Rust: net/minecraft/world/item/component/dyed_item_color.rs:60 fn get_or_default(color: Option<&DyedItemColor>, default_color: i32)
fn skyblocker_custom_dye_color(ci: &mut CallbackInfo<'_>) {
    let original_color: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    let recorded: Option<String> = STACK_UUID.with(|cell| cell.borrow_mut().take());
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        //   java: String itemUuid = stack.getUuid();
        let Some(item_uuid) = recorded else {
            ci.set_return_value(original_color);
            return;
        };
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.contains_key(&item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes::animate_color_transition(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap())));
            return;
        }
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.contains_key(&item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.get(&item_uuid).copied().unwrap_or_default()));
            return;
        }
    }
    ci.set_return_value(original_color);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.color.item.Dye#calculate", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(record_dye_stack)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer#renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(record_equipment_stack)));
    report(hook::register_injector("net.minecraft.world.item.component.DyedItemColor#getOrDefault", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_dye_color)));
}
