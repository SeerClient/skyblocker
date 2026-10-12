//! Ported from de.hysky.skyblocker.skyblock.SwingAnimation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::item_ability::ItemAbility;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class SwingAnimation
#[derive(Clone)]
pub struct SwingAnimation {}

impl SwingAnimation {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean hasAbility(ItemStack stack)
    pub fn has_ability(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut abilities: Vec<ItemAbility> = stack.skyblocker_get_abilities();
        for ability in abilities.iter() {
            if ability.activation().is_right_click() {
                return true;
            }
        }
        return false;
    }

}
