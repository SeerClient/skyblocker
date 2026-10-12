//! Ported from de.hysky.skyblocker.UpdateNotifications by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::net::uri::Uri;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::shared_constants;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final String VERSIONS_URL = "https://api.modrinth.com/v2/project/skyblocker-liap/version?loaders=[%22fabric%22]&include_changelog=false";
pub const VERSIONS_URL: &str = "https://api.modrinth.com/v2/project/skyblocker-liap/version?loaders=[%22fabric%22]&include_changelog=false";
/// java: private static final Version MOD_VERSION = SkyblockerMod.SKYBLOCKER_MOD.getMetadata().getVersion();
pub static MOD_VERSION: compat::MainThread<mcrust::mcrust::modding::version::Version> = compat::MainThread::new(|| skyblocker_mod::SKYBLOCKER_MOD.get_mut().metadata.version.clone());
/// java: private static final Path CONFIG_PATH = SkyblockerMod.CONFIG_DIR.resolve("update_notifications_v2.json");
pub static CONFIG_PATH: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("update_notifications_v2.json"));
// TODO(fabric2seer): `Version.compareTo` (the loader's Version)
//   java: @VisibleForTesting
//   java: 	protected static final Comparator<Version> VERSION_COMPARATOR = Version::compareTo;
/// java: @VisibleForTesting protected static final Comparator<Version> VERSION_COMPARATOR = Version::compareTo;
// Every comparison in this class is between two `SemanticVersion`s (Fabric's `SemanticVersion extends Version`);
// `Version.compareTo` over two semantic versions is the port's `SemanticVersion: Ord`.
pub static VERSION_COMPARATOR: compat::MainThread<Arc<dyn Fn(&mcrust::mcrust::modding::version::SemanticVersion, &mcrust::mcrust::modding::version::SemanticVersion) -> std::cmp::Ordering>> = compat::MainThread::new(|| Arc::new(|a: &mcrust::mcrust::modding::version::SemanticVersion, b: &mcrust::mcrust::modding::version::SemanticVersion| a.cmp(b)));
/// java: @VisibleForTesting protected static final Codec<SemanticVersion> SEMANTIC_VERSION_CODEC = Codec.STRING.comapFlatMap(UpdateNotifications::parseVersion, SemanticVersion::toString);
pub static SEMANTIC_VERSION_CODEC: compat::MainThread<CodecRef<mcrust::mcrust::modding::version::SemanticVersion>> = compat::MainThread::new(|| codec::comap_flat_map(mcrust::com::mojang::serialization::codec::string_codec(), |__a0| UpdateNotifications::parse_version(&compat::jstring(&__a0)), |__a0| compat::jstring(&__a0)));
/// java: @VisibleForTesting protected static final Codec<SemanticVersion> MINECRAFT_VERSION_CODEC = Codec.STRING.comapFlatMap(UpdateNotifications::parseMinecraftVersion, SemanticVersion::toString);
pub static MINECRAFT_VERSION_CODEC: compat::MainThread<CodecRef<mcrust::mcrust::modding::version::SemanticVersion>> = compat::MainThread::new(|| codec::comap_flat_map(mcrust::com::mojang::serialization::codec::string_codec(), |__a0| UpdateNotifications::parse_minecraft_version(&compat::jstring(&__a0)), |__a0| compat::jstring(&__a0)));
// TODO(fabric2seer): the port's `new` ports this Java method with other parameters ( for Java's display_time)
//   java: private static final SystemToast.SystemToastId TOAST_TYPE = new SystemToast.SystemToastId(10000L);
/// java: private static final SystemToast.SystemToastId TOAST_TYPE = new SystemToast.SystemToastId(10000L);
pub static TOAST_TYPE: compat::MainThread<SystemToastId> = compat::MainThread::new(|| SystemToastId::with_display_time(10000i64));
/// java: private static final ProfiledData<Config> CONFIG = new ProfiledData<>(CONFIG_PATH, Config.CODEC);
pub static CONFIG: compat::MainThread<compat::Shared<ProfiledData<Config>>> = compat::MainThread::new(|| ProfiledData::new(CONFIG_PATH.get_mut().clone(), CONFIG_CODEC.get_mut().clone()));
/// java: private static boolean sentUpdateNotification;
pub static SENT_UPDATE_NOTIFICATION: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static @Nullable CompletableFuture<Void> loaded;
pub static LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);
/// java: public static final Config DEFAULT = new Config(true, Channel.ALPHA, Optional.empty());
pub static CONFIG_DEFAULT: compat::MainThread<Config> = compat::MainThread::new(|| Config::new(true, Channel::Alpha, None));
/// java: private static final Codec<Config> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.BOOL.fieldOf("enabled").forGetter(Config::enabled), Channel.CODEC.fieldOf("channel").forGetter(Co
pub static CONFIG_CODEC: compat::MainThread<CodecRef<Config>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &Config| o.enabled(), codec::field_of(mcrust::com::mojang::serialization::codec::bool_codec(), "enabled")), record_codec_builder::of(|o: &Config| o.channel(), codec::field_of(CHANNEL_CODEC.get_mut().clone(), "channel")), record_codec_builder::of(|o: &Config| o.newest_version_used(), codec::optional_field_of_opt(SEMANTIC_VERSION_CODEC.get_mut().clone(), "newestVersionUsed")), move |__a0, __a1, __a2| Config::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: private static final Codec<MrVersion> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("id").forGetter(MrVersion::id), Codec.STRING.fieldOf("name").forGetter(MrVersio
pub static MR_VERSION_CODEC: compat::MainThread<CodecRef<MrVersion>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply5(record_codec_builder::of(|o: &MrVersion| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &MrVersion| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &MrVersion| o.version(), codec::field_of(SEMANTIC_VERSION_CODEC.get_mut().clone(), "version_number")), record_codec_builder::of(|o: &MrVersion| o.minecraft_versions(), codec::field_of(codec::list(MINECRAFT_VERSION_CODEC.get_mut().clone()), "game_versions")), record_codec_builder::of(|o: &MrVersion| o.channel(), codec::field_of(CHANNEL_CODEC.get_mut().clone(), "version_type")), move |__a0, __a1, __a2, __a3, __a4| MrVersion::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone()))));
/// java: private static final Codec<List<MrVersion>> LIST_CODEC = CODEC.listOf();
pub static MR_VERSION_LIST_CODEC: compat::MainThread<CodecRef<Vec<MrVersion>>> = compat::MainThread::new(|| codec::list(MR_VERSION_CODEC.get_mut().clone()));
// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: private static final Codec<Channel> CODEC = StringRepresentable.fromValues(Channel::values);
/// java: private static final Codec<Channel> CODEC = StringRepresentable.fromValues(Channel::values);
pub static CHANNEL_CODEC: compat::MainThread<CodecRef<Channel>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_values(Channel::values(), |c: &Channel| c.get_serialized_name()));

/// java: public class UpdateNotifications
#[derive(Clone)]
pub struct UpdateNotifications {}

impl UpdateNotifications {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> loaded = CONFIG.init());
        skyblock_events::JOIN.register(Arc::new(move || {
            if LOADED.get_mut().is_none() {
                *LOADED.get_mut() = Some(CONFIG.get_mut().lock().borrow_mut().init());
            }
            // java: loaded.thenRunAsync(UpdateNotifications::tryCheckForNewVersion, Minecraft.getInstance());
            LOADED.get_mut().as_ref().unwrap().then_run(|| mcrust::mcrust::modding::game::execute(UpdateNotifications::try_check_for_new_version));
        }));
    }

    /// java: private static void tryCheckForNewVersion()
    pub fn try_check_for_new_version() {
        if UpdateNotifications::get_config().enabled() && !(compat::get_state(&SENT_UPDATE_NOTIFICATION)) {
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_3()); __f(__r, Arc::new(move || { UpdateNotifications::check_for_new_version(); }), 60i32.wrapping_mul(20), true) });
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { UpdateNotifications::introduce_new_update(); }), 60i32.wrapping_mul(20)) });
        }
    }

    /// java: private static void checkForNewVersion()
    pub fn check_for_new_version() {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut current_mod_version: mcrust::mcrust::modding::version::SemanticVersion = MOD_VERSION.get_mut().as_semantic().cloned().expect("ClassCastException: not a SemanticVersion");
            let mut current_minecraft_version: mcrust::mcrust::modding::version::SemanticVersion = UpdateNotifications::parse_minecraft_version(shared_constants::get_current_version().id()).get_or_throw().unwrap();
            // java: String response = Http.sendGetRequest(VERSIONS_URL);
            let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request(VERSIONS_URL);
            let mut mr_versions: Vec<MrVersion> = (MR_VERSION_LIST_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_or_throw().unwrap();
            compat::set_state(&SENT_UPDATE_NOTIFICATION, true);
            let mut optimal_version: Option<MrVersion> = UpdateNotifications::get_optimal_version(&current_mod_version, &current_minecraft_version, &mr_versions);
            if optimal_version.is_some() && !(UpdateNotifications::should_discard(&current_mod_version, &optimal_version.as_ref().unwrap().version())) {
                let mut new_version: MrVersion = optimal_version.unwrap();
                let mut download_link: String = format!("https://modrinth.com/mod/skyblocker-liap/version/{}", new_version.id());
                let mut version_text: MutableComponent = component::literal(&new_version.name()).with_style_fn(|style| style.apply_format(ChatFormatting::Gray).with_underlined(Some(true)).with_click_event(Some(ClickEvent::new_open_url(Uri::create(&download_link).unwrap()))));
                mcrust::mcrust::modding::game::execute(move || {
                    if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                        return;
                    }
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.updateNotifications.newUpdateMessage", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(version_text.clone())])).to_string()); });
                    // java: SystemToast.add(MINECRAFT.gui.toastManager(), TOAST_TYPE, Component.translatable(..title), Component.translatableEscape(..description, newVersion.version()));
                    let client = compat::minecraft();
                    let title = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.updateNotifications.newUpdateToast.title"));
                    let description = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_escape("skyblocker.updateNotifications.newUpdateToast.description", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(new_version.version().to_string())]));
                    system_toast::add(&mut client.gui.toast_manager().borrow_mut(), &client.font, *TOAST_TYPE.get(), &title, Some(&description));
                });
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Update Notifications] Failed to determine if an update is available or not!"));
        }
    }

    /// java: private static void introduceNewUpdate()
    pub fn introduce_new_update() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let mut newest_version_used: Option<mcrust::mcrust::modding::version::SemanticVersion> = UpdateNotifications::get_config().newest_version_used();
            let mut current_mod_version: mcrust::mcrust::modding::version::SemanticVersion = MOD_VERSION.get_mut().as_semantic().cloned().expect("ClassCastException: not a SemanticVersion");
            let mut should_introduce_update: bool = (newest_version_used.is_none() || (VERSION_COMPARATOR.get())(&current_mod_version, newest_version_used.as_ref().unwrap()) == std::cmp::Ordering::Greater);
            if !should_introduce_update {
                return;
            }
            let mut version_tag_string: String = format!("v{}.{}.{}", (current_mod_version.component((0) as usize) as i32), (current_mod_version.component((1) as usize) as i32), (current_mod_version.component((2) as usize) as i32));
            let mut configure_text: MutableComponent = component::translatable("skyblocker.updateNotifications.configureNewVersionHere").with_style_fn(|style| style.apply_format(ChatFormatting::Gray).with_underlined(Some(true)).with_click_event(Some(ClickEvent::new_run_command(format!("/skyblocker config {}", version_tag_string)))));
            let mut new_version_text: MutableComponent = component::translatable_args("skyblocker.updateNotifications.configureNewVersion", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(skyblocker_mod::VERSION.clone()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(configure_text.clone())]);
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(new_version_text.clone()).to_string()); });
            UpdateNotifications::set_config(Arc::new(move |config: Config| config.with_newest_version_used(&current_mod_version)));
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Update Notifications] Failed to introduce the new update!", e);
        //   java: 		}
    }

    /// java: @VisibleForTesting protected static Optional<MrVersion> getOptimalVersion(SemanticVersion currentModVersion, SemanticVersion currentMinecraftVersion, List<MrVersion> mrVersions)
    pub fn get_optimal_version(current_mod_version: &mcrust::mcrust::modding::version::SemanticVersion, current_minecraft_version: &mcrust::mcrust::modding::version::SemanticVersion, mr_versions: &Vec<MrVersion>) -> Option<MrVersion> {
        let mut config: Config = (if crate::de::hysky::skyblocker::debug::debug::Debug::is_test_environment() { CONFIG_DEFAULT.get().clone() } else { UpdateNotifications::get_config() });
        let comparator = VERSION_COMPARATOR.get();
        let mut eligible_mod_versions: Vec<MrVersion> = mr_versions.iter().filter(|release_version| config.encompassing_channels().contains(&release_version.channel())).filter(|release_version| comparator(&release_version.version, current_mod_version) == std::cmp::Ordering::Greater).cloned().collect::<Vec<_>>();
        if eligible_mod_versions.is_empty() {
            return None;
        }
        // java: SemanticVersion newestModVersion = Collections.max(eligibleModVersions, Comparator.comparing(MrVersion::version)).version();
        // (`Collections.max` keeps the FIRST of equal maxima, `Iterator::max_by` the last; only the version is read, so it is the same)
        let mut newest_mod_version: mcrust::mcrust::modding::version::SemanticVersion = eligible_mod_versions.iter().max_by(|a, b| a.version.cmp(&b.version)).unwrap().version();
        let mut latest_mod_version_for_current_minecraft_version: Option<MrVersion> = eligible_mod_versions.iter().filter(|release_version| comparator(&release_version.version, &newest_mod_version) == std::cmp::Ordering::Equal).filter(|release_version| release_version.minecraft_versions.iter().any(|version| comparator(version, current_minecraft_version) == std::cmp::Ordering::Equal)).next().cloned();
        if latest_mod_version_for_current_minecraft_version.is_some() {
            return latest_mod_version_for_current_minecraft_version;
        }
        // java: Stream.max keeps the LAST of equal maxima, as `Iterator::max_by` does
        return eligible_mod_versions.iter().filter(|release_version| comparator(&release_version.version, &newest_mod_version) == std::cmp::Ordering::Equal).max_by(|a, b| comparator(&a.newest_minecraft_version_supported(), &b.newest_minecraft_version_supported())).cloned();
    }

    /// java: private static DataResult<SemanticVersion> parseVersion(String version)
    pub fn parse_version(version: &str) -> DataResult<mcrust::mcrust::modding::version::SemanticVersion> {
        // java: String formattedVersion = version.transform(s -> s.charAt(0) == 'v' ? s.substring(1) : s);
        let mut formatted_version: String = if compat::char_at(version, 0) == 'v' { compat::substring(version, 1) } else { version.to_string() };
        // java: try {..} catch (VersionParsingException _) - `SemanticVersion::parse` answers the exception as `Err`
        match mcrust::mcrust::modding::version::SemanticVersion::parse(&formatted_version) {
            Ok(parsed) => return DataResult::success(parsed),
            Err(_) => {
                return DataResult::error(Arc::new(move || format!("Failed to parse semantic version from string: {}", formatted_version)));
            }
        }
    }

    /// java: private static DataResult<SemanticVersion> parseMinecraftVersion(String minecraftVersion)
    pub fn parse_minecraft_version(minecraft_version: &str) -> DataResult<mcrust::mcrust::modding::version::SemanticVersion> {
        // java: String normalized = McVersionLookup.normalizeVersion(minecraftVersion, McVersionLookup.getRelease(minecraftVersion));
        let mut normalized: String = normalize_minecraft_version(minecraft_version);
        return UpdateNotifications::parse_version(&normalized);
    }

    /// java: private static boolean isUnofficialAlphaOrBeta(SemanticVersion version)
    pub fn is_unofficial_alpha_or_beta(version: &mcrust::mcrust::modding::version::SemanticVersion) -> bool {
        // java: return switch (version.getPrereleaseKey().orElse("")) {
        //   case String s when s.startsWith("alpha") -> s.substring(5).charAt(0) == '-';
        //   case String s when s.startsWith("beta") -> s.substring(4).charAt(0) == '-';
        //   default -> false; };
        let s: &str = version.prerelease.as_deref().unwrap_or("");
        if s.starts_with("alpha") {
            return compat::char_at(&compat::substring(s, 5), 0) == '-';
        }
        if s.starts_with("beta") {
            return compat::char_at(&compat::substring(s, 4), 0) == '-';
        }
        false
    }

    /// java: @VisibleForTesting protected static boolean shouldDiscard(SemanticVersion currentVersion, SemanticVersion latestVersion)
    pub fn should_discard(current_version: &mcrust::mcrust::modding::version::SemanticVersion, latest_version: &mcrust::mcrust::modding::version::SemanticVersion) -> bool {
        if UpdateNotifications::is_unofficial_alpha_or_beta(current_version) {
            let mut current_major: i32 = (current_version.component((0) as usize) as i32);
            let mut current_minor: i32 = (current_version.component((1) as usize) as i32);
            let mut current_patch: i32 = (current_version.component((2) as usize) as i32);
            let mut latest_major: i32 = (latest_version.component((0) as usize) as i32);
            let mut latest_minor: i32 = (latest_version.component((1) as usize) as i32);
            let mut latest_patch: i32 = (latest_version.component((2) as usize) as i32);
            return (current_major == latest_major && current_minor == latest_minor && current_patch == latest_patch);
        }
        return false;
    }

    /// java: public static Config getConfig()
    pub fn get_config() -> Config {
        return ProfiledData::compute_if_absent_3(CONFIG.get(), &Utils::get_uuid(), "".to_string(), Arc::new(move || CONFIG_DEFAULT.get().clone()));
    }

    /// java: public static void setConfig(UnaryOperator<Config> updater)
    pub fn set_config(updater: Arc<dyn Fn(Config) -> Config>) {
        ProfiledData::put_3(CONFIG.get(), &Utils::get_uuid(), "".to_string(), (updater)(UpdateNotifications::get_config()));
    }

}

/// java: public record Config(boolean enabled, Channel channel, Optional<SemanticVersion> newestVersionUsed)
#[derive(Clone)]
pub struct Config {
    /// java: boolean enabled
    pub enabled: bool,
    /// java: Channel channel
    pub channel: Channel,
    /// java: Optional<SemanticVersion> newestVersionUsed
    pub newest_version_used: Option<mcrust::mcrust::modding::version::SemanticVersion>,
}

impl Config {
    /// java: the canonical record constructor
    pub fn new(mut enabled: bool, mut channel: Channel, mut newest_version_used: Option<mcrust::mcrust::modding::version::SemanticVersion>) -> Self {
        Self { enabled, channel, newest_version_used }
    }

    /// java: the record accessor `enabled()`
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// java: the record accessor `channel()`
    pub fn channel(&self) -> Channel {
        self.channel
    }

    /// java: the record accessor `newestVersionUsed()`
    pub fn newest_version_used(&self) -> Option<mcrust::mcrust::modding::version::SemanticVersion> {
        self.newest_version_used.clone()
    }

    /// java: private List<Channel> encompassingChannels()
    pub fn encompassing_channels(&self) -> Vec<Channel> {
        return match self.channel {
            Channel::Beta => vec![Channel::Release, Channel::Beta],
            Channel::Alpha => Channel::values().to_vec(),
            _ => vec![self.channel],
        };
    }

    /// java: public Config withEnabled(boolean newEnabled)
    pub fn with_enabled(&self, mut new_enabled: bool) -> Config {
        return Config::new(new_enabled, self.channel, self.newest_version_used.clone());
    }

    /// java: public Config withChannel(Channel newChannel)
    pub fn with_channel(&self, mut new_channel: Channel) -> Config {
        return Config::new(self.enabled, new_channel, self.newest_version_used.clone());
    }

    /// java: private Config withNewestVersionUsed(SemanticVersion newNewestVersionUsed)
    pub fn with_newest_version_used(&self, new_newest_version_used: &mcrust::mcrust::modding::version::SemanticVersion) -> Config {
        return Config::new(self.enabled, self.channel, Some(new_newest_version_used.clone()));
    }

}

/// java: @VisibleForTesting protected record MrVersion(String id, String name, SemanticVersion version, List<SemanticVersion> minecraftVersions, Channel channel)
#[derive(Clone)]
pub struct MrVersion {
    /// java: String id
    pub id: String,
    /// java: String name
    pub name: String,
    /// java: SemanticVersion version
    pub version: mcrust::mcrust::modding::version::SemanticVersion,
    /// java: List<SemanticVersion> minecraftVersions
    pub minecraft_versions: Vec<mcrust::mcrust::modding::version::SemanticVersion>,
    /// java: Channel channel
    pub channel: Channel,
}

impl MrVersion {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut name: String, mut version: mcrust::mcrust::modding::version::SemanticVersion, mut minecraft_versions: Vec<mcrust::mcrust::modding::version::SemanticVersion>, mut channel: Channel) -> Self {
        Self { id, name, version, minecraft_versions, channel }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `version()`
    pub fn version(&self) -> mcrust::mcrust::modding::version::SemanticVersion {
        self.version.clone()
    }

    /// java: the record accessor `minecraftVersions()`
    pub fn minecraft_versions(&self) -> Vec<mcrust::mcrust::modding::version::SemanticVersion> {
        self.minecraft_versions.clone()
    }

    /// java: the record accessor `channel()`
    pub fn channel(&self) -> Channel {
        self.channel
    }

    /// java: private SemanticVersion newestMinecraftVersionSupported()
    pub fn newest_minecraft_version_supported(&self) -> mcrust::mcrust::modding::version::SemanticVersion {
        return self.minecraft_versions.iter().max_by(|a, b| (VERSION_COMPARATOR.get())(a, b)).cloned().unwrap();
    }

}

/// java: public enum Channel implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Channel {
    Release,
    Beta,
    Alpha,
}

impl Channel {
    pub const VALUES: [Channel; 3] = [Channel::Release, Channel::Beta, Channel::Alpha];

    /// java: values()
    pub fn values() -> Vec<Channel> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Channel::Release => "RELEASE",
            Channel::Beta => "BETA",
            Channel::Alpha => "ALPHA",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Channel {
        match name {
            "RELEASE" => Channel::Release,
            "BETA" => Channel::Beta,
            "ALPHA" => Channel::Alpha,
            _ => panic!("IllegalArgumentException: No enum constant Channel.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // java: return I18n.get("skyblocker.config.general.updateNotifications.updateChannel.channel." + name());
        mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.general.updateNotifications.updateChannel.channel.{}", self.name()), &[])
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // java: return name().toLowerCase(Locale.ENGLISH);
        self.name().to_lowercase()
    }

}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Channel::to_string(self))

    }
}

/// Fabric Loader's `McVersionLookup.normalizeVersion(name, getRelease(name))` for the version ids this client can
/// carry: a release (`26.2`) is its own release and normalizes to itself; `X-preN` / `X-rcN` become the semver
/// pre-releases `X-beta.N` / `X-rc.N`.
/// rust: port gap - the loader's snapshot (`26w14a`) table is not in the crate: such an id is answered unchanged
/// and the semantic-version parse after it fails, as the update check's `catch` expects of a bad version
/// (GAPS_L.md).
fn normalize_minecraft_version(name: &str) -> String {
    let is_release = |v: &str| !v.is_empty() && v.split('.').all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()));
    if is_release(name) {
        return name.to_string();
    }
    if let Some((release, pre)) = name.split_once("-pre")
        && is_release(release)
        && pre.chars().all(|c| c.is_ascii_digit())
    {
        return format!("{release}-beta.{pre}");
    }
    if let Some((release, rc)) = name.split_once("-rc")
        && is_release(release)
        && rc.chars().all(|c| c.is_ascii_digit())
    {
        return format!("{release}-rc.{rc}");
    }
    name.to_string()
}
