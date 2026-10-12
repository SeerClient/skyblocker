//! Ported from de.hysky.skyblocker.skyblock.teleport.ResponsiveSmoothAOTE by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use mcrust::net::minecraft::client::camera_type::CameraType;
use mcrust::net::minecraft::client::options;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static long startTime;
pub static START_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: @Nullable private static Vec3 lastPosition;
pub static LAST_POSITION: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: private static double lastProgress;
pub static LAST_PROGRESS: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));
/// java: @Nullable private static Vec3 cameraStartPos;
pub static CAMERA_START_POS: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);

/// java: public class ResponsiveSmoothAOTE
#[derive(Clone)]
pub struct ResponsiveSmoothAOTE {}

impl ResponsiveSmoothAOTE {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void playerGoingToTeleport()
    pub fn player_going_to_teleport() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.maximum_added_lag == 0 {
            return;
        }
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        let mut item_id: String = held_item.get_skyblock_id();
        let mut custom_data: Tag = ItemUtils::get_custom_data(&held_item);
        let mut distance: i32 = PredictiveSmoothAOTE::get_item_distance(&item_id, &custom_data);
        if distance == -1 {
            return;
        }
        if options::get_camera_type() != CameraType::FirstPerson && !(SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.third_person) {
            return;
        }
        let mut current_intermediate_position: Option<Vec3> = ResponsiveSmoothAOTE::get_interpolated_pos(RenderHelper::get_camera().position());
        *LAST_POSITION.get_mut() = (if current_intermediate_position.is_none() { Some(RenderHelper::get_camera().position()) } else { current_intermediate_position.clone() });
        compat::set_state(&LAST_PROGRESS, ((0) as f64) as f64);
        compat::set_state(&START_TIME, compat::millis());
    }

    /// java: @Nullable public static Vec3 getInterpolatedPos(Vec3 original)
    pub fn get_interpolated_pos(mut original: Vec3) -> Option<Vec3> {
        if LAST_POSITION.get_mut().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return None;
        }
        *CAMERA_START_POS.get_mut() = Some(original.clone());
        let mut progress: f64 = ((((compat::millis()).wrapping_sub(compat::get_state(&START_TIME))) as f64) / (SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.maximum_added_lag as f64));
        if progress >= (1 as f64) {
            *LAST_POSITION.get_mut() = None;
            compat::set_state(&LAST_PROGRESS, ((0) as f64) as f64);
            return None;
        }
        let mut teleport_vector: Vec3 = original.subtract_vec(LAST_POSITION.get_mut().as_ref().unwrap());
        let mut progress_diff: f64 = (progress - compat::get_state(&LAST_PROGRESS));
        let mut relative_progress: f64 = (progress_diff / ((1 as f64) - compat::get_state(&LAST_PROGRESS)));
        if LAST_POSITION.get_mut().is_none() {
            return None;
        }
        *LAST_POSITION.get_mut() = Some(LAST_POSITION.get_mut().as_ref().unwrap().add_vec(&teleport_vector.scale(relative_progress)));
        compat::set_state(&LAST_PROGRESS, progress);
        return LAST_POSITION.get_mut().clone();
    }

    /// java: @Nullable public static Vec3 getInterpolatedPlayerPos(float partialTicks)
    pub fn get_interpolated_player_pos(mut partial_ticks: f32) -> Option<Vec3> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || CAMERA_START_POS.get_mut().is_none() {
            return None;
        }
        let mut diff: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_position_partial(partial_ticks).subtract_vec(CAMERA_START_POS.get_mut().as_ref().unwrap())).unwrap();
        let Some(mut camara) = ResponsiveSmoothAOTE::get_interpolated_pos(CAMERA_START_POS.get_mut().clone().unwrap()) else {
            return None;
        };
        return Some(camara.add_vec(&diff));
    }

}
