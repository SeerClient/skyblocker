//! Ported from de.hysky.skyblocker.skyblock.dwarven.CrystalsLocationsManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::CrystalHollowsLocationsCategory;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::MiningLocationLabel;
use crate::de::hysky::skyblocker::skyblock::dwarven::nucleus_waypoints::NucleusWaypoints;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_pos_argument::ClientPosArgument;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::ws::message::crystals_waypoint_message::CrystalsWaypointMessage;
use crate::de::hysky::skyblocker::utils::ws::message::crystals_waypoint_subscribe_message::CrystalsWaypointSubscribeMessage;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;
use crate::de::hysky::skyblocker::utils::ws::ws_state_manager::WsStateManager;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::phys::vec3;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypoint as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<String, MiningLocationLabel.CrystalHollowsLocationsCategory> WAYPOINT_LOCATIONS = Arrays.stream(MiningLocationLabel.CrystalHollowsLocationsCategory.values()).collect(Collector
pub static WAYPOINT_LOCATIONS: LazyLock<std::collections::HashMap<String, CrystalHollowsLocationsCategory>> = LazyLock::new(|| CrystalHollowsLocationsCategory::values().into_iter().map(|category| (category.get_name(), category)).collect());
/// java: static final Pattern TEXT_CWORDS_PATTERN = Pattern.compile("\\Dx?(\\d{3})(?=[, ]),? ?y?(\\d{2,3})(?=[, ]),? ?z?(\\d{3})\\D?(?!\\d)");
pub static TEXT_CWORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\Dx?(\\d{3})(?=[, ]),? ?y?(\\d{2,3})(?=[, ]),? ?z?(\\d{3})\\D?(?!\\d)"));
/// java: private static final int REMOVE_UNKNOWN_DISTANCE = 50;
pub const REMOVE_UNKNOWN_DISTANCE: i32 = 50;
/// java: protected static Map<String, MiningLocationLabel> activeWaypoints = new HashMap<>();
pub static ACTIVE_WAYPOINTS: compat::MainThread<std::collections::HashMap<String, compat::Shared<MiningLocationLabel>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: protected static List<String> verifiedWaypoints = new ArrayList<>();
pub static VERIFIED_WAYPOINTS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: private static boolean updatedTime = false;
pub static UPDATED_TIME: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean updatedLocation = false;
pub static UPDATED_LOCATION: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final List<MiningLocationLabel.CrystalHollowsLocationsCategory> waypointsSent2Socket = new ArrayList<>();
pub static WAYPOINTS_SENT2_SOCKET: LazyLock<Mutex<Vec<CrystalHollowsLocationsCategory>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// java: public class CrystalsLocationsManager
#[derive(Clone)]
pub struct CrystalsLocationsManager {}

impl CrystalsLocationsManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { CrystalsLocationsManager::update(); }), 40) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CrystalsLocationsManager::extract_rendering(&__a0); }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(CrystalsLocationsManager::extractLocationFromMessage);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| CrystalsLocationsManager::extract_location_from_message(text, overlay));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: ClientCommandRegistrationCallback.EVENT.register(CrystalsLocationsManager::registerWaypointLocationCommands);
                dispatcher.dispatcher().register(&CrystalsLocationsManager::waypoint_location_command());
            },
        ));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |__a0| { CrystalsLocationsManager::on_location_change(__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            CrystalsLocationsManager::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { NucleusWaypoints::extract_rendering(&__a0); }));
    }

    /// java: private static boolean extractLocationFromMessage(Component message, Boolean overlay)
    pub fn extract_location_from_message(message: &MutableComponent, mut overlay: bool) -> bool {
        if !(SkyblockerConfigManager::get().mining.crystals_waypoints.find_in_chat) || !(Utils::is_in_crystal_hollows()) || overlay {
            return true;
        }
        let mut text: String = chat_formatting::strip_formatting(&message.get_string());
        // java: try {..} catch (Exception e) { LOGGER.error("[Skyblocker Crystals Locations Manager] Encountered an exception while extracing a location from a chat message!", e); }
        // (an exception here is a panic, caught by `catch_unwind`; the body's `return true` is the closure's `Some(true)`)
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<bool> {
            if text.contains(":") && !(text.starts_with(&(constants::PREFIX.get_mut())().get_string())) {
                let mut user_message: String = (compat::regex::split_limit(&text, ":", 2))[(1) as usize].clone();
                let mut matcher: compat::regex::Matcher = TEXT_CWORDS_PATTERN.matcher(&user_message);
                if matcher.find() {
                    let mut block_pos: BlockPos = BlockPos::new(matcher.group(1).unwrap().parse::<i32>().expect("NumberFormatException"), matcher.group(2).unwrap().parse::<i32>().expect("NumberFormatException"), matcher.group(3).unwrap().parse::<i32>().expect("NumberFormatException"));
                    let mut location: String = format!("{} {} {}", block_pos.get_x(), block_pos.get_y(), block_pos.get_z());
                    if !(CrystalsLocationsManager::check_in_crystals(&block_pos)) {
                        return Some(true);
                    }
                    for waypoint_location in WAYPOINT_LOCATIONS.keys().cloned() {
                        if (compat::regex::split(&waypoint_location.to_lowercase(), " ")).iter().cloned().any(|word| user_message.to_lowercase().contains(&word)) {
                            if !(ACTIVE_WAYPOINTS.get_mut().contains_key(&waypoint_location)) {
                                CrystalsLocationsManager::add_custom_waypoint(waypoint_location.clone(), &block_pos);
                            }
                            return Some(true);
                        }
                    }
                    if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_connection(compat::minecraft()).is_none() {
                        return Some(true);
                    }
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&CrystalsLocationsManager::get_location_menu(&location, false)); });
                }
            }
            None
        })) {
            Ok(Some(value)) => return value,
            Ok(None) => {}
            Err(e) => {
                logger().error(&format!("[Skyblocker Crystals Locations Manager] Encountered an exception while extracing a location from a chat message! {}", compat::panic_message(&e)));
            }
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && SkyblockerConfigManager::get().mining.crystals_waypoints.enabled {
            for waypoint_location in WAYPOINT_LOCATIONS.values().copied() {
                let mut waypoint_linked_message: Option<String> = waypoint_location.get_linked_message();
                let mut waypoint_name: String = waypoint_location.get_name();
                if waypoint_linked_message.is_some() && text.starts_with(waypoint_linked_message.as_deref().unwrap()) && !(compat::lock(&VERIFIED_WAYPOINTS).iter().any(|e| *e == waypoint_name.clone())) {
                    CrystalsLocationsManager::add_custom_waypoint(waypoint_location.get_name(), &mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap());
                    compat::lock(&VERIFIED_WAYPOINTS).push(waypoint_name.clone());
                    CrystalsLocationsManager::try_send_waypoint2_socket(waypoint_location);
                }
            }
        }
        return true;
    }

    /// java: protected static boolean checkInCrystals(BlockPos pos)
    pub fn check_in_crystals(pos: &BlockPos) -> bool {
        return (pos.get_x() >= 202 && pos.get_x() <= 823 && pos.get_z() >= 202 && pos.get_z() <= 823 && pos.get_y() >= 31 && pos.get_y() <= 188);
    }

    /// java: private static void registerWaypointLocationCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_waypoint_location_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // (the tree is `waypoint_location_command`, which `init` registers through the client command event - the
        // dispatcher this signature names is the event's)
        let _ = (dispatcher, registry_access);
    }

    /// `registerWaypointLocationCommands`'s tree:
    /// java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("crystalWaypoints")
    ///   .then(literal("add").executes(_ -> { if (CLIENT.player == null) return 0;
    ///       CLIENT.player.sendSystemMessage(getLocationMenu((int) CLIENT.player.getX() + " " + (int) CLIENT.player.getY() + " " + (int) CLIENT.player.getZ(), true)); return Command.SINGLE_SUCCESS; })
    ///     .then(argument("pos", ClientBlockPosArgumentType.blockPos()).then(argument("place", StringArgumentType.greedyString())
    ///       .suggests((_, builder) -> suggest(WAYPOINT_LOCATIONS.keySet(), builder))
    ///       .executes(context -> addWaypointFromCommand(context.getSource(), getString(context, "place"), context.getArgument("pos", ClientPosArgument.class))))))
    ///   .then(literal("share").executes(_ -> { if (CLIENT.player == null) return 0; CLIENT.player.sendSystemMessage(getPlacesMenu("share")); return Command.SINGLE_SUCCESS; })
    ///     .then(argument("place", StringArgumentType.greedyString()).suggests(..).executes(context -> shareWaypoint(getString(context, "place")))))
    ///   .then(literal("remove").executes(_ -> { if (CLIENT.player == null) return 0; CLIENT.player.sendSystemMessage(getPlacesMenu("remove")); return Command.SINGLE_SUCCESS; })
    ///     .then(argument("place", StringArgumentType.greedyString()).suggests(..).executes(context -> removeWaypoint(getString(context, "place"))))));
    pub fn waypoint_location_command() -> mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder<client_command_registration_callback::FabricClientCommandSource> {
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_block_pos_argument_type::ClientBlockPosArgumentType;
        type Source = client_command_registration_callback::FabricClientCommandSource;
        // `suggest(WAYPOINT_LOCATIONS.keySet(), builder)`
        fn place_suggestions() -> mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<Source> {
            Arc::new(|_: &CommandContext<Source>, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| {
                let places: Vec<String> = WAYPOINT_LOCATIONS.keys().cloned().collect();
                Ok(mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&places, builder))
            })
        }
        // `CLIENT.player.sendSystemMessage(text)` - the client's chat line
        fn send_system_message(text: MutableComponent) -> i32 {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return 0;
            }
            compat::batch_w_send_feedback(&text);
            1
        }
        // add
        let mut add_place = client_command_registration_callback::argument("place", Arc::new(StringArgumentType::greedy_string()));
        add_place.suggests(Some(place_suggestions()));
        add_place.executes(Arc::new(|context: &CommandContext<Source>| {
            let place: String = StringArgumentType::get_string(context, "place");
            let location = context.get_argument::<compat::Shared<dyn ClientPosArgument>>("pos").unwrap().clone();
            Ok(CrystalsLocationsManager::add_waypoint_from_command(context.get_source(), &place, &location))
        }));
        let mut add_pos = client_command_registration_callback::argument("pos", Arc::new(ClientBlockPosArgumentType::block_pos()));
        add_pos.then(&add_place);
        let mut add = client_command_registration_callback::literal("add");
        add.executes(Arc::new(|_: &CommandContext<Source>| {
            let Some(location) = mcrust::mcrust::modding::game::with_player(|player| format!("{} {} {}", player.base.base.base.base.get_x() as i32, player.base.base.base.base.get_y() as i32, player.base.base.base.base.get_z() as i32)) else {
                return Ok(0);
            };
            Ok(send_system_message(CrystalsLocationsManager::get_location_menu(&location, true)))
        }));
        add.then(&add_pos);
        // share
        let mut share_place = client_command_registration_callback::argument("place", Arc::new(StringArgumentType::greedy_string()));
        share_place.suggests(Some(place_suggestions()));
        share_place.executes(Arc::new(|context: &CommandContext<Source>| Ok(CrystalsLocationsManager::share_waypoint(&StringArgumentType::get_string(context, "place")))));
        let mut share = client_command_registration_callback::literal("share");
        share.executes(Arc::new(|_: &CommandContext<Source>| Ok(send_system_message(CrystalsLocationsManager::get_places_menu("share")))));
        share.then(&share_place);
        // remove
        let mut remove_place = client_command_registration_callback::argument("place", Arc::new(StringArgumentType::greedy_string()));
        remove_place.suggests(Some(place_suggestions()));
        remove_place.executes(Arc::new(|context: &CommandContext<Source>| Ok(CrystalsLocationsManager::remove_waypoint(&StringArgumentType::get_string(context, "place")))));
        let mut remove = client_command_registration_callback::literal("remove");
        remove.executes(Arc::new(|_: &CommandContext<Source>| Ok(send_system_message(CrystalsLocationsManager::get_places_menu("remove")))));
        remove.then(&remove_place);
        let mut crystal_waypoints = client_command_registration_callback::literal("crystalWaypoints");
        crystal_waypoints.then(&add);
        crystal_waypoints.then(&share);
        crystal_waypoints.then(&remove);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&crystal_waypoints);
        root
    }

    /// java: protected static Component getSetLocationMessage(String location, BlockPos blockPos)
    pub fn get_set_location_message(location: &str, block_pos: &BlockPos) -> MutableComponent {
        let mut location_color: i32 = WAYPOINT_LOCATIONS.get(location).cloned().unwrap().get_color();
        return (constants::PREFIX.get_mut())().append(component::translatable_with_fallback_args("skyblocker.config.mining.crystalsWaypoints.addedWaypoint", Some("Added waypoint for '%s' at %s %s %s."), vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(location).with_color(location_color)), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(block_pos.get_x().to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(block_pos.get_y().to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(block_pos.get_z().to_string())]));
    }

    /// java: private static Component getLocationMenu(String location, boolean excludeUnknown)
    pub fn get_location_menu(location: &str, mut exclude_unknown: bool) -> MutableComponent {
        if (ACTIVE_WAYPOINTS.get_mut().len() as i32) == (WAYPOINT_LOCATIONS.len() as i32) || exclude_unknown && (WAYPOINT_LOCATIONS.len() as i32).wrapping_sub(ACTIVE_WAYPOINTS.get_mut().len() as i32) == 1 && !(ACTIVE_WAYPOINTS.get_mut().contains_key(&CrystalHollowsLocationsCategory::Unknown.get_name())) {
            return (constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.allActive").with_style_format(ChatFormatting::Red));
        }
        let mut text: MutableComponent = component::empty();
        for waypoint_location in WAYPOINT_LOCATIONS.keys().cloned() {
            if ACTIVE_WAYPOINTS.get_mut().contains_key(&waypoint_location) || exclude_unknown && waypoint_location == CrystalHollowsLocationsCategory::Unknown.get_name() {
                continue;
            }
            let mut location_color: i32 = WAYPOINT_LOCATIONS.get(&waypoint_location).cloned().unwrap().get_color();
            text = text.append((component::literal(&format!("[{}]", waypoint_location))).with_color(location_color).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker crystalWaypoints add {} {}", location, waypoint_location))))).with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.add").with_color(location_color)))))));
        }
        return (constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.config.mining.crystalsWaypoints.markLocation", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(location.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone())]));
    }

    /// java: private static Component getPlacesMenu(String action)
    pub fn get_places_menu(action: &str) -> MutableComponent {
        let mut text: MutableComponent = (constants::PREFIX.get_mut())();
        if ACTIVE_WAYPOINTS.get_mut().is_empty() {
            return text.append(component::translatable("skyblocker.config.mining.crystalsWaypoints.noActive").with_style_format(ChatFormatting::Red));
        }
        let mut hover_message: MutableComponent;
        if action == "remove" {
            text = text.append(component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.remove").append(component::literal(": ")));
            hover_message = component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.remove");
        } else {
            text = text.append(component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.share").append(component::literal(": ")));
            hover_message = component::translatable("skyblocker.config.mining.crystalsWaypoints.getLocationHover.share");
        }
        for waypoint_location in ACTIVE_WAYPOINTS.get_mut().keys().cloned() {
            let mut location_color: i32 = WAYPOINT_LOCATIONS.get(&waypoint_location).cloned().unwrap().get_color();
            // `hoverMessage.withColor(locationColor)` recolours the one component every iteration's hover holds (Java
            // shares the object); the port's components are values, so each hover gets the component as recoloured.
            hover_message = hover_message.with_color(location_color);
            let hover = hover_message.clone();
            text = text.append((component::literal(&format!("[{}]", waypoint_location))).with_color(location_color).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker crystalWaypoints {} {}", action, waypoint_location))))).with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(hover))))));
        }
        return text;
    }

    /// java: public static int addWaypointFromCommand(FabricClientCommandSource source, String place, ClientPosArgument location)
    pub fn add_waypoint_from_command(source: &client_command_registration_callback::FabricClientCommandSource, place: &str, location: &compat::Shared<dyn ClientPosArgument>) -> i32 {
        let mut block_pos: BlockPos = ({ let __f = compat::dispatch(location, |__t| __t.to_absolute_block_pos()); __f(location, source) });
        if WAYPOINT_LOCATIONS.contains_key(place) {
            CrystalsLocationsManager::add_custom_waypoint(place.to_string(), &block_pos);
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_connection(compat::minecraft()).is_none() {
                return 0;
            }
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&CrystalsLocationsManager::get_set_location_message(place, &block_pos)); });
        }
        return 1;
    }

    /// java: public static int shareWaypoint(String place)
    pub fn share_waypoint(place: &str) -> i32 {
        if ACTIVE_WAYPOINTS.get_mut().contains_key(place) {
            let mut pos: BlockPos = compat::read(&ACTIVE_WAYPOINTS.get_mut().get(place).cloned().unwrap(), |__t| __t.pos.clone()).unwrap();
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("{} {}: {}, {}, {}", (constants::PREFIX.get_mut())().get_string(), place, pos.get_x(), pos.get_y(), pos.get_z()), false);
        } else {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_connection(compat::minecraft()).is_none() {
                return 0;
            }
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.shareFail").with_style_format(ChatFormatting::Red))); });
        }
        return 1;
    }

    /// java: public static int removeWaypoint(String place)
    pub fn remove_waypoint(place: &str) -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_connection(compat::minecraft()).is_none() {
            return 0;
        }
        if ACTIVE_WAYPOINTS.get_mut().contains_key(place) {
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.removeSuccess").with_style_format(ChatFormatting::Green)).append(component::literal(place).with_color(WAYPOINT_LOCATIONS.get(place).cloned().unwrap().get_color()))); });
            ACTIVE_WAYPOINTS.get_mut().remove(place);
            compat::remove_item(&mut compat::lock(&VERIFIED_WAYPOINTS), &place.to_string());
        } else {
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.removeFail").with_style_format(ChatFormatting::Red))); });
        }
        return 1;
    }

    /// java: public static void addCustomWaypointFromSocket(CrystalsWaypointMessage... messages)
    pub fn add_custom_waypoint_from_socket(messages: &Vec<CrystalsWaypointMessage>) {
        let mut received_waypoint_names: MutableComponent = component::empty();
        let mut should_send: bool = false;
        for message in messages.iter() {
            let mut category = message.location();
            let mut pos: BlockPos = message.coordinates();
            if ACTIVE_WAYPOINTS.get_mut().contains_key(&category.get_name()) {
                continue;
            }
            if category == CrystalHollowsLocationsCategory::FairyGrotto && !(SkyblockerConfigManager::get().mining.crystals_waypoints.share_fairy_grotto) {
                continue;
            }
            should_send = true;
            CrystalsLocationsManager::remove_unknown_near(&pos);
            let mut waypoint: compat::Shared<MiningLocationLabel> = MiningLocationLabel::new((compat::shared(category) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), pos.clone());
            compat::lock(&WAYPOINTS_SENT2_SOCKET).push(category);
            ACTIVE_WAYPOINTS.get_mut().insert(category.get_name(), waypoint.clone());
            received_waypoint_names = received_waypoint_names.append(component::literal(&category.get_name()).with_color(category.get_color()));
            if !std::ptr::eq(message, &messages[((messages.len() as i32).wrapping_sub(1)) as usize]) {
                received_waypoint_names = received_waypoint_names.append_str(", ");
            }
        }
        if !should_send {
            return;
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.webSocket.receivedCrystalsWaypoint", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(received_waypoint_names.clone())]))); });
    }

    /// java: protected static void addCustomWaypoint(String waypointName, BlockPos pos)
    pub fn add_custom_waypoint(mut waypoint_name: String, pos: &BlockPos) {
        CrystalsLocationsManager::remove_unknown_near(pos);
        let mut category: CrystalHollowsLocationsCategory = WAYPOINT_LOCATIONS.get(&waypoint_name).cloned().unwrap();
        let mut waypoint: compat::Shared<MiningLocationLabel> = MiningLocationLabel::new((compat::shared(category) as compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category>), pos.clone());
        ACTIVE_WAYPOINTS.get_mut().insert(waypoint_name.clone(), waypoint.clone());
    }

    /// java: private static void removeUnknownNear(BlockPos location)
    pub fn remove_unknown_near(location: &BlockPos) {
        let mut name: String = CrystalHollowsLocationsCategory::Unknown.get_name();
        let mut unknown_waypoint: Option<compat::Shared<MiningLocationLabel>> = ACTIVE_WAYPOINTS.get_mut().get(&name).cloned();
        if let Some(unknown_waypoint) = &unknown_waypoint {
            let mut distance: f64 = { let (__q0,) = (&vec3::at_center_of(location),); unknown_waypoint.lock().borrow().center_pos.as_ref().unwrap().distance_to(__q0) };
            if distance < (REMOVE_UNKNOWN_DISTANCE as f64) {
                ACTIVE_WAYPOINTS.get_mut().remove(&name);
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SkyblockerConfigManager::get().mining.crystals_waypoints.enabled {
            for crystals_waypoint in ACTIVE_WAYPOINTS.get_mut().values().cloned() {
                ({ let __f = compat::dispatch(&crystals_waypoint, |__t| __t.extract_rendering()); __f(&compat::view(&crystals_waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: private static void onLocationChange(Location newLocation)
    pub fn on_location_change(mut new_location: Location) {
        if new_location == Location::CrystalHollows {
            compat::set_state(&UPDATED_LOCATION, true);
        }
    }

    /// java: public static void onTimeUpdate()
    pub fn on_time_update() {
        compat::set_state(&UPDATED_TIME, true);
    }

    /// java: private static void reset()
    pub fn reset() {
        ACTIVE_WAYPOINTS.get_mut().clear();
        compat::lock(&VERIFIED_WAYPOINTS).clear();
        compat::lock(&WAYPOINTS_SENT2_SOCKET).clear();
        compat::set_state(&UPDATED_TIME, false);
        compat::set_state(&UPDATED_LOCATION, false);
    }

    /// java: private static void update()
    pub fn update() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_connection(compat::minecraft()).is_none() || !(SkyblockerConfigManager::get().mining.crystals_waypoints.enabled) || !(Utils::is_in_crystal_hollows()) {
            return;
        }
        if compat::get_state(&UPDATED_TIME) && compat::get_state(&UPDATED_LOCATION) {
            // java: WsStateManager.subscribeServer(Service.CRYSTAL_WAYPOINTS, Optional.of(CrystalsWaypointSubscribeMessage.create(CLIENT.level)));
            let message = CrystalsWaypointSubscribeMessage::create(&mcrust::mcrust::modding::game::level().unwrap());
            WsStateManager::subscribe_server(Service::CrystalWaypoints, Some(&message as &dyn crate::de::hysky::skyblocker::utils::ws::message::message::AnyMessage));
            compat::set_state(&UPDATED_TIME, false);
            compat::set_state(&UPDATED_LOCATION, false);
        }
        let mut location: String = compat::substring(&Utils::get_island_area(), 2);
        if !(location == "Unknown") && WAYPOINT_LOCATIONS.contains_key(&location) {
            if !(ACTIVE_WAYPOINTS.get_mut().contains_key(&location)) {
                let mut player_location: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
                CrystalsLocationsManager::add_custom_waypoint(location.clone(), &player_location);
            }
            CrystalsLocationsManager::try_send_waypoint2_socket(WAYPOINT_LOCATIONS.get(&location).cloned().unwrap());
        }
    }

    /// java: private static void trySendWaypoint2Socket(MiningLocationLabel.CrystalHollowsLocationsCategory category)
    pub fn try_send_waypoint2_socket(mut category: CrystalHollowsLocationsCategory) {
        if compat::lock(&WAYPOINTS_SENT2_SOCKET).iter().any(|e| *e == category) {
            return;
        }
        if category == CrystalHollowsLocationsCategory::FairyGrotto && !(SkyblockerConfigManager::get().mining.crystals_waypoints.share_fairy_grotto) {
            return;
        }
        // java: WsMessageHandler.sendServerMessage(Service.CRYSTAL_WAYPOINTS, new CrystalsWaypointMessage(category, CLIENT.player.blockPosition()));
        WsMessageHandler::send_server_message(Service::CrystalWaypoints, &CrystalsWaypointMessage::new(category, mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap()));
        compat::lock(&WAYPOINTS_SENT2_SOCKET).push(category);
    }

}
