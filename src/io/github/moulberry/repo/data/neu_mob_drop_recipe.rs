//! Ported from io.github.moulberry.repo.data.NEUMobDropRecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::{Arc, Weak};

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::NEUIngredient;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUMobDropRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUMobDropRecipe {
    /// java: @SerializedName("combat_xp") int combatExperience;
    pub(crate) combat_experience: i32,
    /// java: @SerializedName("taming_xp") int tamingExperience;
    pub(crate) taming_experience: i32,
    /// java: @SerializedName("mining_xp") int miningExperience;
    pub(crate) mining_experience: i32,
    /// java: @SerializedName("foraging_xp") int foragingExperience;
    pub(crate) foraging_experience: i32,
    /// java: @SerializedName("hunting_xp") int huntingExperience;
    pub(crate) hunting_experience: i32,
    /// java: @SerializedName("enchanting_xp") int enchantingExperience;
    pub(crate) enchanting_experience: i32,
    /// java: @SerializedName("carpentry_xp") int carpentryExperience;
    pub(crate) carpentry_experience: i32,
    /// java: @SerializedName("farming_xp") int farmingExperience;
    pub(crate) farming_experience: i32,
    /// java: @SerializedName("fishing_xp") int fishingExperience;
    pub(crate) fishing_experience: i32,
    /// java: @SerializedName("alchemy_xp") int alchemyExperience;
    pub(crate) alchemy_experience: i32,
    /// java: @SerializedName("runecrafting_xp") int runecraftingExperience;
    pub(crate) runecrafting_experience: i32,
    /// java: int coins;
    pub(crate) coins: i32,
    /// java: @SerializedName("xp") int experienceOrbs;
    pub(crate) experience_orbs: i32,
    /// java: String name;
    pub(crate) name: Option<String>,
    /// java: String render;
    pub(crate) render: Option<String>,
    /// java: String panorama;
    pub(crate) panorama: Option<String>,
    /// java: int level;
    pub(crate) level: i32,
    /// java: List<String> extra;
    pub(crate) extra: Vec<String>,
    /// java: List<Drop> drops;
    pub(crate) drops: Vec<Drop>,
    /// java: transient NEUItem dropsFrom; (a `Weak`: the item owns its recipes)
    pub(crate) drops_from: Weak<NEUItem>,
}

/// java: @Data @AllArgsConstructor public static class Drop
#[derive(Clone, Debug, PartialEq)]
pub struct Drop {
    /// java: NEUIngredient dropItem;
    pub(crate) drop_item: NEUIngredient,
    /// java: @Nullable String chance;
    pub(crate) chance: Option<String>,
    /// java: List<String> extra;
    pub(crate) extra: Vec<String>,
}

/// java: public static class Serializer implements JsonDeserializer<Drop> (NEUMobDropRecipe.Drop.Serializer)
pub struct DropSerializer;

impl DropSerializer {
    /// java: public Drop deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(json: &Value) -> Drop {
        // java: if (json.isJsonPrimitive()) return new Drop(NEUIngredient.fromString(json.getAsString()), null, Collections.emptyList());
        if !json.is_object() && !json.is_array() && !json.is_null() {
            let s = gson::as_string(json).unwrap_or_default();
            return Drop::new(NEUIngredient::from_string(&s), None, Vec::new());
        }
        // java: JsonObject d = json.getAsJsonObject();
        if !json.is_object() {
            panic!("java.lang.IllegalStateException: Not a JSON Object: {}", json);
        }
        // java: NEUIngredient.fromString(d.get("id").getAsString())
        let id: String = gson::string(json, "id").unwrap_or_else(|| panic!("java.lang.NullPointerException: drop without an id: {}", json));
        // java: d.has("chance") ? d.get("chance").getAsString() : null
        let chance: Option<String> = json.get("chance").and_then(gson::as_string);
        // java: d.has("extra") ? context.deserialize(d.get("extra"), List<String>) : Collections.emptyList()
        let extra: Vec<String> = json.get("extra").and_then(|v| gson::list(v, gson::as_string)).unwrap_or_default();
        Drop::new(NEUIngredient::from_string(&id), chance, extra)
    }
}

impl Drop {
    /// java: public Drop(final NEUIngredient dropItem, @Nullable final String chance, final List<String> extra)
    pub fn new(drop_item: NEUIngredient, chance: Option<String>, extra: Vec<String>) -> Self {
        Drop { drop_item, chance, extra }
    }

    /// java: public NEUIngredient getDropItem()
    pub fn get_drop_item(&self) -> &NEUIngredient {
        &self.drop_item
    }

    /// java: public @Nullable String getChance()
    pub fn get_chance(&self) -> Option<&str> {
        self.chance.as_deref()
    }

    /// java: public List<String> getExtra()
    pub fn get_extra(&self) -> &[String] {
        &self.extra
    }

    /// java: public void setDropItem(final NEUIngredient dropItem)
    pub fn set_drop_item(&mut self, drop_item: NEUIngredient) {
        self.drop_item = drop_item;
    }

    /// java: public void setChance(@Nullable final String chance)
    pub fn set_chance(&mut self, chance: Option<String>) {
        self.chance = chance;
    }

    /// java: public void setExtra(final List<String> extra)
    pub fn set_extra(&mut self, extra: Vec<String>) {
        self.extra = extra;
    }
}

impl std::fmt::Display for Drop {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NEUMobDropRecipe.Drop(dropItem={}, chance={}, extra=[{}])",
            self.drop_item,
            self.chance.as_deref().unwrap_or("null"),
            self.extra.join(", ")
        )
    }
}

impl NEUMobDropRecipe {
    /// Gson's reflective binding of the class (the drops through `Drop.Serializer`).
    pub fn from_json(json: &Value) -> Self {
        NEUMobDropRecipe {
            combat_experience: gson::int(json, "combat_xp"),
            taming_experience: gson::int(json, "taming_xp"),
            mining_experience: gson::int(json, "mining_xp"),
            foraging_experience: gson::int(json, "foraging_xp"),
            hunting_experience: gson::int(json, "hunting_xp"),
            enchanting_experience: gson::int(json, "enchanting_xp"),
            carpentry_experience: gson::int(json, "carpentry_xp"),
            farming_experience: gson::int(json, "farming_xp"),
            fishing_experience: gson::int(json, "fishing_xp"),
            alchemy_experience: gson::int(json, "alchemy_xp"),
            runecrafting_experience: gson::int(json, "runecrafting_xp"),
            coins: gson::int(json, "coins"),
            experience_orbs: gson::int(json, "xp"),
            name: gson::string(json, "name"),
            render: gson::string(json, "render"),
            panorama: gson::string(json, "panorama"),
            level: gson::int(json, "level"),
            extra: gson::string_list(json, "extra").unwrap_or_default(),
            drops: gson::member(json, "drops").and_then(|v| gson::list(v, |d| Some(DropSerializer::deserialize(d)))).unwrap_or_default(),
            drops_from: Weak::new(),
        }
    }

    /// java: public int getCombatExperience()
    pub fn get_combat_experience(&self) -> i32 {
        self.combat_experience
    }

    /// java: public int getTamingExperience()
    pub fn get_taming_experience(&self) -> i32 {
        self.taming_experience
    }

    /// java: public int getMiningExperience()
    pub fn get_mining_experience(&self) -> i32 {
        self.mining_experience
    }

    /// java: public int getForagingExperience()
    pub fn get_foraging_experience(&self) -> i32 {
        self.foraging_experience
    }

    /// java: public int getHuntingExperience()
    pub fn get_hunting_experience(&self) -> i32 {
        self.hunting_experience
    }

    /// java: public int getEnchantingExperience()
    pub fn get_enchanting_experience(&self) -> i32 {
        self.enchanting_experience
    }

    /// java: public int getCarpentryExperience()
    pub fn get_carpentry_experience(&self) -> i32 {
        self.carpentry_experience
    }

    /// java: public int getFarmingExperience()
    pub fn get_farming_experience(&self) -> i32 {
        self.farming_experience
    }

    /// java: public int getFishingExperience()
    pub fn get_fishing_experience(&self) -> i32 {
        self.fishing_experience
    }

    /// java: public int getAlchemyExperience()
    pub fn get_alchemy_experience(&self) -> i32 {
        self.alchemy_experience
    }

    /// java: public int getRunecraftingExperience()
    pub fn get_runecrafting_experience(&self) -> i32 {
        self.runecrafting_experience
    }

    /// java: public int getCoins()
    pub fn get_coins(&self) -> i32 {
        self.coins
    }

    /// java: public int getExperienceOrbs()
    pub fn get_experience_orbs(&self) -> i32 {
        self.experience_orbs
    }

    /// java: public String getName()
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// java: public String getRender()
    pub fn get_render(&self) -> Option<&str> {
        self.render.as_deref()
    }

    /// java: public String getPanorama()
    pub fn get_panorama(&self) -> Option<&str> {
        self.panorama.as_deref()
    }

    /// java: public int getLevel()
    pub fn get_level(&self) -> i32 {
        self.level
    }

    /// java: public List<String> getExtra()
    pub fn get_extra(&self) -> &[String] {
        &self.extra
    }

    /// java: public List<Drop> getDrops()
    pub fn get_drops(&self) -> &[Drop] {
        &self.drops
    }

    /// java: public NEUItem getDropsFrom()
    pub fn get_drops_from(&self) -> Option<Arc<NEUItem>> {
        self.drops_from.upgrade()
    }
}

impl NEURecipe for NEUMobDropRecipe {
    /// java: public void fillItemInfo(NEUItem item) { dropsFrom = item; }
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        self.drops_from = Arc::downgrade(item);
    }

    /// java: public Collection<NEUIngredient> getAllInputs() { return Collections.singletonList(NEUIngredient.fromItem(dropsFrom, 1)); }
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        match self.drops_from.upgrade() {
            Some(drops_from) => vec![NEUIngredient::from_item(&drops_from, 1)],
            None => panic!("java.lang.NullPointerException: NEUMobDropRecipe.dropsFrom"),
        }
    }

    /// java: public Collection<NEUIngredient> getAllOutputs()
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        // java: for (Drop drop : drops) l.add(drop.getDropItem());
        self.drops.iter().map(|drop| drop.get_drop_item().clone()).collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
