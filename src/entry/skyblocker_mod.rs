//! Ported from de.hysky.skyblocker.SkyblockerMod by fabric2seer. Entrypoint: client.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::config_null_fields_fix::ConfigNullFieldsFix;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::BackpackPreview;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::discord::discord_rpc_manager::DiscordRPCManager;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final String NAMESPACE = "skyblocker";
pub const NAMESPACE: &str = "skyblocker";
/// java: public static final ModContainer SKYBLOCKER_MOD = FabricLoader.getInstance().getModContainer(NAMESPACE).orElseThrow();
pub static SKYBLOCKER_MOD: compat::MainThread<std::sync::Arc<mcrust::mcrust::modding::loader::ModContainer>> = compat::MainThread::new(|| mcrust::mcrust::modding::loader::get_mod(NAMESPACE).unwrap());
/// java: public static final String VERSION = SKYBLOCKER_MOD.getMetadata().getVersion().getFriendlyString();
pub static VERSION: LazyLock<String> = LazyLock::new(|| SKYBLOCKER_MOD.get_mut().metadata.version.to_string());
/// java: public static final Path CONFIG_DIR = FabricLoader.getInstance().getConfigDir().resolve(NAMESPACE);
pub static CONFIG_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| mcrust::mcrust::modding::config::config_dir().join(NAMESPACE));
// TODO(fabric2seer): `new GsonBuilder(..)`
//   java: public static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
/// java: public static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
pub static GSON: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GSON` - see the TODO above"));
// TODO(fabric2seer): `new GsonBuilder(..)`
//   java: public static final Gson GSON_COMPACT = new GsonBuilder().create();
/// java: public static final Gson GSON_COMPACT = new GsonBuilder().create();
pub static GSON_COMPACT: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GSON_COMPACT` - see the TODO above"));
// TODO(fabric2seer): static `KeyMapping.Category` has no Rust counterpart in the port's KeyMapping file
//   java: public static final KeyMapping.Category KEYBINDING_CATEGORY = KeyMapping.Category.register(id("main"));
/// java: public static final KeyMapping.Category KEYBINDING_CATEGORY = KeyMapping.Category.register(id("main"));
pub static KEYBINDING_CATEGORY: compat::MainThread<mcrust::net::minecraft::client::key_mapping::Category> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `KEYBINDING_CATEGORY` - see the TODO above"));
// TODO(fabric2seer): Executors.newVirtualThreadPerTaskExecutor: use std::thread / the port's executors
//   java: public static final ExecutorService VIRTUAL_THREAD_EXECUTOR = Executors.newVirtualThreadPerTaskExecutor();
/// java: public static final ExecutorService VIRTUAL_THREAD_EXECUTOR = Executors.newVirtualThreadPerTaskExecutor();
pub static VIRTUAL_THREAD_EXECUTOR: compat::MainThread<crate::unresolved::ExecutorService> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `VIRTUAL_THREAD_EXECUTOR` - see the TODO above"));

pub fn id(path: &str) -> Identifier {
    return Identifier::from_namespace_and_path(NAMESPACE, path);
}

pub fn on_initialize_client() {
    Utils::is_on_skyblock();
    client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(__r) });
        ({ let __r = message_scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Scheduler>)) });
    }));
    SkyblockerConfigManager::init();
    ConfigNullFieldsFix::init();
    ConfigBackupManager::init();
    crate::entry::skyblocker_mod::init();
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { Utils::update(); }), 20) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DiscordRPCManager::update_data_and_presence(); }), 200) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { BackpackPreview::tick(); }), 50) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { PlayerListManager::try_update_list(); }), 2) });
}

pub fn tick(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(__r) });
    ({ let __r = message_scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Scheduler>)) });
}

/// java: private static void init() - "At compile time, ASM completely overwrites the content of this method".
///
/// The body is what Skyblocker's build-time `InitProcessor` (buildSrc `init/{InitReader,InitProcessor,InitInjector}`)
/// writes into it: one `invokestatic` per `@Init`-annotated method of the mod, sorted by
/// `Map.Entry.comparingByValue()` (the `priority`, ascending) `.thenComparing(classDesc().packageName())`.
/// Java breaks a remaining tie (same priority, same package) by `HashMap` iteration order, which is not a
/// stable order; here a tie is broken by the declaring class's simple name.
pub fn init() {
    use crate::de::hysky::skyblocker as sb;
    sb::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager::init(); // @Init (priority = -1) skyblock.tabhud.screenbuilder.WidgetManager.init
    sb::disable_all::DisableAll::init(); // @Init DisableAll.init
    sb::skyblocker_credits_screen::SkyblockerCreditsScreen::init_class(); // @Init SkyblockerCreditsScreen.initClass
    sb::skyblocker_screen::SkyblockerScreen::init_class(); // @Init SkyblockerScreen.initClass
    sb::update_notifications::UpdateNotifications::init(); // @Init UpdateNotifications.init
    sb::config::config_commands::ConfigCommands::init(); // @Init config.ConfigCommands.init
    sb::config::config_patch_loader::ConfigPatchLoader::init(); // @Init config.ConfigPatchLoader.init
    sb::debug::debug::Debug::init(); // @Init debug.Debug.init
    sb::debug::sky_block_resource_pack_downloader::SkyBlockResourcePackDownloader::init(); // @Init debug.SkyBlockResourcePackDownloader.init
    sb::skyblock::builders_wand_preview::BuildersWandPreview::init(); // @Init skyblock.BuildersWandPreview.init
    sb::skyblock::call_autocomplete::CallAutocomplete::init(); // @Init skyblock.CallAutocomplete.init
    sb::skyblock::cat_picture::CatPicture::init(); // @Init skyblock.CatPicture.init
    sb::skyblock::chapters_autocomplete::ChaptersAutocomplete::init(); // @Init skyblock.ChaptersAutocomplete.init
    sb::skyblock::chest_value::ChestValue::init(); // @Init skyblock.ChestValue.init
    sb::skyblock::gyro_overlay::GyroOverlay::init(); // @Init skyblock.GyroOverlay.init
    sb::skyblock::health_bars::HealthBars::init(); // @Init skyblock.HealthBars.init
    sb::skyblock::hide_status_effect_overlay::HideStatusEffectOverlay::init(); // @Init skyblock.HideStatusEffectOverlay.init
    sb::skyblock::inventory_search::InventorySearch::init(); // @Init skyblock.InventorySearch.init
    sb::skyblock::join_instance_autocomplete::JoinInstanceAutocomplete::init(); // @Init skyblock.JoinInstanceAutocomplete.init
    sb::skyblock::loadout_keybinds::LoadoutKeybinds::init(); // @Init skyblock.LoadoutKeybinds.init
    sb::skyblock::night_vision_command::NightVisionCommand::init(); // @Init skyblock.NightVisionCommand.init
    sb::skyblock::pet_cache::PetCache::init(); // @Init skyblock.PetCache.init
    sb::skyblock::quiver_warning::QuiverWarning::init(); // @Init skyblock.QuiverWarning.init
    sb::skyblock::rng_meter_autocomplete::RngMeterAutocomplete::init(); // @Init skyblock.RngMeterAutocomplete.init
    sb::skyblock::sack_item_autocomplete::SackItemAutocomplete::init(); // @Init skyblock.SackItemAutocomplete.init
    sb::skyblock::status_bar_tracker::StatusBarTracker::init(); // @Init skyblock.StatusBarTracker.init
    sb::skyblock::tips::Tips::init(); // @Init skyblock.Tips.init
    sb::skyblock::wardrobe_keybinds::WardrobeKeybinds::init(); // @Init skyblock.WardrobeKeybinds.init
    sb::skyblock::warp_autocomplete::WarpAutocomplete::init(); // @Init skyblock.WarpAutocomplete.init
    sb::skyblock::accessories::accessories_helper::AccessoriesHelper::init(); // @Init skyblock.accessories.AccessoriesHelper.init
    sb::skyblock::barn::glowing_mushrooms::GlowingMushrooms::init(); // @Init skyblock.barn.GlowingMushrooms.init
    sb::skyblock::calculators::calculator_command::CalculatorCommand::init(); // @Init skyblock.calculators.CalculatorCommand.init
    sb::skyblock::carnival::chivalrous_carnival::ChivalrousCarnival::init(); // @Init skyblock.carnival.ChivalrousCarnival.init
    sb::skyblock::carnival::zombie_shootout::ZombieShootout::init(); // @Init skyblock.carnival.ZombieShootout.init
    sb::skyblock::chat::chat_position_share::ChatPositionShare::init(); // @Init skyblock.chat.ChatPositionShare.init
    sb::skyblock::chat::chat_rules_handler::ChatRulesHandler::init(); // @Init skyblock.chat.ChatRulesHandler.init
    sb::skyblock::chat::confirmation_prompt_helper::ConfirmationPromptHelper::init(); // @Init skyblock.chat.ConfirmationPromptHelper.init
    sb::skyblock::chat::sack_message_price::SackMessagePrice::init(); // @Init skyblock.chat.SackMessagePrice.init
    sb::skyblock::chat::skyblock_xp_messages::SkyblockXpMessages::init(); // @Init skyblock.chat.SkyblockXpMessages.init
    sb::skyblock::chocolatefactory::egg_finder::EggFinder::init(); // @Init skyblock.chocolatefactory.EggFinder.init
    sb::skyblock::chocolatefactory::time_tower_reminder::TimeTowerReminder::init(); // @Init skyblock.chocolatefactory.TimeTowerReminder.init
    sb::skyblock::crimson::dojo::dojo_manager::DojoManager::init(); // @Init skyblock.crimson.dojo.DojoManager.init
    sb::skyblock::crimson::kuudra::danger_warning::DangerWarning::init(); // @Init skyblock.crimson.kuudra.DangerWarning.init
    sb::skyblock::crimson::kuudra::kuudra::Kuudra::init(); // @Init skyblock.crimson.kuudra.Kuudra.init
    sb::skyblock::crimson::kuudra::kuudra_waypoints::KuudraWaypoints::init(); // @Init skyblock.crimson.kuudra.KuudraWaypoints.init
    sb::skyblock::dungeon::blood_camp_helper::BloodCampHelper::init(); // @Init skyblock.dungeon.BloodCampHelper.init
    sb::skyblock::dungeon::dungeon_map::DungeonMap::init(); // @Init skyblock.dungeon.DungeonMap.init
    sb::skyblock::dungeon::dungeon_map_labels::DungeonMapLabels::init(); // @Init skyblock.dungeon.DungeonMapLabels.init
    sb::skyblock::dungeon::dungeon_map_texture::DungeonMapTexture::init(); // @Init skyblock.dungeon.DungeonMapTexture.init
    sb::skyblock::dungeon::dungeon_score::DungeonScore::init(); // @Init skyblock.dungeon.DungeonScore.init
    sb::skyblock::dungeon::dungeon_score_hud::DungeonScoreHUD::init(); // @Init skyblock.dungeon.DungeonScoreHUD.init
    sb::skyblock::dungeon::dungeon_textures::DungeonTextures::init(); // @Init skyblock.dungeon.DungeonTextures.init
    sb::skyblock::dungeon::fire_freeze_staff_timer::FireFreezeStaffTimer::init(); // @Init skyblock.dungeon.FireFreezeStaffTimer.init
    sb::skyblock::dungeon::goldor_waypoints_manager::GoldorWaypointsManager::init(); // @Init skyblock.dungeon.GoldorWaypointsManager.init
    sb::skyblock::dungeon::guardian_health::GuardianHealth::init(); // @Init skyblock.dungeon.GuardianHealth.init
    sb::skyblock::dungeon::livid_color::LividColor::init(); // @Init skyblock.dungeon.LividColor.init
    sb::skyblock::dungeon::rare_room_alert::RareRoomAlert::init(); // @Init skyblock.dungeon.RareRoomAlert.init
    sb::skyblock::dungeon::device::arrow_align::ArrowAlign::init(); // @Init skyblock.dungeon.device.ArrowAlign.init
    sb::skyblock::dungeon::device::lights_on::LightsOn::init(); // @Init skyblock.dungeon.device.LightsOn.init
    sb::skyblock::dungeon::device::simon_says::SimonSays::init(); // @Init skyblock.dungeon.device.SimonSays.init
    sb::skyblock::dungeon::device::target_practice::TargetPractice::init(); // @Init skyblock.dungeon.device.TargetPractice.init
    sb::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::init_class(); // @Init skyblock.dungeon.partyfinder.PartyFinderScreen.initClass
    sb::skyblock::dungeon::preview::room_preview::RoomPreview::init(); // @Init skyblock.dungeon.preview.RoomPreview.init
    sb::skyblock::dungeon::preview::room_preview_server::RoomPreviewServer::init(); // @Init skyblock.dungeon.preview.RoomPreviewServer.init
    sb::skyblock::dungeon::puzzle::creeper_beams::CreeperBeams::init(); // @Init skyblock.dungeon.puzzle.CreeperBeams.init
    sb::skyblock::dungeon::puzzle::dungeon_blaze::DungeonBlaze::init(); // @Init skyblock.dungeon.puzzle.DungeonBlaze.init
    sb::skyblock::dungeon::puzzle::ice_fill::IceFill::init(); // @Init skyblock.dungeon.puzzle.IceFill.init
    sb::skyblock::dungeon::puzzle::ice_path::IcePath::init(); // @Init skyblock.dungeon.puzzle.IcePath.init
    sb::skyblock::dungeon::puzzle::three_weirdos::ThreeWeirdos::init(); // @Init skyblock.dungeon.puzzle.ThreeWeirdos.init
    sb::skyblock::dungeon::puzzle::tic_tac_toe::TicTacToe::init(); // @Init skyblock.dungeon.puzzle.TicTacToe.init
    sb::skyblock::dungeon::puzzle::boulder::boulder::Boulder::init(); // @Init skyblock.dungeon.puzzle.boulder.Boulder.init
    sb::skyblock::dungeon::puzzle::waterboard::waterboard_one_flow::WaterboardOneFlow::init(); // @Init skyblock.dungeon.puzzle.waterboard.WaterboardOneFlow.init
    sb::skyblock::dungeon::puzzle::waterboard::waterboard_previewer::WaterboardPreviewer::init(); // @Init skyblock.dungeon.puzzle.waterboard.WaterboardPreviewer.init
    sb::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::init(); // @Init skyblock.dungeon.secrets.DungeonManager.init
    sb::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager::init(); // @Init skyblock.dungeon.secrets.DungeonPlayerManager.init
    sb::skyblock::dungeon::secrets::dungeon_web_socket::DungeonWebSocket::init(); // @Init skyblock.dungeon.secrets.DungeonWebSocket.init
    sb::skyblock::dungeon::secrets::secret_sync::SecretSync::init(); // @Init skyblock.dungeon.secrets.SecretSync.init
    sb::skyblock::dungeon::secrets::secrets_tracker::SecretsTracker::init(); // @Init skyblock.dungeon.secrets.SecretsTracker.init
    sb::skyblock::dwarven::block_break_prediction::BlockBreakPrediction::init(); // @Init skyblock.dwarven.BlockBreakPrediction.init
    sb::skyblock::dwarven::carpet_highlighter::CarpetHighlighter::init(); // @Init skyblock.dwarven.CarpetHighlighter.init
    sb::skyblock::dwarven::commission_labels::CommissionLabels::init(); // @Init skyblock.dwarven.CommissionLabels.init
    sb::skyblock::dwarven::corpse_finder::CorpseFinder::init(); // @Init skyblock.dwarven.CorpseFinder.init
    sb::skyblock::dwarven::crystals_chest_highlighter::CrystalsChestHighlighter::init(); // @Init skyblock.dwarven.CrystalsChestHighlighter.init
    sb::skyblock::dwarven::crystals_locations_manager::CrystalsLocationsManager::init(); // @Init skyblock.dwarven.CrystalsLocationsManager.init
    sb::skyblock::dwarven::glacite_cold_overlay::GlaciteColdOverlay::init(); // @Init skyblock.dwarven.GlaciteColdOverlay.init
    sb::skyblock::dwarven::metal_detector::MetalDetector::init(); // @Init skyblock.dwarven.MetalDetector.init
    sb::skyblock::dwarven::pickobulus_helper::PickobulusHelper::init(); // @Init skyblock.dwarven.PickobulusHelper.init
    sb::skyblock::dwarven::wishing_compass_solver::WishingCompassSolver::init(); // @Init skyblock.dwarven.WishingCompassSolver.init
    sb::skyblock::dwarven::profittrackers::powder_mining_tracker::PowderMiningTracker::init(); // @Init skyblock.dwarven.profittrackers.PowderMiningTracker.init
    sb::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker::CorpseProfitTracker::init(); // @Init skyblock.dwarven.profittrackers.corpse.CorpseProfitTracker.init
    sb::skyblock::end::ender_nodes::EnderNodes::init(); // @Init skyblock.end.EnderNodes.init
    sb::skyblock::end::mute_endermen_sounds::MuteEndermenSounds::init(); // @Init skyblock.end.MuteEndermenSounds.init
    sb::skyblock::end::the_end::TheEnd::init(); // @Init skyblock.end.TheEnd.init
    sb::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::init(); // @Init skyblock.entity.MobBoundingBoxes.init
    sb::skyblock::entity::mob_glow::MobGlow::init(); // @Init skyblock.entity.MobGlow.init
    sb::skyblock::entity::glow::adder::crimson_glow_adder::CrimsonGlowAdder::init(); // @Init skyblock.entity.glow.adder.CrimsonGlowAdder.init
    sb::skyblock::entity::glow::adder::debug_glow_adder::DebugGlowAdder::init(); // @Init skyblock.entity.glow.adder.DebugGlowAdder.init
    sb::skyblock::entity::glow::adder::dungeon_glow_adder::DungeonGlowAdder::init(); // @Init skyblock.entity.glow.adder.DungeonGlowAdder.init
    sb::skyblock::entity::glow::adder::end_glow_adder::EndGlowAdder::init(); // @Init skyblock.entity.glow.adder.EndGlowAdder.init
    sb::skyblock::entity::glow::adder::galatea_glow_adder::GalateaGlowAdder::init(); // @Init skyblock.entity.glow.adder.GalateaGlowAdder.init
    sb::skyblock::entity::glow::adder::garden_glow_adder::GardenGlowAdder::init(); // @Init skyblock.entity.glow.adder.GardenGlowAdder.init
    sb::skyblock::entity::glow::adder::hub_glow_adder::HubGlowAdder::init(); // @Init skyblock.entity.glow.adder.HubGlowAdder.init
    sb::skyblock::entity::glow::adder::kuudra_glow_adder::KuudraGlowAdder::init(); // @Init skyblock.entity.glow.adder.KuudraGlowAdder.init
    sb::skyblock::entity::glow::adder::mushroom_desert_glow_adder::MushroomDesertGlowAdder::init(); // @Init skyblock.entity.glow.adder.MushroomDesertGlowAdder.init
    sb::skyblock::entity::glow::adder::rift_glow_adder::RiftGlowAdder::init(); // @Init skyblock.entity.glow.adder.RiftGlowAdder.init
    sb::skyblock::entity::glow::adder::safari_glow_adder::SafariGlowAdder::init(); // @Init skyblock.entity.glow.adder.SafariGlowAdder.init
    sb::skyblock::entity::glow::adder::slayer_glow_adder::SlayerGlowAdder::init(); // @Init skyblock.entity.glow.adder.SlayerGlowAdder.init
    sb::skyblock::entity::glow::adder::torrhus_canyon_glow_adder::TorrhusCanyonGlowAdder::init(); // @Init skyblock.entity.glow.adder.TorrhusCanyonGlowAdder.init
    sb::skyblock::events::event_notifications::EventNotifications::init(); // @Init skyblock.events.EventNotifications.init
    sb::skyblock::events::greatspook::math_teacher_helper::MathTeacherHelper::init(); // @Init skyblock.events.greatspook.MathTeacherHelper.init
    sb::skyblock::fancybars::fancy_status_bars::FancyStatusBars::init_static(); // @Init skyblock.fancybars.FancyStatusBars.initStatic
    sb::skyblock::fancybars::vanilla_style_mana_bar::VanillaStyleManaBar::init(); // @Init skyblock.fancybars.VanillaStyleManaBar.init
    sb::skyblock::fishing::fishing_helper::FishingHelper::init(); // @Init skyblock.fishing.FishingHelper.init
    sb::skyblock::fishing::fishing_hook_display_helper::FishingHookDisplayHelper::init(); // @Init skyblock.fishing.FishingHookDisplayHelper.init
    sb::skyblock::fishing::sea_creature_tracker::SeaCreatureTracker::init(); // @Init skyblock.fishing.SeaCreatureTracker.init
    sb::skyblock::foraging::sweep_overlay::SweepOverlay::init(); // @Init skyblock.foraging.SweepOverlay.init
    sb::skyblock::foraging::tree_felled_notification::TreeFelledNotification::init(); // @Init skyblock.foraging.TreeFelledNotification.init
    sb::skyblock::foraging::galatea::invisibug_highlighter::InvisibugHighlighter::init(); // @Init skyblock.foraging.galatea.InvisibugHighlighter.init
    sb::skyblock::foraging::galatea::lushlilac_highlighter::LushlilacHighlighter::init_class(); // @Init skyblock.foraging.galatea.LushlilacHighlighter.initClass
    sb::skyblock::foraging::galatea::sea_lumies_highlighter::SeaLumiesHighlighter::init_class(); // @Init skyblock.foraging.galatea.SeaLumiesHighlighter.initClass
    sb::skyblock::foraging::galatea::terracotta_puzzle::TerracottaPuzzle::init(); // @Init skyblock.foraging.galatea.TerracottaPuzzle.init
    sb::skyblock::foraging::torrhus::critter_capsule_helper::CritterCapsuleHelper::init(); // @Init skyblock.foraging.torrhus.CritterCapsuleHelper.init
    sb::skyblock::foraging::torrhus::desert_temple_puzzle::DesertTemplePuzzle::init(); // @Init skyblock.foraging.torrhus.DesertTemplePuzzle.init
    sb::skyblock::foraging::torrhus::honeyhive_highlighter::HoneyhiveHighlighter::init_class(); // @Init skyblock.foraging.torrhus.HoneyhiveHighlighter.initClass
    sb::skyblock::foraging::torrhus::ruby_veilshroom_highlighter::RubyVeilshroomHighlighter::init_class(); // @Init skyblock.foraging.torrhus.RubyVeilshroomHighlighter.initClass
    sb::skyblock::garden::farming_hud::FarmingHud::init(); // @Init skyblock.garden.FarmingHud.init
    sb::skyblock::garden::garden_plots::GardenPlots::init(); // @Init skyblock.garden.GardenPlots.init
    sb::skyblock::garden::greenhouse_paste::GreenhousePaste::init(); // @Init skyblock.garden.GreenhousePaste.init
    sb::skyblock::garden::lower_sensitivity::LowerSensitivity::init(); // @Init skyblock.garden.LowerSensitivity.init
    sb::skyblock::garden::vacuum_cache::VacuumCache::init(); // @Init skyblock.garden.VacuumCache.init
    sb::skyblock::garden::visitor::visitor_helper::VisitorHelper::initialize(); // @Init skyblock.garden.visitor.VisitorHelper.initialize
    sb::skyblock::hunting::attributes::Attributes::init(); // @Init skyblock.hunting.Attributes.init
    sb::skyblock::hunting::attributes_debug::AttributesDebug::init(); // @Init skyblock.hunting.AttributesDebug.init
    sb::skyblock::hunting::floor_drops::FloorDrops::init(); // @Init skyblock.hunting.FloorDrops.init
    sb::skyblock::hunting::silence_phantoms::SilencePhantoms::init(); // @Init skyblock.hunting.SilencePhantoms.init
    sb::skyblock::item::consumable_protection::ConsumableProtection::init(); // @Init skyblock.item.ConsumableProtection.init
    sb::skyblock::item::hotbar_slot_lock::HotbarSlotLock::init(); // @Init skyblock.item.HotbarSlotLock.init
    sb::skyblock::item::item_cooldowns::ItemCooldowns::init(); // @Init skyblock.item.ItemCooldowns.init
    sb::skyblock::item::item_price::ItemPrice::init(); // @Init skyblock.item.ItemPrice.init
    sb::skyblock::item::item_protection::ItemProtection::init(); // @Init skyblock.item.ItemProtection.init
    sb::skyblock::item::skyblock_inventory_screen::SkyblockInventoryScreen::init_equipment(); // @Init skyblock.item.SkyblockInventoryScreen.initEquipment
    sb::skyblock::item::skyblock_item_data::SkyblockItemData::init(); // @Init skyblock.item.SkyblockItemData.init
    sb::skyblock::item::value_breakdown_popup::ValueBreakdownPopup::init_class(); // @Init skyblock.item.ValueBreakdownPopup.initClass
    sb::skyblock::item::background::item_background_manager::ItemBackgroundManager::init(); // @Init skyblock.item.background.ItemBackgroundManager.init
    sb::skyblock::item::custom::custom_animated_helmet_textures::CustomAnimatedHelmetTextures::init(); // @Init skyblock.item.custom.CustomAnimatedHelmetTextures.init
    sb::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes::init(); // @Init skyblock.item.custom.CustomArmorAnimatedDyes.init
    sb::skyblock::item::custom::custom_armor_dye_colors::CustomArmorDyeColors::init(); // @Init skyblock.item.custom.CustomArmorDyeColors.init
    sb::skyblock::item::custom::custom_armor_trims::CustomArmorTrims::init(); // @Init skyblock.item.custom.CustomArmorTrims.init
    sb::skyblock::item::custom::custom_helmet_textures::CustomHelmetTextures::init(); // @Init skyblock.item.custom.CustomHelmetTextures.init
    sb::skyblock::item::custom::custom_item_names::CustomItemNames::init(); // @Init skyblock.item.custom.CustomItemNames.init
    sb::skyblock::item::custom::repo_dye_colors::RepoDyeColors::init(); // @Init skyblock.item.custom.RepoDyeColors.init
    sb::skyblock::item::custom::skyblock_item_models::SkyblockItemModels::init(); // @Init skyblock.item.custom.SkyblockItemModels.init
    sb::skyblock::item::custom::screen::customize_screen::CustomizeScreen::init_things(); // @Init skyblock.item.custom.screen.CustomizeScreen.initThings
    sb::skyblock::item::slottext::slot_text_manager::SlotTextManager::init(); // @Init skyblock.item.slottext.SlotTextManager.init
    sb::skyblock::item::tooltip::backpack_preview::BackpackPreview::init(); // @Init skyblock.item.tooltip.BackpackPreview.init
    sb::skyblock::item::tooltip::item_tooltip::ItemTooltip::init(); // @Init skyblock.item.tooltip.ItemTooltip.init
    sb::skyblock::item::tooltip::tooltip_manager::TooltipManager::init(); // @Init skyblock.item.tooltip.TooltipManager.init
    sb::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager::init(); // @Init skyblock.item.wikilookup.WikiLookupManager.init
    sb::skyblock::itemlist::item_repository::ItemRepository::init(); // @Init skyblock.itemlist.ItemRepository.init
    sb::skyblock::mayors::jerry_timer::JerryTimer::init(); // @Init skyblock.mayors.JerryTimer.init
    sb::skyblock::museum::museum_item_cache::MuseumItemCache::init(); // @Init skyblock.museum.MuseumItemCache.init
    sb::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen::init_class(); // @Init skyblock.profileviewer.ProfileViewerScreen.initClass
    sb::skyblock::profileviewer2::profile_viewer::ProfileViewer::init(); // @Init skyblock.profileviewer2.ProfileViewer.init
    sb::skyblock::profileviewer2::utils::accessory_power_calculator::AccessoryPowerCalculator::init(); // @Init skyblock.profileviewer2.utils.AccessoryPowerCalculator.init
    sb::skyblock::profileviewer2::utils::collection_tiers::CollectionTiers::init(); // @Init skyblock.profileviewer2.utils.CollectionTiers.init
    sb::skyblock::profileviewer2::utils::elite_leaderboards::EliteLeaderboards::init(); // @Init skyblock.profileviewer2.utils.EliteLeaderboards.init
    sb::skyblock::profileviewer2::utils::tree::hotm_tree_builder::HotmTreeBuilder::init_class(); // @Init skyblock.profileviewer2.utils.tree.HotmTreeBuilder.initClass
    sb::skyblock::rift::mc_grubber_updater::McGrubberUpdater::init(); // @Init skyblock.rift.McGrubberUpdater.init
    sb::skyblock::rift::the_rift::TheRift::init(); // @Init skyblock.rift.TheRift.init
    sb::skyblock::searchoverlay::search_over_manager::SearchOverManager::init(); // @Init skyblock.searchoverlay.SearchOverManager.init
    sb::skyblock::shortcut::shortcuts::Shortcuts::init(); // @Init skyblock.shortcut.Shortcuts.init
    sb::skyblock::slayers::slayer_manager::SlayerManager::init(); // @Init skyblock.slayers.SlayerManager.init
    sb::skyblock::slayers::boss::demonlord::mute_blaze_sounds::MuteBlazeSounds::init(); // @Init skyblock.slayers.boss.demonlord.MuteBlazeSounds.init
    sb::skyblock::slayers::boss::sven::mute_wolf_sounds::MuteWolfSounds::init(); // @Init skyblock.slayers.boss.sven.MuteWolfSounds.init
    sb::skyblock::slayers::boss::tarantula::mute_spider_sounds::MuteSpiderSounds::init(); // @Init skyblock.slayers.boss.tarantula.MuteSpiderSounds.init
    sb::skyblock::slayers::boss::voidgloom::beacon_highlighter::BeaconHighlighter::init(); // @Init skyblock.slayers.boss.voidgloom.BeaconHighlighter.init
    sb::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::init(); // @Init skyblock.slayers.boss.voidgloom.LazerTimer.init
    sb::skyblock::slayers::features::slayer_timer::SlayerTimer::init(); // @Init skyblock.slayers.features.SlayerTimer.init
    sb::skyblock::special::dungeons_special_effects::DungeonsSpecialEffects::init(); // @Init skyblock.special.DungeonsSpecialEffects.init
    sb::skyblock::special::dye_special_effects::DyeSpecialEffects::init(); // @Init skyblock.special.DyeSpecialEffects.init
    sb::skyblock::special::rare_drop_special_effects::RareDropSpecialEffects::init(); // @Init skyblock.special.RareDropSpecialEffects.init
    sb::skyblock::special::trophy_special_effects::TrophySpecialEffects::init(); // @Init skyblock.special.TrophySpecialEffects.init
    sb::skyblock::speedpreset::speed_presets::SpeedPresets::init(); // @Init skyblock.speedpreset.SpeedPresets.init
    sb::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::setup(); // @Init skyblock.storageoverlay.StorageOverlayScreen.setup
    sb::skyblock::tabhud::tab_hud::TabHud::init(); // @Init skyblock.tabhud.TabHud.init
    sb::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen::init_commands(); // @Init skyblock.tabhud.config.WidgetsConfigurationScreen.initCommands
    sb::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen::init_commands(); // @Init skyblock.tabhud.config.list.WidgetsListScreen.initCommands
    sb::skyblock::tabhud::util::player_list_manager::PlayerListManager::init(); // @Init skyblock.tabhud.util.PlayerListManager.init
    sb::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::init(); // @Init skyblock.teleport.PredictiveSmoothAOTE.init
    sb::skyblock::teleport::teleport_overlay::TeleportOverlay::init(); // @Init skyblock.teleport.TeleportOverlay.init
    sb::skyblock::waypoint::fairy_souls::FairySouls::init(); // @Init skyblock.waypoint.FairySouls.init
    sb::skyblock::waypoint::individual_waypoint::IndividualWaypoint::init(); // @Init skyblock.waypoint.IndividualWaypoint.init
    sb::skyblock::waypoint::mythological_ritual::MythologicalRitual::init(); // @Init skyblock.waypoint.MythologicalRitual.init
    sb::skyblock::waypoint::relics::Relics::init(); // @Init skyblock.waypoint.Relics.init
    sb::skyblock::waypoint::waypoints::Waypoints::init(); // @Init skyblock.waypoint.Waypoints.init
    sb::utils::api_authentication::ApiAuthentication::init(); // @Init utils.ApiAuthentication.init
    sb::utils::enchanted_book_utils::EnchantedBookUtils::init(); // @Init utils.EnchantedBookUtils.init
    sb::utils::logs_folder_finder::LogsFolderFinder::init(); // @Init utils.LogsFolderFinder.init
    sb::utils::neu_repo_manager::NEURepoManager::init(); // @Init utils.NEURepoManager.init
    sb::utils::server_tick_counter::ServerTickCounter::init(); // @Init utils.ServerTickCounter.init
    sb::utils::utils::Utils::init(); // @Init utils.Utils.init
    sb::utils::chat::chat_message_listener::init(); // @Init utils.chat.ChatMessageListener.init
    sb::utils::container::container_solver_manager::ContainerSolverManager::init(); // @Init utils.container.ContainerSolverManager.init
    sb::utils::data::constants::constant_data::ConstantData::init(); // @Init utils.data.constants.ConstantData.init
    sb::utils::discord::discord_rpc_manager::DiscordRPCManager::init(); // @Init utils.discord.DiscordRPCManager.init
    sb::utils::mayor::mayor_utils::MayorUtils::init(); // @Init utils.mayor.MayorUtils.init
    sb::utils::render::render_helper::RenderHelper::init(); // @Init utils.render.RenderHelper.init
    sb::utils::render::skyblocker_render_pipelines::SkyblockerRenderPipelines::init(); // @Init utils.render.SkyblockerRenderPipelines.init
    sb::utils::render::gui::pip::gui_equipment_renderer::GuiEquipmentRenderer::<()>::init(); // Java static: S unused // @Init utils.render.gui.pip.GuiEquipmentRenderer.init
    sb::utils::render::texture::fallbacked_texture::init(); // @Init utils.render.texture.FallbackedTexture.init
    sb::utils::render::title::title_container::TitleContainer::init(); // @Init utils.render.title.TitleContainer.init
    sb::utils::time::skyblock_time::SkyblockTime::init(); // @Init utils.time.SkyblockTime.init
    sb::utils::ws::skyblocker_web_socket::SkyblockerWebSocket::init(); // @Init utils.ws.SkyblockerWebSocket.init
    sb::utils::ws::ws_state_manager::WsStateManager::init(); // @Init utils.ws.WsStateManager.init
    sb::skyblock::auction_bazaar_autocomplete::AuctionBazaarAutocomplete::init(); // @Init (priority = 100) skyblock.AuctionBazaarAutocomplete.init
    sb::skyblock::dungeon::puzzle::trivia::Trivia::init(); // @Init (priority = 100) skyblock.dungeon.puzzle.Trivia.init
}
