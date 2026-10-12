//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.GlacitePlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class GlacitePlayerData
#[derive(Clone)]
pub struct GlacitePlayerData {
    /// java: @SerializedName("fossil_dust") public double fossilDust;
    pub fossil_dust: f64,
    /// java: @SerializedName("mineshafts_entered") public int mineshaftsEntered;
    pub mineshafts_entered: i32,
    /// java: @SerializedName("corpses_looted") public CorpsesLooted corpsesLooted = new CorpsesLooted();
    pub corpses_looted: CorpsesLooted,
    /// java: @SerializedName("fossils_donated") public List<String> fossilsDonated = new ArrayList<>();
    pub fossils_donated: Vec<String>,
}

impl GlacitePlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fossil_dust: 0.0,
            mineshafts_entered: 0,
            corpses_looted: CorpsesLooted::new(),
            fossils_donated: Vec::new(),
        };
        this
    }

}

/// java: public static class CorpsesLooted
#[derive(Clone)]
pub struct CorpsesLooted {
    /// java: public int lapis;
    pub lapis: i32,
    /// java: public int umber;
    pub umber: i32,
    /// java: public int tungsten;
    pub tungsten: i32,
    /// java: public int vanguard;
    pub vanguard: i32,
}

impl CorpsesLooted {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            lapis: 0,
            umber: 0,
            tungsten: 0,
            vanguard: 0,
        };
        this
    }

    /// java: public int getTotalCorpsesLooted()
    pub fn get_total_corpses_looted(&self) -> i32 {
        return ((self.lapis.wrapping_add(self.umber)).wrapping_add(self.tungsten)).wrapping_add(self.vanguard);
    }

}

/// java: public enum Fossil
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Fossil {
    Claw,
    Spine,
    Clubbed,
    Ugly,
    Helix,
    Footprint,
    Webbed,
    Tusk,
}

impl Fossil {
    pub const VALUES: [Fossil; 8] = [Fossil::Claw, Fossil::Spine, Fossil::Clubbed, Fossil::Ugly, Fossil::Helix, Fossil::Footprint, Fossil::Webbed, Fossil::Tusk];

    /// java: values()
    pub fn values() -> Vec<Fossil> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Fossil::Claw => "CLAW",
            Fossil::Spine => "SPINE",
            Fossil::Clubbed => "CLUBBED",
            Fossil::Ugly => "UGLY",
            Fossil::Helix => "HELIX",
            Fossil::Footprint => "FOOTPRINT",
            Fossil::Webbed => "WEBBED",
            Fossil::Tusk => "TUSK",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Fossil {
        match name {
            "CLAW" => Fossil::Claw,
            "SPINE" => Fossil::Spine,
            "CLUBBED" => Fossil::Clubbed,
            "UGLY" => Fossil::Ugly,
            "HELIX" => Fossil::Helix,
            "FOOTPRINT" => Fossil::Footprint,
            "WEBBED" => Fossil::Webbed,
            "TUSK" => Fossil::Tusk,
            _ => panic!("IllegalArgumentException: No enum constant Fossil.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static FossilFields {
        static FIELDS: LazyLock<Vec<FossilFields>> = LazyLock::new(|| vec![
            FossilFields::new("Claw".to_string()),
            FossilFields::new("Spine".to_string()),
            FossilFields::new("Clubbed".to_string()),
            FossilFields::new("Ugly".to_string()),
            FossilFields::new("Helix".to_string()),
            FossilFields::new("Footprint".to_string()),
            FossilFields::new("Webbed".to_string()),
            FossilFields::new("Tusk".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static List<Fossil> fromDonated(List<String> donated)
    pub fn from_donated(donated: &Vec<String>) -> Vec<Fossil> {
        return compat::sorted_by(Fossil::values().iter().copied().filter(|fossil| donated.iter().any(|e| *e == fossil.name())), |a, b| { let key = |__a0| __a0.ordinal(); key(a).cmp(&key(b)) }).collect::<Vec<_>>();
    }

}

impl std::fmt::Display for Fossil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Fossil constant
pub struct FossilFields {
    pub name: String,
}

impl FossilFields {
    fn new(mut name: String) -> Self {
        let mut this = Self {
            name: Default::default(),
        };
        this.name = name.clone();
        this
    }

}
