//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Shards by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::core::block_pos::BlockPos;

/// java: public class Shards
#[derive(Clone)]
pub struct Shards {
    /// java: public Traps traps = new Traps();
    pub traps: Traps,
    /// java: public List<OwnedShard> owned = List.of();
    pub owned: Vec<OwnedShard>,
    /// java: public int fused;
    pub fused: i32,
}

impl Shards {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            traps: Traps::new(),
            owned: vec![],
            fused: 0,
        };
        this
    }

}

/// java: public static class Traps
#[derive(Clone)]
pub struct Traps {
    /// java: @SerializedName("active_traps") public List<ActiveTrap> activeTraps = List.of();
    pub active_traps: Vec<ActiveTrap>,
}

impl Traps {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            active_traps: vec![],
        };
        this
    }

}

/// java: public static class ActiveTrap
#[derive(Clone)]
pub struct ActiveTrap {
    /// java: @SerializedName("trap_item") public String trapItem = "";
    pub trap_item: String,
    /// java: @SerializedName("capture_time") public long captureTime;
    pub capture_time: i64,
    /// java: public String mode = "";
    pub mode: String,
    /// java: public String location = "";
    pub location: String,
    /// java: @SerializedName("placed_at") public long placedAt;
    pub placed_at: i64,
    /// java: public String shard = "";
    pub shard: String,
    /// java: public boolean captured;
    pub captured: bool,
    /// java: public boolean museum;
    pub museum: bool,
    /// java: public UUID uuid = UUID.randomUUID();
    pub uuid: Uuid,
}

impl ActiveTrap {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            trap_item: "".to_string(),
            capture_time: 0,
            mode: "".to_string(),
            location: "".to_string(),
            placed_at: 0,
            shard: "".to_string(),
            captured: false,
            museum: false,
            uuid: Uuid::random_uuid(),
        };
        this
    }

    /// java: public Location getIsland()
    pub fn get_island(&self) -> Location {
        return Location::from(&self.mode);
    }

    /// java: public BlockPos getPos()
    pub fn get_pos(&self) -> BlockPos {
        let mut split: Vec<String> = compat::regex::split(&self.location, ",");
        return BlockPos::containing((split[(0) as usize]).trim().parse::<f64>().expect("NumberFormatException"), (split[(1) as usize]).trim().parse::<f64>().expect("NumberFormatException"), (split[(2) as usize]).trim().parse::<f64>().expect("NumberFormatException"));
    }

}

/// java: public static class OwnedShard
#[derive(Clone)]
pub struct OwnedShard {
    /// java: public String type = "";
    pub r#type: String,
    /// java: @SerializedName("amount_owned") public int amountOwned;
    pub amount_owned: i32,
    /// java: public long captured;
    pub captured: i64,
}

impl OwnedShard {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            amount_owned: 0,
            captured: 0,
        };
        this
    }

}
