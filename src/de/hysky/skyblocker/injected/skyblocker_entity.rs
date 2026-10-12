//! Ported from de.hysky.skyblocker.injected.SkyblockerEntity by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface SkyblockerEntity
pub trait SkyblockerEntity {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: default @Nullable Component skyblocker$getCustomName()
    fn skyblocker_get_custom_name(&self) -> Option<MutableComponent> {
        panic!("UnsupportedOperationException: {}", "Implemented via Mixin");
    }

    /// java: default void skyblocker$setCustomName(Component customName)
    fn skyblocker_set_custom_name(&self, custom_name: &MutableComponent) {
        panic!("UnsupportedOperationException: {}", "Implemented via Mixin");
    }

}
