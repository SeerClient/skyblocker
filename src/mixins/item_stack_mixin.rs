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
pub static ATTACH_DURABILITY_BAR_FILL: AttachmentType<f32> = AttachmentType::new("skyblocker:item_stack_mixin_durability_bar_fill");
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
pub static ATTACH_UUID: AttachmentType<String> = AttachmentType::new("skyblocker:item_stack_mixin_uuid");
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

/// @Unique
fn skyblocker_should_process(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    // java: `Minecraft.getInstance() != null && Minecraft.getInstance().gui != null`: the game's one Minecraft and
    // its `Gui` always exist here; `current_screen_is` answers false before there is a screen.
    return (!(mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>()) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.enable_drill_fuel && crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::has_custom_durability(this));
}

/// @Unique
fn skyblocker_get_and_cache_durability(this: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
    if !(skyblocker_should_process(this)) {
        this.set_attached(&ATTACH_DURABILITY_BAR_FILL, ((-1) as f32) as f32);
        return;
    }
    let Some(mut durability) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_durability(&this) else {
        this.set_attached(&ATTACH_DURABILITY_BAR_FILL, ((-1) as f32) as f32);
        return;
    };
    this.set_attached(&ATTACH_DURABILITY_BAR_FILL, (((durability.0) as f32) / (durability.1 as f32)));
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
    if let Some(uuid) = this.get_attached(&ATTACH_UUID) {
        return uuid;
    }
    // java: return uuid = ItemUtils.getItemUuid(this);
    let uuid = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_item_uuid(this);
    this.set_attached(&ATTACH_UUID, uuid.clone());
    return uuid;
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
        ci.set_return_value(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_item_names.get(&get_uuid(this)).cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).unwrap_or(original));
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
    // rust: port gap - the injection point (`Consumer.accept` after the item-id line of `addDetailsToTooltip:924`)
    // is a call of the `builder` closure, which is not a hooked method (GAPS_batchD); `this` is the stack.
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut skyblock_id: String = get_skyblock_id(this);
        if !skyblock_id.is_empty() {
            // TODO(fabric2seer): `textConsumer`: this argument is not visible to the injector (see the notes above the handler)
            //   java: textConsumer.accept(Component.literal("skyblock:" + skyblockId).withStyle(ChatFormatting.DARK_GRAY));
        }
    }
}

/// @Inject skyblocker$updateDamage
///   java: @Inject(method = "inventoryTick", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#inventoryTick  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:707 fn inventory_tick(&mut self)
fn skyblocker_update_damage(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    skyblocker_get_and_cache_durability(this);
}

/// @ModifyReturnValue modifyItemBarVisible
///   java: @ModifyReturnValue(method = "isBarVisible", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#isBarVisible  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:814 fn is_bar_visible(&self)
fn modify_item_bar_visible(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    ci.set_return_value((original || this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32) >= 0.0f32));
    return;
}

/// @ModifyReturnValue modifyItemBarStep
///   java: @ModifyReturnValue(method = "getBarWidth", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarWidth  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:820 fn get_bar_width(&self)
fn modify_item_bar_step(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    ci.set_return_value((if this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32) >= (0 as f32) { (((this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32) * (13 as f32))) as i32) } else { original }));
    return;
}

/// @ModifyReturnValue modifyItemBarColor
///   java: @ModifyReturnValue(method = "getBarColor", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarColor  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:826 fn get_bar_color(&self)
fn modify_item_bar_color(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    ci.set_return_value((if this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32) >= (0 as f32) { crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor::interpolate(mcrust::net::minecraft::util::common_colors::RED, mcrust::net::minecraft::util::common_colors::GREEN, this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1.0f32)) } else { original }));
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
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.util.function.Consumer has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.world.item.ItemStack#addDetailsToTooltip`, `java.util.function.Consumer#accept(Ljava/lang/Object;)V`).ordinal(0).priority(1000).inject(Shift::After, skyblocker_skyblock_id_tooltip));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#inventoryTick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_damage)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#isBarVisible", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_visible)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarWidth", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_step)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarColor", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_color)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_init)));
    // java: `method = "set*"` with a `(DataComponentType<T>, T)` handler: the one `set` overload of that shape. At HEAD
    // (see `skyblocker_reset_fields`): its `type` argument is moved into the body.
    report(hook::register_injector("net.minecraft.world.item.ItemStack#set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_reset_fields)));
}
