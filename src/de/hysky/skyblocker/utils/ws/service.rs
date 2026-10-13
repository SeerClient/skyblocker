//! Ported from de.hysky.skyblocker.utils.ws.Service by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

//   java: public static final Codec<Service> CODEC = StringRepresentable.fromValues(Service::values);
/// java: public static final Codec<Service> CODEC = StringRepresentable.fromValues(Service::values);
pub static CODEC: compat::MainThread<CodecRef<Service>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_values(Service::values(), |service: &Service| service.get_serialized_name()));

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
        //   java: return name();
        return self.name().to_string();
    }

}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
