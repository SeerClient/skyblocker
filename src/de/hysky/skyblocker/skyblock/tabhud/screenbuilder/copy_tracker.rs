//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.CopyTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::utils::exclusive_grouping_set::ExclusiveGroupingSet;
use crate::de::hysky::skyblocker::utils::location;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<CopyTracker> CODEC = RecordCodecBuilder.create(instance -> instance.group( Layer.CODEC.fieldOf("hud").forGetter(CopyTracker::hud), Layer.CODEC.fieldOf("tab").forGetter(CopyTracker::tab), Layer.CODEC.fieldOf("secondary_tab").forGetter(CopyTracker::secondaryTab) ).apply(instance, CopyTracker::new));
pub static CODEC: compat::MainThread<CodecRef<CopyTracker>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &CopyTracker| o.hud(), codec::field_of(LAYER_CODEC.get().clone(), "hud")), record_codec_builder::of(|o: &CopyTracker| o.tab(), codec::field_of(LAYER_CODEC.get().clone(), "tab")), record_codec_builder::of(|o: &CopyTracker| o.secondary_tab(), codec::field_of(LAYER_CODEC.get().clone(), "secondary_tab")), move |__a0, __a1, __a2| CopyTracker::new_canonical(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java (Layer): private static final Codec<Set<Location>> LOCATION_SET_CODEC = Codec.either(Location.CODEC.listOf(), Codec.STRING).xmap(
///   e -> e.map(l -> l.isEmpty() ? EnumSet.noneOf(Location.class) : EnumSet.copyOf(l), _ -> EnumSet.copyOf(WidgetManager.ALLOWED_LOCATIONS)),
///   s -> s.equals(WidgetManager.ALLOWED_LOCATIONS) ? Either.right("everywhere") : Either.left(List.copyOf(s)));
/// (a list of locations, or any string for "all locations"; an `EnumSet` lists in ordinal order)
pub static LAYER_LOCATION_SET_CODEC: compat::MainThread<CodecRef<std::collections::HashSet<Location>>> = compat::MainThread::new(|| {
    codec::xmap(
        codec::either(codec::list(location::CODEC.get().clone()), codec::string_codec()),
        |e: Either<Vec<Location>, String>| match e {
            Either::Left(l) => l.into_iter().collect::<std::collections::HashSet<Location>>(),
            Either::Right(_) => widget_manager::ALLOWED_LOCATIONS.iter().copied().collect::<std::collections::HashSet<Location>>(),
        },
        |s: &std::collections::HashSet<Location>| {
            if s.len() == widget_manager::ALLOWED_LOCATIONS.len() && s.iter().all(|l| widget_manager::ALLOWED_LOCATIONS.contains(l)) {
                Either::Right("everywhere".to_string())
            } else {
                let mut list: Vec<Location> = s.iter().copied().collect();
                list.sort_by_key(|l| l.ordinal());
                Either::Left(list)
            }
        },
    )
});
/// java (Layer): private static final Codec<ExclusiveGroupingSet<Location>> LOCATION_SETS_CODEC = ExclusiveGroupingSet.getCodec(LOCATION_SET_CODEC);
pub static LAYER_LOCATION_SETS_CODEC: compat::MainThread<CodecRef<compat::Shared<ExclusiveGroupingSet<Location>>>> = compat::MainThread::new(|| ExclusiveGroupingSet::get_codec(LAYER_LOCATION_SET_CODEC.get()));
/// java (Layer): public static final Codec<Layer> CODEC = Codec.unboundedMap(Codec.STRING, LOCATION_SETS_CODEC)
///   .<Map<String, ExclusiveGroupingSet<Location>>>xmap(Object2ObjectOpenHashMap::new, Function.identity())
///   .xmap(Layer::new, Layer::map);
pub static LAYER_CODEC: compat::MainThread<CodecRef<Layer>> = compat::MainThread::new(|| {
    let map_codec = codec::unbounded_map(codec::string_codec(), LAYER_LOCATION_SETS_CODEC.get().clone());
    let hash_map_codec = codec::xmap(
        map_codec,
        |entries: Vec<(String, compat::Shared<ExclusiveGroupingSet<Location>>)>| entries.into_iter().collect::<std::collections::HashMap<_, _>>(),
        |map: &std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>(),
    );
    codec::xmap(hash_map_codec, |map| Layer::new_canonical(map), |layer: &Layer| compat::read(&layer.map, |m| m.clone()))
});

/// java: public record CopyTracker(Layer hud, Layer tab, Layer secondaryTab)
#[derive(Clone)]
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
        Self::new_canonical(Layer::new(), Layer::new(), Layer::new())
    }

    /// java: the canonical record constructor
    pub fn new_canonical(hud: Layer, tab: Layer, secondary_tab: Layer) -> Self {
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
    pub fn get(&self, layer: ScreenLayer) -> Layer {
        return match layer {
            ScreenLayer::Hud => self.hud.clone(),
            ScreenLayer::MainTab => self.tab.clone(),
            ScreenLayer::SecondaryTab => self.secondary_tab.clone(),
        };
    }

}

/// java: public record Layer(Map<String, ExclusiveGroupingSet<Location>> map)
/// The map is the record's one component and a mutable object (`getOrCreate` adds to it, `map().clear()` empties it),
/// so a `Layer` holds a REFERENCE to it: every copy of the record reaches the same map, as every holder of the Java
/// record does.
#[derive(Clone)]
pub struct Layer {
    /// java: Map<String, ExclusiveGroupingSet<Location>> map
    pub map: compat::Shared<std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>>,
}

impl Layer {
    /// java: public Layer()
    pub fn new() -> Self {
        Self::new_canonical(std::collections::HashMap::new())
    }

    /// java: the canonical record constructor
    pub fn new_canonical(map: std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>) -> Self {
        Self { map: compat::shared(map) }
    }

    /// java: the record accessor `map()` (the map object itself)
    pub fn map(&self) -> compat::Shared<std::collections::HashMap<String, compat::Shared<ExclusiveGroupingSet<Location>>>> {
        self.map.clone()
    }

    /// java: public Optional<ExclusiveGroupingSet<Location>> get(String widgetId)
    pub fn get(&self, widget_id: &str) -> Option<compat::Shared<ExclusiveGroupingSet<Location>>> {
        return compat::read(&self.map, |m| m.get(widget_id).cloned());
    }

    /// java: public ExclusiveGroupingSet<Location> getOrCreate(String widgetId)
    pub fn get_or_create(&self, widget_id: String) -> compat::Shared<ExclusiveGroupingSet<Location>> {
        return compat::write(&self.map, |m| m.entry(widget_id).or_insert_with(|| ExclusiveGroupingSet::new()).clone());
    }

}
