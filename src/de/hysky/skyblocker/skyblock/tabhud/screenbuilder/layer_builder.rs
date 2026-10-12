//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.LayerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config::LayerConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositionerBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::util::problem_reporter::ScopedCollector;
use mcrust::net::minecraft::util::profiling::profiler;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::util::profiling::profiler_filler::ProfilerFiller as _;

/// java: public class LayerBuilder
#[derive(Clone)]
pub struct LayerBuilderBase {
    /// java: protected LayerConfig config = LayerConfig.DUMMY;
    pub config: LayerConfig,
    /// java: protected final Set<HudWidget> renderedWidgets = new ObjectOpenHashSet<>(); (a set by `HudWidget.equals`:
    /// `<dyn HudWidget>::contains_equal`)
    pub rendered_widgets: Vec<compat::Shared<dyn HudWidget>>,
    /// java: protected final List<PositionedWidget> widgets = new LinkedList<>();
    pub widgets: Vec<compat::Shared<PositionedWidget>>,
    /// java: private int positionsHash = 0;
    pub positions_hash: i32,
}

impl LayerBuilderBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn LayerBuilder>, __R)) -> (compat::Shared<dyn LayerBuilder>, __R) {
        let this = Self {
            config: layer_config::DUMMY.get_mut().clone(),
            rendered_widgets: Vec::new(),
            widgets: Vec::new(),
            positions_hash: 0,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn LayerBuilder>), __h)
        }).1
    }

    /// java: public static void updatePositions(Collection<PositionedWidget> widgets, int screenWidth, int screenHeight)
    pub fn update_positions_3(widgets: &Vec<compat::Shared<PositionedWidget>>, mut screen_width: i32, mut screen_height: i32) {
        profiler::get().push("skyblocker:updatePositions");
        for w in widgets.iter() {
            { let __v = false; compat::write(w, |__t| __t.positioned = __v); }
        }
        for widget in widgets.iter() {
            if !(compat::read(widget, |__t| __t.positioned)) {
                // the lambda reads the same collection (`widgets.stream()`): the provider holds its own references to it
                let __widgets: Vec<compat::Shared<PositionedWidget>> = widgets.clone();
                WidgetPositionerBase::apply_rule_to_widget(widget, screen_width, screen_height, Arc::new(move |s: String| __widgets.iter().cloned().filter(|w| (({ let __r = &compat::read(w, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) }) == s)).next().unwrap_or_else(|| {
                    logger().warn(&format!("Tried to get parent '{}' but it doesn't exist in the layer. Please report this to the Skyblocker discord or github along with the hud_widgets.json config file.", s));
                    let mut positioned_widget: compat::Shared<PositionedWidget> = PositionedWidget::new(WidgetManager::get_widget_or_placeholder(s.clone()), position_rule::DEFAULT.get_mut().clone());
                    { let __v = false; compat::write(&positioned_widget, |__t| __t.visible = __v); }
                    { let __v = true; compat::write(&positioned_widget, |__t| __t.positioned = __v); }
                    return positioned_widget;
                })));
            }
        }
        profiler::get().pop();
    }

}

/// java: the virtual methods of LayerBuilder (implemented by LayerBuilderBase and every subclass)
pub trait LayerBuilder: compat::DynAny {
    fn layer_builder_base(&self) -> &LayerBuilderBase;
    fn layer_builder_base_mut(&mut self) -> &mut LayerBuilderBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void setConfig(LayerConfig config) (dispatch: the body for this class)
    fn set_config(&self) -> fn(&compat::Shared<dyn LayerBuilder>, LayerConfig) {
        <dyn LayerBuilder>::set_config_body
    }

    /// java: public void update() (dispatch: the body for this class)
    fn update(&self) -> fn(&compat::Shared<dyn LayerBuilder>) {
        <dyn LayerBuilder>::update_body
    }

    /// java: protected void updateList() (dispatch: the body for this class)
    fn update_list(&self) -> fn(&compat::Shared<dyn LayerBuilder>) {
        <dyn LayerBuilder>::update_list_body
    }

    /// java: public boolean contains(HudWidget widget) (dispatch: the body for this class)
    fn contains(&self) -> fn(&compat::Shared<dyn LayerBuilder>, &compat::Shared<dyn HudWidget>) -> bool {
        <dyn LayerBuilder>::contains_body
    }

    /// java: public void extractRenderStates(GuiGraphicsExtractor graphics, int screenWidth, int screenHeight, boolean config) (dispatch: the body for this class)
    fn extract_render_states(&self) -> fn(&compat::Shared<dyn LayerBuilder>, &mut GuiGraphicsExtractor, i32, i32, bool) {
        <dyn LayerBuilder>::extract_render_states_body
    }

    /// java: public Collection<PositionedWidget> getRendered() (dispatch: the body for this class)
    fn get_rendered(&self) -> fn(&compat::Shared<dyn LayerBuilder>) -> Vec<compat::Shared<PositionedWidget>> {
        <dyn LayerBuilder>::get_rendered_body
    }

    /// java: public void updatePositions(int screenWidth, int screenHeight) (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn LayerBuilder>, i32, i32) {
        <dyn LayerBuilder>::update_positions_body
    }

}

impl dyn LayerBuilder {
    /// java: public void setConfig(LayerConfig config)
    pub fn set_config_body(__this: &compat::Shared<dyn LayerBuilder>, mut config: LayerConfig) {
        { let __v = config.clone(); compat::write(__this, |__t| __t.layer_builder_base_mut().config = __v); }
    }

    /// java: public void update()
    pub fn update_body(__this: &compat::Shared<dyn LayerBuilder>) {
        { let __v = 0; compat::write(__this, |__t| __t.layer_builder_base_mut().positions_hash = __v); }
        compat::write(__this, |__t| __t.layer_builder_base_mut().widgets.clear());
        let __entries = { let __map = compat::read(__this, |__t| __t.layer_builder_base().config.widgets()); compat::read(&__map, |m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()) };
        for entry in __entries {
            if entry.1.config().is_none() {
                continue;
            }
            let mut hud_widget: compat::Shared<dyn HudWidget> = WidgetManager::get_widget_or_placeholder(entry.0.clone());
            {
                // java: try (ProblemReporter.ScopedCollector reporter = new ProblemReporter.ScopedCollector(WidgetManager.LOGGER)) {
                //           hudWidget.load(new JsonValueInput(reporter, entry.getValue().config().get())); }
                let reporter: ScopedCollector = ScopedCollector::new("de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager");
                ({ let __f = compat::dispatch(&hud_widget, |__t| __t.load()); __f(&hud_widget, &JsonValueInput::new(reporter.reporter(), entry.1.config().unwrap())) });
            }
            if entry.1.position().is_none() {
                continue;
            }
            let __h0 = PositionedWidget::new(hud_widget.clone(), entry.1.position().unwrap());
            compat::write(__this, |__t| __t.layer_builder_base_mut().widgets.push(__h0));
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_list()); __f(__this) });
    }

    /// java: protected void updateList()
    pub fn update_list_body(__this: &compat::Shared<dyn LayerBuilder>) {
        compat::write(__this, |__t| __t.layer_builder_base_mut().rendered_widgets.clear());
        for __a0 in ({ let __f = compat::dispatch(__this, |__t| __t.get_rendered()); __f(__this) }).into_iter().map(|w| compat::read(&w, |__t| __t.widget.clone())) {
            // java: renderedWidgets.add(..) - a set: an equal widget is not added twice
            compat::write(__this, |__t| {
                if !<dyn HudWidget>::contains_equal(&__t.layer_builder_base().rendered_widgets, &__a0) {
                    __t.layer_builder_base_mut().rendered_widgets.push(__a0);
                }
            });
        }
    }

    /// java: public boolean contains(HudWidget widget)
    pub fn contains_body(__this: &compat::Shared<dyn LayerBuilder>, widget: &compat::Shared<dyn HudWidget>) -> bool {
        let __rendered = compat::read(__this, |__t| __t.layer_builder_base().rendered_widgets.clone());
        return <dyn HudWidget>::contains_equal(&__rendered, widget);
    }

    /// java: public void extractRenderStates(GuiGraphicsExtractor graphics, int screenWidth, int screenHeight, boolean config)
    pub fn extract_render_states_body(__this: &compat::Shared<dyn LayerBuilder>, graphics: &mut GuiGraphicsExtractor, mut screen_width: i32, mut screen_height: i32, mut config: bool) {
        profiler::get().push("skyblocker:renderHud");
        // java: int hash = Integer.hashCode(screenWidth); (Integer.hashCode(v) is v)
        let mut hash: i32 = screen_width;
        // java: hash = hash * 31 + Integer.hashCode(screenHeight);
        hash = hash.wrapping_mul(31).wrapping_add(screen_height);
        for widget in ({ let __f = compat::dispatch(__this, |__t| __t.get_rendered()); __f(__this) }) {
            let mut should_render: bool = (config || { let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.should_render()); __f(__r) });
            { let __v = should_render; compat::write(&widget, |__t| __t.visible = __v); }
            // java: hash = hash * 31 + Boolean.hashCode(shouldRender); (1231 / 1237)
            hash = hash.wrapping_mul(31).wrapping_add(if should_render { 1231 } else { 1237 });
            let __w = compat::read(&widget, |__t| __t.widget.clone());
            // java: hash = hash * 31 + Integer.hashCode(widget.widget.getWidth());
            hash = hash.wrapping_mul(31).wrapping_add({ let __f = compat::dispatch(&__w, |__t| __t.get_width()); __f(&__w) });
            // java: hash = hash * 31 + Integer.hashCode(widget.widget.getHeight());
            hash = hash.wrapping_mul(31).wrapping_add({ let __f = compat::dispatch(&__w, |__t| __t.get_height()); __f(&__w) });
        }
        if compat::read(__this, |__t| __t.layer_builder_base().positions_hash) != hash {
            { let __v = hash; compat::write(__this, |__t| __t.layer_builder_base_mut().positions_hash = __v); }
            ({ let __f = compat::dispatch(__this, |__t| __t.update_positions()); __f(__this, screen_width, screen_height) });
        }
        // java: float delta = Minecraft.getInstance().getDeltaTracker().getGameTimeDeltaTicks();
        let mut delta: f32 = compat::minecraft().get_delta_tracker().get_game_time_delta_ticks();
        for widget in ({ let __f = compat::dispatch(__this, |__t| __t.get_rendered()); __f(__this) }) {
            if !(compat::read(&widget, |__t| __t.visible)) && !config {
                continue;
            }
            let __pose_1 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) })) as f32, (({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) })) as f32);
            if config {
                ({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.extract_render_state_for_config()); __f(__r, graphics, delta) });
            } else {
                ({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(__r, graphics, delta) });
            }
            gui_graphics_extractor::pop_pose(__pose_1);
        }
        profiler::get().pop();
    }

    /// java: public Collection<PositionedWidget> getRendered()
    pub fn get_rendered_body(__this: &compat::Shared<dyn LayerBuilder>) -> Vec<compat::Shared<PositionedWidget>> {
        return compat::read(__this, |__t| __t.layer_builder_base().widgets.clone());
    }

    /// java: public void updatePositions(int screenWidth, int screenHeight)
    pub fn update_positions_body(__this: &compat::Shared<dyn LayerBuilder>, mut screen_width: i32, mut screen_height: i32) {
        LayerBuilderBase::update_positions_3(&({ let __f = compat::dispatch(__this, |__t| __t.get_rendered()); __f(__this) }).into_iter().filter(|p| !(compat::read(p, |__t| __t.from_tab))).collect::<Vec<_>>(), screen_width, screen_height);
    }

}

impl LayerBuilder for LayerBuilderBase {
    fn layer_builder_base(&self) -> &LayerBuilderBase {
        self
    }
    fn layer_builder_base_mut(&mut self) -> &mut LayerBuilderBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
