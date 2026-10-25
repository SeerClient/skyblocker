//! Ported from de.hysky.skyblocker.skyblock.fishing.FishingHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use crate::de::hysky::skyblocker::skyblock::fishing::fishing_hook_display_helper::FishingHookDisplayHelper;
use crate::de::hysky::skyblocker::config::configs::helper_config::FishingHookDisplay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Title title = new Title("skyblocker.fishing.reelNow", ChatFormatting.GREEN);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.fishing.reelNow".to_string(), ChatFormatting::Green));
/// java: protected static long startTime;
pub static START_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final int BOBBER_TIMEOUT_TICKS = 10;
pub const BOBBER_TIMEOUT_TICKS: i32 = 10;
/// java: private static int waitingForBobberTicks = 0;
pub static WAITING_FOR_BOBBER_TICKS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class FishingHelper
#[derive(Clone)]
pub struct FishingHelper {}

impl FishingHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: UseItemCallback.EVENT.register((player, _, hand) -> { .. });
        mcrust::mcrust::modding::events::use_item_callback::register(|player, _, hand| {
            use mcrust::net::minecraft::world::entity::player::player::Player as _;
            if !Utils::is_on_skyblock() {
                return mcrust::net::minecraft::world::interaction_result::InteractionResult::Pass;
            }
            let stack = player.player().base.get_item_in_hand(hand);
            if stack.is_item(&mcrust::net::minecraft::world::item::items::items().get("fishing_rod")) {
                if player.player().fishing.is_none() {
                    compat::set_state(&WAITING_FOR_BOBBER_TICKS, BOBBER_TIMEOUT_TICKS);
                } else {
                    compat::set_state(&WAITING_FOR_BOBBER_TICKS, 0);
                    FishingHelper::reset();
                }
            }
            mcrust::net::minecraft::world::interaction_result::InteractionResult::Pass
        });
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                if compat::get_state(&WAITING_FOR_BOBBER_TICKS) > 0 {
                    if FishingHookDisplayHelper::player_fishing().is_some() {
                        compat::set_state(&WAITING_FOR_BOBBER_TICKS, 0);
                        FishingHelper::start();
                    } else {
                        compat::set_state(&WAITING_FOR_BOBBER_TICKS, compat::get_state(&WAITING_FOR_BOBBER_TICKS) - 1);
                    }
                } else if FishingHookDisplayHelper::player_fishing().is_none() {
                    FishingHelper::reset();
                }
            }
        }));
    }

    /// java: public static void start()
    pub fn start() {
        compat::set_state(&START_TIME, compat::millis());
    }

    /// java: public static void reset()
    pub fn reset() {
        compat::set_state(&START_TIME, ((0) as i64) as i64);
    }

    /// java: public static void checkIfFishWasCaught(ArmorStand armorStand)
    pub fn check_if_fish_was_caught(armor_stand: &ArmorStand) {
        if Utils::is_on_skyblock() && (SkyblockerConfigManager::get().helpers.fishing.enable_fishing_helper || SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Hud) {
            if !armor_stand.is_invisible_field() || !armor_stand.base.base.has_custom_name() || !armor_stand.base.base.is_custom_name_visible() {
                return;
            }
            if let Some(fishing) = FishingHookDisplayHelper::player_fishing() {
                let mut name: String = armor_stand.base.base.get_custom_name().unwrap().get_string();
                if name == "!!!" && mcrust::mcrust::modding::game::with_entity(fishing, |hook| hook.base().get_bounding_box().inflate(4.0).contains_vec(&armor_stand.base.base.position())).unwrap_or(false) {
                    if SkyblockerConfigManager::get().helpers.fishing.enable_fishing_helper {
                        TitleContainer::add_title_and_play_sound_2(TITLE.get_mut(), 10);
                    }
                }
            }
        }
    }

}
