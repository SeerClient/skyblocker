//! Ported from de.hysky.skyblocker.skyblock.dungeon.RareRoomAlert by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;

/// java: private static final Supplier<DungeonsConfig.RareRoomAlert> CONFIG = () -> SkyblockerConfigManager.get().dungeons.rareRoomAlert;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<crate::de::hysky::skyblocker::config::configs::dungeons_config::RareRoomAlert>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.rare_room_alert.clone()));
/// java: private static final String TRINITY_ROOM_NAME = "trinity-4";
pub const TRINITY_ROOM_NAME: &str = "trinity-4";
/// java: private static final String TOMIOKA_ROOM_NAME = "tomioka-0";
pub const TOMIOKA_ROOM_NAME: &str = "tomioka-0";
/// java: private static final String DUNCAN_ROOM_NAME = "duncan-1";
pub const DUNCAN_ROOM_NAME: &str = "duncan-1";

/// java: public class RareRoomAlert
#[derive(Clone)]
pub struct RareRoomAlert {}

impl RareRoomAlert {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        dungeon_events::ROOM_MATCHED.register(Arc::new(move |room| {
            if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.enabled)) {
                return;
            }
            let mut room_name: Option<String> = ({ let __f = compat::dispatch(&room, |__t| __t.get_name()); __f(&room) });
            match room_name.as_deref() {
                Some(TRINITY_ROOM_NAME) => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_for_trinity)) {
                        return;
                    }
                }
                Some(TOMIOKA_ROOM_NAME) => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_for_tomioka)) {
                        return;
                    }
                }
                Some(DUNCAN_ROOM_NAME) => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_for_duncan)) {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
            RareRoomAlert::show_alert(room_name.as_deref().unwrap());
        }));
    }

    /// java: private static void showAlert(String roomId)
    pub fn show_alert(room_id: &str) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut room_data = DungeonManager::get_room_metadata(room_id);
        let mut room_name: String = (if room_data.is_some() { room_data.as_ref().unwrap().name() } else { room_id.to_string() });
        TitleContainer::add_title_2(&Title::new_1(component::literal(&room_name.to_uppercase()).with_style_format(ChatFormatting::LightPurple)), 100);
        TitleContainer::play_notification_sound();
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.dungeons.rareRoomAlert.foundRoom", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(room_name.clone())])).to_string()); });
    }

}
