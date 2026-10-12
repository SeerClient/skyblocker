//! Ported from de.hysky.skyblocker.skyblock.profileviewer.ProfileViewerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::collections::collections_page::CollectionsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::dungeons::dungeons_page::DungeonsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::inventory_page::InventoryPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_nav_button::ProfileViewerNavButton;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_text_widget::ProfileViewerTextWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer::skills::skills_page::SkillsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::slayers::slayers_page::SlayersPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::collection::Collection;
use crate::de::hysky::skyblocker::utils::api_authentication::ApiAuthentication;
use crate::de::hysky::skyblocker::utils::api_utils::ApiUtils;
use crate::de::hysky::skyblocker::utils::entity_utils;
use crate::de::hysky::skyblocker::utils::profile_utils::ProfileUtils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::util::undashed_uuid;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::java::util::uuid::Uuid;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::player::remote_player::RemotePlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Component TITLE = Component.nullToEmpty("Skyblocker Profile Viewer");
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("Skyblocker Profile Viewer")));
/// java: private static final String HYPIXEL_COLLECTIONS = "https://api.hypixel.net/v2/resources/skyblock/collections";
pub const HYPIXEL_COLLECTIONS: &str = "https://api.hypixel.net/v2/resources/skyblock/collections";
/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/base_plate.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/base_plate.png"));
/// java: private static final int GUI_WIDTH = 322;
pub const GUI_WIDTH: i32 = 322;
/// java: private static final int GUI_HEIGHT = 189;
pub const GUI_HEIGHT: i32 = 189;
/// java: private static Map<String, List<Collection>> collections = Map.of();
pub static COLLECTIONS: LazyLock<Mutex<std::collections::HashMap<String, Vec<Vec<_>>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
/// java: private static final String[] PAGE_NAMES = {"Skills", "Slayers", "Dungeons", "Inventories", "Collections"};
pub static PAGE_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Skills".to_string(), "Slayers".to_string(), "Dungeons".to_string(), "Inventories".to_string(), "Collections".to_string()]);

/// java: public class ProfileViewerScreen extends Screen
pub struct ProfileViewerScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private String playerName;
    pub player_name: Option<String>,
    /// java: private JsonObject hypixelProfile;
    pub hypixel_profile: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private JsonObject playerProfile;
    pub player_profile: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private boolean profileNotFound = false;
    pub profile_not_found: bool,
    /// java: private String errorMessage = "No Profile";
    pub error_message: String,
    /// java: private int activePage = 0;
    pub active_page: i32,
    /// java: private final ProfileViewerPage[] profileViewerPages = new ProfileViewerPage[PAGE_NAMES.length];
    pub profile_viewer_pages: Vec<compat::Shared<dyn ProfileViewerPage>>,
    /// java: private final List<ProfileViewerNavButton> profileViewerNavButtons = new ArrayList<>();
    pub profile_viewer_nav_buttons: Vec<compat::Shared<ProfileViewerNavButton>>,
    /// java: private @Nullable RemotePlayer entity;
    pub entity: Option<RemotePlayer>,
    /// java: private ProfileViewerTextWidget textWidget;
    pub text_widget: Option<ProfileViewerTextWidget>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ProfileViewerScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileViewerScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileViewerScreen {
    /// java: public ProfileViewerScreen(String username)
    pub fn new(mut username: String) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(TITLE.get_mut().get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            player_name: None,
            hypixel_profile: None,
            player_profile: None,
            profile_not_found: false,
            error_message: "No Profile".to_string(),
            active_page: 0,
            profile_viewer_pages: (0..((PAGE_NAMES.len() as i32)) as usize).map(|_| Default::default()).collect::<Vec<_>>(),
            profile_viewer_nav_buttons: Vec::new(),
            entity: None,
            text_widget: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): no overload of `initialisePagesAndWidgets` takes 1 argument(s)
        //   java: fetchPlayerData(username).thenRun(this::initialisePagesAndWidgets).exceptionally(err -> {
        //   java: 			LOGGER.error("[Skyblocker Profile Viewer] An error occurred while fetching player data!", err);
        //   java: 			errorMessage = "Unable to get player data!";
        //   java: 			profileNotFound = true;
        //   java: 			return null;
        //   java: 		});
        for i in 0..(PAGE_NAMES.len() as i32) {
            let __h0 = ProfileViewerNavButton::new(__this.clone(), PAGE_NAMES[(i) as usize].clone(), i, (i == 0));
            __this.lock().borrow_mut().profile_viewer_nav_buttons.push(__h0);
        }
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        ProfileViewerScreen::fetch_collections_data();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command builder outside dispatcher.register(..)
                //   java: LiteralArgumentBuilder<FabricClientCommandSource> literalArgumentBuilder = ClientCommands.literal("pv")
                //   java: 					.then(ClientCommands.argument("username", StringArgumentType.string())
                //   java: 							.suggests((source, builder) -> SharedSuggestionProvider.suggest(getPlayerSuggestions(source.getSource()), builder))
                //   java: 							.executes(Scheduler.queueOpenScreenFactoryCommand(context -> new ProfileViewerScreen(StringArgumentType.getString(context, "username"))))
                //   java: 					)
                //   java: 					.executes(Scheduler.queueOpenScreenCommand(() -> new ProfileViewerScreen(Minecraft.getInstance().getUser().getName())));
                let mut literal_argument_builder: LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> = todo!("fabric2seer: the initialiser of `literalArgumentBuilder` (TODO above)");
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literalArgumentBuilder);
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(literalArgumentBuilder));
            },
        ));
    }

    /// java: private static void fetchCollectionsData()
    pub fn fetch_collections_data() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: JsonObject jsonObject = JsonParser.parseString(Http.sendGetRequest(HYPIXEL_COLLECTIONS)).getAsJsonObject();
                let mut json_object: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `jsonObject` (TODO above)");
                compat::set_state(&COLLECTIONS, Collection::parse(&json_object));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to fetch collections data"));
            }
        })(), compat::executor);
    }

    /// java: public static Map<String, List<Collection>> getCollections()
    pub fn get_collections() -> std::collections::HashMap<String, Vec<Vec<_>>> {
        return compat::get_state(&COLLECTIONS);
    }

    /// java: private static String[] getPlayerSuggestions(FabricClientCommandSource source)
    pub fn get_player_suggestions(source: &crate::unresolved::FabricClientCommandSource) -> Vec<String> {
        // TODO(fabric2seer): `.getOnlinePlayerNames(..)` on a FabricClientCommandSource
        //   java: return source.getOnlinePlayerNames().stream().filter(playerName -> playerName.matches("[A-Za-z0-9_]+")).toArray(String[]::new);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void initialisePagesAndWidgets()
    pub fn initialise_pages_and_widgets(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.profile_not_found) {
            return;
        }
        { let __v = Some(ProfileViewerTextWidget::new(compat::read(__this, |__t| __t.hypixel_profile.clone()).unwrap(), compat::read(__this, |__t| __t.player_profile.clone()).unwrap())); compat::write(__this, |__t| __t.text_widget = __v); }
        let __this_1 = compat::RenderThread::new(__this.clone());
        let mut skills_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| { __this_1.get().lock().borrow_mut().profile_viewer_pages[(0) as usize] = (SkillsPage::new(compat::read(__this_1.get(), |__t| __t.hypixel_profile.clone()).unwrap(), compat::read(__this_1.get(), |__t| __t.player_profile.clone()).unwrap()) as compat::Shared<dyn ProfileViewerPage>); })(), compat::executor);
        let __this_2 = compat::RenderThread::new(__this.clone());
        let mut slayers_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| { __this_2.get().lock().borrow_mut().profile_viewer_pages[(1) as usize] = (SlayersPage::new(compat::read(__this_2.get(), |__t| __t.player_profile.clone()).unwrap()) as compat::Shared<dyn ProfileViewerPage>); })(), compat::executor);
        let __this_3 = compat::RenderThread::new(__this.clone());
        let mut dungeons_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| { __this_3.get().lock().borrow_mut().profile_viewer_pages[(2) as usize] = (DungeonsPage::new(compat::read(__this_3.get(), |__t| __t.player_profile.clone()).unwrap()) as compat::Shared<dyn ProfileViewerPage>); })(), compat::executor);
        let __this_4 = compat::RenderThread::new(__this.clone());
        let mut inventories_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| { __this_4.get().lock().borrow_mut().profile_viewer_pages[(3) as usize] = (InventoryPage::new(compat::read(__this_4.get(), |__t| __t.player_profile.clone()).unwrap()) as compat::Shared<dyn ProfileViewerPage>); })(), compat::executor);
        let __this_5 = compat::RenderThread::new(__this.clone());
        let mut collections_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| { __this_5.get().lock().borrow_mut().profile_viewer_pages[(4) as usize] = (CollectionsPage::new(compat::read(__this_5.get(), |__t| __t.hypixel_profile.clone()).unwrap(), compat::read(__this_5.get(), |__t| __t.player_profile.clone()).unwrap()) as compat::Shared<dyn ProfileViewerPage>); })(), compat::executor);
        let __this_6 = compat::RenderThread::new(__this.clone());
        let __this_7 = compat::RenderThread::new(__this.clone());
        { let (__q0,) = (|err| {
            logger().error(&format!("[Skyblocker Profile Viewer] An error occurred while initializing widgets!"));
            { let __v = "Unable to process player data!".to_string(); compat::write(__this_7.get(), |__t| __t.error_message = __v); }
            { let __v = true; compat::write(__this_7.get(), |__t| __t.profile_not_found = __v); }
            return Default::default(); // java: return null
        },); (CompletableFuture::all_of(&[skills_future.clone(), slayers_future.clone(), dungeons_future.clone(), inventories_future.clone(), collections_future.clone()])).then_run_async(|__a0| __this_6.get().lock().borrow_mut().rebuild_widgets(&mut __a0), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t))).exceptionally(__q0) };
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        ();
        let mut root_x: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(GUI_WIDTH.wrapping_div(2));
        let mut root_y: i32 = (((compat::read(__this, |__t| __t.height)).wrapping_div(2)).wrapping_sub(GUI_HEIGHT.wrapping_div(2))).wrapping_add(5);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, root_y, ((0) as f32) as f32, ((0) as f32) as f32, GUI_WIDTH, GUI_HEIGHT, GUI_WIDTH, GUI_HEIGHT);
        for button in compat::read(__this, |__t| __t.profile_viewer_nav_buttons.clone()) {
            let __wa1 = (root_x.wrapping_add((ProfileViewerNavButton::get_index(&button)).wrapping_mul(28))).wrapping_add(4);
            let __o1 = button.clone();
            compat::write(&__o1, |__t| __t.set_x(__wa1));
            let __wa2 = root_y.wrapping_sub(28);
            let __o2 = button.clone();
            compat::write(&__o2, |__t| __t.set_y(__wa2));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa3 = graphics.clone();
            // rust: let __wa4 = mouse_x;
            // rust: let __wa5 = mouse_y;
            // rust: let __wa6 = delta;
            // rust: let __o3 = button.clone();
            // rust: compat::write(&__o3, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6));
        }
        if compat::read(__this, |__t| __t.text_widget.clone()).is_some() {
            let __h0 = &compat::font();
            let __h1 = root_x.wrapping_add(8);
            let __h2 = root_y.wrapping_add(120);
            __this.lock().borrow().text_widget.as_ref().unwrap().extract_render_state(&mut graphics, __h0, __h1, __h2, mouse_x, mouse_y);
        }
        ProfileViewerScreen::extract_player_entity(__this, graphics, (if compat::read(__this, |__t| __t.player_name.clone()).is_some() { compat::read(__this, |__t| __t.player_name.clone()) } else { Some("Loading...".to_string()) }).as_deref().unwrap(), root_x, root_y, mouse_x, mouse_y);
        if true /* java: __this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_visible()); __f(__r) });
            ({ let __r = &__this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(__r, &mut graphics, mouse_x, mouse_y, delta, root_x.wrapping_add(93), root_y.wrapping_add(7)) });
        } else {
            // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
            //   java: graphics.centeredText(font, profileNotFound ? errorMessage : "Loading...", rootX + 200, rootY + 80, Color.WHITE.getRGB());
        }
    }

    /// java: private void extractPlayerEntity(GuiGraphicsExtractor graphics, String username, int rootX, int rootY, int mouseX, int mouseY)
    pub fn extract_player_entity(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, username: &str, mut root_x: i32, mut root_y: i32, mut mouse_x: i32, mut mouse_y: i32) {
        if compat::read(__this, |__t| __t.entity.clone()).is_some() {
            // TODO(fabric2seer): `Screen.extractEntityInInventoryFollowsMouse` has no Rust method in the port (looked for `extract_entity_in_inventory_follows_mouse` in Screen and JavaSrc)
            // rust: let __h0 = root_x.wrapping_add(9);
            // rust: let __h1 = root_y.wrapping_add(16);
            // rust: let __h2 = root_x.wrapping_add(89);
            // rust: let __h3 = root_y.wrapping_add(124);
            // rust: let __h4 = &compat::read(__this, |__t| __t.entity.clone()).unwrap();
            // rust: __this.lock().borrow().extract_entity_in_inventory_follows_mouse(graphics, __h0, __h1, __h2, __h3, 42, 0.0625f32, mouse_x, mouse_y, __h4);
        }
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.centeredText(font, username.length() > 15 ? username.substring(0, 15) : username, rootX + 47, rootY + 14, Color.WHITE.getRGB());
    }

    /// java: private CompletableFuture<Void> fetchPlayerData(String username)
    pub fn fetch_player_data(__this: &compat::Shared<Self>, username: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>> {
        // TODO(fabric2seer): `CompletableFuture.thenAccept` has no Rust method in the port (looked for `then_accept` in CompletableFuture and JavaSrc)
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: let mut profile_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = ProfileUtils::fetch_full_profile(username).unwrap().then_accept(&Arc::new(move |profiles| {
        // rust: // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        // rust: let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // rust: // TODO(fabric2seer): `.getAsJsonArray(..)` on a value of unknown type
        // rust: //   java: Optional<JsonObject> selectedProfile = profiles.getAsJsonArray("profiles").asList().stream()
        // rust: //   java: 						.map(JsonElement::getAsJsonObject)
        // rust: //   java: 						.filter(profile -> profile.getAsJsonPrimitive("selected").getAsBoolean())
        // rust: //   java: 						.findFirst();
        // rust: let mut selected_profile: Option<serde_json::Map<String, serde_json::Value>> = todo!("fabric2seer: the initialiser of `selectedProfile` (TODO above)");
        // rust: if selected_profile.is_some() {
        // rust: { let __v = Some(selected_profile.clone().unwrap()); compat::write(__this_1.get(), |__t| __t.hypixel_profile = __v); }
        // rust: { let __v = Some(compat::json::get_as_json_object(&(compat::json::get_as_json_object_of(&__this_1.get().lock().borrow().hypixel_profile.as_ref().unwrap(), "members")).get(&ApiUtils::name2_uuid(username)).cloned().unwrap())); compat::write(__this_1.get(), |__t| __t.player_profile = __v); }
        // rust: }
        // rust: }));
        // rust: if let Err(e) = __result {
        // rust: { let __v = (if ApiAuthentication::get_token().is_none() { "Invalid Skyblocker token" } else { "Skyblock profile not found" }).to_string(); compat::write(__this_1.get(), |__t| __t.error_message = __v); }
        // rust: { let __v = true; compat::write(__this_1.get(), |__t| __t.profile_not_found = __v); }
        // rust: logger().warn(&format!("[Skyblocker Profile Viewer] Error while looking for profile"));
        // rust: }
        // rust: }));
        let mut profile_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = todo!("fabric2seer: the initialiser of `profileFuture` (TODO above)");
        let __this_3 = compat::RenderThread::new(__this.clone());
        let mut player_future: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = CompletableFuture::<()>::run_async(move || (|| {
            let mut stringified_uuid: String = ApiUtils::name2_uuid(username);
            if stringified_uuid.is_empty() {
                { let __v = Some("User not found".to_string()); compat::write(__this_3.get(), |__t| __t.player_name = __v); }
                { let __v = "Player UUID not found".to_string(); compat::write(__this_3.get(), |__t| __t.error_message = __v); }
                { let __v = true; compat::write(__this_3.get(), |__t| __t.profile_not_found = __v); }
            }
            let mut uuid: Uuid = undashed_uuid::from_string_lenient(&stringified_uuid).unwrap();
            // TODO(fabric2seer): `.exceptionally(..)` on a value of unknown type
            //   java: Minecraft.getInstance().playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(uuid)).thenAccept(entry -> {
            //   java: 				this.playerName = entry.get().gameProfile().name();
            //   java: 				entity = new RemotePlayer(Minecraft.getInstance().level, entry.get().gameProfile()) {
            //   java: 					@Override
            //   java: 					public PlayerSkin getSkin() {
            //   java: 						PlayerInfo playerListEntry = new PlayerInfo(entry.get().gameProfile(), false);
            //   java: 						return playerListEntry.getSkin();
            //   java: 					}
            //   java:
            //   java: 					@Override
            //   java: 					public boolean isModelPartShown(PlayerModelPart modelPart) {
            //   java: 						return !(modelPart.equals(PlayerModelPart.CAPE));
            //   java: 					}
            //   java:
            //   java: 					@Override
            //   java: 					public boolean isInvisibleTo(Player player) {
            //   java: 						return true;
            //   java: 					}
            //   java: 				};
            //   java: 				entity.setCustomNameVisible(false);
            //   java: 				entity.setId(EntityUtils.PLACEHOLDER_ID);
            //   java: 			}).exceptionally(_ -> {
            //   java: 				// "Player not found" doesn't fit on the screen lol
            //   java: 				this.playerName = "User not found";
            //   java: 				this.errorMessage = "Player skin not found";
            //   java: 				this.profileNotFound = true;
            //   java: 				return null;
            //   java: 			}).join();
        })(), compat::executor);
        return Some(CompletableFuture::all_of(&[profile_future.clone(), player_future.clone()]));
    }

    /// java: public void onNavButtonClick(ProfileViewerNavButton clickedButton)
    pub fn on_nav_button_click(__this: &compat::Shared<Self>, clicked_button: &compat::Shared<ProfileViewerNavButton>) {
        if true /* java: __this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().profile_viewer_pages[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_invisible()); __f(__r) });
        }
        for button in compat::read(__this, |__t| __t.profile_viewer_nav_buttons.clone()) {
            ProfileViewerNavButton::set_toggled(&button, false);
        }
        { let __v = ProfileViewerNavButton::get_index(clicked_button); compat::write(__this, |__t| __t.active_page = __v); }
        ProfileViewerNavButton::set_toggled(clicked_button, true);
    }

    /// java: @Override public void init()
    pub fn init(__this: &compat::Shared<Self>) {
        for __a0 in compat::read(__this, |__t| __t.profile_viewer_nav_buttons.clone()) {
            { let (__q0,) = (compat::WidgetAdapter::widget(__a0.clone()),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        }
        for profile_viewer_page in compat::read(__this, |__t| __t.profile_viewer_pages.clone()) {
            if true /* java: profile_viewer_page != null: never null here */ && ({ let __f = compat::dispatch(&profile_viewer_page, |__t| __t.get_buttons()); __f(&profile_viewer_page) }).is_some() {
                for button in ({ let __f = compat::dispatch(&profile_viewer_page, |__t| __t.get_buttons()); __f(&profile_viewer_page) }).unwrap().iter() {
                    if true /* java: button != null: never null here */ {
                        { let (__q0,) = (button.clone(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
                    }
                }
            }
        }
    }

}

/// ProfileViewerScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ProfileViewerScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ProfileViewerScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ProfileViewerScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override public void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ProfileViewerScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ProfileViewerScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

}

// NOTE(fabric2seer): extractRenderState, init override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
