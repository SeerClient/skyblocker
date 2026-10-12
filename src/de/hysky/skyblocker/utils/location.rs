//! Ported from de.hysky.skyblocker.utils.Location by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public static final Codec<Location> CODEC = StringRepresentable.fromEnum(Location::values);
pub static CODEC: compat::MainThread<CodecRef<Location>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Location::values(), |e: &Location| e.get_serialized_name()));
/// java: public static final Codec<EnumSet<Location>> SET_CODEC = CodecUtils.enumSetCodec(CODEC, Location.class);
pub static SET_CODEC: compat::MainThread<CodecRef<std::collections::HashSet<Location>>> = compat::MainThread::new(|| CodecUtils::enum_set_codec(CODEC.get_mut(), &std::any::TypeId::of::<Location>()));

/// java: public enum Location implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Location {
    PrivateIsland,
    Garden,
    Hub,
    TheFarmingIsland,
    ThePark,
    SpidersDen,
    BlazingFortress,
    TheEnd,
    CrimsonIsle,
    GoldMine,
    DeepCaverns,
    DwarvenMines,
    BackwaterBayou,
    DungeonHub,
    WinterIsland,
    TheRift,
    DarkAuction,
    CrystalHollows,
    Dungeon,
    KuudrasHollow,
    GlaciteMineshafts,
    Galatea,
    TorrhusCanyon,
    Safari,
    LotusAtoll,
    Unknown,
}

impl Location {
    pub const VALUES: [Location; 26] = [Location::PrivateIsland, Location::Garden, Location::Hub, Location::TheFarmingIsland, Location::ThePark, Location::SpidersDen, Location::BlazingFortress, Location::TheEnd, Location::CrimsonIsle, Location::GoldMine, Location::DeepCaverns, Location::DwarvenMines, Location::BackwaterBayou, Location::DungeonHub, Location::WinterIsland, Location::TheRift, Location::DarkAuction, Location::CrystalHollows, Location::Dungeon, Location::KuudrasHollow, Location::GlaciteMineshafts, Location::Galatea, Location::TorrhusCanyon, Location::Safari, Location::LotusAtoll, Location::Unknown];

    /// java: values()
    pub fn values() -> Vec<Location> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Location::PrivateIsland => "PRIVATE_ISLAND",
            Location::Garden => "GARDEN",
            Location::Hub => "HUB",
            Location::TheFarmingIsland => "THE_FARMING_ISLAND",
            Location::ThePark => "THE_PARK",
            Location::SpidersDen => "SPIDERS_DEN",
            Location::BlazingFortress => "BLAZING_FORTRESS",
            Location::TheEnd => "THE_END",
            Location::CrimsonIsle => "CRIMSON_ISLE",
            Location::GoldMine => "GOLD_MINE",
            Location::DeepCaverns => "DEEP_CAVERNS",
            Location::DwarvenMines => "DWARVEN_MINES",
            Location::BackwaterBayou => "BACKWATER_BAYOU",
            Location::DungeonHub => "DUNGEON_HUB",
            Location::WinterIsland => "WINTER_ISLAND",
            Location::TheRift => "THE_RIFT",
            Location::DarkAuction => "DARK_AUCTION",
            Location::CrystalHollows => "CRYSTAL_HOLLOWS",
            Location::Dungeon => "DUNGEON",
            Location::KuudrasHollow => "KUUDRAS_HOLLOW",
            Location::GlaciteMineshafts => "GLACITE_MINESHAFTS",
            Location::Galatea => "GALATEA",
            Location::TorrhusCanyon => "TORRHUS_CANYON",
            Location::Safari => "SAFARI",
            Location::LotusAtoll => "LOTUS_ATOLL",
            Location::Unknown => "UNKNOWN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Location {
        match name {
            "PRIVATE_ISLAND" => Location::PrivateIsland,
            "GARDEN" => Location::Garden,
            "HUB" => Location::Hub,
            "THE_FARMING_ISLAND" => Location::TheFarmingIsland,
            "THE_PARK" => Location::ThePark,
            "SPIDERS_DEN" => Location::SpidersDen,
            "BLAZING_FORTRESS" => Location::BlazingFortress,
            "THE_END" => Location::TheEnd,
            "CRIMSON_ISLE" => Location::CrimsonIsle,
            "GOLD_MINE" => Location::GoldMine,
            "DEEP_CAVERNS" => Location::DeepCaverns,
            "DWARVEN_MINES" => Location::DwarvenMines,
            "BACKWATER_BAYOU" => Location::BackwaterBayou,
            "DUNGEON_HUB" => Location::DungeonHub,
            "WINTER_ISLAND" => Location::WinterIsland,
            "THE_RIFT" => Location::TheRift,
            "DARK_AUCTION" => Location::DarkAuction,
            "CRYSTAL_HOLLOWS" => Location::CrystalHollows,
            "DUNGEON" => Location::Dungeon,
            "KUUDRAS_HOLLOW" => Location::KuudrasHollow,
            "GLACITE_MINESHAFTS" => Location::GlaciteMineshafts,
            "GALATEA" => Location::Galatea,
            "TORRHUS_CANYON" => Location::TorrhusCanyon,
            "SAFARI" => Location::Safari,
            "LOTUS_ATOLL" => Location::LotusAtoll,
            "UNKNOWN" => Location::Unknown,
            _ => panic!("IllegalArgumentException: No enum constant Location.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static LocationFields {
        static FIELDS: LazyLock<Vec<LocationFields>> = LazyLock::new(|| vec![
            LocationFields::new("dynamic".to_string(), "Private Island".to_string()),
            LocationFields::new("garden".to_string(), "Garden".to_string()),
            LocationFields::new("hub".to_string(), "Hub".to_string()),
            LocationFields::new("farming_1".to_string(), "The Farming Islands".to_string()),
            LocationFields::new("foraging_1".to_string(), "The Park".to_string()),
            LocationFields::new("combat_1".to_string(), "Spider's Den".to_string()),
            LocationFields::new("combat_2".to_string(), "Blazing Fortress".to_string()),
            LocationFields::new("combat_3".to_string(), "The End".to_string()),
            LocationFields::new("crimson_isle".to_string(), "Crimson Isle".to_string()),
            LocationFields::new("mining_1".to_string(), "Gold Mine".to_string()),
            LocationFields::new("mining_2".to_string(), "Deep Caverns".to_string()),
            LocationFields::new("mining_3".to_string(), "Dwarven Mines".to_string()),
            LocationFields::new("fishing_1".to_string(), "Backwater Bayou".to_string()),
            LocationFields::new("dungeon_hub".to_string(), "Dungeon Hub".to_string()),
            LocationFields::new("winter".to_string(), "Jerry's Workshop".to_string()),
            LocationFields::new("rift".to_string(), "The Rift".to_string()),
            LocationFields::new("dark_auction".to_string(), "Dark Auction".to_string()),
            LocationFields::new("crystal_hollows".to_string(), "Crystal Hollows".to_string()),
            LocationFields::new("dungeon".to_string(), "Dungeons".to_string()),
            LocationFields::new("kuudra".to_string(), "Kuudra's Hollow".to_string()),
            LocationFields::new("mineshaft".to_string(), "Glacite Mineshafts".to_string()),
            LocationFields::new("foraging_2".to_string(), "Moonglade Marsh".to_string()),
            LocationFields::new("foraging_3".to_string(), "Torrhus Canyon".to_string()),
            LocationFields::new("safari".to_string(), "Safari".to_string()),
            LocationFields::new("lotus_atoll".to_string(), "Lotus Atoll".to_string()),
            LocationFields::new("unknown".to_string(), "Unknown".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public String id()
    pub fn id(&self) -> String {
        return self.fields().id.clone();
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.id();
    }

    /// java: public static Location from(String id)
    pub fn from(id: &str) -> Location {
        return Location::values().iter().copied().filter(|loc| (loc.fields().id == id)).next().unwrap_or(Location::Unknown);
    }

    /// java: public static Location fromFriendlyName(String friendlyName)
    pub fn from_friendly_name(friendly_name: &str) -> Location {
        return Location::values().iter().copied().filter(|loc| compat::equals_ignore_case(&loc.fields().friendly_name, friendly_name)).next().unwrap_or(Location::Unknown);
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().friendly_name.clone();
    }

}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Location::to_string(self))
    }
}

/// the constructor data of each Location constant
pub struct LocationFields {
    pub id: String,
    pub friendly_name: String,
}

impl LocationFields {
    fn new(mut id: String, mut friendly_name: String) -> Self {
        let mut this = Self {
            id: Default::default(),
            friendly_name: Default::default(),
        };
        this.id = id.clone();
        this.friendly_name = friendly_name.clone();
        this
    }

}
