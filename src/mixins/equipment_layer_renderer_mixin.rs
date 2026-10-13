//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EquipmentLayerRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyVariable customArmorModel
///   java: @ModifyVariable(method = "renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", at = @At("HEAD"), argsOnly = true)
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
///
/// `@ModifyVariable(argsOnly = true, at = HEAD)` on the method's own `assetKey` parameter is `@ModifyArg` at HEAD
/// (MODDING.md section 8): a HEAD injector that replaces argument 1. The parameter is a borrow (`&ResourceKey`), so the
/// replacement key is interned (one per identifier - Java's `ResourceKey.create` interns too) and lent from there.
/// `@Local(name = "itemStack")` is argument 4.
static CUSTOM_KEYS: LazyLock<Mutex<std::collections::HashMap<mcrust::net::minecraft::resources::identifier::Identifier, &'static mcrust::net::minecraft::resources::resource_key::ResourceKey>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

fn custom_armor_model(ci: &mut CallbackInfo<'_>) {
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        return;
    }
    let Some(stack) = compat::hook_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(ci, 4) else { return; };
    let uuid: String = stack.get_uuid();
    if uuid.is_empty() {
        return;
    }
    //   java: Identifier identifier = SkyblockerConfigManager.get().general.customArmorModel.get(stack.getUuid());
    let Some(identifier) = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_model.get(&uuid).cloned() else {
        //   java: return identifier == null ? assetKey : ..
        return;
    };
    //   java: return ResourceKey.create(EquipmentAssets.ROOT_ID, identifier);
    let key: &'static mcrust::net::minecraft::resources::resource_key::ResourceKey = *compat::lock(&CUSTOM_KEYS).entry(identifier.clone()).or_insert_with(|| Box::leak(Box::new(mcrust::net::minecraft::resources::resource_key::ResourceKey::create(&mcrust::net::minecraft::world::item::equipment::equipment_assets::root_id(), identifier))));
    if let Some(slot) = compat::hook_arg_mut::<&mcrust::net::minecraft::resources::resource_key::ResourceKey>(ci, 1) {
        *slot = key;
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer#renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(custom_armor_model)));
}
