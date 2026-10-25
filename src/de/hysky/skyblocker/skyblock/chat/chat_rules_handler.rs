//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRulesHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer::ConfigDataFixer;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::AnnouncementMessage;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ChatRule;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::Match;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ToastMessage;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::basic_toast::BasicToast;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Path CHAT_RULE_FILE = SkyblockerMod.CONFIG_DIR.resolve("chat_rules.json");
pub static CHAT_RULE_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("chat_rules.json"));
/// java: @VisibleForTesting static final Codec<List<ChatRule>> UNBOXING_CODEC = ConfigDataFixer.createDataFixingCodec(ConfigDataFixer.CHAT_RULES_TYPE, CodecUtils.mutableOptional(ChatRule.LIST_CODEC.fieldOf("ru
pub static UNBOXING_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<ChatRule>>>> = compat::MainThread::new(|| ConfigDataFixer::create_data_fixing_codec(config_data_fixer::CHAT_RULES_TYPE.get_mut(), &map_codec::codec(CodecUtils::mutable_optional(&codec::field_of(chat_rule::LIST_CODEC.get_mut().clone(), "rules"), Arc::new(move |__a0| __a0.iter().cloned().collect::<Vec<_>>())))));
/// java: protected static final JsonData<List<ChatRule>> CHAT_RULE_LIST = new JsonData<>(CHAT_RULE_FILE, UNBOXING_CODEC, getDefaultChatRules());
pub static CHAT_RULE_LIST: compat::MainThread<compat::Shared<dyn JsonData<Vec<compat::Shared<ChatRule>>>>> = compat::MainThread::new(|| (compat::shared(JsonDataBase::new(CHAT_RULE_FILE.get_mut().clone(), UNBOXING_CODEC.get_mut().clone(), ChatRulesHandler::get_default_chat_rules())) as compat::Shared<dyn JsonData<Vec<compat::Shared<ChatRule>>>>));

/// java: public class ChatRulesHandler
#[derive(Clone)]
pub struct ChatRulesHandler {}

impl ChatRulesHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> CHAT_RULE_LIST.init());
        // (CLIENT_STARTED fires once the `Minecraft` constructor - where the entrypoints run - has returned, before the
        // first client tick: `compat::execute` from an entrypoint)
        compat::execute(|| {
            let __r = CHAT_RULE_LIST.get().clone();
            let _ = __r.lock().borrow_mut().init();
        });
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(ChatRulesHandler::checkMessage);
        compat::register_allow_game(|message, overlay| ChatRulesHandler::check_message(message, overlay));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
                //   java: 		.then(ClientCommands.literal("chatRules").executes(Scheduler.queueOpenScreenCommand(() -> new ChatRulesConfigScreen(null)))));
                // (`queueOpenScreenCommand`: the screen is built and set in a task the client runs next)
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut chat_rules = client_command_registration_callback::literal("chatRules");
                chat_rules.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    mcrust::net::minecraft::client::minecraft::execute(Box::new(|_minecraft| {
                        mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::chat::chat_rules_config_screen::ChatRulesConfigScreen::new(None))));
                    }));
                    Ok(1)
                }));
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&chat_rules);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: @VisibleForTesting static List<ChatRule> getDefaultChatRules()
    pub fn get_default_chat_rules() -> Vec<compat::Shared<ChatRule>> {
        return vec![ChatRule::new_14("Clean Hub Chat".to_string(), false, true, true, true, false, "(selling)|(buying)|(lowb)|(visit)|(/p)|(/ah)|(my ah)".to_string(), std::collections::HashSet::from([Location::Hub]), true, None, None, None, None, None), ChatRule::new_14("Mining Ability Alert".to_string(), false, true, false, true, false, "is now available!".to_string(), std::collections::HashSet::from([Location::DwarvenMines, Location::CrystalHollows]), false, Some("&1Ability".to_string()), None, Some(AnnouncementMessage::new("&1Ability".to_string(), ((3000) as i64) as i64)), None, Some(mcrust::net::minecraft::sounds::sound_events::arrow_hit_player().clone()))].into_iter().collect::<Vec<_>>();
    }

    /// java: private static boolean checkMessage(Component message, boolean overlay)
    pub fn check_message(message: &MutableComponent, mut overlay: bool) -> bool {
        if overlay || !(Utils::is_on_skyblock()) {
            return true;
        }
        let mut rules: Vec<compat::Shared<ChatRule>> = CHAT_RULE_LIST.get_mut().lock().borrow().get_data();
        if !(CHAT_RULE_LIST.get_mut().lock().borrow().is_loaded()) || rules.is_empty() {
            return true;
        }
        for rule in rules.iter() {
            let mut r#match: Match = ChatRule::is_match(rule, &(if ChatRule::get_include_formatting(rule) { TextTransformer::to_legacy(message) } else { chat_formatting::strip_formatting(&message.get_string()) }));
            if !((r#match).matches()) {
                continue;
            }
            let mut send_original: bool = !(ChatRule::get_hide_message(rule));
            if send_original && ChatRule::get_chat_message(rule).is_some() {
                send_original = false;
                Utils::send_message_to_bypass_events(&ChatRulesHandler::format_text(&(r#match).insert_capture_groups(ChatRule::get_chat_message(rule).as_deref().unwrap())));
            }
            if ChatRule::get_announcement_message(rule).is_some() {
                let mut announcement_message: compat::Shared<AnnouncementMessage> = ChatRule::get_announcement_message(rule).unwrap();
                TitleContainer::add_title_2(&Title::new_1(ChatRulesHandler::format_text(&(r#match).insert_capture_groups(&compat::read(&announcement_message, |__t| __t.message.clone())))), (((compat::read(&announcement_message, |__t| __t.display_duration)).wrapping_div(50 as i64)) as i32));
            }
            if ChatRule::get_action_bar_message(rule).is_some() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                // java: CLIENT.player.sendOverlayMessage(..) - `LocalPlayer.sendOverlayMessage` is `minecraft.gui.hud.setOverlayMessage(message, false)`
                let overlay_text = ChatRulesHandler::format_text(&(r#match).insert_capture_groups(ChatRule::get_action_bar_message(rule).as_deref().unwrap()));
                mcrust::net::minecraft::client::gui::hud::set_overlay_message(&mut compat::minecraft().gui.hud, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(overlay_text), false);
            }
            if ChatRule::get_toast_message(rule).is_some() {
                let mut toast_message: compat::Shared<ToastMessage> = ChatRule::get_toast_message(rule).unwrap();
                compat::minecraft().gui.toast_manager().borrow_mut().add_toast(Box::new(BasicToast::new_3(ChatRulesHandler::format_text(&(r#match).insert_capture_groups(&compat::read(&toast_message, |__t| __t.message.clone()))), compat::read(&toast_message, |__t| __t.display_duration), Some(compat::read(&toast_message, |__t| __t.icon.clone()).and_then(|__a0| FlexibleItemStack::get_stack(&__a0)).unwrap_or(mcrust::net::minecraft::world::item::item_stack::empty())))));
            }
            if ChatRule::get_custom_sound(rule).is_some() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(ChatRule::get_custom_sound(rule).unwrap(), 100.0f32, 0.1f32); });
            }
            if !send_original {
                return false;
            }
        }
        return true;
    }

    /// java: protected static MutableComponent formatText(String codedString)
    pub fn format_text(coded_string: &str) -> MutableComponent {
        if coded_string.contains("§") {
            return TextTransformer::from_legacy_3(coded_string, '§', false);
        }
        if coded_string.contains("&") {
            return TextTransformer::from_legacy_3(coded_string, '&', false);
        }
        return component::literal(coded_string);
    }

    /// java: public static void saveChatRules()
    pub fn save_chat_rules() {
        if CHAT_RULE_LIST.get_mut().lock().borrow().is_loaded() {
            CHAT_RULE_LIST.get_mut().lock().borrow().save();
        }
    }

}
