//! java: public class MiscCalculators (net.azureaaron.networth)

use std::sync::LazyLock;

use super::calculation::{Calculation, Type};
use super::networth_result::NetworthResult;
use super::utils::item_constants;
use crate::compat::regex::Pattern;

/// java: private static final Pattern API_RUNE_PATTERN = Pattern.compile("RUNE_(?<rune>[A-Z_]+)_(?<level>\\d)");
static API_RUNE_PATTERN: LazyLock<Pattern> =
    LazyLock::new(|| Pattern::compile("RUNE_(?<rune>[A-Z_]+)_(?<level>\\d)"));

/// java: public class MiscCalculators
pub struct MiscCalculators;

impl MiscCalculators {
    /// java: public static NetworthResult calculateSackItem(String id, int count, ToDoubleFunction<String> prices)
    ///
    /// Calculates the worth of item's in a player's sack.
    pub fn calculate_sack_item(id: &str, count: i32, prices: &dyn Fn(&str) -> f64) -> NetworthResult {
        let mut id: String = id.to_string();
        let mut matcher = API_RUNE_PATTERN.matcher(&id);

        if matcher.matches() {
            // java: Integer.parseInt(matcher.group("level")) - one digit, so it always parses
            let level: i32 = matcher
                .group_named("level")
                .expect("the group matched")
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("NumberFormatException: {e}"));
            id = format!("{}_RUNE_{}", matcher.group_named("rune").expect("the group matched"), level);

            if !item_constants::VALID_RUNES.contains(id.as_str()) {
                return NetworthResult::empty();
            }
        }

        let calculation = Calculation::of_4(Type::SackItem, id.clone(), prices(&id) * count as f64, count);

        NetworthResult::of(calculation.price(), calculation.price(), vec![calculation])
    }

    /// java: public static NetworthResult calculateEssence(String essenceType, int count, ToDoubleFunction<String> prices)
    ///
    /// Calculates the value of essence.
    pub fn calculate_essence(essence_type: &str, count: i32, prices: &dyn Fn(&str) -> f64) -> NetworthResult {
        let calculation = Calculation::of_4(
            Type::Essence,
            essence_type.to_string(),
            prices(&format!("ESSENCE_{essence_type}")) * count as f64,
            count,
        );

        NetworthResult::of(calculation.price(), calculation.price(), vec![calculation])
    }
}
