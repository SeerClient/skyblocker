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

impl MiningCore {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "crystals") {
            if let Some(__x) = Some(Crystals::from_json(__v)) {
                self.crystals = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_mithril") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_mithril") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_mithril") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent_non_refundable1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_mithril_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_mithril_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent_non_refundable2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_mithril_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_mithril_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent_non_refundable3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_mithril_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_mithril_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent_non_refundable4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_mithril_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_mithril_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mithril_powder_spent_non_refundable5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_gemstone") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_gemstone") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_gemstone") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent_non_refundable1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_gemstone_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_gemstone_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent_non_refundable2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_gemstone_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_gemstone_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent_non_refundable3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_gemstone_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_gemstone_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent_non_refundable4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_gemstone_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_gemstone_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.gemstone_powder_spent_non_refundable5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_glacite") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_glacite") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_glacite") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent_non_refundable1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_glacite_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_glacite_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent_non_refundable2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_glacite_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_glacite_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent_non_refundable3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_glacite_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_glacite_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent_non_refundable4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_glacite_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "powder_spent_non_refundable_glacite_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.glacite_powder_spent_non_refundable5 = __x;
            }
        }
    }

    /// `GSON.fromJson(json, MiningCore.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Crystals {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "jade_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.jade_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "amber_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.amber_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "topaz_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.topaz_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "sapphire_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.sapphire_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "amethyst_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.amethyst_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "jasper_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.jasper_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "ruby_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.ruby_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "citrine_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.citrine_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "peridot_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.peridot_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "aquamarine_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.aquamarine_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "onyx_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.onyx_crystal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "opal_crystal") {
            if let Some(__x) = Some(CrystalData::from_json(__v)) {
                self.opal_crystal = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Crystals.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl CrystalData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "state") {
            if let Some(__x) = gson::as_string(__v) {
                self.state = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_placed") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.total_placed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_found") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total_found = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CrystalData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
