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
