//! Ported from de.hysky.skyblocker.skyblock.rift.TheRift by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::rift::effigy_waypoints::EffigyWaypoints;
use crate::de::hysky::skyblocker::skyblock::rift::enigma_souls::EnigmaSouls;
use crate::de::hysky::skyblocker::skyblock::rift::mirrorverse_waypoints::MirrorverseWaypoints;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;

/// java: public class TheRift
#[derive(Clone)]
pub struct TheRift {}

impl TheRift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { MirrorverseWaypoints::extract_rendering(&__a0); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EffigyWaypoints::extract_rendering(&__a0); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EnigmaSouls::extract_rendering(&__a0); }));
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(MirrorverseWaypoints::load);
        mcrust::mcrust::modding::events::client_lifecycle_events::register_client_started(|client| MirrorverseWaypoints::load(client));
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(EnigmaSouls::load);
        mcrust::mcrust::modding::events::client_lifecycle_events::register_client_started(|client| EnigmaSouls::load(client));
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(EnigmaSouls::save);
        mcrust::mcrust::modding::events::client_lifecycle_events::register_client_stopping(|client| EnigmaSouls::save(client));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(EnigmaSouls::onMessage);
        compat::register_allow_game(|text, overlay| EnigmaSouls::on_message(text, overlay));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: ClientCommandRegistrationCallback.EVENT.register(EnigmaSouls::registerCommands);
                EnigmaSouls::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { EffigyWaypoints::update_effigies(); }), SkyblockerConfigManager::get().slayers.vampire_slayer.effigy_update_frequency) });
    }

}
