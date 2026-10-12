//! Ported from de.hysky.skyblocker.skyblock.museum.MuseumUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Set<String> EQUIPMENT_TYPES = Set.of("BELT", "GLOVES", "CLOAK", "GAUNTLET", "NECKLACE", "BRACELET", "HAT", "LOCKET", "VINE", "GRIPPERS");
pub static EQUIPMENT_TYPES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["BELT".to_string(), "GLOVES".to_string(), "CLOAK".to_string(), "GAUNTLET".to_string(), "NECKLACE".to_string(), "BRACELET".to_string(), "HAT".to_string(), "LOCKET".to_string(), "VINE".to_string(), "GRIPPERS".to_string()]));

/// java: public class MuseumUtils
#[derive(Clone)]
pub struct MuseumUtils {}

impl MuseumUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static double getSetCraftCost(String id)
    pub fn get_set_craft_cost(id: &str) -> f64 {
        let mut cost: f64 = ((0) as f64) as f64;
        for donation in museum_item_cache::MUSEUM_DONATIONS.get_mut().iter() {
            if Donation::get_id(donation) == id {
                for piece in Donation::get_set(donation) {
                    cost = (cost + ItemUtils::get_craft_cost(&piece.0));
                }
            }
        }
        return cost;
    }

    /// java: protected static Component getDisplayName(String id, boolean isSet)
    pub fn get_display_name(id: &str, mut is_set: bool) -> MutableComponent {
        if is_set {
            let mut name_style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY;
            let mut set_name: Option<String> = compat::lock(&museum_item_cache::ARMOR_NAMES).get(id).cloned();
            if let Some(set_name) = &set_name {
                for donation in museum_item_cache::MUSEUM_DONATIONS.get_mut().iter() {
                    if Donation::get_id(donation) == id && !(Donation::get_set(donation).is_empty()) {
                        let mut piece_name: Option<MutableComponent> = Some(MuseumUtils::get_display_name(&Donation::get_set(donation)[0].0, false));
                        if let Some(piece_name) = &piece_name {
                            let siblings = piece_name.get_siblings();
                            name_style = (if siblings.is_empty() { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { siblings[0].get_style().clone() });
                        }
                        break;
                    }
                }
                return component::literal(set_name).set_style(name_style.clone());
            }
        } else {
            let mut stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(id);
            if let Some(stack) = &stack {
                return compat::component_of(FlexibleItemStack::get_stack_or_throw(stack).get_hover_name());
            }
        }
        return component::literal(id);
    }

    /// java: protected static @Nullable String getSetID(String id)
    pub fn get_set_id(id: &str) -> Option<String> {
        for donation in museum_item_cache::MUSEUM_DONATIONS.get_mut().iter() {
            for set in Donation::get_set(donation) {
                if set.0 == id {
                    return Some(Donation::get_id(donation));
                }
            }
        }
        return None;
    }

    /// java: protected static List<String> getPiecesBySetID(String donationId)
    pub fn get_pieces_by_set_id(donation_id: &str) -> Vec<String> {
        return museum_item_cache::MUSEUM_DONATIONS.get_mut().iter().cloned().filter(|d| (Donation::get_id(d) == donation_id)).flat_map(|d| Donation::get_set(&d).into_iter()).map(|__a0| __a0.0.clone()).collect::<Vec<_>>();
    }

    /// java: public static String formatArmorName(String id, boolean isEquipment)
    pub fn format_armor_name(id: &str, mut is_equipment: bool) -> String {
        let mut lowercase_key: String = id.to_lowercase();
        let mut with_spaces: String = lowercase_key.replace("_", " ");
        let mut words: Vec<String> = compat::regex::split(&with_spaces, " ");
        let mut formatted_name: String = String::new();
        for word in words.iter() {
            { formatted_name.push(compat::char_upper(compat::char_at(&word, 0))); formatted_name.push_str(&compat::substring(&word, 1)); formatted_name.push_str(" "); };
        }
        if is_equipment {
            { formatted_name.push_str("Equipment"); };
        } else if !(compat::regex::Pattern::matches(".*(armor|outfit|suit|tuxedo).*", &lowercase_key)) {
            { formatted_name.push_str("Armor"); };
        }
        return compat::trim(&formatted_name.clone());
    }

    /// java: public static boolean isEquipment(String skyblockApiId)
    pub fn is_equipment(skyblock_api_id: &str) -> bool {
        let mut upper_id: String = skyblock_api_id.to_uppercase();
        return EQUIPMENT_TYPES.iter().cloned().any(|__a0| upper_id.contains(&__a0));
    }

    /// java: public static String formatPrice(double value)
    pub fn format_price(mut value: f64) -> String {
        return formatters::SHORT_FLOAT_NUMBERS.get_mut().format(value).replace(".0", "");
    }

}
