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
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud;
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud::TabHud;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::placeholder_widget::PlaceholderWidget;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
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
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

// TODO(fabric2seer): static call EnumSet.complementOf(..)
//   java: @SuppressWarnings("deprecation")
//   java: 	public static final Set<Location> ALLOWED_LOCATIONS = Collections.unmodifiableSet(EnumSet.complementOf(EnumSet.of(Location.UNKNOWN, Location.BLAZING_FORTRESS)));
/// java: @SuppressWarnings("deprecation") public static final Set<Location> ALLOWED_LOCATIONS = Collections.unmodifiableSet(EnumSet.complementOf(EnumSet.of(Location.UNKNOWN, Location.BLAZING_FORTRESS)));
pub static ALLOWED_LOCATIONS: LazyLock<std::collections::HashSet<Location>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `ALLOWED_LOCATIONS` - see the TODO above"));
/// java: private static final Identifier FANCY_TAB_HUD = SkyblockerMod.id("fancy_tab_hud");
pub static FANCY_TAB_HUD: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fancy_tab_hud"));
/// java: private static final Identifier FANCY_TAB = SkyblockerMod.id("fancy_tab");
pub static FANCY_TAB: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fancy_tab"));
/// java: private static final int DEFAULTS_VERSION = 1;
pub const DEFAULTS_VERSION: i32 = 1;
/// java: private static final String DEFAULTS_VERSION_KEY = "_defaults_version";
pub const DEFAULTS_VERSION_KEY: &str = "_defaults_version";
/// java: public static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("hud_widgets.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("hud_widgets.json"));
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
// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<ScreenLayer> CODEC = StringRepresentable.fromEnum(ScreenLayer::values);
/// java: public static final Codec<ScreenLayer> CODEC = StringRepresentable.fromEnum(ScreenLayer::values);
pub static SCREEN_LAYER_CODEC: compat::MainThread<CodecRef<ScreenLayer>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final Codec<Config> CODEC = RecordCodecBuilder.create(instance -> instance.group( CodecUtils.mutableOptional(Codec.unboundedMap(Location.CODEC, ScreenConfig.CODEC).fieldOf("configs"), Ob
pub static CONFIG_CODEC: compat::MainThread<CodecRef<Config>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &Config| o.screen_configs(), CodecUtils::mutable_optional(&codec::field_of(codec::unbounded_map(location::CODEC.get_mut().clone(), screen_config::CODEC.get_mut().clone()), "configs"), Arc::new(move |__a0| __a0.clone()))), record_codec_builder::of(|o: &Config| o.copy_tracker(), codec::field_of(copy_tracker::CODEC.get_mut().clone(), "copies")), record_codec_builder::of(|o: &Config| o.version(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "version")), record_codec_builder::of(|_: &Config| DEFAULTS_VERSION, codec::optional_field_of(mcrust::com::mojang::serialization::codec::int_codec(), DEFAULTS_VERSION_KEY, 0)), move |__a0, __a1, __a2, __a3| Config::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));
/// java: public static final Codec<Config> DATA_FIXING_CODEC = ConfigDataFixer.createDataFixingCodec(ConfigDataFixer.HUD_WIDGETS_TYPE, CODEC);
pub static CONFIG_DATA_FIXING_CODEC: compat::MainThread<CodecRef<Config>> = compat::MainThread::new(|| ConfigDataFixer::create_data_fixing_codec(config_data_fixer::HUD_WIDGETS_TYPE.get_mut(), CONFIG_CODEC.get_mut()));

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
    pub fn get_screen_config(mut screen_id: Location) -> ScreenConfig {
        return CONFIG.get_mut().screen_configs().entry(screen_id).or_insert_with_key(|k| (|_| ScreenConfig::new())(k.clone())).clone();
    }

    /// java: public static CopyTracker getCopyTracker()
    pub fn get_copy_tracker() -> CopyTracker {
        return CONFIG.get_mut().copy_tracker();
    }

    /// java: public static boolean hasFancyTab()
    pub fn has_fancy_tab() -> bool {
        return compat::get_state(&HAS_FANCY_TAB);
    }

    /// java: public static HudWidget getWidgetOrPlaceholder(String id)
    pub fn get_widget_or_placeholder(mut id: String) -> compat::Shared<dyn HudWidget> {
        return WIDGET_INSTANCES.get_mut().entry(id.clone()).or_insert_with_key(|k| (|__a0| PlaceholderWidget::new(__a0.clone()))(k.clone())).clone();
    }

    /// java: public static boolean isWidgetInCurrentScreen(HudWidget widget)
    pub fn is_widget_in_current_screen(widget: &compat::Shared<dyn HudWidget>) -> bool {
        return ({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) });
    }

    /// java: public static boolean isWidgetInCurrentLayer(HudWidget widget)
    pub fn is_widget_in_current_layer(widget: &compat::Shared<dyn HudWidget>) -> bool {
        return (compat::get_state(&CURRENT_LAYER).is_some() && { let __r = &({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, compat::get_state(&CURRENT_LAYER)) }); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) });
    }

    /// java: public static List<HudWidget> getWidgetsAvailableIn(Location location)
    pub fn get_widgets_available_in(mut location: Location) -> Vec<compat::Shared<dyn HudWidget>> {
        return WIDGET_INSTANCES.get_mut().values().cloned().filter(|w| (({ let __f = compat::dispatch(w, |__t| __t.get_information()); __f(w) }).available())(location)).collect::<Vec<_>>();
    }

    /// java: @Init(priority = -1) public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> {
        //   java:
        //   java: 			instantiateWidgets();
        //   java: 			for (int i = 1; i < 6; i++) {
        //   java: 				DungeonPlayerWidget widget = new DungeonPlayerWidget(i);
        //   java: 				addWidgetInstance(widget);
        //   java: 			}
        //   java: 			loadConfig();
        //   java:
        //   java: 			PlayerListManager.registerTabListener(WidgetManager::onPlayerListUpdate);
        //   java: 		});
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> saveConfig());
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            WidgetManager::extract_render_state(&graphics, true);
        }));
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            WidgetManager::extract_render_state(&graphics, false);
        }));
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor context, boolean hud)
    pub fn extract_render_state(context: &GuiGraphicsExtractor, mut hud: bool) {
        if !(Utils::is_on_skyblock()) {
            return;
        }
        let client = compat::minecraft();
        let mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen = ScreenUtils::get_underlying_screen_1(client.gui.screen()).unwrap();
        if mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<WidgetsConfigurationScreen>>() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<OtherOptionsScreen>>() {
            return;
        }
        let window = client.get_window();
        let mut scale: f32 = ((SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_scale as f32) / 100.0f32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        WidgetManager::extract_render_state_4(context, ((((window.get_gui_scaled_width() as f32) / scale)) as i32), ((((window.get_gui_scaled_height() as f32) / scale)) as i32), hud);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor context, int w, int h, boolean hud)
    pub fn extract_render_state_4(context: &GuiGraphicsExtractor, mut w: i32, mut h: i32, mut hud: bool) {
        let client = compat::minecraft();
        let mut layer: ScreenLayer;
        // TODO(fabric2seer): `Options.keyPlayerList` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
        // TODO(fabric2seer): `is_down` takes 1 argument(s) in the port (`name: &str`), Java passes 0
        if key_mapping::is_down() {
            if hud || TabHud::should_render_vanilla() {
                return;
            }
            // TODO(fabric2seer): `is_down` takes 1 argument(s) in the port (`name: &str`), Java passes 0
            if key_mapping::is_down() {
                layer = ScreenLayer::SecondaryTab;
            } else {
                layer = ScreenLayer::MainTab;
            }
        } else if hud {
            layer = ScreenLayer::Hud;
        } else {
            return;
        }
        let mut location: Location = Utils::get_location();
        if compat::get_state(&CURRENT_LOCATION).unwrap() != location {
            compat::set_state(&CURRENT_LOCATION, Some(location));
            ({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(__r, WidgetManager::get_screen_config(compat::get_state(&CURRENT_LOCATION))) });
            compat::set_state(&CURRENT_LAYER, None);
        }
        if compat::get_state(&CURRENT_LAYER).unwrap() != layer {
            compat::set_state(&CURRENT_LAYER, Some(layer));
            ({ let __r = &({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, layer) }); let __f = compat::dispatch(__r, |__t| __t.update()); __f(__r) });
        }
        ({ let __r = &({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, compat::get_state(&CURRENT_LAYER)) }); let __f = compat::dispatch(__r, |__t| __t.extract_render_states()); __f(__r, context, w, h, false) });
    }

    /// java: public static void onPlayerListUpdate()
    pub fn on_player_list_update() {
        let mut fancy_tab: bool = SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_enabled;
        if !fancy_tab && compat::get_state(&HAS_FANCY_TAB) {
            ({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.clear_fancy_tab()); __f(__r) });
        } else if fancy_tab {
            ({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab()); __f(__r) });
            compat::set_state(&HAS_FANCY_TAB, true);
        }
    }

    /// java: public static void loadConfig()
    pub fn load_config() {
        // TODO(fabric2seer): `new AtomicReference(..)`
        //   java: AtomicReference<@Nullable String> error = new AtomicReference<>();
        let mut error: crate::unresolved::AtomicReference = todo!("fabric2seer: the initialiser of `error` (TODO above)");
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (NoSuchFileException _) catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call JsonParser.parseReader(..)
                //   java: JsonElement input = JsonParser.parseReader(reader);
                let mut input: serde_json::Value = todo!("fabric2seer: the initialiser of `input` (TODO above)");
                let mut prev_version: i32 = (if compat::json::has(&compat::json::get_as_json_object(&input), "version") { compat::json::get_as_int(&compat::json::get_as_json_object(&input).get("version").cloned().unwrap()) } else { 1 });
                // TODO(fabric2seer): `.set(..)` on a AtomicReference
                //   java: CONFIG = Config.DATA_FIXING_CODEC.decode(JsonOps.INSTANCE, input).resultOrPartial(error::set).orElseThrow().getFirst();
                // TODO(fabric2seer): the condition: `.get(..)` on a AtomicReference
                //   java: error.get() != null
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): `new RuntimeException(..)` outside `throw`
                    //   java: LOGGER.error("[Skyblocker] Failed to load part of the HUD config", new RuntimeException(error.get()));
                    WidgetManager::show_error_toast();
                    ConfigBackupManager::backup_config_1(ConfigType::HudWidgets);
                }
                if CONFIG.get_mut().version != prev_version {
                    ConfigBackupManager::backup_config_1(ConfigType::HudWidgets);
                }
                if CONFIG.get_mut().defaults_version > 0 {
                    WidgetManager::fill_default_config(CONFIG.get_mut().defaults_version);
                }
                for entry in CONFIG.get_mut().screen_configs.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    for layer in entry.1.all_layers() {
                        layer.widgets().retain(|__k, _| !(|id| (WIDGET_INSTANCES.get_mut().contains_key(id) && !((({ let __r = &WIDGET_INSTANCES.get_mut().get(id).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_information()); __f(__r) }).available())(entry.0))))(__k));
                    }
                }
                for map in CONFIG.get_mut().screen_configs.values().cloned().flat_map(|__a0| __a0.all_layers()).map(|__a0| __a0.widgets()) {
                    for entry in map.iter().map(|(k, v)| (k.clone(), v.clone())) {
                        let mut position: Option<PositionRule> = entry.1.position();
                        if position.is_none() {
                            continue;
                        }
                        if position.clone().unwrap().parent().is_some() && !(map.contains_key(&position.clone().unwrap().parent().unwrap())) {
                            // TODO(fabric2seer): `.withParent(..)` on a value of unknown type
                            //   java: entry.setValue(entry.getValue().withPosition(rule -> rule.withParent(null)));
                        }
                    }
                }
            }));
            if let Err(_) = __result {
                WidgetManager::fill_default_config(0);
            }
            // TODO(fabric2seer): the second `catch (Exception e)` clause (merged into the first)
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker] Failed to HUD load config: {}", error.get(), e);
            //   java: 			showErrorToast();
            //   java: 			ConfigBackupManager.backupConfig(ConfigBackupManager.ConfigType.HUD_WIDGETS);
            //   java: 		}
        }
    }

    /// java: private static void showErrorToast()
    pub fn show_error_toast() {
        // TODO(fabric2seer): `add` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
        // rust: system_toast::add(&mut compat::minecraft().gui.toast_manager(), &SystemToastId::new(), component::literal("Error reading Skyblocker HUD Config"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Check your logs!")));
    }

    /// java: private static void fillDefaultConfig(int comingFromVersion)
    pub fn fill_default_config(mut coming_from_version: i32) {
        if coming_from_version < 1 {
            let mut editable_screen_builder: compat::Shared<EditableScreenBuilder> = EditableScreenBuilder::new();
            let mut hud: LayerBuilderEditor = EditableScreenBuilder::get_editor(&editable_screen_builder, ScreenLayer::Hud);
            let mut commissions: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("commissions".to_string());
            let mut powders: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("powders".to_string());
            let mut mining_locations: std::collections::HashSet<Location> = std::collections::HashSet::from([Location::CrystalHollows, Location::DwarvenMines, Location::GlaciteMineshafts]);
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&commissions, |__t| __t.get_internal_id()); __f(&commissions) })), &mining_locations);
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&powders, |__t| __t.get_internal_id()); __f(&powders) })), &mining_locations);
            let mut comms_rule: PositionRule = PositionRule::new(Some(None), Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 5, 5);
            let mut powder_rule: PositionRule = PositionRule::new(Some("commissions".to_string()), Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 0, 2);
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::DwarvenMines)) });
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.serialize_config();
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::CrystalHollows)) });
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.add(&WidgetManager::get_widget_or_placeholder("hud_crystals".to_string()), &PositionRule::new(Some(None), Point::new(VerticalPoint::Top, HorizontalPoint::Right), Point::new(VerticalPoint::Top, HorizontalPoint::Right), -5, -5));
            hud.serialize_config();
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::GlaciteMineshafts)) });
            hud.add(&commissions, &comms_rule);
            hud.add(&powders, &powder_rule);
            hud.serialize_config();
            let mut sweep_details: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder("sweep_details".to_string());
            for location in sweep_details_hud_widget::LOCATIONS.iter().copied() {
                ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(location)) });
                hud.add_1(sweep_details.clone());
                hud.serialize_config();
            }
            ExclusiveGroupingSet::group(&WidgetManager::get_copy_tracker().hud().get_or_create(({ let __f = compat::dispatch(&sweep_details, |__t| __t.get_internal_id()); __f(&sweep_details) })), &sweep_details_hud_widget::LOCATIONS);
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::Galatea)) });
            hud.add(&WidgetManager::get_widget_or_placeholder("hud_treeprogress".to_string()), &PositionRule::new(Some("sweep_details".to_string()), Point::new(VerticalPoint::Bottom, HorizontalPoint::Left), Point::new(VerticalPoint::Top, HorizontalPoint::Left), 0, 2));
            hud.serialize_config();
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::Garden)) });
            hud.add_1(WidgetManager::get_widget_or_placeholder("hud_farming".to_string()));
            hud.serialize_config();
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::TheEnd)) });
            hud.add_1(WidgetManager::get_widget_or_placeholder("hud_end".to_string()));
            hud.serialize_config();
            ({ let __f = compat::dispatch(&editable_screen_builder, |__t| __t.set_config()); __f(&compat::view(&editable_screen_builder, |a| a as compat::Shared<dyn ScreenBuilder>), WidgetManager::get_screen_config(Location::Dungeon)) });
            hud.add(&WidgetManager::get_widget_or_placeholder("dungeon_splits".to_string()), &PositionRule::new(Some(None), Point::new(VerticalPoint::Center, HorizontalPoint::Left), Point::new(VerticalPoint::Center, HorizontalPoint::Left), 5, 0));
            hud.serialize_config();
        }
    }

    /// java: public static void removeAll()
    pub fn remove_all() {
        for __a0 in CONFIG.get_mut().screen_configs().values().cloned().flat_map(|__a0| __a0.all_layers()).map(|__a0| __a0.widgets()) {
            // TODO(fabric2seer): static call Map.clear(..)
            //   java: Map::clear;
        }
        CONFIG.get_mut().copy_tracker().hud().map().clear();
        CONFIG.get_mut().copy_tracker().tab().map().clear();
        CONFIG.get_mut().copy_tracker().secondary_tab().map().clear();
    }

    /// java: public static void saveConfig()
    pub fn save_config() {
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(Config.DATA_FIXING_CODEC.encodeStart(JsonOps.INSTANCE, CONFIG).getOrThrow(), writer);
                logger().info("[Skyblocker] Saved hud widget config");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to save hud widget config"));
            }
        }
    }

    /// java: private static void instantiateWidgets()
    pub fn instantiate_widgets() {
    }

    /// java: public static void addWidgetInstance(HudWidget widget)
    pub fn add_widget_instance(mut widget: compat::Shared<dyn HudWidget>) {
        let mut put: Option<compat::Shared<dyn HudWidget>> = WIDGET_INSTANCES.get_mut().insert(({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) }), widget.clone());
        if put.is_some() && !(put.as_ref().unwrap().lock().borrow().as_any().is::<compat::Shared<PlaceholderWidget>>()) {
            logger().warn(&format!("[Skyblocker] Duplicate hud widget found: {}", compat::jstring(&widget)));
        }
        if compat::get_state(&CURRENT_LAYER).is_some() && mcrust::mcrust::modding::game::level().is_some() {
            ({ let __r = &({ let __r = SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.get()); __f(__r, compat::get_state(&CURRENT_LAYER)) }); let __f = compat::dispatch(__r, |__t| __t.update()); __f(__r) });
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
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for ScreenLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ScreenLayer::to_string(*self))
    }
}

/// java: public record Config(Map<Location, ScreenConfig> screenConfigs, CopyTracker copyTracker, int version, int defaultsVersion)
#[derive(Clone, Debug)]
pub struct Config {
    /// java: Map<Location, ScreenConfig> screenConfigs
    pub screen_configs: std::collections::HashMap<Location, ScreenConfig>,
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
        let mut this = Self::new(std::collections::HashMap::new(), CopyTracker::new(), 0, 0);
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut screen_configs: std::collections::HashMap<Location, ScreenConfig>, mut copy_tracker: CopyTracker, mut version: i32, mut defaults_version: i32) -> Self {
        Self { screen_configs, copy_tracker, version, defaults_version }
    }

    /// java: the record accessor `screenConfigs()`
    pub fn screen_configs(&self) -> std::collections::HashMap<Location, ScreenConfig> {
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
