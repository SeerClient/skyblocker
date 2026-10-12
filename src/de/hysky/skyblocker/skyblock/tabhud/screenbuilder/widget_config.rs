//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::utils::codec_utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<WidgetConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group( CodecUtils.JSON_OBJECT_CODEC.optionalFieldOf("config").forGetter(WidgetConfig::config), PositionRu
pub static CODEC: compat::MainThread<CodecRef<WidgetConfig>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &WidgetConfig| o.config(), codec::optional_field_of_opt(codec_utils::JSON_OBJECT_CODEC.get_mut().clone(), "config")), record_codec_builder::of(|o: &WidgetConfig| o.position(), codec::optional_field_of_opt(position_rule::CODEC.get_mut().clone(), "position")), move |__a0, __a1| WidgetConfig::new_canonical(__a0, __a1))));

/// java: public record WidgetConfig(Optional<JsonObject> config, Optional<PositionRule> position)
#[derive(Clone, Debug)]
pub struct WidgetConfig {
    /// java: Optional<JsonObject> config
    pub config: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: Optional<PositionRule> position
    pub position: Option<PositionRule>,
}

impl WidgetConfig {
    /// java: the canonical record constructor `WidgetConfig(Optional<JsonObject> config, Optional<PositionRule> position)`
    pub fn new_canonical(config: Option<serde_json::Map<String, serde_json::Value>>, position: Option<PositionRule>) -> Self {
        Self { config, position }
    }

    /// java: public WidgetConfig(JsonObject config, PositionRule position)
    pub fn new(config: serde_json::Map<String, serde_json::Value>, position: PositionRule) -> Self {
        // java: this(Optional.of(config), Optional.of(position));
        Self::new_canonical(Some(config), Some(position))
    }

    /// java: the record accessor `config()`
    pub fn config(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.config.clone()
    }

    /// java: the record accessor `position()`
    pub fn position(&self) -> Option<PositionRule> {
        self.position.clone()
    }

    /// java: public static WidgetConfig disabled()
    pub fn disabled() -> WidgetConfig {
        return WidgetConfig::new_canonical(None, None);
    }

    /// java: public WidgetConfig withPosition(@Nullable PositionRule position)
    pub fn with_position(&self, mut position: Option<PositionRule>) -> WidgetConfig {
        // java: new WidgetConfig(config.map(JsonObject::deepCopy), Optional.ofNullable(position))
        return WidgetConfig::new_canonical(self.config.clone(), position);
    }

    /// java: public WidgetConfig withPosition(Function<PositionRule, @Nullable PositionRule> transformer)
    pub fn with_position_function(&self, transformer: Arc<dyn Fn(PositionRule) -> Option<PositionRule>>) -> WidgetConfig {
        // java: return new WidgetConfig(config.map(JsonObject::deepCopy), position.map(transformer));
        // (`Optional.map` of a function answering null is empty)
        return WidgetConfig::new_canonical(self.config.clone(), self.position.clone().and_then(|p| transformer(p)));
    }

}
