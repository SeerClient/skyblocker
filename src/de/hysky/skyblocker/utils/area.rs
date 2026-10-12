//! Ported from de.hysky.skyblocker.utils.Area by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: Area UNKNOWN = new Unknown();
pub static UNKNOWN: compat::MainThread<compat::Shared<dyn Area>> = compat::MainThread::new(|| (compat::shared(Unknown::new()) as compat::Shared<dyn Area>));

/// java: public sealed interface Area
pub trait Area: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: String displayName(); (dispatch: the body for this class)
    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String;

}

/// java: @SuppressWarnings("unchecked") static Area from(String name)
pub fn from(name: &str) -> compat::Shared<dyn Area> {
    // TODO(fabric2seer): `.getPermittedSubclasses(..)` on a Class
    //   java: return Arrays.stream((Class<Area>[]) Area.class.getPermittedSubclasses())
    //   java: 				.<Area>mapMulti((clazz, consumer) -> {
    //   java: 					// Implicitly excluding the Unknown record is fine since thats the fallback value anyways
    //   java: 					if (clazz.isEnum()) {
    //   java: 						for (Area area : clazz.getEnumConstants()) {
    //   java: 							consumer.accept(area);
    //   java: 						}
    //   java: 					}
    //   java: 				})
    //   java: 				.filter(area -> area.displayName().equals(name))
    //   java: 				.findFirst()
    //   java: 				.orElse(UNKNOWN);
    // `getPermittedSubclasses()` answers the sealed interface's subclasses in source order; only the enums
    // contribute constants (the Unknown record is the fallback anyway).
    macro_rules! find_in {
        ($($e:ident),*) => {
            $(
                for area in $e::values() {
                    if area.display_name() == name {
                        return compat::shared(area) as compat::Shared<dyn Area>;
                    }
                }
            )*
        };
    }
    find_in!(PrivateIsland, Hub, TheFarmingIslands, Galatea, TorrhusCanyon, TheEnd, CrimsonIsle, DwarvenMines, CrystalHollows, TheRift);
    return UNKNOWN.get().clone();
}

/// java: @Deprecated record Unknown() implements Area
#[derive(Clone, Debug, PartialEq)]
pub struct Unknown {}

impl Unknown {
    /// java: the canonical record constructor
    pub fn new() -> Self {
        Self {  }
    }

}

impl Area for Unknown {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Deprecated @Override public String displayName() (dispatch: the body for this class)
    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        Unknown::display_name_dyn
    }

}

impl Unknown {
    /// java: @Deprecated @Override public String displayName()
    pub fn display_name_impl(&self) -> String {
        return "Unknown".to_string();
    }

    /// Unknown.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<Self, _, _>(__this, |__s| __s.display_name_impl())
    }

}

/// java: enum PrivateIsland implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrivateIsland {
    YourIsland,
}

impl PrivateIsland {
    pub const VALUES: [PrivateIsland; 1] = [PrivateIsland::YourIsland];

    /// java: values()
    pub fn values() -> Vec<PrivateIsland> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            PrivateIsland::YourIsland => "YOUR_ISLAND",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> PrivateIsland {
        match name {
            "YOUR_ISLAND" => PrivateIsland::YourIsland,
            _ => panic!("IllegalArgumentException: No enum constant PrivateIsland.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static PrivateIslandFields {
        static FIELDS: LazyLock<Vec<PrivateIslandFields>> = LazyLock::new(|| vec![
            PrivateIslandFields::new("Your Island".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for PrivateIsland {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each PrivateIsland constant
pub struct PrivateIslandFields {
    pub display_name: String,
}

impl PrivateIslandFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for PrivateIsland {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        PrivateIsland::display_name_dyn
    }

}

impl PrivateIsland {
    /// PrivateIsland.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<PrivateIsland, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum Hub implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hub {
    Bank,
    Bazaar,
    Carnival,
    ForagingCamp,
    Forest,
}

impl Hub {
    pub const VALUES: [Hub; 5] = [Hub::Bank, Hub::Bazaar, Hub::Carnival, Hub::ForagingCamp, Hub::Forest];

    /// java: values()
    pub fn values() -> Vec<Hub> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Hub::Bank => "BANK",
            Hub::Bazaar => "BAZAAR",
            Hub::Carnival => "CARNIVAL",
            Hub::ForagingCamp => "FORAGING_CAMP",
            Hub::Forest => "FOREST",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Hub {
        match name {
            "BANK" => Hub::Bank,
            "BAZAAR" => Hub::Bazaar,
            "CARNIVAL" => Hub::Carnival,
            "FORAGING_CAMP" => Hub::ForagingCamp,
            "FOREST" => Hub::Forest,
            _ => panic!("IllegalArgumentException: No enum constant Hub.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static HubFields {
        static FIELDS: LazyLock<Vec<HubFields>> = LazyLock::new(|| vec![
            HubFields::new("Bank".to_string()),
            HubFields::new("Bazaar Alley".to_string()),
            HubFields::new("Carnival".to_string()),
            HubFields::new("Foraging Camp".to_string()),
            HubFields::new("Forest".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for Hub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Hub constant
pub struct HubFields {
    pub display_name: String,
}

impl HubFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for Hub {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        Hub::display_name_dyn
    }

}

impl Hub {
    /// Hub.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<Hub, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum TheFarmingIslands implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TheFarmingIslands {
    GlowingMushroomCave,
}

impl TheFarmingIslands {
    pub const VALUES: [TheFarmingIslands; 1] = [TheFarmingIslands::GlowingMushroomCave];

    /// java: values()
    pub fn values() -> Vec<TheFarmingIslands> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TheFarmingIslands::GlowingMushroomCave => "GLOWING_MUSHROOM_CAVE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TheFarmingIslands {
        match name {
            "GLOWING_MUSHROOM_CAVE" => TheFarmingIslands::GlowingMushroomCave,
            _ => panic!("IllegalArgumentException: No enum constant TheFarmingIslands.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TheFarmingIslandsFields {
        static FIELDS: LazyLock<Vec<TheFarmingIslandsFields>> = LazyLock::new(|| vec![
            TheFarmingIslandsFields::new("Glowing Mushroom Cave".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for TheFarmingIslands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each TheFarmingIslands constant
pub struct TheFarmingIslandsFields {
    pub display_name: String,
}

impl TheFarmingIslandsFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for TheFarmingIslands {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        TheFarmingIslands::display_name_dyn
    }

}

impl TheFarmingIslands {
    /// TheFarmingIslands.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<TheFarmingIslands, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum Galatea implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Galatea {
    ForestTemple,
}

impl Galatea {
    pub const VALUES: [Galatea; 1] = [Galatea::ForestTemple];

    /// java: values()
    pub fn values() -> Vec<Galatea> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Galatea::ForestTemple => "FOREST_TEMPLE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Galatea {
        match name {
            "FOREST_TEMPLE" => Galatea::ForestTemple,
            _ => panic!("IllegalArgumentException: No enum constant Galatea.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static GalateaFields {
        static FIELDS: LazyLock<Vec<GalateaFields>> = LazyLock::new(|| vec![
            GalateaFields::new("Forest Temple".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for Galatea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Galatea constant
pub struct GalateaFields {
    pub display_name: String,
}

impl GalateaFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for Galatea {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        Galatea::display_name_dyn
    }

}

impl Galatea {
    /// Galatea.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<Galatea, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum TorrhusCanyon implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TorrhusCanyon {
    TorrhusSprings,
    DesertTemple,
}

impl TorrhusCanyon {
    pub const VALUES: [TorrhusCanyon; 2] = [TorrhusCanyon::TorrhusSprings, TorrhusCanyon::DesertTemple];

    /// java: values()
    pub fn values() -> Vec<TorrhusCanyon> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TorrhusCanyon::TorrhusSprings => "TORRHUS_SPRINGS",
            TorrhusCanyon::DesertTemple => "DESERT_TEMPLE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TorrhusCanyon {
        match name {
            "TORRHUS_SPRINGS" => TorrhusCanyon::TorrhusSprings,
            "DESERT_TEMPLE" => TorrhusCanyon::DesertTemple,
            _ => panic!("IllegalArgumentException: No enum constant TorrhusCanyon.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TorrhusCanyonFields {
        static FIELDS: LazyLock<Vec<TorrhusCanyonFields>> = LazyLock::new(|| vec![
            TorrhusCanyonFields::new("Torrhus Springs".to_string()),
            TorrhusCanyonFields::new("Desert Temple".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for TorrhusCanyon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each TorrhusCanyon constant
pub struct TorrhusCanyonFields {
    pub display_name: String,
}

impl TorrhusCanyonFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for TorrhusCanyon {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        TorrhusCanyon::display_name_dyn
    }

}

impl TorrhusCanyon {
    /// TorrhusCanyon.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<TorrhusCanyon, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum TheEnd implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TheEnd {
    DragonsNest,
    TheEnd,
}

impl TheEnd {
    pub const VALUES: [TheEnd; 2] = [TheEnd::DragonsNest, TheEnd::TheEnd];

    /// java: values()
    pub fn values() -> Vec<TheEnd> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TheEnd::DragonsNest => "DRAGONS_NEST",
            TheEnd::TheEnd => "THE_END",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TheEnd {
        match name {
            "DRAGONS_NEST" => TheEnd::DragonsNest,
            "THE_END" => TheEnd::TheEnd,
            _ => panic!("IllegalArgumentException: No enum constant TheEnd.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TheEndFields {
        static FIELDS: LazyLock<Vec<TheEndFields>> = LazyLock::new(|| vec![
            TheEndFields::new("Dragon's Nest".to_string()),
            TheEndFields::new("The End".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for TheEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each TheEnd constant
pub struct TheEndFields {
    pub display_name: String,
}

impl TheEndFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for TheEnd {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        TheEnd::display_name_dyn
    }

}

impl TheEnd {
    /// TheEnd.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<TheEnd, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum CrimsonIsle implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrimsonIsle {
    Dojo,
}

impl CrimsonIsle {
    pub const VALUES: [CrimsonIsle; 1] = [CrimsonIsle::Dojo];

    /// java: values()
    pub fn values() -> Vec<CrimsonIsle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CrimsonIsle::Dojo => "DOJO",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CrimsonIsle {
        match name {
            "DOJO" => CrimsonIsle::Dojo,
            _ => panic!("IllegalArgumentException: No enum constant CrimsonIsle.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CrimsonIsleFields {
        static FIELDS: LazyLock<Vec<CrimsonIsleFields>> = LazyLock::new(|| vec![
            CrimsonIsleFields::new("Dojo".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for CrimsonIsle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each CrimsonIsle constant
pub struct CrimsonIsleFields {
    pub display_name: String,
}

impl CrimsonIsleFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for CrimsonIsle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        CrimsonIsle::display_name_dyn
    }

}

impl CrimsonIsle {
    /// CrimsonIsle.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<CrimsonIsle, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum DwarvenMines implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DwarvenMines {
    DwarvenBaseCamp,
    GlaciteMineshafts,
    GlaciteTunnels,
    GreatGlaciteLake,
}

impl DwarvenMines {
    pub const VALUES: [DwarvenMines; 4] = [DwarvenMines::DwarvenBaseCamp, DwarvenMines::GlaciteMineshafts, DwarvenMines::GlaciteTunnels, DwarvenMines::GreatGlaciteLake];

    /// java: values()
    pub fn values() -> Vec<DwarvenMines> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DwarvenMines::DwarvenBaseCamp => "DWARVEN_BASE_CAMP",
            DwarvenMines::GlaciteMineshafts => "GLACITE_MINESHAFTS",
            DwarvenMines::GlaciteTunnels => "GLACITE_TUNNELS",
            DwarvenMines::GreatGlaciteLake => "GREAT_GLACITE_LAKE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DwarvenMines {
        match name {
            "DWARVEN_BASE_CAMP" => DwarvenMines::DwarvenBaseCamp,
            "GLACITE_MINESHAFTS" => DwarvenMines::GlaciteMineshafts,
            "GLACITE_TUNNELS" => DwarvenMines::GlaciteTunnels,
            "GREAT_GLACITE_LAKE" => DwarvenMines::GreatGlaciteLake,
            _ => panic!("IllegalArgumentException: No enum constant DwarvenMines.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DwarvenMinesFields {
        static FIELDS: LazyLock<Vec<DwarvenMinesFields>> = LazyLock::new(|| vec![
            DwarvenMinesFields::new("Dwarven Base Camp".to_string()),
            DwarvenMinesFields::new("Glacite Mineshafts".to_string()),
            DwarvenMinesFields::new("Glacite Tunnels".to_string()),
            DwarvenMinesFields::new("Great Glacite Lake".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for DwarvenMines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each DwarvenMines constant
pub struct DwarvenMinesFields {
    pub display_name: String,
}

impl DwarvenMinesFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for DwarvenMines {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        DwarvenMines::display_name_dyn
    }

}

impl DwarvenMines {
    /// DwarvenMines.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<DwarvenMines, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum CrystalHollows implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrystalHollows {
    MinesOfDivan,
    JungleTemple,
}

impl CrystalHollows {
    pub const VALUES: [CrystalHollows; 2] = [CrystalHollows::MinesOfDivan, CrystalHollows::JungleTemple];

    /// java: values()
    pub fn values() -> Vec<CrystalHollows> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CrystalHollows::MinesOfDivan => "MINES_OF_DIVAN",
            CrystalHollows::JungleTemple => "JUNGLE_TEMPLE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CrystalHollows {
        match name {
            "MINES_OF_DIVAN" => CrystalHollows::MinesOfDivan,
            "JUNGLE_TEMPLE" => CrystalHollows::JungleTemple,
            _ => panic!("IllegalArgumentException: No enum constant CrystalHollows.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CrystalHollowsFields {
        static FIELDS: LazyLock<Vec<CrystalHollowsFields>> = LazyLock::new(|| vec![
            CrystalHollowsFields::new("Mines of Divan".to_string()),
            CrystalHollowsFields::new("Jungle Temple".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for CrystalHollows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each CrystalHollows constant
pub struct CrystalHollowsFields {
    pub display_name: String,
}

impl CrystalHollowsFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for CrystalHollows {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        CrystalHollows::display_name_dyn
    }

}

impl CrystalHollows {
    /// CrystalHollows.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<CrystalHollows, _, _>(__this, |__e| __e.display_name())
    }

}

/// java: enum TheRift implements Area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TheRift {
    Chateau,
    Mirrorverse,
}

impl TheRift {
    pub const VALUES: [TheRift; 2] = [TheRift::Chateau, TheRift::Mirrorverse];

    /// java: values()
    pub fn values() -> Vec<TheRift> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TheRift::Chateau => "CHATEAU",
            TheRift::Mirrorverse => "MIRRORVERSE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TheRift {
        match name {
            "CHATEAU" => TheRift::Chateau,
            "MIRRORVERSE" => TheRift::Mirrorverse,
            _ => panic!("IllegalArgumentException: No enum constant TheRift.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TheRiftFields {
        static FIELDS: LazyLock<Vec<TheRiftFields>> = LazyLock::new(|| vec![
            TheRiftFields::new("Stillgore Château".to_string()),
            TheRiftFields::new("Mirrorverse".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().display_name.clone();
    }

}

impl std::fmt::Display for TheRift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each TheRift constant
pub struct TheRiftFields {
    pub display_name: String,
}

impl TheRiftFields {
    fn new(mut display_name: String) -> Self {
        let mut this = Self {
            display_name: Default::default(),
        };
        this.display_name = display_name.clone();
        this
    }

}

impl Area for TheRift {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn display_name(&self) -> fn(&compat::Shared<dyn Area>) -> String {
        TheRift::display_name_dyn
    }

}

impl TheRift {
    /// TheRift.displayName as the dispatch of Area.displayName
    pub fn display_name_dyn(__this: &compat::Shared<dyn Area>) -> String {
        compat::read_as::<TheRift, _, _>(__this, |__e| __e.display_name())
    }

}
