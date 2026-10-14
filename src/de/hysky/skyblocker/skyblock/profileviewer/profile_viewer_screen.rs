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
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
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
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
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
pub static COLLECTIONS: LazyLock<Mutex<std::collections::HashMap<String, Vec<Collection>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
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
    /// (an array: an element is null until its page is built)
    pub profile_viewer_pages: Vec<Option<compat::Shared<dyn ProfileViewerPage>>>,
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
            profile_viewer_pages: (0..((PAGE_NAMES.len() as i32)) as usize).map(|_| None).collect::<Vec<_>>(),
            profile_viewer_nav_buttons: Vec::new(),
            entity: None,
            text_widget: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // java: fetchPlayerData(username).thenRun(this::initialisePagesAndWidgets).exceptionally(err -> {
        // java: 			LOGGER.error("[Skyblocker Profile Viewer] An error occurred while fetching player data!", err);
        // java: 			errorMessage = "Unable to get player data!";
        // java: 			profileNotFound = true;
        // java: 			return null;
        // java: 		});
        // (the stages touch this screen, so they continue on the render thread - `compat::LocalFuture`)
        {
            let (__this_r, __this_e) = (__this.clone(), __this.clone());
            let _ = ProfileViewerScreen::fetch_player_data(&__this, &username)
                .then_apply_async(move |()| ProfileViewerScreen::initialise_pages_and_widgets(&__this_r))
                .exceptionally_async(move |err: String| {
                    logger().error(&format!("[Skyblocker Profile Viewer] An error occurred while fetching player data! {}", err));
                    compat::write(&__this_e, |__t| {
                        __t.error_message = "Unable to get player data!".to_string();
                        __t.profile_not_found = true;
                    });
                });
        }
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
                // java: LiteralArgumentBuilder<FabricClientCommandSource> literalArgumentBuilder = ClientCommands.literal("pv")
                // java: 					.then(ClientCommands.argument("username", StringArgumentType.string())
                // java: 							.suggests((source, builder) -> SharedSuggestionProvider.suggest(getPlayerSuggestions(source.getSource()), builder))
                // java: 							.executes(Scheduler.queueOpenScreenFactoryCommand(context -> new ProfileViewerScreen(StringArgumentType.getString(context, "username"))))
                // java: 					)
                // java: 					.executes(Scheduler.queueOpenScreenCommand(() -> new ProfileViewerScreen(Minecraft.getInstance().getUser().getName())));
                let mut literal_argument_builder: LiteralArgumentBuilder<client_command_registration_callback::FabricClientCommandSource> = client_command_registration_callback::literal("pv");
                let mut username_argument = client_command_registration_callback::argument("username", Arc::new(StringArgumentType::string()));
                let suggestions: mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<client_command_registration_callback::FabricClientCommandSource> = Arc::new(|source: &CommandContext<client_command_registration_callback::FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
                    Ok(shared_suggestion_provider::suggest(&ProfileViewerScreen::get_player_suggestions(source.get_source()), builder))
                });
                username_argument.suggests(Some(suggestions));
                username_argument.executes(SchedulerBase::queue_open_screen_factory_command(Arc::new(|context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    Box::new(compat::ScreenAdapter::new(ProfileViewerScreen::new(StringArgumentType::get_string(context, "username")))) as Box<dyn ModScreen>
                })));
                literal_argument_builder.then(&username_argument);
                literal_argument_builder.executes(SchedulerBase::queue_open_screen_command(Arc::new(|| {
                    let __name: String = compat::minecraft().get_user().get_name().to_string();
                    Box::new(compat::ScreenAdapter::new(ProfileViewerScreen::new(__name))) as Box<dyn ModScreen>
                })));
                // java: dispatcher.register(literalArgumentBuilder);
                dispatcher.dispatcher().register(&literal_argument_builder);
                // java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(literalArgumentBuilder));
                let mut namespace_literal = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                namespace_literal.then(&literal_argument_builder);
                dispatcher.dispatcher().register(&namespace_literal);
            },
        ));
    }

    /// java: private static void fetchCollectionsData()
    pub fn fetch_collections_data() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // java: JsonObject jsonObject = JsonParser.parseString(Http.sendGetRequest(HYPIXEL_COLLECTIONS)).getAsJsonObject();
                let mut json_object: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&serde_json::from_str::<serde_json::Value>(&crate::de::hysky::skyblocker::utils::http::Http::send_get_request(HYPIXEL_COLLECTIONS)).expect("JsonSyntaxException"));
                compat::set_state(&COLLECTIONS, Collection::parse(&json_object));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to fetch collections data {}", compat::panic_message(&e)));
            }
        })(), compat::executor);
    }

    /// java: public static Map<String, List<Collection>> getCollections()
    pub fn get_collections() -> std::collections::HashMap<String, Vec<Collection>> {
        return compat::get_state(&COLLECTIONS);
    }

    /// java: private static String[] getPlayerSuggestions(FabricClientCommandSource source)
    pub fn get_player_suggestions(source: &client_command_registration_callback::FabricClientCommandSource) -> Vec<String> {
        // java: return source.getOnlinePlayerNames().stream().filter(playerName -> playerName.matches("[A-Za-z0-9_]+")).toArray(String[]::new);
        // (`FabricClientCommandSource` is the port's `Arc<ClientSuggestionProvider>`)
        use mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider as _;
        return source.get_online_player_names().into_iter().filter(|player_name| compat::regex::Pattern::matches("[A-Za-z0-9_]+", player_name)).collect::<Vec<_>>();
    }

    /// java: private void initialisePagesAndWidgets()
    pub fn initialise_pages_and_widgets(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.profile_not_found) {
            return;
        }
        { let __v = Some(ProfileViewerTextWidget::new(compat::read(__this, |__t| __t.hypixel_profile.clone()).unwrap(), compat::read(__this, |__t| __t.player_profile.clone()).unwrap())); compat::write(__this, |__t| __t.text_widget = __v); }
        // java: CompletableFuture.runAsync(() -> profileViewerPages[i] = new XPage(..)) - the pages are the mod's objects
        // (not `Send`): each is built by a task on the render thread (`LocalFuture::supply_local`), as in profileviewer2
        let hypixel_profile: serde_json::Map<String, serde_json::Value> = compat::read(__this, |__t| __t.hypixel_profile.clone()).expect("NullPointerException");
        let player_profile: serde_json::Map<String, serde_json::Value> = compat::read(__this, |__t| __t.player_profile.clone()).expect("NullPointerException");
        let (__this_1, __h1, __p1) = (__this.clone(), hypixel_profile.clone(), player_profile.clone());
        let skills_future: compat::LocalFuture<()> = compat::LocalFuture::supply_local(move || {
            let __page = SkillsPage::new(__h1, __p1) as compat::Shared<dyn ProfileViewerPage>;
            compat::write(&__this_1, |__t| __t.profile_viewer_pages[0] = Some(__page));
        });
        let (__this_2, __p2) = (__this.clone(), player_profile.clone());
        let slayers_future: compat::LocalFuture<()> = compat::LocalFuture::supply_local(move || {
            let __page = SlayersPage::new(__p2) as compat::Shared<dyn ProfileViewerPage>;
            compat::write(&__this_2, |__t| __t.profile_viewer_pages[1] = Some(__page));
        });
        let (__this_3, __p3) = (__this.clone(), player_profile.clone());
        let dungeons_future: compat::LocalFuture<()> = compat::LocalFuture::supply_local(move || {
            let __page = DungeonsPage::new(__p3) as compat::Shared<dyn ProfileViewerPage>;
            compat::write(&__this_3, |__t| __t.profile_viewer_pages[2] = Some(__page));
        });
        let (__this_4, __p4) = (__this.clone(), player_profile.clone());
        let inventories_future: compat::LocalFuture<()> = compat::LocalFuture::supply_local(move || {
            let __page = InventoryPage::new(__p4) as compat::Shared<dyn ProfileViewerPage>;
            compat::write(&__this_4, |__t| __t.profile_viewer_pages[3] = Some(__page));
        });
        let (__this_5, __h5, __p5) = (__this.clone(), hypixel_profile, player_profile);
        let collections_future: compat::LocalFuture<()> = compat::LocalFuture::supply_local(move || {
            let __page = CollectionsPage::new(__h5, __p5) as compat::Shared<dyn ProfileViewerPage>;
            compat::write(&__this_5, |__t| __t.profile_viewer_pages[4] = Some(__page));
        });
        // java: CompletableFuture.allOf(skillsFuture, slayersFuture, dungeonsFuture, inventoriesFuture, collectionsFuture)
        let all_of: compat::LocalFuture<()> = skills_future
            .then_combine_async(&slayers_future, |_, _| ())
            .then_combine_async(&dungeons_future, |_, _| ())
            .then_combine_async(&inventories_future, |_, _| ())
            .then_combine_async(&collections_future, |_, _| ());
        let __this_7 = __this.clone();
        // java: .thenRunAsync(this::rebuildWidgets, Minecraft.getInstance())
        let _ = all_of
            .then_accept_async(move |()| {
                // rust: port gap (GAPS_batchC #C1) - `Screen.rebuildWidgets()` from a task: the port's `rebuild_widgets`
                // needs the game's `Screen` half, which is lent to the object only inside the screen's own methods
                // (the page buttons join the screen at its next `init`, a resize)
                // rust: __this_6.get().lock().borrow_mut().rebuild_widgets();
            })
            // java: .exceptionally(err -> {..})
            .exceptionally_async(move |err| {
                logger().error(&format!("[Skyblocker Profile Viewer] An error occurred while initializing widgets! {}", err));
                { let __v = "Unable to process player data!".to_string(); compat::write(&__this_7, |__t| __t.error_message = __v); }
                { let __v = true; compat::write(&__this_7, |__t| __t.profile_not_found = __v); }
                // java: return null
            });
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
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
            // rust: port gap (GAPS_batchC #C2) - `button.extractRenderState(graphics, ..)` on a widget OBJECT: the port's
            // widget is drawn by the screen it was added to (`addRenderableWidget` in `init`, which the game draws)
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
            __this.lock().borrow().text_widget.as_ref().unwrap().extract_render_state(graphics, __h0, __h1, __h2, mouse_x, mouse_y);
        }
        ProfileViewerScreen::extract_player_entity(__this, graphics, (if compat::read(__this, |__t| __t.player_name.clone()).is_some() { compat::read(__this, |__t| __t.player_name.clone()) } else { Some("Loading...".to_string()) }).as_deref().unwrap(), root_x, root_y, mouse_x, mouse_y);
        // java: if (profileViewerPages[activePage] != null)
        let __active = compat::read(__this, |__t| __t.active_page) as usize;
        if let Some(__page) = compat::read(__this, |__t| __t.profile_viewer_pages[__active].clone()) {
            ({ let __f = compat::dispatch(&__page, |__t| __t.mark_widgets_as_visible()); __f(&__page) });
            ({ let __f = compat::dispatch(&__page, |__t| __t.extract_render_state()); __f(&__page, graphics, mouse_x, mouse_y, delta, root_x.wrapping_add(93), root_y.wrapping_add(7)) });
        } else {
            // java: graphics.centeredText(font, profileNotFound ? errorMessage : "Loading...", rootX + 200, rootY + 80, Color.WHITE.getRGB());
            let __text: String = if compat::read(__this, |__t| __t.profile_not_found) { compat::read(__this, |__t| __t.error_message.clone()) } else { "Loading...".to_string() };
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__text, root_x.wrapping_add(200), root_y.wrapping_add(80), mcrust::java::awt::color::white().get_rgb());
        }
    }

    /// java: private void extractPlayerEntity(GuiGraphicsExtractor graphics, String username, int rootX, int rootY, int mouseX, int mouseY)
    pub fn extract_player_entity(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, username: &str, mut root_x: i32, mut root_y: i32, mut mouse_x: i32, mut mouse_y: i32) {
        if compat::read(__this, |__t| __t.entity.is_some()) {
            // java: extractEntityInInventoryFollowsMouse(graphics, rootX + 9, rootY + 16, rootX + 89, rootY + 124, 42, 0.0625f, mouseX, mouseY, entity);
            // (`InventoryScreen.extractEntityInInventoryFollowsMouse`, inherited static; the port's takes the frame's
            // dispatcher, light and item models as `AbstractMountInventoryScreen:42` hands them)
            let client = compat::minecraft();
            if let Some(dispatcher) = client.game_renderer.get_entity_render_dispatcher() {
                let level = mcrust::mcrust::modding::game::level();
                use mcrust::net::minecraft::client::renderer::item::item_model_resolver::{ItemModelResolver, ItemModelSource};
                let baked_models = mcrust::net::minecraft::client::renderer::game_renderer::baked();
                let resolver = baked_models.as_ref().map(|models| ItemModelResolver::new(models.item_models.clone() as Arc<dyn ItemModelSource + Send + Sync>));
                let hud_is_hidden: bool = client.gui.hud.is_hidden;
                // the entity is this screen's own (not in the level); borrowed for the one draw, nothing here reaches the screen
                let __guard = __this.lock();
                let __screen = __guard.borrow();
                if let Some(entity) = __screen.entity.as_ref() {
                    mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::extract_entity_in_inventory_follows_mouse(
                        root_x.wrapping_add(9),
                        root_y.wrapping_add(16),
                        root_x.wrapping_add(89),
                        root_y.wrapping_add(124),
                        42,
                        0.0625f32,
                        mouse_x as f32,
                        mouse_y as f32,
                        dispatcher,
                        entity,
                        &|layer, pos| match level.as_deref() {
                            Some(level) => level.get_brightness(layer, pos),
                            None => 0,
                        },
                        resolver.as_ref(),
                        level.as_deref(),
                        hud_is_hidden,
                    );
                }
            }
        }

        // java: graphics.centeredText(font, username.length() > 15 ? username.substring(0, 15) : username, rootX + 47, rootY + 14, Color.WHITE.getRGB());
        let __name: String = if compat::str_len(username) > 15 { compat::substring2(username, 0, 15) } else { username.to_string() };
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__name, root_x.wrapping_add(47), root_y.wrapping_add(14), mcrust::java::awt::color::white().get_rgb());
    }

    /// java: private CompletableFuture<Void> fetchPlayerData(String username)
    ///
    /// Both halves write this screen's fields, which are the mod's objects (not `Send`): the network half runs where
    /// Java runs it (the fetch future's worker / `SkyblockerMod.VIRTUAL_THREAD_EXECUTOR`), and the field writes run on
    /// the render thread (`game::execute`) in the order Java makes them, each half then completing a
    /// `compat::LocalFuture` (batch B pattern 27). `CompletableFuture.allOf(profileFuture, playerFuture)` is their
    /// `then_combine_async`.
    pub fn fetch_player_data(__this: &compat::Shared<Self>, username: &str) -> compat::LocalFuture<()> {
        let username: String = username.to_string();
        let render_thread: mcrust::java::util::concurrent::completable_future::Executor = Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t));

        // java: CompletableFuture<Void> profileFuture = ProfileUtils.fetchFullProfile(username).thenAccept(profiles -> {
        // java: 	try {
        // java: 		Optional<JsonObject> selectedProfile = profiles.getAsJsonArray("profiles").asList().stream()
        // java: 				.map(JsonElement::getAsJsonObject)
        // java: 				.filter(profile -> profile.getAsJsonPrimitive("selected").getAsBoolean())
        // java: 				.findFirst();
        // java: 		if (selectedProfile.isPresent()) {
        // java: 			this.hypixelProfile = selectedProfile.get();
        // java: 			this.playerProfile = hypixelProfile.getAsJsonObject("members").get(ApiUtils.name2Uuid(username)).getAsJsonObject();
        // java: 		}
        // java: 	} catch (Exception e) {
        // java: 		this.errorMessage = ApiAuthentication.getToken() == null ? "Invalid Skyblocker token" : "Skyblock profile not found";
        // java: 		this.profileNotFound = true;
        // java: 		LOGGER.warn("[Skyblocker Profile Viewer] Error while looking for profile", e);
        // java: 	}
        // java: });
        let profile_future: compat::LocalFuture<()> = compat::LocalFuture::new();
        {
            let __name_1 = username.clone();
            // the action's reads and its `name2Uuid` request, on the thread that completes the fetch:
            // (the selected profile, the member, the exception the try block caught)
            let read = ProfileUtils::fetch_full_profile(&username).expect("fetchFullProfile").then_apply(move |profiles: &Option<serde_json::Map<String, serde_json::Value>>| {
                let mut selected: Option<serde_json::Map<String, serde_json::Value>> = None;
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let profiles = profiles.as_ref().expect("NullPointerException: profiles");
                    let selected_profile: Option<serde_json::Map<String, serde_json::Value>> = compat::json::get_as_json_array_of(profiles, "profiles").iter()
                        .map(|profile| compat::json::get_as_json_object(profile))
                        .find(|profile| compat::json::get_as_boolean(&compat::json::get_as_json_primitive_of(profile, "selected")));
                    if let Some(selected_profile) = selected_profile {
                        selected = Some(selected_profile.clone());
                        let member: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&compat::json::get_as_json_object_of(&selected_profile, "members").get(&ApiUtils::name2_uuid(&__name_1)).cloned().expect("NullPointerException: member"));
                        return Some(member);
                    }
                    None
                }));
                match __result {
                    Ok(member) => (selected, member, None),
                    Err(e) => (selected, None, Some(compat::panic_message(&e))),
                }
            });
            // the writes, on the render thread
            let __this_1 = compat::RenderThread::new(__this.clone());
            let __done_1 = compat::RenderThread::new(profile_future.clone());
            let _ = read.when_complete_async(move |outcome| {
                let (__this_1, __done_1) = (__this_1.get(), __done_1.get());
                match outcome {
                    Ok((selected, member, error)) => {
                        if let Some(selected) = selected {
                            // java: this.hypixelProfile = selectedProfile.get();
                            { let __v = Some(selected.clone()); compat::write(__this_1, |__t| __t.hypixel_profile = __v); }
                        }
                        if let Some(member) = member {
                            // java: this.playerProfile = ..
                            { let __v = Some(member.clone()); compat::write(__this_1, |__t| __t.player_profile = __v); }
                        }
                        if let Some(error) = error {
                            { let __v = (if ApiAuthentication::get_token().is_none() { "Invalid Skyblocker token" } else { "Skyblock profile not found" }).to_string(); compat::write(__this_1, |__t| __t.error_message = __v); }
                            { let __v = true; compat::write(__this_1, |__t| __t.profile_not_found = __v); }
                            logger().warn(&format!("[Skyblocker Profile Viewer] Error while looking for profile {}", error));
                        }
                        __done_1.complete(());
                    }
                    Err(exception) => {
                        __done_1.complete_exceptionally(exception.to_string());
                    }
                }
            }, render_thread.clone());
        }

        // java: CompletableFuture<Void> playerFuture = CompletableFuture.runAsync(() -> {
        // java: 	String stringifiedUuid = ApiUtils.name2Uuid(username);
        // java: 	if (stringifiedUuid.isEmpty()) {
        // java: 		this.playerName = "User not found";
        // java: 		this.errorMessage = "Player UUID not found";
        // java: 		this.profileNotFound = true;
        // java: 	}
        // java: 	UUID uuid = UndashedUuid.fromStringLenient(stringifiedUuid);
        // java: 	Minecraft.getInstance().playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(uuid)).thenAccept(entry -> {
        // java: 		this.playerName = entry.get().gameProfile().name();
        // java: 		entity = new RemotePlayer(Minecraft.getInstance().level, entry.get().gameProfile()) {
        // java: 			@Override public PlayerSkin getSkin() { PlayerInfo playerListEntry = new PlayerInfo(entry.get().gameProfile(), false); return playerListEntry.getSkin(); }
        // java: 			@Override public boolean isModelPartShown(PlayerModelPart modelPart) { return !(modelPart.equals(PlayerModelPart.CAPE)); }
        // java: 			@Override public boolean isInvisibleTo(Player player) { return true; }
        // java: 		};
        // java: 		entity.setCustomNameVisible(false);
        // java: 		entity.setId(EntityUtils.PLACEHOLDER_ID);
        // java: 	}).exceptionally(_ -> {
        // java: 		this.playerName = "User not found";
        // java: 		this.errorMessage = "Player skin not found";
        // java: 		this.profileNotFound = true;
        // java: 		return null;
        // java: 	}).join();
        // java: }, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        let player_future: compat::LocalFuture<()> = compat::LocalFuture::new();
        {
            let __name_3 = username.clone();
            let __this_3 = compat::RenderThread::new(__this.clone());
            let __done_3 = compat::RenderThread::new(player_future.clone());
            let __render_thread = render_thread.clone();
            let _ = CompletableFuture::<()>::run_async(move || {
                // on the worker: the request and the parse (`fromStringLenient` throws on a bad id)
                let mut uuid_not_found: bool = false;
                let parsed: Result<Uuid, String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut stringified_uuid: String = ApiUtils::name2_uuid(&__name_3);
                    if stringified_uuid.is_empty() {
                        uuid_not_found = true;
                    }
                    undashed_uuid::from_string_lenient(&stringified_uuid).expect("IllegalArgumentException: Invalid UUID string")
                })).map_err(|e| compat::panic_message(&e));
                let __render_thread_2 = __render_thread.clone();
                // the rest on the render thread, where the screen's fields and the skin cache live
                (__render_thread)(Box::new(move || {
                    let (__this_3_ref, __done_3_ref) = (__this_3.get(), __done_3.get());
                    if uuid_not_found {
                        // "Player not found" doesn't fit on the screen lol
                        { let __v = Some("User not found".to_string()); compat::write(__this_3_ref, |__t| __t.player_name = __v); }
                        { let __v = "Player UUID not found".to_string(); compat::write(__this_3_ref, |__t| __t.error_message = __v); }
                        { let __v = true; compat::write(__this_3_ref, |__t| __t.profile_not_found = __v); }
                    }
                    let uuid: Uuid = match parsed {
                        Ok(uuid) => uuid,
                        Err(exception) => {
                            __done_3_ref.complete_exceptionally(exception);
                            return;
                        }
                    };
                    // java: Minecraft.getInstance().playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(uuid)).thenAccept(..).exceptionally(..).join();
                    // (the worker's `join()` waits for this lookup; its stages write the screen, so they run here)
                    let lookup = compat::minecraft().game_renderer.player_skin_render_cache.as_ref().zip(ResolvableProfile::create_unresolved_id(uuid).ok()).map(|(cache, profile)| cache.lookup(&profile));
                    let Some(lookup) = lookup else {
                        __done_3_ref.complete_exceptionally("Minecraft.playerSkinRenderCache()".to_string());
                        return;
                    };
                    let __this_4 = compat::RenderThread::new(__this_3_ref.clone());
                    let __done_4 = compat::RenderThread::new(__done_3_ref.clone());
                    let _ = lookup.when_complete_async(move |outcome| {
                        let (__this_4, __done_4) = (__this_4.get(), __done_4.get());
                        // `entry.get()` on an empty Optional throws, which the exceptionally arm catches, as a failed lookup does
                        match outcome {
                            Ok(Some(entry)) => {
                                let game_profile = entry.game_profile().clone();
                                // java: this.playerName = entry.get().gameProfile().name();
                                { let __v = Some(game_profile.name().to_string()); compat::write(__this_4, |__t| __t.player_name = __v); }
                                // java: entity = new RemotePlayer(Minecraft.getInstance().level, entry.get().gameProfile()) {..};
                                // rust: port gap (GAPS_batchB #9) - the anonymous subclass's overrides (`getSkin` from a fresh
                                // `PlayerInfo`, `isModelPartShown` hiding the CAPE, `isInvisibleTo` true) have no home: the port's
                                // entity classes take no mod subclass, so a plain `RemotePlayer` stands in (as `ArmorTab`'s).
                                // Its dimensions are the player's (`EntityType.PLAYER`, 0.6 x 1.8).
                                let mut entity = RemotePlayer::new(mcrust::net::minecraft::world::entity::entity_dimensions::scalable(0.6, 1.8), &game_profile);
                                {
                                    use mcrust::net::minecraft::world::entity::entity::Entity as _;
                                    // java: entity.setCustomNameVisible(false);
                                    entity.base_mut().set_custom_name_visible(false);
                                    // java: entity.setId(EntityUtils.PLACEHOLDER_ID);
                                    entity.base_mut().set_id(entity_utils::PLACEHOLDER_ID);
                                }
                                compat::write(__this_4, |__t| __t.entity = Some(entity));
                            }
                            _ => {
                                // java: .exceptionally(_ -> { this.playerName = "User not found"; this.errorMessage = "Player skin not found"; this.profileNotFound = true; return null; })
                                { let __v = Some("User not found".to_string()); compat::write(__this_4, |__t| __t.player_name = __v); }
                                { let __v = "Player skin not found".to_string(); compat::write(__this_4, |__t| __t.error_message = __v); }
                                { let __v = true; compat::write(__this_4, |__t| __t.profile_not_found = __v); }
                            }
                        }
                        __done_4.complete(());
                    }, __render_thread_2);
                }));
            }, compat::executor);
        }
        // java: return CompletableFuture.allOf(profileFuture, playerFuture);
        return profile_future.then_combine_async(&player_future, |_, _| ());
    }

    /// java: public void onNavButtonClick(ProfileViewerNavButton clickedButton)
    pub fn on_nav_button_click(__this: &compat::Shared<Self>, clicked_button: &compat::Shared<ProfileViewerNavButton>) {
        // java: if (profileViewerPages[activePage] != null) profileViewerPages[activePage].markWidgetsAsInvisible();
        let __active = compat::read(__this, |__t| __t.active_page) as usize;
        if let Some(__page) = compat::read(__this, |__t| __t.profile_viewer_pages[__active].clone()) {
            ({ let __f = compat::dispatch(&__page, |__t| __t.mark_widgets_as_invisible()); __f(&__page) });
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
            // java: if (profileViewerPage != null && profileViewerPage.getButtons() != null) - a page not built yet is null
            let Some(profile_viewer_page) = profile_viewer_page else { continue; };
            // (`getButtons()` builds a fresh list of the same widget objects each call: asked once)
            let __buttons: Option<Vec<AbstractWidget>> = ({ let __f = compat::dispatch(&profile_viewer_page, |__t| __t.get_buttons()); __f(&profile_viewer_page) });
            if let Some(__buttons) = __buttons {
                for button in __buttons {
                    // java: if (button != null) addRenderableWidget(button); - the list holds the widgets themselves
                    __this.lock().borrow_mut().add_renderable_widget(button);
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
        self.lend_ref(screen, |__h| ProfileViewerScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
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
