//! Ported from de.hysky.skyblocker.skyblock.dungeon.LividColor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::Livid;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::hud;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::world::entity::player::player;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<Block, ChatFormatting> WOOL_TO_FORMATTING = Map.of( Blocks.WOOL.red(), ChatFormatting.RED, Blocks.WOOL.yellow(), ChatFormatting.YELLOW, Blocks.WOOL.lime(), ChatFormatting.GREE
/// Keyed by the block OBJECT (Java's `Block` has identity equality; the port's `Block` has no `Hash`/`Eq`), so a
/// list of pairs looked up with `Arc::ptr_eq`.
pub static WOOL_TO_FORMATTING: compat::MainThread<Vec<(Arc<Block>, ChatFormatting)>> = compat::MainThread::new(|| Vec::from([(mcrust::net::minecraft::world::level::block::blocks::wool().red().clone(), ChatFormatting::Red), (mcrust::net::minecraft::world::level::block::blocks::wool().yellow().clone(), ChatFormatting::Yellow), (mcrust::net::minecraft::world::level::block::blocks::wool().lime().clone(), ChatFormatting::Green), (mcrust::net::minecraft::world::level::block::blocks::wool().green().clone(), ChatFormatting::DarkGreen), (mcrust::net::minecraft::world::level::block::blocks::wool().blue().clone(), ChatFormatting::Blue), (mcrust::net::minecraft::world::level::block::blocks::wool().magenta().clone(), ChatFormatting::LightPurple), (mcrust::net::minecraft::world::level::block::blocks::wool().purple().clone(), ChatFormatting::DarkPurple), (mcrust::net::minecraft::world::level::block::blocks::wool().gray().clone(), ChatFormatting::Gray), (mcrust::net::minecraft::world::level::block::blocks::wool().white().clone(), ChatFormatting::White)]));
/// java: private static final Map<String, ChatFormatting> LIVID_TO_FORMATTING = Map.of( "Hockey Livid", ChatFormatting.RED, "Arcade Livid", ChatFormatting.YELLOW, "Smile Livid", ChatFormatting.GREEN, "Frog Liv
pub static LIVID_TO_FORMATTING: compat::MainThread<std::collections::HashMap<String, ChatFormatting>> = compat::MainThread::new(|| std::collections::HashMap::from([("Hockey Livid".to_string(), ChatFormatting::Red), ("Arcade Livid".to_string(), ChatFormatting::Yellow), ("Smile Livid".to_string(), ChatFormatting::Green), ("Frog Livid".to_string(), ChatFormatting::DarkGreen), ("Scream Livid".to_string(), ChatFormatting::Blue), ("Crossed Livid".to_string(), ChatFormatting::LightPurple), ("Purple Livid".to_string(), ChatFormatting::DarkPurple), ("Doctor Livid".to_string(), ChatFormatting::Gray), ("Vendetta Livid".to_string(), ChatFormatting::White)]));
/// java: private static final Supplier<DungeonsConfig.Livid> CONFIG = () -> SkyblockerConfigManager.get().dungeons.livid;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<Livid>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.livid.clone()));
/// java: private static ChatFormatting color = ChatFormatting.AQUA;
pub static COLOR: compat::MainThread<ChatFormatting> = compat::MainThread::new(|| ChatFormatting::Aqua);
/// java: private static Block lastColor = Blocks.AIR;
pub static LAST_COLOR: compat::MainThread<Arc<Block>> = compat::MainThread::new(|| blocks::blocks().get("AIR"));
/// java: private static boolean isInitialized = false;
pub static IS_INITIALIZED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean correctLividIdFound = false;
pub static CORRECT_LIVID_ID_FOUND: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static int correctLividId = 0;
pub static CORRECT_LIVID_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final long OFFSET_DURATION = 2000;
pub const OFFSET_DURATION: i64 = ((2000) as i64) as i64;
/// java: private static long toggleTime = 0;
pub static TOGGLE_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((0) as i64) as i64));

/// java: public class LividColor
#[derive(Clone)]
pub struct LividColor {}

impl LividColor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            LividColor::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { LividColor::update(&__a0); }));
    }

    /// java: private static void update(PrimitiveCollector collector)
    pub fn update(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let mut config: compat::Shared<Livid> = SkyblockerConfigManager::get().dungeons.livid.clone();
        if !(compat::read(&config, |__t| __t.enable_livid_color_text) || compat::read(&config, |__t| __t.enable_livid_color_title) || compat::read(&config, |__t| __t.enable_livid_color_glow) || compat::read(&config, |__t| __t.enable_livid_color_bounding_box)) {
            return;
        }
        let client = compat::minecraft();
        if !(Utils::is_in_dungeons() && DungeonManager::is_in_boss() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().is_some()) {
            return;
        }
        let mut current_color: Arc<Block> = (mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&BlockPos::new(5, 110, 42))).get_block();
        if !(WOOL_TO_FORMATTING.get().iter().any(|(block, _)| Arc::ptr_eq(block, &current_color)) && !(std::ptr::addr_eq(std::sync::Arc::as_ptr(&current_color), std::sync::Arc::as_ptr(LAST_COLOR.get_mut())))) {
            return;
        }
        if !(compat::get_state(&IS_INITIALIZED)) && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.has_effect(mcrust::net::minecraft::world::effect::mob_effects::blindness())).unwrap_or(false) {
            compat::set_state(&TOGGLE_TIME, compat::millis());
            compat::set_state(&IS_INITIALIZED, true);
        } else if compat::get_state(&IS_INITIALIZED) && (compat::millis()).wrapping_sub(compat::get_state(&TOGGLE_TIME)) >= OFFSET_DURATION {
            LividColor::on_livid_color_found(client, &current_color);
            if !(compat::get_state(&CORRECT_LIVID_ID_FOUND)) {
                let color = *COLOR.get();
                let mut livid_name: String = LIVID_TO_FORMATTING.get().iter().find(|(_, formatting)| **formatting == color).map(|(name, _)| name.clone()).unwrap_or_else(|| "unknown".to_string());
                // client.level.players().stream().filter(entity -> entity.getName().getString().equals(lividName)).findFirst()
                //     .ifPresent(entity -> correctLividId = entity.getId());
                {
                    use mcrust::net::minecraft::world::entity::entity::Entity as _;
                    let players = mcrust::mcrust::modding::game::level().unwrap().with_players(|players| players.to_vec());
                    let found = players.iter().find_map(|avatar| {
                        if mcrust::mcrust::modding::game::is_held_here(&**avatar) {
                            return None;
                        }
                        let entity = mcrust::mcrust::modding::game::lock_entity(&**avatar, "livid_color.rs:LividColor.update");
                        (entity.get_game_profile_name() == livid_name).then(|| entity.base().get_id())
                    });
                    if let Some(id) = found {
                        compat::set_state(&CORRECT_LIVID_ID, id);
                    }
                }
                compat::set_state(&CORRECT_LIVID_ID_FOUND, true);
            }
            *LAST_COLOR.get_mut() = current_color.clone();
        }
    }

    /// java: private static void onLividColorFound(Minecraft client, Block color)
    pub fn on_livid_color_found(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, color: &Arc<Block>) {
        *COLOR.get_mut() = WOOL_TO_FORMATTING.get().iter().find(|(block, _)| Arc::ptr_eq(block, color)).map(|(_, formatting)| *formatting).unwrap();
        let mut color_string: String = mcrust::net::minecraft::core::registries::built_in_registries::block().get_key(&color.erased_obj()).unwrap().path().to_string();
        color_string = (compat::substring2(&color_string, 0, (compat::str_len(&color_string)).wrapping_sub(5))).to_uppercase();
        let mut message: MutableComponent = { let (__q0,) = (*COLOR.get(),); (component::literal(&compat::regex::replace_all(&(CONFIG.get_mut())().lock().borrow().livid_color_text, "\\[color]", &color_string))).with_style_format(__q0) };
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.enable_livid_color_text) {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}", (constants::PREFIX.get_mut())().append(message.clone()).get_string()), true);
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.enable_livid_color_title) {
            client.gui.hud.reset_title_times();
            mcrust::net::minecraft::client::multiplayer::client_packet_listener::apply_set_title_text(&mut client.gui.hud, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone()));
        }
    }

    /// java: public static boolean allowGlow()
    pub fn allow_glow() -> bool {
        return (!(compat::read(&SkyblockerConfigManager::get().dungeons.livid, |__t| __t.enable_livid_color_glow)) || !(DungeonManager::get_boss().is_floor(5)));
    }

    /// java: public static boolean shouldGlow(String name)
    pub fn should_glow(name: &str) -> bool {
        return (compat::read(&SkyblockerConfigManager::get().dungeons.livid, |__t| __t.enable_livid_color_glow) && *COLOR.get_mut() == LIVID_TO_FORMATTING.get_mut().get(name).cloned().unwrap());
    }

    /// java: public static boolean shouldDrawBoundingBox(String name)
    pub fn should_draw_bounding_box(name: &str) -> bool {
        return (compat::read(&SkyblockerConfigManager::get().dungeons.livid, |__t| __t.enable_livid_color_bounding_box) && *COLOR.get_mut() == LIVID_TO_FORMATTING.get_mut().get(name).cloned().unwrap());
    }

    /// java: @SuppressWarnings("DataFlowIssue") public static int getGlowColor(String name)
    pub fn get_glow_color(name: &str) -> i32 {
        if compat::read(&SkyblockerConfigManager::get().dungeons.livid, |__t| __t.enable_solid_color) {
            // java: return SkyblockerConfigManager.get().dungeons.livid.customColor.getRGB();
            return compat::read(&SkyblockerConfigManager::get().dungeons.livid, |__t| __t.custom_color.get_rgb());
        }
        if LIVID_TO_FORMATTING.get_mut().contains_key(name) {
            return TextColor::from_legacy_format(LIVID_TO_FORMATTING.get_mut().get(name).cloned().unwrap()).unwrap().get_value();
        }
        return mcrust::net::minecraft::network::chat::text_color::TextColor::WHITE.get_value();
    }

    /// java: public static int getCorrectLividId()
    pub fn get_correct_livid_id() -> i32 {
        return compat::get_state(&CORRECT_LIVID_ID);
    }

    /// java: private static void reset()
    pub fn reset() {
        *LAST_COLOR.get_mut() = blocks::blocks().get("AIR");
        compat::set_state(&TOGGLE_TIME, ((0) as i64) as i64);
        compat::set_state(&IS_INITIALIZED, false);
        compat::set_state(&CORRECT_LIVID_ID_FOUND, false);
        compat::set_state(&CORRECT_LIVID_ID, 0);
        *COLOR.get_mut() = ChatFormatting::Aqua;
    }

}
