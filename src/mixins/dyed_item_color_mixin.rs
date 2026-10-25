//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DyedItemColorMixin by fabric2seer.
//! @Mixin(net.minecraft.world.item.component.DyedItemColor)

#[allow(unused_imports)]
use crate::prelude::*;

thread_local! {
    /// `@Local(name = "itemStack") ItemStack stack` - the port's `getOrDefault` is handed the stack's `DYED_COLOR`
    /// component, not the stack (dyed_item_color.rs:60), so its two callers (`Dye.calculate:19`,
    /// `EquipmentLayerRenderer.renderLayers:69`) record the stack's `getUuid()` at their HEAD, and the
    /// `getOrDefault` call each makes next consumes it.
    ///
    /// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the uuid as the field's own `Arc<str>` (a
    /// reference count, not a `String` copy, per recorded armour piece). The same text. Was:
    ///     // static STACK_UUID: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
    static STACK_UUID: std::cell::RefCell<Option<Arc<str>>> = const { std::cell::RefCell::new(None) };
}

/// HEAD of a `getOrDefault` caller: records `stack.getUuid()` of the caller's `itemStack` (argument `index`).
fn record_stack(ci: &CallbackInfo<'_>, index: usize) {
    // PERF (port-only, same result): the recorded uuid is only ever looked up in the two dye maps; with both
    // empty `skyblocker$customDyeColor` answers `originalColor` whatever it is, so it is not read.
    //
    // PERF (port-only, same result): likewise a uuid in NEITHER map makes `skyblocker$customDyeColor` answer
    // `originalColor`, exactly as a recorded `None` does - so only a uuid one of the maps holds is recorded, read
    // borrowed (`with_uuid`, no `String` per armour piece) and cloned only then. The maps are the config's, which
    // nothing between this HEAD and the `getOrDefault` it feeds writes.
    //
    // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): "is the uuid in a dye map" memoised in the
    // stack's uuid field (`item_stack_mixin::UuidField`), tagged with the config epoch read BEFORE `get()` - a pure
    // function of (uuid, config instance), gone with the field on a `set(CUSTOM_DATA)`. Never memoised: `isOnSkyblock`,
    // the maps' emptiness test, and the cell itself, written every call as before. `SKYBLOCKER_VANILLA_EQUIP_MEMO=1`
    // looks the uuid up every call.
    let memoise = !crate::mixins::item_stack_mixin::VANILLA_EQUIP_MEMO.on();
    let epoch: Option<u64> = if memoise { crate::mixins::item_stack_mixin::config_epoch() } else { None };
    let general = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general;
    let uuid: Option<Arc<str>> = if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && !(general.custom_animated_dyes.is_empty() && general.custom_dye_colors.is_empty()) {
        ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(index).and_then(|stack| {
            if !memoise {
                // Was: (..).then(|| uuid.to_string())
                return crate::mixins::item_stack_mixin::with_uuid(stack, |uuid: &str| {
                    (general.custom_animated_dyes.contains_key(uuid) || general.custom_dye_colors.contains_key(uuid)).then(|| Arc::<str>::from(uuid))
                });
            }
            crate::mixins::item_stack_mixin::with_uuid_field(stack, |field| {
                if let Some(epoch) = epoch
                    && let Some(in_dye_map) = field.dye_memo(epoch)
                {
                    return in_dye_map.then(|| Arc::clone(field.uuid()));
                }
                let uuid: &str = field.uuid();
                let in_dye_map = general.custom_animated_dyes.contains_key(uuid) || general.custom_dye_colors.contains_key(uuid);
                if let Some(epoch) = epoch {
                    field.set_dye_memo(epoch, in_dye_map);
                }
                in_dye_map.then(|| Arc::clone(field.uuid()))
            })
        })
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

/// PERF (port-only, same result): `record_equipment_stack`'s pre-filter (`InjectOptions::when`). Off Skyblock, or
/// with both dye maps empty, `record_stack` writes `None` into `STACK_UUID` and does nothing else - a no-op exactly
/// when the cell already holds `None`. So the handler is skipped only then; a cell still holding a uuid an earlier
/// record left unconsumed is cleared by the handler as before.
fn record_equipment_stack_may_apply() -> bool {
    if STACK_UUID.with(|cell| cell.borrow().is_some()) {
        return true;
    }
    let general = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general;
    crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && !(general.custom_animated_dyes.is_empty() && general.custom_dye_colors.is_empty())
}

/// @ModifyReturnValue skyblocker$customDyeColor
///   java: @ModifyReturnValue(method = "getOrDefault", at = @At("RETURN"))
/// target: net.minecraft.world.item.component.DyedItemColor#getOrDefault  [exists]
/// Rust: net/minecraft/world/item/component/dyed_item_color.rs:60 fn get_or_default(color: Option<&DyedItemColor>, default_color: i32)
fn skyblocker_custom_dye_color(ci: &mut CallbackInfo<'_>) {
    let original_color: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    // (perf round 1, P02-mods-runtime: the recorded uuid is the field's `Arc<str>`.) Was:
    //     let recorded: Option<String> = STACK_UUID.with(|cell| cell.borrow_mut().take());
    let recorded: Option<Arc<str>> = STACK_UUID.with(|cell| cell.borrow_mut().take());
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        //   java: String itemUuid = stack.getUuid();
        let Some(item_uuid) = recorded else {
            ci.set_return_value(original_color);
            return;
        };
        let item_uuid: &str = &item_uuid;
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.contains_key(item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes::animate_color_transition(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.get(item_uuid).cloned().unwrap())));
            return;
        }
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.contains_key(item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.get(item_uuid).copied().unwrap_or_default()));
            return;
        }
    }
    ci.set_return_value(original_color);
    return;
}

/// PERF (port-only, same result): `skyblocker_custom_dye_color`'s pre-filter (`InjectOptions::when`). With nothing
/// recorded, the handler takes `None` out of an empty cell and answers `originalColor` (the return value already) -
/// a no-op. `getOrDefault`'s body does not touch the cell.
fn custom_dye_color_may_apply() -> bool {
    STACK_UUID.with(|cell| cell.borrow().is_some())
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `record_dye_stack` is `record_stack(ci, 0)`: the same pre-filter as the equipment one.
    report(hook::register_injector("net.minecraft.client.color.item.Dye#calculate", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000).when(record_equipment_stack_may_apply), Arc::new(record_dye_stack)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer#renderLayers(Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lnet/minecraft/world/item/ItemStack;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/resources/Identifier;II)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000).when(record_equipment_stack_may_apply), Arc::new(record_equipment_stack)));
    report(hook::register_injector("net.minecraft.world.item.component.DyedItemColor#getOrDefault", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000).when(custom_dye_color_may_apply), Arc::new(skyblocker_custom_dye_color)));
}
