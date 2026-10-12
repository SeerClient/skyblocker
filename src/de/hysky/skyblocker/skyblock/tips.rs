//! Ported from de.hysky.skyblocker.skyblock.Tips by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::constants;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::click_event::Action;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final String modVersion = SkyblockerMod.VERSION.split("\\+")[0];
pub static MOD_VERSION: LazyLock<String> = LazyLock::new(|| (compat::regex::split(&skyblocker_mod::VERSION, "\\+"))[(0) as usize].clone());
/// java: private static final Component DISABLE_TEXT = Component.translatable("skyblocker.tips.clickDisable").withColor(TextColor.RED).withStyle(style -> style.withClickEvent(new ClickEvent.RunCommand("/skyblo
pub static DISABLE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.tips.clickDisable").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command("/skyblocker tips disable".to_string())))));
/// java: private static final Component ENABLE_TEXT = Component.translatable("skyblocker.tips.clickEnable").withColor(TextColor.GREEN).withStyle(style -> style.withClickEvent(new ClickEvent.RunCommand("/skyblo
pub static ENABLE_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.tips.clickEnable").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command("/skyblocker tips enable".to_string())))));
/// java: private static int currentTipIndex = 0;
pub static CURRENT_TIP_INDEX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
// TODO(fabric2seer): static `ClickEvent.Action` has no Rust counterpart in the port's ClickEvent file
//   java: private static final List<Supplier<Component>> TIPS = new ArrayList<>(List.of(
//   java: 			getTipFactory("skyblocker.tips.customItemNames", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker custom renameItem"),
//   java: 			getTipFactory("skyblocker.tips.customArmorDyeColors", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker custom"),
//   java: 			getTipFactory("skyblocker.tips.customArmorTrims", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker custom"),
//   java: 			getTipFactory("skyblocker.tips.customAnimatedDyes", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker custom"),
//   java: 			getTipFactory("skyblocker.tips.fancyTabExtraInfo"),
//   java: 			getTipFactory("skyblocker.tips.helpCommand", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker help"),
//   java: 			getTipFactory("skyblocker.tips.discordRichPresence", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.customDungeonSecretWaypoints", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker dungeons secrets addWaypoint"),
//   java: 			getTipFactory("skyblocker.tips.shortcuts", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker shortcuts"),
//   java: 			getTipFactory("skyblocker.tips.gallery", ClickEvent.Action.OPEN_URL, "https://hysky.de/skyblocker/gallery"),
//   java: 			getTipFactory("skyblocker.tips.itemRarityBackground", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.modMenuUpdate"),
//   java: 			getTipFactory("skyblocker.tips.issues", ClickEvent.Action.OPEN_URL, "https://github.com/SkyblockerMod/Skyblocker/issues"),
//   java: 			getTipFactory("skyblocker.tips.beta", ClickEvent.Action.OPEN_URL, "https://github.com/SkyblockerMod/Skyblocker/actions"),
//   java: 			getTipFactory("skyblocker.tips.contribute", ClickEvent.Action.OPEN_URL, "https://github.com/SkyblockerMod/Skyblocker/wiki/contribute"),
//   java: 			getTipFactory("skyblocker.tips.discord", ClickEvent.Action.OPEN_URL, "https://discord.gg/aNNJHQykck"),
//   java: 			getTipFactory("skyblocker.tips.flameOverlay", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.wikiLookup", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.protectItem", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker protectItem"),
//   java: 			getTipFactory("skyblocker.tips.fairySoulsEnigmaSoulsRelics", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker fairySouls"),
//   java: 			getTipFactory("skyblocker.tips.quickNav", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.waypoints", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker waypoint"),
//   java: 			getTipFactory("skyblocker.tips.orderedWaypoints", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker waypoints"),
//   java: 			getTipFactory("skyblocker.tips.visitorHelper"),
//   java: 			getTipFactory("skyblocker.tips.slotText"),
//   java: 			getTipFactory("skyblocker.tips.profileViewer", ClickEvent.Action.SUGGEST_COMMAND, "/pv"),
//   java: 			getTipFactory("skyblocker.tips.configSearch", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.configureNewFeatures", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config v" + modVersion, modVersion),
//   java: 			getTipFactory("skyblocker.tips.compactDamage", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.skyblockerScreen", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker"),
//   java: 			getTipFactory("skyblocker.tips.tipsClick", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker tips next"),
//   java: 			getTipFactory("skyblocker.tips.eventNotifications", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.signCalculator"),
//   java: 			getTipFactory("skyblocker.tips.calculateCommand", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker calculate"),
//   java: 			getTipFactory("skyblocker.tips.fancierBars", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker bars"),
//   java: 			getTipFactory("skyblocker.tips.crystalWaypointsShare", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker crystalWaypoints share"),
//   java: 			getTipFactory("skyblocker.tips.gardenMouseLock", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker config"),
//   java: 			getTipFactory("skyblocker.tips.newYearCakesHelper"),
//   java: 			getTipFactory("skyblocker.tips.accessoryHelper"),
//   java: 			getTipFactory("skyblocker.tips.fancyAuctionHouseCheapHighlight"),
//   java: 			getTipFactory("skyblocker.tips.viewRecipe")
//   java: 	));
/// java: private static final List<Supplier<Component>> TIPS = new ArrayList<>(List.of( getTipFactory("skyblocker.tips.customItemNames", ClickEvent.Action.SUGGEST_COMMAND, "/skyblocker custom renameItem"), get
pub static TIPS: compat::MainThread<Vec<Arc<dyn Fn() -> MutableComponent>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TIPS` - see the TODO above"));
/// java: private static boolean sentTip = false;
pub static SENT_TIP: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class Tips
#[derive(Clone)]
pub struct Tips {}

impl Tips {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static Supplier<Component> getTipFactory(@Translatable String key)
    pub fn get_tip_factory(key: &str) -> Arc<dyn Fn() -> MutableComponent> {
        return Arc::new(move || component::translatable(key));
    }

    /// java: private static Supplier<Component> getTipFactory(@Translatable String key, ClickEvent.Action clickAction, String value)
    pub fn get_tip_factory_3(key: &str, click_action: &Action, value: &str) -> Arc<dyn Fn() -> MutableComponent> {
        return Tips::get_tip_factory_4(key, click_action, value, &(0..(0) as usize).map(|_| Default::default()).collect::<Vec<_>>());
    }

    /// java: private static Supplier<Component> getTipFactory(@Translatable String key, ClickEvent.Action clickAction, String value, Object... args)
    pub fn get_tip_factory_4(key: &str, click_action: &Action, value: &str, args: &Vec<compat::Object>) -> Arc<dyn Fn() -> MutableComponent> {
        // TODO(fabric2seer): field `SUGGEST_COMMAND` of a value of unknown type
        //   java: ClickEvent event = switch (clickAction) {
        //   java: 			case ClickEvent.Action.SUGGEST_COMMAND -> new ClickEvent.SuggestCommand(value);
        //   java: 			case ClickEvent.Action.OPEN_URL -> new ClickEvent.OpenUrl(URI.create(value));
        //   java:
        //   java: 			default -> throw new IllegalArgumentException("Unexpected value: " + clickAction);
        //   java: 		};
        let mut event: ClickEvent = todo!("fabric2seer: the initialiser of `event` (TODO above)");
        return Arc::new(move || (component::translatable_args(key, args.clone())).with_style_fn(|style| style.with_click_event(Some(event))));
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("tips")
                //   java: 				.then(literal("enable").executes(Tips::enableTips))
                //   java: 				.then(literal("disable").executes(Tips::disableTips))
                //   java: 				.then(literal("previous").executes(Tips::sendPreviousTipCommand))
                //   java: 				.then(literal("next").executes(Tips::sendNextTipCommand))
                //   java: 		));
            },
        ));
        skyblock_events::JOIN.register(Arc::new(move || { Tips::send_next_tip(); }));
        compat::shuffle(&mut TIPS.get_mut());
    }

    /// java: private static void registerTipsCommand(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_tips_command(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("tips")
        //   java: 				.then(literal("enable").executes(Tips::enableTips))
        //   java: 				.then(literal("disable").executes(Tips::disableTips))
        //   java: 				.then(literal("previous").executes(Tips::sendPreviousTipCommand))
        //   java: 				.then(literal("next").executes(Tips::sendNextTipCommand))
        //   java: 		));
    }

    /// java: private static int enableTips(CommandContext<FabricClientCommandSource> context)
    pub fn enable_tips(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        SkyblockerConfigManager::update(Arc::new(move |config| { config.general.enable_tips = true; }));
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.tips.enabled").withColor(TextColor.GREEN)).append(" ").append(DISABLE_TEXT));
        return 1;
    }

    /// java: private static int disableTips(CommandContext<FabricClientCommandSource> context)
    pub fn disable_tips(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        SkyblockerConfigManager::update(Arc::new(move |config| { config.general.enable_tips = false; }));
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.tips.disabled").withColor(TextColor.RED)).append(" ").append(ENABLE_TEXT));
        return 1;
    }

    /// java: private static void sendNextTip()
    pub fn send_next_tip() {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && SkyblockerConfigManager::get().general.enable_tips && !(compat::get_state(&SENT_TIP)) {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&Tips::tip_message(&Tips::next_tip()).to_string()); });
            compat::set_state(&SENT_TIP, true);
        }
    }

    /// java: private static int sendNextTipCommand(CommandContext<FabricClientCommandSource> context)
    pub fn send_next_tip_command(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(tipMessage(nextTip()));
        return 1;
    }

    /// java: public static Component nextTip()
    pub fn next_tip() -> MutableComponent {
        return component::translatable_args("skyblocker.tips.tip", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Tips::next_tip_internal())]);
    }

    /// java: private static Component nextTipInternal()
    pub fn next_tip_internal() -> MutableComponent {
        compat::set_state(&CURRENT_TIP_INDEX, compat::get_state(&CURRENT_TIP_INDEX) + 1);
        compat::set_state(&CURRENT_TIP_INDEX, (compat::get_state(&CURRENT_TIP_INDEX)).wrapping_rem(TIPS.get_mut().len() as i32));
        return ((TIPS.get_mut()[(compat::get_state(&CURRENT_TIP_INDEX)) as usize]))();
    }

    /// java: private static int sendPreviousTipCommand(CommandContext<FabricClientCommandSource> context)
    pub fn send_previous_tip_command(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(tipMessage(previousTip()));
        return 1;
    }

    /// java: public static Component previousTip()
    pub fn previous_tip() -> MutableComponent {
        return component::translatable_args("skyblocker.tips.tip", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Tips::previous_tip_internal())]);
    }

    /// java: private static Component previousTipInternal()
    pub fn previous_tip_internal() -> MutableComponent {
        compat::set_state(&CURRENT_TIP_INDEX, compat::get_state(&CURRENT_TIP_INDEX) - 1);
        compat::set_state(&CURRENT_TIP_INDEX, (compat::get_state(&CURRENT_TIP_INDEX)).wrapping_add(TIPS.get_mut().len() as i32));
        compat::set_state(&CURRENT_TIP_INDEX, (compat::get_state(&CURRENT_TIP_INDEX)).wrapping_rem(TIPS.get_mut().len() as i32));
        return ((TIPS.get_mut()[(compat::get_state(&CURRENT_TIP_INDEX)) as usize]))();
    }

    /// java: private static Component tipMessage(Component tip)
    pub fn tip_message(tip: &MutableComponent) -> MutableComponent {
        return (constants::PREFIX.get_mut())().append(tip.clone()).append_str(" ").append(component::translatable("skyblocker.tips.clickPreviousTip").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command("/skyblocker tips previous".to_string()))))).append_str(" ").append(component::translatable("skyblocker.tips.clickNextTip").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command("/skyblocker tips next".to_string()))))).append_str(" ").append(DISABLE_TEXT.get_mut().clone());
    }

}
