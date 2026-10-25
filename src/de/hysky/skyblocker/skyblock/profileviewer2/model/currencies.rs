//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Currencies by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Currencies
#[derive(Clone)]
pub struct Currencies {
    /// java: @SerializedName("coin_purse") public double coinsInPurse;
    pub coins_in_purse: f64,
    /// java: @SerializedName("motes_purse") public double riftMotes;
    pub rift_motes: f64,
    /// java: public Map<String, EssenceCurrency> essence = Map.of();
    pub essence: std::collections::HashMap<String, EssenceCurrency>,
}

impl Currencies {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            coins_in_purse: 0.0,
            rift_motes: 0.0,
            essence: std::collections::HashMap::from([]),
        };
        this
    }

}

/// java: public static class EssenceCurrency
#[derive(Clone)]
pub struct EssenceCurrency {
    /// java: public int current;
    pub current: i32,
}

impl EssenceCurrency {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current: 0,
        };
        this
    }

}

impl Currencies {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "coin_purse") {
            if let Some(__x) = gson::as_f64(__v) {
                self.coins_in_purse = __x;
            }
        }
        if let Some(__v) = gson::member(json, "motes_purse") {
            if let Some(__x) = gson::as_f64(__v) {
                self.rift_motes = __x;
            }
        }
        if let Some(__v) = gson::member(json, "essence") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(EssenceCurrency::from_json(__e0))) {
                self.essence = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Currencies.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl EssenceCurrency {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "current") {
            if let Some(__x) = gson::as_i32(__v) {
                self.current = __x;
            }
        }
    }

    /// `GSON.fromJson(json, EssenceCurrency.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
