//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.ProfileViewer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::error_profile_viewer_screen::ErrorProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_profile_viewer_screen::LoadingProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile_response::ApiProfileResponse;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer_screen::ProfileViewerScreen;
use crate::de::hysky::skyblocker::utils::api_utils::ApiUtils;
use crate::de::hysky::skyblocker::utils::profile_utils::ProfileUtils;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::util::undashed_uuid;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards::EliteLeaderboards;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;

/// java: private static final Gson GSON = SkyblockerMod.GSON.newBuilder() .registerTypeHierarchyAdapter(UUID.class, new LenientUuidTypeAdapter()) .create();
///
/// The binding itself is each `profileviewer2.model` class's `from_json` (Gson's reflective adapter written out per
/// class, `@SerializedName` the member names), with `UUID`s read by `json_uuid` - the `LenientUuidTypeAdapter` this
/// builder registers - and `Object`s by `json_object` (Gson's `ObjectTypeAdapter`).
pub static GSON: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| crate::unresolved::Gson);
/// java: public static final boolean ENABLED = Debug.debugEnabled();
pub static ENABLED: LazyLock<bool> = LazyLock::new(|| crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled());

/// `String.matches("[A-Za-z0-9_]+")`, compiled once.
static VALID_PLAYER_NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[A-Za-z0-9_]+"));

/// java: public class ProfileViewer
#[derive(Clone)]
pub struct ProfileViewer {}

impl ProfileViewer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if !(*ENABLED) {
            return;
        }
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                //   java: LiteralArgumentBuilder<FabricClientCommandSource> pvCommandBuilder = ClientCommands.literal("pv2")
                //   java: 					.executes(Scheduler.queueOpenScreenFactoryCommand(context -> openProfileViewer(context.getSource(), context.getSource().getClient().getUser().getName())))
                //   java: 					.then(ClientCommands.argument("name", StringArgumentType.string())
                //   java: 							.suggests((context, builder) -> SharedSuggestionProvider.suggest(getPlayerSuggestions(context.getSource()), builder))
                //   java: 							.executes(Scheduler.queueOpenScreenFactoryCommand(context -> openProfileViewer(context.getSource(), StringArgumentType.getString(context, "name")))));
                let mut pv_command_builder: LiteralArgumentBuilder<FabricClientCommandSource> = client_command_registration_callback::literal("pv2");
                pv_command_builder.executes(crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen_factory_command(Arc::new(|context: &CommandContext<FabricClientCommandSource>| {
                    let name: String = compat::minecraft().get_user().get_name().to_string();
                    ProfileViewer::open_profile_viewer(context.get_source(), &name)
                })));
                let mut name_argument = client_command_registration_callback::argument("name", Arc::new(StringArgumentType::string()));
                let player_suggestions: mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<FabricClientCommandSource> = Arc::new(|context: &CommandContext<FabricClientCommandSource>, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| {
                    Ok(mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&ProfileViewer::get_player_suggestions(context.get_source()), builder))
                });
                name_argument.suggests(Some(player_suggestions));
                name_argument.executes(crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen_factory_command(Arc::new(|context: &CommandContext<FabricClientCommandSource>| {
                    ProfileViewer::open_profile_viewer(context.get_source(), &StringArgumentType::get_string(context, "name"))
                })));
                pv_command_builder.then(&name_argument);
                //   java: dispatcher.register(pvCommandBuilder);
                dispatcher.dispatcher().register(&pv_command_builder);
            },
        ));
    }

    /// java: private static String[] getPlayerSuggestions(FabricClientCommandSource source)
    ///
    /// Ensures that "dummy" players aren't included in command suggestions
    pub fn get_player_suggestions(source: &FabricClientCommandSource) -> Vec<String> {
        use mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider as _;
        return source.get_online_player_names().into_iter()
            .filter(|player_name| VALID_PLAYER_NAME.matches_all(player_name))
            .collect();
    }

    /// java: private static Screen openProfileViewer(FabricClientCommandSource source, String name)
    /// (the screen the command opens: a mod screen object, as `game::set_screen` takes it)
    ///
    /// `source.getClient()` is the one `Minecraft`; its executor (`thenApplyAsync(.., minecraft)`) is the render
    /// thread, which is where `compat::LocalFuture`'s `*_async` stages run.
    pub fn open_profile_viewer(_source: &FabricClientCommandSource, name: &str) -> Box<dyn mcrust::mcrust::modding::screens::ModScreen> {
        let name: String = name.to_string();
        // In a variable for stylistic purposes, do not change this
        let __name = name.clone();
        let _data_future: compat::LocalFuture<()> = ProfileViewer::load_data(&name)
            .then_apply_async(move |loaded_data: LoadedData| {
                let api_profile_response: Option<compat::Shared<ApiProfileResponse>> = loaded_data.api_profile_response();
                let selected_profile: Option<compat::Shared<ApiProfile>> = api_profile_response.as_ref().and_then(ApiProfileResponse::get_selected_profile);
                let game_profile: Option<GameProfile> = loaded_data.game_profile();
                // `loadedSkyblockProfile` / `hasSkyblockProfile` / `loadedGameProfile`, tested in Java's order
                match (api_profile_response, selected_profile, game_profile) {
                    (None, _, _) => {
                        return PendingScreen::of(Box::new(compat::ScreenAdapter::new(ErrorProfileViewerScreen::new("Failed to load Skyblock profiles.".to_string()))));
                    }
                    (Some(_), None, _) => {
                        return PendingScreen::of(Box::new(compat::ScreenAdapter::new(ErrorProfileViewerScreen::new("This user has no Skyblock profiles.".to_string()))));
                    }
                    (Some(_), Some(_), None) => {
                        return PendingScreen::of(Box::new(compat::ScreenAdapter::new(ErrorProfileViewerScreen::new("Player not found.".to_string()))));
                    }
                    (Some(api_profile_response), Some(selected_profile), Some(game_profile)) => {
                        // This should never throw since I presume the API cannot return a profile that the user is not a member of (given we request the user's profiles)
                        let member: compat::Shared<ProfileMember> = compat::read(&selected_profile, |__t| __t.members.get(&game_profile.id()).cloned()).unwrap_or_else(|| panic!("NullPointerException: {}", "profile member must not be null"));
                        logger().info(&format!("[Skyblocker Profile Viewer] Successfully loaded the profile for {}!", __name));
                        return PendingScreen::of(Box::new(compat::ScreenAdapter::new(ProfileViewerScreen::new(api_profile_response, selected_profile, game_profile, member, loaded_data.leaderboards()))));
                    }
                }
            })
            //   java: .thenAcceptAsync(minecraft.gui::setScreen, minecraft)
            .then_accept_async(|screen: PendingScreen| {
                if let Some(screen) = screen.take() {
                    mcrust::mcrust::modding::game::set_screen(screen);
                }
            })
            .exceptionally_async(|throwable: String| {
                logger().error(&format!("[Skyblocker Profile Viewer] Encountered an unknown exception when loading the data. {}", throwable));
                mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(ErrorProfileViewerScreen::new("Encountered an unknown error.".to_string()))));
                // java: return null
            });
        return Box::new(compat::ScreenAdapter::new(LoadingProfileViewerScreen::new(name)));
    }

    /// java: private static CompletableFuture<LoadedData> loadData(String name)
    ///
    /// {@return a {@link Pair} optionally containing the user's {@link ApiProfileResponse} and {@link GameProfile}}
    ///
    /// The network half - `name2Uuid`, the profiles request and the skin lookup, joined in turn - runs on the mod's
    /// executor as Java's `supplyAsync(.., VIRTUAL_THREAD_EXECUTOR)` does, over `Send` values only. The mod objects
    /// (`ApiProfileResponse`, which are render-thread `Shared`s) are built from the fetched JSON on the render thread,
    /// and the leaderboards request - which needs the selected profile's id - follows from there; the answer is a
    /// `LocalFuture` completed on the render thread.
    pub fn load_data(name: &str) -> compat::LocalFuture<LoadedData> {
        let name: String = name.to_string();
        // `Minecraft minecraft = Minecraft.getInstance()` - its `playerSkinRenderCache()`
        let player_skin_render_cache = compat::minecraft().game_renderer.player_skin_render_cache.clone();
        let fetched: CompletableFuture<FetchedData> = CompletableFuture::supply_async(move || {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                let uuid: String = ApiUtils::name2_uuid(&name);
                if uuid.is_empty() {
                    panic!("IllegalStateException: {}", "Invalid username");
                }
                // Load skyblock profiles
                //   java: ProfileUtils.fetchFullProfileByUuid(uuid).thenApply(Optional::ofNullable) - joined below
                let profiles_json: Option<serde_json::Map<String, serde_json::Value>> = match ProfileUtils::fetch_full_profile_by_uuid(&uuid) {
                    Some(future) => future.join_blocking().unwrap_or_else(|e| panic!("CompletionException: {e}")),
                    None => None,
                };
                // Load game profile
                //   java: ResolvableProfile resolvableProfile = ResolvableProfile.createUnresolved(UndashedUuid.fromString(uuid));
                //   java: minecraft.playerSkinRenderCache().lookup(resolvableProfile).thenApply(optional -> optional.map(PlayerSkinRenderCache.RenderInfo::gameProfile))
                let resolvable_profile: ResolvableProfile = ResolvableProfile::create_unresolved_id(undashed_uuid::from_string(&uuid).unwrap_or_else(|| panic!("IllegalArgumentException: Invalid UUID string: {uuid}"))).unwrap_or_else(|e| panic!("{e}"));
                let game_profile: Option<GameProfile> = match &player_skin_render_cache {
                    Some(cache) => cache.lookup(&resolvable_profile).join_blocking().unwrap_or_else(|e| panic!("CompletionException: {e}")).map(|render_info| render_info.game_profile().clone()),
                    None => None,
                };
                FetchedData { uuid, profiles_json, game_profile }
            })).map_err(|e| compat::panic_message(&e))
        }, compat::executor);
        let loaded: compat::LocalFuture<LoadedData> = compat::LocalFuture::new();
        let target = loaded.clone();
        let failed = loaded.clone();
        compat::local_of(&fetched)
            .then_apply_async(move |fetched: FetchedData| {
                //   java: .thenApply(opt -> opt.map(json -> GSON.fromJson(json, ApiProfileResponse.class)))
                let api_profile_response: Option<compat::Shared<ApiProfileResponse>> = fetched.profiles_json.as_ref().and_then(api_profile_response_from_json);
                // Load leaderboards for current profile
                //   java: skyblockProfileFuture.thenCompose(apiProfileResponse -> apiProfileResponse.map(ApiProfileResponse::getSelectedProfile)
                //   java: 		.map(profile -> EliteLeaderboards.fetchPlayerLeaderboards(uuid, profile.profileId))
                //   java: 		.orElse(CompletableFuture.completedFuture(Map.of())));
                let leaderboards: compat::LocalFuture<std::collections::HashMap<String, i32>> = match api_profile_response.as_ref().and_then(ApiProfileResponse::get_selected_profile) {
                    Some(profile) => compat::local_of(&EliteLeaderboards::fetch_player_leaderboards(&fetched.uuid, &compat::read(&profile, |__t| __t.profile_id.clone()))),
                    None => compat::LocalFuture::completed_future(std::collections::HashMap::new()),
                };
                let game_profile = fetched.game_profile;
                let failed = target.clone();
                leaderboards
                    .then_accept_async(move |leaderboards: std::collections::HashMap<String, i32>| {
                        target.complete(LoadedData::new(api_profile_response, leaderboards, game_profile));
                    })
                    .exceptionally_async(move |throwable: String| {
                        failed.complete_exceptionally(throwable);
                    });
            })
            .exceptionally_async(move |throwable: String| {
                failed.complete_exceptionally(throwable);
            });
        return loaded;
    }

}

/// What `loadData`'s supplier reads off the network (`Send`, so it rides from the worker to the render thread).
#[derive(Clone)]
struct FetchedData {
    uuid: String,
    profiles_json: Option<serde_json::Map<String, serde_json::Value>>,
    game_profile: Option<GameProfile>,
}

/// `GSON.fromJson(json, ApiProfileResponse.class)`.
fn api_profile_response_from_json(json: &serde_json::Map<String, serde_json::Value>) -> Option<compat::Shared<ApiProfileResponse>> {
    let _ = GSON.get();
    Some(ApiProfileResponse::from_json(&serde_json::Value::Object(json.clone())))
}

/// `GSON`'s `UUID` type: `LenientUuidTypeAdapter.deserialize` (`UUIDUtil.AUTHLIB_CODEC` - an int array, a dashed or an
/// undashed string). A value the codec rejects is `None` (the field keeps its constructor's value).
pub(crate) fn json_uuid(json: &serde_json::Value) -> Option<mcrust::java::util::uuid::Uuid> {
    use mcrust::com::mojang::serialization::decoder::Decoder as _;
    mcrust::net::minecraft::core::uuid_util::authlib_codec().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(json)).get_or_throw().ok()
}

/// A `Map<UUID, V>` key: Gson's `MapTypeAdapterFactory` reads the member name as a JSON string through the `UUID`
/// adapter (`json_uuid`).
pub(crate) fn json_uuid_key(key: &str) -> Option<mcrust::java::util::uuid::Uuid> {
    json_uuid(&serde_json::Value::String(key.to_string()))
}

/// Gson's `ObjectTypeAdapter` (an `Object` field or map value): a number is a `Double` (`ToNumberPolicy.DOUBLE`, the
/// default `SkyblockerMod.GSON` keeps), a string a `String`, a boolean a `Boolean`, an array an `ArrayList<Object>`, an
/// object a `LinkedTreeMap<String, Object>` (here `Vec<Object>` / `HashMap<String, Object>`; `null` elements are
/// `None`).
pub(crate) fn json_object(json: &serde_json::Value) -> compat::Object {
    match json {
        serde_json::Value::Number(n) => std::sync::Arc::new(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => std::sync::Arc::new(s.clone()),
        serde_json::Value::Bool(b) => std::sync::Arc::new(*b),
        serde_json::Value::Array(a) => std::sync::Arc::new(a.iter().map(|e| if e.is_null() { None } else { Some(json_object(e)) }).collect::<Vec<Option<compat::Object>>>()),
        serde_json::Value::Object(o) => std::sync::Arc::new(o.iter().map(|(k, e)| (k.clone(), if e.is_null() { None } else { Some(json_object(e)) })).collect::<std::collections::HashMap<String, Option<compat::Object>>>()),
        serde_json::Value::Null => std::sync::Arc::new(()),
    }
}


/// The screen `thenApplyAsync` hands `thenAcceptAsync(minecraft.gui::setScreen)` (`LocalFuture` values are cloned
/// between stages; the screen itself is moved out once).
#[derive(Clone)]
struct PendingScreen(std::rc::Rc<std::cell::RefCell<Option<Box<dyn mcrust::mcrust::modding::screens::ModScreen>>>>);

impl PendingScreen {
    fn of(screen: Box<dyn mcrust::mcrust::modding::screens::ModScreen>) -> PendingScreen {
        PendingScreen(std::rc::Rc::new(std::cell::RefCell::new(Some(screen))))
    }

    fn take(&self) -> Option<Box<dyn mcrust::mcrust::modding::screens::ModScreen>> {
        self.0.borrow_mut().take()
    }
}

/// java: private record LoadedData(Optional<ApiProfileResponse> apiProfileResponse, Map<String, Integer> leaderboards, Optional<GameProfile> gameProfile)
#[derive(Clone)]
pub struct LoadedData {
    /// java: Optional<ApiProfileResponse> apiProfileResponse
    pub api_profile_response: Option<compat::Shared<ApiProfileResponse>>,
    /// java: Map<String, Integer> leaderboards
    pub leaderboards: std::collections::HashMap<String, i32>,
    /// java: Optional<GameProfile> gameProfile
    pub game_profile: Option<GameProfile>,
}

impl LoadedData {
    /// java: the canonical record constructor
    pub fn new(mut api_profile_response: Option<compat::Shared<ApiProfileResponse>>, mut leaderboards: std::collections::HashMap<String, i32>, mut game_profile: Option<GameProfile>) -> Self {
        Self { api_profile_response, leaderboards, game_profile }
    }

    /// java: the record accessor `apiProfileResponse()`
    pub fn api_profile_response(&self) -> Option<compat::Shared<ApiProfileResponse>> {
        self.api_profile_response.clone()
    }

    /// java: the record accessor `leaderboards()`
    pub fn leaderboards(&self) -> std::collections::HashMap<String, i32> {
        self.leaderboards.clone()
    }

    /// java: the record accessor `gameProfile()`
    pub fn game_profile(&self) -> Option<GameProfile> {
        self.game_profile.clone()
    }

}
