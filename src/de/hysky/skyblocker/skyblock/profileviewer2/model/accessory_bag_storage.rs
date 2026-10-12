//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.AccessoryBagStorage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class AccessoryBagStorage
#[derive(Clone)]
pub struct AccessoryBagStorage {
    /// java: @SerializedName("highest_magical_power") public int highestMagicalPower;
    pub highest_magical_power: i32,
    /// java: @SerializedName("selected_power") public String selectedPower = "";
    pub selected_power: String,
    /// java: @SerializedName("bag_upgrades_purchased") public int bagUpgradesPurchased;
    pub bag_upgrades_purchased: i32,
    /// java: @SerializedName("unlocked_powers") public List<String> unlockedPowers = List.of();
    pub unlocked_powers: Vec<String>,
    /// java: public Tuning tuning = new Tuning();
    pub tuning: Tuning,
}

impl AccessoryBagStorage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highest_magical_power: 0,
            selected_power: "".to_string(),
            bag_upgrades_purchased: 0,
            unlocked_powers: vec![],
            tuning: Tuning::new(),
        };
        this
    }

}

/// java: public static class Tuning
#[derive(Clone)]
pub struct Tuning {
    /// java: @SerializedName("highest_unlocked_slot") public int highestUnlockedSlot;
    pub highest_unlocked_slot: i32,
    /// java: @SerializedName("refund_1") public boolean refund1;
    pub refund1: bool,
    /// java: @SerializedName("refund_2") public boolean refund2;
    pub refund2: bool,
    /// java: @SerializedName("slot_0") public TuningSlot activeSlot = new TuningSlot();
    pub active_slot: compat::Shared<dyn TuningSlot>,
    /// java: @SerializedName("slot_1") public TuningTemplate slot1 = new TuningTemplate();
    pub slot1: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_2") public TuningTemplate slot2 = new TuningTemplate();
    pub slot2: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_3") public TuningTemplate slot3 = new TuningTemplate();
    pub slot3: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_4") public TuningTemplate slot4 = new TuningTemplate();
    pub slot4: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_5") public TuningTemplate slot5 = new TuningTemplate();
    pub slot5: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_6") public TuningTemplate slot6 = new TuningTemplate();
    pub slot6: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_7") public TuningTemplate slot7 = new TuningTemplate();
    pub slot7: compat::Shared<TuningTemplate>,
    /// java: @SerializedName("slot_8") public TuningTemplate slot8 = new TuningTemplate();
    pub slot8: compat::Shared<TuningTemplate>,
}

impl Tuning {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highest_unlocked_slot: 0,
            refund1: false,
            refund2: false,
            active_slot: (TuningSlotBase::new() as compat::Shared<dyn TuningSlot>),
            slot1: TuningTemplate::new(),
            slot2: TuningTemplate::new(),
            slot3: TuningTemplate::new(),
            slot4: TuningTemplate::new(),
            slot5: TuningTemplate::new(),
            slot6: TuningTemplate::new(),
            slot7: TuningTemplate::new(),
            slot8: TuningTemplate::new(),
        };
        this
    }

    /// java: public List<TuningSlot> unlockedSlots()
    pub fn unlocked_slots(&self) -> Vec<compat::Shared<dyn TuningSlot>> {
        let mut all_slots: Vec<compat::Shared<dyn TuningSlot>> = vec![self.active_slot.clone(), (self.slot1.clone() as compat::Shared<dyn TuningSlot>), (self.slot2.clone() as compat::Shared<dyn TuningSlot>), (self.slot3.clone() as compat::Shared<dyn TuningSlot>), (self.slot4.clone() as compat::Shared<dyn TuningSlot>), (self.slot5.clone() as compat::Shared<dyn TuningSlot>), (self.slot6.clone() as compat::Shared<dyn TuningSlot>), (self.slot7.clone() as compat::Shared<dyn TuningSlot>), (self.slot8.clone() as compat::Shared<dyn TuningSlot>)];
        // java: return allSlots.stream().filter(slot -> slot instanceof TuningTemplate template ? template.purchased() : true).toList();
        return all_slots.into_iter().filter(|slot| match compat::try_view::<TuningTemplate, _>(slot) {
            Some(template) => TuningTemplate::purchased(&template),
            None => true,
        }).collect::<Vec<_>>();
    }

}

/// java: public static class TuningSlot
#[derive(Clone)]
pub struct TuningSlotBase {
    /// java: public int health;
    pub health: i32,
    /// java: @SerializedName("defense") public int defence;
    pub defence: i32,
    /// java: @SerializedName("walk_speed") public int walkSpeed;
    pub walk_speed: i32,
    /// java: public int strength;
    pub strength: i32,
    /// java: @SerializedName("critical_damage") public int criticalDamage;
    pub critical_damage: i32,
    /// java: @SerializedName("critical_chance") public int criticalChance;
    pub critical_chance: i32,
    /// java: @SerializedName("attack_speed") public int attackSpeed;
    pub attack_speed: i32,
    /// java: public int intelligence;
    pub intelligence: i32,
}

impl TuningSlotBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn TuningSlot>, __R)) -> (compat::Shared<dyn TuningSlot>, __R) {
        let this = Self {
            health: 0,
            defence: 0,
            walk_speed: 0,
            strength: 0,
            critical_damage: 0,
            critical_chance: 0,
            attack_speed: 0,
            intelligence: 0,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn TuningSlot>), __h)
        }).1
    }

}

/// java: the virtual methods of TuningSlot (implemented by TuningSlotBase and every subclass)
pub trait TuningSlot: compat::DynAny {
    fn tuning_slot_base(&self) -> &TuningSlotBase;
    fn tuning_slot_base_mut(&mut self) -> &mut TuningSlotBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override @GenEquals public native boolean equals(Object o); (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn TuningSlot>, &compat::Shared<dyn TuningSlot>) -> bool {
        <dyn TuningSlot>::equals_body
    }

    /// java: @Override @GenHashCode public native int hashCode(); (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn TuningSlot>) -> i32 {
        <dyn TuningSlot>::hash_code_body
    }

}

impl dyn TuningSlot {
    /// The fields `@GenEquals` / `@GenHashCode` read: the eight `TuningSlot` declares, in declaration order.
    fn gen_fields(&self) -> [i32; 8] {
        let b = self.tuning_slot_base();
        [b.health, b.defence, b.walk_speed, b.strength, b.critical_damage, b.critical_chance, b.attack_speed, b.intelligence]
    }

    /// java: @GenEquals - the build's `ObjectMethods.bootstrap("equals", ..)` over TuningSlot's fields: the same object, or
    /// another `TuningSlot` (`isInstance`, so a `TuningTemplate` too) whose fields are all equal
    pub fn equals_body(__this: &compat::Shared<dyn TuningSlot>, o: &compat::Shared<dyn TuningSlot>) -> bool {
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(__this), std::sync::Arc::as_ptr(o)) {
            return true;
        }
        compat::read(__this, |a| a.gen_fields()) == compat::read(o, |b| b.gen_fields())
    }

    /// java: @GenHashCode - `ObjectMethods.bootstrap("hashCode", ..)`: `31 * result + Integer.hashCode(field)` over the fields
    pub fn hash_code_body(__this: &compat::Shared<dyn TuningSlot>) -> i32 {
        compat::read(__this, |t| t.gen_fields()).iter().fold(0i32, |acc, v| acc.wrapping_mul(31).wrapping_add(*v))
    }
}

impl TuningSlot for TuningSlotBase {
    fn tuning_slot_base(&self) -> &TuningSlotBase {
        self
    }
    fn tuning_slot_base_mut(&mut self) -> &mut TuningSlotBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class TuningTemplate extends TuningSlot
#[derive(Clone)]
pub struct TuningTemplate {
    /// java: extends TuningSlot
    pub base: TuningSlotBase,
    /// java: @SerializedName("purchase_ts") public @Nullable Long purchaseTimestamp;
    pub purchase_timestamp: Option<i64>,
}

impl std::ops::Deref for TuningTemplate {
    type Target = TuningSlotBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TuningTemplate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TuningTemplate {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TuningSlotBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                purchase_timestamp: None,
            });
            ((__h.clone() as compat::Shared<dyn TuningSlot>), __h)
        });
        __this
    }

    /// java: public boolean purchased()
    pub fn purchased(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.purchase_timestamp.clone()).is_some();
    }

}

impl TuningSlot for TuningTemplate {
    fn tuning_slot_base(&self) -> &TuningSlotBase {
        &self.base
    }
    fn tuning_slot_base_mut(&mut self) -> &mut TuningSlotBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
