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
    // PERF (port-only, same result): with no custom armor model configured the lookup below answers null for
    // every uuid (`assetKey` kept), so the stack's uuid is not read.
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_model.is_empty() {
        return;
    }
    let Some(stack) = compat::hook_arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(ci, 4) else { return; };
    // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the key memoised in the stack's uuid field
    // (`custom_armor_key_memoised`); `SKYBLOCKER_VANILLA_EQUIP_MEMO=1` looks it up every call (`custom_armor_key`).
    let key: Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey> = if crate::mixins::item_stack_mixin::VANILLA_EQUIP_MEMO.on() {
        custom_armor_key(stack)
    } else {
        custom_armor_key_memoised(stack)
    };
    let Some(key) = key else {
        //   java: return identifier == null ? assetKey : ..
        return;
    };
    if let Some(slot) = compat::hook_arg_mut::<&mcrust::net::minecraft::resources::resource_key::ResourceKey>(ci, 1) {
        *slot = key;
    }
}

/// `customArmorModel`'s answer for `stack`, looked up: the asset key to use, `None` for "keep `assetKey`".
fn custom_armor_key(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey> {
    // PERF (port-only, same result): the uuid borrowed (`with_uuid`), not a `String` per armour piece per frame -
    // it is only looked up in the map.
    //   java: Identifier identifier = SkyblockerConfigManager.get().general.customArmorModel.get(stack.getUuid());
    // (the config is read before the borrow: `with_uuid` runs its closure with the stack's attachments locked)
    let custom_armor_model = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_model;
    let found: Option<mcrust::net::minecraft::resources::identifier::Identifier> = crate::mixins::item_stack_mixin::with_uuid(stack, |uuid: &str| {
        if uuid.is_empty() {
            return None;
        }
        custom_armor_model.get(uuid).cloned()
    });
    //   java: return identifier == null ? assetKey : ..
    let identifier = found?;
    Some(intern_custom_key(identifier))
}

/// `ResourceKey.create(EquipmentAssets.ROOT_ID, identifier)`, interned (one leaked key per identifier).
fn intern_custom_key(identifier: mcrust::net::minecraft::resources::identifier::Identifier) -> &'static mcrust::net::minecraft::resources::resource_key::ResourceKey {
    //   java: return ResourceKey.create(EquipmentAssets.ROOT_ID, identifier);
    *compat::lock(&CUSTOM_KEYS).entry(identifier.clone()).or_insert_with(|| Box::leak(Box::new(mcrust::net::minecraft::resources::resource_key::ResourceKey::create(&mcrust::net::minecraft::world::item::equipment::equipment_assets::root_id(), identifier))))
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `custom_armor_key` answered from the memo kept in
/// the stack's uuid field (`item_stack_mixin::UuidField`) - one attachment read, no `HashMap<String, _>` lookup of a
/// 36-character uuid, no `Identifier` clones and no `CUSTOM_KEYS` lock per armour piece per frame.
///
/// The key is a pure function of (the cached uuid, the config instance). The memo is tagged with the config epoch
/// read BEFORE `get()` (`config_epoch`) and lives inside the uuid field, so it is gone wherever the field is removed
/// (`set(CUSTOM_DATA)`). A miss computes exactly what `custom_armor_key` computes - the lookup under the attachment
/// lock, the interning (whose first `ResourceKey.create` is a hooked call) outside it - and stores the answer into the
/// field it was computed from.
fn custom_armor_key_memoised(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey> {
    enum Probe {
        Hit(Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey>),
        Miss(Option<mcrust::net::minecraft::resources::identifier::Identifier>, Arc<str>),
    }
    let epoch: Option<u64> = crate::mixins::item_stack_mixin::config_epoch();
    let custom_armor_model = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_model;
    let probe = crate::mixins::item_stack_mixin::with_uuid_field(stack, |field| {
        if let Some(epoch) = epoch
            && let Some(key) = field.armor_memo(epoch)
        {
            return Probe::Hit(key);
        }
        let uuid: &Arc<str> = field.uuid();
        let found = if uuid.is_empty() { None } else { custom_armor_model.get(&**uuid).cloned() };
        Probe::Miss(found, Arc::clone(uuid))
    });
    match probe {
        Probe::Hit(key) => key,
        Probe::Miss(found, uuid) => {
            let key = found.map(intern_custom_key);
            if let Some(epoch) = epoch {
                crate::mixins::item_stack_mixin::store_armor_memo(stack, &uuid, epoch, key);
            }
            key
        }
    }
}

/// PERF (port-only, same result): the injector's pre-filter (`InjectOptions::when`) - exactly `custom_armor_model`'s
/// own two early returns. Off Skyblock, or with no custom armor model configured, the handler returns before touching
/// an argument (`assetKey` kept), so the call needs no `CallbackInfo` and no argument capture at all.
fn custom_armor_model_may_apply() -> bool {
    crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()
        && !crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_model.is_empty()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer#renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000).when(custom_armor_model_may_apply), Arc::new(custom_armor_model)));
}
