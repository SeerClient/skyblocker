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
            //   java: InputStream stream = file.stream(); (try-with-resources)
            // java: try {..} catch (Exception ex) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                use mcrust::com::mojang::serialization::decoder::Decoder as _;
                //   java: attributes = AttributesFile.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(new String(stream.readAllBytes()))).getOrThrow().attributes();
                let content: String = neu_repo_file_read_all(&file);
                let parsed: attributes_file::AttributesFile = attributes_file::CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&content).expect("JsonSyntaxException"))).get_or_throw().unwrap_or_else(|e| panic!("{e}"));
                compat::set_state(&ATTRIBUTES, parsed.attributes());
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
        //   java: String name = stack.get(DataComponents.CUSTOM_NAME).getString();
        use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter as _;
        let mut name: String = stack.get(mcrust::net::minecraft::core::component::data_components::custom_name()).and_then(|o| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(&o).map(|text| text.get_string())).expect("NullPointerException: CUSTOM_NAME");
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

/// `new String(file.stream().readAllBytes())`.
///
/// rust: port gap - the NEU repo library (`io.github.moulberry.repo`) is a `crate::unresolved` placeholder with no
/// file behind it (`NEURepoManager.file(..)` answers none; GAPS_L, GAPS_M #10): a placeholder file has no content.
fn neu_repo_file_read_all(file: &crate::unresolved::NEURepoFile) -> String {
    let _ = file;
    String::new()
}
