//! Ported from de.hysky.skyblocker.skyblock.crimson.CrimsonFaction by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<CrimsonFaction> CODEC = StringRepresentable.fromValues(CrimsonFaction::values);
/// java: public static final Codec<CrimsonFaction> CODEC = StringRepresentable.fromValues(CrimsonFaction::values);
pub static CODEC: compat::MainThread<CodecRef<CrimsonFaction>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum CrimsonFaction implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrimsonFaction {
    Mage,
    Barbarian,
}

impl CrimsonFaction {
    pub const VALUES: [CrimsonFaction; 2] = [CrimsonFaction::Mage, CrimsonFaction::Barbarian];

    /// java: values()
    pub fn values() -> Vec<CrimsonFaction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CrimsonFaction::Mage => "MAGE",
            CrimsonFaction::Barbarian => "BARBARIAN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CrimsonFaction {
        match name {
            "MAGE" => CrimsonFaction::Mage,
            "BARBARIAN" => CrimsonFaction::Barbarian,
            _ => panic!("IllegalArgumentException: No enum constant CrimsonFaction.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return this.name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for CrimsonFaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
