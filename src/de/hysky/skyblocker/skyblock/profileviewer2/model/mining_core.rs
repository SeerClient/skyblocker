//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.MiningCore by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class MiningCore
#[derive(Clone)]
pub struct MiningCore {
    /// java: public Crystals crystals = new Crystals();
    pub crystals: Crystals,
    /// java: @SerializedName("powder_mithril") public long mithrilPowder;
    pub mithril_powder: i64,
    /// java: @SerializedName("powder_spent_mithril") public long mithrilPowderSpent1;
    pub mithril_powder_spent1: i64,
    /// java: @SerializedName("powder_spent_non_refundable_mithril") public long mithrilPowderSpentNonRefundable1;
    pub mithril_powder_spent_non_refundable1: i64,
    /// java: @SerializedName("powder_spent_mithril_2") public long mithrilPowderSpent2;
    pub mithril_powder_spent2: i64,
    /// java: @SerializedName("powder_spent_non_refundable_mithril_2") public long mithrilPowderSpentNonRefundable2;
    pub mithril_powder_spent_non_refundable2: i64,
    /// java: @SerializedName("powder_spent_mithril_3") public long mithrilPowderSpent3;
    pub mithril_powder_spent3: i64,
    /// java: @SerializedName("powder_spent_non_refundable_mithril_3") public long mithrilPowderSpentNonRefundable3;
    pub mithril_powder_spent_non_refundable3: i64,
    /// java: @SerializedName("powder_spent_mithril_4") public long mithrilPowderSpent4;
    pub mithril_powder_spent4: i64,
    /// java: @SerializedName("powder_spent_non_refundable_mithril_4") public long mithrilPowderSpentNonRefundable4;
    pub mithril_powder_spent_non_refundable4: i64,
    /// java: @SerializedName("powder_spent_mithril_5") public long mithrilPowderSpent5;
    pub mithril_powder_spent5: i64,
    /// java: @SerializedName("powder_spent_non_refundable_mithril_5") public long mithrilPowderSpentNonRefundable5;
    pub mithril_powder_spent_non_refundable5: i64,
    /// java: @SerializedName("powder_gemstone") public long gemstonePowder;
    pub gemstone_powder: i64,
    /// java: @SerializedName("powder_spent_gemstone") public long gemstonePowderSpent1;
    pub gemstone_powder_spent1: i64,
    /// java: @SerializedName("powder_spent_non_refundable_gemstone") public long gemstonePowderSpentNonRefundable1;
    pub gemstone_powder_spent_non_refundable1: i64,
    /// java: @SerializedName("powder_spent_gemstone_2") public long gemstonePowderSpent2;
    pub gemstone_powder_spent2: i64,
    /// java: @SerializedName("powder_spent_non_refundable_gemstone_2") public long gemstonePowderSpentNonRefundable2;
    pub gemstone_powder_spent_non_refundable2: i64,
    /// java: @SerializedName("powder_spent_gemstone_3") public long gemstonePowderSpent3;
    pub gemstone_powder_spent3: i64,
    /// java: @SerializedName("powder_spent_non_refundable_gemstone_3") public long gemstonePowderSpentNonRefundable3;
    pub gemstone_powder_spent_non_refundable3: i64,
    /// java: @SerializedName("powder_spent_gemstone_4") public long gemstonePowderSpent4;
    pub gemstone_powder_spent4: i64,
    /// java: @SerializedName("powder_spent_non_refundable_gemstone_4") public long gemstonePowderSpentNonRefundable4;
    pub gemstone_powder_spent_non_refundable4: i64,
    /// java: @SerializedName("powder_spent_gemstone_5") public long gemstonePowderSpent5;
    pub gemstone_powder_spent5: i64,
    /// java: @SerializedName("powder_spent_non_refundable_gemstone_5") public long gemstonePowderSpentNonRefundable5;
    pub gemstone_powder_spent_non_refundable5: i64,
    /// java: @SerializedName("powder_glacite") public long glacitePowder;
    pub glacite_powder: i64,
    /// java: @SerializedName("powder_spent_glacite") public long glacitePowderSpent1;
    pub glacite_powder_spent1: i64,
    /// java: @SerializedName("powder_spent_non_refundable_glacite") public long glacitePowderSpentNonRefundable1;
    pub glacite_powder_spent_non_refundable1: i64,
    /// java: @SerializedName("powder_spent_glacite_2") public long glacitePowderSpent2;
    pub glacite_powder_spent2: i64,
    /// java: @SerializedName("powder_spent_non_refundable_glacite_2") public long glacitePowderSpentNonRefundable2;
    pub glacite_powder_spent_non_refundable2: i64,
    /// java: @SerializedName("powder_spent_glacite_3") public long glacitePowderSpent3;
    pub glacite_powder_spent3: i64,
    /// java: @SerializedName("powder_spent_non_refundable_glacite_3") public long glacitePowderSpentNonRefundable3;
    pub glacite_powder_spent_non_refundable3: i64,
    /// java: @SerializedName("powder_spent_glacite_4") public long glacitePowderSpent4;
    pub glacite_powder_spent4: i64,
    /// java: @SerializedName("powder_spent_non_refundable_glacite_4") public long glacitePowderSpentNonRefundable4;
    pub glacite_powder_spent_non_refundable4: i64,
    /// java: @SerializedName("powder_spent_glacite_5") public long glacitePowderSpent5;
    pub glacite_powder_spent5: i64,
    /// java: @SerializedName("powder_spent_non_refundable_glacite_5") public long glacitePowderSpentNonRefundable5;
    pub glacite_powder_spent_non_refundable5: i64,
}

impl MiningCore {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            crystals: Crystals::new(),
            mithril_powder: 0,
            mithril_powder_spent1: 0,
            mithril_powder_spent_non_refundable1: 0,
            mithril_powder_spent2: 0,
            mithril_powder_spent_non_refundable2: 0,
            mithril_powder_spent3: 0,
            mithril_powder_spent_non_refundable3: 0,
            mithril_powder_spent4: 0,
            mithril_powder_spent_non_refundable4: 0,
            mithril_powder_spent5: 0,
            mithril_powder_spent_non_refundable5: 0,
            gemstone_powder: 0,
            gemstone_powder_spent1: 0,
            gemstone_powder_spent_non_refundable1: 0,
            gemstone_powder_spent2: 0,
            gemstone_powder_spent_non_refundable2: 0,
            gemstone_powder_spent3: 0,
            gemstone_powder_spent_non_refundable3: 0,
            gemstone_powder_spent4: 0,
            gemstone_powder_spent_non_refundable4: 0,
            gemstone_powder_spent5: 0,
            gemstone_powder_spent_non_refundable5: 0,
            glacite_powder: 0,
            glacite_powder_spent1: 0,
            glacite_powder_spent_non_refundable1: 0,
            glacite_powder_spent2: 0,
            glacite_powder_spent_non_refundable2: 0,
            glacite_powder_spent3: 0,
            glacite_powder_spent_non_refundable3: 0,
            glacite_powder_spent4: 0,
            glacite_powder_spent_non_refundable4: 0,
            glacite_powder_spent5: 0,
            glacite_powder_spent_non_refundable5: 0,
        };
        this
    }

    /// java: public long getSpentMithrilPowder(int slot)
    pub fn get_spent_mithril_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.mithril_powder_spent1,
            2 => self.mithril_powder_spent2,
            3 => self.mithril_powder_spent3,
            4 => self.mithril_powder_spent4,
            5 => self.mithril_powder_spent5,
            _ => {
                panic!("IllegalArgumentException");
            }
        };
    }

    /// java: public long getSpentNonRefundableMithrilPowder(int slot)
    pub fn get_spent_non_refundable_mithril_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.mithril_powder_spent_non_refundable1,
            2 => self.mithril_powder_spent_non_refundable2,
            3 => self.mithril_powder_spent_non_refundable3,
            4 => self.mithril_powder_spent_non_refundable4,
            5 => self.mithril_powder_spent_non_refundable5,
            _ => {
                panic!("IllegalArgumentException");
            }
        };
    }

    /// java: public long getSpentGemstonePowder(int slot)
    pub fn get_spent_gemstone_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.gemstone_powder_spent1,
            2 => self.gemstone_powder_spent2,
            3 => self.gemstone_powder_spent3,
            4 => self.gemstone_powder_spent4,
            5 => self.gemstone_powder_spent5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public long getSpentNonRefundableGemstonePowder(int slot)
    pub fn get_spent_non_refundable_gemstone_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.gemstone_powder_spent_non_refundable1,
            2 => self.gemstone_powder_spent_non_refundable2,
            3 => self.gemstone_powder_spent_non_refundable3,
            4 => self.gemstone_powder_spent_non_refundable4,
            5 => self.gemstone_powder_spent_non_refundable5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public long getSpentGlacitePowder(int slot)
    pub fn get_spent_glacite_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.glacite_powder_spent1,
            2 => self.glacite_powder_spent2,
            3 => self.glacite_powder_spent3,
            4 => self.glacite_powder_spent4,
            5 => self.glacite_powder_spent5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public long getSpentNonRefundableGlacitePowder(int slot)
    pub fn get_spent_non_refundable_glacite_powder(&self, mut slot: i32) -> i64 {
        return match slot {
            1 => self.glacite_powder_spent_non_refundable1,
            2 => self.glacite_powder_spent_non_refundable2,
            3 => self.glacite_powder_spent_non_refundable3,
            4 => self.glacite_powder_spent_non_refundable4,
            5 => self.glacite_powder_spent_non_refundable5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

}

/// java: public static class Crystals
#[derive(Clone)]
pub struct Crystals {
    /// java: @SerializedName("jade_crystal") public CrystalData jadeCrystal = new CrystalData();
    pub jade_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("amber_crystal") public CrystalData amberCrystal = new CrystalData();
    pub amber_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("topaz_crystal") public CrystalData topazCrystal = new CrystalData();
    pub topaz_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("sapphire_crystal") public CrystalData sapphireCrystal = new CrystalData();
    pub sapphire_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("amethyst_crystal") public CrystalData amethystCrystal = new CrystalData();
    pub amethyst_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("jasper_crystal") public CrystalData jasperCrystal = new CrystalData();
    pub jasper_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("ruby_crystal") public CrystalData rubyCrystal = new CrystalData();
    pub ruby_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("citrine_crystal") public CrystalData citrineCrystal = new CrystalData();
    pub citrine_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("peridot_crystal") public CrystalData peridotCrystal = new CrystalData();
    pub peridot_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("aquamarine_crystal") public CrystalData aquamarineCrystal = new CrystalData();
    pub aquamarine_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("onyx_crystal") public CrystalData onyxCrystal = new CrystalData();
    pub onyx_crystal: compat::Shared<CrystalData>,
    /// java: @SerializedName("opal_crystal") public CrystalData opalCrystal = new CrystalData();
    pub opal_crystal: compat::Shared<CrystalData>,
}

impl Crystals {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            jade_crystal: CrystalData::new(),
            amber_crystal: CrystalData::new(),
            topaz_crystal: CrystalData::new(),
            sapphire_crystal: CrystalData::new(),
            amethyst_crystal: CrystalData::new(),
            jasper_crystal: CrystalData::new(),
            ruby_crystal: CrystalData::new(),
            citrine_crystal: CrystalData::new(),
            peridot_crystal: CrystalData::new(),
            aquamarine_crystal: CrystalData::new(),
            onyx_crystal: CrystalData::new(),
            opal_crystal: CrystalData::new(),
        };
        this
    }

}

/// java: public static class CrystalData
#[derive(Clone)]
pub struct CrystalData {
    /// java: public String state = "";
    pub state: String,
    /// java: @SerializedName("total_placed") public @Nullable Integer totalPlaced;
    pub total_placed: Option<i32>,
    /// java: @SerializedName("total_found") public int totalFound;
    pub total_found: i32,
}

impl CrystalData {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            state: "".to_string(),
            total_placed: None,
            total_found: 0,
        });
        __this
    }

}
