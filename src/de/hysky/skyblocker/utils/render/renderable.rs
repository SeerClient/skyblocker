//! Ported from de.hysky.skyblocker.utils.render.Renderable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;

/// java: public interface Renderable
pub trait Renderable: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void extractRendering(PrimitiveCollector collector); (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>);

}
