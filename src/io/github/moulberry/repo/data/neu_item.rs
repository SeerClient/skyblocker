//! Ported from io.github.moulberry.repo.data.NEUItem (neurepoparser 1.12.0).

use std::sync::{Arc, Mutex, OnceLock, Weak};

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::{RECIPE_ADAPTER, gson};

/// java: @Getter public class NEUItem
///
/// Shared as `Arc<NEUItem>` (Java's object reference): the repository's item map, the recipes that point back at
/// the item they belong to (`NEUMobDropRecipe.dropsFrom`, `NEUNpcShopRecipe.isSoldBy`) and every caller hold the
/// same object. Java's nullable `String` / `List` fields that every item in the repository carries (`itemid`,
/// `displayname`, `nbttag`, `lore`, `internalname`) read as `""` / empty when absent; the ones only some items
/// carry are `Option`.
pub struct NEUItem {
    /// java: @SerializedName("itemid") String minecraftItemId;
    pub(crate) minecraft_item_id: String,
    /// java: @SerializedName("displayname") String displayName;
    pub(crate) display_name: String,
    /// java: String nbttag;
    pub(crate) nbttag: String,
    /// java: int damage;
    pub(crate) damage: i32,
    /// java: List<String> lore;
    pub(crate) lore: Vec<String>,
    /// java: @SerializedName("internalname") @NEUId String skyblockItemId;
    pub(crate) skyblock_item_id: String,
    /// java: String crafttext;
    pub(crate) crafttext: Option<String>,
    /// java: String clickcommand;
    pub(crate) clickcommand: Option<String>,
    /// java: String modver;
    pub(crate) modver: Option<String>,
    /// java: String infoType;
    pub(crate) info_type: Option<String>,
    /// java: List<String> info;
    pub(crate) info: Option<Vec<String>>,
    /// java: boolean vanilla = false;
    pub(crate) vanilla: bool,
    /// java: int x;
    pub(crate) x: i32,
    /// java: int y;
    pub(crate) y: i32,
    /// java: int z;
    pub(crate) z: i32,
    /// java: String island;
    pub(crate) island: Option<String>,
    /// java: @SerializedName("breakingpower") int breakingPower;
    pub(crate) breaking_power: i32,
    /// java: @SerializedName("recipe") NEURecipe singletonRecipe = null; and
    ///       @SerializedName("recipes") List<NEURecipe> recipeList = new ArrayList<>();
    /// - parsed, not yet filled with this item (`getRecipes` takes them, in Java's `recipeList` then `singletonRecipe`
    /// order, the first time it runs).
    unfilled_recipes: Mutex<Vec<Box<dyn NEURecipe>>>,
    /// java: volatile transient List<NEURecipe> recipes; (double-checked lazy init)
    recipes: OnceLock<Vec<Arc<dyn NEURecipe>>>,
    /// `this` - what `getRecipes` hands each recipe's `fillItemInfo`.
    this: Weak<NEUItem>,
}

impl NEUItem {
    /// Gson's reflective binding of `NEUItem.class` over the repository's item JSON (`rf.json(NEUItem.class)`), the
    /// two recipe members read through the repository's polymorphic `NEURecipe` adapter.
    ///
    /// A recipe the adapter rejects is Gson's `JsonParseException`: a panic, caught by the repository reload.
    pub fn from_json(json: &Value) -> Arc<NEUItem> {
        let read_recipe = |value: &Value| -> Box<dyn NEURecipe> {
            RECIPE_ADAPTER.read(value).unwrap_or_else(|e| panic!("{}", e))
        };
        let mut recipes: Vec<Box<dyn NEURecipe>> = Vec::new();
        // java: newRecipes.addAll(recipeList);
        if let Some(Value::Array(list)) = gson::member(json, "recipes") {
            for value in list {
                if !value.is_null() {
                    recipes.push(read_recipe(value));
                }
            }
        }
        // java: if (singletonRecipe != null) newRecipes.add(singletonRecipe);
        if let Some(value) = gson::member(json, "recipe") {
            recipes.push(read_recipe(value));
        }
        Arc::new_cyclic(|this| NEUItem {
            minecraft_item_id: gson::string(json, "itemid").unwrap_or_default(),
            display_name: gson::string(json, "displayname").unwrap_or_default(),
            nbttag: gson::string(json, "nbttag").unwrap_or_default(),
            damage: gson::int(json, "damage"),
            lore: gson::string_list(json, "lore").unwrap_or_default(),
            skyblock_item_id: gson::string(json, "internalname").unwrap_or_default(),
            crafttext: gson::string(json, "crafttext"),
            clickcommand: gson::string(json, "clickcommand"),
            modver: gson::string(json, "modver"),
            info_type: gson::string(json, "infoType"),
            info: gson::string_list(json, "info"),
            vanilla: gson::boolean(json, "vanilla").unwrap_or(false),
            x: gson::int(json, "x"),
            y: gson::int(json, "y"),
            z: gson::int(json, "z"),
            island: gson::string(json, "island"),
            breaking_power: gson::int(json, "breakingpower"),
            unfilled_recipes: Mutex::new(recipes),
            recipes: OnceLock::new(),
            this: this.clone(),
        })
    }

    /// java: public List<NEURecipe> getRecipes()
    pub fn get_recipes(&self) -> &[Arc<dyn NEURecipe>] {
        // java: if (recipes == null) { synchronized (this) { if (recipes == null) { ..; for (NEURecipe recipe : recipes) recipe.fillItemInfo(this); } } }
        self.recipes.get_or_init(|| {
            let unfilled: Vec<Box<dyn NEURecipe>> = std::mem::take(&mut *self.unfilled_recipes.lock().unwrap_or_else(|p| p.into_inner()));
            let this: Option<Arc<NEUItem>> = self.this.upgrade();
            unfilled
                .into_iter()
                .map(|mut recipe| {
                    if let Some(this) = &this {
                        recipe.fill_item_info(this);
                    }
                    Arc::from(recipe)
                })
                .collect()
        })
    }

    /// java: public String getMinecraftItemId()
    pub fn get_minecraft_item_id(&self) -> &str {
        &self.minecraft_item_id
    }

    /// java: public String getDisplayName()
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// java: public String getNbttag()
    pub fn get_nbttag(&self) -> &str {
        &self.nbttag
    }

    /// java: public int getDamage()
    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    /// java: public List<String> getLore()
    pub fn get_lore(&self) -> &[String] {
        &self.lore
    }

    /// java: public String getSkyblockItemId()
    pub fn get_skyblock_item_id(&self) -> &str {
        &self.skyblock_item_id
    }

    /// java: public String getCrafttext()
    pub fn get_crafttext(&self) -> Option<&str> {
        self.crafttext.as_deref()
    }

    /// java: public String getClickcommand()
    pub fn get_clickcommand(&self) -> Option<&str> {
        self.clickcommand.as_deref()
    }

    /// java: public String getModver()
    pub fn get_modver(&self) -> Option<&str> {
        self.modver.as_deref()
    }

    /// java: public String getInfoType()
    pub fn get_info_type(&self) -> Option<&str> {
        self.info_type.as_deref()
    }

    /// java: public List<String> getInfo()
    pub fn get_info(&self) -> Option<&[String]> {
        self.info.as_deref()
    }

    /// java: public boolean isVanilla()
    pub fn is_vanilla(&self) -> bool {
        self.vanilla
    }

    /// java: public int getX()
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// java: public int getY()
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// java: public int getZ()
    pub fn get_z(&self) -> i32 {
        self.z
    }

    /// java: public String getIsland()
    pub fn get_island(&self) -> Option<&str> {
        self.island.as_deref()
    }

    /// java: public int getBreakingPower()
    pub fn get_breaking_power(&self) -> i32 {
        self.breaking_power
    }
}

impl std::fmt::Debug for NEUItem {
    /// `Object.toString()` stands in as the id (the class has no `@ToString`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NEUItem({})", self.skyblock_item_id)
    }
}
