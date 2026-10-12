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
