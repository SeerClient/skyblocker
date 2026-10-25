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
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
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
    pub side_panel_widget: Option<compat::Shared<SidePanelWidget>>,
    /// java: private AddWidgetWidget addWidgetWidget;
    pub add_widget_widget: Option<compat::Shared<AddWidgetWidget>>,
    /// java: private TopBarWidget topBarWidget;
    pub top_bar_widget: Option<compat::Shared<TopBarWidget>>,
    /// `AbstractContainerEventHandler.focused` over the three children `init` adds (`addWidget(addWidgetWidget);
    /// addWidget(topBarWidget); addWidget(sidePanelWidget);`) - their place in that order
    pub focused_child: Option<usize>,
    /// `AbstractContainerEventHandler.isDragging`
    pub dragging: bool,
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
        let __init_previous_screen = previous_screen;
        let __init_current_location = Some(location);
        let __init_current_screen_layer = Some(ScreenLayer::Hud);
        // java: private final PositionedWidget screenWidget = new PositionedWidget(new HudWidget(new HudWidget.Information(null, null)) {..}, PositionRule.DEFAULT);
        // (the anonymous class is `ScreenHudWidget` below; its handle to the enclosing screen is set once the screen exists)
        let __screen_hud_widget: compat::Shared<ScreenHudWidget> = ScreenHudWidget::new();
        let __init_screen_widget = PositionedWidget::new(__screen_hud_widget.clone() as compat::Shared<dyn HudWidget>, position_rule::DEFAULT.get_mut().clone());
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
            focused_child: None,
            dragging: false,
            screen_widget: __init_screen_widget,
            hovered_widget: None,
            selected_widget: None,
            drag_relative: None,
            open_panel_after_dragging: false,
            auto_anchor: true,
            select_widget_prompt: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = std::sync::Arc::downgrade(&__this); compat::write(&__screen_hud_widget, |__t| __t.outer = __v); }
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
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("hud").executes(Scheduler.queueOpenScreenCommand(WidgetsConfigurationScreen::new))));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut hud = client_command_registration_callback::literal("hud");
                hud.executes(crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen_command(Arc::new(|| {
                    Box::new(compat::ScreenAdapter::new(WidgetsConfigurationScreen::new())) as Box<dyn mcrust::mcrust::modding::screens::ModScreen>
                })));
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&hud);
                dispatcher.dispatcher().register(&root);
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
        { let __v = Some(SidePanelWidget::new((compat::read(__this, |__t| __t.width)).wrapping_div(4), compat::read(__this, |__t| __t.height), __this)); compat::write(__this, |__t| __t.side_panel_widget = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(AddWidgetWidget::new(compat::minecraft(), Arc::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { WidgetsConfigurationScreen::add_widget((&__this_1), __a0.clone()); } }))); compat::write(__this, |__t| __t.add_widget_widget = __v); }
        { let __v = Some(TopBarWidget::new(compat::read(__this, |__t| __t.width), __this)); compat::write(__this, |__t| __t.top_bar_widget = __v); }
        // java: addWidget(addWidgetWidget); addWidget(topBarWidget); addWidget(sidePanelWidget); - `Screen.addWidget`, the
        // event listeners (this class's private `addWidget(HudWidget)` is another overload): `children()` is the three, in
        // that order (`super_mouse_clicked` and its siblings below are `ContainerEventHandler`'s bodies over them), and
        // they are drawn by this class's `extractRenderState`, not by the screen's renderables
        compat::write(__this, |__t| { __t.focused_child = None; __t.dragging = false; });
        WidgetsConfigurationScreen::reposition_elements(__this);
    }

    /// java: private void addWidget(HudWidget widget)
    pub fn add_widget(__this: &compat::Shared<Self>, mut widget: compat::Shared<dyn HudWidget>) {
        { let __v = PositionRule::new(Some("screen".to_string()), position_rule::POINT_DEFAULT.clone(), position_rule::POINT_DEFAULT.clone(), (((compat::minecraft().mouse_handler.get_scaled_x_pos_of(compat::minecraft().get_window()) / (TabHud::get_scale_factor() as f64))) as i32), (((compat::minecraft().mouse_handler.get_scaled_y_pos_of(compat::minecraft().get_window()) / (TabHud::get_scale_factor() as f64))) as i32)); compat::write(&__this.lock().borrow().layer.as_ref().unwrap().editor().add_1(widget.clone()), |__t| __t.rule = __v); }
        WidgetsConfigurationScreen::update_builder_positions(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let (__width, __height) = compat::read(__this, |__t| (__t.width, __t.height));
        compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| {
            __p.set_width(__width.wrapping_div(4));
            __p.set_height(__height);
            if __p.is_open() {
                let __x = if __p.right_side { __width.wrapping_sub(__p.get_width()) } else { 0 };
                __p.set_x(__x);
            }
        });
        compat::write(&WidgetsConfigurationScreen::top_bar(__this), |__t| __t.set_width(__width));
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_background(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // java: super.extractBackground(context, mouseX, mouseY, deltaTicks); - the game's `Screen.extractBackground` has run
        // when this is called (`ModScreen::extract_background_calls_super` answers `true`)
        let mut text: MutableComponent = component::translatable("skyblocker.config.hud.screen.rightClick");
        let mut text_width: i32 = compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), ((compat::read(__this, |__t| __t.width)).wrapping_sub(text_width)).wrapping_div(2), ((compat::read(__this, |__t| __t.height)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), 0xFFFFFFFFu32 as i32);
    }

    /// java: @Override protected void extractBlurredBackground(GuiGraphicsExtractor graphics)
    /// - `if (minecraft.level != null && !minecraft.hasControlDown()) super.extractBlurredBackground(graphics);`: whether
    /// this frame's override calls `super` (`ModScreen::extract_blurred_background_calls_super`)
    pub fn extract_blurred_background_calls_super() -> bool {
        mcrust::mcrust::modding::game::level().is_some() && !(minecraft::has_control_down())
    }

    /// java: @Override protected void extractMenuBackground(GuiGraphicsExtractor graphics)
    /// - `if (minecraft.level != null && !minecraft.hasControlDown()) super.extractMenuBackground(graphics);`
    /// (`ModScreen::extract_menu_background_calls_super`)
    pub fn extract_menu_background_calls_super() -> bool {
        mcrust::mcrust::modding::game::level().is_some() && !(minecraft::has_control_down())
    }

    /// The side panel (`sidePanelWidget`, set by `init`)
    pub fn side_panel(__this: &compat::Shared<Self>) -> compat::Shared<SidePanelWidget> {
        compat::read(__this, |__t| __t.side_panel_widget.clone()).expect("sidePanelWidget before init()")
    }

    /// The top bar (`topBarWidget`, set by `init`)
    pub fn top_bar(__this: &compat::Shared<Self>) -> compat::Shared<TopBarWidget> {
        compat::read(__this, |__t| __t.top_bar_widget.clone()).expect("topBarWidget before init()")
    }

    /// The add-widget list (`addWidgetWidget`, set by `init`)
    pub fn add_widget_list(__this: &compat::Shared<Self>) -> compat::Shared<AddWidgetWidget> {
        compat::read(__this, |__t| __t.add_widget_widget.clone()).expect("addWidgetWidget before init()")
    }

    /// `child.isMouseOver(x, y)` for `children()`'s `index`th (`addWidgetWidget`, `topBarWidget`, `sidePanelWidget`)
    fn child_is_mouse_over(__this: &compat::Shared<Self>, index: usize, x: f64, y: f64) -> bool {
        match index {
            0 => compat::read(&WidgetsConfigurationScreen::add_widget_list(__this), |__t| __t.is_mouse_over(x, y)),
            1 => compat::read(&WidgetsConfigurationScreen::top_bar(__this), |__t| __t.is_mouse_over(x, y)),
            _ => compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__t| __t.is_mouse_over(x, y)),
        }
    }

    /// `ContainerEventHandler.getChildAt(x, y)` over `children()`
    pub fn get_child_at(__this: &compat::Shared<Self>, x: f64, y: f64) -> Option<usize> {
        if compat::read(__this, |__t| __t.side_panel_widget.is_none()) {
            return None;
        }
        (0..3usize).find(|&i| WidgetsConfigurationScreen::child_is_mouse_over(__this, i, x, y))
    }

    /// `super.mouseClicked(click, doubled)` - `ContainerEventHandler.mouseClicked`: the child under the pointer takes
    /// the press (it is focused, and dragging for the left button, when it handled it); `true` whenever there is one
    fn super_mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let Some(index) = WidgetsConfigurationScreen::get_child_at(__this, click.x(), click.y()) else {
            return false;
        };
        let handled = match index {
            0 => AddWidgetWidget::mouse_clicked(&WidgetsConfigurationScreen::add_widget_list(__this), click, doubled),
            1 => TopBarWidget::mouse_clicked(&WidgetsConfigurationScreen::top_bar(__this), click, doubled),
            _ => SidePanelWidget::mouse_clicked(&WidgetsConfigurationScreen::side_panel(__this), click, doubled),
        };
        if handled {
            // java: this.setFocused(widget); if (event.button() == 0) this.setDragging(true);
            compat::write(__this, |__t| {
                __t.focused_child = Some(index);
                if click.button() == 1 {
                    __t.dragging = true;
                }
            });
        }
        true
    }

    /// `super.mouseReleased(click)` - `ContainerEventHandler.mouseReleased`: the focused child, when it was dragging
    fn super_mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        let (focused, dragging) = compat::read(__this, |__t| (__t.focused_child, __t.dragging));
        if click.button() == 1 && dragging {
            compat::write(__this, |__t| __t.dragging = false);
            return match focused {
                Some(0) => AddWidgetWidget::mouse_released(&WidgetsConfigurationScreen::add_widget_list(__this), click),
                Some(1) => TopBarWidget::mouse_released(&WidgetsConfigurationScreen::top_bar(__this), click),
                Some(_) => SidePanelWidget::mouse_released(&WidgetsConfigurationScreen::side_panel(__this), click),
                None => false,
            };
        }
        false
    }

    /// `super.mouseDragged(click, dx, dy)` - `ContainerEventHandler.mouseDragged`: the focused, dragging child's
    fn super_mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let (focused, dragging) = compat::read(__this, |__t| (__t.focused_child, __t.dragging));
        if !dragging || click.button() != 1 {
            return false;
        }
        match focused {
            Some(0) => AddWidgetWidget::mouse_dragged(&WidgetsConfigurationScreen::add_widget_list(__this), click, dx, dy),
            Some(1) => TopBarWidget::mouse_dragged(&WidgetsConfigurationScreen::top_bar(__this), click, dx, dy),
            Some(_) => SidePanelWidget::mouse_dragged(&WidgetsConfigurationScreen::side_panel(__this), click, dx, dy),
            None => false,
        }
    }

    /// java: `ContainerEventHandler.mouseScrolled` - `getChildAt(x, y).filter(child -> child.mouseScrolled(..)).isPresent()`
    /// (this screen does not override it)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        match WidgetsConfigurationScreen::get_child_at(__this, x, y) {
            Some(0) => AddWidgetWidget::mouse_scrolled(&WidgetsConfigurationScreen::add_widget_list(__this), x, y, scroll_x, scroll_y),
            Some(1) => TopBarWidget::mouse_scrolled(&WidgetsConfigurationScreen::top_bar(__this), x, y, scroll_x, scroll_y),
            Some(_) => SidePanelWidget::mouse_scrolled(&WidgetsConfigurationScreen::side_panel(__this), x, y, scroll_x, scroll_y),
            None => false,
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    /// (`render` is the frame's screen render context: the three children's draws take a widget render context)
    pub fn extract_render_state(__this: &compat::Shared<Self>, render: &mut ScreenRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // java: super.extractRenderState(context, mouseX, mouseY, deltaTicks); - the generic pass has drawn the screen's
        // renderables (none: the three children are `addWidget` listeners, drawn below)
        let context: &mut GuiGraphicsExtractor = &mut render.graphics;
        let mut scale: f32 = TabHud::get_scale_factor();
        let __pose_1 = gui_graphics_extractor::push_pose();
        // java: matrices.scale(scale);
        gui_graphics_extractor::pose_scale(scale, scale);
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
        // java: Matrix3x2f scaleMatrix = new Matrix3x2f().scale(scale); - `new Matrix3x2f()` is the identity
        let mut scale_matrix: Matrix3x2f = mcrust::org::joml::matrix3x2f::IDENTITY.scale_xy(scale);
        if compat::read(__this, |__t| __t.hovered_widget.clone()).is_some() && { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); WidgetsConfigurationScreen::get_child_at(__this, __q0, __q1) }.is_none() {
            // java: ScreenRectangle rect = hoveredWidget.widget.getRectangle().transformAxisAligned(scaleMatrix);
            let mut rect: ScreenRectangle = hud_widget_rectangle(&compat::read(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap(), |__t| __t.widget.clone())).transform_axis_aligned(&scale_matrix);
            GuiHelper::border(context, rect.left().wrapping_sub(1), rect.top().wrapping_sub(1), rect.width().wrapping_add(2), rect.height().wrapping_add(2), mcrust::net::minecraft::util::common_colors::YELLOW);
        }
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_some() {
            // java: ScreenRectangle rect = selectedWidget.widget.getRectangle().transformAxisAligned(scaleMatrix);
            let mut rect: ScreenRectangle = hud_widget_rectangle(&compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone())).transform_axis_aligned(&scale_matrix);
            GuiHelper::border(context, rect.left().wrapping_sub(1), rect.top().wrapping_sub(1), rect.width().wrapping_add(2), rect.height().wrapping_add(2), mcrust::net::minecraft::util::common_colors::GREEN);
        }
        // java: topBarWidget.visible = selectedWidget == null || selectedWidget.widget.getY() >= 16; - the widget is read only when selected
        let __visible: bool = match compat::read(__this, |__t| __t.selected_widget.clone()) {
            None => true,
            Some(__selected) => { let __r = compat::read(&__selected, |__t| __t.widget.clone()); let __f = compat::dispatch(&__r, |__t| __t.get_y()); __f(&__r) >= 16 }
        };
        compat::write(&WidgetsConfigurationScreen::top_bar(__this), |__t| __t.visible = __visible);
        {
            let __minecraft: &mcrust::net::minecraft::client::minecraft::Minecraft = &*render.minecraft;
            let mut __widgets = mcrust::mcrust::modding::widgets::WidgetRenderContext::new(render.gl, __minecraft);
            // java: sidePanelWidget.extractRenderState(context, mouseX, mouseY, deltaTicks);
            SidePanelWidget::extract_render_state(&WidgetsConfigurationScreen::side_panel(__this), &mut __widgets, mouse_x, mouse_y, delta_ticks);
            // Render on top of everything
            // java: topBarWidget.extractRenderState(context, mouseX, mouseY, deltaTicks);
            TopBarWidget::extract_render_state(&WidgetsConfigurationScreen::top_bar(__this), &mut __widgets, mouse_x, mouse_y, delta_ticks);
            // java: addWidgetWidget.extractRenderState(context, mouseX, mouseY, deltaTicks);
            AddWidgetWidget::extract_render_state(&WidgetsConfigurationScreen::add_widget_list(__this), &mut __widgets, mouse_x, mouse_y, delta_ticks);
        }
        let context: &mut GuiGraphicsExtractor = &mut render.graphics;
        if compat::read(__this, |__t| __t.current_location.clone()).unwrap() != Utils::get_location() {
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.screen.locationWarning[0]")), (compat::read(__this, |__t| __t.width)).wrapping_div(2), (compat::read(__this, |__t| __t.height)).wrapping_sub(30), 0xFFFFFFFFu32 as i32);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.screen.locationWarning[1]")), (compat::read(__this, |__t| __t.width)).wrapping_div(2), (compat::read(__this, |__t| __t.height)).wrapping_sub(20), 0xFFFFFFFFu32 as i32);
        }
        if compat::read(__this, |__t| __t.select_widget_prompt.clone()).is_some() {
            // java: context.setTooltipForNextFrame(selectWidgetPrompt.tooltip(), mouseX, mouseY);
            let __tooltip = __this.lock().borrow().select_widget_prompt.as_ref().unwrap().tooltip();
            compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__tooltip)]);
        }
        // java: screenWidget.widget.setPosition(0, 0); - `LayoutElement.setPosition`: setX then setY
        { let __r = compat::read(&compat::read(__this, |__t| __t.screen_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(&__r, |__t| __t.set_x()); __f(&__r, 0); let __f = compat::dispatch(&__r, |__t| __t.set_y()); __f(&__r, 0); }
        ({ let __r = &compat::read(&compat::read(__this, |__t| __t.screen_widget.clone()), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(__r, context, delta_ticks) });
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double deltaX, double deltaY)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut delta_x: f64, mut delta_y: f64) -> bool {
        // java: if (super.mouseDragged(click, deltaX, deltaY)) return true;
        if WidgetsConfigurationScreen::super_mouse_dragged(__this, click, delta_x, delta_y) {
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
            // java: new PositionRule(Optional.ofNullable(pos.parent), ..) - the canonical constructor
            { let __v = PositionRule::new_canonical(pos.parent.clone(), pos.parent_point.clone(), pos.this_point.clone(), pos.relative_x.clone().unwrap_or(((((mouse_x) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().x())).wrapping_sub(start_position.x())).wrapping_add((((({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }) as f32) * pos.this_point.horizontal_point().get_percentage())) as i32)), pos.relative_y.clone().unwrap_or(((((mouse_y) as i32).wrapping_sub(__this.lock().borrow().drag_relative.as_ref().unwrap().y())).wrapping_sub(start_position.y())).wrapping_add((((({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }) as f32) * pos.this_point.vertical_point().get_percentage())) as i32))); compat::write(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule = __v); }
            WidgetsConfigurationScreen::update_builder_positions(__this);
            let mut side_panel: ScreenRectangle = compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__p| ScreenRectangle::new_at(__p.get_x(), __p.get_y(), __p.get_width(), __p.get_height()));
            let mut selected: ScreenRectangle = ScreenRectangle::new_at(({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }), ({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }));
            if compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.is_open()) && side_panel.overlaps(selected.clone()) {
                compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.close());
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
        // java: ScreenRectangle[] selectedSnapBoxes = Arrays.stream(ScreenDirection.values()).map(dir -> ScreenUtils.getSnapBox(selectedRect, dir)).toArray(ScreenRectangle[]::new);
        let mut selected_snap_boxes: Vec<ScreenRectangle> = screen_direction_values().iter().cloned().map(|dir| ScreenUtils::get_snap_box(&selected_rect, &dir)).collect::<Vec<_>>();
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
        // java: ScreenRectangle otherRect = otherWidget.widget.getRectangle();
        let mut other_rect: ScreenRectangle = hud_widget_rectangle(&compat::read(other_widget, |__t| __t.widget.clone()));
        let mut point: Point = WidgetsConfigurationScreen::get_point(__this, &compat::read(other_widget, |__t| __t.widget.clone()));
        let mut dist_x: i32 = 10;
        let mut dist_y: i32 = 10;
        let mut parent_point_h: Option<HorizontalPoint> = None;
        let mut this_point_h: Option<HorizontalPoint> = None;
        let mut parent_point_v: Option<VerticalPoint> = None;
        let mut this_point_v: Option<VerticalPoint> = None;
        let mut relative_x: Option<i32> = None;
        let mut relative_y: Option<i32> = None;
        for other_edge in screen_direction_values() {
            let mut select_edge: ScreenDirection = other_edge.get_opposite();
            let mut other_snap_box: ScreenRectangle = ScreenUtils::get_snap_box(&other_rect, &other_edge);
            // java: ScreenRectangle selectedSnapBox = selectedSnapBoxes[selectEdge.ordinal()];
            let mut selected_snap_box: ScreenRectangle = selected_snap_boxes[screen_direction_ordinal(select_edge)];
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
        for edge in screen_direction_values() {
            let mut other_snap_box: ScreenRectangle = ScreenUtils::get_snap_box(&other_rect, &edge);
            // java: ScreenRectangle selectedSnapBox = selectedSnapBoxes[edge.ordinal()];
            let mut selected_snap_box: ScreenRectangle = selected_snap_boxes[screen_direction_ordinal(edge)];
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
        return Some((Some(SnapResult::new(Some(({ let __r = &compat::read(other_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })), parent_point.clone(), this_point.clone(), relative_x.clone(), relative_y.clone())), dist));
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
        // java: if (super.mouseClicked(click, doubled)) return true;
        if WidgetsConfigurationScreen::super_mouse_clicked(__this, click, doubled) {
            return true;
        }
        let mut mouse_x: f64 = click.x();
        let mut mouse_y: f64 = click.y();
        if compat::read(__this, |__t| __t.select_widget_prompt.clone()).is_some() {
            if compat::read(__this, |__t| __t.hovered_widget.clone()).is_some() && !(__this.lock().borrow().select_widget_prompt.as_ref().unwrap().allow_itself()) && positioned_equals(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap(), &compat::read(__this, |__t| __t.selected_widget.clone())) {
                return true;
            }
            (__this.lock().borrow().select_widget_prompt.as_ref().unwrap().callback())((if compat::read(__this, |__t| __t.hovered_widget.clone()).is_none() { None } else { Some(compat::read(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap(), |__t| __t.widget.clone())) }));
            { let __v = None; compat::write(__this, |__t| __t.select_widget_prompt = __v); }
            compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.open());
            return true;
        }
        if compat::read(__this, |__t| __t.hovered_widget.clone()).is_none() {
            compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| if __p.is_open() { __p.close(); });
            { let __v = None; compat::write(__this, |__t| __t.selected_widget = __v); }
            if click.button() == 3 {
                let mut available_widgets: Vec<compat::Shared<dyn HudWidget>> = WidgetManager::get_widgets_available_in(compat::read(__this, |__t| __t.current_location.clone()).unwrap()).into_iter().collect::<Vec<_>>();
                // java: availableWidgets.removeAll(..) - `HudWidget.equals` (the same class and internal id)
                { let __o = ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }).into_iter().filter(|w| !(compat::read(w, |__t| __t.from_tab))).map(|w| compat::read(&w, |__t| __t.widget.clone())).collect::<Vec<_>>(); available_widgets.retain(|e| !<dyn HudWidget>::contains_equal(&__o, e)); };
                let __list = WidgetsConfigurationScreen::add_widget_list(__this);
                AddWidgetWidget::open_with(&__list, &available_widgets);
                let (__width, __height) = compat::read(__this, |__t| (__t.width, __t.height));
                // java: addWidgetWidget.setX(Math.clamp((int) mouseX, 5, width - addWidgetWidget.getWidth() - 5)); (and setY)
                // (`Math.clamp` throws when min > max; `max(lo)` keeps the bounds ordered for a screen narrower than the list)
                compat::write(&__list, |__t| {
                    let __hi = (__width.wrapping_sub(__t.get_width())).wrapping_sub(5);
                    let __x = ((mouse_x) as i32).clamp(5, __hi.max(5));
                    __t.set_x(__x);
                    let __hi = (__height.wrapping_sub(__t.get_height())).wrapping_sub(5);
                    let __y = ((mouse_y) as i32).clamp(5, __hi.max(5));
                    __t.set_y(__y);
                    // refreshes the positions of the entries
                    __t.refresh_scroll_amount();
                });
            }
            return true;
        }
        if !(positioned_equals(&compat::read(__this, |__t| __t.hovered_widget.clone()).unwrap(), &compat::read(__this, |__t| __t.selected_widget.clone()))) {
            { let __v = compat::read(__this, |__t| __t.hovered_widget.clone()); compat::write(__this, |__t| __t.selected_widget = __v); }
        }
        mouse_x = (mouse_x / (TabHud::get_scale_factor() as f64));
        mouse_y = (mouse_y / (TabHud::get_scale_factor() as f64));
        if !(compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.from_tab)) {
            { let __v = Some(ScreenPosition::new((((mouse_x - (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }) as f64))) as i32), (((mouse_y - (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) as f64))) as i32))); compat::write(__this, |__t| __t.drag_relative = __v); }
        }
        // java: if (click.button() == MOUSE_BUTTON_RIGHT && (!sidePanelWidget.isOpen() || !selectedWidget.equals(sidePanelWidget.getPositionedWidget())))
        let __panel_widget: Option<compat::Shared<PositionedWidget>> = compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.get_positioned_widget());
        let __selected: compat::Shared<PositionedWidget> = compat::read(__this, |__t| __t.selected_widget.clone()).unwrap();
        let __panel_open: bool = compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.is_open());
        if click.button() == 3 && (!__panel_open || !(positioned_equals(&__selected, &__panel_widget))) {
            WidgetsConfigurationScreen::open_side_panel(__this);
        } else if click.button() == 1 && __panel_open && !(positioned_equals(&__selected, &__panel_widget)) {
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
        // java: return super.mouseReleased(click);
        return WidgetsConfigurationScreen::super_mouse_released(__this, click);
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyInput)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_some() {
            // java: if (selectedWidget == hoveredWidget) - the same object (a null hover is not the selection)
            if compat::read(__this, |__t| __t.hovered_widget.as_ref().is_some_and(|__h| std::sync::Arc::ptr_eq(__h, __t.selected_widget.as_ref().unwrap()))) {
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
                    { let __v = PositionRule::new_canonical(old_rule.parent(), old_rule.parent_point(), old_rule.this_point(), old_rule.relative_x().wrapping_add(x), old_rule.relative_y().wrapping_add(y)); compat::write(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.rule = __v); }
                    WidgetsConfigurationScreen::update_builder_positions(__this);
                    return true;
                }
            }
            if key_input.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_DELETE {
                WidgetsConfigurationScreen::remove_widget(__this, &compat::read(__this, |__t| __t.selected_widget.clone()).unwrap());
                return true;
            }
        }
        // java: return super.keyPressed(keyInput); - `Screen.keyPressed`: Escape closes (`shouldCloseOnEsc()` is `true`),
        // otherwise `ContainerEventHandler.keyPressed` - the focused child's (the side panel's focused option widget);
        // the focus navigation that follows is the loop's generic pass, which runs when this answers `false`
        if key_input.is_escape() {
            WidgetsConfigurationScreen::on_close(__this);
            return true;
        }
        match compat::read(__this, |__t| __t.focused_child) {
            Some(2) => SidePanelWidget::key_pressed(&WidgetsConfigurationScreen::side_panel(__this), key_input),
            _ => false,
        }
    }

    /// java: private void openSidePanel()
    pub fn open_side_panel(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() {
            return;
        }
        let mut right_side: bool = (({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_add(({ let __r = &compat::read(&compat::read(__this, |__t| __t.selected_widget.clone()).unwrap(), |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }).wrapping_div(2)) < (WidgetsConfigurationScreen::get_screen_width(__this)).wrapping_div(2));
        let __h0 = compat::read(__this, |__t| __t.selected_widget.clone()).unwrap();
        SidePanelWidget::open_2(&WidgetsConfigurationScreen::side_panel(__this), __h0, right_side);
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_widget.clone()).is_none() && compat::read(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.is_open()) {
            compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.close());
        }
        for widget in ({ let __r = &__this.lock().borrow().layer.as_ref().unwrap().builder(); let __f = compat::dispatch(__r, |__t| __t.get_rendered()); __f(__r) }) {
            // java: widget.widget.getRectangle().intersects(new ScreenRectangle(0, 0, getScreenWidth(), getScreenHeight())) || widget.rule.parent().isPresent()
            if hud_widget_rectangle(&compat::read(&widget, |__t| __t.widget.clone())).intersects(ScreenRectangle::new_at(0, 0, WidgetsConfigurationScreen::get_screen_width(__this), WidgetsConfigurationScreen::get_screen_height(__this))) || compat::read(&widget, |__t| __t.rule.parent().is_some()) {
                continue;
            }
            { let __v = position_rule::DEFAULT.get_mut().clone(); compat::write(&widget, |__t| __t.rule = __v); }
            WidgetsConfigurationScreen::update_builder_positions(__this);
        }
    }

    /// java: public void promptSelectWidget(Consumer<@Nullable HudWidget> callback, boolean allowItself, Component tooltip)
    pub fn prompt_select_widget(__this: &compat::Shared<Self>, callback: Arc<dyn Fn(Option<compat::Shared<dyn HudWidget>>)>, mut allow_itself: bool, tooltip: &MutableComponent) {
        { let __v = Some(SelectWidgetPrompt::new(callback.clone(), allow_itself, tooltip.clone())); compat::write(__this, |__t| __t.select_widget_prompt = __v); }
        compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.close());
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
                { let __v = PositionRule::new_canonical(deleted.parent(), deleted.parent_point(), rule.this_point(), deleted.relative_x().wrapping_add(rule.relative_x()), deleted.relative_y().wrapping_add(rule.relative_y())); compat::write(&positioned_widget, |__t| __t.rule = __v); }
            }
        }
        WidgetsConfigurationScreen::update_builder_positions(__this);
        let mut location: Location = WidgetsConfigurationScreen::get_current_location(__this);
        if let Some(copy_tracker) = (WidgetManager::get_copy_tracker().get(compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap())).get(&({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })) {
            if let Some(set) = ExclusiveGroupingSet::get_group(&copy_tracker, &location) {
                // java: openPopup(screen -> new PopupScreen.Builder(screen, Component.translatable("skyblocker.config.hud.copy.delete"))
                //           .addMessage(Component.translatable("skyblocker.config.hud.copy.delete.description"))
                //           .addButton(CommonComponents.GUI_YES, popup -> { removeCopies(set, widget.widget.getInternalID()); copyTracker.removeAll(set); popup.onClose(); })
                //           .addButton(CommonComponents.GUI_NO, popup -> { copyTracker.remove(location); popup.onClose(); })
                //           .build());
                let __widget_id: String = { let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) };
                let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
                let __tracker = compat::RenderThread::new(copy_tracker.clone());
                let __set = set.clone();
                WidgetsConfigurationScreen::open_popup(__this, Arc::new(move |_screen| {
                    let (__this_yes, __tracker_yes, __set_yes, __id_yes) = (__this_w.get().clone(), __tracker.get().clone(), __set.clone(), __widget_id.clone());
                    let __tracker_no = __tracker.get().clone();
                    Box::new(compat::BatchUPopupScreenBuilder::new(component::translatable("skyblocker.config.hud.copy.delete"))
                        .add_message(component::translatable("skyblocker.config.hud.copy.delete.description"))
                        .add_button(component::translatable("gui.yes"), move |popup| {
                            if let Some(__screen) = __this_yes.upgrade() {
                                WidgetsConfigurationScreen::remove_copies(&__screen, &__set_yes, &__id_yes);
                            }
                            ExclusiveGroupingSet::remove_all(&__tracker_yes, &__set_yes.iter().copied().collect::<Vec<_>>());
                            popup.on_close();
                        })
                        .add_button(component::translatable("gui.no"), move |popup| {
                            ExclusiveGroupingSet::remove(&__tracker_no, &location);
                            popup.on_close();
                        })
                        .build()) as Box<dyn ModScreen>
                }));
            }
        }
        // java: if (selectedWidget == widget) - a null selection is not the widget
        if compat::read(__this, |__t| __t.selected_widget.as_ref().is_some_and(|__s| std::sync::Arc::ptr_eq(__s, widget))) {
            compat::write(&WidgetsConfigurationScreen::side_panel(__this), |__p| __p.close());
            { let __v = None; compat::write(__this, |__t| __t.selected_widget = __v); }
        }
    }

    /// java: private void removeCopies(Set<Location> locations, String widgetId)
    pub fn remove_copies(__this: &compat::Shared<Self>, locations: &std::collections::HashSet<Location>, widget_id: &str) {
        for location in locations.iter().copied() {
            let mut config: LayerConfig = WidgetManager::get_screen_config(location).get(compat::read(__this, |__t| __t.current_screen_layer.clone()).unwrap());
            // java: WidgetConfig deletedConfig = config.widgets().remove(widgetId); if (deletedConfig == null) continue;
            let Some(mut deleted_config) = compat::write(&config.widgets(), |__m| __m.remove(widget_id)) else {
                continue;
            };
            // java: for (Map.Entry<String, WidgetConfig> entry : config.widgets().entrySet()) { .. entry.setValue(..) }
            let __keys: Vec<String> = compat::read(&config.widgets(), |__m| __m.keys().cloned().collect());
            for __key in __keys.iter() {
                let widget_config: WidgetConfig = compat::read(&config.widgets(), |__m| __m.get(__key).cloned()).unwrap();
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
                    new_rule = PositionRule::new_canonical(old_position.parent(), old_position.parent_point(), old_position.this_point(), old_position.relative_x().wrapping_add(rule.relative_x()), old_position.relative_y().wrapping_add(rule.relative_y()));
                } else {
                    new_rule = position_rule::DEFAULT.get_mut().clone();
                }
                let __v = WidgetConfig::new_canonical(widget_config.config(), Some(new_rule.clone()));
                compat::write(&config.widgets(), |__m| { __m.insert(__key.clone(), __v); });
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
            logger().warn("Trying to edit selected widget but nothing is selected?");
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
    /// (a popup is the mod's screen object; it is opened over this one, which it returns to - Java's popups hold this
    /// screen as their background and `setScreen` it back on close)
    pub fn open_popup(__this: &compat::Shared<Self>, popup_creator: Arc<dyn Fn(compat::Shared<WidgetsConfigurationScreen>) -> Box<dyn ModScreen>>) {
        // java: minecraft.setScreen(popupCreator.apply(this));
        mcrust::mcrust::modding::game::set_screen_keeping_parent((popup_creator)(__this.clone()));
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

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    fn extract_background(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| WidgetsConfigurationScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| WidgetsConfigurationScreen::extract_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void extractBlurredBackground(GuiGraphicsExtractor graphics)
    fn extract_blurred_background_calls_super(&self) -> bool {
        WidgetsConfigurationScreen::extract_blurred_background_calls_super()
    }

    /// java: @Override protected void extractMenuBackground(GuiGraphicsExtractor graphics)
    fn extract_menu_background_calls_super(&self) -> bool {
        WidgetsConfigurationScreen::extract_menu_background_calls_super()
    }

    /// java: `ContainerEventHandler.mouseScrolled` over `children()` (not overridden by the class)
    fn mouse_scrolled(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| WidgetsConfigurationScreen::mouse_scrolled(__h, x, y, scroll_x, scroll_y))
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


/// java: private record SelectWidgetPrompt(Consumer<@Nullable HudWidget> callback, boolean allowItself, Component tooltip)
#[derive(Clone)]
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

/// `ScreenDirection.values()` - the enum's declaration order (`UP, DOWN, LEFT, RIGHT`)
fn screen_direction_values() -> [ScreenDirection; 4] {
    [ScreenDirection::Up, ScreenDirection::Down, ScreenDirection::Left, ScreenDirection::Right]
}

/// `ScreenDirection.ordinal()`
fn screen_direction_ordinal(direction: ScreenDirection) -> usize {
    screen_direction_values().iter().position(|d| *d == direction).unwrap()
}

/// `LayoutElement.getRectangle()` (HudWidget implements LayoutElement): `new ScreenRectangle(getX(), getY(), getWidth(), getHeight())`
fn hud_widget_rectangle(widget: &compat::Shared<dyn HudWidget>) -> ScreenRectangle {
    let x = { let __f = compat::dispatch(widget, |__t| __t.get_x()); __f(widget) };
    let y = { let __f = compat::dispatch(widget, |__t| __t.get_y()); __f(widget) };
    let w = { let __f = compat::dispatch(widget, |__t| __t.get_width()); __f(widget) };
    let h = { let __f = compat::dispatch(widget, |__t| __t.get_height()); __f(widget) };
    ScreenRectangle::new_at(x, y, w, h)
}

/// `positionedWidget.equals(other)` with a nullable `other` (`PositionedWidget.equals(null)` is false)
fn positioned_equals(widget: &compat::Shared<PositionedWidget>, other: &Option<compat::Shared<PositionedWidget>>) -> bool {
    other.as_ref().is_some_and(|other| PositionedWidget::equals_shared(widget, other))
}

/// java: the anonymous `new HudWidget(new HudWidget.Information(null, null)) {..}` of `screenWidget`: a widget that
/// covers the entire screen, used for snapping
pub struct ScreenHudWidget {
    /// java: extends HudWidget
    pub base: HudWidgetBase,
    /// the enclosing `WidgetsConfigurationScreen.this` (an anonymous class's outer instance)
    pub outer: compat::WeakShared<WidgetsConfigurationScreen>,
}

impl std::ops::Deref for ScreenHudWidget {
    type Target = HudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ScreenHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ScreenHudWidget {
    fn new() -> compat::Shared<Self> {
        let (_, __this) = HudWidgetBase::new_in(Information::new(None, None), move |__base| {
            let __h = compat::shared(Self { base: __base, outer: std::sync::Weak::new() });
            ((__h.clone() as compat::Shared<dyn HudWidget>), __h)
        });
        __this
    }

    /// java: { w = getScreenWidth(); h = getScreenHeight(); } - the body of both overrides
    fn cover_screen(__this: &compat::Shared<Self>) {
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else {
            return;
        };
        let w = WidgetsConfigurationScreen::get_screen_width(&outer);
        let h = WidgetsConfigurationScreen::get_screen_height(&outer);
        compat::write(__this, |__t| { __t.base.w = w; __t.base.h = h; });
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn HudWidget>, _graphics: &mut GuiGraphicsExtractor, _delta: f32) {
        ScreenHudWidget::cover_screen(&compat::view_as::<Self, _, Self>(__this, |a| a));
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_for_config_dyn(__this: &compat::Shared<dyn HudWidget>, _graphics: &mut GuiGraphicsExtractor, _delta: f32) {
        ScreenHudWidget::cover_screen(&compat::view_as::<Self, _, Self>(__this, |a| a));
    }
}

impl HudWidget for ScreenHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        ScreenHudWidget::extract_widget_render_state_dyn
    }

    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        ScreenHudWidget::extract_widget_render_state_for_config_dyn
    }
}
