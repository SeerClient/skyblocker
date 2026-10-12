//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Inventories by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Inventories
#[derive(Clone)]
pub struct Inventories {
    /// java: @SerializedName("bag_contents") public BagContents bagContents = new BagContents();
    pub bag_contents: BagContents,
    /// java: @SerializedName("inv_contents") public @Nullable AbstractInventoryContents inventoryContents;
    pub inventory_contents: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("ender_chest_contents") public @Nullable AbstractInventoryContents enderChestContents;
    pub ender_chest_contents: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("backpack_icons") public Map<String, AbstractInventoryContents> backpackIcons = Map.of();
    pub backpack_icons: std::collections::HashMap<String, compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("backpack_contents") public Map<String, AbstractInventoryContents> backpackContents = Map.of();
    pub backpack_contents: std::collections::HashMap<String, compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("inv_armor") public @Nullable AbstractInventoryContents armourContents;
    pub armour_contents: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("equipment_contents") public @Nullable AbstractInventoryContents equipmentContents;
    pub equipment_contents: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("personal_vault_contents") public @Nullable AbstractInventoryContents personalVaultContents;
    pub personal_vault_contents: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("sacks_counts") public @Nullable Map<String, Integer> sacksCounts;
    pub sacks_counts: Option<std::collections::HashMap<String, i32>>,
}

impl Inventories {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            bag_contents: BagContents::new(),
            inventory_contents: None,
            ender_chest_contents: None,
            backpack_icons: std::collections::HashMap::from([]),
            backpack_contents: std::collections::HashMap::from([]),
            armour_contents: None,
            equipment_contents: None,
            personal_vault_contents: None,
            sacks_counts: None,
        };
        this
    }

}

/// java: public static class AbstractInventoryContents
#[derive(Clone)]
pub struct AbstractInventoryContents {
    /// java: public String data = "";
    pub data: String,
}

impl AbstractInventoryContents {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            data: "".to_string(),
        });
        __this
    }

}

/// java: public static class BagContents
#[derive(Clone)]
pub struct BagContents {
    /// java: @SerializedName("sacks_bag") public @Nullable AbstractInventoryContents sacksBag;
    pub sacks_bag: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("potion_bag") public @Nullable AbstractInventoryContents potionBag;
    pub potion_bag: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("talisman_bag") public @Nullable AbstractInventoryContents talismanBag;
    pub talisman_bag: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("fishing_bag") public @Nullable AbstractInventoryContents fishingBag;
    pub fishing_bag: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: public @Nullable AbstractInventoryContents quiver;
    pub quiver: Option<compat::Shared<AbstractInventoryContents>>,
}

impl BagContents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            sacks_bag: None,
            potion_bag: None,
            talisman_bag: None,
            fishing_bag: None,
            quiver: None,
        };
        this
    }

}
