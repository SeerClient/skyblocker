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

impl Events {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "easter") {
            if let Some(__x) = Some(EasterEvent::from_json(__v)) {
                self.easter = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Events.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
