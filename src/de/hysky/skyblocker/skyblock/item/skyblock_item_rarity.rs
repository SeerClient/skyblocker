//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockItemRarity by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::enum_utils::EnumUtils;
use crate::de::hysky::skyblocker::utils::sky_block_colors;
use crate::de::hysky::skyblocker::utils::sky_block_tooltip_styles;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<SkyblockItemRarity> CODEC = StringRepresentable.fromEnum(SkyblockItemRarity::values);
/// java: public static final Codec<SkyblockItemRarity> CODEC = StringRepresentable.fromEnum(SkyblockItemRarity::values);
pub static CODEC: compat::MainThread<CodecRef<SkyblockItemRarity>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum SkyblockItemRarity implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SkyblockItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Divine,
    Special,
    VerySpecial,
    Ultimate,
    Admin,
    Unknown,
}

impl SkyblockItemRarity {
    pub const VALUES: [SkyblockItemRarity; 12] = [SkyblockItemRarity::Common, SkyblockItemRarity::Uncommon, SkyblockItemRarity::Rare, SkyblockItemRarity::Epic, SkyblockItemRarity::Legendary, SkyblockItemRarity::Mythic, SkyblockItemRarity::Divine, SkyblockItemRarity::Special, SkyblockItemRarity::VerySpecial, SkyblockItemRarity::Ultimate, SkyblockItemRarity::Admin, SkyblockItemRarity::Unknown];

    /// java: values()
    pub fn values() -> Vec<SkyblockItemRarity> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SkyblockItemRarity::Common => "COMMON",
            SkyblockItemRarity::Uncommon => "UNCOMMON",
            SkyblockItemRarity::Rare => "RARE",
            SkyblockItemRarity::Epic => "EPIC",
            SkyblockItemRarity::Legendary => "LEGENDARY",
            SkyblockItemRarity::Mythic => "MYTHIC",
            SkyblockItemRarity::Divine => "DIVINE",
            SkyblockItemRarity::Special => "SPECIAL",
            SkyblockItemRarity::VerySpecial => "VERY_SPECIAL",
            SkyblockItemRarity::Ultimate => "ULTIMATE",
            SkyblockItemRarity::Admin => "ADMIN",
            SkyblockItemRarity::Unknown => "UNKNOWN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SkyblockItemRarity {
        match name {
            "COMMON" => SkyblockItemRarity::Common,
            "UNCOMMON" => SkyblockItemRarity::Uncommon,
            "RARE" => SkyblockItemRarity::Rare,
            "EPIC" => SkyblockItemRarity::Epic,
            "LEGENDARY" => SkyblockItemRarity::Legendary,
            "MYTHIC" => SkyblockItemRarity::Mythic,
            "DIVINE" => SkyblockItemRarity::Divine,
            "SPECIAL" => SkyblockItemRarity::Special,
            "VERY_SPECIAL" => SkyblockItemRarity::VerySpecial,
            "ULTIMATE" => SkyblockItemRarity::Ultimate,
            "ADMIN" => SkyblockItemRarity::Admin,
            "UNKNOWN" => SkyblockItemRarity::Unknown,
            _ => panic!("IllegalArgumentException: No enum constant SkyblockItemRarity.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SkyblockItemRarityFields {
        static FIELDS: LazyLock<Vec<SkyblockItemRarityFields>> = LazyLock::new(|| vec![
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::WHITE),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN),
            SkyblockItemRarityFields::new(sky_block_colors::BLUE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::BLUE),
            SkyblockItemRarityFields::new(sky_block_colors::DARK_PURPLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_PURPLE),
            SkyblockItemRarityFields::new(sky_block_colors::GOLD.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::LIGHT_PURPLE),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::RED),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::RED),
            SkyblockItemRarityFields::new(sky_block_colors::DARK_RED.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_RED),
            SkyblockItemRarityFields::new(sky_block_colors::DARK_RED.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_RED),
            SkyblockItemRarityFields::new_1(mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_GRAY),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public int getMP()
    pub fn get_mp(&self) -> i32 {
        return match *self {
            SkyblockItemRarity::Common | SkyblockItemRarity::Special => 3,
            SkyblockItemRarity::Uncommon | SkyblockItemRarity::VerySpecial => 5,
            SkyblockItemRarity::Rare => 8,
            SkyblockItemRarity::Epic => 12,
            SkyblockItemRarity::Legendary => 16,
            SkyblockItemRarity::Mythic | SkyblockItemRarity::Admin => 22,
            SkyblockItemRarity::Divine => 28,
            _ => 1,
        };
    }

    /// java: public SkyblockItemRarity recombobulate()
    pub fn recombobulate(&self) -> SkyblockItemRarity {
        return match *self {
            SkyblockItemRarity::Common => UNCOMMON,
            SkyblockItemRarity::Uncommon => RARE,
            SkyblockItemRarity::Rare => EPIC,
            SkyblockItemRarity::Epic => LEGENDARY,
            SkyblockItemRarity::Legendary => MYTHIC,
            SkyblockItemRarity::Mythic => DIVINE,
            SkyblockItemRarity::Divine => SPECIAL,
            SkyblockItemRarity::Special | SkyblockItemRarity::VerySpecial | SkyblockItemRarity::Ultimate => VERY_SPECIAL,
            _ => UNKNOWN,
        };
    }

    /// java: public Rarity toNeuRarity()
    pub fn to_neu_rarity(&self) -> crate::unresolved::Rarity {
        // TODO(fabric2seer): static field Rarity.COMMON
        //   java: return switch (this) {
        //   java: 			case SkyblockItemRarity.COMMON -> Rarity.COMMON;
        //   java: 			case SkyblockItemRarity.UNCOMMON -> Rarity.UNCOMMON;
        //   java: 			case SkyblockItemRarity.RARE -> Rarity.RARE;
        //   java: 			case SkyblockItemRarity.EPIC -> Rarity.EPIC;
        //   java: 			case SkyblockItemRarity.LEGENDARY -> Rarity.LEGENDARY;
        //   java: 			case SkyblockItemRarity.MYTHIC -> Rarity.MYTHIC;
        //   java: 			case SkyblockItemRarity.DIVINE -> Rarity.DIVINE;
        //   java: 			case SkyblockItemRarity.SPECIAL -> Rarity.SPECIAL;
        //   java: 			case SkyblockItemRarity.VERY_SPECIAL -> Rarity.VERY_SPECIAL;
        //   java: 			case SkyblockItemRarity.ULTIMATE -> Rarity.SUPREME;
        //   java: 			case SkyblockItemRarity.ADMIN, SkyblockItemRarity.UNKNOWN -> Rarity.UNKNOWN;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public @Nullable Identifier toTooltipStyle()
    pub fn to_tooltip_style(&self) -> Option<Identifier> {
        return Some(match *self {
            SkyblockItemRarity::Common => sky_block_tooltip_styles::COMMON.get_mut().clone(),
            SkyblockItemRarity::Uncommon => sky_block_tooltip_styles::UNCOMMON.get_mut().clone(),
            SkyblockItemRarity::Rare => sky_block_tooltip_styles::RARE.get_mut().clone(),
            SkyblockItemRarity::Epic => sky_block_tooltip_styles::EPIC.get_mut().clone(),
            SkyblockItemRarity::Legendary => sky_block_tooltip_styles::LEGENDARY.get_mut().clone(),
            SkyblockItemRarity::Mythic => sky_block_tooltip_styles::MYTHIC.get_mut().clone(),
            SkyblockItemRarity::Divine => sky_block_tooltip_styles::DIVINE.get_mut().clone(),
            SkyblockItemRarity::Special => sky_block_tooltip_styles::SPECIAL.get_mut().clone(),
            SkyblockItemRarity::VerySpecial => sky_block_tooltip_styles::VERY_SPECIAL.get_mut().clone(),
            SkyblockItemRarity::Ultimate => sky_block_tooltip_styles::ULTIMATE.get_mut().clone(),
            SkyblockItemRarity::Admin => sky_block_tooltip_styles::ADMIN.get_mut().clone(),
            _ => None,
        });
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return this.name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public SkyblockItemRarity next()
    pub fn next(&self) -> SkyblockItemRarity {
        return EnumUtils::cycle(self);
    }

    /// java: public static Optional<SkyblockItemRarity> containsName(String name)
    pub fn contains_name(name: &str) -> Option<SkyblockItemRarity> {
        // TODO(fabric2seer): static call Streams.findLast(..)
        //   java: return Streams.findLast(Arrays.stream(SkyblockItemRarity.values())
        //   java: 				.filter(rarity -> name.contains(rarity.toString()))
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static SkyblockItemRarity fromColor(int color)
    pub fn from_color(mut color: i32) -> SkyblockItemRarity {
        return SkyblockItemRarity::values().iter().copied().filter(|rarity| (argb::opaque(rarity.fields().color) == argb::opaque(color))).next().unwrap_or(UNKNOWN);
    }

}

impl std::fmt::Display for SkyblockItemRarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SkyblockItemRarity::to_string(*self))
    }
}

/// the constructor data of each SkyblockItemRarity constant
pub struct SkyblockItemRarityFields {
    pub name: String,
    pub color: i32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub legacy_color: i32,
}

impl SkyblockItemRarityFields {
    fn new(mut color: TextColor, mut legacy_color: TextColor) -> Self {
        let mut this = Self {
            name: Default::default(),
            color: 0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            legacy_color: 0,
        };
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: this.name = this.name().replace("_", " ");
        this.color = color.get_value();
        this.r = argb::red_float(this.color);
        this.g = argb::green_float(this.color);
        this.b = argb::blue_float(this.color);
        this.legacy_color = legacy_color.get_value();
        this
    }

    fn new_1(mut color: TextColor) -> Self {
        let mut this = Self {
            name: Default::default(),
            color: 0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            legacy_color: 0,
        };
        // TODO(fabric2seer): this statement
        //   java: this(color, color);
        this
    }

}
