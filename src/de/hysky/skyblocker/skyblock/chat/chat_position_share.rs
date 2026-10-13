//! Ported from de.hysky.skyblocker.skyblock.chat.ChatPositionShare by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::Area;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern SIMPLE_COORDS_PATTERN = Pattern.compile("(?<x>-?\\d+)(?:\\.\\d*)? (?<y>\\d+)(?:\\.\\d*)? (?<z>-?\\d+)(?:\\.\\d*)?");
pub static SIMPLE_COORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<x>-?\\d+)(?:\\.\\d*)? (?<y>\\d+)(?:\\.\\d*)? (?<z>-?\\d+)(?:\\.\\d*)?"));
/// java: private static final Pattern SIMPLE_COMMA_COORDS_PATTERN = Pattern.compile("(?<x>-?\\d+)(?:\\.\\d*)?, (?<y>\\d+)(?:\\.\\d*)?, (?<z>-?\\d+)(?:\\.\\d*)?");
pub static SIMPLE_COMMA_COORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<x>-?\\d+)(?:\\.\\d*)?, (?<y>\\d+)(?:\\.\\d*)?, (?<z>-?\\d+)(?:\\.\\d*)?"));
/// java: private static final Pattern GENERIC_COORDS_PATTERN = Pattern.compile("x: (?<x>-?\\d+)(?:\\.\\d*)?, y: (?<y>\\d+)(?:\\.\\d*)?, z: (?<z>-?\\d+)(?:\\.\\d*)?");
pub static GENERIC_COORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("x: (?<x>-?\\d+)(?:\\.\\d*)?, y: (?<y>\\d+)(?:\\.\\d*)?, z: (?<z>-?\\d+)(?:\\.\\d*)?"));
/// java: private static final Pattern SKYBLOCKER_COORDS_PATTERN = Pattern.compile("x: (?<x>-?\\d+)(?:\\.\\d*)?, y: (?<y>\\d+)(?:\\.\\d*)?, z: (?<z>-?\\d+)(?:\\.\\d*)?(?: \\| (?<area>[^|]+))");
pub static SKYBLOCKER_COORDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("x: (?<x>-?\\d+)(?:\\.\\d*)?, y: (?<y>\\d+)(?:\\.\\d*)?, z: (?<z>-?\\d+)(?:\\.\\d*)?(?: \\| (?<area>[^|]+))"));
/// java: private static final Pattern SKYHANNI_DIANA_PATTERN = Pattern.compile("A MINOS INQUISITOR has spawned near \\[(?<area>[^]]*)] at Coords (?<x>-?\\d+)(?:\\.\\d*)? (?<y>\\d+)(?:\\.\\d*)? (?<z>-?\\d+)(?:\
pub static SKYHANNI_DIANA_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("A MINOS INQUISITOR has spawned near \\[(?<area>[^]]*)] at Coords (?<x>-?\\d+)(?:\\.\\d*)? (?<y>\\d+)(?:\\.\\d*)? (?<z>-?\\d+)(?:\\.\\d*)?"));
/// java: private static final List<Pattern> PATTERNS = List.of(SKYBLOCKER_COORDS_PATTERN, SKYHANNI_DIANA_PATTERN, GENERIC_COORDS_PATTERN, SIMPLE_COMMA_COORDS_PATTERN, SIMPLE_COORDS_PATTERN);
pub static PATTERNS: LazyLock<Vec<compat::regex::Pattern>> = LazyLock::new(|| vec![SKYBLOCKER_COORDS_PATTERN.clone(), SKYHANNI_DIANA_PATTERN.clone(), GENERIC_COORDS_PATTERN.clone(), SIMPLE_COMMA_COORDS_PATTERN.clone(), SIMPLE_COORDS_PATTERN.clone()]);

/// java: public class ChatPositionShare
#[derive(Clone)]
pub struct ChatPositionShare {}

impl ChatPositionShare {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                dispatcher.register("skyblocker", |ctx| {
                    if ctx.args.first().map(|s| &**s) == Some("sharePosition") {
                        ChatPositionShare::share_player_position(ctx);
                        return Ok(());
                    }
                    #[allow(unreachable_code)]
                    Ok(())
                });
                dispatcher.register("skyblocker", |ctx| {
                    if ctx.args.first().map(|s| &**s) == Some("shareCoords") {
                        ChatPositionShare::share_player_position(ctx);
                        return Ok(());
                    }
                    #[allow(unreachable_code)]
                    Ok(())
                });
            },
        ));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(ChatPositionShare::onMessage);
    }

    /// java: private static int sharePlayerPosition(FabricClientCommandSource source)
    /// (`source` is SeerClient's client command context: `FabricClientCommandSource`'s position is the player's)
    pub fn share_player_position(source: &client_command_registration_callback::ClientCommandContext<'_>) -> i32 {
        // java: Vec3 pos = source.getPosition(); - `client.player.position()`
        let mut pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.position()).expect("NullPointerException");
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("x: {}, y: {}, z: {} | {}", ((pos.x()) as i32), ((pos.y()) as i32), ((pos.z()) as i32), ({ let __r = &Utils::get_area(); let __f = compat::dispatch(__r, |__t| __t.display_name()); __f(__r) })), true);
        return 1;
    }

    /// java: private static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_chat_waypoints {
            let mut message: String = chat_formatting::strip_formatting(&text.get_string());
            if message.starts_with("[Skyblocker]") || message.starts_with("[NPC] Treasure Hunter:") {
                return true;
            }
            for pattern in PATTERNS.iter() {
                let mut matcher: compat::regex::Matcher = pattern.matcher(&message);
                if matcher.find() {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut x: String = matcher.group_named("x").unwrap();
                        let mut y: String = matcher.group_named("y").unwrap();
                        let mut z: String = matcher.group_named("z").unwrap();
                        //   java: String area = matcher.namedGroups().containsKey("area") ? matcher.group("area") : "";
                        // (`group_named` answers `None` for a name the pattern does not have; every pattern here that has
                        // `area` matches it unconditionally)
                        let mut area: String = matcher.group_named("area").unwrap_or_default();
                        ChatPositionShare::request_waypoint(&x, &y, &z, &area);
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("[Skyblocker Chat Waypoints] Error creating chat waypoint: "));
                    }
                    break;
                }
            }
        }
        return true;
    }

    /// java: private static void requestWaypoint(String x, String y, String z, String area)
    pub fn request_waypoint(x: &str, y: &str, z: &str, area: &str) {
        let mut command: String = format!("/skyblocker waypoints individual {} {} {} {}", x, y, z, area);
        let mut request_message: MutableComponent = (constants::PREFIX.get_mut())().append((component::translatable_args("skyblocker.waypoints.chat.display", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(x.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(y.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(z.to_string())])).with_style_format(ChatFormatting::Aqua).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command(compat::trim(&command))))));
        if !area.is_empty() {
            request_message = request_message.append_str(" at ").append(component::literal(area).with_style_format(ChatFormatting::Aqua));
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&request_message.to_string()); });
    }

}
