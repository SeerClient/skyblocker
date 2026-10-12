//! Ported from de.hysky.skyblocker.skyblock.dwarven.CommissionLabels by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::mining_config::CommissionWaypointMode;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::DwarvenCategory;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::DwarvenEmissaries;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::GlaciteCategory;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::MiningLocationLabel;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::comms_widget;
use crate::de::hysky::skyblocker::utils::area::Area;
use crate::de::hysky::skyblocker::utils::area::DwarvenMines;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypoint as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<String, MiningLocationLabel.DwarvenCategory> DWARVEN_LOCATIONS = Arrays.stream(MiningLocationLabel.DwarvenCategory.values()).collect(Collectors.toMap(MiningLocationLabel.Dwarv
pub static DWARVEN_LOCATIONS: LazyLock<std::collections::HashMap<String, DwarvenCategory>> = LazyLock::new(|| DwarvenCategory::values().into_iter().map(|category| (category.to_string(), category)).collect());
/// java: private static final List<MiningLocationLabel.DwarvenEmissaries> DWARVEN_EMISSARIES = Arrays.stream(MiningLocationLabel.DwarvenEmissaries.values()).toList();
pub static DWARVEN_EMISSARIES: LazyLock<Vec<DwarvenEmissaries>> = LazyLock::new(|| DwarvenEmissaries::values().iter().copied().collect::<Vec<_>>());
/// java: private static final Map<String, MiningLocationLabel.GlaciteCategory> GLACITE_LOCATIONS = Arrays.stream(MiningLocationLabel.GlaciteCategory.values()).collect(Collectors.toMap(MiningLocationLabel.Glaci
pub static GLACITE_LOCATIONS: LazyLock<std::collections::HashMap<String, GlaciteCategory>> = LazyLock::new(|| GlaciteCategory::values().into_iter().map(|category| (category.to_string(), category)).collect());
/// java: protected static List<MiningLocationLabel> activeWaypoints = new ArrayList<>();
pub static ACTIVE_WAYPOINTS: compat::MainThread<Vec<compat::Shared<MiningLocationLabel>>> = compat::MainThread::new(|| Vec::new());
/// java: private static List<String> commissions = List.of();
pub static COMMISSIONS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));
/// java: private static boolean commissionDone = false;
pub static COMMISSION_DONE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class CommissionLabels
#[derive(Clone)]
pub struct CommissionLabels {}

impl CommissionLabels {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CommissionLabels::extract_rendering(&__a0); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { CommissionLabels::tick(); }), 20) });
    }

    /// java: public static boolean enabled()
    pub fn enabled() -> bool {
        return (SkyblockerConfigManager::get().mining.commission_waypoints.mode != CommissionWaypointMode::Off);
    }

    /// java: private static void tick()
    pub fn tick() {
        if !(Utils::is_in_dwarven_mines()) || !(CommissionLabels::enabled()) {
            return;
        }
        let mut found_commissions: bool = false;
        let mut new_commissions: Vec<String> = Vec::new();
        let mut new_commission_done: bool = false;
        for i in 0..(PlayerListManager::get_player_list().len() as i32) {
            let entry = &PlayerListManager::get_player_list()[(i) as usize];
            let Some(mut display_name) = compat::read(entry, |__e| __e.get_tab_list_display_name().cloned()) else {
                continue;
            };
            let mut string: String = display_name.get_string();
            if found_commissions {
                if !(string.starts_with(" ")) {
                    break;
                }
                string = compat::substring(&string, 1);
                let mut matcher: compat::regex::Matcher = comms_widget::COMM_PATTERN.matcher(&string);
                if matcher.matches() {
                    let mut name: String = matcher.group_named("name").unwrap();
                    let mut progress: String = matcher.group_named("progress").unwrap();
                    new_commission_done = ((new_commission_done | ("DONE" == progress))) as bool;
                    new_commissions.push(name.clone());
                }
            } else if string.starts_with("Commissions") {
                found_commissions = true;
            }
        }
        if !(new_commissions == compat::get_state(&COMMISSIONS)) || new_commission_done != compat::get_state(&COMMISSION_DONE) {
            compat::set_state(&COMMISSIONS, new_commissions.clone());
            compat::set_state(&COMMISSION_DONE, new_commission_done);
            CommissionLabels::update(&compat::get_state(&COMMISSIONS), compat::get_state(&COMMISSION_DONE));
        }
    }

    /// java: public static void update(List<String> newCommissions, boolean completed)
    pub fn update(new_commissions: &Vec<String>, mut completed: bool) {
        if !(CommissionLabels::enabled()) {
            return;
        }
        let mut current_mode: CommissionWaypointMode = SkyblockerConfigManager::get().mining.commission_waypoints.mode;
        ACTIVE_WAYPOINTS.get_mut().clear();
        let mut area: compat::Shared<dyn Area> = Utils::get_area();
        // `area.equals(Area.DwarvenMines.X)`: the area is that enum constant.
        let area_is = |constant: DwarvenMines| compat::read(&area, |__t| compat::DynAny::dyn_any(__t).downcast_ref::<DwarvenMines>().is_some_and(|__e| *__e == constant));
        if area_is(DwarvenMines::DwarvenBaseCamp) || area_is(DwarvenMines::GlaciteTunnels) || area_is(DwarvenMines::GlaciteMineshafts) || area_is(DwarvenMines::GreatGlaciteLake) {
            if current_mode != CommissionWaypointMode::Both && current_mode != CommissionWaypointMode::Glacite {
                return;
            }
            for commission in new_commissions.iter() {
                for glacite_location in GLACITE_LOCATIONS.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    if commission.contains(&glacite_location.0) {
                        let mut category: GlaciteCategory = glacite_location.1;
                        for gemstone_location in category.get_locations() {
                            ACTIVE_WAYPOINTS.get_mut().push(MiningLocationLabel::new((compat::shared(category) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), gemstone_location.clone()));
                        }
                    }
                }
            }
            if SkyblockerConfigManager::get().mining.commission_waypoints.show_base_camp {
                ACTIVE_WAYPOINTS.get_mut().push(MiningLocationLabel::new((compat::shared(GlaciteCategory::Campfire) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), GlaciteCategory::Campfire.get_locations()[(0) as usize].clone()));
            }
            return;
        }
        if current_mode != CommissionWaypointMode::Both && current_mode != CommissionWaypointMode::Dwarven {
            return;
        }
        for commission in new_commissions.iter() {
            for dwarven_location in DWARVEN_LOCATIONS.iter().map(|(k, v)| (k.clone(), v.clone())) {
                if commission.contains(&dwarven_location.0) {
                    let mut category: DwarvenCategory = dwarven_location.1;
                    category.fields().is_titanium.store(commission.contains("Titanium"), std::sync::atomic::Ordering::Relaxed);
                    ACTIVE_WAYPOINTS.get_mut().push(MiningLocationLabel::new((compat::shared(category) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), category.get_location()));
                }
            }
        }
        if SkyblockerConfigManager::get().mining.commission_waypoints.show_emissary && completed {
            if SkyblockerConfigManager::get().mining.commission_waypoints.hide_emissary_on_pigeon {
                // `for (ItemStack stack : player.getInventory().getNonEquipmentItems()) if (..) return;` - the list is
                // read where the player is lent.
                let has_royal_pigeon: bool = mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items().iter().any(|stack| stack.get_skyblock_id() == "ROYAL_PIGEON")).unwrap_or(false);
                if has_royal_pigeon {
                    return;
                }
            }
            for emissaries in DWARVEN_EMISSARIES.iter().copied() {
                ACTIVE_WAYPOINTS.get_mut().push(MiningLocationLabel::new((compat::shared(emissaries) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), emissaries.get_location()));
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if Location::DwarvenMines != Utils::get_location() || !(CommissionLabels::enabled()) {
            return;
        }
        for mininglocationlabel in ACTIVE_WAYPOINTS.get_mut().iter() {
            ({ let __f = compat::dispatch(mininglocationlabel, |__t| __t.extract_rendering()); __f(&compat::view(mininglocationlabel, |a| a as compat::Shared<dyn Renderable>), collector) });
        }
    }

}
