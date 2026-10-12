//! Ported from de.hysky.skyblocker.config.SkyblockerConfigManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer::ConfigDataFixer;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::utils::datafixer::json_helper::JsonHelper;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;

/// java: public static final int CONFIG_VERSION = 13;
pub const CONFIG_VERSION: i32 = 13;
/// java: private static final String CONFIGS_PACKAGE = "de.hysky.skyblocker.config.configs";
pub const CONFIGS_PACKAGE: &str = "de.hysky.skyblocker.config.configs";
/// java: private static final Path CONFIG_DIR = FabricLoader.getInstance().getConfigDir();
pub static CONFIG_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| mcrust::mcrust::modding::config::config_dir());
/// java: private static final Path CONFIG_FILE = CONFIG_DIR.resolve("skyblocker.json");
pub static CONFIG_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| CONFIG_DIR.get_mut().join("skyblocker.json"));
// TODO(fabric2seer): static call ConfigManager.create(..)
//   java: private static final ConfigManager<SkyblockerConfig> CONFIG_MANAGER = ConfigManager.create(SkyblockerConfig.class, CONFIG_FILE, UnaryOperator.identity());
/// java: private static final ConfigManager<SkyblockerConfig> CONFIG_MANAGER = ConfigManager.create(SkyblockerConfig.class, CONFIG_FILE, UnaryOperator.identity());
pub static CONFIG_MANAGER: compat::MainThread<crate::unresolved::ConfigManager> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CONFIG_MANAGER` - see the TODO above"));

/// java: public class SkyblockerConfigManager
#[derive(Clone)]
pub struct SkyblockerConfigManager {}

impl SkyblockerConfigManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static SkyblockerConfig get()
    pub fn get() -> SkyblockerConfig {
        // TODO(fabric2seer): `.instance(..)` on a ConfigManager
        //   java: return CONFIG_MANAGER.instance();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected static SkyblockerConfig getUnpatched()
    pub fn get_unpatched() -> SkyblockerConfig {
        // TODO(fabric2seer): `.unpatchedInstance(..)` on a ConfigManager
        //   java: return CONFIG_MANAGER.unpatchedInstance();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void init()
    pub fn init() {
        // TODO(fabric2seer): the condition: static call StackWalker.getInstance(..)
        //   java: StackWalker.getInstance(Option.RETAIN_CLASS_REFERENCE).getCallerClass() != SkyblockerMod.class
        if todo!("fabric2seer: the condition (TODO above)") {
            panic!("RuntimeException: {}", "Skyblocker: Called config init from an illegal place!");
        }
        SkyblockerConfigManager::data_fix(CONFIG_FILE.get_mut(), &CONFIG_DIR.get_mut().join("skyblocker.json.old"));
        // TODO(fabric2seer): `.load(..)` on a ConfigManager
        //   java: CONFIG_MANAGER.load();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): `.getTitle(..)` on a value of unknown type
            //   java: if (get().uiAndVisuals.showConfigButton && screen instanceof ContainerScreen genericContainerScreen && screen.getTitle().getString().equals("SkyBlock Menu")) {
            //   java: 				Screens.getWidgets(screen).add(Button
            //   java: 						.builder(Component.literal("\uD83D\uDD27"), _ -> client.gui.setScreen(createGUI(screen)))
            //   java: 						.bounds(((AbstractContainerScreenAccessor) genericContainerScreen).getX() + ((AbstractContainerScreenAccessor) genericContainerScreen).getImageWidth() - 16, ((AbstractContainerScreenAccessor) genericContainerScreen).getY() + 4, 12, 12)
            //   java: 						.tooltip(Tooltip.create(Component.translatable("skyblocker.config.title", Component.translatable("skyblocker.config.title.settings"))))
            //   java: 						.build());
            //   java: 			}
        }));
    }

    /// java: protected static void setPatches(List<ConfigPatch> patches)
    pub fn set_patches(patches: &Vec<crate::unresolved::ConfigPatch>) {
        // TODO(fabric2seer): `.setPatches(..)` on a ConfigManager
        //   java: CONFIG_MANAGER.setPatches(patches);
    }

    /// java: public static void update(Consumer<SkyblockerConfig> action)
    pub fn update(action: Arc<dyn Fn(SkyblockerConfig)>) {
        (action)(SkyblockerConfigManager::get_unpatched());
        ConfigBackupManager::backup_config();
        // TODO(fabric2seer): `.save(..)` on a ConfigManager
        //   java: CONFIG_MANAGER.save();
    }

    /// java: public static void updateOnly(Consumer<SkyblockerConfig> action)
    pub fn update_only(action: Arc<dyn Fn(SkyblockerConfig)>) {
        (action)(SkyblockerConfigManager::get_unpatched());
        // TODO(fabric2seer): `.updatePatchedInstance(..)` on a ConfigManager
        //   java: CONFIG_MANAGER.updatePatchedInstance();
    }

    /// java: public static Screen createGUI(@Nullable Screen parent)
    pub fn create_gui(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        return SkyblockerConfigManager::create_gui_2(parent.clone(), "");
    }

    /// java: public static Screen createGUI(@Nullable Screen parent, String search)
    pub fn create_gui_2(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, search: &str) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        // TODO(fabric2seer): static call DandelionConfigScreen.create(..)
        //   java: return DandelionConfigScreen.create(CONFIG_MANAGER, (defaults, config, builder) -> builder
        //   java: 				.title(Component.translatable("skyblocker.config.title", SkyblockerMod.VERSION))
        //   java: 				.category(GeneralCategory.create(defaults, config))
        //   java: 				.category(UIAndVisualsCategory.create(defaults, config))
        //   java: 				.category(HelperCategory.create(defaults, config))
        //   java: 				.category(DungeonsCategory.create(defaults, config))
        //   java: 				.category(ForagingCategory.create(defaults, config))
        //   java: 				.category(CrimsonIsleCategory.create(defaults, config))
        //   java: 				.category(MiningCategory.create(defaults, config))
        //   java: 				.category(FarmingCategory.create(defaults, config))
        //   java: 				.category(HuntingCategory.create(defaults, config))
        //   java: 				.category(OtherLocationsCategory.create(defaults, config))
        //   java: 				.category(SlayersCategory.create(defaults, config))
        //   java: 				.category(ChatCategory.create(defaults, config))
        //   java: 				.category(QuickNavigationCategory.create(defaults, config))
        //   java: 				.category(EventNotificationsCategory.create(defaults, config))
        //   java: 				.category(MiscCategory.create(defaults, config))
        //   java: 				.categoryIf(Debug.debugEnabled(), DebugCategory.create(defaults, config))
        //   java: 				.search(search)
        //   java: 				.platformLinks(PlatformLinks.createBuilder()
        //   java: 						.link(Component.literal("GitHub"), PlatformLinks.GITHUB_ICON, "https://github.com/SkyblockerMod/Skyblocker")
        //   java: 						.link(Component.literal("Modrinth"), PlatformLinks.MODRINTH_ICON, "https://modrinth.com/mod/skyblocker-liap")
        //   java: 						.build())
        //   java: 		).generateScreen(parent, get().misc.configBackend);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static Path getConfigPath()
    pub fn get_config_path() -> std::path::PathBuf {
        return CONFIG_FILE.get_mut().clone();
    }

    /// java: public static void reload()
    pub fn reload() {
        // TODO(fabric2seer): `.load(..)` on a ConfigManager
        //   java: CONFIG_MANAGER.load();
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> configLiteral(String name)
    pub fn config_literal(name: &str) -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(name).executes(Scheduler.queueOpenScreenCommand(() -> createGUI(null)))
        //   java: 				.then(argument("search", StringArgumentType.greedyString()).executes(ctx -> Scheduler.queueOpenScreen(createGUI(null, ctx.getArgument("search", String.class)))));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("JavadocReference") public static boolean isConfigClass(Class<?> clazz)
    pub fn is_config_class(clazz: &crate::unresolved::Class) -> bool {
        // TODO(fabric2seer): `.isPrimitive(..)` on a Class
        //   java: return !clazz.isPrimitive()
        //   java: 			&& !clazz.isEnum()
        //   java: 			&& !clazz.isRecord()
        //   java: 			&& !clazz.equals(String.class)
        //   java: 			&& !Number.class.isAssignableFrom(clazz)
        //   java: 			&& !Map.class.isAssignableFrom(clazz)
        //   java: 			&& !Collection.class.isAssignableFrom(clazz)
        //   java: 			&& clazz.getPackageName().startsWith(CONFIGS_PACKAGE);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void dataFix(Path configDir, Path backupDir)
    pub fn data_fix(config_dir: &std::path::PathBuf, backup_dir: &std::path::PathBuf) {
        if !(CONFIG_DIR.get_mut().exists()) || !config_dir.exists() {
            return;
        }
        let mut old_config: Option<serde_json::Map<String, serde_json::Value>> = SkyblockerConfigManager::load_config(config_dir);
        if old_config.is_none() || JsonHelper::get_int(old_config.as_ref().unwrap(), "version").unwrap_or(1) == CONFIG_VERSION {
            return;
        }
        let mut new_config: serde_json::Map<String, serde_json::Value> = ConfigDataFixer::apply(config_data_fixer::CONFIG_TYPE.get_mut(), old_config.as_ref().unwrap());
        if !(SkyblockerConfigManager::write_config(config_dir, &new_config)) {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Data Fixer] Failed to fix up config file!");
            SkyblockerConfigManager::write_config(backup_dir, old_config.as_ref().unwrap());
        }
    }

    /// java: private static @Nullable JsonObject loadConfig(Path path)
    pub fn load_config(path: &std::path::PathBuf) -> Option<serde_json::Map<String, serde_json::Value>> {
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (Throwable t) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call JsonParser.parseReader(..)
                //   java: return JsonParser.parseReader(reader).getAsJsonObject();
            })) {
                Ok(__v) => return __v,
                Err(t) => {
                    logger().error(&format!("[Skyblocker Config Data Fixer] Failed to load config file!"));
                }
            }
        }
        return None;
    }

    /// java: private static boolean writeConfig(Path path, JsonObject config)
    pub fn write_config(path: &std::path::PathBuf, config: &serde_json::Map<String, serde_json::Value>) -> bool {
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (Throwable t) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(config, writer);
                return true;
            })) {
                Ok(__v) => return __v,
                Err(t) => {
                    logger().error(&format!("[Skyblocker Config Data Fixer] Failed to save config file at {}!", path.display()));
                }
            }
        }
        return false;
    }

}
