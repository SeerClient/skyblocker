//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Attributes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Attributes
#[derive(Clone)]
pub struct Attributes {
    /// java: public Map<String, Integer> stacks = Map.of();
    pub stacks: std::collections::HashMap<String, i32>,
}

impl Attributes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            stacks: std::collections::HashMap::from([]),
        };
        this
    }

}
