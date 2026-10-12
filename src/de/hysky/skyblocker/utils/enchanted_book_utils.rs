//! Ported from de.hysky.skyblocker.utils.EnchantedBookUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

// TODO(fabric2seer): static call Object2ObjectMaps.synchronize(..)
//   java: private static final Map<String, String> API_ID_OVERRIDES = Object2ObjectMaps.synchronize(new Object2ObjectOpenHashMap<>());
/// java: private static final Map<String, String> API_ID_OVERRIDES = Object2ObjectMaps.synchronize(new Object2ObjectOpenHashMap<>());
pub static API_ID_OVERRIDES: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(todo!("fabric2seer: the initialiser of `API_ID_OVERRIDES` - see the TODO above")));

/// java: public final class EnchantedBookUtils
#[derive(Clone)]
pub struct EnchantedBookUtils {}

impl EnchantedBookUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || {
            compat::lock(&API_ID_OVERRIDES).clear();
            NEURepoManager::for_each_item(Arc::new(move |item| {
                // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
                //   java: String itemId = item.getSkyblockItemId();
                let mut item_id: String = todo!("fabric2seer: the initialiser of `itemId` (TODO above)");
                if !(item_id.contains(";")) {
                    return;
                }
                // TODO(fabric2seer): the condition: `.getMinecraftItemId(..)` on a NEUItem
                //   java: !item.getMinecraftItemId().equals("minecraft:enchanted_book")
                if todo!("fabric2seer: the condition (TODO above)") {
                    return;
                }
                // TODO(fabric2seer): `.getBazaarStocks(..)` on a NEUConstants
                //   java: String actualId = NEURepoManager.getConstants().getBazaarStocks().getBazaarStockOrDefault(itemId);
                let mut actual_id: String = todo!("fabric2seer: the initialiser of `actualId` (TODO above)");
                if item_id == actual_id {
                    return;
                }
                // TODO(fabric2seer): `.getLore(..)` on a NEUItem
                //   java: String guessId = getApiIdByName(TextTransformer.fromLegacy(getEnchantNameFromLore(item.getLore())));
                let mut guess_id: String = todo!("fabric2seer: the initialiser of `guessId` (TODO above)");
                if guess_id == actual_id {
                    return;
                }
                compat::lock(&API_ID_OVERRIDES).insert(guess_id.clone(), actual_id.clone());
            }));
        }));
    }

    /// java: public static String getApiIdByName(Component enchantName)
    pub fn get_api_id_by_name(enchant_name: &MutableComponent) -> String {
        let mut name: String = enchant_name.get_string().to_uppercase();
        name = (name.replace("BUY ", "")).replace("SELL ", "").replace("'", "").replace("-", "_");
        let mut parts: Vec<String> = compat::regex::split(&name, " ");
        if (parts.len() as i32) == 0 {
            return "".to_string();
        }
        if RomanNumerals::is_valid_roman_numeral(Some(parts[((parts.len() as i32).wrapping_sub(1)) as usize].clone())) {
            let __v = RomanNumerals::roman_to_decimal(Some(parts[((parts.len() as i32).wrapping_sub(1)) as usize].clone())).to_string();
            let __i = ((parts.len() as i32).wrapping_sub(1)) as usize;
            parts[__i] = __v;
        }
        let mut is_ultimate: bool = (!(enchant_name.get_siblings().is_empty()) && (enchant_name.get_siblings()[enchant_name.get_siblings().len() - 1]).get_style().is_bold());
        let mut id: String = format!("ENCHANTMENT_{}{}", (if is_ultimate { "ULTIMATE_" } else { "" }), parts.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("_"));
        return compat::lock(&API_ID_OVERRIDES).get(&id).cloned().unwrap_or_else(|| id.clone());
    }

    /// java: public static String getEnchantNameFromLore(List<String> lore)
    pub fn get_enchant_name_from_lore(lore: &Vec<String>) -> String {
        let mut line: Option<String> = lore.iter().cloned().filter(|x| !(x.contains("Combinable in Anvil"))).filter(|x| !x.is_empty()).next();
        return line.clone().unwrap_or("".to_string());
    }

}
