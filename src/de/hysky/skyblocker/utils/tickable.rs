//! Ported from de.hysky.skyblocker.utils.Tickable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface Tickable
pub trait Tickable: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void tick(Minecraft client); (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft);

}
