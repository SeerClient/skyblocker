//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.HudMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Hud)

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): state `SLOT_LOCK_ICON` of type `Supplier<Identifier>` - give it a Rust type (`static SLOT_LOCK_ICON: LazyLock<Mutex<T>>`)
/// java: @Unique private boolean isQuiverSlot = false;
/// One value per Hud OBJECT (a Mixin field): an attachment.
pub static ATTACH_IS_QUIVER_SLOT: AttachmentType<bool> = AttachmentType::new("skyblocker:hud_mixin_is_quiver_slot");
pub static PREV_HASH: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
pub static PREV_QUIVER_SLOT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// @Unique
fn is_quiver_item(this: &mut mcrust::net::minecraft::client::gui::hud::Hud, stack: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    // TODO(fabric2seer): static call System.identityHashCode(..)
    //   java: int hashCode = System.identityHashCode(stack);
    let mut hash_code: i32 = todo!("fabric2seer: the initialiser of `hashCode` (TODO above)");
    if hash_code == compat::get_state(&PREV_HASH) {
        return compat::get_state(&PREV_QUIVER_SLOT);
    }
    compat::set_state(&PREV_HASH, hash_code);
    // TODO(fabric2seer): `.get(..)` on a ItemStack
    //   java: CustomData component = stack.get(DataComponents.CUSTOM_DATA);
    let mut component: mcrust::net::minecraft::world::item::component::custom_data::CustomData = todo!("fabric2seer: the initialiser of `component` (TODO above)");
    if false /* java: component == null: never null here */ {
        return false;
    }
    let mut compound: mcrust::net::minecraft::nbt::tag::Tag = component.copy_tag();
    let mut element: mcrust::net::minecraft::nbt::tag::Tag = compound.get("quiver_arrow").unwrap();
    // TODO(fabric2seer): Optional.equals
    //   java: prevQuiverSlot = element != null && (element.asBoolean().orElse(false) || element.asString().orElse("false").equals("true"));
    return compat::get_state(&PREV_QUIVER_SLOT);
}

/// @Inject skyblocker$extractHotbarItemLockOrBackground
///   java: @Inject(method = "extractItemHotbar", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Hud;extractSlot(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/client/DeltaTracker;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;I)V", ordinal = 0))
/// target: net.minecraft.client.gui.Hud#extractSlot  [exists]
/// within: net.minecraft.client.gui.Hud#extractItemHotbar
/// Rust: net/minecraft/client/gui/hud.rs:2076 fn extract_slot(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, x: i32, y: i32, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, seed: i32, resolver: Option< &crate::net::minecraft::client::renderer::item::item_model_resolver::ItemMode
/// note: ordinal 0: the call is in a loop in the Java of the method, so the run-time count (per run of the method) may differ from Mixin's bytecode ordinal - check it
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_extract_hotbar_item_lock_or_background(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // TODO(fabric2seer): `player`: this argument is not visible to the injector (see the notes above the handler)
        //   java: ItemBackgroundManager.drawBackgrounds(player.getInventory().getNonEquipmentItems().get(index), graphics, x, y);
        // TODO(fabric2seer): the condition: `index`: this argument is not visible to the injector (see the notes above the handler)
        //   java: HotbarSlotLock.isLocked(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
            //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, SLOT_LOCK_ICON.get(), x, y, 16, 16);
        }
        // TODO(fabric2seer): the condition: `player`: this argument is not visible to the injector (see the notes above the handler)
        //   java: ItemProtection.isItemProtected(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
            //   java: ItemProtection.drawSlotIcon(graphics, x, y);
        }
        // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
        //   java: isQuiverSlot = index == 8;
    }
}

/// @WrapOperation skyblocker$extractQuiverAmount
///   java: @WrapOperation(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V  [exists]
/// within: net.minecraft.client.gui.Hud#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:154 fn item_decorations(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
fn skyblocker_extract_quiver_amount(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(2)) else { return; };
    let x: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let y: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: Utils.isOnSkyblock() && SkyblockerConfigManager.get().uiAndVisuals.trueQuiverCount && isQuiverSlot && isQuiverItem(stack)
    if todo!("fabric2seer: the condition (TODO above)") {
        let Some(mut arrow) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_lore_line_if(&stack, Arc::new(move |s| compat::trim(&s).starts_with("Arrows Remaining"))) else {
            // TODO(fabric2seer): unknown name `graphics`
            //   java: original.call(graphics, textRenderer, stack, x, y);
            return;
        };
        let mut i: i32 = compat::last_index_of(&arrow, ':');
        if i == -1 {
            // TODO(fabric2seer): unknown name `graphics`
            //   java: original.call(graphics, textRenderer, stack, x, y);
            return;
        }
        arrow = compat::trim(&(compat::substring(&arrow, i.wrapping_add(1))).replace(",", ""));
        let mut an_int: Option<i32> = crate::de::hysky::skyblocker::utils::utils::Utils::parse_int(&arrow);
        if an_int.is_none() {
            // TODO(fabric2seer): unknown name `graphics`
            //   java: original.call(graphics, textRenderer, stack, x, y);
            return;
        }
        let mut format: String = crate::de::hysky::skyblocker::utils::formatters::SHORT_INTEGER_NUMBERS.get_mut().format((an_int.clone().unwrap()) as f64);
        // TODO(fabric2seer): unknown name `graphics`
        //   java: graphics.itemDecorations(textRenderer, stack, x, y, format);
    } else {
        // TODO(fabric2seer): unknown name `graphics`
        //   java: original.call(graphics, textRenderer, stack, x, y);
    }
}

/// @ModifyExpressionValue skyblocker$modifyAttackIndicatorCooldown
///   java: @ModifyExpressionValue(method = "extractCrosshair", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/LocalPlayer;getAttackStrengthScale(F)F"))
/// target: net.minecraft.world.entity.player.Player#getAttackStrengthScale  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.gui.Hud#extractCrosshair
/// note: `getAttackStrengthScale` is inherited: hooked on net.minecraft.world.entity.player.Player (it fires for every subclass - test the object's type in the handler if net.minecraft.client.player.LocalPlayer matters)
fn skyblocker_modify_attack_indicator_cooldown(v: f32) -> f32 {
    let mut cooldown_progress = v;
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: Utils.isOnSkyblock() && minecraft.player != null
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
        //   java: ItemStack stack = minecraft.player.getMainHandItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        if crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::is_on_cooldown(&stack) {
            return crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::get_item_cooldown_entry(&stack).get_remaining_cooldown_percent();
        }
    }
    return cooldown_progress;
}

/// @ModifyExpressionValue skyblocker$hideSafariColdOverlay
///   java: @ModifyExpressionValue(method = "extractCameraOverlays", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/LocalPlayer;getTicksFrozen()I"))
/// target: net.minecraft.world.entity.Entity#getTicksFrozen  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.gui.Hud#extractCameraOverlays
/// note: `getTicksFrozen` is inherited: hooked on net.minecraft.world.entity.Entity (it fires for every subclass - test the object's type in the handler if net.minecraft.client.player.LocalPlayer matters)
fn skyblocker_hide_safari_cold_overlay(v: i32) -> i32 {
    let mut original = v;
    return (if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::skyblock::hunting::safari::safari_utils::SafariUtils::is_in_icy_biome() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().hunting.icy_biome.hide_cold_overlay { 0 } else { original });
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.gui.Hud#extractItemHotbar", "net.minecraft.client.gui.Hud#extractSlot").ordinal(0).priority(1000).inject(Shift::Before, skyblocker_extract_hotbar_item_lock_or_background));
    report(hook::call_site("net.minecraft.client.gui.Hud#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V").priority(1000).wrap_operation(skyblocker_extract_quiver_amount));
    report(hook::call_site("net.minecraft.client.gui.Hud#extractCrosshair", "net.minecraft.world.entity.player.Player#getAttackStrengthScale").priority(1000).modify_expression_value::<f32>(skyblocker_modify_attack_indicator_cooldown));
    report(hook::call_site("net.minecraft.client.gui.Hud#extractCameraOverlays", "net.minecraft.world.entity.Entity#getTicksFrozen").priority(1000).modify_expression_value::<i32>(skyblocker_hide_safari_cold_overlay));
}
