//! java: @ApiStatus.Internal public record DrillInfo(Optional<String> fuelTank, Optional<String> engine, Optional<String> upgradeModule, int polarvoids, OptionalInt pickonimbusDurability, OptionalInt divanPowderCoating)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<DrillInfo> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<DrillInfo>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply6(
        record_codec_builder::of(
            |o: &DrillInfo| o.fuel_tank.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "drill_part_fuel_tank"),
        ),
        record_codec_builder::of(
            |o: &DrillInfo| o.engine.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "drill_part_engine"),
        ),
        record_codec_builder::of(
            |o: &DrillInfo| o.upgrade_module.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "drill_part_upgrade_module"),
        ),
        record_codec_builder::of(
            |o: &DrillInfo| o.polarvoids,
            codec::optional_field_of(codec::int_codec(), "polarvoid", 0),
        ),
        record_codec_builder::of(
            |o: &DrillInfo| o.pickonimbus_durability,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "pickonimbus_durability",
            )),
        ),
        record_codec_builder::of(
            |o: &DrillInfo| o.divan_powder_coating,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "divan_powder_coating",
            )),
        ),
        DrillInfo::new,
    ))
});

/// java: @ApiStatus.Internal public record DrillInfo(Optional<String> fuelTank, Optional<String> engine, Optional<String> upgradeModule, int polarvoids, OptionalInt pickonimbusDurability, OptionalInt divanPowderCoating)
#[derive(Clone, Debug, PartialEq)]
pub struct DrillInfo {
    pub fuel_tank: Option<String>,
    pub engine: Option<String>,
    pub upgrade_module: Option<String>,
    pub polarvoids: i32,
    pub pickonimbus_durability: Option<i32>,
    pub divan_powder_coating: Option<i32>,
}

impl DrillInfo {
    /// java: the canonical record constructor
    pub fn new(
        fuel_tank: Option<String>,
        engine: Option<String>,
        upgrade_module: Option<String>,
        polarvoids: i32,
        pickonimbus_durability: Option<i32>,
        divan_powder_coating: Option<i32>,
    ) -> Self {
        Self {
            fuel_tank,
            engine,
            upgrade_module,
            polarvoids,
            pickonimbus_durability,
            divan_powder_coating,
        }
    }

    /// java: the record accessor `fuelTank()`
    pub fn fuel_tank(&self) -> Option<&String> {
        self.fuel_tank.as_ref()
    }

    /// java: the record accessor `engine()`
    pub fn engine(&self) -> Option<&String> {
        self.engine.as_ref()
    }

    /// java: the record accessor `upgradeModule()`
    pub fn upgrade_module(&self) -> Option<&String> {
        self.upgrade_module.as_ref()
    }

    /// java: the record accessor `polarvoids()`
    pub fn polarvoids(&self) -> i32 {
        self.polarvoids
    }

    /// java: the record accessor `pickonimbusDurability()`
    pub fn pickonimbus_durability(&self) -> Option<i32> {
        self.pickonimbus_durability
    }

    /// java: the record accessor `divanPowderCoating()`
    pub fn divan_powder_coating(&self) -> Option<i32> {
        self.divan_powder_coating
    }

    /// java: public boolean isAnyDrillPartPresent()
    pub fn is_any_drill_part_present(&self) -> bool {
        self.fuel_tank.is_some()
            || self.engine.is_some()
            || self.upgrade_module.is_some()
            || self.polarvoids > 0
    }
}
