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
            CompletableFuture::<()>::run_async(move || (|| { SkyblockItemModels::update_item_models(); })(), |__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t));
        }));
    }

    /// java: private static void updateItemModels()
    pub fn update_item_models() {
        SkyblockerConfigManager::update(Arc::new(move |config| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.getResourcePack(..)` on a NEUConstants
                //   java: ResourcePack.ItemModels itemModelsData = NEURepoManager.getConstants().getResourcePack().getItemModels();
                let mut item_models_data: crate::unresolved::ItemModels = todo!("fabric2seer: the initialiser of `itemModelsData` (TODO above)");
                let mut iterator = config.general.custom_item_model.iter().map(|(k, v)| (k.clone(), v.clone()));
                // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
                //   java: iterator.hasNext(..)
                while todo!("fabric2seer: the condition (TODO above)") {
                    let mut entry: (String, Identifier) = iterator.next().unwrap();
                    let item_model = &entry.1;
                    // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
                    // rust: let mut item_model_string: String = item_model.fmt();
                    let mut item_model_string: String = todo!("fabric2seer: the initialiser of `itemModelString` (TODO above)");
                    if item_model.namespace() == utils::HYPIXEL_SKYBLOCK_NAMESPACE {
                        // TODO(fabric2seer): the condition: `.getRemoved(..)` on a ItemModels
                        //   java: itemModelsData.getRemoved(..).contains(..)
                        if todo!("fabric2seer: the condition (TODO above)") {
                            // TODO(fabric2seer): Stream.remove
                            //   java: iterator.remove();
                        }
                        // TODO(fabric2seer): the condition: `.getRenamed(..)` on a ItemModels
                        //   java: !itemModelsData.getRenamed().containsKey(itemModelString)
                        if todo!("fabric2seer: the condition (TODO above)") {
                            continue;
                        }
                        // TODO(fabric2seer): `.getRenamed(..)` on a ItemModels
                        //   java: String currentModel = itemModelsData.getRenamed().get(itemModelString);
                        let mut current_model: String = todo!("fabric2seer: the initialiser of `currentModel` (TODO above)");
                        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
                        visited.insert(item_model_string.clone());
                        // TODO(fabric2seer): `.getRenamed(..)` on a ItemModels
                        //   java: String nextModel = itemModelsData.getRenamed().get(currentModel);
                        let mut next_model: String = todo!("fabric2seer: the initialiser of `nextModel` (TODO above)");
                        while true /* java: next_model != null: never null here */ {
                            if visited.contains(&next_model) {
                                break;
                            }
                            entry.1 = Identifier::parse(&next_model);
                            visited.insert(next_model.clone());
                            current_model = next_model.clone();
                            // TODO(fabric2seer): `.getRenamed(..)` on a ItemModels
                            //   java: nextModel = itemModelsData.getRenamed().get(currentModel);
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
