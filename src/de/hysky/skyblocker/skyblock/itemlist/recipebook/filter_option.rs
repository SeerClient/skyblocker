//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.FilterOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public enum FilterOption implements Supplier<Identifier>, Predicate<String>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FilterOption {
    All,
    Entities,
    Npcs,
    Mayors,
    Items,
}

impl FilterOption {
    pub const VALUES: [FilterOption; 5] = [FilterOption::All, FilterOption::Entities, FilterOption::Npcs, FilterOption::Mayors, FilterOption::Items];

    /// java: values()
    pub fn values() -> Vec<FilterOption> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            FilterOption::All => "ALL",
            FilterOption::Entities => "ENTITIES",
            FilterOption::Npcs => "NPCS",
            FilterOption::Mayors => "MAYORS",
            FilterOption::Items => "ITEMS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> FilterOption {
        match name {
            "ALL" => FilterOption::All,
            "ENTITIES" => FilterOption::Entities,
            "NPCS" => FilterOption::Npcs,
            "MAYORS" => FilterOption::Mayors,
            "ITEMS" => FilterOption::Items,
            _ => panic!("IllegalArgumentException: No enum constant FilterOption.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static FilterOptionFields {
        static FIELDS: LazyLock<Vec<FilterOptionFields>> = LazyLock::new(|| vec![
            FilterOptionFields::new(Arc::new(move |_: String| true), crate::entry::skyblocker_mod::id("textures/gui/filter/all.png")),
            FilterOptionFields::new(Arc::new(move |query: String| (query.ends_with("(monster)") || query.ends_with("(miniboss)") || query.ends_with("(boss)") || query.ends_with("(animal)") || query.ends_with("(pest)") || query.ends_with("(sea creature)"))), crate::entry::skyblocker_mod::id("textures/gui/filter/entities.png")),
            FilterOptionFields::new(Arc::new(move |query: String| (query.ends_with("(npc)") || query.ends_with("(rift npc)"))), crate::entry::skyblocker_mod::id("textures/gui/filter/npcs.png")),
            FilterOptionFields::new(Arc::new(move |query: String| (query.ends_with("(mayor)") || query.ends_with("(retired mayor)"))), crate::entry::skyblocker_mod::id("textures/gui/filter/mayors.png")),
            // java: ITEMS(query -> !ENTITIES.test(query) && !NPCS.test(query) && !MAYORS.test(query), ..) - "basically a negation on everything else"
            FilterOptionFields::new(Arc::new(move |query: String| !FilterOption::Entities.test(&query) && !FilterOption::Npcs.test(&query) && !FilterOption::Mayors.test(&query)), crate::entry::skyblocker_mod::id("textures/gui/filter/items.png")),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public boolean test(String query)
    pub fn test(&self, query: &str) -> bool {
        return (self.fields().matching_predicate)(query.to_string());
    }

    /// java: @Override public Identifier get()
    pub fn get(&self) -> Identifier {
        return self.fields().texture.clone();
    }

}

impl std::fmt::Display for FilterOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each FilterOption constant
pub struct FilterOptionFields {
    pub matching_predicate: Arc<dyn Fn(String) -> bool + Send + Sync>,
    pub texture: Identifier,
}

impl FilterOptionFields {
    fn new(matching_predicate: Arc<dyn Fn(String) -> bool + Send + Sync>, mut texture: Identifier) -> Self {
        Self { matching_predicate, texture }
    }

}

/// java: `FilterOption extends Enum<FilterOption>` - what `CyclingTextureWidget<T extends Enum<T> & ..>` reaches
impl crate::de::hysky::skyblocker::utils::enum_utils::JavaEnum for FilterOption {
    fn java_name(&self) -> &'static str {
        self.name()
    }
    fn java_ordinal(&self) -> i32 {
        self.ordinal()
    }
    fn java_values() -> Vec<Self> {
        FilterOption::values()
    }
}

/// java: `FilterOption implements Supplier<Identifier>`
impl crate::de::hysky::skyblocker::utils::render::gui::cycling_texture_widget::IdentifierSupplier for FilterOption {
    fn get(&self) -> mcrust::net::minecraft::resources::identifier::Identifier {
        FilterOption::get(self)
    }
}
