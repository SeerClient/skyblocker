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
/// Whether the open chat screen's AFTER_INIT registered the `beforeMouseClick` listener (see `init`).
static CHAT_SCREEN_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

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
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(ConfirmationPromptHelper::onMessage);
        compat::register_allow_game(|message, overlay| ConfirmationPromptHelper::on_message(message, overlay));
        // `ScreenMouseEvents.beforeMouseClick(screen)` is a per-screen event in Fabric; the port's
        // `screen_mouse_events::ALLOW_MOUSE_CLICK` is one global event (the HEAD of `MouseHandler#onButton`), so the
        // listener is registered once and asks what the per-screen registration encoded: that the open screen is the
        // chat screen whose AFTER_INIT registered it (`CHAT_SCREEN_REGISTERED`).
        screen_mouse_events::ALLOW_MOUSE_CLICK.register(Arc::new(|_screen: &mut Screen, click: (i32, i32), pressed: bool| {
            if !pressed || !CHAT_SCREEN_REGISTERED.load(std::sync::atomic::Ordering::Relaxed) || !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Chat) {
                return true;
            }
            ConfirmationPromptHelper::before_mouse_click(click);
            true
        }));
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            //Don't check for the command being present in case the user opens the chat before the prompt is sent
            let registered: bool = Utils::is_on_skyblock() && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Chat) && SkyblockerConfigManager::get().chat.confirmation_prompt_helper;
            CHAT_SCREEN_REGISTERED.store(registered, std::sync::atomic::Ordering::Relaxed);
            if registered {
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

    /// The `ScreenMouseEvents.beforeMouseClick` listener `init` registers on a chat screen (`click` in GUI pixels).
    fn before_mouse_click(click: (i32, i32)) {
        if ConfirmationPromptHelper::has_command() {
            // java: if (client.gui.screen() instanceof ChatScreen) {	// Ignore clicks on other interactive elements
            if matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Chat) {
                // java: ActiveTextCollector.ClickableStyleFinder clickHandler = new ActiveTextCollector.ClickableStyleFinder(screen.getFont(), (int) click.x(), (int) click.y()).includeInsertions(false);
                // java: Style clickedStyle = clickHandler.result();
                // (the finder is asked for its result without being handed any text, so it answers its initial null)
                let click_handler = mcrust::net::minecraft::client::gui::active_text_collector::ClickableStyleFinder::new(click.0 as f64, click.1 as f64).include_insertions(false);
                let clicked_style: Option<mcrust::net::minecraft::network::chat::style::Style> = click_handler.result().cloned();
                // clicking on some prompts invalidates first prompt but not in all cases, so I decided not to nullify command
                if clicked_style.as_ref().is_some_and(|style| style.get_click_event().is_some()) {
                    return;
                }
            }
            // java: MessageScheduler.INSTANCE.sendMessageAfterCooldown(command, true); command = null; commandFoundAt = 0;
            let command: String = compat::get_state(&COMMAND).expect("hasCommand");
            crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler::send_message_after_cooldown(crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::INSTANCE.get_mut(), &command, true);
            compat::set_state(&COMMAND, None);
            compat::set_state(&COMMAND_FOUND_AT, 0);
        }
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
            //   java: Optional<String> confirmationCommand = message.visit((style, asString) -> {
            //   java: 	ClickEvent event = style.getClickEvent();
            //   java: 	asString = asString.replaceAll("\\s+", " ").trim();
            //   java: 	if (CONFIRMATION_PHRASES_FORMATTING.contains(asString) && event instanceof ClickEvent.RunCommand(String command) && (command.startsWith("/chatprompt") || command.startsWith("/selectnpcoption"))) {
            //   java: 		return Optional.of(command);
            //   java: 	}
            //   java: 	return Optional.empty();
            //   java: }, Style.EMPTY);
            // (the port's consumer answers whether to go on: the first `Optional.of` is kept and stops the walk)
            let mut confirmation_command: Option<String> = None;
            message.visit_styled(&mut |style: &mcrust::net::minecraft::network::chat::style::Style, as_string: &str| {
                let event: Option<&ClickEvent> = style.get_click_event();
                let as_string: String = compat::trim(&(compat::regex::replace_all(as_string, "\\s+", " ")));
                if CONFIRMATION_PHRASES_FORMATTING.contains(&as_string) && let Some(ClickEvent::RunCommand { command }) = event && (command.starts_with("/chatprompt") || command.starts_with("/selectnpcoption")) {
                    confirmation_command = Some(command.clone());
                    return false;
                }
                true
            }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
            if confirmation_command.is_some() {
                compat::set_state(&COMMAND, Some(confirmation_command.clone().unwrap()));
                compat::set_state(&COMMAND_FOUND_AT, compat::millis());
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.chat.confirmationPromptNotification")).to_string()); });
            }
        }
        return true;
    }

}
