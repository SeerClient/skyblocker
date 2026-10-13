//! package de.hysky.skyblocker.config.categories
pub mod chat_category;
pub mod crimson_isle_category;
pub mod debug_category;
pub mod dungeons_category;
pub mod event_notifications_category;
pub mod farming_category;
pub mod foraging_category;
pub mod general_category;
pub mod helper_category;
pub mod hunting_category;
pub mod mining_category;
pub mod misc_category;
pub mod other_locations_category;
pub mod quick_navigation_category;
pub mod slayers_category;
pub mod ui_and_visuals_category;

use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// The enums the categories hand to Dandelion's `EnumController` (`Option.<SomeEnum>createBuilder()`), as
/// `java.lang.Enum` sees them: `getEnumConstants()`, `ordinal()`, `name()`, and their own `toString()` (which
/// `EnumControllerImpl.TO_STRING_FORMATTER` displays).
pub trait ConfigEnum: dandelion::compat::JavaEnum {
    /// `toString()`: the enum's override where it has one, `name()` otherwise.
    fn java_to_string(&self) -> String;
}

macro_rules! config_enum {
    (@to_string to_string $s:expr) => { $s.to_string() };
    (@to_string name $s:expr) => { <Self>::name(*$s).to_string() };
    ($($t:path => $to_string:ident),* $(,)?) => {$(
        impl dandelion::compat::JavaEnum for $t {
            fn enum_constants() -> Vec<Self> {
                <$t>::values()
            }
            fn ordinal(&self) -> i32 {
                <$t>::ordinal(*self)
            }
            fn name(&self) -> String {
                <$t>::name(*self).to_string()
            }
        }
        impl ConfigEnum for $t {
            fn java_to_string(&self) -> String {
                config_enum!(@to_string $to_string self)
            }
        }
    )*};
}

config_enum! {
    crate::de::hysky::skyblocker::config::configs::dungeons_config::Type => to_string,
    crate::de::hysky::skyblocker::config::configs::event_notifications_config::Criterion => to_string,
    crate::de::hysky::skyblocker::config::configs::event_notifications_config::Sound => to_string,
    crate::de::hysky::skyblocker::config::configs::general_config::Average => to_string,
    crate::de::hysky::skyblocker::config::configs::general_config::Craft => to_string,
    crate::de::hysky::skyblocker::config::configs::general_config::ItemBackgroundStyle => to_string,
    crate::de::hysky::skyblocker::config::configs::general_config::SlotLockStyle => to_string,
    crate::de::hysky::skyblocker::config::configs::helper_config::FishingHookDisplay => to_string,
    crate::de::hysky::skyblocker::config::configs::mining_config::CommissionWaypointMode => to_string,
    crate::de::hysky::skyblocker::config::configs::misc_config::Info => to_string,
    crate::de::hysky::skyblocker::config::configs::slayers_config::FirePillar => to_string,
    crate::de::hysky::skyblocker::config::configs::slayers_config::HighlightSlayerEntities => to_string,
    crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::EnableState => to_string,
    crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::HighlightType => to_string,
    crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::IntelligenceDisplay => name,
    crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::TabHudStyle => to_string,
    crate::de::hysky::skyblocker::debug::debug::DumpFormat => name,
    crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_labels::RoomLabelType => to_string,
    crate::de::hysky::skyblocker::skyblock::gyro_overlay::Mode => to_string,
    crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity => to_string,
    crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_mode::SlotTextMode => to_string,
    crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::positioner::Positioner => to_string,
    crate::de::hysky::skyblocker::update_notifications::Channel => to_string,
    crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult => to_string,
    crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type => to_string,
}

/// `ConfigType` (Dandelion's): `toString()` is its translated name. Its `JavaEnum` form is Dandelion's own
/// (`dandelion::...::config_type`).
impl ConfigEnum for dandelion::net::azureaaron::dandelion::api::config_type::ConfigType {
    fn java_to_string(&self) -> String {
        dandelion::net::azureaaron::dandelion::api::config_type::ConfigType::to_string(self)
    }
}

/// `ChatFormatting` as the value type of an `Option.<ChatFormatting>`: the game's enum, wrapped because neither
/// `JavaEnum` (YACL's) nor `ChatFormatting` (the game's) belongs to this crate. Every binding wraps and unwraps it, so
/// the option reads and writes the `ChatFormatting` field itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChatFormattingEnum(pub ChatFormatting);

impl ChatFormattingEnum {
    /// `ConfigUtils.FORMATTING_FORMATTER` over the wrapped value.
    pub fn formatter() -> Arc<dyn Fn(ChatFormattingEnum) -> MutableComponent> {
        let formatter = crate::de::hysky::skyblocker::config::config_utils::FORMATTING_FORMATTER.get().clone();
        Arc::new(move |value: ChatFormattingEnum| formatter(value.0))
    }
}

impl dandelion::compat::JavaEnum for ChatFormattingEnum {
    fn enum_constants() -> Vec<Self> {
        mcrust::net::minecraft::chat_formatting::VALUES.iter().map(|f| ChatFormattingEnum(*f)).collect()
    }
    fn ordinal(&self) -> i32 {
        mcrust::net::minecraft::chat_formatting::VALUES.iter().position(|f| *f == self.0).map(|i| i as i32).unwrap_or(-1)
    }
    fn name(&self) -> String {
        crate::compat::chat_formatting_name(self.0).to_string()
    }
}

impl ConfigEnum for ChatFormattingEnum {
    /// `ChatFormatting.toString()`: the section sign and the code
    fn java_to_string(&self) -> String {
        self.0.to_string()
    }
}

use std::sync::Arc;

/// `KeyMappingOption.Builder.keyMapping(@Nullable KeyMapping)` for the key mappings Skyblocker keeps in a nullable
/// static (`Debug.dumpHoveredItemKey`, `HotbarSlotLock.hotbarSlotLock`, ..): Java hands the field as it is, so a mapping
/// that was never registered reaches the builder as `null`.
pub trait KeyMappingNullable {
    fn key_mapping_nullable(self, key_mapping: Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>) -> Self;
}

impl KeyMappingNullable for dandelion::net::azureaaron::dandelion::r#impl::key_mapping_option_impl::KeyMappingOptionBuilder {
    fn key_mapping_nullable(mut self, key_mapping: Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>) -> Self {
        self.key_mapping = key_mapping;
        self
    }
}
