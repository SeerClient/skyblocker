//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityRenderDispatcherMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderDispatcher)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$shouldRender
///   java: @ModifyReturnValue(method = "shouldRender", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.entity.EntityRenderDispatcher#shouldRender  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_render_dispatcher.rs:1162 fn should_render(&self, entity: &dyn crate::net::minecraft::world::entity::entity::Entity, should_render_at_distance: bool, frustum_is_visible: &dyn Fn(&crate::net::minecraft::world::phys::aabb::AABB) -> bool, level: Option<&crate::net::minecraft::client::multiplayer::client_level::ClientLevel>,)
fn skyblocker_should_render(ci: &mut CallbackInfo<'_>) {
    // PERF (port-only, same result): both arms below need the hub/park or
    // dungeons; anywhere else the method answers `original`, which is
    // already the return value - so leave before any argument, config or
    // downcast work (~600 calls a frame). Two relaxed atomic loads.
    let in_hub_or_park = crate::de::hysky::skyblocker::utils::utils::Utils::is_in_hub() || crate::de::hysky::skyblocker::utils::utils::Utils::is_in_park();
    let in_dungeons = crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons();
    if !in_hub_or_park && !in_dungeons {
        return;
    }
    // PERF (port-only, same result): likewise when the arm for this location has its option off - checked
    // before the receiver/argument downcasts (both arms read only `original` then).
    let config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
    if !(in_hub_or_park && config.slayers.wolf_slayer.hide_sven_pup_nametag) && !(in_dungeons && config.dungeons.hide_soulweaver_skulls) {
        return;
    }
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::renderer::entity::entity_render_dispatcher::EntityRenderDispatcher>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // `Entity entity` is `shouldRender`'s argument 0 (a `&dyn Entity`).
    let Some(entity) = compat::hook_arg::<dyn mcrust::net::minecraft::world::entity::entity::Entity>(ci, 0) else { return; };
    let _ = this;
    use mcrust::net::minecraft::world::entity::entity::Entity as _;
    use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
    use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
    type ArmorStand = mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
    let armor_stand: Option<&ArmorStand> = mcrust::net::minecraft::world::entity::entity::Entity::as_any(entity).and_then(|a| a.downcast_ref::<ArmorStand>());
    // java: if ((Utils.isInHub() || Utils.isInPark()) && ...hideSvenPupNametag && entity instanceof ArmorStand armorStand
    //   && armorStand.getCustomName() instanceof Component nameTag && nameTag.getString().contains("Sven Pup")) return false;
    // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the name test memoised per stand
    // (`sven_pup_name_tag`). Was:
    //     && let Some(name_tag) = armor_stand.base().get_custom_name()
    //     && name_tag.get_string().contains("Sven Pup")
    if in_hub_or_park
        && config.slayers.wolf_slayer.hide_sven_pup_nametag
        && let Some(armor_stand) = armor_stand
        && sven_pup_name_tag(armor_stand)
    {
        ci.set_return_value(false);
        return;
    }
    // java: return Utils.isInDungeons() && ...hideSoulweaverSkulls && entity instanceof ArmorStand armorStand && entity.isInvisible()
    //   && armorStand.hasItemInSlot(EquipmentSlot.HEAD) ? !ItemUtils.getHeadTexture(armorStand.getItemBySlot(EquipmentSlot.HEAD)).equals(HeadTextures.SOULWEAVER_HAUNTED_SKULL) : original;
    if in_dungeons
        && config.dungeons.hide_soulweaver_skulls
        && let Some(armor_stand) = armor_stand
        && entity.base().is_invisible()
        && armor_stand.has_item_in_slot(EquipmentSlot::Head)
    {
        ci.set_return_value(crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_head_texture(armor_stand.get_item_by_slot(EquipmentSlot::Head)) != crate::de::hysky::skyblocker::skyblock::item::head_textures::SOULWEAVER_HAUNTED_SKULL);
        return;
    }
    ci.set_return_value(original);
}

/// `armorStand.getCustomName() instanceof Component nameTag && nameTag.getString().contains("Sven Pup")`, as written.
fn sven_pup_name_tag_vanilla(armor_stand: &mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand) -> bool {
    use mcrust::net::minecraft::world::entity::entity::Entity as _;
    armor_stand.base().get_custom_name().is_some_and(|name_tag| name_tag.get_string().contains("Sven Pup"))
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_SHOULD_RENDER=1` runs
/// `sven_pup_name_tag_vanilla` on every stand, every frame, as before.
static VANILLA_SHOULD_RENDER: compat::EnvSwitch = compat::EnvSwitch::new("SKYBLOCKER_VANILLA_SHOULD_RENDER");

/// The hooked methods the vanilla test calls and the memo does not (or calls once per name instead of per frame):
/// `Entity#getCustomName` and the `SynchedEntityData#get` inside it, and `getString`'s walk of the text. The memo
/// reads the name OBJECT through `Entity#getEntityData` instead, which the vanilla test does not call. While any
/// non-Track injector targets one of the exact methods - this mod's own included (it has none there) - the vanilla
/// test runs.
static SVEN_PUP_STAND_DOWN: compat::HookStandDown = compat::HookStandDown::new(
    &[
        ("net.minecraft.world.entity.Entity", "getCustomName"),
        ("net.minecraft.world.entity.Entity", "getEntityData"),
        ("net.minecraft.network.syncher.SynchedEntityData", "get"),
    ],
    false,
);

/// The same for the text classes `getString` walks, where this mod's own injectors do not count: its
/// `MutableComponent#getVisualOrderText` and `Language#getVisualOrder` are not on `getString`'s way (it visits the
/// contents; it never builds the visual order).
static SVEN_PUP_TEXT_STAND_DOWN: compat::HookStandDown =
    compat::HookStandDown::new(&[("net.minecraft.network.chat.", "*"), ("net.minecraft.locale.", "*")], true);

/// The stand's memo: the custom-name OBJECT the answer was computed from, HELD - its address cannot be reused while
/// held, and `DataItem::replace_value` writes a value in place only for a sole holder, so every write of the custom
/// name installs a NEW object and the next frame misses.
struct SvenPupMemo {
    name: mcrust::com::mojang::serialization::Obj,
    contains: bool,
}

/// One per stand (a per-object `@Unique`, port-only).
static ATTACH_SVEN_PUP_NAME_TAG: AttachmentType<SvenPupMemo> = AttachmentType::new("skyblocker:entity_render_dispatcher_mixin_sven_pup_name_tag");

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `sven_pup_name_tag_vanilla` without the deep
/// clone of the custom name (`getCustomName`) and the `String` built from it (`getString`) per stand per frame (~95
/// stands a hub frame). The answer is a pure function of the custom name's value, so it is kept with the name object
/// it was computed from (`SvenPupMemo`) and recomputed - from the same value, by the same expression - only when the
/// stand holds another object. The same boolean.
fn sven_pup_name_tag(armor_stand: &mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand) -> bool {
    use mcrust::net::minecraft::world::entity::entity::Entity as _;
    if VANILLA_SHOULD_RENDER.on() || !SVEN_PUP_STAND_DOWN.clear() || !SVEN_PUP_TEXT_STAND_DOWN.clear() {
        return sven_pup_name_tag_vanilla(armor_stand);
    }
    let name: &mcrust::com::mojang::serialization::Obj =
        armor_stand.base().get_entity_data().get_value_obj(&mcrust::net::minecraft::world::entity::entity::accessors().custom_name);
    let same_name = |memo: &SvenPupMemo| std::ptr::addr_eq(Arc::as_ptr(&memo.name), Arc::as_ptr(name));
    if let Some(contains) = armor_stand.with_attached(&ATTACH_SVEN_PUP_NAME_TAG, |memo: Option<&SvenPupMemo>| memo.filter(|memo| same_name(*memo)).map(|memo| memo.contains)) {
        return contains;
    }
    // `getCustomName()` is this value's clone; a value of another type is `get`'s panic, so it goes the vanilla way.
    let Some(custom_name) = mcrust::com::mojang::serialization::cast::<Option<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>>(name) else {
        return sven_pup_name_tag_vanilla(armor_stand);
    };
    let contains = custom_name.as_ref().is_some_and(|name_tag| name_tag.get_string().contains("Sven Pup"));
    armor_stand.set_attached(&ATTACH_SVEN_PUP_NAME_TAG, SvenPupMemo { name: Arc::clone(name), contains });
    contains
}

/// PERF (port-only, same result): the injector's pre-filter (`InjectOptions::when`) - the handler's own two leading
/// tests. Outside the hub/park/dungeons, or with the option of that location off, it answers `original`, so
/// `shouldRender` (~600-1600 calls a frame) then runs as if unhooked.
fn should_render_may_apply() -> bool {
    use crate::de::hysky::skyblocker::utils::utils::Utils;
    let in_hub_or_park = Utils::is_in_hub() || Utils::is_in_park();
    let in_dungeons = Utils::is_in_dungeons();
    if !in_hub_or_park && !in_dungeons {
        return false;
    }
    let config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
    (in_hub_or_park && config.slayers.wolf_slayer.hide_sven_pup_nametag) || (in_dungeons && config.dungeons.hide_soulweaver_skulls)
}

/// PERF (port-only, same result): the injector's ARGUMENT-AWARE pre-filter (`InjectOptions::when_args`). In the
/// hub/park (where `should_render_may_apply` passes on every call while `hideSvenPupNametag` is on) both arms still
/// need `entity instanceof ArmorStand`; for any other entity the handler answers `original`, which is already the
/// return value. So players, mobs, items and displays skip the handler and the rest of `dispatch`.
fn should_render_may_apply_to(ci: &CallbackInfo<'_>) -> bool {
    let Some(entity) = compat::hook_arg::<dyn mcrust::net::minecraft::world::entity::entity::Entity>(ci, 0) else {
        // Not readable: hand the call to the handler, as before this filter existed.
        return true;
    };
    mcrust::net::minecraft::world::entity::entity::Entity::as_any(entity)
        .is_some_and(|a| a.is::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>())
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderDispatcher#shouldRender", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000).when(should_render_may_apply).when_args(should_render_may_apply_to), Arc::new(skyblocker_should_render)));
}
