//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.PowderMiningTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::item_price_update_event;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker::AbstractProfitTracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::abstract_profit_tracker::AbstractProfitTrackerBase;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::powder_mining_widget;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final PowderMiningTracker INSTANCE = new PowderMiningTracker();
pub static INSTANCE: compat::MainThread<compat::Shared<PowderMiningTracker>> = compat::MainThread::new(|| PowderMiningTracker::new());
/// java: private static final Object2ObjectArrayMap<String, String> NAME2ID_MAP = new Object2ObjectArrayMap<>(50);
/// An `Object2ObjectArrayMap` keeps insertion order (its iteration order is read: `comparePriority`, the filter
/// screen's item list), so it is an `IndexMap`.
pub static NAME2ID_MAP: LazyLock<indexmap::IndexMap<String, String>> = LazyLock::new(|| {
    let mut value = indexmap::IndexMap::with_capacity((50) as usize);
    value.insert("Gemstone Powder".to_string(), "GEMSTONE_POWDER".to_string());
    value.insert(format!("{} Rough Ruby Gemstone", sky_block_icons::HEALTH), "ROUGH_RUBY_GEM".to_string());
    value.insert(format!("{} Flawed Ruby Gemstone", sky_block_icons::HEALTH), "FLAWED_RUBY_GEM".to_string());
    value.insert(format!("{} Fine Ruby Gemstone", sky_block_icons::HEALTH), "FINE_RUBY_GEM".to_string());
    value.insert(format!("{} Flawless Ruby Gemstone", sky_block_icons::HEALTH), "FLAWLESS_RUBY_GEM".to_string());
    value.insert(format!("{} Rough Amethyst Gemstone", sky_block_icons::DEFENSE), "ROUGH_AMETHYST_GEM".to_string());
    value.insert(format!("{} Flawed Amethyst Gemstone", sky_block_icons::DEFENSE), "FLAWED_AMETHYST_GEM".to_string());
    value.insert(format!("{} Fine Amethyst Gemstone", sky_block_icons::DEFENSE), "FINE_AMETHYST_GEM".to_string());
    value.insert(format!("{} Flawless Amethyst Gemstone", sky_block_icons::DEFENSE), "FLAWLESS_AMETHYST_GEM".to_string());
    value.insert(format!("{} Rough Jade Gemstone", sky_block_icons::MINING_FORTUNE), "ROUGH_JADE_GEM".to_string());
    value.insert(format!("{} Flawed Jade Gemstone", sky_block_icons::MINING_FORTUNE), "FLAWED_JADE_GEM".to_string());
    value.insert(format!("{} Fine Jade Gemstone", sky_block_icons::MINING_FORTUNE), "FINE_JADE_GEM".to_string());
    value.insert(format!("{} Flawless Jade Gemstone", sky_block_icons::MINING_FORTUNE), "FLAWLESS_JADE_GEM".to_string());
    value.insert(format!("{} Rough Amber Gemstone", sky_block_icons::MINING_SPEED), "ROUGH_AMBER_GEM".to_string());
    value.insert(format!("{} Flawed Amber Gemstone", sky_block_icons::MINING_SPEED), "FLAWED_AMBER_GEM".to_string());
    value.insert(format!("{} Fine Amber Gemstone", sky_block_icons::MINING_SPEED), "FINE_AMBER_GEM".to_string());
    value.insert(format!("{} Flawless Amber Gemstone", sky_block_icons::MINING_SPEED), "FLAWLESS_AMBER_GEM".to_string());
    value.insert(format!("{} Rough Sapphire Gemstone", sky_block_icons::INTELLIGENCE), "ROUGH_SAPPHIRE_GEM".to_string());
    value.insert(format!("{} Flawed Sapphire Gemstone", sky_block_icons::INTELLIGENCE), "FLAWED_SAPPHIRE_GEM".to_string());
    value.insert(format!("{} Fine Sapphire Gemstone", sky_block_icons::INTELLIGENCE), "FINE_SAPPHIRE_GEM".to_string());
    value.insert(format!("{} Flawless Sapphire Gemstone", sky_block_icons::INTELLIGENCE), "FLAWLESS_SAPPHIRE_GEM".to_string());
    value.insert(format!("{} Rough Topaz Gemstone", sky_block_icons::PRISTINE), "ROUGH_TOPAZ_GEM".to_string());
    value.insert(format!("{} Flawed Topaz Gemstone", sky_block_icons::PRISTINE), "FLAWED_TOPAZ_GEM".to_string());
    value.insert(format!("{} Fine Topaz Gemstone", sky_block_icons::PRISTINE), "FINE_TOPAZ_GEM".to_string());
    value.insert(format!("{} Flawless Topaz Gemstone", sky_block_icons::PRISTINE), "FLAWLESS_TOPAZ_GEM".to_string());
    value.insert(format!("{} Rough Jasper Gemstone", sky_block_icons::STRENGTH), "ROUGH_JASPER_GEM".to_string());
    value.insert(format!("{} Flawed Jasper Gemstone", sky_block_icons::STRENGTH), "FLAWED_JASPER_GEM".to_string());
    value.insert(format!("{} Fine Jasper Gemstone", sky_block_icons::STRENGTH), "FINE_JASPER_GEM".to_string());
    value.insert(format!("{} Flawless Jasper Gemstone", sky_block_icons::STRENGTH), "FLAWLESS_JASPER_GEM".to_string());
    value.insert("Pickonimbus 2000".to_string(), "PICKONIMBUS".to_string());
    value.insert("Ascension Rope".to_string(), "ASCENSION_ROPE".to_string());
    value.insert("Wishing Compass".to_string(), "WISHING_COMPASS".to_string());
    value.insert("Gold Essence".to_string(), "ESSENCE_GOLD".to_string());
    value.insert("Diamond Essence".to_string(), "ESSENCE_DIAMOND".to_string());
    value.insert("Prehistoric Egg".to_string(), "PREHISTORIC_EGG".to_string());
    value.insert("Sludge Juice".to_string(), "SLUDGE_JUICE".to_string());
    value.insert("Oil Barrel".to_string(), "OIL_BARREL".to_string());
    value.insert("Jungle Heart".to_string(), "JUNGLE_HEART".to_string());
    value.insert("Treasurite".to_string(), "TREASURITE".to_string());
    value.insert("Yoggie".to_string(), "YOGGIE".to_string());
    value.insert("Goblin Egg".to_string(), "GOBLIN_EGG".to_string());
    value.insert("Green Goblin Egg".to_string(), "GOBLIN_EGG_GREEN".to_string());
    value.insert("Blue Goblin Egg".to_string(), "GOBLIN_EGG_BLUE".to_string());
    value.insert("Red Goblin Egg".to_string(), "GOBLIN_EGG_RED".to_string());
    value.insert("Yellow Goblin Egg".to_string(), "GOBLIN_EGG_YELLOW".to_string());
    value.insert("Control Switch".to_string(), "CONTROL_SWITCH".to_string());
    value.insert("Electron Transmitter".to_string(), "ELECTRON_TRANSMITTER".to_string());
    value.insert("FTX 3070".to_string(), "FTX_3070".to_string());
    value.insert("Synthetic Heart".to_string(), "SYNTHETIC_HEART".to_string());
    value.insert("Robotron Reflector".to_string(), "ROBOTRON_REFLECTOR".to_string());
    value.insert("Superlite Motor".to_string(), "SUPERLITE_MOTOR".to_string());
    value
});

/// java: public final class PowderMiningTracker extends AbstractProfitTracker
#[derive(Clone)]
pub struct PowderMiningTracker {
    /// java: extends AbstractProfitTracker
    pub base: AbstractProfitTrackerBase,
    /// java: @SuppressWarnings("JavadocReference") private Object2IntMap<String> currentProfileRewards = new Object2IntOpenHashMap<>();
    pub current_profile_rewards: std::collections::HashMap<String, i32>,
    /// java: private final Object2IntAVLTreeMap<Component> shownRewards = new Object2IntAVLTreeMap<>(Comparator.<Component>comparingInt(text -> comparePriority(text.getString())).thenComparing(c -> c.getString()));
    /// `Component` has no `Ord`; the tree is kept as entries sorted by that comparator, and two keys are the same key
    /// when the comparator answers 0 (equal `getString()`), as in the AVL map - see `shown_rewards_merge`/`_put`.
    pub shown_rewards: Vec<(MutableComponent, i32)>,
    /// java: private final ProfiledData<Object2IntMap<String>> allRewards = new ProfiledData<>(getRewardFilePath("powder-mining.json"), CodecUtils.object2IntMapCodec(Codec.S
    pub all_rewards: Option<compat::Shared<ProfiledData<std::collections::HashMap<String, i32>>>>,
    /// java: private boolean insideChestMessage = false;
    pub inside_chest_message: bool,
    /// java: private double profit = 0;
    pub profit: f64,
}

impl std::ops::Deref for PowderMiningTracker {
    type Target = AbstractProfitTrackerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PowderMiningTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PowderMiningTracker {
    /// java: private PowderMiningTracker()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractProfitTrackerBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                current_profile_rewards: std::collections::HashMap::new(),
                shown_rewards: Vec::new(),
                all_rewards: None,
                inside_chest_message: false,
                profit: ((0) as f64) as f64,
            });
            ((__h.clone() as compat::Shared<dyn AbstractProfitTracker>), __h)
        });
        { let __v = Some(ProfiledData::new(({ let __f = compat::dispatch(&__this, |__t| __t.get_reward_file_path()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractProfitTracker>), "powder-mining.json") }), CodecUtils::object2_int_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()))); compat::write(&__this, |__t| __t.all_rewards = __v); }
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(INSTANCE::onChatMessage);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| PowderMiningTracker::on_chat_message(INSTANCE.get_mut(), text, overlay));
        item_price_update_event::ON_PRICE_UPDATE.register(Arc::new(move || { PowderMiningTracker::on_price_update(INSTANCE.get_mut()); }));
        let all_rewards = compat::read(INSTANCE.get(), |__t| __t.all_rewards.clone()).unwrap();
        compat::write(&all_rewards, |__t| { __t.init(); });
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
                type Source = client_command_registration_callback::FabricClientCommandSource;
                // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("rewardTrackers").then(literal("powderMining")
                //   .then(literal("list").executes(ctx -> { .. }))
                //   .then(literal("reset").executes(ctx -> { .. })))));
                let mut list = client_command_registration_callback::literal("list");
                list.executes(Arc::new(|_: &CommandContext<Source>| {
                    let prefix = || (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())();
                    // java: if (INSTANCE.currentProfileRewards.isEmpty()) {..} else if (INSTANCE.shownRewards.isEmpty()) {..}
                    if compat::read(INSTANCE.get(), |__t| __t.current_profile_rewards.is_empty()) {
                        compat::batch_w_send_feedback(&prefix().append(component::translatable("skyblocker.powderTracker.emptyHistory").with_style_format(ChatFormatting::Red)));
                        return Ok(1);
                    } else if compat::read(INSTANCE.get(), |__t| __t.shown_rewards.is_empty()) {
                        compat::batch_w_send_feedback(&prefix().append(component::translatable("skyblocker.powderTracker.rewardsFilteredOut").with_style_format(ChatFormatting::Red)));
                        return Ok(1);
                    }
                    // java: for (Entry<Component> entry : INSTANCE.shownRewards.object2IntEntrySet()) ctx.getSource().sendFeedback(Component.empty().append(entry.getKey())
                    //   .append(Component.literal(": ").withStyle(ChatFormatting.GRAY)).append(Component.literal(String.valueOf(entry.getIntValue()))));
                    let shown_rewards = compat::read(INSTANCE.get(), |__t| __t.shown_rewards.clone());
                    for (key, value) in shown_rewards.iter() {
                        compat::batch_w_send_feedback(&component::empty().append(key.clone()).append(component::literal(": ").with_style_format(ChatFormatting::Gray)).append(component::literal(&value.to_string())));
                    }
                    // java: ctx.getSource().sendFeedback(Component.translatable("skyblocker.powderTracker.profit", NumberFormat.getInstance().format(INSTANCE.profit)).withStyle(ChatFormatting.GOLD));
                    let profit = compat::read(INSTANCE.get(), |__t| __t.profit);
                    compat::batch_w_send_feedback(&component::translatable_args("skyblocker.powderTracker.profit", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(&compat::number_format_instance().format(profit))]).with_style_format(ChatFormatting::Gold));
                    Ok(1)
                }));
                let mut reset = client_command_registration_callback::literal("reset");
                reset.executes(Arc::new(|_: &CommandContext<Source>| {
                    // java: INSTANCE.currentProfileRewards.clear(); INSTANCE.allRewards.save(); INSTANCE.shownRewards.clear();
                    compat::write(INSTANCE.get(), |__t| __t.current_profile_rewards.clear());
                    let all_rewards = compat::read(INSTANCE.get(), |__t| __t.all_rewards.clone());
                    if let Some(all_rewards) = all_rewards {
                        compat::read(&all_rewards, |__t| __t.save());
                    }
                    compat::write(INSTANCE.get(), |__t| __t.shown_rewards.clear());
                    compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.powderTracker.historyReset").with_style_format(ChatFormatting::Green)));
                    Ok(1)
                }));
                let mut powder_mining = client_command_registration_callback::literal("powderMining");
                powder_mining.then(&list);
                powder_mining.then(&reset);
                let mut reward_trackers = client_command_registration_callback::literal("rewardTrackers");
                reward_trackers.then(&powder_mining);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&reward_trackers);
                dispatcher.dispatcher().register(&root);
            },
        ));
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |__a0, __a1| { PowderMiningTracker::on_profile_change(INSTANCE.get_mut(), &__a0, &__a1); }));
    }

    /// java: private static int comparePriority(String itemName)
    pub fn compare_priority(item_name: &str) -> i32 {
        let s = AbstractProfitTrackerBase::replace_gemstone_symbols(item_name);
        return match s.as_str() {
            "Gemstone Powder" => 1,
            "Gold Essence" => 2,
            "Diamond Essence" => 3,
            // The guards are tried in order, so "Flawed"/"Flawless" (which also start with "Fla") are told apart as Java does.
            s if s.starts_with("Rough") => 4,
            s if s.starts_with("Flawed") => 5,
            s if s.starts_with("Fine") => 6,
            s if s.starts_with("Flawless") => 7,
            _ => 8,
        };
    }

    /// The `shownRewards` comparator: `comparingInt(text -> comparePriority(text.getString())).thenComparing(c -> c.getString())`.
    fn shown_rewards_compare(a: &str, b: &str) -> std::cmp::Ordering {
        PowderMiningTracker::compare_priority(a).cmp(&PowderMiningTracker::compare_priority(b)).then_with(|| compat::compare_to(a, b).cmp(&0))
    }

    /// `shownRewards.mergeInt(key, amount, Integer::sum)` on the sorted entries.
    fn shown_rewards_merge(shown_rewards: &mut Vec<(MutableComponent, i32)>, key: MutableComponent, amount: i32) {
        let key_string = key.get_string();
        match shown_rewards.binary_search_by(|(k, _)| PowderMiningTracker::shown_rewards_compare(&k.get_string(), &key_string)) {
            Ok(i) => shown_rewards[i].1 = shown_rewards[i].1.wrapping_add(amount),
            Err(i) => shown_rewards.insert(i, (key, amount)),
        }
    }

    /// `shownRewards.put(key, value)` on the sorted entries (an AVL put keeps the existing key object).
    fn shown_rewards_put(shown_rewards: &mut Vec<(MutableComponent, i32)>, key: MutableComponent, value: i32) {
        let key_string = key.get_string();
        match shown_rewards.binary_search_by(|(k, _)| PowderMiningTracker::shown_rewards_compare(&k.get_string(), &key_string)) {
            Ok(i) => shown_rewards[i].1 = value,
            Err(i) => shown_rewards.insert(i, (key, value)),
        }
    }

    /// java: @Unmodifiable public static Object2ObjectMap<String, String> getName2IdMap()
    pub fn get_name2_id_map() -> &'static indexmap::IndexMap<String, String> {
        // `Object2ObjectMaps.unmodifiable(NAME2ID_MAP)`: a read-only view of the map.
        return &NAME2ID_MAP;
    }

    /// java: public static double getProfit()
    pub fn get_profit() -> f64 {
        return compat::read(INSTANCE.get_mut(), |__t| __t.profit);
    }

    /// java: public static Object2IntMap<Component> getShownRewards()
    pub fn get_shown_rewards() -> Vec<(MutableComponent, i32)> {
        return compat::read(INSTANCE.get_mut(), |__t| __t.shown_rewards.clone());
    }

    /// java: @SuppressWarnings("BooleanMethodIsAlwaysInverted") public boolean isEnabled()
    pub fn is_enabled(__this: &compat::Shared<Self>) -> bool {
        // `PowderMiningWidget.INSTANCE` is assigned by the widget's constructor; before that Java passes null, which is in
        // no screen.
        return powder_mining_widget::INSTANCE.get().clone().is_some_and(|widget| WidgetManager::is_widget_in_current_screen(&(widget as compat::Shared<dyn HudWidget>)));
    }

    /// java: private void onProfileChange(String prevProfileId, String newProfileId)
    pub fn on_profile_change(__this: &compat::Shared<Self>, prev_profile_id: &str, new_profile_id: &str) {
        { let __v = ProfiledData::compute_if_absent(&compat::read(__this, |__t| __t.all_rewards.clone()).unwrap(), Arc::new(move || std::collections::HashMap::new())); compat::write(__this, |__t| __t.current_profile_rewards = __v); }
        if !(PowderMiningTracker::is_enabled(__this)) {
            return;
        }
        PowderMiningTracker::recalculate_all(__this);
    }

    /// java: @SuppressWarnings("SameReturnValue") private boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(__this: &compat::Shared<Self>, text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::get_location() != Location::CrystalHollows || !(PowderMiningTracker::is_enabled(INSTANCE.get_mut())) || overlay {
            return true;
        }
        let mut message: String = text.get_string();
        if compat::read(__this, |__t| __t.inside_chest_message) && message == "▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬" {
            { let __v = false; compat::write(__this, |__t| __t.inside_chest_message = __v); }
            return true;
        }
        if !(compat::read(__this, |__t| __t.inside_chest_message)) && (message == "  CHEST LOCKPICKED " || (SkyblockerConfigManager::get().mining.crystal_hollows.count_natural_chests_in_tracker && message == "  LOOT CHEST COLLECTED ")) {
            { let __v = true; compat::write(__this, |__t| __t.inside_chest_message = __v); }
            return true;
        }
        if !(compat::read(__this, |__t| __t.inside_chest_message)) {
            return true;
        }
        let mut matcher: compat::regex::Matcher = abstract_profit_tracker::REWARD_PATTERN.matcher(&message);
        if !matcher.matches() {
            return true;
        }
        let mut item_name: String = matcher.group(1).unwrap();
        // `NumberUtils.toInt(matcher.group(2).replace(",", ""), 1)`: `Integer.parseInt` (no trim), 1 on null or failure.
        let mut amount: i32 = matcher.group(2).and_then(|g| g.replace(",", "").parse::<i32>().ok()).unwrap_or(1);
        let mut item_id: String = PowderMiningTracker::get_item_id(__this, &item_name);
        if item_id.is_empty() {
            logger().error(&format!("No matching item id for name `{}`. Report this!", item_name));
            return true;
        }
        PowderMiningTracker::increment_reward(__this, &item_name, &item_id, amount);
        PowderMiningTracker::calculate_profit_for_item(__this, &item_id, amount);
        return true;
    }

    /// java: private void incrementReward(String itemName, String itemId, int amount)
    pub fn increment_reward(__this: &compat::Shared<Self>, item_name: &str, item_id: &str, mut amount: i32) {
        // `currentProfileRewards.mergeInt(itemId, amount, Integer::sum)`. Java's map IS the entry of `allRewards` for
        // this profile (`computeIfAbsent` hands back the stored object); `ProfiledData::compute_if_absent` returns a
        // copy, so the merged map is put back as the entry to keep the two the same.
        {
            let current = compat::write(__this, |__t| {
                let value = __t.current_profile_rewards.entry(item_id.to_string()).or_insert(0);
                *value = value.wrapping_add(amount);
                __t.current_profile_rewards.clone()
            });
            ProfiledData::put(&compat::read(__this, |__t| __t.all_rewards.clone()).unwrap(), current);
        }
        if !(SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().any(|e| *e == item_name.to_string())) {
            if item_id == "GEMSTONE_POWDER" {
                compat::write(__this, |__t| PowderMiningTracker::shown_rewards_merge(&mut __t.shown_rewards, component::literal("Gemstone Powder").with_style_format(ChatFormatting::LightPurple), amount));
            } else {
                let Some(mut stack) = ItemRepository::get_item_stack(item_id) else {
                    logger().warn(&format!("Item stack for id `{}` is null! This might be caused by failed item repository downloads.", item_id));
                    return;
                };
                // `stack.get(DataComponents.CUSTOM_NAME)`; a null name throws in the comparator's `getString()` in Java.
                let name = compat::component_of(FlexibleItemStack::get::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(&stack, mcrust::net::minecraft::core::component::data_components::custom_name()).expect("NullPointerException: CUSTOM_NAME"));
                compat::write(__this, |__t| PowderMiningTracker::shown_rewards_merge(&mut __t.shown_rewards, name, amount));
            }
        }
    }

    /// java: private void onPriceUpdate()
    pub fn on_price_update(__this: &compat::Shared<Self>) {
        if PowderMiningTracker::is_enabled(__this) {
            PowderMiningTracker::recalculate_prices(__this);
        }
    }

    /// java: private void calculateProfitForItem(String itemId, int amount)
    pub fn calculate_profit_for_item(__this: &compat::Shared<Self>, item_id: &str, mut amount: i32) {
        let mut price: Option<f64> = ItemUtils::get_item_price_string(Some(item_id.to_string()));
        if price.is_some() {
            { let __v = (compat::read(__this, |__t| __t.profit) + (price.clone().unwrap() * (amount as f64))); compat::write(__this, |__t| __t.profit = __v); }
        }
    }

    /// java: private void recalculatePrices()
    pub fn recalculate_prices(__this: &compat::Shared<Self>) {
        { let __v = ((0) as f64) as f64; compat::write(__this, |__t| __t.profit = __v); }
        let mut set: Vec<(String, i32)> = compat::read(__this, |__t| __t.shown_rewards.iter().map(|(k, v)| (k.get_string(), *v)).collect::<Vec<_>>());
        for entry in set.iter() {
            PowderMiningTracker::calculate_profit_for_item(__this, &PowderMiningTracker::get_item_id(__this, &entry.0), entry.1);
        }
    }

    /// java: public void recalculateAll()
    pub fn recalculate_all(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.shown_rewards.clear());
        let mut set: Vec<(String, i32)> = compat::read(__this, |__t| __t.current_profile_rewards.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>());
        let mut filters: Vec<String> = SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().cloned().map(|__a0| PowderMiningTracker::get_item_id(INSTANCE.get_mut(), &__a0)).collect::<Vec<_>>();
        for entry in set.iter() {
            if filters.iter().any(|e| *e == entry.0.clone()) {
                continue;
            }
            if entry.0 == "GEMSTONE_POWDER" {
                let __h0 = component::literal("Gemstone Powder").with_style_format(ChatFormatting::LightPurple);
                let __h1 = entry.1.clone();
                compat::write(__this, |__t| PowderMiningTracker::shown_rewards_put(&mut __t.shown_rewards, __h0, __h1));
            } else {
                let Some(mut stack) = ItemRepository::get_item_stack(&compat::jstring(&entry.0)) else {
                    logger().warn(&format!("Item stack for id `{}` is null! This might be caused by failed item repository downloads.", compat::jstring(&entry.0)));
                    continue;
                };
                let __h0 = compat::component_of(FlexibleItemStack::get::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(&stack, mcrust::net::minecraft::core::component::data_components::custom_name()).expect("NullPointerException: CUSTOM_NAME"));
                let __h1 = entry.1.clone();
                compat::write(__this, |__t| PowderMiningTracker::shown_rewards_put(&mut __t.shown_rewards, __h0, __h1));
            }
        }
        PowderMiningTracker::recalculate_prices(__this);
    }

    /// java: private String getItemId(String itemName)
    pub fn get_item_id(__this: &compat::Shared<Self>, item_name: &str) -> String {
        return NAME2ID_MAP.get(item_name).cloned().unwrap_or_else(|| "".to_string());
    }

}

impl AbstractProfitTracker for PowderMiningTracker {
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
