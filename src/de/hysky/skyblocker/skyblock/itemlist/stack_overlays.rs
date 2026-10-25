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
use crate::io::github::moulberry::repo::data::item_overlays::ItemOverlayFile;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;

/// java: private static final int DATA_VERSION = SharedConstants.getCurrentVersion().dataVersion().version();
pub static DATA_VERSION: LazyLock<i32> = LazyLock::new(|| shared_constants::get_current_version().data_version().version());
/// java: private static Map<@NEUId String, ItemOverlayFile> overlays = Map.of();
pub static OVERLAYS: compat::MainThread<std::collections::HashMap<String, ItemOverlayFile>> = compat::MainThread::new(|| std::collections::HashMap::from([]));

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
    pub fn apply_overlay(neu_item: &NEUItem, stack: &compat::Shared<FlexibleItemStack>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: ItemOverlayFile overlayFile = overlays.get(neuItem.getSkyblockItemId());
            let overlay_file: Option<ItemOverlayFile> = OVERLAYS.get().get(neu_item.get_skyblock_item_id()).cloned();
            // The returned file is null if it does not exist
            if let Some(overlay_file) = overlay_file {
                //   java: String overlayData = Files.readString(overlayFile.getFile().getFsPath());
                let mut overlay_data: String = std::fs::read_to_string(overlay_file.get_file().get_fs_path()).unwrap_or_else(|e| panic!("java.io.IOException: {e}"));
                //   java: FlexibleItemStack overlayStack = FlexibleItemStack.CODEC.parse(RegistryUtils.getRegistryWrapperLookup()
                //     .createSerializationContext(NbtOps.INSTANCE), TagParser.parseCompoundFully(overlayData))
                //     .setPartial(FlexibleItemStack.EMPTY).resultOrPartial(error -> logParseError(neuItem, error)).get();
                let ops = holder_lookup::create_serialization_context(RegistryUtils::get_registry_wrapper_lookup(), mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
                let overlay_tag = tag_parser::parse_compound_fully(&overlay_data).unwrap_or_else(|e| panic!("CommandSyntaxException: {e:?}"));
                let empty: compat::Shared<FlexibleItemStack> = flexible_item_stack::EMPTY.get().clone();
                let mut overlay_stack: compat::Shared<FlexibleItemStack> = flexible_item_stack::CODEC
                    .get()
                    .parse(&ops, &mcrust::com::mojang::serialization::obj(overlay_tag))
                    .set_partial(empty)
                    .result_or_partial_with(|error| StackOverlays::log_parse_error(neu_item, &error))
                    .expect("NoSuchElementException: No value present");
                if !(FlexibleItemStack::is_empty(&overlay_stack)) {
                    let mut changes: DataComponentPatch = FlexibleItemStack::components(&overlay_stack);
                    FlexibleItemStack::apply_components(stack, &changes);
                }
            }
        }));
        if let Err(e) = __result {
            //   java: LOGGER.error("[Skyblocker Stack Overlays] Failed to apply stack overlay! Item: {}", neuItem.getSkyblockItemId(), e);
            logger().error(&format!("[Skyblocker Stack Overlays] Failed to apply stack overlay! Item: {} {}", neu_item.get_skyblock_item_id(), compat::panic_message(&e)));
        }
    }

    /// java: protected static void cleanUpOverlays()
    pub fn clean_up_overlays() {
        *OVERLAYS.get_mut() = std::collections::HashMap::from([]);
    }

    /// java: private static void logParseError(NEUItem neuItem, String message)
    pub fn log_parse_error(neu_item: &NEUItem, message: &str) {
        //   java: LOGGER.error("[Skyblocker Stack Overlays] Failed to parse item \"{}\". Error: {}", neuItem.getSkyblockItemId(), message);
        logger().error(&format!("[Skyblocker Stack Overlays] Failed to parse item \"{}\". Error: {}", neu_item.get_skyblock_item_id(), message));
    }

}
