//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Loadouts by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::inventories::AbstractInventoryContents;

/// java: public class Loadouts
#[derive(Clone)]
pub struct Loadouts {
    /// java: @SerializedName("armor") public Armour armour = new Armour();
    pub armour: Armour,
    /// java: public Equipment equipment = new Equipment();
    pub equipment: Equipment,
}

impl Loadouts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            armour: Armour::new(),
            equipment: Equipment::new(),
        };
        this
    }

}

/// java: public static class Armour
#[derive(Clone)]
pub struct Armour {
    /// java: @SerializedName("1") public ArmourLoadout one = new ArmourLoadout();
    pub one: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("2") public ArmourLoadout two = new ArmourLoadout();
    pub two: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("3") public ArmourLoadout three = new ArmourLoadout();
    pub three: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("4") public ArmourLoadout four = new ArmourLoadout();
    pub four: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("5") public ArmourLoadout five = new ArmourLoadout();
    pub five: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("6") public ArmourLoadout six = new ArmourLoadout();
    pub six: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("7") public ArmourLoadout seven = new ArmourLoadout();
    pub seven: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("8") public ArmourLoadout eight = new ArmourLoadout();
    pub eight: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("9") public ArmourLoadout nine = new ArmourLoadout();
    pub nine: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("10") public ArmourLoadout ten = new ArmourLoadout();
    pub ten: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("11") public ArmourLoadout eleven = new ArmourLoadout();
    pub eleven: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("12") public ArmourLoadout twelve = new ArmourLoadout();
    pub twelve: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("13") public ArmourLoadout thirteen = new ArmourLoadout();
    pub thirteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("14") public ArmourLoadout fourteen = new ArmourLoadout();
    pub fourteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("15") public ArmourLoadout fifteen = new ArmourLoadout();
    pub fifteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("16") public ArmourLoadout sixteen = new ArmourLoadout();
    pub sixteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("17") public ArmourLoadout seventeen = new ArmourLoadout();
    pub seventeen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("18") public ArmourLoadout eighteen = new ArmourLoadout();
    pub eighteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("19") public ArmourLoadout nineteen = new ArmourLoadout();
    pub nineteen: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("20") public ArmourLoadout twenty = new ArmourLoadout();
    pub twenty: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("21") public ArmourLoadout twentyOne = new ArmourLoadout();
    pub twenty_one: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("22") public ArmourLoadout twentyTwo = new ArmourLoadout();
    pub twenty_two: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("23") public ArmourLoadout twentyThree = new ArmourLoadout();
    pub twenty_three: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("24") public ArmourLoadout twentyFour = new ArmourLoadout();
    pub twenty_four: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("25") public ArmourLoadout twentyFive = new ArmourLoadout();
    pub twenty_five: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("26") public ArmourLoadout twentySix = new ArmourLoadout();
    pub twenty_six: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("27") public ArmourLoadout twentySeven = new ArmourLoadout();
    pub twenty_seven: compat::Shared<ArmourLoadout>,
    /// java: @SerializedName("equipped_set") public @Nullable Integer equippedSet;
    pub equipped_set: Option<i32>,
}

impl Armour {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            one: ArmourLoadout::new(),
            two: ArmourLoadout::new(),
            three: ArmourLoadout::new(),
            four: ArmourLoadout::new(),
            five: ArmourLoadout::new(),
            six: ArmourLoadout::new(),
            seven: ArmourLoadout::new(),
            eight: ArmourLoadout::new(),
            nine: ArmourLoadout::new(),
            ten: ArmourLoadout::new(),
            eleven: ArmourLoadout::new(),
            twelve: ArmourLoadout::new(),
            thirteen: ArmourLoadout::new(),
            fourteen: ArmourLoadout::new(),
            fifteen: ArmourLoadout::new(),
            sixteen: ArmourLoadout::new(),
            seventeen: ArmourLoadout::new(),
            eighteen: ArmourLoadout::new(),
            nineteen: ArmourLoadout::new(),
            twenty: ArmourLoadout::new(),
            twenty_one: ArmourLoadout::new(),
            twenty_two: ArmourLoadout::new(),
            twenty_three: ArmourLoadout::new(),
            twenty_four: ArmourLoadout::new(),
            twenty_five: ArmourLoadout::new(),
            twenty_six: ArmourLoadout::new(),
            twenty_seven: ArmourLoadout::new(),
            equipped_set: None,
        };
        this
    }

    /// java: public List<ArmourLoadout> getLoadouts()
    pub fn get_loadouts(&self) -> Vec<compat::Shared<ArmourLoadout>> {
        return vec![self.one.clone(), self.two.clone(), self.three.clone(), self.four.clone(), self.five.clone(), self.six.clone(), self.seven.clone(), self.eight.clone(), self.nine.clone(), self.ten.clone(), self.eleven.clone(), self.twelve.clone(), self.thirteen.clone(), self.fourteen.clone(), self.fifteen.clone(), self.sixteen.clone(), self.seventeen.clone(), self.eighteen.clone(), self.nineteen.clone(), self.twenty.clone(), self.twenty_one.clone(), self.twenty_two.clone(), self.twenty_three.clone(), self.twenty_four.clone(), self.twenty_five.clone(), self.twenty_six.clone(), self.twenty_seven.clone()];
    }

}

/// java: public static class ArmourLoadout
#[derive(Clone)]
pub struct ArmourLoadout {
    /// java: public int id;
    pub id: i32,
    /// java: @SerializedName("HELMET") public Inventories.@Nullable AbstractInventoryContents helmet;
    pub helmet: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("CHESTPLATE") public Inventories.@Nullable AbstractInventoryContents chestplate;
    pub chestplate: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("LEGGINGS") public Inventories.@Nullable AbstractInventoryContents leggings;
    pub leggings: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("BOOTS") public Inventories.@Nullable AbstractInventoryContents boots;
    pub boots: Option<compat::Shared<AbstractInventoryContents>>,
}

impl ArmourLoadout {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            id: 0,
            helmet: None,
            chestplate: None,
            leggings: None,
            boots: None,
        });
        __this
    }

}

/// java: public static class Equipment
#[derive(Clone)]
pub struct Equipment {
    /// java: @SerializedName("1") public EquipmentLoadout one = new EquipmentLoadout();
    pub one: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("2") public EquipmentLoadout two = new EquipmentLoadout();
    pub two: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("3") public EquipmentLoadout three = new EquipmentLoadout();
    pub three: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("4") public EquipmentLoadout four = new EquipmentLoadout();
    pub four: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("5") public EquipmentLoadout five = new EquipmentLoadout();
    pub five: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("6") public EquipmentLoadout six = new EquipmentLoadout();
    pub six: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("7") public EquipmentLoadout seven = new EquipmentLoadout();
    pub seven: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("8") public EquipmentLoadout eight = new EquipmentLoadout();
    pub eight: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("9") public EquipmentLoadout nine = new EquipmentLoadout();
    pub nine: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("10") public EquipmentLoadout ten = new EquipmentLoadout();
    pub ten: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("11") public EquipmentLoadout eleven = new EquipmentLoadout();
    pub eleven: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("12") public EquipmentLoadout twelve = new EquipmentLoadout();
    pub twelve: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("13") public EquipmentLoadout thirteen = new EquipmentLoadout();
    pub thirteen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("14") public EquipmentLoadout fourteen = new EquipmentLoadout();
    pub fourteen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("15") public EquipmentLoadout fifteen = new EquipmentLoadout();
    pub fifteen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("16") public EquipmentLoadout sixteen = new EquipmentLoadout();
    pub sixteen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("17") public EquipmentLoadout seventeen = new EquipmentLoadout();
    pub seventeen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("18") public EquipmentLoadout eighteen = new EquipmentLoadout();
    pub eighteen: compat::Shared<EquipmentLoadout>,
    /// java: @SerializedName("equipped_set") public @Nullable Integer equippedSet;
    pub equipped_set: Option<i32>,
}

impl Equipment {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            one: EquipmentLoadout::new(),
            two: EquipmentLoadout::new(),
            three: EquipmentLoadout::new(),
            four: EquipmentLoadout::new(),
            five: EquipmentLoadout::new(),
            six: EquipmentLoadout::new(),
            seven: EquipmentLoadout::new(),
            eight: EquipmentLoadout::new(),
            nine: EquipmentLoadout::new(),
            ten: EquipmentLoadout::new(),
            eleven: EquipmentLoadout::new(),
            twelve: EquipmentLoadout::new(),
            thirteen: EquipmentLoadout::new(),
            fourteen: EquipmentLoadout::new(),
            fifteen: EquipmentLoadout::new(),
            sixteen: EquipmentLoadout::new(),
            seventeen: EquipmentLoadout::new(),
            eighteen: EquipmentLoadout::new(),
            equipped_set: None,
        };
        this
    }

    /// java: public List<EquipmentLoadout> getLoadouts()
    pub fn get_loadouts(&self) -> Vec<compat::Shared<EquipmentLoadout>> {
        return vec![self.one.clone(), self.two.clone(), self.three.clone(), self.four.clone(), self.five.clone(), self.six.clone(), self.seven.clone(), self.eight.clone(), self.nine.clone(), self.ten.clone(), self.eleven.clone(), self.twelve.clone(), self.thirteen.clone(), self.fourteen.clone(), self.fifteen.clone(), self.sixteen.clone(), self.seventeen.clone(), self.eighteen.clone()];
    }

}

/// java: public static class EquipmentLoadout
#[derive(Clone)]
pub struct EquipmentLoadout {
    /// java: public int id;
    pub id: i32,
    /// java: @SerializedName("EQUIPMENT_SLOT_1") public Inventories.@Nullable AbstractInventoryContents necklace;
    pub necklace: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("EQUIPMENT_SLOT_2") public Inventories.@Nullable AbstractInventoryContents cloak;
    pub cloak: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("EQUIPMENT_SLOT_3") public Inventories.@Nullable AbstractInventoryContents belt;
    pub belt: Option<compat::Shared<AbstractInventoryContents>>,
    /// java: @SerializedName("EQUIPMENT_SLOT_4") public Inventories.@Nullable AbstractInventoryContents braceletOrGloves;
    pub bracelet_or_gloves: Option<compat::Shared<AbstractInventoryContents>>,
}

impl EquipmentLoadout {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            id: 0,
            necklace: None,
            cloak: None,
            belt: None,
            bracelet_or_gloves: None,
        });
        __this
    }

}

impl Loadouts {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "armor") {
            if let Some(__x) = Some(Armour::from_json(__v)) {
                self.armour = __x;
            }
        }
        if let Some(__v) = gson::member(json, "equipment") {
            if let Some(__x) = Some(Equipment::from_json(__v)) {
                self.equipment = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Loadouts.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Armour {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "1") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.one = __x;
            }
        }
        if let Some(__v) = gson::member(json, "2") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.two = __x;
            }
        }
        if let Some(__v) = gson::member(json, "3") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.three = __x;
            }
        }
        if let Some(__v) = gson::member(json, "4") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.four = __x;
            }
        }
        if let Some(__v) = gson::member(json, "5") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.five = __x;
            }
        }
        if let Some(__v) = gson::member(json, "6") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.six = __x;
            }
        }
        if let Some(__v) = gson::member(json, "7") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.seven = __x;
            }
        }
        if let Some(__v) = gson::member(json, "8") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.eight = __x;
            }
        }
        if let Some(__v) = gson::member(json, "9") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.nine = __x;
            }
        }
        if let Some(__v) = gson::member(json, "10") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.ten = __x;
            }
        }
        if let Some(__v) = gson::member(json, "11") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.eleven = __x;
            }
        }
        if let Some(__v) = gson::member(json, "12") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twelve = __x;
            }
        }
        if let Some(__v) = gson::member(json, "13") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.thirteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "14") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.fourteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "15") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.fifteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "16") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.sixteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "17") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.seventeen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "18") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.eighteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "19") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.nineteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "20") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty = __x;
            }
        }
        if let Some(__v) = gson::member(json, "21") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_one = __x;
            }
        }
        if let Some(__v) = gson::member(json, "22") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_two = __x;
            }
        }
        if let Some(__v) = gson::member(json, "23") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_three = __x;
            }
        }
        if let Some(__v) = gson::member(json, "24") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_four = __x;
            }
        }
        if let Some(__v) = gson::member(json, "25") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_five = __x;
            }
        }
        if let Some(__v) = gson::member(json, "26") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_six = __x;
            }
        }
        if let Some(__v) = gson::member(json, "27") {
            if let Some(__x) = Some(ArmourLoadout::from_json(__v)) {
                self.twenty_seven = __x;
            }
        }
        if let Some(__v) = gson::member(json, "equipped_set") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.equipped_set = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Armour.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ArmourLoadout {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "id") {
            if let Some(__x) = gson::as_i32(__v) {
                self.id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "HELMET") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.helmet = __x;
            }
        }
        if let Some(__v) = gson::member(json, "CHESTPLATE") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.chestplate = __x;
            }
        }
        if let Some(__v) = gson::member(json, "LEGGINGS") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.leggings = __x;
            }
        }
        if let Some(__v) = gson::member(json, "BOOTS") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.boots = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ArmourLoadout.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

impl Equipment {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "1") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.one = __x;
            }
        }
        if let Some(__v) = gson::member(json, "2") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.two = __x;
            }
        }
        if let Some(__v) = gson::member(json, "3") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.three = __x;
            }
        }
        if let Some(__v) = gson::member(json, "4") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.four = __x;
            }
        }
        if let Some(__v) = gson::member(json, "5") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.five = __x;
            }
        }
        if let Some(__v) = gson::member(json, "6") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.six = __x;
            }
        }
        if let Some(__v) = gson::member(json, "7") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.seven = __x;
            }
        }
        if let Some(__v) = gson::member(json, "8") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.eight = __x;
            }
        }
        if let Some(__v) = gson::member(json, "9") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.nine = __x;
            }
        }
        if let Some(__v) = gson::member(json, "10") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.ten = __x;
            }
        }
        if let Some(__v) = gson::member(json, "11") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.eleven = __x;
            }
        }
        if let Some(__v) = gson::member(json, "12") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.twelve = __x;
            }
        }
        if let Some(__v) = gson::member(json, "13") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.thirteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "14") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.fourteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "15") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.fifteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "16") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.sixteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "17") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.seventeen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "18") {
            if let Some(__x) = Some(EquipmentLoadout::from_json(__v)) {
                self.eighteen = __x;
            }
        }
        if let Some(__v) = gson::member(json, "equipped_set") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.equipped_set = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Equipment.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl EquipmentLoadout {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "id") {
            if let Some(__x) = gson::as_i32(__v) {
                self.id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EQUIPMENT_SLOT_1") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.necklace = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EQUIPMENT_SLOT_2") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.cloak = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EQUIPMENT_SLOT_3") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.belt = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EQUIPMENT_SLOT_4") {
            if let Some(__x) = (Some(AbstractInventoryContents::from_json(__v))).map(Some) {
                self.bracelet_or_gloves = __x;
            }
        }
    }

    /// `GSON.fromJson(json, EquipmentLoadout.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
