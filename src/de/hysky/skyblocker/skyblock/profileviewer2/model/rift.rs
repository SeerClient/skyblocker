//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Rift by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Rift
#[derive(Clone)]
pub struct Rift {
    /// java: public Access access = new Access();
    pub access: Access,
}

impl Rift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            access: Access::new(),
        };
        this
    }

}

/// java: public static class Access
#[derive(Clone)]
pub struct Access {
    /// java: @SerializedName("consumed_prism") public boolean consumedPrism;
    pub consumed_prism: bool,
}

impl Access {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            consumed_prism: false,
        };
        this
    }

}
