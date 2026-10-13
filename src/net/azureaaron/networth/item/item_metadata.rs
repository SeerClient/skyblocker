//! java: @ApiStatus.Internal public sealed interface ItemMetadata permits DynamicItemMetadata (net.azureaaron.networth.item)
//!
//! This interface is used for retrieving formatted "metadata" from an item's custom data. It is part of the internal
//! implementation and subject to breaking changes at any time.
//!
//! `Object2IntMap<String>` is `Vec<(String, i32)>` and `IntList` is `Vec<i32>`. `gemstones()` hands out the map
//! behind a `RefCell` because `ItemCalculator.calculate` removes entries from it (`meta.gemstones().remove(..)`), as
//! the Java mutates the `Object2ObjectOpenHashMap` the record holds.

use std::cell::RefCell;

use mcrust::com::mojang::datafixers::util::either::Either;

use super::accessory_upgrades::AccessoryUpgrades;
use super::auction_bid_info::AuctionBidInfo;
use super::cosmetics::Cosmetics;
use super::drill_info::DrillInfo;
use super::dungeon_upgrades::DungeonUpgrades;
use super::gear_upgrades::GearUpgrades;
use super::gemstone::Gemstone;
use super::limited_edition_info::LimitedEditionInfo;
use super::misc_modifiers::MiscModifiers;
use super::pet_info::PetInfo;

/// java: public sealed interface ItemMetadata
pub trait ItemMetadata {
    /// java: Object2IntMap<String> enchantments();
    fn enchantments(&self) -> &Vec<(String, i32)>;

    /// java: int rarityUpgrades();
    fn rarity_upgrades(&self) -> i32;

    /// java: Optional<String> reforge();
    fn reforge(&self) -> Option<&String>;

    /// java: int upgradeLevel();
    fn upgrade_level(&self) -> i32;

    /// java: DungeonUpgrades dungeonUpgrades();
    fn dungeon_upgrades(&self) -> &DungeonUpgrades;

    /// java: GearUpgrades gearUpgrades();
    fn gear_upgrades(&self) -> &GearUpgrades;

    /// java: List<String> gemstoneSlots();
    fn gemstone_slots(&self) -> &Vec<String>;

    /// java: Map<String, Either<String, Gemstone>> gemstones();
    fn gemstones(&self) -> &RefCell<Vec<(String, Either<String, Gemstone>)>>;

    /// java: DrillInfo drillInfo();
    fn drill_info(&self) -> &DrillInfo;

    /// java: Object2IntMap<String> attributes();
    fn attributes(&self) -> &Vec<(String, i32)>;

    /// java: AuctionBidInfo auctionBidInfo();
    fn auction_bid_info(&self) -> &AuctionBidInfo;

    /// java: MiscModifiers miscModifiers();
    fn misc_modifiers(&self) -> &MiscModifiers;

    /// java: AccessoryUpgrades accessoryUpgrades();
    fn accessory_upgrades(&self) -> &AccessoryUpgrades;

    /// java: Cosmetics cosmetics();
    fn cosmetics(&self) -> &Cosmetics;

    /// java: PetInfo petInfo();
    fn pet_info(&self) -> &PetInfo;

    /// java: LimitedEditionInfo limitedEditionInfo();
    fn limited_edition_info(&self) -> &LimitedEditionInfo;

    /// java: IntList cakeBagCakeYears();
    fn cake_bag_cake_years(&self) -> &Vec<i32>;
}
