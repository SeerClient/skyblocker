//! Ported from de.hysky.skyblocker.skyblock.dwarven.fossil.FossilTypes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TileState;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TransformationOptions;

/// java: public enum FossilTypes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FossilTypes {
    Claw,
    Tusk,
    Ugly,
    Helix,
    Webbed,
    Footprint,
    Clubbed,
    Spine,
}

impl FossilTypes {
    pub const VALUES: [FossilTypes; 8] = [FossilTypes::Claw, FossilTypes::Tusk, FossilTypes::Ugly, FossilTypes::Helix, FossilTypes::Webbed, FossilTypes::Footprint, FossilTypes::Clubbed, FossilTypes::Spine];

    /// java: values()
    pub fn values() -> Vec<FossilTypes> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            FossilTypes::Claw => "CLAW",
            FossilTypes::Tusk => "TUSK",
            FossilTypes::Ugly => "UGLY",
            FossilTypes::Helix => "HELIX",
            FossilTypes::Webbed => "WEBBED",
            FossilTypes::Footprint => "FOOTPRINT",
            FossilTypes::Clubbed => "CLUBBED",
            FossilTypes::Spine => "SPINE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> FossilTypes {
        match name {
            "CLAW" => FossilTypes::Claw,
            "TUSK" => FossilTypes::Tusk,
            "UGLY" => FossilTypes::Ugly,
            "HELIX" => FossilTypes::Helix,
            "WEBBED" => FossilTypes::Webbed,
            "FOOTPRINT" => FossilTypes::Footprint,
            "CLUBBED" => FossilTypes::Clubbed,
            "SPINE" => FossilTypes::Spine,
            _ => panic!("IllegalArgumentException: No enum constant FossilTypes.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static FossilTypesFields {
        static FIELDS: LazyLock<Vec<FossilTypesFields>> = LazyLock::new(|| vec![
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty], vec![TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil], vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated180, TransformationOptions::FlipRotated90, TransformationOptions::FlipRotated180], "7.7".to_string(), 13, "Claw".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty], vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::FlipRotated270], "12.5".to_string(), 8, "Tusk".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty], vec![TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::Rotated270], "6.2".to_string(), 16, "Ugly".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::FlipRotated270], "7.1".to_string(), 14, "Helix".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty], vec![TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty], vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::FlipRotated0], "10".to_string(), 10, "Webbed".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty], vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::Rotated270], "7.7".to_string(), 13, "Footprint".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Fossil], vec![TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Fossil], vec![TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Empty, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Empty, TileState::Empty]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated180, TransformationOptions::FlipRotated0, TransformationOptions::FlipRotated180], "9.1".to_string(), 11, "Clubbed".to_string()),
            FossilTypesFields::new(vec![vec![TileState::Empty, TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Empty, TileState::Empty], vec![TileState::Empty, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Empty], vec![TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil, TileState::Fossil]], vec![TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::Rotated270], "8.3".to_string(), 12, "Spine".to_string()),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for FossilTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each FossilTypes constant
pub struct FossilTypesFields {
    pub rotations: Vec<TransformationOptions>,
    pub grid: Vec<Vec<TileState>>,
    pub percentage: String,
    pub tile_count: i32,
    pub name: String,
}

impl FossilTypesFields {
    fn new(mut grid: Vec<Vec<TileState>>, mut rotations: Vec<TransformationOptions>, mut percentage: String, mut tile_count: i32, mut name: String) -> Self {
        let mut this = Self {
            rotations: Default::default(),
            grid: Default::default(),
            percentage: Default::default(),
            tile_count: 0,
            name: Default::default(),
        };
        this.grid = grid.clone();
        this.rotations = rotations.clone();
        this.percentage = percentage.clone();
        this.tile_count = tile_count;
        this.name = name.clone();
        this
    }

}
