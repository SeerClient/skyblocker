//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Forge by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Forge
#[derive(Clone)]
pub struct Forge {
    /// java: @SerializedName("forge_processes") public ForgeProcesses forgeProcesses = new ForgeProcesses();
    pub forge_processes: ForgeProcesses,
}

impl Forge {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            forge_processes: ForgeProcesses::new(),
        };
        this
    }

}

/// java: public static class ForgeProcesses
#[derive(Clone)]
pub struct ForgeProcesses {
    /// java: public Map<String, Node> forge_1 = Map.of();
    pub forge_1: std::collections::HashMap<String, Node>,
}

impl ForgeProcesses {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            forge_1: std::collections::HashMap::from([]),
        };
        this
    }

}

/// java: public static class Node
#[derive(Clone)]
pub struct Node {
    /// java: public String type = "";
    pub r#type: String,
    /// java: public String id = "";
    pub id: String,
    /// java: public long startTime;
    pub start_time: i64,
    /// java: public int slot;
    pub slot: i32,
    /// java: public boolean notified = false;
    pub notified: bool,
}

impl Node {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            id: "".to_string(),
            start_time: 0,
            slot: 0,
            notified: false,
        };
        this
    }

}
