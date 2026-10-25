//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.CorpseProfitTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::item_price_update_event;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseType;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker::AbstractProfitTracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker::AbstractProfitTrackerBase;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_loot;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_loot::CorpseLoot;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward::Reward;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final String GLACITE_POWDER = "GLACITE_POWDER";
pub const GLACITE_POWDER: &str = "GLACITE_POWDER";
/// java: public static final String OPAL_CRYSTAL = "OPAL_CRYSTAL";
pub const OPAL_CRYSTAL: &str = "OPAL_CRYSTAL";
/// java: public static final String ONYX_CRYSTAL = "ONYX_CRYSTAL";
pub const ONYX_CRYSTAL: &str = "ONYX_CRYSTAL";
/// java: public static final String AQUAMARINE_CRYSTAL = "AQUAMARINE_CRYSTAL";
pub const AQUAMARINE_CRYSTAL: &str = "AQUAMARINE_CRYSTAL";
/// java: public static final String PERIDOT_CRYSTAL = "PERIDOT_CRYSTAL";
pub const PERIDOT_CRYSTAL: &str = "PERIDOT_CRYSTAL";
/// java: public static final String CITRINE_CRYSTAL = "CITRINE_CRYSTAL";
pub const CITRINE_CRYSTAL: &str = "CITRINE_CRYSTAL";
/// java: public static final String RUBY_CRYSTAL = "RUBY_CRYSTAL";
pub const RUBY_CRYSTAL: &str = "RUBY_CRYSTAL";
/// java: public static final String JASPER_CRYSTAL = "JASPER_CRYSTAL";
pub const JASPER_CRYSTAL: &str = "JASPER_CRYSTAL";
/// java: public static final String ENCHANTMENT_ICE_COLD_1 = "ENCHANTMENT_ICE_COLD_1";
pub const ENCHANTMENT_ICE_COLD_1: &str = "ENCHANTMENT_ICE_COLD_1";
/// java: public static final @Unmodifiable List<String> PRICELESS_ITEMS = List.of(GLACITE_POWDER, OPAL_CRYSTAL, ONYX_CRYSTAL, AQUAMARINE_CRYSTAL, PERIDOT_CRYSTAL, CITRINE_CRYSTAL, RUBY_CRYSTAL, JASPER_CRYSTAL)
pub static PRICELESS_ITEMS: LazyLock<Vec<String>> = LazyLock::new(|| vec![GLACITE_POWDER.to_string(), OPAL_CRYSTAL.to_string(), ONYX_CRYSTAL.to_string(), AQUAMARINE_CRYSTAL.to_string(), PERIDOT_CRYSTAL.to_string(), CITRINE_CRYSTAL.to_string(), RUBY_CRYSTAL.to_string(), JASPER_CRYSTAL.to_string()]);
/// java: public static final String CORPSE_PROFIT_MESSAGE = "Corpse Profit: %s";
pub const CORPSE_PROFIT_MESSAGE: &str = "Corpse Profit: %s";
/// java: public static final CorpseProfitTracker INSTANCE = new CorpseProfitTracker();
pub static INSTANCE: compat::MainThread<compat::Shared<CorpseProfitTracker>> = compat::MainThread::new(|| CorpseProfitTracker::new());
/// java: private static final Pattern CORPSE_PATTERN = Pattern.compile(" {2}(LAPIS|UMBER|TUNGSTEN|VANGUARD) CORPSE LOOT! *");
pub static CORPSE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" {2}(LAPIS|UMBER|TUNGSTEN|VANGUARD) CORPSE LOOT! *"));
/// java: private static final Object2ObjectArrayMap<String, String> NAME2ID_MAP = new Object2ObjectArrayMap<>(50);
/// An `Object2ObjectArrayMap` keeps insertion order (its iteration order is read: `comparePriority`, the filter
/// screen's item list), so it is an `IndexMap`.
pub static NAME2ID_MAP: LazyLock<indexmap::IndexMap<String, String>> = LazyLock::new(|| {
    let mut value = indexmap::IndexMap::with_capacity((50) as usize);
    value.insert(format!("{} Flawed Onyx Gemstone", sky_block_icons::CRIT_DAMAGE), "FLAWED_ONYX_GEM".to_string());
    value.insert(format!("{} Fine Onyx Gemstone", sky_block_icons::CRIT_DAMAGE), "FINE_ONYX_GEM".to_string());
    value.insert(format!("{} Flawless Onyx Gemstone", sky_block_icons::CRIT_DAMAGE), "FLAWLESS_ONYX_GEM".to_string());
    value.insert(format!("{} Flawed Peridot Gemstone", sky_block_icons::FARMING_FORTUNE), "FLAWED_PERIDOT_GEM".to_string());
    value.insert(format!("{} Fine Peridot Gemstone", sky_block_icons::FARMING_FORTUNE), "FINE_PERIDOT_GEM".to_string());
    value.insert(format!("{} Flawless Peridot Gemstone", sky_block_icons::FARMING_FORTUNE), "FLAWLESS_PERIDOT_GEM".to_string());
    value.insert(format!("{} Flawed Citrine Gemstone", sky_block_icons::FORAGING_FORTUNE), "FLAWED_CITRINE_GEM".to_string());
    value.insert(format!("{} Fine Citrine Gemstone", sky_block_icons::FORAGING_FORTUNE), "FINE_CITRINE_GEM".to_string());
    value.insert(format!("{} Flawless Citrine Gemstone", sky_block_icons::FORAGING_FORTUNE), "FLAWLESS_CITRINE_GEM".to_string());
    value.insert(format!("{} Flawed Aquamarine Gemstone", sky_block_icons::FISHING_SPEED), "FLAWED_AQUAMARINE_GEM".to_string());
    value.insert(format!("{} Fine Aquamarine Gemstone", sky_block_icons::FISHING_SPEED), "FINE_AQUAMARINE_GEM".to_string());
    value.insert(format!("{} Flawless Aquamarine Gemstone", sky_block_icons::FISHING_SPEED), "FLAWLESS_AQUAMARINE_GEM".to_string());
    value.insert("Goblin Egg".to_string(), "GOBLIN_EGG".to_string());
    value.insert("Green Goblin Egg".to_string(), "GOBLIN_EGG_GREEN".to_string());
    value.insert("Blue Goblin Egg".to_string(), "GOBLIN_EGG_BLUE".to_string());
    value.insert("Red Goblin Egg".to_string(), "GOBLIN_EGG_RED".to_string());
    value.insert("Yellow Goblin Egg".to_string(), "GOBLIN_EGG_YELLOW".to_string());
    value.insert("Enchanted Glacite".to_string(), "ENCHANTED_GLACITE".to_string());
    value.insert("Enchanted Umber".to_string(), "ENCHANTED_UMBER".to_string());
    value.insert("Enchanted Tungsten".to_string(), "ENCHANTED_TUNGSTEN".to_string());
    value.insert("Refined Umber".to_string(), "REFINED_UMBER".to_string());
    value.insert("Refined Tungsten".to_string(), "REFINED_TUNGSTEN".to_string());
    value.insert("Refined Mithril".to_string(), "REFINED_MITHRIL".to_string());
    value.insert("Refined Titanium".to_string(), "REFINED_TITANIUM".to_string());
    value.insert("Mithril Plate".to_string(), "MITHRIL_PLATE".to_string());
    value.insert("Umber Plate".to_string(), "UMBER_PLATE".to_string());
    value.insert("Tungsten Plate".to_string(), "TUNGSTEN_PLATE".to_string());
    value.insert("Skeleton Key".to_string(), "SKELETON_KEY".to_string());
    value.insert("Tungsten Key".to_string(), "TUNGSTEN_KEY".to_string());
    value.insert("Umber Key".to_string(), "UMBER_KEY".to_string());
    value.insert("Glacite Amalgamation".to_string(), "GLACITE_AMALGAMATION".to_string());
    value.insert("Bejeweled Handle".to_string(), "BEJEWELED_HANDLE".to_string());
    value.insert("Glacite Jewel".to_string(), "GLACITE_JEWEL".to_string());
    value.insert("Suspicious Scrap".to_string(), "SUSPICIOUS_SCRAP".to_string());
    value.insert("Enchanted Book (Ice Cold I)".to_string(), "ENCHANTMENT_ICE_COLD_1".to_string());
    value.insert("Dwarven O's Metallic Minis".to_string(), "DWARVEN_OS_METALLIC_MINIS".to_string());
    value.insert("Frozen Scute".to_string(), "FROZEN_SCUTE".to_string());
    value.insert("Shattered Locket".to_string(), "SHATTERED_PENDANT".to_string());
    value.insert("Caged Wisp".to_string(), "CAGED_WISP".to_string());
    value.insert("Frostbitten Dye".to_string(), "DYE_FROSTBITTEN".to_string());
    value.insert("Opal Crystal".to_string(), OPAL_CRYSTAL.to_string());
    value.insert("Onyx Crystal".to_string(), ONYX_CRYSTAL.to_string());
    value.insert("Aquamarine Crystal".to_string(), AQUAMARINE_CRYSTAL.to_string());
    value.insert("Peridot Crystal".to_string(), PERIDOT_CRYSTAL.to_string());
    value.insert("Citrine Crystal".to_string(), CITRINE_CRYSTAL.to_string());
    value.insert("Ruby Crystal".to_string(), RUBY_CRYSTAL.to_string());
    value.insert("Jasper Crystal".to_string(), JASPER_CRYSTAL.to_string());
    value.insert("Glacite Powder".to_string(), GLACITE_POWDER.to_string());
    value
});

/// java: public final class CorpseProfitTracker extends AbstractProfitTracker
#[derive(Clone)]
pub struct CorpseProfitTracker {
    /// java: extends AbstractProfitTracker
    pub base: AbstractProfitTrackerBase,
    /// java: private ObjectArrayList<CorpseLoot> currentProfileRewards = new ObjectArrayList<>();
    pub current_profile_rewards: Vec<compat::Shared<CorpseLoot>>,
    /// java: private final ProfiledData<ObjectArrayList<CorpseLoot>> allRewards = new ProfiledData<>(getRewardFilePath("corpse-profits.json"), CorpseLoot.CODEC.listOf().xmap
    pub all_rewards: Option<compat::Shared<ProfiledData<Vec<compat::Shared<CorpseLoot>>>>>,
    /// java: private boolean insideRewardMessage = false;
    pub inside_reward_message: bool,
    /// java: private @Nullable CorpseLoot lastCorpseLoot = null;
    pub last_corpse_loot: Option<compat::Shared<CorpseLoot>>,
}

impl std::ops::Deref for CorpseProfitTracker {
    type Target = AbstractProfitTrackerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CorpseProfitTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CorpseProfitTracker {
    /// java: private CorpseProfitTracker()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractProfitTrackerBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                current_profile_rewards: Vec::new(),
                all_rewards: None,
                inside_reward_message: false,
                last_corpse_loot: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractProfitTracker>), __h)
        });
        // java: private final ProfiledData<ObjectArrayList<CorpseLoot>> allRewards = new ProfiledData<>(getRewardFilePath("corpse-profits.json"), CorpseLoot.CODEC.listOf().xmap(ObjectArrayList::new, Function.identity()));
        // (`xmap(ObjectArrayList::new, Function.identity())` is the list itself here; `getRewardFilePath` reads `this`, so
        // the field is written once the object exists)
        {
            let __file = ({ let __f = compat::dispatch(&__this, |__t| __t.get_reward_file_path()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractProfitTracker>), "corpse-profits.json") });
            let __v = Some(ProfiledData::new(__file, codec::list(corpse_loot::CODEC.get_mut().clone())));
            compat::write(&__this, |__t| __t.all_rewards = __v);
        }
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(INSTANCE::onChatMessage);
        compat::register_allow_game(|text: &MutableComponent, overlay: bool| CorpseProfitTracker::on_chat_message(INSTANCE.get_mut(), text, overlay));
        // java: INSTANCE.allRewards.init();
        let all_rewards = compat::read(INSTANCE.get(), |__t| __t.all_rewards.clone()).expect("allRewards is built by the constructor");
        compat::write(&all_rewards, |__t| { __t.init(); });
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |__a0, __a1| { CorpseProfitTracker::on_profile_change(INSTANCE.get_mut(), &__a0, &__a1); }));
        // java: ClientCommandRegistrationCallback.EVENT.register((dispatcher, _) -> dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   .then(literal("rewardTrackers").then(literal("corpse")
        //     .then(literal("list")
        //       .then(argument("summaryView", BoolArgumentType.bool()).executes(ctx -> { Scheduler.queueOpenScreen(new CorpseProfitScreen(ctx.getSource().getClient().gui.screen(), BoolArgumentType.getBool(ctx, "summaryView"))); return Command.SINGLE_SUCCESS; }))
        //       .executes(ctx -> { Scheduler.queueOpenScreen(new CorpseProfitScreen(ctx.getSource().getClient().gui.screen())); return Command.SINGLE_SUCCESS; }))
        //     .then(literal("reset").executes(ctx -> { INSTANCE.currentProfileRewards.clear(); INSTANCE.allRewards.save();
        //       ctx.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.corpseTracker.historyReset").withStyle(ChatFormatting.GREEN))); return Command.SINGLE_SUCCESS; }))))));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
                use mcrust::com::mojang::brigadier::arguments::bool_argument_type::BoolArgumentType;
                type Source = client_command_registration_callback::FabricClientCommandSource;
                // (the screen is the game's to open: its parent is the screen it is opened over, which the game holds)
                let mut summary_view = client_command_registration_callback::argument("summaryView", Arc::new(BoolArgumentType::bool()));
                summary_view.executes(Arc::new(|ctx: &CommandContext<Source>| {
                    let summary_view: bool = BoolArgumentType::get_bool(ctx, "summaryView");
                    crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen(Arc::new(move || -> Box<dyn mcrust::mcrust::modding::screens::ModScreen> { Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_screen::CorpseProfitScreen::new_2(None, summary_view))) }));
                    Ok(1)
                }));
                let mut list = client_command_registration_callback::literal("list");
                list.then(&summary_view);
                list.executes(Arc::new(|_: &CommandContext<Source>| {
                    crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen(Arc::new(|| -> Box<dyn mcrust::mcrust::modding::screens::ModScreen> { Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_screen::CorpseProfitScreen::new(None))) }));
                    Ok(1)
                }));
                let mut reset = client_command_registration_callback::literal("reset");
                reset.executes(Arc::new(|_: &CommandContext<Source>| {
                    compat::write(INSTANCE.get(), |__t| __t.current_profile_rewards.clear());
                    let all_rewards = compat::read(INSTANCE.get(), |__t| __t.all_rewards.clone());
                    if let Some(all_rewards) = all_rewards {
                        compat::read(&all_rewards, |__t| __t.save());
                    }
                    compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.corpseTracker.historyReset").with_style_format(ChatFormatting::Green)));
                    Ok(1)
                }));
                let mut corpse = client_command_registration_callback::literal("corpse");
                corpse.then(&list);
                corpse.then(&reset);
                let mut reward_trackers = client_command_registration_callback::literal("rewardTrackers");
                reward_trackers.then(&corpse);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&reward_trackers);
                dispatcher.dispatcher().register(&root);
            },
        ));
        item_price_update_event::ON_PRICE_UPDATE.register(Arc::new(move || { CorpseProfitTracker::recalculate_all(INSTANCE.get_mut()); }));
    }

    /// java: @UnmodifiableView public static List<CorpseLoot> getCurrentProfileRewards()
    pub fn get_current_profile_rewards() -> Vec<compat::Shared<CorpseLoot>> {
        // java: return ObjectLists.unmodifiable(INSTANCE.currentProfileRewards); - the rewards, read-only (the entries are
        // the tracker's own objects)
        return compat::read(INSTANCE.get(), |__t| __t.current_profile_rewards.clone());
    }

    /// java: @UnmodifiableView public static Object2ObjectMap<String, String> getName2IdMap()
    pub fn get_name2_id_map() -> &'static indexmap::IndexMap<String, String> {
        // `Object2ObjectMaps.unmodifiable(NAME2ID_MAP)`: a read-only view of the map.
        return &NAME2ID_MAP;
    }

    /// java: private void onProfileChange(String prevProfileId, String newProfileId)
    pub fn on_profile_change(__this: &compat::Shared<Self>, prev_profile_id: &str, new_profile_id: &str) {
        if !(CorpseProfitTracker::is_enabled(__this)) {
            return;
        }
        { let __v = ProfiledData::compute_if_absent(&compat::read(__this, |__t| __t.all_rewards.clone()).unwrap(), Arc::new(move || Vec::new())); compat::write(__this, |__t| __t.current_profile_rewards = __v); }
        CorpseProfitTracker::recalculate_all(__this);
    }

    /// java: public boolean isEnabled()
    pub fn is_enabled(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().mining.glacite.enable_corpse_profit_tracker;
    }

    /// java: @SuppressWarnings("SameReturnValue") private boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(__this: &compat::Shared<Self>, text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::get_location() != Location::GlaciteMineshafts || !(CorpseProfitTracker::is_enabled(INSTANCE.get_mut())) || overlay {
            return true;
        }
        let mut message: String = text.get_string();
        if compat::read(__this, |__t| __t.inside_reward_message) && message == "▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬" {
            if compat::read(__this, |__t| __t.last_corpse_loot.clone()).is_none() {
                logger().error("Received a reward message end without a corresponding start. Report this!");
                return true;
            }
            let __h0 = compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap();
            __this.lock().borrow_mut().current_profile_rewards.push(__h0);
            if !(CorpseLoot::is_price_data_complete(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap())) {
                // java: Minecraft.getInstance().gui.hud.getChat().addClientSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.corpseTracker.somethingWentWrong").withStyle(ChatFormatting.GOLD)));
                add_client_system_message((constants::PREFIX.get_mut())().append(component::translatable("skyblocker.corpseTracker.somethingWentWrong").with_style_format(ChatFormatting::Gold)));
            } else {
                if !(SkyblockerConfigManager::get().mining.glacite.force_english_corpse_profit_tracker) {
                    // java: Minecraft.getInstance().gui.hud.getChat().addClientSystemMessage(..);
                    add_client_system_message(((constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.corpseTracker.corpseProfit", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&formatters::INTEGER_NUMBERS.get_mut().format(CorpseLoot::profit(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap())))).with_style_format((if CorpseLoot::profit(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap()) > (0 as f64) { ChatFormatting::Green } else { ChatFormatting::Red })))]))).with_style_fn(|style| style.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.corpseTracker.hoverText").with_style_format(ChatFormatting::Green))))).with_click_event(Some(ClickEvent::new_run_command("/skyblocker rewardTrackers corpse list false".to_string())))));
                } else {
                    // java: Minecraft.getInstance().gui.hud.getChat().addClientSystemMessage(Constants.PREFIX.get()
                    //   .append(Component.literal(String.format(CORPSE_PROFIT_MESSAGE, Formatters.INTEGER_NUMBERS.format(lastCorpseLoot.profit())))
                    //     .withStyle(lastCorpseLoot.profit() > 0 ? ChatFormatting.GREEN : ChatFormatting.RED))
                    //   .withStyle(style -> style.withHoverEvent(new HoverEvent.ShowText(Component.translatable("skyblocker.corpseTracker.hoverText").withStyle(ChatFormatting.GREEN)))
                    //     .withClickEvent(new ClickEvent.RunCommand("/skyblocker rewardTrackers corpse list false"))));
                    let __profit = CorpseLoot::profit(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap());
                    let __text = compat::string_format(CORPSE_PROFIT_MESSAGE, &[compat::FormatArg::from(formatters::INTEGER_NUMBERS.get_mut().format(__profit))]);
                    add_client_system_message((constants::PREFIX.get_mut())().append(component::literal(&__text).with_style_format(if __profit > (0 as f64) { ChatFormatting::Green } else { ChatFormatting::Red })).with_style_fn(|style| style.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.corpseTracker.hoverText").with_style_format(ChatFormatting::Green))))).with_click_event(Some(ClickEvent::new_run_command("/skyblocker rewardTrackers corpse list false".to_string())))));
                }
            }
            { let __v = None; compat::write(__this, |__t| __t.last_corpse_loot = __v); }
            { let __v = false; compat::write(__this, |__t| __t.inside_reward_message = __v); }
            return true;
        }
        let mut matcher: compat::regex::Matcher = CORPSE_PATTERN.matcher(&message);
        if !(compat::read(__this, |__t| __t.inside_reward_message)) && matcher.matches() {
            let mut corpse: String = matcher.group(1).unwrap();
            // java: try {..} catch (IllegalArgumentException _) - an exception here is a panic, caught by `catch_unwind`;
            // the try block assigns `type`, which the code after it reads, so the block hands it out.
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let r#type = CorpseType::value_of(&corpse.to_uppercase());
                { let __v = Some(CorpseLoot::new_3(r#type, Vec::new(), std::time::SystemTime::now())); compat::write(__this, |__t| __t.last_corpse_loot = __v); }
                r#type
            }));
            let r#type: CorpseType = match __result {
                Ok(r#type) => r#type,
                Err(_) => {
                    logger().error(&format!("Unknown corpse type `{}` for message: `{}`. Report this!", corpse, message));
                    return true;
                }
            };
            // java: try {..} catch (IllegalStateException _) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                CorpseLoot::profit_1(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap(), (CorpseLoot::profit(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap()) - (r#type).get_key_price()));
            }));
            if let Err(_) = __result {
                logger().warn(&format!("No key price found for corpse type `{}`. Profit calculation will not be accurate, therefore it will not be sent to chat. It will still be added to the corpse history.", corpse));
                CorpseLoot::mark_price_data_incomplete(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap());
            }
            { let __v = true; compat::write(__this, |__t| __t.inside_reward_message = __v); }
            return true;
        }
        if !(compat::read(__this, |__t| __t.inside_reward_message)) || compat::read(__this, |__t| __t.last_corpse_loot.clone()).is_none() || !(matcher.use_pattern(&abstract_profit_tracker::REWARD_PATTERN).matches()) {
            return true;
        }
        let mut item_name: String = matcher.group(1).unwrap();
        let mut amount: i32 = compat::number_utils_to_int(&matcher.group(2).unwrap().replace(",", ""), 1);
        if matcher.use_pattern(&abstract_profit_tracker::HOTM_XP_PATTERN).matches() {
            return true;
        }
        CorpseLoot::add_loot(&compat::read(__this, |__t| __t.last_corpse_loot.clone()).unwrap(), &item_name, amount);
        return true;
    }

    /// java: private void recalculateAll()
    pub fn recalculate_all(__this: &compat::Shared<Self>) {
        for corpse_loot in compat::read(__this, |__t| __t.current_profile_rewards.clone()) {
            CorpseLoot::profit_1(&corpse_loot, ((0) as f64) as f64);
            CorpseLoot::mark_price_data_complete(&corpse_loot);
            for reward in CorpseLoot::rewards(&corpse_loot) {
                if PRICELESS_ITEMS.iter().any(|e| *e == Reward::item_id(&reward)) {
                    continue;
                }
                let mut price: Option<f64> = ItemUtils::get_item_price_string(Some(Reward::item_id(&reward)));
                if price.is_none() {
                    logger().warn(&format!("No price found for item `{}`.", Reward::item_id(&reward)));
                    CorpseLoot::mark_price_data_incomplete(&corpse_loot);
                    continue;
                }
                CorpseLoot::profit_1(&corpse_loot, (CorpseLoot::profit(&corpse_loot) + (price.clone().unwrap() * (Reward::amount(&reward) as f64))));
                Reward::price_per_unit_1(&reward, price.clone().unwrap());
            }
            // java: try {..} catch (IllegalStateException _) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                CorpseLoot::profit_1(&corpse_loot, (CorpseLoot::profit(&corpse_loot) - CorpseLoot::corpse_type(&corpse_loot).get_key_price()));
            }));
            if let Err(_) = __result {
                logger().warn(&format!("No key price found for corpse type `{}`. Profit calculation will not be accurate.", compat::jstring(&CorpseLoot::corpse_type(&corpse_loot))));
                CorpseLoot::mark_price_data_incomplete(&corpse_loot);
            }
        }
    }

}

impl AbstractProfitTracker for CorpseProfitTracker {
    fn abstract_profit_tracker_base(&self) -> &AbstractProfitTrackerBase {
        &self.base
    }
    fn abstract_profit_tracker_base_mut(&mut self) -> &mut AbstractProfitTrackerBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// `Minecraft.getInstance().gui.hud.getChat().addClientSystemMessage(message)`: the chat line, with the frame values
/// the port's `ChatComponent.addClientSystemMessage` takes (the gui tick, the font, the chat options)
fn add_client_system_message(message: MutableComponent) {
    let client = compat::minecraft();
    let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message);
    let chat_focused = client.gui.screen_kind == mcrust::net::minecraft::client::gui::gui::ScreenKind::Chat;
    let options = mcrust::net::minecraft::client::gui::components::chat_component::chat_options(&client.options.borrow(), chat_focused);
    let ticks = client.gui.hud.tick_count;
    let splitter = client.font.splitter();
    client.gui.hud.chat.add_client_system_message(message, ticks, &splitter, options, chat_focused);
}
