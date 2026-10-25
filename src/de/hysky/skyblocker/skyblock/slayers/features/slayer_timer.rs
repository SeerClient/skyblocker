//! Ported from de.hysky.skyblocker.skyblock.slayers.features.SlayerTimer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerQuest;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_tier;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_tier::SlayerTier;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("slayer_personal_best.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("slayer_personal_best.json"));
/// java: private static final ProfiledData<Object2ObjectOpenHashMap<SlayerType, Object2ObjectOpenHashMap<SlayerTier, SlayerPersonalBest>>> CACHED_SLAYER_STATS = new ProfiledData<>(FILE, SlayerPersonalBest.SERI
pub static CACHED_SLAYER_STATS: compat::MainThread<compat::Shared<ProfiledData<std::collections::HashMap<SlayerType, std::collections::HashMap<SlayerTier, SlayerPersonalBest>>>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), SLAYER_PERSONAL_BEST_SERIALIZATION_CODEC.get_mut().clone()));
/// java: private static final Codec<SlayerPersonalBest> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.LONG.fieldOf("bestTimeMillis").forGetter(SlayerPersonalBest::bestTimeMillis), Codec.L
pub static SLAYER_PERSONAL_BEST_CODEC: compat::MainThread<CodecRef<SlayerPersonalBest>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &SlayerPersonalBest| o.best_time_millis(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "bestTimeMillis")), record_codec_builder::of(|o: &SlayerPersonalBest| o.date_millis(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "dateMillis")), move |__a0, __a1| SlayerPersonalBest::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<Object2ObjectOpenHashMap<SlayerType, Object2ObjectOpenHashMap<SlayerTier, SlayerPersonalBest>>> SERIALIZATION_CODEC = Codec.unboundedMap(SlayerType.CODEC, Codec.unboundedMap
pub static SLAYER_PERSONAL_BEST_SERIALIZATION_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<SlayerType, std::collections::HashMap<SlayerTier, SlayerPersonalBest>>>> = compat::MainThread::new(|| { use mcrust::com::mojang::serialization::codec as c; use crate::de::hysky::skyblocker::skyblock::slayers::{slayer_type, slayer_tier}; let __inner: CodecRef<std::collections::HashMap<SlayerTier, SlayerPersonalBest>> = c::xmap(c::unbounded_map(slayer_tier::CODEC.get_mut().clone(), SLAYER_PERSONAL_BEST_CODEC.get_mut().clone()), |entries: Vec<(SlayerTier, SlayerPersonalBest)>| entries.into_iter().collect::<std::collections::HashMap<SlayerTier, SlayerPersonalBest>>(), |map: &std::collections::HashMap<SlayerTier, SlayerPersonalBest>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()); c::xmap(c::unbounded_map(slayer_type::CODEC.get_mut().clone(), __inner), |entries: Vec<(SlayerType, std::collections::HashMap<SlayerTier, SlayerPersonalBest>)>| entries.into_iter().collect::<std::collections::HashMap<SlayerType, std::collections::HashMap<SlayerTier, SlayerPersonalBest>>>(), |map: &std::collections::HashMap<SlayerType, std::collections::HashMap<SlayerTier, SlayerPersonalBest>>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()) });

/// java: public class SlayerTimer
#[derive(Clone)]
pub struct SlayerTimer {}

impl SlayerTimer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CACHED_SLAYER_STATS.get_mut().lock().borrow_mut().load();
        // java: ClientCommandRegistrationCallback.EVENT.register(SlayerTimer::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                SlayerTimer::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext commandBuildContext)
    pub fn register_commands(dispatcher: &CommandDispatcher<client_command_registration_callback::FabricClientCommandSource>, command_build_context: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("slayers")
        //   java:     .then(literal("revenant").executes(context -> SlayerTimer.sendSlayerPersonalBest(context, SlayerType.REVENANT))) .. "vampire"));
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        let _ = command_build_context;
        let mut slayers = client_command_registration_callback::literal("slayers");
        for (name, slayer_type) in [("revenant", SlayerType::Revenant), ("tarantula", SlayerType::Tarantula), ("sven", SlayerType::Sven), ("voidgloom", SlayerType::Voidgloom), ("demonlord", SlayerType::Demonlord), ("vampire", SlayerType::Vampire)] {
            let mut node = client_command_registration_callback::literal(name);
            node.executes(Arc::new(move |context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| Ok(SlayerTimer::send_slayer_personal_best(context, slayer_type))));
            slayers.then(&node);
        }
        let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
        root.then(&slayers);
        dispatcher.register(&root);
    }

    /// java: private static int sendSlayerPersonalBest(CommandContext<FabricClientCommandSource> context, SlayerType slayerType)
    pub fn send_slayer_personal_best(context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>, mut slayer_type: SlayerType) -> i32 {
        let source = context.get_source();
        let mut tiers: Vec<SlayerTier> = SlayerTier::values();
        {
            let mut i: i32 = (tiers.len() as i32).wrapping_sub(1);
            while i >= 0 {
                let mut slayer_tier: SlayerTier = tiers[(i) as usize];
                let mut time: i64 = SlayerTimer::get_personal_best(slayer_type, slayer_tier);
                if time != (-1 as i64) {
                    let mut boss_text: MutableComponent = (component::literal(&format!("{} {}", slayer_type.fields().boss_name, slayer_tier.name()))).with_style_format(ChatFormatting::DarkPurple);
                    let mut time_text: MutableComponent = component::literal(&SlayerTimer::format_time(time)).with_style_format(ChatFormatting::Aqua);
                    // java: source.sendFeedback(Constants.PREFIX.get().append(bossText.append(": ").append(timeText)));
                    compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(boss_text.clone().append_str(": ").append(time_text.clone())));
                    return 1;
                }
                i -= 1;
            }
        }
        // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.slayer.personalBestMissing", slayerType.bossName)));
        compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.slayer.personalBestMissing", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(slayer_type.fields().boss_name.clone())])));
        return 0;
    }

    /// java: public static void sendMessage()
    pub fn send_message() {
        if !(SkyblockerConfigManager::get().slayers.slain_time) {
            return;
        }
        let mut slayer_quest: Option<compat::Shared<SlayerQuest>> = SlayerManager::get_slayer_quest();
        if slayer_quest.is_none() || compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.boss_spawn_time.clone()).is_none() {
            return;
        }
        let mut boss_death_time: std::time::SystemTime = (if compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.boss_death_time.clone()).is_some() { compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.boss_death_time.clone()) } else { Some(std::time::SystemTime::now()) }).unwrap();
        let mut current_pb_mills: i64 = SlayerTimer::get_personal_best(compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.slayer_type), compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.slayer_tier));
        let mut new_pb_mills: i64 = { let __start = compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.boss_spawn_time.clone()).expect("NullPointerException: slayerQuest.bossSpawnTime"); match boss_death_time.duration_since(__start) { Ok(d) => d.as_millis() as i64, Err(e) => -(e.duration().as_millis() as i64) } };
        let mut current_pb: String = SlayerTimer::format_time(current_pb_mills);
        let mut new_pb: String = SlayerTimer::format_time(new_pb_mills);
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        if current_pb_mills != (-1 as i64) && current_pb_mills > new_pb_mills {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::translatable_args("skyblocker.slayer.slainTime", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&new_pb).with_style_format(ChatFormatting::Yellow))])).append_str(" ").append(component::translatable("skyblocker.slayer.personalBest").with_style_format(ChatFormatting::LightPurple))).to_string()); });
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.slayer.previousPersonalBest", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&current_pb).with_style_format(ChatFormatting::Yellow))])).to_string()); });
            TitleContainer::add_title_and_play_sound_2(&Title::new("skyblocker.slayer.personalBest".to_string(), ChatFormatting::Aqua), 100);
            TitleContainer::add_title_2(&Title::new_1((component::literal(&current_pb).with_style_format(ChatFormatting::Yellow).append((component::literal(" ➜ ")).with_style_format(ChatFormatting::DarkAqua))).append(component::literal(&new_pb).with_style_format(ChatFormatting::Green))), 100);
            SlayerTimer::update_best_time(slayer_quest.as_ref().unwrap(), new_pb_mills);
        } else {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.slayer.slainTime", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&new_pb).with_style_format(ChatFormatting::Yellow))])).to_string()); });
            if current_pb_mills == (-1 as i64) {
                SlayerTimer::update_best_time(slayer_quest.as_ref().unwrap(), new_pb_mills);
            }
        }
    }

    /// java: private static long getPersonalBest(SlayerType slayerType, SlayerTier slayerTier)
    pub fn get_personal_best(mut slayer_type: SlayerType, mut slayer_tier: SlayerTier) -> i64 {
        // java: var profileData = CACHED_SLAYER_STATS.computeIfAbsent(Object2ObjectOpenHashMap::new);
        // java: var typeData = profileData.computeIfAbsent(slayerType, _ -> new Object2ObjectOpenHashMap<>());
        // java: SlayerPersonalBest currentBest = typeData.get(slayerTier);
        let current_best: Option<SlayerPersonalBest> = ProfiledData::compute_if_absent_with(CACHED_SLAYER_STATS.get_mut(), Arc::new(move || std::collections::HashMap::new()), |profile_data| {
            let type_data = profile_data.entry(slayer_type).or_insert_with(std::collections::HashMap::new);
            type_data.get(&slayer_tier).cloned()
        });
        // java: return currentBest != null ? currentBest.bestTimeMillis() : -1;
        return match current_best { Some(current_best) => current_best.best_time_millis(), None => -1 };
    }

    /// java: private static void updateBestTime(SlayerManager.SlayerQuest slayerQuest, long timeElapsed)
    pub fn update_best_time(slayer_quest: &compat::Shared<SlayerQuest>, mut time_elapsed: i64) {
        // java: var profileData = CACHED_SLAYER_STATS.computeIfAbsent(Object2ObjectOpenHashMap::new);
        // java: var typeData = profileData.computeIfAbsent(slayerQuest.slayerType, _ -> new Object2ObjectOpenHashMap<>());
        // java: typeData.put(slayerQuest.slayerTier, new SlayerPersonalBest(timeElapsed, System.currentTimeMillis()));
        let (slayer_type, slayer_tier) = compat::read(slayer_quest, |__t| (__t.slayer_type, __t.slayer_tier));
        ProfiledData::compute_if_absent_with(CACHED_SLAYER_STATS.get_mut(), Arc::new(move || std::collections::HashMap::new()), |profile_data| {
            let type_data = profile_data.entry(slayer_type).or_insert_with(std::collections::HashMap::new);
            type_data.insert(slayer_tier, SlayerPersonalBest::new(time_elapsed, compat::millis()));
        });
        CACHED_SLAYER_STATS.get_mut().lock().borrow().save();
    }

    /// java: private static String formatTime(long millis)
    pub fn format_time(mut millis: i64) -> String {
        return compat::string_format("%.2fs", &[((millis as f64) / 1000.0).into()]);
    }

}

/// java: private record SlayerPersonalBest(long bestTimeMillis, long dateMillis)
#[derive(Clone, Debug, PartialEq)]
pub struct SlayerPersonalBest {
    /// java: long bestTimeMillis
    pub best_time_millis: i64,
    /// java: long dateMillis
    pub date_millis: i64,
}

impl SlayerPersonalBest {
    /// java: the canonical record constructor
    pub fn new(mut best_time_millis: i64, mut date_millis: i64) -> Self {
        Self { best_time_millis, date_millis }
    }

    /// java: the record accessor `bestTimeMillis()`
    pub fn best_time_millis(&self) -> i64 {
        self.best_time_millis
    }

    /// java: the record accessor `dateMillis()`
    pub fn date_millis(&self) -> i64 {
        self.date_millis
    }

}
