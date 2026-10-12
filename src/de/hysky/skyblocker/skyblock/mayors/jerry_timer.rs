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
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			if (overlay || !isJerryActive || !SkyblockerConfigManager.get().helpers.jerry.enableJerryTimer) return true;
        //   java: 			String text = message.getString();
        //   java: 			//This part of hypixel still uses legacy text formatting, so we can't directly check for the actual text
        //   java: 			if (!text.startsWith("§b ☺ ") || !text.contains("Jerry")) return true;
        //   java: 			HoverEvent hoverEvent = message.getStyle().getHoverEvent();
        //   java: 			if (hoverEvent == null || hoverEvent.action() != HoverEvent.Action.SHOW_TEXT) return true;
        //   java: 			LocalPlayer player = Minecraft.getInstance().player;
        //   java: 			Scheduler.INSTANCE.schedule(() -> {
        //   java: 				if (player == null || !Utils.isOnSkyblock()) return;
        //   java: 				player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.helpers.jerry.sendJerryTimerMessage")).withStyle(ChatFormatting.GREEN));
        //   java: 				player.playSound(SoundEvents.VILLAGER_TRADE, 100f, 1.0f);
        //   java: 			}, 20 * 60 * 6); // 6 minutes
        //   java:
        //   java: 			return true;
        //   java: 		});
        skyblock_events::MAYOR_CHANGE.register(Arc::new(move || { compat::set_state(&IS_JERRY_ACTIVE, MayorUtils::get_active_perks().iter().any(|e| *e == "Jerrypocalypse")); }));
    }

}
