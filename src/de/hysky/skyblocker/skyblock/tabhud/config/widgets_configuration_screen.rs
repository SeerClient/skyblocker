//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.WidgetsConfigurationScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::add_widget_widget::AddWidgetWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::side_panel_widget::SidePanelWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::top_bar_widget::TopBarWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::editable_screen_builder::EditableLayer;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::editable_screen_builder::EditableScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_config::LayerConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::HorizontalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::Point;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::VerticalPoint;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config::ScreenConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config::WidgetConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositionerBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::tab_hud::TabHud;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::placeholder_widget::PlaceholderWidget;
use crate::de::hysky::skyblocker::utils::exclusive_grouping_set::ExclusiveGroupingSet;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::screen_utils::ScreenUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::lang::throwable::Throwable;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_axis::ScreenAxis;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static Pattern SCREEN_TITLE_PATTERN = Pattern.compile("(\\(\\d/\\d]\\) )?widgets (in|on)");
pub static SCREEN_TITLE_PATTERN: LazyLock<Mutex<compat::regex::Pattern>> = LazyLock::new(|| Mutex::new(compat::regex::Pattern::compile("(\\(\\d/\\d]\\) )?widgets (in|on)")));

/// java: public class WidgetsConfigurationScreen extends Screen
pub struct WidgetsConfigurationScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen previousScreen;
    pub previous_screen: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private Location currentLocation;
    pub current_location: Option<Location>,
    /// java: private WidgetManager.ScreenLayer currentScreenLayer;
    pub current_screen_layer: Option<ScreenLayer>,
    /// java: protected final EditableScreenBuilder screenBuilder = new EditableScreenBuilder();
    pub screen_builder: compat::Shared<EditableScreenBuilder>,
    /// java: private ScreenConfig screenConfig;
    pub screen_config: Option<ScreenConfig>,
    /// java: private EditableScreenBuilder.EditableLayer layer;
    pub layer: Option<EditableLayer>,
    /// java: private SidePanelWidget sidePanelWidget;
    pub side_panel_widget: Option<SidePanelWidget>,
    /// java: private AddWidgetWidget addWidgetWidget;
    pub add_widget_widget: Option<AddWidgetWidget>,
    /// java: private TopBarWidget topBarWidget;
    pub top_bar_widget: Option<TopBarWidget>,
    /// java: @SuppressWarnings("DataFlowIssue") private final PositionedWidget screenWidget = new PositionedWidget(new HudWidget(new HudWidget.Information(null, null)) { @Ov
    pub screen_widget: compat::Shared<PositionedWidget>,
    /// java: private @Nullable PositionedWidget hoveredWidget;
    pub hovered_widget: Option<compat::Shared<PositionedWidget>>,
    /// java: private @Nullable PositionedWidget selectedWidget;
    pub selected_widget: Option<compat::Shared<PositionedWidget>>,
    /// java: private @Nullable ScreenPosition dragRelative = null;
    pub drag_relative: Option<ScreenPosition>,
    /// java: private boolean openPanelAfterDragging = false;
    pub open_panel_after_dragging: bool,
    /// java: boolean autoAnchor = true;
    pub auto_anchor: bool,
    /// java: private @Nullable SelectWidgetPrompt selectWidgetPrompt = null;
    pub select_widget_prompt: Option<SelectWidgetPrompt>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for WidgetsConfigurationScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetsConfigurationScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetsConfigurationScreen {
    /// java: public WidgetsConfigurationScreen()
    pub fn new() -> compat::Shared<Self> {
        let __this = Self::new_2(Utils::get_location(), None);
        __this
    }

    /// java: public WidgetsConfigurationScreen(Location location, @Nullable Screen previousScreen)
    pub fn new_2(mut location: Location, mut previous_screen: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_previous_screen = previous_screen.clone();
        let __init_current_location = Some(location);
        let __init_current_screen_layer = Some(ScreenLayer::Hud);
        // TODO(fabric2seer): the initialiser of `screenWidget`: `new HudWidget(..) {..}` (an anonymous class with state or several methods)
        //   java: @SuppressWarnings("DataFlowIssue")
        //   java: 	private final PositionedWidget screenWidget = new PositionedWidget(new HudWidget(new HudWidget.Information(null, null)) {
        //   java: 		@Override
        //   java: 		protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta) {
        //   java: 			w = getScreenWidth();
        //   java: 			h = getScreenHeight();
        //   java: 		}
        //   java:
        //   java: 		@Override
        //   java: 		protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) {
        //   java: 			w = getScreenWidth();
        //   java: 			h = getScreenHeight();
        //   java: 		}
        //   java: 	}, PositionRule.DEFAULT);
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::literal("Widgets Config Screen").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            previous_screen: __init_previous_screen,
            current_location: __init_current_location,
            current_screen_layer: __init_current_screen_layer,
            screen_builder: EditableScreenBuilder::new(),
            screen_config: None,
            layer: None,
            side_panel_widget: None,
            add_widget_widget: None,
            top_bar_widget: None,
            screen_widget: Default::default(),
            hovered_widget: None,
            selected_widget: None,
            drag_relative: None,
            open_panel_after_dragging: false,
            auto_anchor: true,
            select_widget_prompt: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(WidgetManager::get_screen_config(compat::read(&__this, |__t| __t.current_location.clone()).unwrap())); compat::write(&__this, |__t| __t.screen_config = __v); }
        ({ let __r = &compat::read(&__this, |__t| __t.screen_builder.clone()); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>), compat::read(&__this, |__t| __t.screen_config.clone()).unwrap()) });
        { let __v = Some(EditableScreenBuilder::get_layer(&compat::read(&__this, |__t| __t.screen_builder.clone()), compat::read(&__this, |__t| __t.current_screen_layer.clone()).unwrap())); compat::write(&__this, |__t| __t.layer = __v); }
        __this.lock().borrow_mut().layer.as_mut().unwrap().update();
        if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_enabled {
            ({ let __r = &compat::read(&__this, |__t| __t.screen_builder.clone()); let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>)) });
        }
        __this
    }

    /// java: @Init public static void initCommands()
    pub fn init_commands() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static ScreenRectangle getBorder(ScreenRectangle rect, ScreenDirection side)
    pub fn get_border(rect: &ScreenRectangle, side: &ScreenDirection) -> ScreenRectangle {
        let mut extra_x: i32 = rect.width().wrapping_div(2);
        let mut extra_y: i32 = rect.height().wrapping_div(2);
        let mut thickness: i32 = 5i32.wrapping_add(if side.get_axis() == ScreenAxis::Horizontal { extra_x } else { extra_y });
        let mut i: i32 = rect.get_bound_in_direction(*side);
        let mut other_axis: ScreenAxis = side.get_axis().orthogonal();
        let mut j: i32 = rect.get_bound_in_direction(other_axis.get_negative());
        let mut k: i32 = rect.get_length(other_axis);
        let mut screen_rect: ScreenRectangle = ScreenRectangle::of(side.get_axis(), i, j, thickness, k);
        let mut offset_x: i32 = (if side.get_axis() == ScreenAxis::Horizontal { (if side.is_positive() { (-extra_x) } else { -5 }) } else { 0 });
        let mut offset_y: i32 = (if side.get_axis() == ScreenAxis::Vertical { (if side.is_positive() { (-extra_y) } else { -5 }) } else { 0 });
        return ScreenRectangle::new_at(screen_rect.left().wrapping_add(offset_x), screen_rect.top().wrapping_add(offset_y), screen_rect.width(), screen_rect.height());
    }

    /// java: public void setCurrentLocation(Location newLocation)
    pub fn set_current_location(__this: &compat::Shared<Self>, mut new_location: Location) {
        __this.lock().borrow().layer.as_ref().unwrap().editor().serialize_config();
        { let __v = Some(new_location); compat::write(__this, |__t| __t.current_location = __v); }
        { let __v = Some(WidgetManager::get_screen_config(compat::read(__this, |__t| __t.current_location.clone()).unwrap())); compat::write(__this, |__t| __t.screen_config = __v); }
        ({ let __r = &compat::read(__this, |__t| __t.screen_builder.clone()); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>), compat::read(__this, |__t| __t.screen_config.clone()).unwrap()) });
        { let __v = Some(EditableScreenBuilder::get_layer(&compat::read(__this, |__t| __t.screen_builder.clone()), compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap())); compat::write(__this, |__t| __t.layer = __v); }
        __this.lock().borrow_mut().layer.as_mut().unwrap().update();
        if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_enabled {
            ({ let __r = &compat::read(__this, |__t| __t.screen_builder.clone()); let __f = compat::dispatch(__r, |__t| __t.update_fancy_tab()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ScreenBuilder>)) });
        }
    }

    /// java: public void setCurrentScreenLayer(WidgetManager.ScreenLayer newScreenLayer)
    pub fn set_current_screen_layer(__this: &compat::Shared<Self>, mut new_screen_layer: ScreenLayer) {
        __this.lock().borrow().layer.as_ref().unwrap().editor().serialize_config();
        { let __v = Some(new_screen_layer); compat::write(__this, |__t| __t.current_screen_layer = __v); }
        { let __v = Some(EditableScreenBuilder::get_layer(&compat::read(__this, |__t| __t.screen_builder.clone()), new_screen_layer)); compat::write(__this, |__t| __t.layer = __v); }
        __this.lock().borrow_mut().layer.as_mut().unwrap().update();
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        { let __v = Some(SidePanelWidget::new((compat::read(__this, |__t| __t.width)).wrapping_div(4), compat::read(__this, |__t| __t.height), __this.clone())); compat::write(__this, |__t| __t.side_panel_widget = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(AddWidgetWidget::new(compat::minecraft(), Arc::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { WidgetsConfigurationScreen::add_widget((&__this_1), __a0.clone()); } }))); compat::write(__this, |__t| __t.add_widget_widget = __v); }
        { let __v = Some(TopBarWidget::new(compat::read(__this, |__t| __t.width), __this.clone())); compat::write(__this, |__t| __t.top_bar_widget = __v); }
        WidgetsConfigurationScreen::add_widget(__this, Box::new(compat::read(__this, |__t| __t.add_widget_widget.clone())));
        WidgetsConfigurationScreen::add_widget(__this, Box::new(compat::read(__this, |__t| __t.top_bar_widget.clone())));
        WidgetsConfigurationScreen::add_widget(__this, Box::new(compat::read(__this, |__t| __t.side_panel_widget.clone())));
        WidgetsConfigurationScreen::reposition_elements(__this);
    }

    /// java: private void addWidget(HudWidget widget)
    pub fn add_widget(__this: &compat::Shared<Self>, mut widget: compat::Shared<dyn HudWidget>) {
        { let __v = PositionRule::new(Some("screen".to_string()), position_rule::POINT_DEFAULT.clone(), position_rule::POINT_DEFAULT.clone(), (((compat::minecraft().mouse_handler.get_scaled_x_pos_of(compat::minecraft().get_window()) / (TabHud::get_scale_factor() as f64))) as i32), (((compat::minecraft().mouse_handler.get_scaled_y_pos_of(compat::minecraft().get_window()) / (TabHud::get_scale_factor() as f64))) as i32)); compat::write(&__this.lock().borrow().layer.as_ref().unwrap().editor().add_1(widget.clone()), |__t| __t.rule = __v); }
        WidgetsConfigurationScreen::update_builder_positions(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __h0 = (compat::read(__this, |__t| __t.width)).wrapping_div(4);
        __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().set_width(__h0);
        let __h0 = compat::read(__this, |__t| __t.height);
        __this.lock().borrow().side_panel_widget.as_ref().unwrap().set_height(__h0);
        if __this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open() {
            let __h0 = (if __this.lock().borrow().side_panel_widget.as_ref().unwrap().right_side { (compat::read(__this, |__t| __t.width)).wrapping_sub(__this.lock().borrow().side_panel_widget.as_ref().unwrap().get_width()) } else { 0 });
            __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().set_x(__h0);
        }
        let __h0 = compat::read(__this, |__t| __t.width);
        __this.lock().borrow().top_bar_widget.as_ref().unwrap().set_width(__h0);
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_background(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // TODO(fabric2seer): `Screen.extractBackground` has no Rust method in the port (looked for `extract_background` in Screen and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_background(context, mouse_x, mouse_y, delta_ticks);
        let mut text: MutableComponent = component::translatable("skyblocker.config.hud.screen.rightClick");
        let mut text_width: i32 = compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), ((compat::read(__this, |__t| __t.width)).wrapping_sub(text_width)).wrapping_div(2), ((compat::read(__this, |__t| __t.height)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), 0xFFFFFFFFu32 as i32);
    }

    /// java: @Override protected void extractBlurredBackground(GuiGraphicsExtractor graphics)
    pub fn extract_blurred_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        if mcrust::mcrust::modding::game::level().is_some() && !(minecraft::has_control_down()) {
            // TODO(fabric2seer): `Screen.extractBlurredBackground` has no Rust method in the port (looked for `extract_blurred_background` in Screen and its supertypes)
            // rust: __this.lock().borrow_mut().base.extract_blurred_background(graphics);
        }
    }

    /// java: @Override protected void extractMenuBackground(GuiGraphicsExtractor graphics)
    pub fn extract_menu_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        if mcrust::mcrust::modding::game::level().is_some() && !(minecraft::has_control_down()) {
            // TODO(fabric2seer): `Screen.extractMenuBackground` has no Rust method in the port (looked for `extract_menu_background` in Screen and its supertypes)
            // rust: __this.lock().borrow_mut().base.extract_menu_background(graphics);
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        ();
        let mut scale: f32 = TabHud::get_scale_factor();
        let __pose_1 = gui_graphics_extractor::push_pose();
        // TODO(fabric2seer): Matrix3x2fStack.scale
        //   java: matrices.scale(scale);
        ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.extract_render_states()); __f(__r, context, WidgetsConfigurationScreen::get_screen_width(__this), WidgetsConfigurationScreen::get_screen_height(__this), true) });
        gui_graphics_extractor::pop_pose(__pose_1);
        { let __v = None; compat::write(__this, |__t| __t.hovered_widget = __v); }
        let mut scaled_mouse_x: f64 = (((mouse_x as f32) / scale)) as f64;
        let mut scaled_mouse_y: f64 = (((mouse_y as f32) / scale)) as f64;
        for hud_widget in ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }) {
            if { let __r = &compat::read(&hud_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.is_mouse_over()); __f(__r, scaled_mouse_x, scaled_mouse_y) } {
                { let __v = Some(hud_widget.clone()); compat::write(__this, |__t| __t.hovered_widget = __v); }
                break;
            }
        }
        // TODO(fabric2seer): `new Matrix3x2f(..)`: the port has no `Matrix3x2f::new`
        // rust: let mut scale_matrix: Matrix3x2f = Matrix3x2f::new().scale_xy(scale);
        let mut scale_matrix: Matrix3x2f = todo!("fabric2seer: the initialiser of `scaleMatrix` (TODO above)");
        if compat::read(__this, |__t| __t.hovered_widget.clone()).is_some() && { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().get_child_at(__q0, __q1) }.is_none() {
            // TODO(fabric2seer): `getRectangle` is not a method of HudWidget
            //   java: ScreenRectangle rect = hoveredWidget.widget.getRectangle().transformAxisAligned(scaleMatrix);
            let mut rect: ScreenRectangle = todo!("fabric2seer: the initialiser of `rect` (TODO above)");
            GuiHelper::border(context, rect.left().wrapping_sub(1), rect.top().wrapping_sub(1), rect.width().wrapping_add(2), rect.height().wrapping_add(2), mcrust::net::minecraft::util::common_colors::YELLOW);
        }
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_some() {
            // TODO(fabric2seer): `getRectangle` is not a method of HudWidget
            //   java: ScreenRectangle rect = selectedWidget.widget.getRectangle().transformAxisAligned(scaleMatrix);
            let mut rect: ScreenRectangle = todo!("fabric2seer: the initialiser of `rect` (TODO above)");
            GuiHelper::border(context, rect.left().wrapping_sub(1), rect.top().wrapping_sub(1), rect.width().wrapping_add(2), rect.height().wrapping_add(2), mcrust::net::minecraft::util::common_colors::GREEN);
        }
        let __r0 = compat::read(__this, |__t| __t.selected_widget.clone());
        let __r1 = compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone());
        __this.lock().borrow_mut().top_bar_widget.as_mut().unwrap().visible = (__r0.is_none() || ({ let __r = &__r1; let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) >= 16);
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractContainerWidget` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks,); __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractContainerWidget` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks,); __this.lock().borrow_mut().top_bar_widget.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractSelectionList` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks,); __this.lock().borrow_mut().add_widget_widget.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        if compat::read(__this, |__t| __t.current_location.clone()).unwrap() != Utils::get_location() {
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.screen.locationWarning[0]")), (compat::read(__this, |__t| __t.width)).wrapping_div(2), (compat::read(__this, |__t| __t.height)).wrapping_sub(30), 0xFFFFFFFFu32 as i32);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.screen.locationWarning[1]")), (compat::read(__this, |__t| __t.width)).wrapping_div(2), (compat::read(__this, |__t| __t.height)).wrapping_sub(20), 0xFFFFFFFFu32 as i32);
        }
        if compat::read(__this, |__t| __t.select_widget_prompt.clone()).is_some() {
            // TODO(fabric2seer): the port's `set_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, component for Java's component, x, y)
            // TODO(fabric2seer): `set_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, component: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 3
            // rust: gui_graphics_extractor::set_tooltip_for_next_frame(&mut __this.lock().borrow().select_widget_prompt.as_ref().unwrap().tooltip(), mouse_x, mouse_y);
        }
        // TODO(fabric2seer): `setPosition` is not a method of HudWidget
        //   java: screenWidget.widget.setPosition(0, 0);
        ({ let __r = &compat::read(&compat::read(__this, |__t| __t.screen_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(__r, context, delta_ticks) });
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double deltaX, double deltaY)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut delta_x: f64, mut delta_y: f64) -> bool {
        if false {
            return true;
        }
        let mut mouse_x: f64 = click.x();
        let mut mouse_y: f64 = click.y();
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_some() && !(compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.from_tab)) && compat::read(__this, |__t| __t.drag_relative.clone()).is_some() {
            let mut old_rule: PositionRule = compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule.clone());
            mouse_x = (mouse_x / (TabHud::get_scale_factor() as f64));
            mouse_y = (mouse_y / (TabHud::get_scale_factor() as f64));
            let mut pos: SnapResult;
            if compat::read(__this, |__t| __t.auto_anchor) && old_rule.parent().is_none() {
                let mut point: Point = WidgetsConfigurationScreen::get_point_3(__this, &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()), ((mouse_x) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().x()), ((mouse_y) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().y()));
                pos = SnapResult::new(None, point.clone(), point.clone(), None, None);
            } else {
                pos = SnapResult::new(old_rule.parent(), old_rule.parent_point(), old_rule.this_point(), None, None);
            }
            if minecraft::has_shift_down() {
                pos = WidgetsConfigurationScreen::snap_selected_widget(__this, ((mouse_x) as i32), ((mouse_y) as i32)).unwrap_or(pos.clone());
            }
            let mut start_position: ScreenPosition = WidgetPositionerBase::get_start_position(pos.parent.clone(), WidgetsConfigurationScreen::get_screen_width(__this), WidgetsConfigurationScreen::get_screen_height(__this), &pos.parent_point);
            { let __v = PositionRule::new(pos.parent.clone(), pos.parent_point.clone(), pos.this_point.clone(), pos.relative_x.clone().unwrap_or(((((mouse_x) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().x())).wrapping_sub(start_position.x())).wrapping_add((((({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }) as f32) * pos.this_point.horizontal_point().get_percentage())) as i32)), pos.relative_y.clone().unwrap_or(((((mouse_y) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().y())).wrapping_sub(start_position.y())).wrapping_add((((({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }) as f32) * pos.this_point.vertical_point().get_percentage())) as i32))); compat::write(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule = __v); }
            WidgetsConfigurationScreen::update_builder_positions(__this);
            let mut side_panel: ScreenRectangle = ScreenRectangle::new_at(__this.lock().borrow().side_panel_widget.as_ref().unwrap().get_x(), __this.lock().borrow().side_panel_widget.as_ref().unwrap().get_y(), __this.lock().borrow().side_panel_widget.as_ref().unwrap().get_width(), __this.lock().borrow().side_panel_widget.as_ref().unwrap().get_height());
            let mut selected: ScreenRectangle = ScreenRectangle::new_at(({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }));
            if __this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open() && side_panel.overlaps(selected.clone()) {
                __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().close();
                WidgetsConfigurationScreen::open_side_panel(__this);
                { let __v = true; compat::write(__this, |__t| __t.open_panel_after_dragging = __v); }
            }
            return true;
        }
        return false;
    }

    /// java: private Optional<SnapResult> snapSelectedWidget(int mouseX, int mouseY)
    pub fn snap_selected_widget(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> Option<SnapResult> {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() || compat::read(__this, |__t| __t.drag_relative.clone()).is_none() {
            return None;
        }
        let mut result: (Option<SnapResult>, i32) = (None, i32::MAX);
        let mut selected_rect: ScreenRectangle = ScreenRectangle::new_at(mouse_x.wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().x()), mouse_y.wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().y()), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }));
        // TODO(fabric2seer): static `ScreenDirection.values` has no Rust counterpart in the port
        // rust: let mut selected_snap_boxes: Vec<ScreenRectangle> = ScreenDirection::values().iter().cloned().map(|dir| ScreenUtils::get_snap_box(&selected_rect, &dir)).collect::<Vec<_>>();
        let mut selected_snap_boxes: Vec<ScreenRectangle> = todo!("fabric2seer: the initialiser of `selectedSnapBoxes` (TODO above)");
        result = WidgetsConfigurationScreen::snap_selected_to_widget(__this, &compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), &compat::read(__this, |__t| __t.screen_widget.clone()), &selected_snap_boxes, &selected_rect, result.1).unwrap_or(result.clone());
        for other_widget in ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }) {
            result = WidgetsConfigurationScreen::snap_selected_to_widget(__this, &compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), &other_widget, &selected_snap_boxes, &selected_rect, result.1).unwrap_or(result.clone());
        }
        return result.0.clone();
    }

    /// java: private Optional<ObjectIntPair<@Nullable SnapResult>> snapSelectedToWidget(PositionedWidget selectedWidget, PositionedWidget otherWidget, ScreenRectangle[] selectedSnapBoxes, ScreenRectangle selectedR
    pub fn snap_selected_to_widget(__this: &compat::Shared<Self>, selected_widget: &compat::Shared<PositionedWidget>, other_widget: &compat::Shared<PositionedWidget>, selected_snap_boxes: &Vec<ScreenRectangle>, selected_rect: &ScreenRectangle, mut snap_score: i32) -> Option<(Option<SnapResult>, i32)> {
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(other_widget), std::sync::Arc::as_ptr(selected_widget)) {
            return None;
        }
        if Some(&*(({ let __r = &compat::read(selected_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) }))) == other_widget.lock().borrow().rule.parent().as_deref() {
            return None;
        }
        // TODO(fabric2seer): `getRectangle` is not a method of HudWidget
        //   java: ScreenRectangle otherRect = otherWidget.widget.getRectangle();
        let mut other_rect: ScreenRectangle = todo!("fabric2seer: the initialiser of `otherRect` (TODO above)");
        let mut point: Point = WidgetsConfigurationScreen::get_point(__this, &compat::read(other_widget, |__t| __t.widget.clone()));
        let mut dist_x: i32 = 10;
        let mut dist_y: i32 = 10;
        let mut parent_point_h: Option<HorizontalPoint> = None;
        let mut this_point_h: Option<HorizontalPoint> = None;
        let mut parent_point_v: Option<VerticalPoint> = None;
        let mut this_point_v: Option<VerticalPoint> = None;
        let mut relative_x: Option<i32> = None;
        let mut relative_y: Option<i32> = None;
        // TODO(fabric2seer): static `ScreenDirection.values` has no Rust counterpart in the port
        for other_edge in ScreenDirection::values() {
            let mut select_edge: ScreenDirection = other_edge.get_opposite();
            let mut other_snap_box: ScreenRectangle = ScreenUtils::get_snap_box(&other_rect, &other_edge);
            // TODO(fabric2seer): `ScreenDirection.ordinal` has no Rust method in the port (looked for `ordinal` in ScreenDirection and JavaSrc)
            // rust: let selected_snap_box = &selected_snap_boxes[(select_edge.ordinal()) as usize];
            let mut selected_snap_box: ScreenRectangle = todo!("fabric2seer: the initialiser of `selectedSnapBox` (TODO above)");
            if !(selected_snap_box.overlaps(other_snap_box.clone())) {
                continue;
            }
            let mut dist: i32 = ((selected_rect.get_bound_in_direction(select_edge)).wrapping_sub(other_rect.get_bound_in_direction(other_edge))).wrapping_abs();
            if other_edge.get_axis() == ScreenAxis::Horizontal && dist < dist_x {
                dist_x = dist;
                relative_x = Some((if other_edge.is_positive() { 1 } else { -2 }));
                parent_point_h = Some((if other_edge.is_positive() { HorizontalPoint::Right } else { HorizontalPoint::Left }));
                this_point_h = Some((if other_edge.is_positive() { HorizontalPoint::Left } else { HorizontalPoint::Right }));
            } else if other_edge.get_axis() == ScreenAxis::Vertical && dist < dist_y {
                dist_y = dist;
                relative_y = Some((if other_edge.is_positive() { 1 } else { -2 }));
                parent_point_v = Some((if other_edge.is_positive() { VerticalPoint::Bottom } else { VerticalPoint::Top }));
                this_point_v = Some((if other_edge.is_positive() { VerticalPoint::Top } else { VerticalPoint::Bottom }));
            }
        }
        // TODO(fabric2seer): static `ScreenDirection.values` has no Rust counterpart in the port
        for edge in ScreenDirection::values() {
            let mut other_snap_box: ScreenRectangle = ScreenUtils::get_snap_box(&other_rect, &edge);
            // TODO(fabric2seer): `ScreenDirection.ordinal` has no Rust method in the port (looked for `ordinal` in ScreenDirection and JavaSrc)
            // rust: let selected_snap_box = &selected_snap_boxes[(edge.ordinal()) as usize];
            let mut selected_snap_box: ScreenRectangle = todo!("fabric2seer: the initialiser of `selectedSnapBox` (TODO above)");
            if !(selected_snap_box.overlaps(other_snap_box.clone())) {
                continue;
            }
            let mut dist: i32 = ((selected_rect.get_bound_in_direction(edge)).wrapping_sub(other_rect.get_bound_in_direction(edge))).wrapping_abs();
            if edge.get_axis() == ScreenAxis::Horizontal && dist < dist_x {
                dist_x = dist;
                relative_x = Some(0);
                parent_point_h = { this_point_h = Some((if edge.is_positive() { HorizontalPoint::Right } else { HorizontalPoint::Left })); this_point_h.clone() };
            } else if edge.get_axis() == ScreenAxis::Vertical && dist < dist_y {
                dist_y = dist;
                relative_y = Some(0);
                parent_point_v = { this_point_v = Some((if edge.is_positive() { VerticalPoint::Bottom } else { VerticalPoint::Top })); this_point_v.clone() };
            }
        }
        if parent_point_h.is_none() && parent_point_v.is_none() {
            return None;
        }
        let mut dist: i32 = dist_x.wrapping_add(dist_y);
        if dist >= snap_score {
            return None;
        }
        let mut parent_point: Point = Point::new((if parent_point_v.is_some() { parent_point_v.clone() } else { Some(point.vertical_point()) }).unwrap(), (if parent_point_h.is_some() { parent_point_h.clone() } else { Some(point.horizontal_point()) }).unwrap());
        let mut this_point: Point = Point::new((if this_point_v.is_some() { this_point_v.clone() } else { Some(point.vertical_point()) }).unwrap(), (if this_point_h.is_some() { this_point_h.clone() } else { Some(point.horizontal_point()) }).unwrap());
        return Some((SnapResult::new(Some(({ let __r = &compat::read(other_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })), parent_point.clone(), this_point.clone(), relative_x.clone(), relative_y.clone()), dist));
    }

    /// java: private PositionRule.Point getPoint(HudWidget widget)
    pub fn get_point(__this: &compat::Shared<Self>, widget: &compat::Shared<dyn HudWidget>) -> Point {
        return WidgetsConfigurationScreen::get_point_3(__this, widget, ({ let __f = compat::dispatch(widget, |__t| __t.get_x()); __f(widget) }), ({ let __f = compat::dispatch(widget, |__t| __t.get_y()); __f(widget) }));
    }

    /// java: private PositionRule.Point getPoint(HudWidget widget, int x, int y)
    pub fn get_point_3(__this: &compat::Shared<Self>, widget: &compat::Shared<dyn HudWidget>, mut x: i32, mut y: i32) -> Point {
        let mut widget_center_x: i32 = (x.wrapping_add(({ let __f = compat::dispatch(widget, |__t| __t.get_width()); __f(widget) }).wrapping_div(2))).wrapping_sub((WidgetsConfigurationScreen::get_screen_width(__this)).wrapping_div(2));
        let mut widget_center_y: i32 = (y.wrapping_add(({ let __f = compat::dispatch(widget, |__t| __t.get_height()); __f(widget) }).wrapping_div(2))).wrapping_sub((WidgetsConfigurationScreen::get_screen_height(__this)).wrapping_div(2));
        let mut h_point: HorizontalPoint = (if widget_center_x < -25 { HorizontalPoint::Left } else { (if widget_center_x > 25 { HorizontalPoint::Right } else { HorizontalPoint::Center }) });
        let mut v_point: VerticalPoint = (if widget_center_y < -25 { VerticalPoint::Top } else { (if widget_center_y > 25 { VerticalPoint::Bottom } else { VerticalPoint::Center }) });
        return Point::new(v_point, h_point);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if false {
            return true;
        }
        let mut mouse_x: f64 = click.x();
        let mut mouse_y: f64 = click.y();
        if compat::read(__this, |__t| __t.select_widget_prompt.clone()).is_some() {
            if compat::read(__this, |__t| __t.hovered_widget.clone()).is_some() && !(__this.lock().borrow().select_widget_prompt.as_ref().unwrap().allow_itself()) && { let (__a0,) = (compat::read(__this, |__t| __t.selected_widget.clone()),); __this.lock().borrow().hovered_widget.as_ref().unwrap().equals(__a0) } {
                return true;
            }
            (__this.lock().borrow().select_widget_prompt.as_ref().unwrap().callback())((if compat::read(__this, |__t| __t.hovered_widget.clone()).is_none() { None } else { Some(compat::read(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap(), |__t| __t.widget.clone())) }));
            { let __v = None; compat::write(__this, |__t| __t.select_widget_prompt = __v); }
            __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().open();
            return true;
        }
        if compat::read(__this, |__t| __t.hovered_widget.clone()).is_none() {
            if __this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open() {
                __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().close();
            }
            { let __v = None; compat::write(__this, |__t| __t.selected_widget = __v); }
            if click.button() == 1 {
                let mut available_widgets: Vec<compat::Shared<dyn HudWidget>> = WidgetManager::get_widgets_available_in(compat::read(__this, |__t| __t.current_location.clone()).unwrap()).into_iter().collect::<Vec<_>>();
                { let __o = ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }).into_iter().filter(|w| !(compat::read(w, |__t| __t.from_tab))).map(|w| compat::read(&w, |__t| __t.widget.clone())).collect::<Vec<_>>(); available_widgets.retain(|e| !__o.contains(e)); };
                __this.lock().borrow().add_widget_widget.as_ref().unwrap().open_with(&available_widgets);
                let __r0 = compat::read(__this, |__t| __t.width);
                { let (__q0,) = (((mouse_x) as i32).clamp(5, ((__r0).wrapping_sub(__this.lock().borrow().add_widget_widget.as_ref().unwrap().get_width())).wrapping_sub(5)),); __this.lock().borrow_mut().add_widget_widget.as_mut().unwrap().set_x(__q0) };
                let __r0 = compat::read(__this, |__t| __t.height);
                { let (__q0,) = (((mouse_y) as i32).clamp(5, ((__r0).wrapping_sub(__this.lock().borrow().add_widget_widget.as_ref().unwrap().get_height())).wrapping_sub(5)),); __this.lock().borrow_mut().add_widget_widget.as_mut().unwrap().set_y(__q0) };
                __this.lock().borrow_mut().add_widget_widget.as_mut().unwrap().refresh_scroll_amount();
            }
            return true;
        }
        if !({ let (__a0,) = (compat::read(__this, |__t| __t.selected_widget.clone()),); __this.lock().borrow().hovered_widget.as_ref().unwrap().equals(__a0) }) {
            { let __v = compat::read(__this, |__t| __t.hovered_widget.clone()); compat::write(__this, |__t| __t.selected_widget = __v); }
        }
        mouse_x = (mouse_x / (TabHud::get_scale_factor() as f64));
        mouse_y = (mouse_y / (TabHud::get_scale_factor() as f64));
        if !(compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.from_tab)) {
            { let __v = Some(ScreenPosition::new((((mouse_x - (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }) as f64))) as i32), (((mouse_y - (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) as f64))) as i32))); compat::write(__this, |__t| __t.drag_relative = __v); }
        }
        if click.button() == 1 && !(__this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open()) || !({ let (__a0,) = (__this.lock().borrow().side_panel_widget.as_ref().unwrap().get_positioned_widget(),); __this.lock().borrow().selected_widget.as_ref().unwrap().equals(__a0) }) {
            WidgetsConfigurationScreen::open_side_panel(__this);
        } else if click.button() == 0 && __this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open() && !({ let (__a0,) = (__this.lock().borrow().side_panel_widget.as_ref().unwrap().get_positioned_widget(),); __this.lock().borrow().selected_widget.as_ref().unwrap().equals(__a0) }) {
            WidgetsConfigurationScreen::open_side_panel(__this);
        }
        return true;
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        { let __v = None; compat::write(__this, |__t| __t.drag_relative = __v); }
        if compat::read(__this, |__t| __t.open_panel_after_dragging) {
            { let __v = false; compat::write(__this, |__t| __t.open_panel_after_dragging = __v); }
            WidgetsConfigurationScreen::open_side_panel(__this);
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyInput)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_some() {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap()), std::sync::Arc::as_ptr(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap())) {
                let mut r#move: bool = true;
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                if key_input.is_left() {
                    x = -1;
                } else if key_input.is_right() {
                    x = 1;
                } else if key_input.is_up() {
                    y = -1;
                } else if key_input.is_down() {
                    y = 1;
                } else {
                    r#move = false;
                }
                if r#move {
                    let mut old_rule: PositionRule = compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule.clone());
                    { let __v = PositionRule::new(old_rule.parent(), old_rule.parent_point(), old_rule.this_point(), old_rule.relative_x().wrapping_add(x), old_rule.relative_y().wrapping_add(y)); compat::write(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule = __v); }
                    WidgetsConfigurationScreen::update_builder_positions(__this);
                    return true;
                }
            }
            if key_input.key() == 261 {
                WidgetsConfigurationScreen::remove_widget(__this, &compat::read(__this, |__t| __t.selected_widget.clone()).unwrap());
                return true;
            }
        }
        return (if key_input.is_escape() && true { WidgetsConfigurationScreen::on_close(__this); true } else { false });
    }

    /// java: private void openSidePanel()
    pub fn open_side_panel(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() {
            return;
        }
        let mut right_side: bool = (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_add(({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }).wrapping_div(2)) < (WidgetsConfigurationScreen::get_screen_width(__this)).wrapping_div(2));
        let __h0 = compat::read(__this, |__t| __t.selected_widget.clone()).unwrap();
        __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().open_2(__h0, right_side);
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() && __this.lock().borrow().side_panel_widget.as_ref().unwrap().is_open() {
            __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().close();
        }
        for widget in ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }) {
            // TODO(fabric2seer): the condition: `getRectangle` is not a method of HudWidget
            //   java: widget.widget.getRectangle().intersects(new ScreenRectangle(0, 0, getScreenWidth(), getScreenHeight())) || widget.rule.parent().isPresent()
            if todo!("fabric2seer: the condition (TODO above)") {
                continue;
            }
            { let __v = position_rule::DEFAULT.get_mut().clone(); compat::write(&widget, |__t| __t.rule = __v); }
            WidgetsConfigurationScreen::update_builder_positions(__this);
        }
    }

    /// java: public void promptSelectWidget(Consumer<@Nullable HudWidget> callback, boolean allowItself, Component tooltip)
    pub fn prompt_select_widget(__this: &compat::Shared<Self>, callback: Arc<dyn Fn(Option<compat::Shared<dyn HudWidget>>)>, mut allow_itself: bool, tooltip: &MutableComponent) {
        { let __v = Some(SelectWidgetPrompt::new(callback.clone(), allow_itself, tooltip.clone())); compat::write(__this, |__t| __t.select_widget_prompt = __v); }
        __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().close();
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        __this.lock().borrow().layer.as_ref().unwrap().editor().serialize_config();
        ({ let __r = &({ let __r = widget_manager::SCREEN_BUILDER.get_mut(); let __f = compat::dispatch(__r, |__t| __t.hud()); __f(__r) }); let __f = compat::dispatch(__r, |__t| __t.update()); __f(__r) });
        mcrust::mcrust::modding::game::set_screen_to_parent();
        WidgetManager::save_config();
    }

    /// java: public void removeWidget(PositionedWidget widget)
    pub fn remove_widget(__this: &compat::Shared<Self>, widget: &compat::Shared<PositionedWidget>) {
        __this.lock().borrow().layer.as_ref().unwrap().editor().remove_positioned_widget(widget);
        let mut deleted: PositionRule = compat::read(widget, |__t| __t.rule.clone());
        for positioned_widget in ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }) {
            let mut rule: PositionRule = compat::read(&positioned_widget, |__t| __t.rule.clone());
            if rule.parent().is_none() {
                continue;
            }
            if rule.parent().unwrap() == ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) }) {
                { let __v = PositionRule::new(deleted.parent(), deleted.parent_point(), rule.this_point(), deleted.relative_x().wrapping_add(rule.relative_x()), deleted.relative_y().wrapping_add(rule.relative_y())); compat::write(&positioned_widget, |__t| __t.rule = __v); }
            }
        }
        WidgetsConfigurationScreen::update_builder_positions(__this);
        let mut location: Location = WidgetsConfigurationScreen::get_current_location(__this);
        if let Some(copy_tracker) = (WidgetManager::get_copy_tracker().get(compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap())).get(&({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })) {
            if let Some(set) = ExclusiveGroupingSet::get_group(&copy_tracker, &location) {
                // TODO(fabric2seer): `addButton` on a void
                //   java: openPopup(screen ->
                //   java: 						new PopupScreen.Builder(screen, Component.translatable("skyblocker.config.hud.copy.delete"))
                //   java: 								.addMessage(Component.translatable("skyblocker.config.hud.copy.delete.description"))
                //   java: 								.addButton(CommonComponents.GUI_YES, popup -> {
                //   java: 									removeCopies(set, widget.widget.getInternalID());
                //   java: 									copyTracker.removeAll(set);
                //   java: 									popup.onClose();
                //   java: 								})
                //   java: 								.addButton(CommonComponents.GUI_NO, popup -> {
                //   java: 									copyTracker.remove(location);
                //   java: 									popup.onClose();
                //   java: 								})
                //   java: 								.build()
                //   java: 				);
            }
        }
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap()), std::sync::Arc::as_ptr(widget)) {
            __this.lock().borrow_mut().side_panel_widget.as_mut().unwrap().close();
            { let __v = None; compat::write(__this, |__t| __t.selected_widget = __v); }
        }
    }

    /// java: private void removeCopies(Set<Location> locations, String widgetId)
    pub fn remove_copies(__this: &compat::Shared<Self>, locations: &std::collections::HashSet<Location>, widget_id: &str) {
        for location in locations.iter().copied() {
            let mut config: LayerConfig = WidgetManager::get_screen_config(location).get(compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap());
            let Some(mut deleted_config) = config.widgets().remove(widget_id) else {
                continue;
            };
            for entry in config.widgets().iter().map(|(k, v)| (k.clone(), v.clone())) {
                let widget_config = &entry.1;
                let mut pos_opt: Option<PositionRule> = widget_config.position();
                if pos_opt.is_none() {
                    continue;
                }
                let mut rule: PositionRule = pos_opt.clone().unwrap();
                if rule.parent().filter(|__a0| (widget_id == __a0)).is_none() {
                    continue;
                }
                let mut new_rule: PositionRule;
                if deleted_config.position().is_some() {
                    let mut old_position: PositionRule = deleted_config.position().unwrap();
                    new_rule = PositionRule::new(old_position.parent(), old_position.parent_point(), old_position.this_point(), old_position.relative_x().wrapping_add(rule.relative_x()), old_position.relative_y().wrapping_add(rule.relative_y()));
                } else {
                    new_rule = position_rule::DEFAULT.get_mut().clone();
                }
                entry.1 = WidgetConfig::new(widget_config.config(), Some(new_rule.clone()));
            }
        }
    }

    /// java: private void updateBuilderPositions()
    pub fn update_builder_positions(__this: &compat::Shared<Self>) {
        ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.update_positions()); __f(__r, WidgetsConfigurationScreen::get_screen_width(__this), WidgetsConfigurationScreen::get_screen_height(__this)) });
    }

    /// java: public HudWidget getEditedWidget()
    pub fn get_edited_widget(__this: &compat::Shared<Self>) -> compat::Shared<dyn HudWidget> {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() {
            // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`message: Option<String>`), Java passes 0
            // rust: logger().warn(&format!("Trying to edit selected widget but nothing is selected?"));
            return (PlaceholderWidget::new("".to_string()) as compat::Shared<dyn HudWidget>);
        }
        return compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone());
    }

    /// java: public WidgetManager.ScreenLayer getCurrentScreenLayer()
    pub fn get_current_screen_layer(__this: &compat::Shared<Self>) -> ScreenLayer {
        return compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap();
    }

    /// java: public Location getCurrentLocation()
    pub fn get_current_location(__this: &compat::Shared<Self>) -> Location {
        return compat::read(__this, |__t| __t.current_location.clone()).unwrap();
    }

    /// java: public ScreenConfig getScreenConfig()
    pub fn get_screen_config(__this: &compat::Shared<Self>) -> ScreenConfig {
        return compat::read(__this, |__t| __t.screen_config.clone()).unwrap();
    }

    /// java: public int getScreenWidth()
    pub fn get_screen_width(__this: &compat::Shared<Self>) -> i32 {
        return ((((compat::read(__this, |__t| __t.width) as f32) / TabHud::get_scale_factor())) as i32);
    }

    /// java: public int getScreenHeight()
    pub fn get_screen_height(__this: &compat::Shared<Self>) -> i32 {
        return ((((compat::read(__this, |__t| __t.height) as f32) / TabHud::get_scale_factor())) as i32);
    }

    /// java: public void openPopup(Function<WidgetsConfigurationScreen, Screen> popupCreator)
    pub fn open_popup(__this: &compat::Shared<Self>, popup_creator: Arc<dyn Fn(compat::Shared<WidgetsConfigurationScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) {
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // rust: gui::set_screen(&mut (popup_creator)(__this.clone()));
    }

}

/// WidgetsConfigurationScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WidgetsConfigurationScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WidgetsConfigurationScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WidgetsConfigurationScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsConfigurationScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsConfigurationScreen::reposition_elements(__h))
    }

    /// java: @Override public void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsConfigurationScreen::tick(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| WidgetsConfigurationScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyInput)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| WidgetsConfigurationScreen::key_pressed(__h, event))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| WidgetsConfigurationScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    fn mouse_released(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| WidgetsConfigurationScreen::mouse_released(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0))))
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double deltaX, double deltaY)
    fn mouse_dragged(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.lend(screen, |__h| WidgetsConfigurationScreen::mouse_dragged(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), dx, dy))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WidgetsConfigurationScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): extractBackground, extractBlurredBackground, extractMenuBackground, extractRenderState, init, keyPressed, mouseClicked, mouseDragged, mouseReleased, onClose, repositionElements, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record SelectWidgetPrompt(Consumer<@Nullable HudWidget> callback, boolean allowItself, Component tooltip)
#[derive(Clone, Debug)]
pub struct SelectWidgetPrompt {
    /// java: Consumer<@Nullable HudWidget> callback
    pub callback: Arc<dyn Fn(Option<compat::Shared<dyn HudWidget>>)>,
    /// java: boolean allowItself
    pub allow_itself: bool,
    /// java: Component tooltip
    pub tooltip: MutableComponent,
}

impl SelectWidgetPrompt {
    /// java: the canonical record constructor
    pub fn new(mut callback: Arc<dyn Fn(Option<compat::Shared<dyn HudWidget>>)>, mut allow_itself: bool, mut tooltip: MutableComponent) -> Self {
        Self { callback, allow_itself, tooltip }
    }

    /// java: the record accessor `callback()`
    pub fn callback(&self) -> Arc<dyn Fn(Option<compat::Shared<dyn HudWidget>>)> {
        self.callback.clone()
    }

    /// java: the record accessor `allowItself()`
    pub fn allow_itself(&self) -> bool {
        self.allow_itself
    }

    /// java: the record accessor `tooltip()`
    pub fn tooltip(&self) -> MutableComponent {
        self.tooltip.clone()
    }

}

/// java: private record SnapResult(@Nullable String parent, PositionRule.Point parentPoint, PositionRule.Point thisPoint, OptionalInt relativeX, OptionalInt relativeY)
#[derive(Clone, Debug, PartialEq)]
pub struct SnapResult {
    /// java: String parent
    pub parent: Option<String>,
    /// java: PositionRule.Point parentPoint
    pub parent_point: Point,
    /// java: PositionRule.Point thisPoint
    pub this_point: Point,
    /// java: OptionalInt relativeX
    pub relative_x: Option<i32>,
    /// java: OptionalInt relativeY
    pub relative_y: Option<i32>,
}

impl SnapResult {
    /// java: the canonical record constructor
    pub fn new(mut parent: Option<String>, mut parent_point: Point, mut this_point: Point, mut relative_x: Option<i32>, mut relative_y: Option<i32>) -> Self {
        Self { parent, parent_point, this_point, relative_x, relative_y }
    }

    /// java: the record accessor `parent()`
    pub fn parent(&self) -> Option<String> {
        self.parent.clone()
    }

    /// java: the record accessor `parentPoint()`
    pub fn parent_point(&self) -> Point {
        self.parent_point.clone()
    }

    /// java: the record accessor `thisPoint()`
    pub fn this_point(&self) -> Point {
        self.this_point.clone()
    }

    /// java: the record accessor `relativeX()`
    pub fn relative_x(&self) -> Option<i32> {
        self.relative_x.clone()
    }

    /// java: the record accessor `relativeY()`
    pub fn relative_y(&self) -> Option<i32> {
        self.relative_y.clone()
    }

}
