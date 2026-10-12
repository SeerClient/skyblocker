//! Ported from de.hysky.skyblocker.skyblock.item.ConsumableProtection by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::level::Level;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::collision_getter::CollisionGetter as _;

/// java: private static final Set<String> PROTECTED_CONSUMABLES = Set.of("NEW_BOTTLE_OF_JYRRE", "DARK_CACAO_TRUFFLE", "DISCRITE", "MOBY_DUCK", "ROSEWATER_FLASK");
pub static PROTECTED_CONSUMABLES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["NEW_BOTTLE_OF_JYRRE".to_string(), "DARK_CACAO_TRUFFLE".to_string(), "DISCRITE".to_string(), "MOBY_DUCK".to_string(), "ROSEWATER_FLASK".to_string()]));

/// java: public class ConsumableProtection
#[derive(Clone)]
pub struct ConsumableProtection {}

impl ConsumableProtection {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): UseItemCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseItemCallback.EVENT.register(ConsumableProtection::onInteract);
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register((player, world, hand, _) -> onInteract(player, world, hand));
    }

    /// java: private static InteractionResult onInteract(Player player, Level world, InteractionHand hand)
    pub fn on_interact(player: &dyn Player, world: &Level, hand: &InteractionHand) -> InteractionResult {
        if world.is_client_side_level() && SkyblockerConfigManager::get().general.item_protection.protect_valuable_consumables && Utils::is_on_skyblock() {
            let stack = player.get_item_in_hand(*hand);
            let mut skyblock_id: String = stack.get_skyblock_id();
            if !skyblock_id.is_empty() && PROTECTED_CONSUMABLES.contains(&skyblock_id) {
                return InteractionResult::Fail;
            }
        }
        return InteractionResult::Pass;
    }

}
