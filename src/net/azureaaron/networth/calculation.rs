//! java: public record Calculation(Type type, String id, double price, int count) (net.azureaaron.networth)
//!
//! Used to describe a calculation made for the networth of an an object. There is a `LIST_CODEC` provided for
//! serializing a list of this to some other format such as JSON.

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::data_result::{self, DataResult};

/// java: private static final Codec<Calculation> CODEC
static CODEC: LazyLock<CodecRef<Calculation>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(
            |o: &Calculation| o.r#type,
            codec::field_of(
                // java: Codec.STRING.comapFlatMap(name -> { try { success(Type.valueOf(name)) } catch (IllegalArgumentException) { error } }, Type::name)
                codec::comap_flat_map(
                    codec::string_codec(),
                    |name: String| match Type::value_of(&name) {
                        Some(r#type) => DataResult::success(r#type),
                        None => DataResult::error(data_result::message(format!(
                            "Unknown calculation type: {name}"
                        ))),
                    },
                    |r#type: &Type| r#type.name().to_string(),
                ),
                "type",
            ),
        ),
        record_codec_builder::of(
            |o: &Calculation| o.id.clone(),
            codec::field_of(codec::string_codec(), "id"),
        ),
        record_codec_builder::of(
            |o: &Calculation| o.price,
            codec::field_of(codec::double_codec(), "price"),
        ),
        record_codec_builder::of(
            |o: &Calculation| o.count,
            codec::field_of(codec::int_codec(), "count"),
        ),
        Calculation::new,
    ))
});

/// java: public static final Codec<List<Calculation>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: LazyLock<CodecRef<Vec<Calculation>>> =
    LazyLock::new(|| codec::list(CODEC.clone()));

/// java: public record Calculation(Type type, String id, double price, int count)
#[derive(Clone, Debug, PartialEq)]
pub struct Calculation {
    pub r#type: Type,
    pub id: String,
    pub price: f64,
    pub count: i32,
}

impl Calculation {
    /// java: @ApiStatus.Internal public Calculation(Type type, String id, double price, int count)
    pub fn new(r#type: Type, id: String, price: f64, count: i32) -> Self {
        Self {
            r#type,
            id,
            price,
            count,
        }
    }

    /// java: static Calculation of(Type type, String id, double price)
    pub fn of(r#type: Type, id: String, price: f64) -> Self {
        Self::new(r#type, id, price, 1)
    }

    /// java: static Calculation of(Type type, String id, double price, int count)
    pub fn of_4(r#type: Type, id: String, price: f64, count: i32) -> Self {
        Self::new(r#type, id, price, count)
    }

    /// java: Calculation add(double addedPrice)
    pub fn add(&self, added_price: f64) -> Self {
        Self::new(
            self.r#type,
            self.id.clone(),
            self.price + added_price,
            self.count,
        )
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> Type {
        self.r#type
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `price()`
    pub fn price(&self) -> f64 {
        self.price
    }

    /// java: the record accessor `count()`
    pub fn count(&self) -> i32 {
        self.count
    }
}

/// java: public enum Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Star,
    Prestige,
    GodRoll,
    ShenAuction,
    WinningBid,
    Enchantment,
    Skin,
    Silex,
    GoldenBounty,
    Attribute,
    PocketSackInASack,
    WoodSingularity,
    JalapenoBook,
    TransmissionTuner,
    ManaDisintegrator,
    ThunderInABottle,
    Rune,
    FumingPotatoBook,
    HotPotatoBook,
    Dye,
    ArtOfWar,
    ArtOfPeace,
    FarmingForDummies,
    TalismanEnrichment,
    Recombobulator,
    GemstoneSlot,
    Gemstone,
    PowerScroll,
    Reforge,
    MasterStar,
    WitherBladeScroll,
    DrillPart,
    PolarvoidBook,
    DivanPowderCoating,
    EtherwarpConduit,
    NewYearCakes,
    PetItem,
    SackItem,
    Essence,
}

impl Type {
    /// java: values()
    pub const VALUES: &'static [Type] = &[
        Type::Star,
        Type::Prestige,
        Type::GodRoll,
        Type::ShenAuction,
        Type::WinningBid,
        Type::Enchantment,
        Type::Skin,
        Type::Silex,
        Type::GoldenBounty,
        Type::Attribute,
        Type::PocketSackInASack,
        Type::WoodSingularity,
        Type::JalapenoBook,
        Type::TransmissionTuner,
        Type::ManaDisintegrator,
        Type::ThunderInABottle,
        Type::Rune,
        Type::FumingPotatoBook,
        Type::HotPotatoBook,
        Type::Dye,
        Type::ArtOfWar,
        Type::ArtOfPeace,
        Type::FarmingForDummies,
        Type::TalismanEnrichment,
        Type::Recombobulator,
        Type::GemstoneSlot,
        Type::Gemstone,
        Type::PowerScroll,
        Type::Reforge,
        Type::MasterStar,
        Type::WitherBladeScroll,
        Type::DrillPart,
        Type::PolarvoidBook,
        Type::DivanPowderCoating,
        Type::EtherwarpConduit,
        Type::NewYearCakes,
        Type::PetItem,
        Type::SackItem,
        Type::Essence,
    ];

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Type::Star => "STAR",
            Type::Prestige => "PRESTIGE",
            Type::GodRoll => "GOD_ROLL",
            Type::ShenAuction => "SHEN_AUCTION",
            Type::WinningBid => "WINNING_BID",
            Type::Enchantment => "ENCHANTMENT",
            Type::Skin => "SKIN",
            Type::Silex => "SILEX",
            Type::GoldenBounty => "GOLDEN_BOUNTY",
            Type::Attribute => "ATTRIBUTE",
            Type::PocketSackInASack => "POCKET_SACK_IN_A_SACK",
            Type::WoodSingularity => "WOOD_SINGULARITY",
            Type::JalapenoBook => "JALAPENO_BOOK",
            Type::TransmissionTuner => "TRANSMISSION_TUNER",
            Type::ManaDisintegrator => "MANA_DISINTEGRATOR",
            Type::ThunderInABottle => "THUNDER_IN_A_BOTTLE",
            Type::Rune => "RUNE",
            Type::FumingPotatoBook => "FUMING_POTATO_BOOK",
            Type::HotPotatoBook => "HOT_POTATO_BOOK",
            Type::Dye => "DYE",
            Type::ArtOfWar => "ART_OF_WAR",
            Type::ArtOfPeace => "ART_OF_PEACE",
            Type::FarmingForDummies => "FARMING_FOR_DUMMIES",
            Type::TalismanEnrichment => "TALISMAN_ENRICHMENT",
            Type::Recombobulator => "RECOMBOBULATOR",
            Type::GemstoneSlot => "GEMSTONE_SLOT",
            Type::Gemstone => "GEMSTONE",
            Type::PowerScroll => "POWER_SCROLL",
            Type::Reforge => "REFORGE",
            Type::MasterStar => "MASTER_STAR",
            Type::WitherBladeScroll => "WITHER_BLADE_SCROLL",
            Type::DrillPart => "DRILL_PART",
            Type::PolarvoidBook => "POLARVOID_BOOK",
            Type::DivanPowderCoating => "DIVAN_POWDER_COATING",
            Type::EtherwarpConduit => "ETHERWARP_CONDUIT",
            Type::NewYearCakes => "NEW_YEAR_CAKES",
            Type::PetItem => "PET_ITEM",
            Type::SackItem => "SACK_ITEM",
            Type::Essence => "ESSENCE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Option<Type> {
        Type::VALUES.iter().copied().find(|t| t.name() == name)
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

/// java: Enum.toString() - the constant's name
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
