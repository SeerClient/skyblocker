//! Skyblocker - ported from Fabric by fabric2seer (SeerClient native mod).
//! See PORTING_REPORT.md for what was translated and what is left as TODO.
#![allow(unused_imports, unused_mut, unused_variables, unused_parens, dead_code, non_snake_case,
         clippy::all)]

// The game crate. Linked as the prebuilt SDK `mcrust.dll` (`-L crate=<sdk>/bin`);
// an explicit `extern crate` because a command-line `--extern mcrust` breaks
// resolving it through another mod (dandelion -> yacl).
extern crate mcrust;

pub mod compat;
pub mod overflow_trace;
pub mod prelude;
pub mod mixins;
pub mod entry;
pub mod de;
pub mod unresolved;
pub mod net;
/// io.github.moulberry.repo - the NEU repository parser library (`moe.nea:neurepoparser`).
pub mod io;
/// meteordevelopment.discordipc - the Discord Rich Presence IPC library (`meteordevelopment:discord-ipc:1.1`).
pub mod meteordevelopment;

use mcrust::mcrust::modding::entrypoint::{ClientModInitializer, ModInitializer, PreLaunchEntrypoint};

/// Registers every ported Mixin (preLaunch: before the game bootstraps, as Mixin applies).
#[derive(Default)]
pub struct Mixins;

impl PreLaunchEntrypoint for Mixins {
    fn on_pre_launch(&self) {
        compat::register_hooks(); // the hooks the compat layer reads the game through
        // the hm-api library's mixins (its Hypixel Mod API channels) - Fabric applies a dependency's mixins with the mod's
        net::azureaaron::hmapi::hm_api::HMApi::register_mixins();
        // the Render Chest library's mixins (render-chest.mixins.json) - a bundled mod, applied with the mod's
        net::azureaaron::renderchest::r#impl::render_chest::RenderChest::register_mixins();
        mixins::abstract_container_menu_mixin::register();
        mixins::abstract_container_screen_mixin::register();
        mixins::abstract_sign_edit_screen_mixin::register();
        mixins::bat_mixin::register();
        mixins::boss_health_overlay_mixin::register();
        mixins::camera_mixin::register();
        mixins::chest_block_entity_mixin::register();
        mixins::clientbound_commands_packet_mixin::register();
        mixins::client_common_packet_listener_impl_mixin::register();
        mixins::client_level_mixin::register();
        mixins::client_packet_listener_mixin::register();
        mixins::client_tooltip_component_mixin::register();
        mixins::container_screen_mixin::register();
        mixins::data_component_holder_mixin::register();
        mixins::dyed_item_color_mixin::register();
        mixins::entity_mixin::register();
        mixins::entity_render_dispatcher_mixin::register();
        mixins::entity_renderer_mixin::register();
        mixins::equipment_layer_renderer_mixin::register();
        mixins::fishing_hook_mixin::register();
        mixins::fog_renderer_mixin::register();
        mixins::game_renderer_mixin::register();
        mixins::gl_command_encoder_mixin::register();
        mixins::gui_graphics_extractor_mixin::register();
        mixins::hud_mixin::register();
        mixins::inventory_mixin::register();
        mixins::inventory_screen_mixin::register();
        mixins::item_in_hand_renderer_mixin::register();
        mixins::item_stack_mixin::register();
        mixins::key_mapping_mixin::register();
        mixins::level_extractor_mixin::register();
        mixins::level_loading_screen_mixin::register();
        mixins::local_player_mixin::register();
        mixins::map_renderer_mixin::register();
        mixins::menu_screens_constructor_mixin::register();
        mixins::minecraft_mixin::register();
        mixins::mouse_handler_mixin::register();
        mixins::multi_player_game_mode_mixin::register();
        mixins::mutable_component_mixin::register();
        mixins::options_mixin::register();
        mixins::ping_debug_monitor_mixin::register();
        mixins::player_tab_overlay_mixin::register();
        mixins::quick_nav_mixin::register();
        mixins::quick_nav_screen_mixin::register();
        mixins::render_fish_mixin::register();
        mixins::scoreboard_mixin::register();
        mixins::screen_effect_renderer_mixin::register();
        mixins::screen_mixin::register();
        mixins::skin_manager_mixin::register();
        mixins::skin_texture_downloader_mixin::register();
        mixins::social_interactions_player_list_mixin::register();
        mixins::synched_entity_data_mixin::register();
        mixins::window_mixin::register();
        mixins::yggdrasil_minecraft_session_service_mixin::register();
        mixins::yggdrasil_services_key_info_mixin::register();
        mixins::abstract_container_screen_accessor::register();
        mixins::block_entity_render_state_accessor::register();
        mixins::chat_listener_accessor::register();
        mixins::checkbox_accessor::register();
        mixins::custom_data_accessor::register();
        mixins::data_component_patch_accessor::register();
        mixins::direct_state_access_invoker::register();
        mixins::edit_box_accessor::register();
        mixins::ender_man_accessor::register();
        mixins::entity_render_dispatcher_accessor::register();
        mixins::font_accessor::register();
        mixins::frustum_invoker::register();
        mixins::gui_graphics_extractor_invoker::register();
        mixins::hud_accessor::register();
        mixins::inventory_screen_invoker::register();
        mixins::map_item_saved_data_accessor::register();
        mixins::map_renderer_invoker::register();
        mixins::minecraft_accessor::register();
        mixins::model_manager_accessor::register();
        mixins::player_tab_overlay_accessor::register();
        mixins::popup_screen_accessor::register();
        mixins::recipe_book_component_accessor::register();
        mixins::screen_accessor::register();
        mixins::slot_accessor::register();
        mixins::sprite_contents_accessor::register();
        mixins::scroll_tracker_mixin::register();
        mixins::connection_mixin::register();
        mixins::system_reader_mixin::register();
        mixins::url_config_mixin::register();
    }
}

/// the "client" entrypoint
#[derive(Default)]
pub struct Client;

impl ClientModInitializer for Client {
    fn on_initialize_client(&self) {
        compat::install(); // Minecraft.getInstance() for the translated classes
        // hm-api's ClientModInitializer: a dependency of Skyblocker, so Fabric initializes it first
        net::azureaaron::hmapi::hm_api::HMApi::on_initialize_client();
        entry::skyblocker_mod::on_initialize_client();
        // java: the "modmenu" entrypoint, de.hysky.skyblocker.compatibility.modmenu.ModMenuEntry -
        // `getModConfigScreenFactory()` is `SkyblockerConfigManager::createGUI`. SeerClient's mod list reads it from
        // `mod_menu::register_config_screen*`; the config screen is built by Dandelion and opened by its own `.set()`,
        // with the mod list (the current screen) as its parent.
        mcrust::mcrust::modding::mod_menu::register_config_screen_opener("skyblocker", || {
            crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::create_gui(Some(dandelion::compat::ScreenRef::Current)).set();
        });
    }
}

mcrust::declare_mod! {
    client: [Client],
    pre_launch: [Mixins],
}
