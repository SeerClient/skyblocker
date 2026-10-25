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
pub static CORPSES_BY_TYPE: compat::MainThread<std::collections::HashMap<CorpseType, Vec<Corpse>>> = compat::MainThread::new(|| std::collections::HashMap::new());
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
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(CorpseFinder::onChatMessage);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| CorpseFinder::on_chat_message(text, overlay));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CorpseFinder::extract_rendering(&__a0); }));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            if !(compat::get_state(&IS_LOCATION_CORRECT)) {
                return;
            }
            for corpses in CORPSES_BY_TYPE.get_mut().values_mut() {
                for corpse in corpses.iter_mut() {
                    // `client.player.hasLineOfSight(corpse.entity)`
                    if !corpse.seen && mcrust::mcrust::modding::game::with_player(|player| mcrust::mcrust::modding::game::with_entity(corpse.entity, |entity| compat::has_line_of_sight(&*player, &*entity))).flatten().unwrap_or(false) {
                        CorpseFinder::set_seen(corpse);
                    }
                }
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
                use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_block_pos_argument_type::ClientBlockPosArgumentType;
                use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseTypeArgumentType;
                type Source = client_command_registration_callback::FabricClientCommandSource;
                // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("corpseHelper").then(literal("shareLocation")
                //   .then(argument("blockPos", ClientBlockPosArgumentType.blockPos()).then(argument("corpseType", CorpseTypeArgumentType.corpseType())
                //     .executes(context -> { shareLocation(ClientBlockPosArgumentType.getBlockPos(context, "blockPos"), CorpseTypeArgumentType.getCorpseType(context, "corpseType")); return Command.SINGLE_SUCCESS; }))))));
                let mut corpse_type = client_command_registration_callback::argument("corpseType", Arc::new(CorpseTypeArgumentType::corpse_type().base));
                corpse_type.executes(Arc::new(|context: &CommandContext<Source>| {
                    CorpseFinder::share_location(&ClientBlockPosArgumentType::get_block_pos(context, "blockPos"), CorpseTypeArgumentType::get_corpse_type(context, "corpseType"));
                    Ok(1)
                }));
                let mut block_pos = client_command_registration_callback::argument("blockPos", Arc::new(ClientBlockPosArgumentType::block_pos()));
                block_pos.then(&corpse_type);
                let mut share_location = client_command_registration_callback::literal("shareLocation");
                share_location.then(&block_pos);
                let mut corpse_helper = client_command_registration_callback::literal("corpseHelper");
                corpse_helper.then(&share_location);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&corpse_helper);
                dispatcher.dispatcher().register(&root);
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
        let corpses = CORPSES_BY_TYPE.get_mut().entry(corpse_type).or_insert_with(Vec::new);
        let armor_stand_position: BlockPos = armor_stand.base.base.block_position();
        if !corpses.iter().any(|c| c.block_position() == armor_stand_position) {
            let mut corpse_waypoint: compat::Shared<dyn Waypoint>;
            let mut color: Vec<f32> = CorpseFinder::get_colors(&corpse_type.fields().color);
            corpse_waypoint = (WaypointBase::new(armor_stand_position.above(), Type::OutlinedWaypoint, color.clone()) as compat::Shared<dyn Waypoint>);
            // `!seenDebugWarning && (seenDebugWarning = true)`
            if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && SkyblockerConfigManager::get().debug.corpse_finder_debug && !compat::get_state(&SEEN_DEBUG_WARNING) && { compat::set_state(&SEEN_DEBUG_WARNING, true); true } {
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append((component::literal("Corpse finder debug mode is active! Please use it only for the sake of debugging corpse detection!")).with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold]))); });
            }
            let mut new_corpse: Corpse = Corpse::new(armor_stand, corpse_waypoint.clone(), corpse_type);
            corpses.push(new_corpse);
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().mining.glacite.enable_corpse_finder) || !(compat::get_state(&IS_LOCATION_CORRECT)) {
            return;
        }
        for corpses in CORPSES_BY_TYPE.get().values() {
            for corpse in corpses.iter() {
                if { let __f = compat::dispatch(&corpse.waypoint, |__t| __t.should_render()); __f(&corpse.waypoint) } && (corpse.seen || (crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && SkyblockerConfigManager::get().debug.corpse_finder_debug)) {
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
        logger().debug(&format!("{}State of corpsesByType: {:?}", PREFIX, CORPSES_BY_TYPE.get()));
        let mut corpse_type_string: String = matcher_corpse.group(1).unwrap().to_uppercase();
        // `EnumUtils.getEnum(CorpseType.class, corpseTypeString, CorpseType.UNKNOWN)`: the constant of that name, else
        // the default.
        let mut corpse_type: CorpseType = CorpseType::values().into_iter().find(|value| value.name() == corpse_type_string).unwrap_or(CorpseType::Unknown);
        let Some(corpses) = CORPSES_BY_TYPE.get().get(&corpse_type) else {
            logger().warn(&format!("{}Couldn't get corpses! corpse type string: {}, parsed corpse type: {}", PREFIX, corpse_type_string, corpse_type));
            return true;
        };
        // Since squared distance comparison will yield the same result as normal distance comparison, we can use squared distance to avoid square root calculation
        let distance_to_player = |corpse: &Corpse| -> f64 {
            mcrust::mcrust::modding::game::with_player(|player| mcrust::mcrust::modding::game::with_entity(corpse.entity, |entity| entity.base().distance_to_sqr_entity(&player.base.base.base.base))).flatten().unwrap_or(f64::INFINITY)
        };
        if let Some(corpse) = corpses.iter().min_by(|a, b| distance_to_player(a).total_cmp(&distance_to_player(b))) {
            logger().info(&format!("{}Found corpse, marking as found! {}: {}", PREFIX, corpse.entity, corpse.block_position().as_vec3i().to_short_string()));
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
            CorpseFinder::share_location(&corpse.block_position().above(), corpse.corpse_type);
            return;
        }
        if (util::get_millis()).wrapping_sub(corpse.message_last_sent) < (300 as i64) {
            return;
        }
        corpse.message_last_sent = util::get_millis();
        let waypoint_pos: BlockPos = compat::read(&corpse.waypoint, |__t| __t.waypoint_base().pos.clone()).unwrap();
        let mut message: MutableComponent = (constants::PREFIX.get_mut())()
            .append_str("Found a ")
            .append(component::literal(&format!("{} Corpse", compat::capitalize_fully(&corpse.corpse_type.get_serialized_name()))).with_color(TextColor::from_legacy_format(corpse.corpse_type.fields().color).unwrap().get_value()))
            .append_str(&format!(" at {}!", corpse.block_position().above().as_vec3i().to_short_string()));
        message = message.with_style_fn(|style| style
            .with_click_event(Some(mcrust::net::minecraft::network::chat::click_event::ClickEvent::new_run_command(format!("/skyblocker corpseHelper shareLocation {} {}", crate::de::hysky::skyblocker::utils::pos_utils::PosUtils::to_space_separated_string(&waypoint_pos), corpse.corpse_type.to_string().to_lowercase()))))
            // java: .withHoverEvent(new HoverEvent.ShowText(Component.literal("Click to share the location in chat!").withStyle(ChatFormatting.GREEN)))
            .with_hover_event(Some(mcrust::net::minecraft::network::chat::hover_event::HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Click to share the location in chat!").with_style_format(ChatFormatting::Green))))));
        // java: Minecraft.getInstance().player.sendSystemMessage(..) - the component itself (its click and hover events)
        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&message); });
    }

    /// java: private static void shareLocation(BlockPos pos, CorpseType corpseType)
    pub fn share_location(pos: &BlockPos, mut corpse_type: CorpseType) {
        message_scheduler::MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get(), &format!("/pc {} | ({} Corpse)", CorpseFinder::to_skyhanni_format(pos), compat::capitalize_fully(&corpse_type.get_serialized_name())), true);
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
        // java: try { x = Integer.parseInt(..); y = ..; z = ..; } catch (NumberFormatException e) - `parseInt` does
        // not trim; the first failing group is the exception, whose message is `For input string: "<s>"`.
        let parse = |group: &str| -> Result<i32, String> {
            let s = matcher.group_named(group).unwrap();
            s.parse::<i32>().map_err(|_| format!("For input string: \"{}\"", s))
        };
        let (x, y, z): (i32, i32, i32) = match (|| Ok::<_, String>((parse("x")?, parse("y")?, parse("z")?)))() {
            Ok(xyz) => xyz,
            Err(reason) => {
                logger().warn(&format!("{}Failed to parse message: `{}`, reason: {}", PREFIX, message, reason));
                return;
            }
        };
        logger().debug(&format!("{}Parsed message! X:{}, Y:{}, Z:{}", PREFIX, x, y, z));
        let mut found_corpse: bool = false;
        let mut parsed_pos: BlockPos = BlockPos::new(x.wrapping_sub(1), y, z.wrapping_sub(1));
        for corpses in CORPSES_BY_TYPE.get_mut().values_mut() {
            for corpse in corpses.iter_mut() {
                if compat::read(&corpse.waypoint, |__t| __t.waypoint_base().pos.clone()) == Some(parsed_pos) {
                    corpse.seen = true;
                    found_corpse = true;
                    logger().info(&format!("{}Setting corpse {} as seen!", PREFIX, corpse.entity));
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append_str("Parsed message from chat, adding corpse at ").append_str(&corpse.block_position().as_vec3i().to_short_string())); });
                    break;
                }
            }
        }
        if !found_corpse {
            logger().warn(&format!("{}Did NOT find any match for corpses! corpsesByType.values(): {:?}", PREFIX, CORPSES_BY_TYPE.get().values().collect::<Vec<_>>()));
            logger().info(&format!("{}Proceeding to iterate over all corpses!", PREFIX));
            for corpses in CORPSES_BY_TYPE.get().values() {
                for corpse in corpses.iter() {
                    logger().info(&format!("{}Corpse: {}, BlockPos: {}", PREFIX, corpse.entity, corpse.block_position().as_vec3i().to_short_string()));
                }
            }
        }
    }

}

/// java: static class Corpse
pub struct Corpse {
    /// java: private final ArmorStand entity;
    /// The port's entities live in the level's storage and are lent by id (`game::with_entity`): Java's reference
    /// is the entity's id here.
    pub entity: i32,
    /// java: private final Waypoint waypoint;
    /// Waypoint position is always 1 above entity position
    pub waypoint: compat::Shared<dyn Waypoint>,
    /// java: private final CorpseType corpseType;
    pub corpse_type: CorpseType,
    /// java: private boolean seen;
    pub seen: bool,
    /// java: private long messageLastSent = 0;
    pub message_last_sent: i64,
}

impl std::fmt::Debug for Corpse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Corpse[entity={}, corpseType={}, seen={}]", self.entity, self.corpse_type, self.seen)
    }
}

impl Corpse {
    /// java: Corpse(ArmorStand entity, Waypoint waypoint, CorpseType corpseType)
    pub fn new(entity: &ArmorStand, mut waypoint: compat::Shared<dyn Waypoint>, mut corpse_type: CorpseType) -> Self {
        let mut this = Self {
            entity: entity.base.base.get_id(),
            waypoint,
            corpse_type,
            seen: false,
            message_last_sent: ((0) as i64) as i64,
        };
        this
    }

    /// `this.entity.blockPosition()`: the live entity's; once the level has dropped it, the position it was found at
    /// (the waypoint is "always 1 above entity position", and a corpse's armor stand does not move).
    fn block_position(&self) -> BlockPos {
        mcrust::mcrust::modding::game::with_entity(self.entity, |entity| entity.base().block_position())
            .unwrap_or_else(|| compat::read(&self.waypoint, |__t| __t.waypoint_base().pos.clone()).unwrap().below())
    }
}
