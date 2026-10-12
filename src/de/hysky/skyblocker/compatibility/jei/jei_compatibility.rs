//! Ported from de.hysky.skyblocker.compatibility.jei.JEICompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public static final boolean JEI_LOADED = FabricLoader.getInstance().isModLoaded("jei");
pub static JEI_LOADED: LazyLock<bool> = LazyLock::new(|| mcrust::mcrust::modding::loader::is_mod_loaded("jei"));

/// java: public class JEICompatibility
#[derive(Clone)]
pub struct JEICompatibility {}

impl JEICompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
