//! Ported from de.hysky.skyblocker.utils.ws.Service by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<Service> CODEC = StringRepresentable.fromValues(Service::values);
/// java: public static final Codec<Service> CODEC = StringRepresentable.fromValues(Service::values);
pub static CODEC: compat::MainThread<CodecRef<Service>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum Service implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Service {
    CrystalWaypoints,
    DungeonSecrets,
    EggWaypoints,
}

impl Service {
    pub const VALUES: [Service; 3] = [Service::CrystalWaypoints, Service::DungeonSecrets, Service::EggWaypoints];

    /// java: values()
    pub fn values() -> Vec<Service> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Service::CrystalWaypoints => "CRYSTAL_WAYPOINTS",
            Service::DungeonSecrets => "DUNGEON_SECRETS",
            Service::EggWaypoints => "EGG_WAYPOINTS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Service {
        match name {
            "CRYSTAL_WAYPOINTS" => Service::CrystalWaypoints,
            "DUNGEON_SECRETS" => Service::DungeonSecrets,
            "EGG_WAYPOINTS" => Service::EggWaypoints,
            _ => panic!("IllegalArgumentException: No enum constant Service.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
