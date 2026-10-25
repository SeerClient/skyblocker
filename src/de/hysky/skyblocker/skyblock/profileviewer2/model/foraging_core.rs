//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ForagingCore by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class ForagingCore
#[derive(Clone)]
pub struct ForagingCore {
    /// java: public Whispers whispers = new Whispers();
    pub whispers: Whispers,
}

impl ForagingCore {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            whispers: Whispers::new(),
        };
        this
    }

}

/// java: public static class Whispers
#[derive(Clone)]
pub struct Whispers {
    /// java: public AbstractWhispersData forest = new AbstractWhispersData();
    pub forest: compat::Shared<AbstractWhispersData>,
    /// java: public AbstractWhispersData desert = new AbstractWhispersData();
    pub desert: compat::Shared<AbstractWhispersData>,
}

impl Whispers {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            forest: AbstractWhispersData::new(),
            desert: AbstractWhispersData::new(),
        };
        this
    }

}

/// java: public static class AbstractWhispersData
#[derive(Clone)]
pub struct AbstractWhispersData {
    /// java: public long total;
    pub total: i64,
    /// java: @SerializedName("1") public WhispersSlot slot1 = new WhispersSlot();
    pub slot1: compat::Shared<WhispersSlot>,
    /// java: @SerializedName("2") public WhispersSlot slot2 = new WhispersSlot();
    pub slot2: compat::Shared<WhispersSlot>,
    /// java: @SerializedName("3") public WhispersSlot slot3 = new WhispersSlot();
    pub slot3: compat::Shared<WhispersSlot>,
    /// java: @SerializedName("4") public WhispersSlot slot4 = new WhispersSlot();
    pub slot4: compat::Shared<WhispersSlot>,
    /// java: @SerializedName("5") public WhispersSlot slot5 = new WhispersSlot();
    pub slot5: compat::Shared<WhispersSlot>,
}

impl AbstractWhispersData {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            total: 0,
            slot1: WhispersSlot::new(),
            slot2: WhispersSlot::new(),
            slot3: WhispersSlot::new(),
            slot4: WhispersSlot::new(),
            slot5: WhispersSlot::new(),
        });
        __this
    }

    /// java: public WhispersSlot getSlot(int index)
    pub fn get_slot(__this: &compat::Shared<Self>, mut index: i32) -> compat::Shared<WhispersSlot> {
        return match index {
            1 => compat::read(__this, |__t| __t.slot1.clone()),
            2 => compat::read(__this, |__t| __t.slot2.clone()),
            3 => compat::read(__this, |__t| __t.slot3.clone()),
            4 => compat::read(__this, |__t| __t.slot4.clone()),
            5 => compat::read(__this, |__t| __t.slot5.clone()),
            _ => {
                panic!("IllegalArgumentException: {}", "Index must be between 1-5.");
            }
        };
    }

}

/// java: public static class WhispersSlot
#[derive(Clone)]
pub struct WhispersSlot {
    /// java: public long spent;
    pub spent: i64,
    /// java: @SerializedName("spent_non_refundable") public long spentNonRefundable;
    pub spent_non_refundable: i64,
}

impl WhispersSlot {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            spent: 0,
            spent_non_refundable: 0,
        });
        __this
    }

}

impl ForagingCore {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "whispers") {
            if let Some(__x) = Some(Whispers::from_json(__v)) {
                self.whispers = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ForagingCore.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Whispers {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "forest") {
            if let Some(__x) = Some(AbstractWhispersData::from_json(__v)) {
                self.forest = __x;
            }
        }
        if let Some(__v) = gson::member(json, "desert") {
            if let Some(__x) = Some(AbstractWhispersData::from_json(__v)) {
                self.desert = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Whispers.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl AbstractWhispersData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "total") {
            if let Some(__x) = gson::as_i64(__v) {
                self.total = __x;
            }
        }
        if let Some(__v) = gson::member(json, "1") {
            if let Some(__x) = Some(WhispersSlot::from_json(__v)) {
                self.slot1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "2") {
            if let Some(__x) = Some(WhispersSlot::from_json(__v)) {
                self.slot2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "3") {
            if let Some(__x) = Some(WhispersSlot::from_json(__v)) {
                self.slot3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "4") {
            if let Some(__x) = Some(WhispersSlot::from_json(__v)) {
                self.slot4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "5") {
            if let Some(__x) = Some(WhispersSlot::from_json(__v)) {
                self.slot5 = __x;
            }
        }
    }

    /// `GSON.fromJson(json, AbstractWhispersData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

impl WhispersSlot {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "spent") {
            if let Some(__x) = gson::as_i64(__v) {
                self.spent = __x;
            }
        }
        if let Some(__v) = gson::member(json, "spent_non_refundable") {
            if let Some(__x) = gson::as_i64(__v) {
                self.spent_non_refundable = __x;
            }
        }
    }

    /// `GSON.fromJson(json, WhispersSlot.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
