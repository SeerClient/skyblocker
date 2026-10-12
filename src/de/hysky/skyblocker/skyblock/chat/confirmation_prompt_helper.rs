//! Ported from de.hysky.skyblocker.skyblock.chat.ConfirmationPromptHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::screens::chat_screen::ChatScreen;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final List<String> CONFIRMATION_PHRASES = List.of( "[Aye sure do!]", // [NPC] Carnival Pirateman "[You guessed it!]", // [NPC] Carnival Fisherman "[Sure thing, partner!]", // [NPC] Carn
pub static CONFIRMATION_PHRASES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["[Aye sure do!]".to_string(), "[You guessed it!]".to_string(), "[Sure thing, partner!]".to_string(), "YES".to_string(), "Yes".to_string()]);
/// java: private static final List<String> CONFIRMATION_PHRASES_FORMATTING = List.of( "§e ➜ §a[Aye sure do!]", // [NPC] Carnival Pirateman "§e ➜ §a[You guessed it!]", // [NPC] Carnival Fisherman "§e ➜ §a[Sure 
pub static CONFIRMATION_PHRASES_FORMATTING: LazyLock<Vec<String>> = LazyLock::new(|| vec!["§e ➜ §a[Aye sure do!]".to_string(), "§e ➜ §a[You guessed it!]".to_string(), "§e ➜ §a[Sure thing, partner!]".to_string(), "§a§l[YES]".to_string(), "§a[Yes]".to_string()]);
/// java: private static String command;
pub static COMMAND: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static long commandFoundAt;
pub static COMMAND_FOUND_AT: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));

/// java: public class ConfirmationPromptHelper
#[derive(Clone)]
pub struct ConfirmationPromptHelper {}

impl ConfirmationPromptHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(ConfirmationPromptHelper::onMessage);
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            if Utils::is_on_skyblock() && screen.as_any().is::<ChatScreen>() && SkyblockerConfigManager::get().chat.confirmation_prompt_helper {
                // TODO(fabric2seer): ScreenMouseEvents.beforeMouseClick(..).register: no SeerClient equivalent yet
                //   java: ScreenMouseEvents.beforeMouseClick(screen).register((_, click) -> {
                //   java: 					if (hasCommand()) {
                //   java: 						Minecraft client = Minecraft.getInstance();
                //   java: 						if (client.gui.screen() instanceof ChatScreen) {	// Ignore clicks on other interactive elements
                //   java: 							ActiveTextCollector.ClickableStyleFinder clickHandler = new ActiveTextCollector.ClickableStyleFinder(screen.getFont(), (int) click.x(), (int) click.y())
                //   java: 									.includeInsertions(false);
                //   java: 							Style clickedStyle = clickHandler.result();
                //   java:
                //   java: 							if (clickedStyle != null && clickedStyle.getClickEvent() != null) {	// clicking on some prompts invalidates first prompt but not in all cases, so I decided not to nullify command
                //   java: 								return;
                //   java: 							}
                //   java: 						}
                //   java:
                //   java: 						MessageScheduler.INSTANCE.sendMessageAfterCooldown(command, true);
                //   java: 						command = null;
                //   java: 						commandFoundAt = 0;
                //   java: 					}
                //   java: 				});
            }
        }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            compat::set_state(&COMMAND, None);
            compat::set_state(&COMMAND_FOUND_AT, ((0) as i64) as i64);
        }));
    }

    /// java: private static boolean hasCommand()
    pub fn has_command() -> bool {
        return (compat::get_state(&COMMAND).is_some() && (compat::get_state(&COMMAND_FOUND_AT)).wrapping_add(60000 as i64) > compat::millis());
    }

    /// java: private static boolean containsConfirmationPhrase(Component message)
    pub fn contains_confirmation_phrase(message: &MutableComponent) -> bool {
        let mut message_str: String = message.get_string();
        for phrase in CONFIRMATION_PHRASES.iter() {
            if message_str.contains(phrase) {
                return true;
            }
        }
        return false;
    }

    /// java: private static boolean onMessage(Component message, boolean overlay)
    pub fn on_message(message: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && !overlay && SkyblockerConfigManager::get().chat.confirmation_prompt_helper && ConfirmationPromptHelper::contains_confirmation_phrase(message) {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.network.chat.FormattedText.StyledContentConsumer<T>`)
            // rust: let mut confirmation_command: Option<String> = message.visit_styled(|style, as_string| {
            // rust: let mut event: ClickEvent = style.get_click_event().unwrap();
            // rust: as_string = compat::trim(&(compat::regex::replace_all(&as_string, "\\s+", " ")));
            // rust: // TODO(fabric2seer): not translated (expected ')' at '(' (offset 4257))
            // rust: //   java: if (CONFIRMATION_PHRASES_FORMATTING.contains(asString) && event instanceof ClickEvent.RunCommand(String command) && (command.startsWith("/chatprompt") || command.startsWith("/selectnpcoption"))) {
            // rust: //   java: 					return Optional.of(command);
            // rust: //   java: 				}
            // rust: return None;
            // rust: }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
            let mut confirmation_command: Option<String> = todo!("fabric2seer: the initialiser of `confirmationCommand` (TODO above)");
            if confirmation_command.is_some() {
                compat::set_state(&COMMAND, Some(confirmation_command.clone().unwrap()));
                compat::set_state(&COMMAND_FOUND_AT, compat::millis());
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.chat.confirmationPromptNotification")).to_string()); });
            }
        }
        return true;
    }

}
