//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.ScreenConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config::LayerConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final ScreenConfig DUMMY = new ScreenConfig();
pub static DUMMY: compat::MainThread<ScreenConfig> = compat::MainThread::new(|| ScreenConfig::new());
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: public static final Codec<ScreenConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			LayerConfig.CODEC.fieldOf("hud").forGetter(ScreenConfig::hud),
//   java: 			LayerConfig.CODEC.fieldOf("tab").forGetter(ScreenConfig::tab),
//   java: 			LayerConfig.CODEC.fieldOf("secondary_tab").forGetter(ScreenConfig::secondaryTab),
//   java: 			// Must be optional for backwards compatibility with configs that did not have this
//   java: 			Codec.STRING.listOf().optionalFieldOf("hidden_tab_widgets", List.of()).<Set<String>>xmap(ObjectOpenHashSet::new, List::copyOf).forGetter(ScreenConfig::hiddenTabWidgets)
//   java: 	).apply(instance, ScreenConfig::new));
/// java: public static final Codec<ScreenConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group( LayerConfig.CODEC.fieldOf("hud").forGetter(ScreenConfig::hud), LayerConfig.CODEC.fieldOf("tab").fo
pub static CODEC: compat::MainThread<CodecRef<ScreenConfig>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public record ScreenConfig(LayerConfig hud, LayerConfig tab, LayerConfig secondaryTab, Set<String> hiddenTabWidgets)
#[derive(Clone, Debug)]
pub struct ScreenConfig {
    /// java: LayerConfig hud
    pub hud: LayerConfig,
    /// java: LayerConfig tab
    pub tab: LayerConfig,
    /// java: LayerConfig secondaryTab
    pub secondary_tab: LayerConfig,
    /// java: Set<String> hiddenTabWidgets
    pub hidden_tab_widgets: std::collections::HashSet<String>,
}

impl ScreenConfig {
    /// java: public ScreenConfig()
    pub fn new() -> Self {
        let mut this = Self::new(LayerConfig::new_0(), LayerConfig::new_0(), LayerConfig::new_0(), std::collections::HashSet::new());
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut hud: LayerConfig, mut tab: LayerConfig, mut secondary_tab: LayerConfig, mut hidden_tab_widgets: std::collections::HashSet<String>) -> Self {
        Self { hud, tab, secondary_tab, hidden_tab_widgets }
    }

    /// java: the record accessor `hud()`
    pub fn hud(&self) -> LayerConfig {
        self.hud.clone()
    }

    /// java: the record accessor `tab()`
    pub fn tab(&self) -> LayerConfig {
        self.tab.clone()
    }

    /// java: the record accessor `secondaryTab()`
    pub fn secondary_tab(&self) -> LayerConfig {
        self.secondary_tab.clone()
    }

    /// java: the record accessor `hiddenTabWidgets()`
    pub fn hidden_tab_widgets(&self) -> std::collections::HashSet<String> {
        self.hidden_tab_widgets.clone()
    }

    /// java: public LayerConfig get(WidgetManager.ScreenLayer layer)
    pub fn get(&self, mut layer: ScreenLayer) -> LayerConfig {
        return match layer {
            ScreenLayer::Hud => self.hud(),
            ScreenLayer::MainTab => self.tab(),
            ScreenLayer::SecondaryTab => self.secondary_tab(),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: public Stream<LayerConfig> allLayers()
    pub fn all_layers(&self) -> Box<dyn Iterator<Item = LayerConfig> + '_> {
        return Box::new(vec![self.hud.clone(), self.tab.clone(), self.secondary_tab.clone()].into_iter());
    }

}
