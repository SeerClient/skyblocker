//! Ported from de.hysky.skyblocker.skyblock.mayors.JerryTimer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::mayor::mayor_utils::MayorUtils;

/// java: private static boolean isJerryActive = false;
pub static IS_JERRY_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public final class JerryTimer
#[derive(Clone)]
pub struct JerryTimer {}

impl JerryTimer {
    /// java: private JerryTimer()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> { .. });
        compat::register_allow_game(|message, overlay| {
            if overlay || !compat::get_state(&IS_JERRY_ACTIVE) || !crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().helpers.jerry.enable_jerry_timer {
                return true;
            }
            let text: String = mcrust::net::minecraft::network::chat::component::Component::get_string(message);
            // This part of hypixel still uses legacy text formatting, so we can't directly check for the actual text
            if !text.starts_with("\u{a7}b \u{263a} ") || !text.contains("Jerry") {
                return true;
            }
            // java: HoverEvent hoverEvent = message.getStyle().getHoverEvent(); if (hoverEvent == null || hoverEvent.action() != HoverEvent.Action.SHOW_TEXT) return true;
            if !matches!(mcrust::net::minecraft::network::chat::component::Component::get_style(message).get_hover_event(), Some(mcrust::net::minecraft::network::chat::hover_event::HoverEvent::ShowText { .. })) {
                return true;
            }
            // java: LocalPlayer player = Minecraft.getInstance().player; (captured: its `player == null` is answered now)
            let has_player: bool = mcrust::mcrust::modding::game::with_player(|_| ()).is_some();
            ({ let __r = crate::de::hysky::skyblocker::utils::scheduler::scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
                if !has_player || !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
                    return;
                }
                // java: player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.helpers.jerry.sendJerryTimerMessage")).withStyle(ChatFormatting.GREEN));
                compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.config.helpers.jerry.sendJerryTimerMessage")).with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Green));
                // java: player.playSound(SoundEvents.VILLAGER_TRADE, 100f, 1.0f);
                mcrust::mcrust::modding::game::with_player(|player| { mcrust::net::minecraft::world::entity::living_entity::LivingEntity::play_sound(player, mcrust::net::minecraft::sounds::sound_events::villager_trade().clone(), 100.0f32, 1.0f32); });
            }), 20 * 60 * 6) }); // 6 minutes
            true
        });
        skyblock_events::MAYOR_CHANGE.register(Arc::new(move || { compat::set_state(&IS_JERRY_ACTIVE, MayorUtils::get_active_perks().iter().any(|e| *e == "Jerrypocalypse")); }));
    }

}
