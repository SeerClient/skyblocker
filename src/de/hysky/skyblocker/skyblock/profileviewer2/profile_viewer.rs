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
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::util::undashed_uuid;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;

// TODO(fabric2seer): `.newBuilder(..)` on a Gson
//   java: private static final Gson GSON = SkyblockerMod.GSON.newBuilder()
//   java: 			.registerTypeHierarchyAdapter(UUID.class, new LenientUuidTypeAdapter())
//   java: 			.create();
/// java: private static final Gson GSON = SkyblockerMod.GSON.newBuilder() .registerTypeHierarchyAdapter(UUID.class, new LenientUuidTypeAdapter()) .create();
pub static GSON: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GSON` - see the TODO above"));
/// java: public static final boolean ENABLED = Debug.debugEnabled();
pub static ENABLED: LazyLock<bool> = LazyLock::new(|| crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled());

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
                // TODO(fabric2seer): a command builder outside dispatcher.register(..)
                //   java: LiteralArgumentBuilder<FabricClientCommandSource> pvCommandBuilder = ClientCommands.literal("pv2")
                //   java: 					.executes(Scheduler.queueOpenScreenFactoryCommand(context -> openProfileViewer(context.getSource(), context.getSource().getClient().getUser().getName())))
                //   java: 					.then(ClientCommands.argument("name", StringArgumentType.string())
                //   java: 							.suggests((context, builder) -> SharedSuggestionProvider.suggest(getPlayerSuggestions(context.getSource()), builder))
                //   java: 							.executes(Scheduler.queueOpenScreenFactoryCommand(context -> openProfileViewer(context.getSource(), StringArgumentType.getString(context, "name")))));
                let mut pv_command_builder: LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> = todo!("fabric2seer: the initialiser of `pvCommandBuilder` (TODO above)");
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(pvCommandBuilder);
            },
        ));
    }

    /// java: private static String[] getPlayerSuggestions(FabricClientCommandSource source)
    pub fn get_player_suggestions(source: &crate::unresolved::FabricClientCommandSource) -> Vec<String> {
        // TODO(fabric2seer): `.getOnlinePlayerNames(..)` on a FabricClientCommandSource
        //   java: return source.getOnlinePlayerNames().stream()
        //   java: 				.filter(playerName -> playerName.matches("[A-Za-z0-9_]+"))
        //   java: 				.toArray(String[]::new);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static Screen openProfileViewer(FabricClientCommandSource source, String name)
    pub fn open_profile_viewer(source: &crate::unresolved::FabricClientCommandSource, name: &str) -> Option<mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        // TODO(fabric2seer): `.getClient(..)` on a FabricClientCommandSource
        //   java: Minecraft minecraft = source.getClient();
        let mut minecraft: mcrust::net::minecraft::client::minecraft::Minecraft = todo!("fabric2seer: the initialiser of `minecraft` (TODO above)");
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // rust: let mut data_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = { let (__q0, __q1,) = (|throwable| {
        // rust: logger().error(&format!("[Skyblocker Profile Viewer] Encountered an unknown exception when loading the data."));
        // rust: mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(ErrorProfileViewerScreen::new("Encountered an unknown error.".to_string()))));
        // rust: return Default::default(); // java: return null
        // rust: }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)),); ({ let (__q0, __q1,) = (|__a0| gui::set_screen(&mut __a0), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)),); (ProfileViewer::load_data(name).then_apply_async(|loaded_data| {
        // rust: // TODO(fabric2seer): `.apiProfileResponse(..)` on a value of unknown type
        // rust: //   java: Optional<ApiProfileResponse> apiProfileResponse = loadedData.apiProfileResponse();
        // rust: let mut api_profile_response: Option<compat::Shared<ApiProfileResponse>> = todo!("fabric2seer: the initialiser of `apiProfileResponse` (TODO above)");
        // rust: let mut selected_profile: Option<compat::Shared<ApiProfile>> = api_profile_response.clone().and_then(|__a0| ApiProfileResponse::get_selected_profile(&__a0));
        // rust: // TODO(fabric2seer): `.gameProfile(..)` on a value of unknown type
        // rust: //   java: Optional<GameProfile> gameProfile = loadedData.gameProfile();
        // rust: let mut game_profile: Option<GameProfile> = todo!("fabric2seer: the initialiser of `gameProfile` (TODO above)");
        // rust: let mut loaded_skyblock_profile: bool = api_profile_response.is_some();
        // rust: let mut has_skyblock_profile: bool = selected_profile.is_some();
        // rust: let mut loaded_game_profile: bool = game_profile.is_some();
        // rust: if !loaded_skyblock_profile {
        // rust: return ErrorProfileViewerScreen::new("Failed to load Skyblock profiles.".to_string());
        // rust: } else if !has_skyblock_profile {
        // rust: return ErrorProfileViewerScreen::new("This user has no Skyblock profiles.".to_string());
        // rust: } else if !loaded_game_profile {
        // rust: return ErrorProfileViewerScreen::new("Player not found.".to_string());
        // rust: } else {
        // rust: let mut member: compat::Shared<ProfileMember> = selected_profile.clone().unwrap().lock().borrow().members.get(&game_profile.clone().unwrap().id()).cloned().unwrap();
        // rust: logger().info(&format!("[Skyblocker Profile Viewer] Successfully loaded the profile for {}!", name));
        // rust: // TODO(fabric2seer): `.leaderboards(..)` on a value of unknown type
        // rust: //   java: return new ProfileViewerScreen(apiProfileResponse.get(), selectedProfile.get(), gameProfile.get(), member, loadedData.leaderboards());
        // rust: }
        // rust: }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)))).then_accept_async(__q0, __q1) }).exceptionally_async(__q0, __q1) };
        let mut data_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = todo!("fabric2seer: the initialiser of `dataFuture` (TODO above)");
        return Some(LoadingProfileViewerScreen::new(name.to_string()));
    }

    /// java: private static CompletableFuture<LoadedData> loadData(String name)
    pub fn load_data(name: &str) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<LoadedData> {
        let minecraft = compat::minecraft();
        return CompletableFuture::supply_async(move || Ok((|| {
            let mut uuid: String = ApiUtils::name2_uuid(name);
            if uuid.is_empty() {
                panic!("IllegalStateException: {}", "Invalid username");
            }
            // TODO(fabric2seer): `.map(..)` on a value of unknown type
            //   java: CompletableFuture<Optional<ApiProfileResponse>> skyblockProfileFuture = ProfileUtils.fetchFullProfileByUuid(uuid)
            //   java: 					.thenApply(Optional::ofNullable)
            //   java: 					.thenApply(opt -> opt.map(json -> GSON.fromJson(json, ApiProfileResponse.class)));
            let mut skyblock_profile_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<ApiProfileResponse>>> = todo!("fabric2seer: the initialiser of `skyblockProfileFuture` (TODO above)");
            let mut resolvable_profile: ResolvableProfile = ResolvableProfile::create_unresolved_id(undashed_uuid::from_string(&uuid).unwrap()).unwrap();
            // TODO(fabric2seer): `.map(..)` on a value of unknown type
            //   java: CompletableFuture<Optional<GameProfile>> gameProfileFuture = minecraft.playerSkinRenderCache().lookup(resolvableProfile)
            //   java: 					.thenApply(optional -> optional.map(PlayerSkinRenderCache.RenderInfo::gameProfile));
            let mut game_profile_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<GameProfile>> = todo!("fabric2seer: the initialiser of `gameProfileFuture` (TODO above)");
            // TODO(fabric2seer): `.map(..)` on a value of unknown type
            //   java: CompletableFuture<Map<String, Integer>> leaderboards = skyblockProfileFuture
            //   java: 					.thenCompose(apiProfileResponse -> apiProfileResponse.map(ApiProfileResponse::getSelectedProfile)
            //   java: 							.map(profile -> EliteLeaderboards.fetchPlayerLeaderboards(uuid, profile.profileId))
            //   java: 							.orElse(CompletableFuture.completedFuture(Map.of()))
            //   java: 							);
            let mut leaderboards: mcrust::java::util::concurrent::completable_future::CompletableFuture<std::collections::HashMap<String, i32>> = todo!("fabric2seer: the initialiser of `leaderboards` (TODO above)");
            return LoadedData::new(skyblock_profile_future.join().unwrap(), leaderboards.join().unwrap(), game_profile_future.join().unwrap());
        })()), compat::executor);
    }

}

/// java: private record LoadedData(Optional<ApiProfileResponse> apiProfileResponse, Map<String, Integer> leaderboards, Optional<GameProfile> gameProfile)
#[derive(Clone, Debug)]
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
