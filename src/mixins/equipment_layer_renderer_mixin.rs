//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EquipmentLayerRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyVariable customArmorModel
///   java: @ModifyVariable(method = "renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", at = @At("HEAD"), argsOnly = true)
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
fn custom_armor_model(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyVariable is not expressible - the handler is kept for reference
    //   java: @ModifyVariable(method = "renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", at = @At("HEAD"), argsOnly = true)
    //   java: 	private ResourceKey<EquipmentAsset> customArmorModel(ResourceKey<EquipmentAsset> assetKey, @Local(name = "itemStack") ItemStack stack) {
    //   java: 		if (Utils.isOnSkyblock() && !stack.getUuid().isEmpty()) {
    //   java: 			Identifier identifier = SkyblockerConfigManager.get().general.customArmorModel.get(stack.getUuid());
    //   java: 			return identifier == null ? assetKey : ResourceKey.create(EquipmentAssets.ROOT_ID, identifier);
    //   java: 		}
    //   java:
    //   java: 		return assetKey;
    //   java: 	}
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): @ModifyVariable customArmorModel: nothing to register - @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
}
