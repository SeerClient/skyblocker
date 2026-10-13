//! java: @ApiStatus.Internal record DynamicItemMetadata(..) implements ItemMetadata (net.azureaaron.networth.item)
//!
//! Every `getOrThrow()` is `.get_or_throw()` followed by a panic carrying the `IllegalStateException`'s message: the
//! Java throws out of `SkyblockItemStack.of`, and Skyblocker's `NetworthCalculator.getItemNetworth` catches it
//! (`catch_unwind` there).

use std::cell::RefCell;
use std::sync::LazyLock;

use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::decoder::Decoder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike;
use mcrust::com::mojang::serialization::json_ops::JsonOps;

use super::accessory_upgrades::{self, AccessoryUpgrades};
use super::auction_bid_info::{self, AuctionBidInfo};
use super::cosmetics::{self, Cosmetics};
use super::drill_info::{self, DrillInfo};
use super::dungeon_upgrades::{self, DungeonUpgrades};
use super::gear_upgrades::{self, GearUpgrades};
use super::gemstone::{self, Gemstone};
use super::item_metadata::ItemMetadata;
use super::item_metadata_retriever::ItemMetadataRetriever;
use super::limited_edition_info::{self, LimitedEditionInfo};
use super::misc_modifiers::{self, MiscModifiers};
use super::pet_info::{self, PetInfo};
use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: private static final Codec<Object2IntMap<String>> OBJECT_2_INT_MAP_CODEC = CodecUtils.createObject2IntMapCodec(Codec.STRING);
static OBJECT_2_INT_MAP_CODEC: LazyLock<CodecRef<Vec<(String, i32)>>> =
    LazyLock::new(|| CodecUtils::create_object2_int_map_codec(codec::string_codec()));

/// java: private static final Function<Dynamic<?>, String> STRING_MAPPER = dynamic -> dynamic.asString("");
fn string_mapper(dynamic: Dynamic) -> String {
    dynamic.as_string_or("")
}

/// java: getOrThrow() - an error throws `IllegalStateException` with the error's message
fn get_or_throw<R>(result: mcrust::com::mojang::serialization::data_result::DataResult<R>) -> R {
    result
        .get_or_throw()
        .unwrap_or_else(|e| panic!("IllegalStateException: {e}"))
}

/// java: record DynamicItemMetadata(Object2IntMap<String> enchantments, int rarityUpgrades, Optional<String> reforge,
/// int upgradeLevel, DungeonUpgrades dungeonUpgrades, GearUpgrades gearUpgrades, List<String> gemstoneSlots,
/// Map<String, Either<String, Gemstone>> gemstones, DrillInfo drillInfo, Object2IntMap<String> attributes,
/// AuctionBidInfo auctionBidInfo, MiscModifiers miscModifiers, AccessoryUpgrades accessoryUpgrades, Cosmetics cosmetics,
/// PetInfo petInfo, LimitedEditionInfo limitedEditionInfo, IntList cakeBagCakeYears) implements ItemMetadata
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicItemMetadata {
    pub enchantments: Vec<(String, i32)>,
    pub rarity_upgrades: i32,
    pub reforge: Option<String>,
    pub upgrade_level: i32,
    pub dungeon_upgrades: DungeonUpgrades,
    pub gear_upgrades: GearUpgrades,
    pub gemstone_slots: Vec<String>,
    pub gemstones: RefCell<Vec<(String, Either<String, Gemstone>)>>,
    pub drill_info: DrillInfo,
    pub attributes: Vec<(String, i32)>,
    pub auction_bid_info: AuctionBidInfo,
    pub misc_modifiers: MiscModifiers,
    pub accessory_upgrades: AccessoryUpgrades,
    pub cosmetics: Cosmetics,
    pub pet_info: PetInfo,
    pub limited_edition_info: LimitedEditionInfo,
    pub cake_bag_cake_years: Vec<i32>,
}

impl DynamicItemMetadata {
    /// java: the canonical record constructor
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        enchantments: Vec<(String, i32)>,
        rarity_upgrades: i32,
        reforge: Option<String>,
        upgrade_level: i32,
        dungeon_upgrades: DungeonUpgrades,
        gear_upgrades: GearUpgrades,
        gemstone_slots: Vec<String>,
        gemstones: Vec<(String, Either<String, Gemstone>)>,
        drill_info: DrillInfo,
        attributes: Vec<(String, i32)>,
        auction_bid_info: AuctionBidInfo,
        misc_modifiers: MiscModifiers,
        accessory_upgrades: AccessoryUpgrades,
        cosmetics: Cosmetics,
        pet_info: PetInfo,
        limited_edition_info: LimitedEditionInfo,
        cake_bag_cake_years: Vec<i32>,
    ) -> Self {
        Self {
            enchantments,
            rarity_upgrades,
            reforge,
            upgrade_level,
            dungeon_upgrades,
            gear_upgrades,
            gemstone_slots,
            gemstones: RefCell::new(gemstones),
            drill_info,
            attributes,
            auction_bid_info,
            misc_modifiers,
            accessory_upgrades,
            cosmetics,
            pet_info,
            limited_edition_info,
            cake_bag_cake_years,
        }
    }

    /// java: static DynamicItemMetadata of(Dynamic<?> customData, ItemMetadataRetriever retriever)
    pub fn of(custom_data: &Dynamic, retriever: &dyn ItemMetadataRetriever) -> DynamicItemMetadata {
        //TODO Add an EMPTY state for each record and use that as a partial if all else fails, perhaps setup some logging for it too
        let enchantments: Vec<(String, i32)> = get_or_throw(
            OBJECT_2_INT_MAP_CODEC.parse_dynamic(&custom_data.get("enchantments").or_else_empty_map()),
        );
        let rarity_upgrades: i32 = custom_data.get("rarity_upgrades").as_int(0);
        let reforge: Option<String> = custom_data.get("modifier").result().map(string_mapper);
        let upgrade_level: i32 = custom_data.get("upgrade_level").as_int(0);
        let dungeon_upgrades: DungeonUpgrades =
            get_or_throw(dungeon_upgrades::CODEC.parse_dynamic(custom_data));
        let gear_upgrades: GearUpgrades = get_or_throw(gear_upgrades::CODEC.parse_dynamic(custom_data));
        let gemstone_slots: Vec<String> = get_or_throw(
            codec::list(codec::string_codec()).parse_dynamic(
                &custom_data
                    .get("gems")
                    .or_else_empty_map()
                    .get("unlocked_slots")
                    .or_else_empty_list(),
            ),
        );
        let gemstones: Vec<(String, Either<String, Gemstone>)> = get_or_throw(
            gemstone::MAP_EITHER_CODEC
                .parse_dynamic(&custom_data.get("gems").or_else_empty_map().remove("unlocked_slots")),
        );
        let drill_info: DrillInfo = get_or_throw(drill_info::CODEC.parse_dynamic(custom_data));
        let attributes: Vec<(String, i32)> = get_or_throw(
            OBJECT_2_INT_MAP_CODEC.parse_dynamic(&custom_data.get("attributes").or_else_empty_map()),
        );
        let auction_bid_info: AuctionBidInfo =
            get_or_throw(auction_bid_info::CODEC.parse_dynamic(custom_data));
        let misc_modifiers: MiscModifiers = get_or_throw(misc_modifiers::CODEC.parse_dynamic(custom_data));
        let accessory_upgrades: AccessoryUpgrades =
            get_or_throw(accessory_upgrades::CODEC.parse_dynamic(custom_data));
        let cosmetics: Cosmetics = get_or_throw(cosmetics::CODEC.parse_dynamic(custom_data));
        let pet_info: PetInfo = custom_data
            .get("petInfo")
            .result()
            //NB: The petInfo field is expected to be JSON if its present
            .map(|pet_info_json_dynamic| {
                // java: JsonParser.parseString(..) - a malformed string throws JsonSyntaxException
                let json = mcrust::lwjgl::json::parse_lenient(&pet_info_json_dynamic.as_string_or(""))
                    .unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                pet_info::CODEC
                    .parse(&JsonOps::instance(), &mcrust::com::mojang::serialization::obj(json))
                    .set_partial(PetInfo::empty())
                    .get_partial_or_throw()
                    .unwrap_or_else(|e| panic!("IllegalStateException: {e}"))
            })
            .unwrap_or_else(PetInfo::empty);
        let limited_edition_info: LimitedEditionInfo =
            get_or_throw(limited_edition_info::CODEC.parse_dynamic(custom_data));
        let cake_bag_cake_years: Vec<i32> = retriever.cake_bag_cake_years();

        DynamicItemMetadata::new(
            enchantments,
            rarity_upgrades,
            reforge,
            upgrade_level,
            dungeon_upgrades,
            gear_upgrades,
            gemstone_slots,
            gemstones,
            drill_info,
            attributes,
            auction_bid_info,
            misc_modifiers,
            accessory_upgrades,
            cosmetics,
            pet_info,
            limited_edition_info,
            cake_bag_cake_years,
        )
    }
}

impl ItemMetadata for DynamicItemMetadata {
    fn enchantments(&self) -> &Vec<(String, i32)> {
        &self.enchantments
    }

    fn rarity_upgrades(&self) -> i32 {
        self.rarity_upgrades
    }

    fn reforge(&self) -> Option<&String> {
        self.reforge.as_ref()
    }

    fn upgrade_level(&self) -> i32 {
        self.upgrade_level
    }

    fn dungeon_upgrades(&self) -> &DungeonUpgrades {
        &self.dungeon_upgrades
    }

    fn gear_upgrades(&self) -> &GearUpgrades {
        &self.gear_upgrades
    }

    fn gemstone_slots(&self) -> &Vec<String> {
        &self.gemstone_slots
    }

    fn gemstones(&self) -> &RefCell<Vec<(String, Either<String, Gemstone>)>> {
        &self.gemstones
    }

    fn drill_info(&self) -> &DrillInfo {
        &self.drill_info
    }

    fn attributes(&self) -> &Vec<(String, i32)> {
        &self.attributes
    }

    fn auction_bid_info(&self) -> &AuctionBidInfo {
        &self.auction_bid_info
    }

    fn misc_modifiers(&self) -> &MiscModifiers {
        &self.misc_modifiers
    }

    fn accessory_upgrades(&self) -> &AccessoryUpgrades {
        &self.accessory_upgrades
    }

    fn cosmetics(&self) -> &Cosmetics {
        &self.cosmetics
    }

    fn pet_info(&self) -> &PetInfo {
        &self.pet_info
    }

    fn limited_edition_info(&self) -> &LimitedEditionInfo {
        &self.limited_edition_info
    }

    fn cake_bag_cake_years(&self) -> &Vec<i32> {
        &self.cake_bag_cake_years
    }
}
