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
pub static CODEC: compat::MainThread<CodecRef<WidgetConfig>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &WidgetConfig| o.config(), codec::optional_field_of_opt(codec_utils::JSON_OBJECT_CODEC.get_mut().clone(), "config")), record_codec_builder::of(|o: &WidgetConfig| o.position(), codec::optional_field_of_opt(position_rule::CODEC.get_mut().clone(), "position")), move |__a0, __a1| WidgetConfig::new(__a0.clone(), __a1.clone()))));

/// java: public record WidgetConfig(Optional<JsonObject> config, Optional<PositionRule> position)
#[derive(Clone, Debug)]
pub struct WidgetConfig {
    /// java: Optional<JsonObject> config
    pub config: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: Optional<PositionRule> position
    pub position: Option<PositionRule>,
}

impl WidgetConfig {
    /// java: public WidgetConfig(JsonObject config, PositionRule position)
    pub fn new(mut config: serde_json::Map<String, serde_json::Value>, mut position: PositionRule) -> Self {
        // TODO(fabric2seer): this(..) to an unknown constructor
        //   java: this(Optional.of(config), Optional.of(position));
        let mut this: Self = todo!("constructor");
        this
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
        return WidgetConfig::new(None, None);
    }

    /// java: public WidgetConfig withPosition(@Nullable PositionRule position)
    pub fn with_position(&self, mut position: Option<PositionRule>) -> WidgetConfig {
        return WidgetConfig::new(self.config.clone().map(|__a0| __a0.clone()), position.clone());
    }

    /// java: public WidgetConfig withPosition(Function<PositionRule, @Nullable PositionRule> transformer)
    pub fn with_position_function(&self, transformer: Arc<dyn Fn(PositionRule) -> Option<PositionRule>>) -> WidgetConfig {
        // TODO(fabric2seer): not a lambda
        //   java: return new WidgetConfig(config.map(JsonObject::deepCopy), position.map(transformer));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
