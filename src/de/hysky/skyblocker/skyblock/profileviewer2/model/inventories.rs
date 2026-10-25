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

impl Inventories {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "bag_contents") {
            if let Some(__x) = Some(BagContents::from_json(__v)) {
                self.bag_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "inv_contents") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.inventory_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "ender_chest_contents") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.ender_chest_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "backpack_icons") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(AbstractInventoryContents::from_json(__e0))) {
                self.backpack_icons = __x;
            }
        }
        if let Some(__v) = gson::member(json, "backpack_contents") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(AbstractInventoryContents::from_json(__e0))) {
                self.backpack_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "inv_armor") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.armour_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "equipment_contents") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.equipment_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "personal_vault_contents") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.personal_vault_contents = __x;
            }
        }
        if let Some(__v) = gson::member(json, "sacks_counts") {
            if let Some(__x) = (gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0))).map(Some) {
                self.sacks_counts = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Inventories.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl AbstractInventoryContents {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "data") {
            if let Some(__x) = gson::as_string(__v) {
                self.data = __x;
            }
        }
    }

    /// `GSON.fromJson(json, AbstractInventoryContents.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

impl BagContents {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "sacks_bag") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.sacks_bag = __x;
            }
        }
        if let Some(__v) = gson::member(json, "potion_bag") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.potion_bag = __x;
            }
        }
        if let Some(__v) = gson::member(json, "talisman_bag") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.talisman_bag = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fishing_bag") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.fishing_bag = __x;
            }
        }
        if let Some(__v) = gson::member(json, "quiver") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.quiver = __x;
            }
        }
    }

    /// `GSON.fromJson(json, BagContents.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
