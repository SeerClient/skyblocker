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
// TODO(fabric2seer): field `widgets` of a value of unknown type
//   java: public static final Codec<LayerConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.unboundedMap(Codec.STRING, WidgetConfig.CODEC).fieldOf("widgets").forGetter(c -> c.widgets)
//   java: 	).apply(instance, LayerConfig::new));
/// java: public static final Codec<LayerConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.STRING, WidgetConfig.CODEC).fieldOf("widgets").forGetter(c -> c.widgets) )
pub static CODEC: compat::MainThread<CodecRef<LayerConfig>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public record LayerConfig(Map<String, WidgetConfig> widgets)
#[derive(Clone, Debug)]
pub struct LayerConfig {
    /// java: Map<String, WidgetConfig> widgets
    pub widgets: std::collections::HashMap<String, WidgetConfig>,
}

impl LayerConfig {
    /// java: public LayerConfig(Map<String, WidgetConfig> widgets)
    pub fn new(mut widgets: std::collections::HashMap<String, WidgetConfig>) -> Self {
        let __init_widgets = widgets.clone();
        let mut this = Self {
            widgets: __init_widgets,
        };
        this
    }

    /// java: public LayerConfig()
    pub fn new_0() -> Self {
        let mut this = Self::new(std::collections::HashMap::new());
        this
    }

    /// java: the record accessor `widgets()`
    pub fn widgets(&self) -> std::collections::HashMap<String, WidgetConfig> {
        self.widgets.clone()
    }

}
