//! Ported from de.hysky.skyblocker.config.SkyblockerConfigManager.
//!
//! The config is Dandelion's `ConfigManager` (the `dandelion` crate): it owns the unpatched instance (what the file
//! holds and the config screen edits) and the patched one (`get()`), loads and saves `config/skyblocker.json` through
//! the generated Gson form of `SkyblockerConfig` (same file name, same field names - an existing Skyblocker config is
//! read as it is), and opens the YACL-backed config screen.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::categories::chat_category::ChatCategory;
use crate::de::hysky::skyblocker::config::categories::crimson_isle_category::CrimsonIsleCategory;
use crate::de::hysky::skyblocker::config::categories::debug_category::DebugCategory;
use crate::de::hysky::skyblocker::config::categories::dungeons_category::DungeonsCategory;
use crate::de::hysky::skyblocker::config::categories::event_notifications_category::EventNotificationsCategory;
use crate::de::hysky::skyblocker::config::categories::farming_category::FarmingCategory;
use crate::de::hysky::skyblocker::config::categories::foraging_category::ForagingCategory;
use crate::de::hysky::skyblocker::config::categories::general_category::GeneralCategory;
use crate::de::hysky::skyblocker::config::categories::helper_category::HelperCategory;
use crate::de::hysky::skyblocker::config::categories::hunting_category::HuntingCategory;
use crate::de::hysky::skyblocker::config::categories::mining_category::MiningCategory;
use crate::de::hysky::skyblocker::config::categories::misc_category::MiscCategory;
use crate::de::hysky::skyblocker::config::categories::other_locations_category::OtherLocationsCategory;
use crate::de::hysky::skyblocker::config::categories::quick_navigation_category::QuickNavigationCategory;
use crate::de::hysky::skyblocker::config::categories::slayers_category::SlayersCategory;
use crate::de::hysky::skyblocker::config::categories::ui_and_visuals_category::UIAndVisualsCategory;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer::ConfigDataFixer;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::debug::debug::Debug;
use crate::de::hysky::skyblocker::utils::datafixer::json_helper::JsonHelper;
use crate::entry::skyblocker_mod;
use dandelion::compat::ScreenRef;
use dandelion::net::azureaaron::dandelion::api::config_manager::{self, ConfigManager};
use dandelion::net::azureaaron::dandelion::api::config_type::ConfigType;
use dandelion::net::azureaaron::dandelion::api::dandelion_config_screen::{self, Builder as _, DandelionConfigScreen};
use dandelion::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use dandelion::net::azureaaron::dandelion::api::platform_links::{self, Builder as _};
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;

/// java: public static final int CONFIG_VERSION = 13;
pub const CONFIG_VERSION: i32 = 13;
/// java: private static final String CONFIGS_PACKAGE = "de.hysky.skyblocker.config.configs";
pub const CONFIGS_PACKAGE: &str = "de.hysky.skyblocker.config.configs";
/// java: private static final Path CONFIG_DIR = FabricLoader.getInstance().getConfigDir();
pub static CONFIG_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| mcrust::mcrust::modding::config::config_dir());
/// java: private static final Path CONFIG_FILE = CONFIG_DIR.resolve("skyblocker.json");
pub static CONFIG_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| CONFIG_DIR.get_mut().join("skyblocker.json"));
/// java: private static final ConfigManager<SkyblockerConfig> CONFIG_MANAGER = ConfigManager.create(SkyblockerConfig.class, CONFIG_FILE, UnaryOperator.identity());
pub static CONFIG_MANAGER: compat::MainThread<Arc<dyn ConfigManager<SkyblockerConfig>>> =
    compat::MainThread::new(|| config_manager::create(SkyblockerConfig::__class_of(), CONFIG_FILE.get().clone(), |builder| builder));

/// The value `get()` lends: the patched instance `CONFIG_MANAGER.instance()` answered, and the reference handed out
/// for it.
///
/// Java's `get()` returns the manager's patched object itself, and every one of its ~640 callers reads fields off it
/// (`get().general.x`). Here the manager holds that object behind a `compat::Shared` and REPLACES it (a new object) on
/// every `save` / `load` / `updatePatchedInstance` / `setPatches`, exactly as Java's `updatePatchedInstance` assigns
/// `this.instance = copy`. A patched instance is never changed after it is published (patches are applied before
/// anyone can read it, and every change goes through the unpatched instance), so `get()` lends an immutable copy of
/// it, taken once per published instance and kept for the rest of the run - the copy a Java caller holding an old
/// `get()` result would also keep alive. Its `compat::Shared` members are the patched instance's own objects (a clone
/// of the `Arc`), so a caller writing through one changes the patched instance, as in Java.
static PATCHED_VIEW: compat::MainThread<Option<(compat::Shared<SkyblockerConfig>, &'static SkyblockerConfig)>> = compat::MainThread::new(|| None);
/// The manager's `instance_epoch()` under which `PATCHED_VIEW` was last confirmed current (`u64::MAX`: none -
/// the manager keeps no epoch, or nothing confirmed yet).
static VIEW_EPOCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

/// java: public class SkyblockerConfigManager
#[derive(Clone)]
pub struct SkyblockerConfigManager {}

impl SkyblockerConfigManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static SkyblockerConfig get()
    pub fn get() -> &'static SkyblockerConfig {
        // PERF (port-only, same result): ~hundreds of calls a frame. The manager's instance epoch changes exactly
        // when `instance()` starts answering another object, so while it equals the epoch the lent view was built
        // under, the view is still the published instance's - one atomic load, no `instance()` (a RefCell borrow
        // and an `Arc` clone + drop). The epoch is read BEFORE `instance()`, so a stale number only forces the
        // pointer comparison below again, never a stale view.
        let manager = CONFIG_MANAGER.get();
        let epoch: Option<u64> = manager.instance_epoch();
        if let Some(epoch) = epoch
            && epoch == VIEW_EPOCH.load(std::sync::atomic::Ordering::Relaxed)
            && let Some((_, lent)) = PATCHED_VIEW.get()
        {
            return *lent;
        }
        // java: return CONFIG_MANAGER.instance();
        let instance: compat::Shared<SkyblockerConfig> = manager.instance();
        let view = PATCHED_VIEW.get_mut();
        if let Some((published, lent)) = view {
            if Arc::ptr_eq(published, &instance) {
                VIEW_EPOCH.store(epoch.unwrap_or(u64::MAX), std::sync::atomic::Ordering::Relaxed);
                return *lent;
            }
        }
        let lent: &'static SkyblockerConfig = Box::leak(Box::new(compat::read(&instance, |config| config.clone())));
        *view = Some((instance, lent));
        VIEW_EPOCH.store(epoch.unwrap_or(u64::MAX), std::sync::atomic::Ordering::Relaxed);
        lent
    }

    /// java: protected static SkyblockerConfig getUnpatched()
    ///
    /// The live unpatched instance, LENT to `f` for one call (it is a `compat::Shared`, borrowed per statement).
    /// Every Java caller uses the object at once (`update`'s `action.accept(getUnpatched())`,
    /// `ConfigCommands.registerConfigEntries`).
    pub fn get_unpatched<R>(f: impl FnOnce(&mut SkyblockerConfig) -> R) -> R {
        // java: return CONFIG_MANAGER.unpatchedInstance();
        let unpatched: compat::Shared<SkyblockerConfig> = CONFIG_MANAGER.get().unpatched_instance();
        compat::write(&unpatched, f)
    }

    /// java: public static void init()
    ///
    /// This method is caller sensitive and can only be called by the mod initializer, this is enforced (Java:
    /// `StackWalker.getCallerClass() != SkyblockerMod.class`; here the caller's source file, `#[track_caller]`).
    #[track_caller]
    pub fn init() {
        let caller = std::panic::Location::caller();
        if !caller.file().replace('\\', "/").ends_with("entry/skyblocker_mod.rs") {
            panic!("RuntimeException: {}", "Skyblocker: Called config init from an illegal place!");
        }
        SkyblockerConfigManager::data_fix(&CONFIG_FILE.get().clone(), &CONFIG_DIR.get().join("skyblocker.json.old"));

        CONFIG_MANAGER.get().load();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(configLiteral("config")).then(configLiteral("options")))
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                root.then(&SkyblockerConfigManager::config_literal("config"));
                root.then(&SkyblockerConfigManager::config_literal("options"));
                dispatcher.dispatcher().register(&root);
            },
        ));
        // java: ScreenEvents.AFTER_INIT.register((client, screen, _, _) -> { .. });
        // The listener reads the container screen's `AbstractContainerScreenAccessor` geometry, so it runs where that
        // screen is lent whole: the RETURN of `AbstractContainerScreen#init` (inside `Screen.init(II)`, after its
        // `clearWidgets()`, so the button stays in the screen's widget list as `Screens.getWidgets(screen).add` leaves it).
        let result = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", mcrust::mcrust::modding::hook::At::Return, |ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            let Some(screen) = ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>() else { return };
            // java: if (get().uiAndVisuals.showConfigButton && screen instanceof ContainerScreen genericContainerScreen
            //       && screen.getTitle().getString().equals("SkyBlock Menu"))
            if SkyblockerConfigManager::get().ui_and_visuals.show_config_button
                && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container)
                && screen.screen.get_title() == "SkyBlock Menu"
            {
                //   java: Screens.getWidgets(screen).add(Button
                //   java: 		.builder(Component.literal("🔧"), _ -> client.gui.setScreen(createGUI(screen)))
                //   java: 		.bounds(accessor.getX() + accessor.getImageWidth() - 16, accessor.getY() + 4, 12, 12)
                //   java: 		.tooltip(Tooltip.create(Component.translatable("skyblocker.config.title", Component.translatable("skyblocker.config.title.settings"))))
                //   java: 		.build());
                use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, PressLambda, WidgetKind};
                use mcrust::net::minecraft::network::chat::component::Component as _;
                let (x, y) = (screen.left_pos.wrapping_add(screen.image_width).wrapping_sub(16), screen.top_pos.wrapping_add(4));
                let mut button = AbstractWidget::new(x, y, 12, 12, "🔧".to_string(), WidgetKind::Button, OnPress::Lambda(PressLambda::new(|_, _| {
                    // `createGUI(screen)`: the container screen that is up is the config screen's parent
                    SkyblockerConfigManager::create_gui(Some(ScreenRef::Current)).set();
                })));
                let tooltip = component::translatable_args("skyblocker.config.title", vec![FormattedText::from(component::translatable("skyblocker.config.title.settings"))]);
                button.set_tooltip(mcrust::net::minecraft::client::gui::components::tooltip::Tooltip::create(tooltip.get_string()));
                screen.screen.add_renderable_widget(button);
            }
        });
        if let Err(e) = result {
            logger().error(&format!("[Skyblocker Config Manager] Failed to hook the container screen init: {:?}", e));
        }
    }

    /// java: protected static void setPatches(List<ConfigPatch> patches)
    pub fn set_patches(patches: &Vec<Arc<dyn ConfigPatch>>) {
        CONFIG_MANAGER.get().set_patches(patches.clone());
    }

    /// java: public static void update(Consumer<SkyblockerConfig> action)
    ///
    /// Executes the given `action` to update fields in the config, then saves the changes.
    pub fn update(action: Arc<dyn Fn(&mut SkyblockerConfig)>) {
        // java: action.accept(getUnpatched());
        SkyblockerConfigManager::get_unpatched(|config| (action)(config));
        ConfigBackupManager::backup_config();
        CONFIG_MANAGER.get().save();
    }

    /// java: public static void updateOnly(Consumer<SkyblockerConfig> action)
    ///
    /// Executes the given `action` to update fields in the config, without saving the changes.
    pub fn update_only(action: Arc<dyn Fn(&mut SkyblockerConfig)>) {
        // java: action.accept(getUnpatched());
        SkyblockerConfigManager::get_unpatched(|config| (action)(config));
        CONFIG_MANAGER.get().update_patched_instance();
    }

    /// java: public static Screen createGUI(@Nullable Screen parent)
    pub fn create_gui(parent: Option<ScreenRef>) -> ScreenRef {
        return SkyblockerConfigManager::create_gui_2(parent, "");
    }

    /// java: public static Screen createGUI(@Nullable Screen parent, String search)
    pub fn create_gui_2(parent: Option<ScreenRef>, search: &str) -> ScreenRef {
        let search: String = search.to_string();
        let screen: Arc<dyn DandelionConfigScreen> = dandelion_config_screen::create(CONFIG_MANAGER.get().clone(), move |defaults, config, builder| {
            builder
                .title(component::translatable_args("skyblocker.config.title", vec![FormattedText::of(skyblocker_mod::VERSION.as_str())]))
                .category(GeneralCategory::create(&defaults, &config))
                .category(UIAndVisualsCategory::create(&defaults, &config))
                .category(HelperCategory::create(&defaults, &config))
                .category(DungeonsCategory::create(&defaults, &config))
                .category(ForagingCategory::create(&defaults, &config))
                .category(CrimsonIsleCategory::create(&defaults, &config))
                .category(MiningCategory::create(&defaults, &config))
                .category(FarmingCategory::create(&defaults, &config))
                .category(HuntingCategory::create(&defaults, &config))
                .category(OtherLocationsCategory::create(&defaults, &config))
                .category(SlayersCategory::create(&defaults, &config))
                .category(ChatCategory::create(&defaults, &config))
                .category(QuickNavigationCategory::create(&defaults, &config))
                .category(EventNotificationsCategory::create(&defaults, &config))
                .category(MiscCategory::create(&defaults, &config))
                .category_if(Debug::debug_enabled(), DebugCategory::create(&defaults, &config))
                .search(&search)
                .platform_links(
                    platform_links::create_builder()
                        .link(component::literal("GitHub"), platform_links::GITHUB_ICON(), "https://github.com/SkyblockerMod/Skyblocker")
                        .link(component::literal("Modrinth"), platform_links::MODRINTH_ICON(), "https://modrinth.com/mod/skyblocker-liap")
                        .build(),
                )
        });
        // java: .generateScreen(parent, get().misc.configBackend) - MOUL_CONFIG (the default) reaches Dandelion's
        // MoulConfig backend (the `moulconfig` library crate), YACL the YACL one
        let backend: ConfigType = SkyblockerConfigManager::get().misc.config_backend;
        return screen.generate_screen(parent, backend);
    }

    /// java: public static Path getConfigPath()
    ///
    /// Returns the path to the config file.
    pub fn get_config_path() -> std::path::PathBuf {
        return CONFIG_FILE.get().clone();
    }

    /// java: public static void reload()
    ///
    /// Reloads the config from disk.
    pub fn reload() {
        CONFIG_MANAGER.get().load();
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> configLiteral(String name)
    ///
    /// Registers a command argument to open the config.
    pub fn config_literal(name: &str) -> LiteralArgumentBuilder<client_command_registration_callback::FabricClientCommandSource> {
        // java: return literal(name).executes(Scheduler.queueOpenScreenCommand(() -> createGUI(null)))
        //       .then(argument("search", StringArgumentType.greedyString()).executes(ctx -> Scheduler.queueOpenScreen(createGUI(null, ctx.getArgument("search", String.class)))));
        //
        // `Scheduler.queueOpenScreen(screen)` is `Minecraft.getInstance().schedule(() -> setScreen(screen))`; the
        // screen is built inside the scheduled task, on the render thread (the YACL screen holds non-`Send` state).
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        type Source = client_command_registration_callback::FabricClientCommandSource;
        let mut literal = client_command_registration_callback::literal(name);
        literal.executes(Arc::new(|_: &CommandContext<Source>| {
            mcrust::net::minecraft::client::minecraft::execute(Box::new(|_minecraft| SkyblockerConfigManager::create_gui(None).set()));
            Ok(1)
        }));
        let mut search = client_command_registration_callback::argument("search", Arc::new(StringArgumentType::greedy_string()));
        search.executes(Arc::new(|ctx: &CommandContext<Source>| {
            let search: String = StringArgumentType::get_string(ctx, "search");
            mcrust::net::minecraft::client::minecraft::execute(Box::new(move |_minecraft| SkyblockerConfigManager::create_gui_2(None, &search).set()));
            Ok(1)
        }));
        literal.then(&search);
        return literal;
    }

    /// java: @SuppressWarnings("JavadocReference") public static boolean isConfigClass(Class<?> clazz)
    ///
    /// Returns `true` if the given class represents one of our config classes. This prevents
    /// `DisableAll#disableEntries(Object)` from touching unrelated objects from other mods.
    ///
    /// `Class<?>` is a `ClassOf` (what reflection answers for a generated class: its binary name, `isEnum()`). A
    /// primitive, record, `String`, `Number`, `Map` or `Collection` class is never in the configs package, so the
    /// package test covers those `!..` terms.
    pub fn is_config_class<T>(clazz: &dandelion::compat::ClassOf<T>) -> bool {
        return !clazz.is_enum && clazz.name.starts_with(CONFIGS_PACKAGE);
    }

    /// java: public static void dataFix(Path configDir, Path backupDir)
    pub fn data_fix(config_dir: &std::path::PathBuf, backup_dir: &std::path::PathBuf) {
        //User is new - has no config file (or maybe config folder)
        if !(CONFIG_DIR.get().exists()) || !config_dir.exists() {
            return;
        }
        //Should never be null if the file exists unless its malformed JSON or something in which case well it gets reset
        let old_config: Option<serde_json::Map<String, serde_json::Value>> = SkyblockerConfigManager::load_config(config_dir);
        if old_config.is_none() || JsonHelper::get_int(old_config.as_ref().unwrap(), "version").unwrap_or(1) == CONFIG_VERSION {
            return;
        }
        let new_config: serde_json::Map<String, serde_json::Value> = ConfigDataFixer::apply(config_data_fixer::CONFIG_TYPE.get_mut(), old_config.as_ref().unwrap());
        //Write the updated file
        if !(SkyblockerConfigManager::write_config(config_dir, &new_config)) {
            logger().error("[Skyblocker Config Data Fixer] Failed to fix up config file!");
            SkyblockerConfigManager::write_config(backup_dir, old_config.as_ref().unwrap());
        }
    }

    /// java: private static @Nullable JsonObject loadConfig(Path path)
    pub fn load_config(path: &std::path::PathBuf) -> Option<serde_json::Map<String, serde_json::Value>> {
        // java: try (BufferedReader reader = Files.newBufferedReader(path)) { return JsonParser.parseReader(reader).getAsJsonObject(); }
        // catch (Throwable t) {..}
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let reader: String = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("IOException: {e}"));
            let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
            match json {
                serde_json::Value::Object(object) => object,
                other => panic!("IllegalStateException: Not a JSON Object: {other}"),
            }
        }));
        match result {
            Ok(object) => return Some(object),
            Err(t) => {
                logger().error(&format!("[Skyblocker Config Data Fixer] Failed to load config file! {}", compat::panic_message(&t)));
            }
        }
        return None;
    }

    /// java: private static boolean writeConfig(Path path, JsonObject config)
    pub fn write_config(path: &std::path::PathBuf, config: &serde_json::Map<String, serde_json::Value>) -> bool {
        // java: try (BufferedWriter writer = Files.newBufferedWriter(path)) { SkyblockerMod.GSON.toJson(config, writer); return true; }
        // catch (Throwable t) {..} (GSON is `setPrettyPrinting()`)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let text: String = dandelion::compat::gson_text(&dandelion::compat::GsonValue::of_json(&serde_json::Value::Object(config.clone())), true);
            std::fs::write(path, text).unwrap_or_else(|e| panic!("IOException: {e}"));
            return true;
        }));
        match result {
            Ok(__v) => return __v,
            Err(t) => {
                logger().error(&format!("[Skyblocker Config Data Fixer] Failed to save config file at {}! {}", path.display(), compat::panic_message(&t)));
            }
        }
        return false;
    }
}
