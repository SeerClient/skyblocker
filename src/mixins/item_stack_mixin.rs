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
fn skyblocker_should_process(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    return (!(true /* java: compat::minecraft() != null: never null here */ && true /* java: compat::minecraft().gui != null: never null here */ && mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>()) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.enable_drill_fuel && crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::has_custom_durability(&this));
}

/// @Unique
fn skyblocker_get_and_cache_durability(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) {
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
fn get_skyblock_id(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if this.get_attached(&ATTACH_SKYBLOCK_ID).is_some() && !(this.get_attached(&ATTACH_SKYBLOCK_ID).unwrap().is_empty()) {
        return this.get_attached(&ATTACH_SKYBLOCK_ID).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return skyblockId = ItemUtils.getItemId(this);
}

/// @SuppressWarnings("deprecation")
fn get_skyblock_api_id(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if this.get_attached(&ATTACH_SKYBLOCK_API_ID).is_some() && !(this.get_attached(&ATTACH_SKYBLOCK_API_ID).unwrap().is_empty()) {
        return this.get_attached(&ATTACH_SKYBLOCK_API_ID).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return skyblockApiId = ItemUtils.getSkyblockApiId(this);
}

/// @SuppressWarnings("deprecation")
fn get_neu_name(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if this.get_attached(&ATTACH_NEU_NAME).is_some() && !(this.get_attached(&ATTACH_NEU_NAME).unwrap().is_empty()) {
        return this.get_attached(&ATTACH_NEU_NAME).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return neuName = ItemUtils.getNeuId((ItemStack) (Object) this);
}

/// @SuppressWarnings("deprecation")
fn get_uuid(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
    if this.get_attached(&ATTACH_UUID).is_some() {
        return this.get_attached(&ATTACH_UUID).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return uuid = ItemUtils.getItemUuid(this);
}

/// @SuppressWarnings("deprecation")
fn skyblocker_get_lore_strings(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> /* TODO: List<String> */ () {
    if this.get_attached(&ATTACH_LORE_STRING).is_some() {
        return this.get_attached(&ATTACH_LORE_STRING).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return loreString = ItemUtils.getLore((ItemStack) (Object) this).stream().map(Component::getString).toList();
}

/// @SuppressWarnings("deprecation")
fn skyblocker_get_abilities(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> /* TODO: List<ItemAbility> */ () {
    if this.get_attached(&ATTACH_ABILITIES).is_some() {
        return this.get_attached(&ATTACH_ABILITIES).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return abilities = ItemAbility.getAbilities((ItemStack) (Object) this);
}

/// @SuppressWarnings("deprecation")
fn get_pet_info(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> /* TODO: PetInfo */ () {
    if this.get_attached(&ATTACH_PET_INFO).is_some() {
        return this.get_attached(&ATTACH_PET_INFO).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return petInfo = ItemUtils.getPetInfo((ItemStack) (Object) this);
}

/// @SuppressWarnings("deprecation")
fn get_skyblock_rarity(this: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> /* TODO: SkyblockItemRarity */ () {
    if this.get_attached(&ATTACH_SKYBLOCK_RARITY).is_some() {
        return this.get_attached(&ATTACH_SKYBLOCK_RARITY).unwrap();
    }
    // TODO(fabric2seer): an assignment used as a value
    //   java: return skyblockRarity = ItemUtils.getItemRarity((ItemStack) (Object) this);
}

/// @ModifyReturnValue skyblocker$customItemNames
///   java: @ModifyReturnValue(method = "getHoverName", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getHoverName  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:986 fn get_hover_name(&self)
fn skyblocker_custom_item_names(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (Component)
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        ci.set_return_value(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_item_names.get(&this.get_uuid()).cloned().unwrap_or_else(|| original.clone()));
        return;
    }
    ci.set_return_value(original.clone());
    return;
}

/// @ModifyExpressionValue skyblocker$hideVanillaEnchants
///   java: @ModifyExpressionValue(method = "addToTooltip", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/item/component/TooltipDisplay;shows(Lnet/minecraft/core/component/DataComponentType;)Z"))
/// target: net.minecraft.world.item.component.TooltipDisplay#shows  [exists]
/// within: net.minecraft.world.item.ItemStack#addToTooltip
/// Rust: net/minecraft/world/item/component/tooltip_display.rs:98 fn shows(&self, component: &Arc<DataComponentType>)
fn skyblocker_hide_vanilla_enchants(v: bool) -> bool {
    let mut should_display = v;
    // TODO(fabric2seer): `component`: this argument is not visible to the injector (see the notes above the handler)
    //   java: return shouldDisplay && !(Utils.isOnSkyblock() && component instanceof ItemEnchantments);
    todo!("fabric2seer: the value of this function is in a TODO above")
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
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut skyblock_id: String = get_skyblock_id();
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
    ci.set_return_value((original || this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1) >= 0.0f32));
    return;
}

/// @ModifyReturnValue modifyItemBarStep
///   java: @ModifyReturnValue(method = "getBarWidth", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarWidth  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:820 fn get_bar_width(&self)
fn modify_item_bar_step(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    ci.set_return_value((if this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1) >= (0 as f32) { (((this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1) * (13 as f32))) as i32) } else { original }));
    return;
}

/// @ModifyReturnValue modifyItemBarColor
///   java: @ModifyReturnValue(method = "getBarColor", at = @At("RETURN"))
/// target: net.minecraft.world.item.ItemStack#getBarColor  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:826 fn get_bar_color(&self)
fn modify_item_bar_color(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let original: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    ci.set_return_value((if this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1) >= (0 as f32) { crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor::interpolate(mcrust::net::minecraft::util::common_colors::RED, mcrust::net::minecraft::util::common_colors::GREEN, this.get_attached(&ATTACH_DURABILITY_BAR_FILL).unwrap_or(-1)) } else { original }));
    return;
}

/// @Inject onInit
///   java: @Inject(method = "<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V  [exists]
/// Rust: net/minecraft/world/item/item_stack.rs:223 fn new_with_components(item: Arc<Item>, count: i32, components: PatchedDataComponentMap)
fn on_init(ci: &mut CallbackInfo<'_>) {
    skyblocker_get_and_cache_durability();
}

/// @Inject skyblocker$resetFields
///   java: @Inject(method = "set*", at = @At("TAIL"))
/// target: net.minecraft.world.item.ItemStack#set  [exists with a different descriptor]
/// Rust: net/minecraft/world/item/item_stack.rs:453 fn set(&mut self, ty: Arc<DataComponentType>, value: Option<Obj>)
/// note: target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;, (Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Object;; the mod gave no descriptor; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
fn skyblocker_reset_fields(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    // TODO(fabric2seer): `value` is Rust argument 1: `Option<Obj>` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(1) }`
    // TODO(fabric2seer): the condition: `type`: this argument is not visible to the injector (see the notes above the handler)
    //   java: type == DataComponents.CUSTOM_DATA
    if todo!("fabric2seer: the condition (TODO above)") {
        this.remove_attached(&ATTACH_UUID);
        this.remove_attached(&ATTACH_SKYBLOCK_ID);
        this.remove_attached(&ATTACH_SKYBLOCK_API_ID);
        this.remove_attached(&ATTACH_NEU_NAME);
        this.remove_attached(&ATTACH_PET_INFO);
    }
    // TODO(fabric2seer): the condition: `type`: this argument is not visible to the injector (see the notes above the handler)
    //   java: type == DataComponents.LORE
    if todo!("fabric2seer: the condition (TODO above)") {
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
        if self.get_attached(&ATTACH_SKYBLOCK_ID).is_some() && !(self.get_attached(&ATTACH_SKYBLOCK_ID).unwrap().is_empty()) {
            return self.get_attached(&ATTACH_SKYBLOCK_ID).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return skyblockId = ItemUtils.getItemId(this);
    }

    fn get_skyblock_api_id(&self) -> String {
        if self.get_attached(&ATTACH_SKYBLOCK_API_ID).is_some() && !(self.get_attached(&ATTACH_SKYBLOCK_API_ID).unwrap().is_empty()) {
            return self.get_attached(&ATTACH_SKYBLOCK_API_ID).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return skyblockApiId = ItemUtils.getSkyblockApiId(this);
    }

    fn get_neu_name(&self) -> String {
        if self.get_attached(&ATTACH_NEU_NAME).is_some() && !(self.get_attached(&ATTACH_NEU_NAME).unwrap().is_empty()) {
            return self.get_attached(&ATTACH_NEU_NAME).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return neuName = ItemUtils.getNeuId((ItemStack) (Object) this);
    }

    fn get_uuid(&self) -> String {
        if self.get_attached(&ATTACH_UUID).is_some() {
            return self.get_attached(&ATTACH_UUID).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return uuid = ItemUtils.getItemUuid(this);
    }

    fn skyblocker_get_lore_strings(&self) -> Vec<String> {
        if self.get_attached(&ATTACH_LORE_STRING).is_some() {
            return self.get_attached(&ATTACH_LORE_STRING).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return loreString = ItemUtils.getLore((ItemStack) (Object) this).stream().map(Component::getString).toList();
    }

    fn skyblocker_get_abilities(&self) -> Vec<ItemAbility> {
        if self.get_attached(&ATTACH_ABILITIES).is_some() {
            return self.get_attached(&ATTACH_ABILITIES).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return abilities = ItemAbility.getAbilities((ItemStack) (Object) this);
    }

    fn get_pet_info(&self) -> PetInfo {
        if self.get_attached(&ATTACH_PET_INFO).is_some() {
            return self.get_attached(&ATTACH_PET_INFO).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return petInfo = ItemUtils.getPetInfo((ItemStack) (Object) this);
    }

    fn get_skyblock_rarity(&self) -> SkyblockItemRarity {
        if self.get_attached(&ATTACH_SKYBLOCK_RARITY).is_some() {
            return self.get_attached(&ATTACH_SKYBLOCK_RARITY).unwrap();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return skyblockRarity = ItemUtils.getItemRarity((ItemStack) (Object) this);
    }

}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getHoverName", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_item_names)));
    report(hook::call_site("net.minecraft.world.item.ItemStack#addToTooltip", "net.minecraft.world.item.component.TooltipDisplay#shows").priority(1000).modify_expression_value::<bool>(skyblocker_hide_vanilla_enchants));
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.util.function.Consumer has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.world.item.ItemStack#addDetailsToTooltip`, `java.util.function.Consumer#accept(Ljava/lang/Object;)V`).ordinal(0).priority(1000).inject(Shift::After, skyblocker_skyblock_id_tooltip));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#inventoryTick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_damage)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#isBarVisible", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_visible)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarWidth", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_step)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#getBarColor", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_item_bar_color)));
    report(hook::register_injector("net.minecraft.world.item.ItemStack#<init>(Lnet/minecraft/core/Holder;ILnet/minecraft/core/component/PatchedDataComponentMap;)V", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_init)));
    // TODO(fabric2seer): not registered - target hooked only with another descriptor - hooked overloads: (Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;, (Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Object;; the mod gave no descriptor; the registration (with the mod's own descriptor) is left commented out until that overload is hooked
    // report(hook::register_injector(`net.minecraft.world.item.ItemStack#set`, InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_reset_fields)));
}
