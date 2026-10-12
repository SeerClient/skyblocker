//! Ported from de.hysky.skyblocker.skyblock.hunting.Attributes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::hunting::attribute::Attribute;
use crate::de::hysky::skyblocker::skyblock::hunting::attributes_file;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::net::minecraft::core::component::data_component_holder::DataComponentHolder;

/// java: private static List<Attribute> attributes = List.of();
pub static ATTRIBUTES: LazyLock<Mutex<Vec<Attribute>>> = LazyLock::new(|| Mutex::new(vec![]));

/// java: public class Attributes
#[derive(Clone)]
pub struct Attributes {}

impl Attributes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { Attributes::load_shards(); }));
    }

    /// java: private static void loadShards()
    pub fn load_shards() {
        let Some(mut file) = NEURepoManager::file("constants/attribute_shards.json") else {
            return;
        };
        {
            // TODO(fabric2seer): `.stream(..)` on a NEURepoFile
            //   java: InputStream stream = file.stream(..);
            let mut stream: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `stream` (TODO above)");
            // java: try {..} catch (Exception ex) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.readAllBytes(..)` on a InputStream
                //   java: attributes = AttributesFile.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(new String(stream.readAllBytes()))).getOrThrow().attributes();
                logger().info("[Skyblocker Attributes] Successfully loaded attributes!");
            }));
            if let Err(ex) = __result {
                logger().error(&format!("[Skyblocker Attributes] Failed to load attributes!"));
            }
        }
    }

    /// java: public static @Nullable Attribute getAttributeFromItemName(DataComponentHolder stack)
    pub fn get_attribute_from_item_name(stack: &dyn DataComponentHolder) -> Option<Attribute> {
        if !(stack.has(mcrust::net::minecraft::core::component::data_components::custom_name())) {
            return None;
        }
        // TODO(fabric2seer): `.getString(..)` on a value of unknown type
        //   java: String name = stack.get(DataComponents.CUSTOM_NAME).getString();
        let mut name: String = todo!("fabric2seer: the initialiser of `name` (TODO above)");
        return Attributes::get_attribute_from_item_name_string(name.clone());
    }

    /// java: public static @Nullable Attribute getAttributeFromItemName(String name)
    pub fn get_attribute_from_item_name_string(mut name: String) -> Option<Attribute> {
        let mut index: i32 = compat::index_of(&name, "Shard");
        if index > -1 {
            name = compat::substring2(&name, 0, index.wrapping_sub(1));
        }
        name = (name.replace("BUY ", "")).replace("SELL ", "");
        for attribute in compat::get_state(&ATTRIBUTES) {
            if attribute.shard_name() == name {
                return Some(attribute.clone());
            }
        }
        return None;
    }

    /// java: public static @Nullable Attribute getAttributeFromAbilityName(String name)
    pub fn get_attribute_from_ability_name(name: &str) -> Option<Attribute> {
        for attribute in compat::get_state(&ATTRIBUTES) {
            if name.starts_with(&attribute.name()) {
                return Some(attribute.clone());
            }
        }
        return None;
    }

}
