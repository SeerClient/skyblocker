//! Ported from de.hysky.skyblocker.injected.SkyblockerStack by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::utils::item_ability::ItemAbility;

/// java: public interface SkyblockerStack
pub trait SkyblockerStack {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: default String getSkyblockId()
    fn get_skyblock_id(&self) -> String {
        return "".to_string();
    }

    /// java: default String getSkyblockApiId()
    fn get_skyblock_api_id(&self) -> String {
        return "".to_string();
    }

    /// java: default String getNeuName()
    fn get_neu_name(&self) -> String {
        return "".to_string();
    }

    /// java: default String getUuid()
    fn get_uuid(&self) -> String {
        return "".to_string();
    }

    /// java: default List<String> skyblocker$getLoreStrings()
    fn skyblocker_get_lore_strings(&self) -> Vec<String> {
        return vec![];
    }

    /// java: default List<ItemAbility> skyblocker$getAbilities()
    fn skyblocker_get_abilities(&self) -> Vec<ItemAbility> {
        return vec![];
    }

    /// java: default PetInfo getPetInfo()
    fn get_pet_info(&self) -> PetInfo {
        return pet_info::EMPTY.clone();
    }

    /// java: default SkyblockItemRarity getSkyblockRarity()
    fn get_skyblock_rarity(&self) -> SkyblockItemRarity {
        return SkyblockItemRarity::Unknown;
    }

}
