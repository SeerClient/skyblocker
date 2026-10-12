//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DataComponentHolderMixin by fabric2seer.
//! @Mixin(net.minecraft.core.component.DataComponentHolder)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$customComponents
///   java: @SuppressWarnings("unchecked")
///   java: @ModifyReturnValue(method = "get", at = @At("RETURN"))
/// target: net.minecraft.core.component.DataComponentHolder#get  [exists]
/// Rust: net/minecraft/core/component/data_component_holder.rs:51 fn get_from_components(&self, ty: &Arc<DataComponentType>)
fn skyblocker_custom_components(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::core::component::data_component_holder::DataComponentHolder>()) else { return; };
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (T)
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && let Some(stack) = this.downcast_ref::<mcrust::net::minecraft::world::item::item_stack::ItemStack>() {
        let mut item_uuid: String = stack.get_uuid();
        // TODO(fabric2seer): unknown name `dataComponentType`
        //   java: if (dataComponentType == DataComponents.TRIM) {
        //   java: 				CustomArmorTrims.ArmorTrimId trimKey = SkyblockerConfigManager.get().general.customArmorTrims.get(itemUuid);
        //   java:
        //   java: 				if (trimKey != null) {
        //   java: 					return (T) CustomArmorTrims.TRIMS_CACHE.getOrDefault(trimKey, (ArmorTrim) original);
        //   java: 				}
        //   java: 			} else if (dataComponentType == DataComponents.PROFILE && stack.is(Items.PLAYER_HEAD)) {
        //   java: 				// Normal head textures
        //   java: 				String tex = SkyblockerConfigManager.get().general.customHelmetTextures.get(itemUuid);
        //   java:
        //   java: 				if (tex != null) {
        //   java: 					return (T) CustomHelmetTextures.getProfile(tex);
        //   java: 				}
        //   java:
        //   java: 				// Animated heads
        //   java: 				String animatedTexId = SkyblockerConfigManager.get().general.customAnimatedHelmetTextures.get(itemUuid);
        //   java: 				ResolvableProfile frame = animatedTexId != null ? CustomAnimatedHelmetTextures.animateHeadTexture(animatedTexId) : null;
        //   java:
        //   java: 				if (frame != null) {
        //   java: 					return (T) frame;
        //   java: 				}
        //   java: 			} else if (dataComponentType == DataComponents.ENCHANTMENT_GLINT_OVERRIDE) {
        //   java: 				@SuppressWarnings("deprecation")
        //   java: 				Boolean glint = SkyblockerConfigManager.get().general.customGlint.get(itemUuid); // sorry fastutil :(
        //   java:
        //   java: 				if (glint != null) {
        //   java: 					return (T) glint;
        //   java: 				}
        //   java: 			} else if (dataComponentType == DataComponents.ITEM_MODEL) {
        //   java: 				Identifier id = SkyblockerConfigManager.get().general.customItemModel.get(itemUuid);
        //   java:
        //   java: 				if (id != null) {
        //   java: 					return (T) id;
        //   java: 				}
        //   java: 			}
    }
    ci.set_return_value(original.clone());
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.core.component.DataComponentHolder#get", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_components)));
}
