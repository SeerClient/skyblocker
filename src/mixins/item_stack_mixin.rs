//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ItemStackMixin by fabric2seer.
//! @Mixin(net.minecraft.world.item.ItemStack)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::utils::item_ability::ItemAbility;

/// java: @Unique private float durabilityBarFill = -1;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
///
/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the field together with the inventoryTick memo it
/// was last computed under (`DurabilityBarFill`). Was:
///     // pub static ATTACH_DURABILITY_BAR_FILL: AttachmentType<f32> = AttachmentType::new("skyblocker:item_stack_mixin_durability_bar_fill");
pub static ATTACH_DURABILITY_BAR_FILL: AttachmentType<DurabilityBarFill> = AttachmentType::new("skyblocker:item_stack_mixin_durability_bar_fill");
/// java: @Unique private @Nullable String skyblockId;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_SKYBLOCK_ID: AttachmentType<String> = AttachmentType::new("skyblocker:item_stack_mixin_skyblock_id");
/// java: @Unique private @Nullable String skyblockApiId;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_SKYBLOCK_API_ID: AttachmentType<String> = AttachmentType::new("skyblocker:item_stack_mixin_skyblock_api_id");
/// java: @Unique private @Nullable String neuName;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_NEU_NAME: AttachmentType<String> = AttachmentType::new("skyblocker:item_stack_mixin_neu_name");
/// java: @Unique private @Nullable String uuid;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
///
/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the cached uuid together with the
/// equipment-render memo derived from it (`UuidField`). Was:
///     // pub static ATTACH_UUID: AttachmentType<String> = AttachmentType::new("skyblocker:item_stack_mixin_uuid");
pub static ATTACH_UUID: AttachmentType<UuidField> = AttachmentType::new("skyblocker:item_stack_mixin_uuid");
/// java: @Unique private @Nullable List<String> loreString;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_LORE_STRING: AttachmentType<Vec<String>> = AttachmentType::new("skyblocker:item_stack_mixin_lore_string");
/// java: @Unique private @Nullable List<ItemAbility> abilities;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_ABILITIES: AttachmentType<Vec<crate::de::hysky::skyblocker::utils::item_ability::ItemAbility>> = AttachmentType::new("skyblocker:item_stack_mixin_abilities");
/// java: @Unique private @Nullable PetInfo petInfo;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_PET_INFO: AttachmentType<crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo> = AttachmentType::new("skyblocker:item_stack_mixin_pet_info");
/// java: @Unique private @Nullable SkyblockItemRarity skyblockRarity;
/// One value per ItemStack OBJECT (a Mixin field): an attachment.
pub static ATTACH_SKYBLOCK_RARITY: AttachmentType<crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity> = AttachmentType::new("skyblocker:item_stack_mixin_skyblock_rarity");

// ===========================================================================
// perf round 1 (P02-mods-runtime): the memos behind `ATTACH_UUID` and `ATTACH_DURABILITY_BAR_FILL`
// ===========================================================================

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_EQUIP_MEMO=1` makes both
/// `EquipmentLayerRenderer#renderLayers` HEAD injectors (`custom_armor_model`, `record_equipment_stack`) and
/// `Dye#calculate`'s look the uuid up in the config maps on every call, as before.
pub static VANILLA_EQUIP_MEMO: compat::EnvSwitch = compat::EnvSwitch::new("SKYBLOCKER_VANILLA_EQUIP_MEMO");

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_DURABILITY=1` makes every
/// `inventoryTick` run `skyblocker$updateDamage`'s whole body, as before.
pub static VANILLA_DURABILITY: compat::EnvSwitch = compat::EnvSwitch::new("SKYBLOCKER_VANILLA_DURABILITY");

/// A memo tag no config epoch takes.
const NO_EPOCH: u64 = u64::MAX;

/// The config manager's instance epoch, as a memo's tag: it changes exactly when `SkyblockerConfigManager.get()`
/// starts answering another instance (a published instance is never changed). `None` - memoise nothing - when the
/// manager keeps no epoch. A caller reads it BEFORE `get()`: epochs only grow, so a tag is never newer than the
/// config it was computed from, and a memo computed from a newer config than its tag can never be hit.
pub fn config_epoch() -> Option<u64> {
    #[allow(unused_imports)]
    use dandelion::net::azureaaron::dandelion::api::config_manager::ConfigManager as _;
    crate::de::hysky::skyblocker::config::skyblocker_config_manager::CONFIG_MANAGER.get().instance_epoch().filter(|epoch| *epoch != NO_EPOCH)
}

/// `uuid` - the cached `getUuid()` - and, OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime), the
/// equipment-render memo derived from it, so both `EquipmentLayerRenderer#renderLayers` HEAD injectors answer from
/// this ONE attachment read instead of a uuid lookup in a `HashMap<String, _>` each (168 calls a hub frame).
///
/// Each half is a pure function of (`uuid`, the config instance), tagged with the config epoch it was computed under
/// (`config_epoch`); it leaves with the field wherever `ATTACH_UUID` is removed (`skyblocker$resetFields` on a
/// `set(CUSTOM_DATA)`), so it answers exactly what the uncached lookup answers. The halves are atomics only so a
/// `with_attached` reader, which gets `&UuidField`, can fill them: every access happens under the stack's attachment
/// lock.
pub struct UuidField {
    uuid: Arc<str>,
    /// `customArmorModel`'s asset key for `uuid` (null: none configured) under config epoch `armor_epoch`.
    armor_epoch: std::sync::atomic::AtomicU64,
    armor_key: std::sync::atomic::AtomicPtr<mcrust::net::minecraft::resources::resource_key::ResourceKey>,
    /// Whether `uuid` is in one of the two dye maps (`DyedItemColorMixin`), under config epoch `dye_epoch`.
    dye_epoch: std::sync::atomic::AtomicU64,
    in_dye_map: std::sync::atomic::AtomicBool,
}

impl UuidField {
    fn new(uuid: &str) -> UuidField {
        UuidField {
            uuid: Arc::from(uuid),
            armor_epoch: std::sync::atomic::AtomicU64::new(NO_EPOCH),
            armor_key: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
            dye_epoch: std::sync::atomic::AtomicU64::new(NO_EPOCH),
            in_dye_map: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// The cached `getUuid()`.
    pub fn uuid(&self) -> &Arc<str> {
        &self.uuid
    }

    /// The armour-model half, if it was computed under `epoch`: `Some(None)` is "no custom model".
    pub fn armor_memo(&self, epoch: u64) -> Option<Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey>> {
        use std::sync::atomic::Ordering;
        if self.armor_epoch.load(Ordering::Relaxed) != epoch {
            return None;
        }
        let key = self.armor_key.load(Ordering::Relaxed);
        // SAFETY: `set_armor_memo` stores null or a `&'static ResourceKey` (one `CUSTOM_KEYS` leaked).
        Some(unsafe { key.as_ref() })
    }

    pub fn set_armor_memo(&self, epoch: u64, key: Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey>) {
        use std::sync::atomic::Ordering;
        let key = key.map_or(std::ptr::null_mut(), |key| key as *const _ as *mut _);
        self.armor_key.store(key, Ordering::Relaxed);
        self.armor_epoch.store(epoch, Ordering::Relaxed);
    }

    /// The dye-map half, if it was computed under `epoch`.
    pub fn dye_memo(&self, epoch: u64) -> Option<bool> {
        use std::sync::atomic::Ordering;
        (self.dye_epoch.load(Ordering::Relaxed) == epoch).then(|| self.in_dye_map.load(Ordering::Relaxed))
    }

    pub fn set_dye_memo(&self, epoch: u64, in_dye_map: bool) {
        use std::sync::atomic::Ordering;
        self.in_dye_map.store(in_dye_map, Ordering::Relaxed);
        self.dye_epoch.store(epoch, Ordering::Relaxed);
    }
}

/// `durabilityBarFill`, and (OPTIMIZED BUT NOT FULLY VANILLA, perf round 1, P02-mods-runtime) the stack state the
/// full `skyblocker$updateDamage` body last computed it from.
pub struct DurabilityBarFill {
    /// java: durabilityBarFill - an unset attachment is the field's initial `-1`.
    pub fill: f32,
    /// `Some` only when `fill` is what the stack half of the body answers for this item and component patch (with
    /// every global conjunct true): written with the fill it vouches for, and dropped by any other write of the fill
    /// that changes it. See `skyblocker_get_and_cache_durability_memoised`.
    memo: Option<DurabilityMemo>,
}

/// The objects the durability was computed from, HELD: neither address can be reused while the memo lives, and a
/// held patch cannot be written in place - `PatchedDataComponentMap` writes through `Arc::make_mut` or replaces the
/// `Arc`, and a second owner makes every such write copy first (the `ItemResolutionKey` argument, chunk 337). So the
/// stack still holding the SAME patch object means it still holds the same components.
struct DurabilityMemo {
    item: Arc<mcrust::net::minecraft::world::item::item::Item>,
    patch: Arc<mcrust::net::minecraft::core::component::data_component_patch::PatchMap>,
}

/// `durabilityBarFill` as its readers see it.
fn durability_bar_fill(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> f32 {
    this.with_attached(&ATTACH_DURABILITY_BAR_FILL, |value: Option<&DurabilityBarFill>| value.map_or(-1.0f32, |value| value.fill))
}

/// The hooked game classes `skyblocker$updateDamage`'s stack half reaches (the stack, its components and custom data,
/// NBT, the lore's text). While another mod has a non-Track injector there, the memo stands down and every call runs
/// the whole body, so nothing such a mod observes changes. This mod's own injectors in those classes are inert on the
/// path: `DataComponentHolder#get` only acts for TRIM / PROFILE / ENCHANTMENT_GLINT_OVERRIDE / ITEM_MODEL (the body
/// reads CUSTOM_DATA and LORE), `getHoverName` / the tooltip / bar / `set` / `<init>` / `inventoryTick` injectors are
/// on methods the body does not call, and `getVisualOrderText` / `Language#getVisualOrder` are not on `getString`'s way.
static DURABILITY_STAND_DOWN: compat::HookStandDown = compat::HookStandDown::new(
    &[
        ("net.minecraft.world.item.", "*"),
        ("net.minecraft.core.component.", "*"),
        ("net.minecraft.nbt.", "*"),
        ("net.minecraft.network.chat.", "*"),
        ("net.minecraft.locale.", "*"),
        ("net.minecraft.ChatFormatting", "*"),
    ],
    true,
);

/// @Unique
fn skyblocker_should_process(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    // java: `Minecraft.getInstance() != null && Minecraft.getInstance().gui != null`: the game's one Minecraft and
    // its `Gui` always exist here; `current_screen_is` answers false before there is a screen.
    // PERF (port-only, same result): the side-effect-free conjuncts are tested cheapest first - `isOnSkyblock()`
    // is one relaxed atomic load, the screen test a lock - so off Skyblock (every stack, every inventory tick)
    // nothing else runs; `hasCustomDurability` (which caches the stack's skyblock id) stays last, as in Java.
    return (crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && !(mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>()) && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.enable_drill_fuel && crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::has_custom_durability(this));
}

/// `durabilityBarFill = fill`. PERF (port-only, same result): every reader takes an unset attachment as the
/// field's initial `-1`, so a write that would leave the value the readers see unchanged is skipped - it was one
/// boxed allocation and a map insert per stack per inventory tick, nearly always storing the `-1` already there.
fn skyblocker_set_durability_bar_fill(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack, fill: f32) {
    // Was: |current: Option<&f32>| current.copied().unwrap_or(-1.0f32) == fill
    let unchanged: bool = this.with_attached(&ATTACH_DURABILITY_BAR_FILL, |current: Option<&DurabilityBarFill>| current.map_or(-1.0f32, |current| current.fill) == fill);
    if !unchanged {
        // A changed fill drops any memo (perf round 1, P02-mods-runtime): the memo vouches for the fill stored with it.
        // Was: this.set_attached(&ATTACH_DURABILITY_BAR_FILL, fill);
        this.set_attached(&ATTACH_DURABILITY_BAR_FILL, DurabilityBarFill { fill, memo: None });
    }
}

/// @Unique
fn skyblocker_get_and_cache_durability(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
    if !(skyblocker_should_process(this)) {
        skyblocker_set_durability_bar_fill(this, ((-1) as f32) as f32);
        return;
    }
    let Some(mut durability) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_durability(&this) else {
        skyblocker_set_durability_bar_fill(this, ((-1) as f32) as f32);
        return;
    };
    skyblocker_set_durability_bar_fill(this, (((durability.0) as f32) / (durability.1 as f32)));
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `skyblocker_get_and_cache_durability` for the
/// TAIL of `inventoryTick` (~195 calls a hub tick, ~0.87 us each), with the stack half memoised.
///
/// `skyblockerShouldProcess`' three global conjuncts are evaluated on EVERY call, first, and short-circuited as there
/// (never memoised); while any is false the body's first arm runs as before. With all three true, the rest of the body -
/// `hasCustomDurability`, then `getDurability` - reads only the stack: its item, its components, and the skyblock-id
/// and lore caches derived from them, which only `set(CUSTOM_DATA / LORE)` (a component write) clears. So when the
/// stored fill carries a memo naming this stack's SAME item and SAME (held) patch object, the fill already is what the
/// rest of the body would write, and the call returns. The injector itself still runs on every call; only its body
/// short-circuits.
fn skyblocker_get_and_cache_durability_memoised(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
    let globals = crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()
        && !(mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>())
        && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.enable_drill_fuel;
    if !globals {
        // `!skyblockerShouldProcess()` before its stack half.
        skyblocker_set_durability_bar_fill(this, ((-1) as f32) as f32);
        return;
    }
    // `None`: an empty stack, or another mod's injector on the path (`DURABILITY_STAND_DOWN`) - the whole stack half.
    let parts = if DURABILITY_STAND_DOWN.clear() { this.render_memo_parts() } else { None };
    let Some((item, _prototype, patch)) = parts else {
        skyblocker_set_durability_bar_fill(this, skyblocker_durability_stack_half(this));
        return;
    };
    let hit: bool = this.with_attached(&ATTACH_DURABILITY_BAR_FILL, |value: Option<&DurabilityBarFill>| {
        value
            .and_then(|value| value.memo.as_ref())
            .is_some_and(|memo| Arc::ptr_eq(&memo.item, item) && Arc::ptr_eq(&memo.patch, patch))
    });
    if hit {
        return;
    }
    // Held from here: a write to the stack's components after this point copies the patch, and the next call misses.
    let memo = DurabilityMemo { item: Arc::clone(item), patch: Arc::clone(patch) };
    let fill = skyblocker_durability_stack_half(this);
    // (A panic above leaves the fill and any older memo as they were, as the body leaves the fill.)
    this.set_attached(&ATTACH_DURABILITY_BAR_FILL, DurabilityBarFill { fill, memo: Some(memo) });
}

/// The body after `skyblockerShouldProcess`' global conjuncts held: `hasCustomDurability`, then `getDurability` - the
/// fill they leave.
fn skyblocker_durability_stack_half(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> f32 {
    if !crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::has_custom_durability(this) {
        return ((-1) as f32) as f32;
    }
    match crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_durability(&this) {
        None => ((-1) as f32) as f32,
        Some(durability) => ((durability.0) as f32) / (durability.1 as f32),
    }
}

/// @SuppressWarnings("deprecation")
fn get_skyblock_id(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if let Some(skyblock_id) = this.get_attached(&ATTACH_SKYBLOCK_ID) && !skyblock_id.is_empty() {
        return skyblock_id;
    }
    // java: return skyblockId = ItemUtils.getItemId(this);
    let skyblock_id = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_item_id(this);
    this.set_attached(&ATTACH_SKYBLOCK_ID, skyblock_id.clone());
    return skyblock_id;
}

/// @SuppressWarnings("deprecation")
fn get_skyblock_api_id(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if let Some(skyblock_api_id) = this.get_attached(&ATTACH_SKYBLOCK_API_ID) && !skyblock_api_id.is_empty() {
        return skyblock_api_id;
    }
    // java: return skyblockApiId = ItemUtils.getSkyblockApiId(this);
    let skyblock_api_id = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_skyblock_api_id(this);
    this.set_attached(&ATTACH_SKYBLOCK_API_ID, skyblock_api_id.clone());
    return skyblock_api_id;
}

/// @SuppressWarnings("deprecation")
fn get_neu_name(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if let Some(neu_name) = this.get_attached(&ATTACH_NEU_NAME) && !neu_name.is_empty() {
        return neu_name;
    }
    // java: return neuName = ItemUtils.getNeuId((ItemStack) (Object) this);
    let neu_name = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_neu_id(this);
    this.set_attached(&ATTACH_NEU_NAME, neu_name.clone());
    return neu_name;
}

/// @SuppressWarnings("deprecation")
fn get_uuid(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    // (perf round 1, P02-mods-runtime: the field is a `UuidField` now; the same `String` is answered.) Was:
    //     if let Some(uuid) = this.get_attached(&ATTACH_UUID) {
    //         return uuid;
    //     }
    if let Some(uuid) = this.with_attached(&ATTACH_UUID, |field: Option<&UuidField>| field.map(|field| field.uuid.to_string())) {
        return uuid;
    }
    // java: return uuid = ItemUtils.getItemUuid(this);
    let uuid = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_item_uuid(this);
    // Was: this.set_attached(&ATTACH_UUID, uuid.clone());
    this.set_attached(&ATTACH_UUID, UuidField::new(&uuid));
    return uuid;
}

/// `f(stack.getUuid())`, with the uuid BORROWED - `get_uuid` with no `String` clone when the cached field is set.
/// PERF (port-only, same result): the render-path readers (`DataComponentHolderMixin`, `DyedItemColorMixin`,
/// `EquipmentLayerRendererMixin`) only look the uuid up in config maps, per armour piece per frame. The same value
/// `get_uuid` answers, and the same cache write on a miss. `f` runs with the stack's attachment cell locked when the
/// field is cached: it must not touch this stack's attachments.
pub fn with_uuid<R>(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack, f: impl FnOnce(&str) -> R) -> R {
    let mut f = Some(f);
    // Was: |uuid: Option<&String>| uuid.map(|uuid| (f.take().expect("with_uuid: f runs once"))(uuid))
    if let Some(r) = this.with_attached(&ATTACH_UUID, |field: Option<&UuidField>| field.map(|field| (f.take().expect("with_uuid: f runs once"))(&*field.uuid))) {
        return r;
    }
    let uuid: String = get_uuid(this);
    (f.take().expect("with_uuid: f runs once"))(&uuid)
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `with_uuid` handing over the cached FIELD, so a
/// reader can consult and fill the memo stored with the uuid (`UuidField`). The same uuid `with_uuid` lends and the
/// same cache write on a miss; `f` runs with the stack's attachment cell locked and must not touch this stack's
/// attachments. On a miss `f` gets the field `get_uuid` just stored (a memo it writes stays with it); should that be
/// gone again already, a field that is not stored.
pub fn with_uuid_field<R>(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack, f: impl FnOnce(&UuidField) -> R) -> R {
    let mut f = Some(f);
    if let Some(r) = this.with_attached(&ATTACH_UUID, |field: Option<&UuidField>| field.map(|field| (f.take().expect("with_uuid_field: f runs once"))(field))) {
        return r;
    }
    let uuid: String = get_uuid(this);
    if let Some(r) = this.with_attached(&ATTACH_UUID, |field: Option<&UuidField>| field.map(|field| (f.take().expect("with_uuid_field: f runs once"))(field))) {
        return r;
    }
    (f.take().expect("with_uuid_field: f runs once"))(&UuidField::new(&uuid))
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): stores the armour-model half of the memo -
/// `key`, computed under config `epoch` - into the field whose uuid it was computed from. A `set(CUSTOM_DATA)` since
/// replaces the field (`uuid` is then not its `Arc`, and stays held so its address cannot be reused), and nothing is
/// stored.
pub fn store_armor_memo(
    this: &mcrust::net::minecraft::world::item::item_stack::ItemStack,
    uuid: &Arc<str>,
    epoch: u64,
    key: Option<&'static mcrust::net::minecraft::resources::resource_key::ResourceKey>,
) {
    this.with_attached(&ATTACH_UUID, |field: Option<&UuidField>| {
        if let Some(field) = field
            && Arc::ptr_eq(&field.uuid, uuid)
        {
            field.set_armor_memo(epoch, key);
        }
    });
}

/// `f(stack.getSkyblockId())`, with the id BORROWED - `get_skyblock_id` with no `String` clone when the cached field
/// is set. OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the per-frame / per-slot readers only
/// compare the id. The same value `get_skyblock_id` answers (an empty cached id is recomputed, as there) and the same
/// cache write on a miss. `f` runs with the stack's attachment cell locked when the field is cached: it must not
/// touch this stack's attachments.
pub fn with_skyblock_id<R>(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack, f: impl FnOnce(&str) -> R) -> R {
    let mut f = Some(f);
    if let Some(r) = this.with_attached(&ATTACH_SKYBLOCK_ID, |skyblock_id: Option<&String>| {
        skyblock_id.filter(|skyblock_id| !skyblock_id.is_empty()).map(|skyblock_id| (f.take().expect("with_skyblock_id: f runs once"))(skyblock_id.as_str()))
    }) {
        return r;
    }
    let skyblock_id: String = get_skyblock_id(this);
    (f.take().expect("with_skyblock_id: f runs once"))(&skyblock_id)
}

/// @SuppressWarnings("deprecation")
fn skyblocker_get_lore_strings(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Vec<String> {
    if let Some(lore_string) = this.get_attached(&ATTACH_LORE_STRING) {
        return lore_string;
    }
    // java: return loreString = ItemUtils.getLore((ItemStack) (Object) this).stream().map(Component::getString).toList();
    let lore_string: Vec<String> = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_lore(this)
        .iter()
        .map(|line| <_ as mcrust::net::minecraft::network::chat::component::Component>::get_string(line))
        .collect();
    this.set_attached(&ATTACH_LORE_STRING, lore_string.clone());
    return lore_string;
}

/// @SuppressWarnings("deprecation")
fn skyblocker_get_abilities(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Vec<ItemAbility> {
    if let Some(abilities) = this.get_attached(&ATTACH_ABILITIES) {
        return abilities;
    }
    // java: return abilities = ItemAbility.getAbilities((ItemStack) (Object) this);
    let abilities = ItemAbility::get_abilities(this);
    this.set_attached(&ATTACH_ABILITIES, abilities.clone());
    return abilities;
}

/// @SuppressWarnings("deprecation")
fn get_pet_info(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> PetInfo {
    if let Some(pet_info) = this.get_attached(&ATTACH_PET_INFO) {
        return pet_info;
    }
    // java: return petInfo = ItemUtils.getPetInfo((ItemStack) (Object) this);
    let pet_info = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_pet_info(this);
    this.set_attached(&ATTACH_PET_INFO, pet_info.clone());
    return pet_info;
}

/// @SuppressWarnings("deprecation")
fn get_skyblock_rarity(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> SkyblockItemRarity {
    if let Some(skyblock_rarity) = this.get_attached(&ATTACH_SKYBLOCK_RARITY) {
        return skyblock_rarity;
    }
    // java: return skyblockRarity = ItemUtils.getItemRarity((ItemStack) (Object) this);
    let skyblock_rarity = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_item_rarity(this);
    this.set_attached(&ATTACH_SKYBLOCK_RARITY, skyblock_rarity);
    return skyblock_rarity;
}

/// @ModifyReturnValue skyblocker$customItemNames
///   java: @ModifyReturnValue(method = "getHoverName", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getHoverName  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:986 fn get_hover_name(&self)
fn skyblocker_custom_item_names(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    // `Component original`: the port's `getHoverName` answers the component as the `FormattedText` it is.
    let Some(original) = ci.take_return_value::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>() else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let custom_item_names = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_item_names;
        // PERF (port-only, same result): an empty map answers `original` for every uuid - skip reading it.
        if custom_item_names.is_empty() {
            ci.set_return_value(original);
            return;
        }
        ci.set_return_value(custom_item_names.get(&get_uuid(this)).cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).unwrap_or(original));
        return;
    }
    ci.set_return_value(original);
    return;
}

/// @ModifyExpressionValue skyblocker$hideVanillaEnchants
///   java: @ModifyExpressionValue(method = "addToTooltip", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/item/component/TooltipDisplay;shows(Lnet/minecraft/core/component/DataComponentType;)Z"))
/// target: net.minecraft.world.item.component.TooltipDisplay#shows  [exists]
/// within: net.minecraft.world.item.ItemStack#addToTooltip
/// Rust: net/minecraft/world/item/component/tooltip_display.rs:98 fn shows(&self, component: &Arc<DataComponentType>)
fn skyblocker_hide_vanilla_enchants(ci: &mut CallbackInfo<'_>) {
    let Some(should_display) = ci.return_value::<bool>().copied() else { return; };
    // `@Local(name = "component") TooltipProvider component` is `this.get(type)` of `addToTooltip:845`; the call's
    // own argument is that `type`, and a component is an `ItemEnchantments` exactly when its type is one of the two
    // `ItemEnchantments`-valued components (`ENCHANTMENTS`, `STORED_ENCHANTMENTS`).
    let Some(component_type) = ci.arg::<std::sync::Arc<mcrust::net::minecraft::core::component::data_component_type::DataComponentType>>(0) else { return; };
    let is_item_enchantments = std::sync::Arc::ptr_eq(component_type, mcrust::net::minecraft::core::component::data_components::enchantments())
        || std::sync::Arc::ptr_eq(component_type, mcrust::net::minecraft::core::component::data_components::stored_enchantments());
    ci.set_return_value(should_display && !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && is_item_enchantments));
}

/// @Inject skyblocker$skyblockIdTooltip
///   java: @Inject(method = "addDetailsToTooltip",
/// target: java.util.function.Consumer#accept(Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.world.item.ItemStack#addDetailsToTooltip
/// note: ordinal 0: the call is under an `if` in the Java of the method, so the run-time count (per run of the method) may differ from Mixin's bytecode ordinal - check it
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: slice = @Slice(..) ignored
/// note: class not in JavaSrc: java.util.function.Consumer has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_skyblock_id_tooltip(ci: &mut CallbackInfo<'_>) {
    // The game's `consumer_accept(builder, value)` is the item-id line's `builder.accept(..)` inside
    // `ItemStack#addDetailsToTooltip`; `this` is the stack whose method the call is in (`ci.caller`'s receiver is
    // not handed over, so the stack is the one the tooltip is for: `addDetailsToTooltip`'s receiver, recorded at its
    // HEAD by `skyblocker_record_details_stack`). `shift = AFTER` at the call's HEAD: the call's own line is emitted
    // first, then this one, and the call is cancelled.
    let Some(this) = DETAILS_STACK.with(|cell| cell.borrow().clone()) else { return; };
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        return;
    }
    let skyblock_id: String = get_skyblock_id(&this);
    if skyblock_id.is_empty() {
        return;
    }
    let Some(value) = ci.arg::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(1).cloned() else { return; };
    // SAFETY: argument 0 is `builder: &mut dyn FnMut(FormattedText)`, valid for this callback.
    let Some(text_consumer) = (unsafe { ci.arg_unchecked_mut::<dyn FnMut(mcrust::net::minecraft::network::chat::formatted_text::FormattedText)>(0) }) else { return; };
    text_consumer(value);
    //   java: textConsumer.accept(Component.literal("skyblock:" + skyblockId).withStyle(ChatFormatting.DARK_GRAY));
    text_consumer(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::component::literal(&format!("skyblock:{}", skyblock_id)).with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::DarkGray)));
    ci.cancel();
}

thread_local! {
    /// `this` of the `addDetailsToTooltip` call in flight (its HEAD), for the call-site handler above.
    static DETAILS_STACK: std::cell::RefCell<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = const { std::cell::RefCell::new(None) };
}

/// Not a Java injector: records `addDetailsToTooltip`'s receiver for `skyblocker$skyblockIdTooltip`.
fn skyblocker_record_details_stack(ci: &mut CallbackInfo<'_>) {
    let stack = ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>().map(|s| s.duplicate());
    DETAILS_STACK.with(|cell| *cell.borrow_mut() = stack);
}

/// @Inject skyblocker$updateDamage
///   java: @Inject(method = "inventoryTick", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#inventoryTick  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:707 fn inventory_tick(&mut self)
fn skyblocker_update_damage(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the stack half memoised (see there). Was:
    //     skyblocker_get_and_cache_durability(this);
    if VANILLA_DURABILITY.on() {
        skyblocker_get_and_cache_durability(this);
        return;
    }
    skyblocker_get_and_cache_durability_memoised(this);
}

/// @ModifyReturnValue modifyItemBarVisible
///   java: @ModifyReturnValue(method = "isBarVisible", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#isBarVisible  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:814 fn is_bar_visible(&self)
fn modify_item_bar_visible(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // (perf round 1, P02-mods-runtime: the field read in place.) Was:
    //     ci.set_return_value((original || this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32) >= 0.0f32));
    ci.set_return_value((original || durability_bar_fill(this) >= 0.0f32));
    return;
}

/// @ModifyReturnValue modifyItemBarStep
///   java: @ModifyReturnValue(method = "getBarWidth", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarWidth  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:820 fn get_bar_width(&self)
fn modify_item_bar_step(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    // (the field read once: the two reads of `durabilityBarFill` in Java's expression see one value)
    // (perf round 1, P02-mods-runtime: read in place.) Was:
    //     let durability_bar_fill: f32 = this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32);
    let durability_bar_fill: f32 = durability_bar_fill(this);
    ci.set_return_value((if durability_bar_fill >= (0 as f32) { (((durability_bar_fill * (13 as f32))) as i32) } else { original }));
    return;
}

/// @ModifyReturnValue modifyItemBarColor
///   java: @ModifyReturnValue(method = "getBarColor", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarColor  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:826 fn get_bar_color(&self)
fn modify_item_bar_color(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    // (the field read once: the two reads of `durabilityBarFill` in Java's expression see one value)
    // (perf round 1, P02-mods-runtime: read in place.) Was:
    //     let durability_bar_fill: f32 = this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32);
    let durability_bar_fill: f32 = durability_bar_fill(this);
    ci.set_return_value((if durability_bar_fill >= (0 as f32) { crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor::interpolate(mcrust::net::minecraft::util::common_colors::RED, mcrust::net::minecraft::util::common_colors::GREEN, durability_bar_fill) } else { original }));
    return;
}

/// @Inject onInit
///   java: @Inject(method = "<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:223 fn new_with_components(item: Arc<Item>, count: i32, components: PatchedDataComponentMap)
fn on_init(ci: &mut CallbackInfo<'_>) {
    // The port's constructor is the static `new_with_components`: the object under construction is its value.
    let Some(this) = compat::detach_ref(ci.return_value::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    skyblocker_get_and_cache_durability(this);
}

/// @Inject skyblocker$resetFields
///   java: @Inject(method = "set*", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#set  [exists with a different descriptor]
/// Rust: net/minecraft/world/item/item_stack.rs:453 fn set(&mut self, ty: Arc<DataComponentType>, value: Option<Obj>)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;, (Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Object;; the mod gave no descriptor; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_reset_fields(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    // `type` is by value and moved into the body, so this runs at HEAD (registered there): the caches are only read
    // lazily by the getters above, none of which `set` calls, so clearing them before the write is clearing them
    // after it.
    let Some(r#type) = ci.arg::<std::sync::Arc<mcrust::net::minecraft::core::component::data_component_type::DataComponentType>>(0) else { return; };
    if std::sync::Arc::ptr_eq(r#type, mcrust::net::minecraft::core::component::data_components::custom_data()) {
        this.remove_attached(&ATTACH_UUID);
        this.remove_attached(&ATTACH_SKYBLOCK_ID);
        this.remove_attached(&ATTACH_SKYBLOCK_API_ID);
        this.remove_attached(&ATTACH_NEU_NAME);
        this.remove_attached(&ATTACH_PET_INFO);
    }
    if std::sync::Arc::ptr_eq(r#type, mcrust::net::minecraft::core::component::data_components::lore()) {
        this.remove_attached(&ATTACH_LORE_STRING);
        this.remove_attached(&ATTACH_SKYBLOCK_RARITY);
        this.remove_attached(&ATTACH_ABILITIES);
    }
}

/// ItemStackMixin makes every ItemStack implement SkyblockerStack (an interface added by a Mixin): an
/// extension trait of the mod, implemented for the port's type.
impl SkyblockerStack for mcrust::net::minecraft::world::item::item_stack::ItemStack {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_skyblock_id(&self) -> String {
        get_skyblock_id(self)
    }

    fn get_skyblock_api_id(&self) -> String {
        get_skyblock_api_id(self)
    }

    fn get_neu_name(&self) -> String {
        get_neu_name(self)
    }

    fn get_uuid(&self) -> String {
        get_uuid(self)
    }

    fn skyblocker_get_lore_strings(&self) -> Vec<String> {
        skyblocker_get_lore_strings(self)
    }

    fn skyblocker_get_abilities(&self) -> Vec<ItemAbility> {
        skyblocker_get_abilities(self)
    }

    fn get_pet_info(&self) -> PetInfo {
        get_pet_info(self)
    }

    fn get_skyblock_rarity(&self) -> SkyblockItemRarity {
        get_skyblock_rarity(self)
    }

}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getHoverName", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_item_names)));
    // @ModifyExpressionValue: runs after the call (it reads the call's `type` argument and replaces its value).
    report(hook::call_site("net.minecraft.world.item.ItemStack#addToTooltip", "net.minecraft.world.item.component.TooltipDisplay#shows").priority(1000).inject(Shift::After, skyblocker_hide_vanilla_enchants));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#addDetailsToTooltip", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_record_details_stack)));
    report(hook::call_site("net.minecraft.world.item.ItemStack#addDetailsToTooltip", "java.util.function.Consumer#accept(Ljava/lang/Object;)V").ordinal(0).priority(1000).inject(Shift::Before, skyblocker_skyblock_id_tooltip));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#inventoryTick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_damage)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#isBarVisible", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_visible)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarWidth", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_step)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarColor", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_color)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_init)));
    // java: `method = "set*"` with a `(DataComponentType<T>, T)` handler: the one `set` overload of that shape. At HEAD
    // (see `skyblocker_reset_fields`): its `type` argument is moved into the body.
    report(hook::register_injector("net.minecraft.world.item.ItemStack#set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_reset_fields)));
}

#[cfg(test)]
mod perf_round_1_tests {
    use super::{UuidField, NO_EPOCH};

    /// Each half of the equipment memo answers only under the config epoch it was computed under, and a fresh field
    /// (a new uuid, or the field `set(CUSTOM_DATA)` replaced) carries none.
    #[test]
    fn a_memo_half_answers_only_under_its_own_config_epoch() {
        let field = UuidField::new("3f0e8f9e-0d6f-4d5e-9a33-4c3b0c1d2e3f");
        assert_eq!(&**field.uuid(), "3f0e8f9e-0d6f-4d5e-9a33-4c3b0c1d2e3f");
        assert!(field.armor_memo(7).is_none() && field.dye_memo(7).is_none(), "nothing memoised yet");
        assert!(field.armor_memo(NO_EPOCH).is_some_and(|key| key.is_none()), "the unset tag reads as an empty memo, which is why no epoch may be NO_EPOCH");
        field.set_armor_memo(7, None);
        assert!(matches!(field.armor_memo(7), Some(None)), "\"no custom model\" is a memoised answer");
        assert!(field.armor_memo(8).is_none(), "another config instance");
        field.set_dye_memo(7, true);
        assert_eq!(field.dye_memo(7), Some(true));
        assert_eq!(field.dye_memo(6), None);
        field.set_dye_memo(9, false);
        assert_eq!(field.dye_memo(9), Some(false));
        assert_eq!(field.dye_memo(7), None, "re-tagged");
        let fresh = UuidField::new("");
        assert!(fresh.armor_memo(7).is_none() && fresh.dye_memo(7).is_none());
    }
}
