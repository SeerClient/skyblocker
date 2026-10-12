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
/// java: public static final Codec<ScreenConfig> CODEC = RecordCodecBuilder.create(instance -> instance.group(
///   LayerConfig.CODEC.fieldOf("hud").forGetter(ScreenConfig::hud),
///   LayerConfig.CODEC.fieldOf("tab").forGetter(ScreenConfig::tab),
///   LayerConfig.CODEC.fieldOf("secondary_tab").forGetter(ScreenConfig::secondaryTab),
///   Codec.STRING.listOf().optionalFieldOf("hidden_tab_widgets", List.of()).<Set<String>>xmap(ObjectOpenHashSet::new, List::copyOf).forGetter(ScreenConfig::hiddenTabWidgets)
/// ).apply(instance, ScreenConfig::new));
pub static CODEC: compat::MainThread<CodecRef<ScreenConfig>> = compat::MainThread::new(|| {
    // `ObjectOpenHashSet::new` / `List::copyOf`
    let hidden_tab_widgets = mcrust::com::mojang::serialization::map_codec::xmap(
        codec::optional_field_of(codec::list(codec::string_codec()), "hidden_tab_widgets", Vec::new()),
        |list: Vec<String>| list.into_iter().collect::<std::collections::HashSet<String>>(),
        |set: &std::collections::HashSet<String>| set.iter().cloned().collect::<Vec<String>>(),
    );
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(|o: &ScreenConfig| o.hud(), codec::field_of(layer_config::CODEC.get().clone(), "hud")),
        record_codec_builder::of(|o: &ScreenConfig| o.tab(), codec::field_of(layer_config::CODEC.get().clone(), "tab")),
        record_codec_builder::of(|o: &ScreenConfig| o.secondary_tab(), codec::field_of(layer_config::CODEC.get().clone(), "secondary_tab")),
        record_codec_builder::of(|o: &ScreenConfig| compat::read(&o.hidden_tab_widgets, |s| s.clone()), hidden_tab_widgets),
        move |__a0, __a1, __a2, __a3| ScreenConfig::new_canonical(__a0, __a1, __a2, __a3),
    ))
});

/// java: public record ScreenConfig(LayerConfig hud, LayerConfig tab, LayerConfig secondaryTab, Set<String> hiddenTabWidgets)
/// (`hiddenTabWidgets` is a mutable set the record holds - `OtherOptionsScreen` adds to it - so the record holds a
/// reference to it, as `LayerConfig` does to its map)
#[derive(Clone)]
pub struct ScreenConfig {
    /// java: LayerConfig hud
    pub hud: LayerConfig,
    /// java: LayerConfig tab
    pub tab: LayerConfig,
    /// java: LayerConfig secondaryTab
    pub secondary_tab: LayerConfig,
    /// java: Set<String> hiddenTabWidgets
    pub hidden_tab_widgets: compat::Shared<std::collections::HashSet<String>>,
}

impl ScreenConfig {
    /// java: public ScreenConfig()
    pub fn new() -> Self {
        Self::new_canonical(LayerConfig::new_0(), LayerConfig::new_0(), LayerConfig::new_0(), std::collections::HashSet::new())
    }

    /// java: the canonical record constructor
    pub fn new_canonical(hud: LayerConfig, tab: LayerConfig, secondary_tab: LayerConfig, hidden_tab_widgets: std::collections::HashSet<String>) -> Self {
        Self { hud, tab, secondary_tab, hidden_tab_widgets: compat::shared(hidden_tab_widgets) }
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

    /// java: the record accessor `hiddenTabWidgets()` (the set object itself)
    pub fn hidden_tab_widgets(&self) -> compat::Shared<std::collections::HashSet<String>> {
        self.hidden_tab_widgets.clone()
    }

    /// java: public LayerConfig get(WidgetManager.ScreenLayer layer)
    pub fn get(&self, layer: ScreenLayer) -> LayerConfig {
        return match layer {
            ScreenLayer::Hud => self.hud(),
            ScreenLayer::MainTab => self.tab(),
            ScreenLayer::SecondaryTab => self.secondary_tab(),
        };
    }

    /// java: public Stream<LayerConfig> allLayers()
    pub fn all_layers(&self) -> std::vec::IntoIter<LayerConfig> {
        return vec![self.hud.clone(), self.tab.clone(), self.secondary_tab.clone()].into_iter();
    }

}
