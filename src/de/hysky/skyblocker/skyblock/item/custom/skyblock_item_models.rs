//! Ported from de.hysky.skyblocker.skyblock.item.custom.SkyblockItemModels by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::utils;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class SkyblockItemModels
#[derive(Clone)]
pub struct SkyblockItemModels {}

impl SkyblockItemModels {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || {
            CompletableFuture::<()>::run_async(move || (|| { SkyblockItemModels::update_item_models(); })(), |__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t));
        }));
    }

    /// java: private static void updateItemModels()
    pub fn update_item_models() {
        SkyblockerConfigManager::update(Arc::new(move |mut config| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                //   java: ResourcePack.ItemModels itemModelsData = NEURepoManager.getConstants().getResourcePack().getItemModels();
                let resource_pack = NEURepoManager::get_constants().get_resource_pack();
                let item_models_data = resource_pack.get_item_models();
                //   java: Iterator<Map.Entry<String, Identifier>> iterator = config.general.customItemModel.entrySet().iterator();
                //   java: while (iterator.hasNext()) { Map.Entry<String, Identifier> entry = iterator.next(); ..
                // (the entries are walked by key; `iterator.remove()` detaches the entry, so a later `entry.setValue`
                // on it no longer reaches the map - `removed` keeps that)
                let keys: Vec<String> = config.general.custom_item_model.keys().cloned().collect();
                for key in keys {
                    let Some(item_model) = config.general.custom_item_model.get(&key).cloned() else {
                        continue;
                    };
                    let mut item_model_string: String = item_model.to_string();
                    if item_model.namespace() == utils::HYPIXEL_SKYBLOCK_NAMESPACE {
                        let mut removed: bool = false;
                        // Remove item models that no longer exist
                        if item_models_data.get_removed().contains(&item_model_string) {
                            //   java: iterator.remove();
                            config.general.custom_item_model.remove(&key);
                            removed = true;
                        }
                        // Skip updating model name if possible
                        if !item_models_data.get_renamed().contains_key(&item_model_string) {
                            continue;
                        }
                        // Update old item models recursively with some protection against circular map references
                        //   java: String currentModel = itemModelsData.getRenamed().get(itemModelString);
                        let mut current_model: String = item_models_data.get_renamed().get(&item_model_string).cloned().unwrap_or_default();
                        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
                        visited.insert(item_model_string.clone());
                        //   java: String nextModel = itemModelsData.getRenamed().get(currentModel);
                        let mut next_model: Option<String> = item_models_data.get_renamed().get(&current_model).cloned();
                        // java: while (nextModel != null) - `Map.get` answers null for a model that was not renamed
                        while let Some(next) = next_model.clone() {
                            if visited.contains(&next) {
                                // Circular reference so break loop
                                break;
                            }
                            //   java: entry.setValue(Identifier.parse(nextModel));
                            if !removed {
                                config.general.custom_item_model.insert(key.clone(), Identifier::parse(&next));
                            }
                            visited.insert(next.clone());
                            current_model = next;
                            //   java: nextModel = itemModelsData.getRenamed().get(currentModel);
                            next_model = item_models_data.get_renamed().get(&current_model).cloned();
                        }
                    }
                }
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Skyblock Item Models] Failed to update item models"));
            }
        }));
    }

}
