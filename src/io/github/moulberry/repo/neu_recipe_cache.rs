//! Ported from io.github.moulberry.repo.NEURecipeCache (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::i_reloadable::IReloadable;
use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// `Map<@NEUId String, Set<NEURecipe>>`. Each set is a `Vec` holding every recipe once (Java's `HashSet` of recipe
/// objects compares them by identity, so `add` of the same recipe twice keeps one - see `add_to_set`).
pub type RecipeMap = HashMap<String, Vec<Arc<dyn NEURecipe>>>;

/// java: public class NEURecipeCache implements IReloadable
pub struct NEURecipeCache {
    /// java: @Getter Map<@NEUId String, Set<NEURecipe>> recipes = new HashMap<>();
    recipes: RwLock<Arc<RecipeMap>>,
    /// java: @Getter Map<@NEUId String, Set<NEURecipe>> usages = new HashMap<>();
    usages: RwLock<Arc<RecipeMap>>,
}

/// `set.add(recipe)` on an identity `HashSet`.
fn add_to_set(set: &mut Vec<Arc<dyn NEURecipe>>, recipe: &Arc<dyn NEURecipe>) {
    if !set.iter().any(|r| Arc::ptr_eq(r, recipe)) {
        set.push(recipe.clone());
    }
}

impl NEURecipeCache {
    /// java: private NEURecipeCache() {}
    fn new() -> Self {
        NEURecipeCache { recipes: RwLock::new(Arc::new(HashMap::new())), usages: RwLock::new(Arc::new(HashMap::new())) }
    }

    /// Create a recipe cache: an unitialized repository cache, that can be initialized by calling
    /// `NEURepository::reload`, or by manually `reload`ing it.
    ///
    /// java: public static NEURecipeCache forRepo(NEURepository repository)
    pub fn for_repo(repository: &Arc<NEURepository>) -> Arc<NEURecipeCache> {
        let neu_recipe_cache = Arc::new(NEURecipeCache::new());
        repository.register_reload_listener(neu_recipe_cache.clone());
        neu_recipe_cache
    }

    /// java: public Map<@NEUId String, Set<NEURecipe>> getRecipes()
    pub fn get_recipes(&self) -> Arc<RecipeMap> {
        self.recipes.read().unwrap_or_else(|p| p.into_inner()).clone()
    }

    /// java: public Map<@NEUId String, Set<NEURecipe>> getUsages()
    pub fn get_usages(&self) -> Arc<RecipeMap> {
        self.usages.read().unwrap_or_else(|p| p.into_inner()).clone()
    }
}

impl IReloadable for NEURecipeCache {
    /// java: public void reload(NEURepository repository) throws NEURepositoryException
    fn reload(&self, repository: &Arc<NEURepository>) -> Result<(), NEURepositoryException> {
        let mut recipes: RecipeMap = HashMap::new();
        let mut usages: RecipeMap = HashMap::new();
        // java: for (NEUItem item : repository.getItems().getItems().values())
        for item in repository.get_items().get_items().values() {
            // java: for (NEURecipe recipe : item.getRecipes())
            for recipe in item.get_recipes() {
                // java: for (NEUIngredient input : recipe.getAllInputs()) usages.computeIfAbsent(input.getItemId(), ignored -> new HashSet<>()).add(recipe);
                for input in recipe.get_all_inputs() {
                    add_to_set(usages.entry(input.get_item_id().to_string()).or_default(), recipe);
                }
                // java: for (NEUIngredient output : recipe.getAllOutputs()) recipes.computeIfAbsent(output.getItemId(), ignored -> new HashSet<>()).add(recipe);
                for output in recipe.get_all_outputs() {
                    add_to_set(recipes.entry(output.get_item_id().to_string()).or_default(), recipe);
                }
            }
        }
        // java: this.recipes = recipes; this.usages = usages;
        *self.recipes.write().unwrap_or_else(|p| p.into_inner()) = Arc::new(recipes);
        *self.usages.write().unwrap_or_else(|p| p.into_inner()) = Arc::new(usages);
        Ok(())
    }
}
