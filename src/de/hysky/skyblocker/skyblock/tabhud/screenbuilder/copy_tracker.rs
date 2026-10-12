//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.CopyTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::utils::exclusive_grouping_set::ExclusiveGroupingSet;
use crate::de::hysky::skyblocker::utils::location;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<CopyTracker> CODEC = RecordCodecBuilder.create(instance -> instance.group( Layer.CODEC.fieldOf("hud").forGetter(CopyTracker::hud), Layer.CODEC.fieldOf("tab").forGetter(CopyTr
pub static CODEC: compat::MainThread<CodecRef<CopyTracker>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &CopyTracker| o.hud(), codec::field_of(LAYER_CODEC.get_mut().clone(), "hud")), record_codec_builder::of(|o: &CopyTracker| o.tab(), codec::field_of(LAYER_CODEC.get_mut().clone(), "tab")), record_codec_builder::of(|o: &CopyTracker| o.secondary_tab(), codec::field_of(LAYER_CODEC.get_mut().clone(), "secondary_tab")), move |__a0, __a1, __a2| CopyTracker::new(__a0.clone(), __a1.clone(), __a2.clone()))));
// TODO(fabric2seer): `.map(..)` on a A
//   java: private static final Codec<Set<Location>> LOCATION_SET_CODEC = Codec.either(Location.CODEC.listOf(), Codec.STRING).xmap(
//   java: 				// There was a bug where the Set<Location> could be empty which needs handling so the config can load (EnumSet#copyOf does not work with empty collections).
//   java: 				// Versions after 6.10.1 should not have this issue anymore and won't serialize an empty set.
//   java: 				e -> e.map(l -> l.isEmpty() ? EnumSet.noneOf(Location.class) : EnumSet.copyOf(l), _ -> EnumSet.copyOf(WidgetManager.ALLOWED_LOCATIONS)),
//   java: 				s -> s.equals(WidgetManager.ALLOWED_LOCATIONS) ? Either.right("everywhere") : Either.left(List.copyOf(s))
//   java: 		);
/// java: private static final Codec<Set<Location>> LOCATION_SET_CODEC = Codec.either(Location.CODEC.listOf(), Codec.STRING).xmap( // There was a bug where the Set<Location> could be empty which needs handling 
pub static LAYER_LOCATION_SET_CODEC: compat::MainThread<CodecRef<std::collections::HashSet<Location>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOCATION_SET_CODEC` - see the TODO above"));
// TODO(fabric2seer): `Codec.getCodec` has no Rust method in the port (looked for `get_codec` in Codec and JavaSrc)
//   java: private static final Codec<ExclusiveGroupingSet<Location>> LOCATION_SETS_CODEC = ExclusiveGroupingSet.getCodec(LOCATION_SET_CODEC);
/// java: private static final Codec<ExclusiveGroupingSet<Location>> LOCATION_SETS_CODEC = ExclusiveGroupingSet.getCodec(LOCATION_SET_CODEC);
pub static LAYER_LOCATION_SETS_CODEC: compat::MainThread<CodecRef<compat::Shared<ExclusiveGroupingSet<Location>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOCATION_SETS_CODEC` - see the TODO above"));
// TODO(fabric2seer): expression not parsed
//   java:
/// java: 
pub static LAYER_CODEC: compat::MainThread<CodecRef<Layer>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: 
pub static LAYER_EXCLUSIVE_GROUPING_SET: compat::MainThread<CodecRef<Layer>> = compat::MainThread::new(|| Default::default());

/// java: public record CopyTracker(Layer hud, Layer tab, Layer secondaryTab)
#[derive(Clone, Debug)]
pub struct CopyTracker {
    /// java: Layer hud
    pub hud: Layer,
    /// java: Layer tab
    pub tab: Layer,
    /// java: Layer secondaryTab
    pub secondary_tab: Layer,
}

impl CopyTracker {
    /// java: public CopyTracker()
    pub fn new() -> Self {
        let mut this = Self::new(Layer::new(), Layer::new(), Layer::new());
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut hud: Layer, mut tab: Layer, mut secondary_tab: Layer) -> Self {
        Self { hud, tab, secondary_tab }
    }

    /// java: the record accessor `hud()`
    pub fn hud(&self) -> Layer {
        self.hud.clone()
    }

    /// java: the record accessor `tab()`
    pub fn tab(&self) -> Layer {
        self.tab.clone()
    }

    /// java: the record accessor `secondaryTab()`
    pub fn secondary_tab(&self) -> Layer {
        self.secondary_tab.clone()
    }

    /// java: public Layer get(WidgetManager.ScreenLayer layer)
    pub fn get(&self, mut layer: ScreenLayer) -> Layer {
        return match layer {
            ScreenLayer::Hud => self.hud.clone(),
            ScreenLayer::MainTab => self.tab.clone(),
            ScreenLayer::SecondaryTab => self.secondary_tab.clone(),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

/// java: public record Layer(Map<String, ExclusiveGroupingSet<Location>> map)
#[derive(Clone, Debug)]
pub struct Layer {
    /// java: Map<String, ExclusiveGroupingSet<Location>> map
    pub map: std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>,
}

impl Layer {
    /// java: public Layer()
    pub fn new() -> Self {
        let mut this = Self::new(std::collections::HashMap::new());
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut map: std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>) -> Self {
        Self { map }
    }

    /// java: the record accessor `map()`
    pub fn map(&self) -> std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>> {
        self.map.clone()
    }

    /// java: public Optional<ExclusiveGroupingSet<Location>> get(String widgetId)
    pub fn get(&self, widget_id: &str) -> Option<compat::Shared<ExclusiveGroupingSet<Location>>> {
        return self.map.get(widget_id).cloned();
    }

    /// java: public ExclusiveGroupingSet<Location> getOrCreate(String widgetId)
    pub fn get_or_create(&mut self, mut widget_id: String) -> compat::Shared<ExclusiveGroupingSet<Location>> {
        return self.map.entry(widget_id.clone()).or_insert_with_key(|k| (|_| ExclusiveGroupingSet::new())(k.clone())).clone();
    }

}
