//! Ported from de.hysky.skyblocker.skyblock.itemlist.StackOverlays by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::nbt::tag_parser;
use mcrust::net::minecraft::shared_constants;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final int DATA_VERSION = SharedConstants.getCurrentVersion().dataVersion().version();
pub static DATA_VERSION: LazyLock<i32> = LazyLock::new(|| shared_constants::get_current_version().data_version().version());
/// java: private static Map<@NEUId String, ItemOverlayFile> overlays = Map.of();
pub static OVERLAYS: compat::MainThread<std::collections::HashMap<String, crate::unresolved::ItemOverlayFile>> = compat::MainThread::new(|| std::collections::HashMap::from([]));

/// java: public class StackOverlays
#[derive(Clone)]
pub struct StackOverlays {}

impl StackOverlays {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void loadOverlays()
    pub fn load_overlays() {
        *OVERLAYS.get_mut() = NEURepoManager::get_stack_overlays(*DATA_VERSION);
    }

    /// java: protected static void applyOverlay(NEUItem neuItem, FlexibleItemStack stack)
    pub fn apply_overlay(neu_item: &crate::unresolved::NEUItem, stack: &compat::Shared<FlexibleItemStack>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
            //   java: ItemOverlayFile overlayFile = overlays.get(neuItem.getSkyblockItemId());
            let mut overlay_file: crate::unresolved::ItemOverlayFile = todo!("fabric2seer: the initialiser of `overlayFile` (TODO above)");
            if true /* java: overlay_file != null: never null here */ {
                // TODO(fabric2seer): `.getFile(..)` on a ItemOverlayFile
                //   java: String overlayData = Files.readString(overlayFile.getFile().getFsPath());
                let mut overlay_data: String = todo!("fabric2seer: the initialiser of `overlayData` (TODO above)");
                // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
                // rust: let mut overlay_stack: compat::Shared<FlexibleItemStack> = (flexible_item_stack::CODEC.get_mut().parse(&holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance()), &tag_parser::parse_compound_fully(&overlay_data).unwrap())).set_partial(flexible_item_stack::EMPTY.get_mut().clone()).result_or_partial_with(|error| { StackOverlays::log_parse_error(neu_item, &error); }).unwrap();
                let mut overlay_stack: compat::Shared<FlexibleItemStack> = todo!("fabric2seer: the initialiser of `overlayStack` (TODO above)");
                if !(FlexibleItemStack::is_empty(&overlay_stack)) {
                    let mut changes: DataComponentPatch = FlexibleItemStack::components(&overlay_stack);
                    FlexibleItemStack::apply_components(stack, &changes);
                }
            }
        }));
        if let Err(e) = __result {
            // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
            //   java: LOGGER.error("[Skyblocker Stack Overlays] Failed to apply stack overlay! Item: {}", neuItem.getSkyblockItemId(), e);
        }
    }

    /// java: protected static void cleanUpOverlays()
    pub fn clean_up_overlays() {
        *OVERLAYS.get_mut() = std::collections::HashMap::from([]);
    }

    /// java: private static void logParseError(NEUItem neuItem, String message)
    pub fn log_parse_error(neu_item: &crate::unresolved::NEUItem, message: &str) {
        // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
        //   java: LOGGER.error("[Skyblocker Stack Overlays] Failed to parse item \"{}\". Error: {}", neuItem.getSkyblockItemId(), message);
    }

}
