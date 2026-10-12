//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.LayerConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config::WidgetConfig;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final LayerConfig DUMMY = new LayerConfig();
pub static DUMMY: compat::MainThread<LayerConfig> = compat::MainThread::new(|| LayerConfig::new_0());
/// java: public static final Codec<LayerConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.STRING, WidgetConfig.CODEC).fieldOf("widgets").forGetter(c -> c.widgets) ).apply(instance, LayerConfig::new));
pub static CODEC: compat::MainThread<CodecRef<LayerConfig>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|c: &LayerConfig| compat::read(&c.widgets, |m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()), codec::field_of(codec::unbounded_map(codec::string_codec(), widget_config::CODEC.get().clone()), "widgets")), move |__a0: Vec<(String, WidgetConfig)>| LayerConfig::new(__a0.into_iter().collect()))));

/// java: public record LayerConfig(Map<String, WidgetConfig> widgets)
/// The map is a mutable object the record holds (`config.widgets().put(..)`, `removeIf`, `clear`): a `LayerConfig`
/// holds a reference to it, so every copy of the record reaches the same map, as every holder of the Java record does.
#[derive(Clone)]
pub struct LayerConfig {
    /// java: Map<String, WidgetConfig> widgets
    pub widgets: compat::Shared<std::collections::HashMap<String, WidgetConfig>>,
}

impl LayerConfig {
    /// java: public LayerConfig(Map<String, WidgetConfig> widgets) - `this.widgets = new Object2ObjectOpenHashMap<>(widgets)`
    pub fn new(widgets: std::collections::HashMap<String, WidgetConfig>) -> Self {
        Self { widgets: compat::shared(widgets) }
    }

    /// java: public LayerConfig()
    pub fn new_0() -> Self {
        Self::new(std::collections::HashMap::new())
    }

    /// java: the record accessor `widgets()` (the map object itself)
    pub fn widgets(&self) -> compat::Shared<std::collections::HashMap<String, WidgetConfig>> {
        self.widgets.clone()
    }

}
