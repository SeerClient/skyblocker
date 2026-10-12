//! Ported from de.hysky.skyblocker.compatibility.rei.SkyblockerREICommonPlugin by fabric2seer. Entrypoint: rei_common.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;


pub fn register_item_comparators(registry: &crate::unresolved::ItemComparatorRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
        return;
    }
    // TODO(fabric2seer): `.registerGlobal(..)` on a ItemComparatorRegistry
    //   java: registry.registerGlobal(new SkyblockItemComparator());
}
