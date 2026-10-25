//! Ported from de.hysky.skyblocker.skyblock.garden.LowerSensitivity by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::entity::player::player::Player;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::garden::farming_hud_widget::FARMING_TOOLS;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static boolean sensitivityLowered = false;
pub static SENSITIVITY_LOWERED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class LowerSensitivity
#[derive(Clone)]
pub struct LowerSensitivity {}

impl LowerSensitivity {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientTickEvents.END_LEVEL_TICK.register(_ -> {
        // java: 	if (Utils.getLocation() != Location.GARDEN || CLIENT.player == null || isInBarn(CLIENT.player) || !SkyblockerConfigManager.get().farming.mouseLock.lockMouseTool) {
        // java: 		if (sensitivityLowered) lowerSensitivity(false);
        // java: 		return;
        // java: 	}
        // java: 	ItemStack mainHandStack = CLIENT.player.getMainHandItem();
        // java: 	String itemId = mainHandStack.getSkyblockId();
        // java: 	boolean shouldLockMouse = FarmingHudWidget.FARMING_TOOLS.containsKey(itemId) && (!SkyblockerConfigManager.get().farming.mouseLock.lockMouseGroundOnly || CLIENT.player.onGround());
        // java: 	if (shouldLockMouse && !sensitivityLowered) lowerSensitivity(true);
        // java: 	else if (!shouldLockMouse && sensitivityLowered) lowerSensitivity(false);
        // java: });
        // (Fabric fires END_LEVEL_TICK at the end of `ClientLevel.tick`, inside `Minecraft.tick`, while a level exists; the
        // port runs that tick's statements inline in `Minecraft.tick` - no hooked `ClientLevel#tick` - so the listener runs
        // at the end of the same client tick, with the same level guard.)
        mcrust::mcrust::modding::events::client_tick_events::END_CLIENT_TICK.register(Arc::new(|_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft| {
            if mcrust::mcrust::modding::game::level().is_none() {
                return;
            }
            // (`CLIENT.player == null || isInBarn(CLIENT.player)`, and the main hand's id and `onGround()`, read in one visit)
            let player_state: Option<(bool, String, bool)> = mcrust::mcrust::modding::game::with_player(|player| {
                (LowerSensitivity::is_in_barn(&*player), player.base.base.base.get_main_hand_item().get_skyblock_id(), player.base.base.base.base.on_ground())
            });
            let Some((in_barn, item_id, on_ground)) = player_state.filter(|_| Utils::get_location() == Location::Garden && SkyblockerConfigManager::get().farming.mouse_lock.lock_mouse_tool) else {
                if compat::get_state(&SENSITIVITY_LOWERED) {
                    LowerSensitivity::lower_sensitivity(false);
                }
                return;
            };
            if in_barn {
                if compat::get_state(&SENSITIVITY_LOWERED) {
                    LowerSensitivity::lower_sensitivity(false);
                }
                return;
            }
            let should_lock_mouse: bool = FARMING_TOOLS.contains_key(&item_id) && (!(SkyblockerConfigManager::get().farming.mouse_lock.lock_mouse_ground_only) || on_ground);
            if should_lock_mouse && !compat::get_state(&SENSITIVITY_LOWERED) {
                LowerSensitivity::lower_sensitivity(true);
            } else if !should_lock_mouse && compat::get_state(&SENSITIVITY_LOWERED) {
                LowerSensitivity::lower_sensitivity(false);
            }
        }));
    }

    /// java: private static boolean isInBarn(Player player)
    pub fn is_in_barn(player: &dyn Player) -> bool {
        return (player.base().get_x() <= 35.5 && player.base().get_x() >= -32.5 && player.base().get_z() <= -4.5 && player.base().get_z() >= -46.5);
    }

    /// java: public static void lowerSensitivity(boolean lowerSensitivity)
    pub fn lower_sensitivity(mut lower_sensitivity: bool) {
        compat::set_state(&SENSITIVITY_LOWERED, lower_sensitivity);
    }

    /// java: public static boolean isSensitivityLowered()
    pub fn is_sensitivity_lowered() -> bool {
        return compat::get_state(&SENSITIVITY_LOWERED);
    }

}
