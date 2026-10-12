//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigType;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer::ConfigDataFixer;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::sweep_details_hud_widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::other_options_screen::OtherOptionsScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::copy_tracker;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::copy_tracker::CopyTracker;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::editable_screen_builder::EditableScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder_editor::LayerBuilderEditor;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::HorizontalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::Point;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::VerticalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config::ScreenConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config::WidgetConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud;
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud::TabHud;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::dungeon_player_widget::DungeonPlayerWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::placeholder_widget::PlaceholderWidget;
use crate::de::hysky::skyblocker::utils::exclusive_grouping_set::ExclusiveGroupingSet;
use crate::de::hysky::skyblocker::utils::location;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::screen_utils::ScreenUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::json_ops::JsonOps;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: @SuppressWarnings("deprecation") public static final Set<Location> ALLOWED_LOCATIONS = Collections.unmodifiableSet(EnumSet.complementOf(EnumSet.of(Location.UNKNOWN, Location.BLAZING_FORTRESS)));
/// (an `EnumSet`: every other location, in ordinal order)
pub static ALLOWED_LOCATIONS: LazyLock<indexmap::IndexSet<Location>> = LazyLock::new(|| Location::values().into_iter().filter(|l| *l != Location::Unknown && *l != Location::BlazingFortress).collect());
/// java: private static final Identifier FANCY_TAB_HUD = SkyblockerMod.id("fancy_tab_hud");
pub static FANCY_TAB_HUD: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fancy_tab_hud"));
/// java: private static final Identifier FANCY_TAB = SkyblockerMod.id("fancy_tab");
pub static FANCY_TAB: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fancy_tab"));
/// java: private static final int DEFAULTS_VERSION = 1;
pub const DEFAULTS_VERSION: i32 = 1;
/// java: private static final String DEFAULTS_VERSION_KEY = "_defaults_version";
pub const DEFAULTS_VERSION_KEY: &str = "_defaults_version";
/// java: public static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("hud_widgets.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get().join("hud_widgets.json"));
/// java: public static final ScreenBuilder SCREEN_BUILDER = new ScreenBuilder();
pub static SCREEN_BUILDER: compat::MainThread<compat::Shared<dyn ScreenBuilder>> = compat::MainThread::new(|| (ScreenBuilderBase::new_0() as compat::Shared<dyn ScreenBuilder>));
/// java: private static Config CONFIG = new Config();
pub static CONFIG: compat::MainThread<Config> = compat::MainThread::new(|| Config::new());
/// java: public static final Map<String, HudWidget> WIDGET_INSTANCES = new HashMap<>();
pub static WIDGET_INSTANCES: compat::MainThread<std::collections::HashMap<String, compat::Shared<dyn HudWidget>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static boolean hasFancyTab = false;
pub static HAS_FANCY_TAB: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static @Nullable Location currentLocation;
pub static CURRENT_LOCATION: LazyLock<Mutex<Option<Location>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static @Nullable ScreenLayer currentLayer;
pub static CURRENT_LAYER: LazyLock<Mutex<Option<ScreenLayer>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static final Codec<ScreenLayer> CODEC = StringRepresentable.fromEnum(ScreenLayer::values);
pub static SCREEN_LAYER_CODEC: compat::MainThread<CodecRef<ScreenLayer>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(ScreenLayer::values(), |l: &ScreenLayer| l.get_serialized_name()));
/// java: public static final Codec<Config> CODEC = RecordCodecBuilder.create(instance -> instance.group(
///   CodecUtils.mutableOptional(Codec.unboundedMap(Location.CODEC, ScreenConfig.CODEC).fieldOf("configs"), Object2ObjectOpenHashMap::new).forGetter(Config::screenConfigs),
///   CopyTracker.CODEC.fieldOf("copies").forGetter(Config::copyTracker),
///   Codec.INT.fieldOf("version").forGetter(Config::version),
///   Codec.INT.optionalFieldOf(DEFAULTS_VERSION_KEY, 0).forGetter(_ -> DEFAULTS_VERSION)
/// ).apply(instance, Config::new));
/// (`mutableOptional(.., Object2ObjectOpenHashMap::new)` is `xmap(mutableFactory, identity)`: the decoded map becomes a
/// new mutable map object - the record's reference to it)
pub static CONFIG_CODEC: compat::MainThread<CodecRef<Config>> = compat::MainThread::new(|| {
    let configs = map_codec::xmap(
        codec::field_of(codec::unbounded_map(location::CODEC.get().clone(), screen_config::CODEC.get().clone()), "configs"),
        |entries: Vec<(Location, ScreenConfig)>| compat::shared(entries.into_iter().collect::<std::collections::HashMap<Location, ScreenConfig>>()),
        |map: &compat::Shared<std::collections::HashMap<Location, ScreenConfig>>| compat::read(map, |m| m.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>()),
    );
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(|o: &Config| o.screen_configs(), configs),
        record_codec_builder::of(|o: &Config| o.copy_tracker(), codec::field_of(copy_tracker::CODEC.get().clone(), "copies")),
        record_codec_builder::of(|o: &Config| o.version(), codec::field_of(codec::int_codec(), "version")),
        record_codec_builder::of(|_: &Config| DEFAULTS_VERSION, codec::optional_field_of(codec::int_codec(), DEFAULTS_VERSION_KEY, 0)),
        move |__a0, __a1, __a2, __a3| Config::new_shared(__a0, __a1, __a2, __a3),
    ))
});
/// java: public static final Codec<Config> DATA_FIXING_CODEC = ConfigDataFixer.createDataFixingCodec(ConfigDataFixer.HUD_WIDGETS_TYPE, CODEC);
pub static CONFIG_DATA_FIXING_CODEC: compat::MainThread<CodecRef<Config>> = compat::MainThread::new(|| ConfigDataFixer::create_data_fixing_codec(config_data_fixer::HUD_WIDGETS_TYPE.get(), CONFIG_CODEC.get()));

/// java: public class WidgetManager
#[derive(Clone)]
pub struct WidgetManager {}

impl WidgetManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ScreenConfig getScreenConfig(Location screenId)
    pub fn get_screen_config(screen_id: Location) -> ScreenConfig {
        // java: return CONFIG.screenConfigs().computeIfAbsent(screenId, _ -> new ScreenConfig());
        let configs = CONFIG.get().screen_configs();
        return compat::write(&configs, |m| m.entry(screen_id).or_insert_with(ScreenConfig::new).clone());
    }

    /// java: public static CopyTracker getCopyTracker()
    pub fn get_copy_tracker() -> CopyTracker {
        return CONFIG.get().copy_tracker();
    }

    /// java: public static boolean hasFancyTab()
    pub fn has_fancy_tab() -> bool {
        return compat::get_state(&HAS_FANCY_TAB);
    }

    /// java: public static HudWidget getWidgetOrPlaceholder(String id)
    pub fn get_widget_or_placeholder(id: String) -> compat::Shared<dyn HudWidget> {
        // java: return WIDGET_INSTANCES.computeIfAbsent(id, PlaceholderWidget::new);
        if let Some(widget) = WIDGET_INSTANCES.get().get(&id) {
            return widget.clone();
        }
        let widget: compat::Shared<dyn HudWidget> = PlaceholderWidget::new(id.clone());
        WIDGET_INSTANCES.get_mut().insert(id, widget.clone());
        return widget;
    }

    /// java: public static boolean isWidgetInCurrentScreen(HudWidget widget)
    pub fn is_widget_in_current_screen(widget: &compat::Shared<dyn HudWidget>) -> bool {
        return ({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) });
    }

    /// java: public static boolean isWidgetInCurrentLayer(HudWidget widget)
    pub fn is_widget_in_current_layer(widget: &compat::Shared<dyn HudWidget>) -> bool {
        let Some(current_layer) = compat::get_state(&CURRENT_LAYER) else {
            return false;
        };
        return { let __r = &({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, current_layer) }); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) };
    }

    /// java: public static List<HudWidget> getWidgetsAvailableIn(Location location)
    pub fn get_widgets_available_in(location: Location) -> Vec<compat::Shared<dyn HudWidget>> {
        let widgets: Vec<compat::Shared<dyn HudWidget>> = WIDGET_INSTANCES.get().values().cloned().collect();
        return widgets.into_iter().filter(|w| (({ let __f = compat::dispatch(w, |__t| __t.get_information()); __f(w) }).available())(location)).collect::<Vec<_>>();
    }

    /// java: @Init(priority = -1) public static void init()
    pub fn init() {
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> {..}); - RETURN of `Minecraft`'s constructor (the client
        // entrypoints run inside it, so this registers before it returns)
        let _ = hook::inject("net.minecraft.client.Minecraft#<init>", At::Return, |_: &mut CallbackInfo<'_>| {
            WidgetManager::instantiate_widgets();
            for i in 1..6 {
                let widget = DungeonPlayerWidget::new(i);
                WidgetManager::add_widget_instance(widget as compat::Shared<dyn HudWidget>);
            }
            WidgetManager::load_config();
            PlayerListManager::register_tab_listener(Arc::new(|| WidgetManager::on_player_list_update()));
        })
        .map_err(|e| logger().error(&format!("[Skyblocker] Failed to hook the client start: {:?}", e)));
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> saveConfig()); - HEAD of `Minecraft#stop`
        let _ = hook::inject("net.minecraft.client.Minecraft#stop", At::Head, |_: &mut CallbackInfo<'_>| {
            WidgetManager::save_config();
        })
        .map_err(|e| logger().error(&format!("[Skyblocker] Failed to hook the client stop: {:?}", e)));
        // java: HudElementRegistry.attachElementBefore(VanillaHudElements.DEMO_TIMER, FANCY_TAB_HUD, (context, _) -> extractRenderState(context, true));
        // java: HudElementRegistry.attachElementBefore(VanillaHudElements.PLAYER_LIST, FANCY_TAB, (context, _) -> extractRenderState(context, false));
        // rust: port gap - no HUD element registry: both draw at RETURN of `Hud#extractRenderState`, after the vanilla HUD
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            WidgetManager::extract_render_state(&mut ctx.graphics, true);
        }));
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            WidgetManager::extract_render_state(&mut ctx.graphics, false);
        }));
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor context, boolean hud)
    pub fn extract_render_state(context: &mut GuiGraphicsExtractor, hud: bool) {
        if !(Utils::is_on_skyblock()) {
            return;
        }
        let client = compat::minecraft();
        // java: Screen screen = ScreenUtils.getUnderlyingScreen(client.gui.screen());
        //       if (screen instanceof WidgetsConfigurationScreen || screen instanceof OtherOptionsScreen) return;
        // rust: port gap - the open screen object is reached by its class only on top of the stack
        // (`game::current_screen_is`); a popup over one of these two screens is not seen through (GAPS_batchA #19)
        if mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<WidgetsConfigurationScreen>>() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<OtherOptionsScreen>>() {
            return;
        }
        let window = client.get_window();
        let scale: f32 = ((SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_scale as f32) / 100.0f32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_scale(scale, scale);
        WidgetManager::extract_render_state_4(context, (((window.get_gui_scaled_width() as f32) / scale) as i32), (((window.get_gui_scaled_height() as f32) / scale) as i32), hud);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: public static void onPlayerListUpdate()
    pub fn on_player_list_update() {
        let fancy_tab: bool = SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_enabled;
        if !fancy_tab && compat::get_state(&HAS_FANCY_TAB) {
            ({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.clear_fancy_tab()); __f(__r) });
        } else if fancy_tab {
            ({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab()); __f(__r) });
            compat::set_state(&HAS_FANCY_TAB, true);
        }
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor context, int w, int h, boolean hud)
    pub fn extract_render_state_4(context: &mut GuiGraphicsExtractor, w: i32, h: i32, hud: bool) {
        let layer: ScreenLayer;
        // java: if (client.options.keyPlayerList.isDown()) {
        if key_mapping::is_down("key.playerlist") {
            if hud || TabHud::should_render_vanilla() {
                return;
            }
            // java: if (TabHud.toggleSecondary.isDown())
            if key_mapping::is_down("key.skyblocker.toggleA") {
                layer = ScreenLayer::SecondaryTab;
            } else {
                layer = ScreenLayer::MainTab;
            }
        } else if hud {
            layer = ScreenLayer::Hud;
        } else {
            return;
        }
        let location: Location = Utils::get_location();
        if compat::get_state(&CURRENT_LOCATION) != Some(location) {
            compat::set_state(&CURRENT_LOCATION, Some(location));
            ({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(__r, WidgetManager::get_screen_config(location)) });
            compat::set_state(&CURRENT_LAYER, None);
        }
        if compat::get_state(&CURRENT_LAYER) != Some(layer) {
            compat::set_state(&CURRENT_LAYER, Some(layer));
            ({ let __r = &({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, layer) }); let __f = compat::dispatch(__r, |__t| __t.update()); __f(__r) });
        }
        ({ let __r = &({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, layer) }); let __f = compat::dispatch(__r, |__t| __t.extract_render_states()); __f(__r, context, w, h, false) });
    }

    /// java: public static void loadConfig()
    pub fn load_config() {
        // java: AtomicReference<@Nullable String> error = new AtomicReference<>();
        let mut error: Option<String> = None;
        // java: try (BufferedReader reader = Files.newBufferedReader(FILE)) {..} catch (NoSuchFileException _) {..} catch (Exception e) {..}
        let reader = match std::fs::read_to_string(FILE.get()) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Fill default config
                WidgetManager::fill_default_config(0);
                return;
            }
            Err(e) => Err(format!("IOException: {e}")),
            Ok(reader) => Ok(reader),
        };
        let __result = reader.and_then(|reader| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // java: JsonElement input = JsonParser.parseReader(reader);
                let input: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                let prev_version: i32 = (if compat::json::has(&compat::json::get_as_json_object(&input), "version") { compat::json::get_as_int(&compat::json::get_as_json_object(&input).get("version").cloned().unwrap()) } else { 1 });
                // java: CONFIG = Config.DATA_FIXING_CODEC.decode(JsonOps.INSTANCE, input).resultOrPartial(error::set).orElseThrow().getFirst();
                let decoded = CONFIG_DATA_FIXING_CODEC.get().decode(&JsonOps::instance(), &compat::json_to_port(&input));
                let config = decoded.result_or_partial_with(|e| error = Some(e)).expect("NoSuchElementException: No value present").0;
                *CONFIG.get_mut() = config;
                if let Some(error) = &error {
                    // separate it to not run when the config fully cannot load
                    logger().error(&format!("[Skyblocker] Failed to load part of the HUD config {}", error));
                    WidgetManager::show_error_toast();
                    ConfigBackupManager::backup_config_1(ConfigType::HudWidgets);
                }
                // Backup if version is different
                if CONFIG.get().version != prev_version {
                    ConfigBackupManager::backup_config_1(ConfigType::HudWidgets);
                }
                // Do not fill defaults if migrating from old config
                if CONFIG.get().defaults_version > 0 {
                    WidgetManager::fill_default_config(CONFIG.get().defaults_version);
                }
                // clean up widgets that are in islands they shouldn't be in
                let entries: Vec<(Location, ScreenConfig)> = compat::read(&CONFIG.get().screen_configs(), |m| m.iter().map(|(k, v)| (*k, v.clone())).collect());
                for entry in entries {
                    for layer in entry.1.all_layers() {
                        // java: layer.widgets().keySet().removeIf(id -> WIDGET_INSTANCES.containsKey(id) && !WIDGET_INSTANCES.get(id).getInformation().available().test(entry.getKey()));
                        let ids: Vec<String> = compat::read(&layer.widgets(), |m| m.keys().cloned().collect());
                        for id in ids {
                            let remove = match WIDGET_INSTANCES.get().get(&id).cloned() {
                                Some(widget) => !(({ let __f = compat::dispatch(&widget, |__t| __t.get_information()); __f(&widget) }).available())(entry.0),
                                None => false,
                            };
                            if remove {
                                compat::write(&layer.widgets(), |m| m.remove(&id));
                            }
                        }
                    }
                }
                // clean up widgets that are parented to a widget that isn't the layer
                let screen_configs: Vec<ScreenConfig> = compat::read(&CONFIG.get().screen_configs(), |m| m.values().cloned().collect());
                for map in screen_configs.iter().flat_map(|c| c.all_layers()).map(|l| l.widgets()) {
                    let entries: Vec<(String, WidgetConfig)> = compat::read(&map, |m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
                    for entry in entries {
                        let Some(position) = entry.1.position() else {
                            continue;
                        };
                        if let Some(parent) = position.parent() && !compat::read(&map, |m| m.contains_key(&parent)) {
                            // java: entry.setValue(entry.getValue().withPosition(rule -> rule.withParent(null)));
                            let value = entry.1.with_position_function(Arc::new(|rule: PositionRule| Some(rule.with_parent(None))));
                            compat::write(&map, |m| m.insert(entry.0.clone(), value));
                        }
                    }
                }
            }))
            .map_err(|e| compat::panic_message(&e))
        });
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to HUD load config: {} {}", compat::opt_str(&error), e));
            WidgetManager::show_error_toast();
            ConfigBackupManager::backup_config_1(ConfigType::HudWidgets);
        }
    }

    /// java: private static void showErrorToast()
    pub fn show_error_toast() {
        // java: SystemToast.add(Minecraft.getInstance().gui.toastManager(), new SystemToast.SystemToastId(), Component.literal("Error reading Skyblocker HUD Config"), Component.literal("Check your logs!"));
        let client = compat::minecraft();
        let title = FormattedText::from(component::literal("Error reading Skyblocker HUD Config"));
        let message = FormattedText::from(component::literal("Check your logs!"));
        system_toast::add(&mut client.gui.toast_manager().borrow_mut(), &client.font, SystemToastId::new(), &title, Some(&message));
    }

    /// java: private static void fillDefaultConfig(int comingFromVersion)
    pub fn fill_default_config(coming_from_version: i32) {
        if coming_from_version < 1 {
            let editable_screen_builder: compat::Shared<EditableScreenBuilder> = EditableScreenBuilder::new();
            let hud: LayerBuilderEditor = EditableScreenBuilder::get_editor(&editable_screen_builder, ScreenLayer::Hud);
            let set_config = |config: ScreenConfig| {
                let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config());
                __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), config)
            };
            // Mining related stuff
            let commissions: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("commissions".to_string());
            let powders: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("powders".to_string());
            let mining_locations: std::collections::HashSet<Location> = std::collections::HashSet::from([Location::CrystalHollows, Location::DwarvenMines, Location::GlaciteMineshafts]);
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&commissions, |__t| __t.get_internal_id()); __f(&commissions) })), &mining_locations);
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&powders, |__t| __t.get_internal_id()); __f(&powders) })), &mining_locations);
            let comms_rule: PositionRule = PositionRule::new_canonical(None, Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 5, 5);
            let powder_rule: PositionRule = PositionRule::new(Some("commissions".to_string()), Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 0, 2);
            set_config(WidgetManager::get_screen_config(Location::DwarvenMines));
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.serialize_config();

            set_config(WidgetManager::get_screen_config(Location::CrystalHollows));
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.add(&WidgetManager::get_widget_or_placeholder("hud_crystals".to_string()), &PositionRule::new_canonical(None, Point::new(VerticalPoint::Top, HorizontalPoint::Right), Point::new(VerticalPoint::Top, HorizontalPoint::Right), -5, -5));
            hud.serialize_config();

            set_config(WidgetManager::get_screen_config(Location::GlaciteMineshafts));
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.serialize_config();

            // Sweep details
            let sweep_details: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("sweep_details".to_string());
            for location in sweep_details_hud_widget::LOCATIONS.iter().copied() {
                set_config(WidgetManager::get_screen_config(location));
                hud.add_1(sweep_details.clone());
                hud.serialize_config();
            }
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&sweep_details, |__t| __t.get_internal_id()); __f(&sweep_details) })), &sweep_details_hud_widget::LOCATIONS);

            // Galatea
            set_config(WidgetManager::get_screen_config(Location::Galatea));
            hud.add(&WidgetManager::get_widget_or_placeholder("hud_treeprogress".to_string()), &PositionRule::new(Some("sweep_details".to_string()), Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 0, 2));
            hud.serialize_config();

            // Garden
            set_config(WidgetManager::get_screen_config(Location::Garden));
            hud.add_1(WidgetManager::get_widget_or_placeholder("hud_farming".to_string()));
            hud.serialize_config();

            // The end
            set_config(WidgetManager::get_screen_config(Location::TheEnd));
            hud.add_1(WidgetManager::get_widget_or_placeholder("hud_end".to_string()));
            hud.serialize_config();

            set_config(WidgetManager::get_screen_config(Location::Dungeon));
            hud.add(&WidgetManager::get_widget_or_placeholder("dungeon_splits".to_string()), &PositionRule::new_canonical(None, Point::new(VerticalPoint::Center, HorizontalPoint::Left), Point::new(VerticalPoint::Center, HorizontalPoint::Left), 5, 0));
            hud.serialize_config();
        }
    }

    /// java: public static void removeAll()
    pub fn remove_all() {
        // java: CONFIG.screenConfigs().values().stream().flatMap(ScreenConfig::allLayers).map(LayerConfig::widgets).forEach(Map::clear);
        let screen_configs: Vec<ScreenConfig> = compat::read(&CONFIG.get().screen_configs(), |m| m.values().cloned().collect());
        for map in screen_configs.iter().flat_map(|c| c.all_layers()).map(|l| l.widgets()) {
            compat::write(&map, |m| m.clear());
        }
        compat::write(&CONFIG.get().copy_tracker().hud().map(), |m| m.clear());
        compat::write(&CONFIG.get().copy_tracker().tab().map(), |m| m.clear());
        compat::write(&CONFIG.get().copy_tracker().secondary_tab().map(), |m| m.clear());
    }

    /// java: public static void saveConfig()
    pub fn save_config() {
        // java: try (BufferedWriter writer = Files.newBufferedWriter(FILE)) {
        //           SkyblockerMod.GSON.toJson(Config.DATA_FIXING_CODEC.encodeStart(JsonOps.INSTANCE, CONFIG).getOrThrow(), writer); ..
        // (`newBufferedWriter` truncates the file before the config is encoded; GSON is `setPrettyPrinting()`)
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            use std::io::Write as _;
            let mut writer = std::io::BufWriter::new(std::fs::File::create(FILE.get()).unwrap_or_else(|e| panic!("IOException: {e}")));
            let encoded = CONFIG_DATA_FIXING_CODEC.get().encode_start(&JsonOps::instance(), CONFIG.get()).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
            let text: String = serde_json::to_string_pretty(&compat::json_from_port(&encoded)).unwrap_or_else(|e| panic!("JsonIOException: {e}"));
            writer.write_all(text.as_bytes()).unwrap_or_else(|e| panic!("IOException: {e}"));
            writer.flush().unwrap_or_else(|e| panic!("IOException: {e}"));
            logger().info("[Skyblocker] Saved hud widget config");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to save hud widget config {}", compat::panic_message(&e)));
        }
    }

    /// java: private static void instantiateWidgets()
    /// "Filled at compile item with ASM" - the `@RegisterWidget` classes' `addWidgetInstance(new X())` calls.
    /// rust: port gap - the mod's widget registration (`hud/hud_injector.rs`, an ASM class-file pass) has no
    /// counterpart, so nothing fills this body
    pub fn instantiate_widgets() {
    }

    /// java: public static void addWidgetInstance(HudWidget widget)
    pub fn add_widget_instance(widget: compat::Shared<dyn HudWidget>) {
        let id = ({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) });
        let put: Option<compat::Shared<dyn HudWidget>> = WIDGET_INSTANCES.get_mut().insert(id.clone(), widget.clone());
        // java: if (put != null && !(put instanceof PlaceholderWidget)) LOGGER.warn("[Skyblocker] Duplicate hud widget found: {}", widget);
        if let Some(put) = &put && !compat::read(put, |__t| __t.as_any().is::<PlaceholderWidget>()) {
            logger().warn(&format!("[Skyblocker] Duplicate hud widget found: {}@{:x}", id, compat::string_hash_code(&id)));
        }
        if let Some(current_layer) = compat::get_state(&CURRENT_LAYER) && mcrust::mcrust::modding::game::level().is_some() {
            ({ let __r = &({ let __r = SCREEN_BUILDER.get(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, current_layer) }); let __f = compat::dispatch(__r, |__t| __t.update()); __f(__r) });
        }
    }

}

/// java: public enum ScreenLayer implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScreenLayer {
    MainTab,
    SecondaryTab,
    Hud,
}

impl ScreenLayer {
    pub const VALUES: [ScreenLayer; 3] = [ScreenLayer::MainTab, ScreenLayer::SecondaryTab, ScreenLayer::Hud];

    /// java: values()
    pub fn values() -> Vec<ScreenLayer> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ScreenLayer::MainTab => "MAIN_TAB",
            ScreenLayer::SecondaryTab => "SECONDARY_TAB",
            ScreenLayer::Hud => "HUD",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ScreenLayer {
        match name {
            "MAIN_TAB" => ScreenLayer::MainTab,
            "SECONDARY_TAB" => ScreenLayer::SecondaryTab,
            "HUD" => ScreenLayer::Hud,
            _ => panic!("IllegalArgumentException: No enum constant ScreenLayer.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return match *self {
            ScreenLayer::MainTab => "Main Tab".to_string(),
            ScreenLayer::SecondaryTab => "Secondary Tab".to_string(),
            ScreenLayer::Hud => "HUD".to_string(),
        };
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // java: return name().toLowerCase(Locale.ENGLISH);
        return self.name().to_lowercase();
    }

}

impl std::fmt::Display for ScreenLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ScreenLayer::to_string(self))
    }
}


/// java: public record Config(Map<Location, ScreenConfig> screenConfigs, CopyTracker copyTracker, int version, int defaultsVersion)
/// (`screenConfigs` is a mutable map the record holds - `getScreenConfig` adds to it - so the record holds a reference
/// to it)
#[derive(Clone)]
pub struct Config {
    /// java: Map<Location, ScreenConfig> screenConfigs
    pub screen_configs: compat::Shared<std::collections::HashMap<Location, ScreenConfig>>,
    /// java: CopyTracker copyTracker
    pub copy_tracker: CopyTracker,
    /// java: int version
    pub version: i32,
    /// java: int defaultsVersion
    pub defaults_version: i32,
}

impl Config {
    /// java: public Config()
    pub fn new() -> Self {
        Self::new_canonical(std::collections::HashMap::new(), CopyTracker::new(), 0, 0)
    }

    /// java: the canonical record constructor
    pub fn new_canonical(screen_configs: std::collections::HashMap<Location, ScreenConfig>, copy_tracker: CopyTracker, version: i32, defaults_version: i32) -> Self {
        Self::new_shared(compat::shared(screen_configs), copy_tracker, version, defaults_version)
    }

    /// the canonical constructor over a map object the caller already holds (the codec's mutable copy)
    pub fn new_shared(screen_configs: compat::Shared<std::collections::HashMap<Location, ScreenConfig>>, copy_tracker: CopyTracker, version: i32, defaults_version: i32) -> Self {
        Self { screen_configs, copy_tracker, version, defaults_version }
    }

    /// java: the record accessor `screenConfigs()` (the map object itself)
    pub fn screen_configs(&self) -> compat::Shared<std::collections::HashMap<Location, ScreenConfig>> {
        self.screen_configs.clone()
    }

    /// java: the record accessor `copyTracker()`
    pub fn copy_tracker(&self) -> CopyTracker {
        self.copy_tracker.clone()
    }

    /// java: the record accessor `version()`
    pub fn version(&self) -> i32 {
        self.version
    }

    /// java: the record accessor `defaultsVersion()`
    pub fn defaults_version(&self) -> i32 {
        self.defaults_version
    }

}
