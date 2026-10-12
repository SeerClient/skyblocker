//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Events by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::easter_event::EasterEvent;

/// java: public class Events
#[derive(Clone)]
pub struct Events {
    /// java: public EasterEvent easter = new EasterEvent();
    pub easter: EasterEvent,
}

impl Events {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            easter: EasterEvent::new(),
        };
        this
    }

}
