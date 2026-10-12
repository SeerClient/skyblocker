//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.RecipeItemStackCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;

/// java: public static final Map<String, FlexibleItemStack> CACHE = new ConcurrentHashMap<>();
pub static CACHE: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public final class RecipeItemStackCache
#[derive(Clone)]
pub struct RecipeItemStackCache {}

impl RecipeItemStackCache {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static String getCacheKey(String id, int count)
    pub fn get_cache_key(id: &str, mut count: i32) -> String {
        return format!("{}+{}", id, count);
    }

}
