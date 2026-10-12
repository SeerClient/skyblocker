//! Ported from de.hysky.skyblocker.skyblock.garden.LowerSensitivity by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::entity::player::player::Player;

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
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of ClientLevel#tick (`net.minecraft.client.multiplayer.ClientLevel#tick`: ported but not hooked)
        //   java: ClientTickEvents.END_LEVEL_TICK.register(_ -> {
        //   java: 			if (Utils.getLocation() != Location.GARDEN || CLIENT.player == null || isInBarn(CLIENT.player) || !SkyblockerConfigManager.get().farming.mouseLock.lockMouseTool) {
        //   java: 				if (sensitivityLowered) lowerSensitivity(false);
        //   java: 				return;
        //   java: 			}
        //   java: 			ItemStack mainHandStack = CLIENT.player.getMainHandItem();
        //   java: 			String itemId = mainHandStack.getSkyblockId();
        //   java: 			boolean shouldLockMouse = FarmingHudWidget.FARMING_TOOLS.containsKey(itemId) && (!SkyblockerConfigManager.get().farming.mouseLock.lockMouseGroundOnly || CLIENT.player.onGround());
        //   java: 			if (shouldLockMouse && !sensitivityLowered) lowerSensitivity(true);
        //   java: 			else if (!shouldLockMouse && sensitivityLowered) lowerSensitivity(false);
        //   java: 		});
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
