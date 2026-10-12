//! Ported from de.hysky.skyblocker.skyblock.dwarven.CorpseFinder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseType;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static boolean isLocationCorrect = false;
pub static IS_LOCATION_CORRECT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final Pattern CORPSE_FOUND_PATTERN = Pattern.compile("([A-Z]+) CORPSE LOOT!");
pub static CORPSE_FOUND_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([A-Z]+) CORPSE LOOT!"));
/// java: private static final Pattern COORDS_PATTERN = Pattern.compile("x: (?<x>-?\\d+), y: (?<y>\\d+), z: (?<z>-?\\d+)");
pub static COORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("x: (?<x>-?\\d+), y: (?<y>\\d+), z: (?<z>-?\\d+)"));
/// java: private static final String PREFIX = "[Skyblocker Corpse Finder] ";
pub const PREFIX: &str = "[Skyblocker Corpse Finder] ";
/// java: private static final Map<CorpseType, List<Corpse>> corpsesByType = new EnumMap<>(CorpseType.class);
pub static CORPSES_BY_TYPE: compat::MainThread<std::collections::HashMap<CorpseType, Vec<Corpse>>> = compat::MainThread::new(|| std::any::TypeId::of::<CorpseType>());
/// java: private static boolean seenDebugWarning = false;
pub static SEEN_DEBUG_WARNING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class CorpseFinder
#[derive(Clone)]
pub struct CorpseFinder {}

impl CorpseFinder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            compat::set_state(&IS_LOCATION_CORRECT, false);
            CORPSES_BY_TYPE.get_mut().clear();
        }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |__a0| { CorpseFinder::handle_location_change(__a0); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(CorpseFinder::onChatMessage);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CorpseFinder::extract_rendering(&__a0); }));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            if !(compat::get_state(&IS_LOCATION_CORRECT)) {
                return;
            }
            for corpses in CORPSES_BY_TYPE.get_mut().values().cloned() {
                for corpse in corpses.iter() {
                    // TODO(fabric2seer): `LocalPlayer.hasLineOfSight` has no Rust method in the port (looked for `has_line_of_sight` in LocalPlayer and its supertypes)
                    if !corpse.seen && mcrust::mcrust::modding::game::with_player(|player| player.has_line_of_sight(&corpse.entity)).unwrap_or(false) {
                        CorpseFinder::set_seen(&mut corpse);
                    }
                }
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static void handleLocationChange(Location location)
    pub fn handle_location_change(mut location: Location) {
        compat::set_state(&IS_LOCATION_CORRECT, (location == Location::GlaciteMineshafts));
    }

    /// java: public static void checkIfCorpse(Entity entity)
    pub fn check_if_corpse(entity: &dyn Entity) {
        if let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() {
            CorpseFinder::check_if_corpse_armor_stand(armor_stand);
        }
    }

    /// java: public static void checkIfCorpse(ArmorStand armorStand)
    pub fn check_if_corpse_armor_stand(armor_stand: &ArmorStand) {
        if !(compat::get_state(&IS_LOCATION_CORRECT)) || !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) {
            return;
        }
        if armor_stand.base.base.has_custom_name() || armor_stand.is_invisible_field() || armor_stand.show_base_plate() {
            return;
        }
        CorpseFinder::handle_armor_stand(armor_stand);
    }

    /// java: private static void handleArmorStand(ArmorStand armorStand)
    pub fn handle_armor_stand(armor_stand: &ArmorStand) {
        let mut helmet_item_id: String = armor_stand.get_item_by_slot(EquipmentSlot::Head).get_skyblock_id();
        let mut corpse_type: CorpseType = CorpseType::from_helmet_item_id(&helmet_item_id);
        if corpse_type == CorpseType::Unknown {
            return;
        }
        logger().debug(&format!("{}Triggered code for handleArmorStand and matched with ITEM_IDS", PREFIX));
        let corpses = CORPSES_BY_TYPE.get_mut().entry(corpse_type).or_insert_with_key(|k| (|_| Vec::new())(k.clone()));
        if !corpses.iter().cloned().any(|c| (c.entity.base.base.block_position() == armor_stand.base.base.block_position())) {
            let mut corpse_waypoint: compat::Shared<dyn Waypoint>;
            let mut color: Vec<f32> = CorpseFinder::get_colors(&corpse_type.fields().color);
            corpse_waypoint = (WaypointBase::new(armor_stand.base.base.block_position().above(), Type::OutlinedWaypoint, color.clone()) as compat::Shared<dyn Waypoint>);
            // TODO(fabric2seer): the condition: an assignment used as a value
            //   java: Debug.debugEnabled() && SkyblockerConfigManager.get().debug.corpseFinderDebug && !seenDebugWarning && (seenDebugWarning = true)
            if todo!("fabric2seer: the condition (TODO above)") {
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::literal("Corpse finder debug mode is active! Please use it only for the sake of debugging corpse detection!")).with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold])).to_string()); });
            }
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: let mut new_corpse: Corpse = Corpse::new(armor_stand.clone(), corpse_waypoint.clone(), corpse_type);
            let mut new_corpse: Corpse = todo!("fabric2seer: the initialiser of `newCorpse` (TODO above)");
            corpses.push(new_corpse.clone());
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) || !(compat::get_state(&IS_LOCATION_CORRECT)) {
            return;
        }
        for corpses in CORPSES_BY_TYPE.get_mut().values().cloned() {
            for corpse in corpses.iter() {
                if { let __f = compat::dispatch(&corpse.waypoint, |__t| __t.should_render()); __f(&corpse.waypoint) } && corpse.seen || crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && SkyblockerConfigManager::get().debug.corpse_finder_debug {
                    ({ let __f = compat::dispatch(&corpse.waypoint, |__t| __t.extract_rendering()); __f(&compat::view(&corpse.waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
        }
    }

    /// java: private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if overlay || !(compat::get_state(&IS_LOCATION_CORRECT)) || !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return true;
        }
        let mut string: String = text.get_string();
        if string.contains(compat::minecraft().get_user().get_name()) {
            return true;
        }
        if SkyblockerConfigManager::get().mining.glacite.enable_parsing_chat_corpse_finder {
            CorpseFinder::parse_cords(text);
        }
        let mut matcher_corpse: compat::regex::Matcher = CORPSE_FOUND_PATTERN.matcher(&string);
        if !matcher_corpse.find() {
            return true;
        }
        logger().debug(&format!("{}Triggered code for onChatMessage", PREFIX));
        // TODO(fabric2seer): a log call with a non-literal format
        //   java: LOGGER.debug(PREFIX + "State of corpsesByType: {}", corpsesByType);
        let mut corpse_type_string: String = matcher_corpse.group(1).unwrap().to_uppercase();
        // TODO(fabric2seer): static call EnumUtils.getEnum(..)
        //   java: CorpseType corpseType = EnumUtils.getEnum(CorpseType.class, corpseTypeString, CorpseType.UNKNOWN);
        let mut corpse_type: CorpseType = todo!("fabric2seer: the initialiser of `corpseType` (TODO above)");
        let Some(mut corpses) = CORPSES_BY_TYPE.get_mut().get(&corpse_type).cloned() else {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.warn(PREFIX + "Couldn't get corpses! corpse type string: {}, parsed corpse type: {}", corpseTypeString, corpseType);
            return true;
        };
        if let Some(corpse) = corpses.iter().cloned().min_by(|a, b| { let key = |corpse| mcrust::mcrust::modding::game::with_player(|player| corpse.entity.base.base.distance_to_sqr_entity(player)); key(a).total_cmp(&key(b)) }) {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.info(PREFIX + "Found corpse, marking as found! {}: {}", corpse.entity.getType(), corpse.entity.blockPosition().toShortString());
            ({ let __f = compat::dispatch(&corpse.waypoint, |__t| __t.set_found()); __f(&corpse.waypoint) });
        } else {
            logger().warn(&format!("{}Couldn't find the closest corpse despite triggering onChatMessage!", PREFIX));
        }
        return true;
    }

    /// java: @SuppressWarnings("DataFlowIssue") private static void setSeen(Corpse corpse)
    pub fn set_seen(corpse: &mut Corpse) {
        corpse.seen = true;
        if SkyblockerConfigManager::get().mining.glacite.auto_share_corpses {
            CorpseFinder::share_location(&corpse.entity.base.base.block_position().above(), corpse.corpse_type);
            return;
        }
        if (util::get_millis()).wrapping_sub(corpse.message_last_sent) < (300 as i64) {
            return;
        }
        corpse.message_last_sent = util::get_millis();
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: Minecraft.getInstance().player.sendSystemMessage(
        //   java: 				Constants.PREFIX.get()
        //   java: 						.append("Found a ")
        //   java: 						.append(Component.literal(WordUtils.capitalizeFully(corpse.corpseType.getSerializedName()) + " Corpse")
        //   java: 								.withColor(TextColor.fromLegacyFormat(corpse.corpseType.color).getValue()))
        //   java: 						.append(" at " + corpse.entity.blockPosition().above().toShortString() + "!")
        //   java: 						.withStyle(style -> style.withClickEvent(new ClickEvent.RunCommand("/skyblocker corpseHelper shareLocation " + PosUtils.toSpaceSeparatedString(corpse.waypoint.pos) + " " + corpse.corpseType.toString().toLowerCase(Locale.ENGLISH)))
        //   java: 								.withHoverEvent(new HoverEvent.ShowText(Component.literal("Click to share the location in chat!").withStyle(ChatFormatting.GREEN)))));
    }

    /// java: private static void shareLocation(BlockPos pos, CorpseType corpseType)
    pub fn share_location(pos: &BlockPos, mut corpse_type: CorpseType) {
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: MessageScheduler.INSTANCE.sendMessageAfterCooldown("/pc " + toSkyhanniFormat(pos) + " | (" + WordUtils.capitalizeFully(corpseType.getSerializedName()) + " Corpse)", true);
    }

    /// java: @SuppressWarnings("DataFlowIssue") private static float[] getColors(ChatFormatting color)
    pub fn get_colors(color: &ChatFormatting) -> Vec<f32> {
        return ColorUtils::get_float_components(TextColor::from_legacy_format(*color).unwrap().get_value());
    }

    /// java: private static String toSkyhanniFormat(BlockPos pos)
    pub fn to_skyhanni_format(pos: &BlockPos) -> String {
        return format!("x: {}, y: {}, z: {}", pos.get_x().wrapping_add(1), pos.get_y(), pos.get_z().wrapping_add(1));
    }

    /// java: private static void parseCords(Component text)
    pub fn parse_cords(text: &MutableComponent) {
        let mut message: String = text.get_string();
        let mut matcher: compat::regex::Matcher = COORDS_PATTERN.matcher(&message);
        if !matcher.find() {
            return;
        }
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        // java: try {..} catch (NumberFormatException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            x = matcher.group_named("x").unwrap().trim().parse::<i32>().expect("NumberFormatException");
            y = matcher.group_named("y").unwrap().trim().parse::<i32>().expect("NumberFormatException");
            z = matcher.group_named("z").unwrap().trim().parse::<i32>().expect("NumberFormatException");
        }));
        if let Err(e) = __result {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.warn(PREFIX + "Failed to parse message: `{}`, reason: {}", message, e.getMessage());
            return;
        }
        // TODO(fabric2seer): a log call with a non-literal format
        //   java: LOGGER.debug(PREFIX + "Parsed message! X:{}, Y:{}, Z:{}", x, y, z);
        let mut found_corpse: bool = false;
        let mut parsed_pos: BlockPos = BlockPos::new(x.wrapping_sub(1), y, z.wrapping_sub(1));
        for corpses in CORPSES_BY_TYPE.get_mut().values().cloned() {
            for corpse in corpses.iter_mut() {
                if compat::read(&corpse.waypoint, |__t| __t.waypoint_base().pos.clone()).unwrap() == parsed_pos {
                    corpse.seen = true;
                    found_corpse = true;
                    // TODO(fabric2seer): a log call with a non-literal format
                    //   java: LOGGER.info(PREFIX + "Setting corpse {} as seen!", corpse.entity);
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str("Parsed message from chat, adding corpse at ").append_str(&corpse.entity.base.base.block_position().as_vec3i().to_short_string()).to_string()); });
                    break;
                }
            }
        }
        if !found_corpse {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.warn(PREFIX + "Did NOT find any match for corpses! corpsesByType.values(): {}", corpsesByType.values());
            logger().info(&format!("{}Proceeding to iterate over all corpses!", PREFIX));
            for corpses in CORPSES_BY_TYPE.get_mut().values().cloned() {
                for corpse in corpses.iter() {
                    // TODO(fabric2seer): a log call with a non-literal format
                    //   java: LOGGER.info(PREFIX + "Corpse: {}, BlockPos: {}", corpse.entity, corpse.entity.blockPosition());
                }
            }
        }
    }

}

/// java: static class Corpse
pub struct Corpse {
    /// java: private final ArmorStand entity;
    pub entity: ArmorStand,
    /// java: private final Waypoint waypoint;
    pub waypoint: compat::Shared<dyn Waypoint>,
    /// java: private final CorpseType corpseType;
    pub corpse_type: CorpseType,
    /// java: private boolean seen;
    pub seen: bool,
    /// java: private long messageLastSent = 0;
    pub message_last_sent: i64,
}

impl Corpse {
    /// java: Corpse(ArmorStand entity, Waypoint waypoint, CorpseType corpseType)
    pub fn new(mut entity: ArmorStand, mut waypoint: compat::Shared<dyn Waypoint>, mut corpse_type: CorpseType) -> Self {
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        //   java: this.entity = entity;
        let __init_waypoint = waypoint.clone();
        let __init_seen = false;
        let __init_corpse_type = corpse_type;
        let mut this = Self {
            entity: todo!("fabric2seer: the field value (TODO above)"),
            waypoint: __init_waypoint,
            corpse_type: __init_corpse_type,
            seen: __init_seen,
            message_last_sent: ((0) as i64) as i64,
        };
        this
    }

}
