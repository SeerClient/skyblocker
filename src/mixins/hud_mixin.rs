//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.HudMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Hud)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private static final Supplier<Identifier> SLOT_LOCK_ICON = () -> SkyblockerConfigManager.get().general.itemProtection.slotLockStyle.tex;
fn slot_lock_icon() -> mcrust::net::minecraft::resources::identifier::Identifier {
    crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.item_protection.slot_lock_style.fields().tex.clone()
}
/// java: @Unique private boolean isQuiverSlot = false;
/// A field of the game's one `Hud`; the port's hotbar extraction is free functions with no `Hud` object to attach
/// it to, so it is this mixin's own cell.
pub static IS_QUIVER_SLOT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static PREV_HASH: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
pub static PREV_QUIVER_SLOT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// @Unique
fn is_quiver_item(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter as _;
    let mut hash_code: i32 = compat::identity_hash_code(stack);
    if hash_code == compat::get_state(&PREV_HASH) {
        return compat::get_state(&PREV_QUIVER_SLOT);
    }
    compat::set_state(&PREV_HASH, hash_code);
    let Some(component) = stack.get(mcrust::net::minecraft::core::component::data_components::custom_data()) else {
        return false;
    };
    let Some(component) = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::custom_data::CustomData>(&component) else {
        return false;
    };
    let compound: mcrust::net::minecraft::nbt::tag::Tag = component.copy_tag();
    let element: Option<&mcrust::net::minecraft::nbt::tag::Tag> = compound.get("quiver_arrow");
    compat::set_state(&PREV_QUIVER_SLOT, element.is_some_and(|element| element.as_boolean().unwrap_or(false) || element.as_string().unwrap_or("false") == "true"));
    return compat::get_state(&PREV_QUIVER_SLOT);
}

/// @Inject skyblocker$extractHotbarItemLockOrBackground
///   java: @Inject(method = "extractItemHotbar", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Hud;extractSlot(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/client/DeltaTracker;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;I)V", ordinal = 0))
/// target: net.minecraft.client.gui.Hud#extractSlot  [exists]
/// within: net.minecraft.client.gui.Hud#extractItemHotbar
/// Rust: net/minecraft/client/gui/hud.rs:2076 fn extract_slot(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, x: i32, y: i32, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, seed: i32, resolver: Option< &crate::net::minecraft::client::renderer::item::item_model_resolver::ItemModelResolver, >, ...)
fn skyblocker_extract_hotbar_item_lock_or_background(ci: &mut CallbackInfo<'_>) {
    // `ordinal = 0` is the loop's call (`:594-599`), for every hotbar slot; the offhand's (`:601-608`) is the next
    // call instruction. The port hands the loop's call `seed = i + 1` and the offhand's `seed = 10`
    // (`hud.rs:2001-2026`), so `@Local(name = "i") int index` is `seed - 1` for the nine loop calls.
    let seed: i32 = ci.arg::<i32>(5).copied().unwrap_or_default();
    if !(1..=9).contains(&seed) {
        return;
    }
    let index: i32 = seed - 1;
    let x: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    let y: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    // `player.getInventory().getNonEquipmentItems().get(index)` is the stack the call draws (argument 4,
    // `inventory.get_item(i)`).
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(4)) else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // rust: port gap - the port's hotbar draws through `gl` / `gfx` and the `HotbarItem` list, with no
        // `GuiGraphicsExtractor` to hand `ItemBackgroundManager`, `blitSprite` or `ItemProtection.drawSlotIcon`
        // (GAPS_batchD); the three draws are not made.
        //   java: ItemBackgroundManager.drawBackgrounds(player.getInventory().getNonEquipmentItems().get(index), graphics, x, y);
        // slot lock
        if crate::de::hysky::skyblocker::skyblock::item::hotbar_slot_lock::HotbarSlotLock::is_locked(index) {
            //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, SLOT_LOCK_ICON.get(), x, y, 16, 16);
            let _ = (slot_lock_icon, x, y);
        }
        //item protection
        if crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(stack.duplicate())) {
            //   java: ItemProtection.drawSlotIcon(graphics, x, y);
        }
        IS_QUIVER_SLOT.store(index == 8, std::sync::atomic::Ordering::Relaxed);
    }
}

/// @WrapOperation skyblocker$extractQuiverAmount
///   java: @WrapOperation(method = "extractSlot", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V"))
/// target: net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V  [exists]
/// within: net.minecraft.client.gui.Hud#extractSlot
/// Rust: net/minecraft/client/gui/gui_graphics_extractor.rs:154 fn item_decorations(gl: &Gl, gfx: &Minecraft, item_stack: &crate::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32, cooldowns: Option<&crate::net::minecraft::world::item::item_cooldowns::ItemCooldowns>, partial_tick: f32)
fn skyblocker_extract_quiver_amount(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(2)) else {
        original.call(ci);
        return;
    };
    let x: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let y: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.true_quiver_count && IS_QUIVER_SLOT.load(std::sync::atomic::Ordering::Relaxed) && is_quiver_item(stack) {
        let Some(mut arrow) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_lore_line_if(stack, Arc::new(move |s: String| compat::trim(&s).starts_with("Arrows Remaining"))) else {
            original.call(ci);
            return;
        };
        let mut i: i32 = compat::last_index_of(&arrow, ':');
        if i == -1 {
            original.call(ci);
            return;
        }
        arrow = compat::trim(&(compat::substring(&arrow, i.wrapping_add(1))).replace(",", ""));
        let mut an_int: Option<i32> = crate::de::hysky::skyblocker::utils::utils::Utils::parse_int(&arrow);
        let Some(an_int) = an_int else {
            original.call(ci);
            return;
        };
        let mut format: String = crate::de::hysky::skyblocker::utils::formatters::SHORT_INTEGER_NUMBERS.get_mut().format(an_int as f64);
        // `graphics.itemDecorations(textRenderer, stack, x, y, format)`: the five-argument overload, handed the call's
        // own frame (`gl`, `gfx`, `cooldowns`, `partial_tick` - arguments 0, 1, 5, 6).
        let (Some(gl), Some(gfx)) = (compat::hook_arg::<mcrust::lwjgl::opengl::Gl>(ci, 0), compat::hook_arg::<mcrust::net::minecraft::client::minecraft::Minecraft>(ci, 1)) else {
            original.call(ci);
            return;
        };
        let cooldowns: Option<&mcrust::net::minecraft::world::item::item_cooldowns::ItemCooldowns> = compat::hook_arg::<Option<&mcrust::net::minecraft::world::item::item_cooldowns::ItemCooldowns>>(ci, 5).copied().flatten();
        let partial_tick: f32 = ci.arg::<f32>(6).copied().unwrap_or_default();
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::item_decorations_with_count(gl, gfx, stack, x, y, Some(&format), cooldowns, partial_tick);
    } else {
        original.call(ci);
    }
}

/// @ModifyExpressionValue skyblocker$modifyAttackIndicatorCooldown
///   java: @ModifyExpressionValue(method = "extractCrosshair", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/LocalPlayer;getAttackStrengthScale(F)F"))
/// target: net.minecraft.world.entity.player.Player#getAttackStrengthScale  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.gui.Hud#extractCrosshair
/// note: `getAttackStrengthScale` is inherited: hooked on net.minecraft.world.entity.player.Player (it fires for every subclass - test the object's type in the handler if net.minecraft.client.player.LocalPlayer matters)
fn skyblocker_modify_attack_indicator_cooldown(ci: &mut CallbackInfo<'_>) {
    let Some(cooldown_progress) = ci.return_value::<f32>().copied() else { return; };
    // `minecraft.player` is the receiver of the call (the HUD reads the local player's scale), held by the game for
    // it - `this` here, never `with_player`.
    let Some(player) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::player::player::PlayerBase>()) else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // `getMainHandItem()` of a player is the inventory's selected item.
        let stack = player.get_inventory().get_selected_item();
        if crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::is_on_cooldown(stack) {
            ci.set_return_value(crate::de::hysky::skyblocker::skyblock::item::item_cooldowns::ItemCooldowns::get_item_cooldown_entry(stack).get_remaining_cooldown_percent());
            return;
        }
    }
    ci.set_return_value(cooldown_progress);
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
    // `ordinal = 0` (the loop's call instruction) - see the handler: selected by the call's `seed`, not by SeerClient's
    // run-time `ordinal(n)`, which would select only the FIRST slot.
    report(hook::call_site("net.minecraft.client.gui.Hud#extractItemHotbar", "net.minecraft.client.gui.Hud#extractSlot").priority(1000).inject(Shift::Before, skyblocker_extract_hotbar_item_lock_or_background));
    report(hook::call_site("net.minecraft.client.gui.Hud#extractSlot", "net.minecraft.client.gui.GuiGraphicsExtractor#itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V").priority(1000).wrap_operation(skyblocker_extract_quiver_amount));
    // @ModifyExpressionValue: runs after the call (it reads the call's receiver and replaces its value).
    report(hook::call_site("net.minecraft.client.gui.Hud#extractCrosshair", "net.minecraft.world.entity.player.Player#getAttackStrengthScale").priority(1000).inject(Shift::After, skyblocker_modify_attack_indicator_cooldown));
    report(hook::call_site("net.minecraft.client.gui.Hud#extractCameraOverlays", "net.minecraft.world.entity.Entity#getTicksFrozen").priority(1000).modify_expression_value::<i32>(skyblocker_hide_safari_cold_overlay));
}
