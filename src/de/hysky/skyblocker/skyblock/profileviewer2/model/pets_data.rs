//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.PetsData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;

/// java: public class PetsData
#[derive(Clone)]
pub struct PetsData {
    /// java: @SerializedName("pet_care") public PetCare petCare = new PetCare();
    pub pet_care: PetCare,
    /// java: public List<Pet> pets = List.of();
    pub pets: Vec<compat::Shared<Pet>>,
}

impl PetsData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            pet_care: PetCare::new(),
            pets: vec![],
        };
        this
    }

}

/// java: public static class PetCare
#[derive(Clone)]
pub struct PetCare {
    /// java: @SerializedName("pet_types_sacrificed") public List<String> petTypesSacrificed = List.of();
    pub pet_types_sacrificed: Vec<String>,
}

impl PetCare {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            pet_types_sacrificed: vec![],
        };
        this
    }

}

/// java: public static class Pet
#[derive(Clone)]
pub struct Pet {
    /// java: public @Nullable UUID uuid;
    pub uuid: Option<Uuid>,
    /// java: public String type = "";
    pub r#type: String,
    /// java: public double exp;
    pub exp: f64,
    /// java: public String tier = "";
    pub tier: String,
    /// java: public @Nullable String heldItem;
    pub held_item: Option<String>,
    /// java: public int candyUsed;
    pub candy_used: i32,
    /// java: public @Nullable String skin;
    pub skin: Option<String>,
}

impl Pet {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            uuid: None,
            r#type: "".to_string(),
            exp: 0.0,
            tier: "".to_string(),
            held_item: None,
            candy_used: 0,
            skin: None,
        });
        __this
    }

    /// java: @GenToString @Override public native String toString();
    /// The body the build's `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' toString -
    /// `Pet[f=String.valueOf(f), ..]` over every non-static, non-transient field in declaration order.
    pub fn to_string(&self) -> String {
        format!("Pet[uuid={}, type={}, exp={}, tier={}, heldItem={}, candyUsed={}, skin={}]", compat::opt_str(&self.uuid), self.r#type, mcrust::javacompat::java_double_to_string(self.exp), self.tier, compat::opt_str(&self.held_item), self.candy_used, compat::opt_str(&self.skin))
    }

}

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Pet::to_string(self))
    }
}

impl PetsData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "pet_care") {
            if let Some(__x) = Some(PetCare::from_json(__v)) {
                self.pet_care = __x;
            }
        }
        if let Some(__v) = gson::member(json, "pets") {
            if let Some(__x) = gson::list(__v, |__e0| Some(Pet::from_json(__e0))) {
                self.pets = __x;
            }
        }
    }

    /// `GSON.fromJson(json, PetsData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl PetCare {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "pet_types_sacrificed") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.pet_types_sacrificed = __x;
            }
        }
    }

    /// `GSON.fromJson(json, PetCare.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Pet {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "uuid") {
            if let Some(__x) = (crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v)).map(Some) {
                self.uuid = __x;
            }
        }
        if let Some(__v) = gson::member(json, "exp") {
            if let Some(__x) = gson::as_f64(__v) {
                self.exp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "tier") {
            if let Some(__x) = gson::as_string(__v) {
                self.tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "heldItem") {
            if let Some(__x) = (gson::as_string(__v)).map(Some) {
                self.held_item = __x;
            }
        }
        if let Some(__v) = gson::member(json, "candyUsed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.candy_used = __x;
            }
        }
        if let Some(__v) = gson::member(json, "skin") {
            if let Some(__x) = (gson::as_string(__v)).map(Some) {
                self.skin = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Pet.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
