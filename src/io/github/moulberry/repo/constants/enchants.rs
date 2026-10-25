//! Ported from io.github.moulberry.repo.constants.Enchants (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class Enchants
#[derive(Clone, Debug, Default)]
pub struct Enchants {
    /// java: @SerializedName("enchants") Map<String, List<String>> availableEnchants;
    pub(crate) available_enchants: Option<HashMap<String, Vec<String>>>,
    /// java: @SerializedName("enchant_pools") List<List<String>> enchantPools;
    pub(crate) enchant_pools: Option<Vec<Vec<String>>>,
    /// java: @SerializedName("enchants_xp_cost") Map<String, List<Integer>> enchantExperienceCost;
    pub(crate) enchant_experience_cost: Option<HashMap<String, Vec<i32>>>,
}

impl Enchants {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Enchants {
            available_enchants: gson::member(json, "enchants")
                .and_then(|v| gson::map(v, gson::string_key, |l| gson::list(l, gson::as_string))),
            enchant_pools: gson::member(json, "enchant_pools").and_then(|v| gson::list(v, |l| gson::list(l, gson::as_string))),
            enchant_experience_cost: gson::member(json, "enchants_xp_cost")
                .and_then(|v| gson::map(v, gson::string_key, |l| gson::list(l, gson::as_i32))),
        }
    }

    /// java: public List<String> getAvailableEnchants(String tooltype)
    pub fn get_available_enchants_1(&self, tooltype: &str) -> Option<&Vec<String>> {
        // java: if (availableEnchants == null) return null; return availableEnchants.get(tooltype);
        self.available_enchants.as_ref()?.get(tooltype)
    }

    /// java: public List<String> getConflictingEnchants(String enchant)
    pub fn get_conflicting_enchants(&self, enchant: &str) -> Vec<String> {
        // java: if (enchantPools == null) return new ArrayList<>();
        let Some(enchant_pools) = &self.enchant_pools else {
            return Vec::new();
        };
        let mut conflicts: Vec<String> = Vec::new();
        for enchant_pool in enchant_pools {
            if enchant_pool.iter().any(|e| e == enchant) {
                // java: conflicts.addAll(enchantPool); conflicts.remove(enchant); - `remove(Object)` drops the FIRST equal element
                conflicts.extend(enchant_pool.iter().cloned());
                if let Some(index) = conflicts.iter().position(|e| e == enchant) {
                    conflicts.remove(index);
                }
            }
        }
        conflicts
    }

    /// java: public Map<String, List<String>> getAvailableEnchants()
    pub fn get_available_enchants(&self) -> Option<&HashMap<String, Vec<String>>> {
        self.available_enchants.as_ref()
    }

    /// java: public List<List<String>> getEnchantPools()
    pub fn get_enchant_pools(&self) -> Option<&Vec<Vec<String>>> {
        self.enchant_pools.as_ref()
    }

    /// java: public Map<String, List<Integer>> getEnchantExperienceCost()
    pub fn get_enchant_experience_cost(&self) -> Option<&HashMap<String, Vec<i32>>> {
        self.enchant_experience_cost.as_ref()
    }
}
