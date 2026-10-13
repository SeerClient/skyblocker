//! Ported from de.hysky.skyblocker.config.configs.CrimsonIsleConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;

/// java: public class CrimsonIsleConfig
#[derive(Clone)]
pub struct CrimsonIsleConfig {
    /// java: public Kuudra kuudra = new Kuudra();
    pub kuudra: compat::Shared<Kuudra>,
    /// java: public Dojo dojo = new Dojo();
    pub dojo: Dojo,
    /// java: public boolean extendNetherFog = true;
    pub extend_nether_fog: bool,
}

impl CrimsonIsleConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            kuudra: Kuudra::new(),
            dojo: Dojo::new(),
            extend_nether_fog: true,
        };
        this
    }

}

/// java: public static class Kuudra
#[derive(Clone)]
pub struct Kuudra {
    /// java: public boolean supplyWaypoints = true;
    pub supply_waypoints: bool,
    /// java: public boolean fuelWaypoints = true;
    pub fuel_waypoints: bool,
    /// java: public Waypoint.Type suppliesAndFuelWaypointType = Waypoint.Type.WAYPOINT;
    pub supplies_and_fuel_waypoint_type: Type,
    /// java: public boolean ballistaBuildWaypoints = true;
    pub ballista_build_waypoints: bool,
    /// java: public boolean safeSpotWaypoints = true;
    pub safe_spot_waypoints: bool,
    /// java: public boolean pearlWaypoints = true;
    pub pearl_waypoints: bool,
    /// java: public boolean noArrowPoisonWarning = true;
    pub no_arrow_poison_warning: bool,
    /// java: public int arrowPoisonThreshold = 32;
    pub arrow_poison_threshold: i32,
    /// java: public boolean kuudraGlow = true;
    pub kuudra_glow: bool,
    /// java: public boolean dangerWarning = true;
    pub danger_warning: bool,
}

impl Kuudra {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            supply_waypoints: true,
            fuel_waypoints: true,
            supplies_and_fuel_waypoint_type: Type::Waypoint,
            ballista_build_waypoints: true,
            safe_spot_waypoints: true,
            pearl_waypoints: true,
            no_arrow_poison_warning: true,
            arrow_poison_threshold: 32,
            kuudra_glow: true,
            danger_warning: true,
        });
        __this
    }

}

/// java: public static class Dojo
#[derive(Clone)]
pub struct Dojo {
    /// java: public boolean enableForceHelper = true;
    pub enable_force_helper: bool,
    /// java: public boolean enableStaminaHelper = true;
    pub enable_stamina_helper: bool,
    /// java: public boolean enableMasteryHelper = true;
    pub enable_mastery_helper: bool,
    /// java: public boolean enableDisciplineHelper = true;
    pub enable_discipline_helper: bool,
    /// java: public boolean enableSwiftnessHelper = true;
    pub enable_swiftness_helper: bool,
    /// java: public boolean enableControlHelper = true;
    pub enable_control_helper: bool,
    /// java: public boolean enableTenacityHelper = true;
    pub enable_tenacity_helper: bool,
}

impl Dojo {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_force_helper: true,
            enable_stamina_helper: true,
            enable_mastery_helper: true,
            enable_discipline_helper: true,
            enable_swiftness_helper: true,
            enable_control_helper: true,
            enable_tenacity_helper: true,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/crimson_isle_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// CrimsonIsleConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CrimsonIsleConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("kuudra".to_string(), compat::read((&self.kuudra), |__t| __t.__gson_write())),
            ("dojo".to_string(), ((&self.dojo)).__gson_write()),
            ("extendNetherFog".to_string(), dandelion::compat::GsonValue::Bool(*(&self.extend_nether_fog))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CrimsonIsleConfig> {
        let __o = __v.as_object()?;
        let mut __this = CrimsonIsleConfig::new();
        if let Some(__v) = __o.get("kuudra") {
            if let Some(__x) = Kuudra::__gson_read(__v) { __this.kuudra = __x; }
        }
        if let Some(__v) = __o.get("dojo") {
            if let Some(__x) = Dojo::__gson_read(__v) { __this.dojo = __x; }
        }
        if let Some(__v) = __o.get("extendNetherFog") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.extend_nether_fog = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "extendNetherFog" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.extend_nether_fog = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "kuudra" => compat::write(&self.kuudra, |__t| __t.__gson_patch(__rest, __value)),
                "dojo" => self.dojo.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `CrimsonIsleConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CrimsonIsleConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.CrimsonIsleConfig",
            new: CrimsonIsleConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CrimsonIsleConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Kuudra's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Kuudra {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("supplyWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.supply_waypoints))),
            ("fuelWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fuel_waypoints))),
            ("suppliesAndFuelWaypointType".to_string(), dandelion::compat::GsonValue::str(((&self.supplies_and_fuel_waypoint_type)).name())),
            ("ballistaBuildWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.ballista_build_waypoints))),
            ("safeSpotWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.safe_spot_waypoints))),
            ("pearlWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.pearl_waypoints))),
            ("noArrowPoisonWarning".to_string(), dandelion::compat::GsonValue::Bool(*(&self.no_arrow_poison_warning))),
            ("arrowPoisonThreshold".to_string(), dandelion::compat::GsonValue::int(*(&self.arrow_poison_threshold) as i64)),
            ("kuudraGlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.kuudra_glow))),
            ("dangerWarning".to_string(), dandelion::compat::GsonValue::Bool(*(&self.danger_warning))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<Kuudra>> {
        let __o = __v.as_object()?;
        let __h = Kuudra::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("supplyWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.supply_waypoints = __x; }
            }
            if let Some(__v) = __o.get("fuelWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fuel_waypoints = __x; }
            }
            if let Some(__v) = __o.get("suppliesAndFuelWaypointType") {
                if let Some(__x) = (__v).as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.supplies_and_fuel_waypoint_type = __x; }
            }
            if let Some(__v) = __o.get("ballistaBuildWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.ballista_build_waypoints = __x; }
            }
            if let Some(__v) = __o.get("safeSpotWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.safe_spot_waypoints = __x; }
            }
            if let Some(__v) = __o.get("pearlWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.pearl_waypoints = __x; }
            }
            if let Some(__v) = __o.get("noArrowPoisonWarning") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.no_arrow_poison_warning = __x; }
            }
            if let Some(__v) = __o.get("arrowPoisonThreshold") {
                if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.arrow_poison_threshold = __x; }
            }
            if let Some(__v) = __o.get("kuudraGlow") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.kuudra_glow = __x; }
            }
            if let Some(__v) = __o.get("dangerWarning") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.danger_warning = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "supplyWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.supply_waypoints = *__b; true } else { false },
                "fuelWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fuel_waypoints = *__b; true } else { false },
                "suppliesAndFuelWaypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.supplies_and_fuel_waypoint_type = __c; true } None => false } } else { false },
                "ballistaBuildWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.ballista_build_waypoints = *__b; true } else { false },
                "safeSpotWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.safe_spot_waypoints = *__b; true } else { false },
                "pearlWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.pearl_waypoints = *__b; true } else { false },
                "noArrowPoisonWarning" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.no_arrow_poison_warning = *__b; true } else { false },
                "kuudraGlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.kuudra_glow = *__b; true } else { false },
                "dangerWarning" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.danger_warning = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Kuudra.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<Kuudra>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.CrimsonIsleConfig$Kuudra",
            new: Kuudra::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: Kuudra::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Dojo's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Dojo {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableForceHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_force_helper))),
            ("enableStaminaHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_stamina_helper))),
            ("enableMasteryHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_mastery_helper))),
            ("enableDisciplineHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_discipline_helper))),
            ("enableSwiftnessHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_swiftness_helper))),
            ("enableControlHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_control_helper))),
            ("enableTenacityHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_tenacity_helper))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Dojo> {
        let __o = __v.as_object()?;
        let mut __this = Dojo::new();
        if let Some(__v) = __o.get("enableForceHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_force_helper = __x; }
        }
        if let Some(__v) = __o.get("enableStaminaHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_stamina_helper = __x; }
        }
        if let Some(__v) = __o.get("enableMasteryHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_mastery_helper = __x; }
        }
        if let Some(__v) = __o.get("enableDisciplineHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_discipline_helper = __x; }
        }
        if let Some(__v) = __o.get("enableSwiftnessHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_swiftness_helper = __x; }
        }
        if let Some(__v) = __o.get("enableControlHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_control_helper = __x; }
        }
        if let Some(__v) = __o.get("enableTenacityHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_tenacity_helper = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableForceHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_force_helper = *__b; true } else { false },
                "enableStaminaHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_stamina_helper = *__b; true } else { false },
                "enableMasteryHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_mastery_helper = *__b; true } else { false },
                "enableDisciplineHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_discipline_helper = *__b; true } else { false },
                "enableSwiftnessHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_swiftness_helper = *__b; true } else { false },
                "enableControlHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_control_helper = *__b; true } else { false },
                "enableTenacityHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_tenacity_helper = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Dojo.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Dojo> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.CrimsonIsleConfig$Dojo",
            new: Dojo::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Dojo::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
