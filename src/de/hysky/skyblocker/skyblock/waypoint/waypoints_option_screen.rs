//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsOptionScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Waypoints;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui::argb_text_input::ARGBTextInput;
use crate::de::hysky::skyblocker::utils::render::gui::color_picker_widget::ColorPickerWidget;
use crate::de::hysky::skyblocker::utils::render::gui::ranged_slider_widget::RangedSliderWidget;
use mcrust::java::awt::color::Color;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class WaypointsOptionScreen extends Screen
pub struct WaypointsOptionScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for WaypointsOptionScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointsOptionScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointsOptionScreen {
    /// java: protected WaypointsOptionScreen(Screen parent)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.waypoints.waypointsOptions").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent: todo!("fabric2seer: the field value (TODO above)"),
            layout: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut grid: GridLayout = GridLayout::new_default().spacing(2);
        { let (__q0,) = (LayoutElement::Grid(grid.clone()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        let mut adder: RowHelper = grid.create_row_helper(2);
        let waypoints_read_only = &SkyblockerConfigManager::get().ui_and_visuals.waypoints;
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.render_line)).create(0, 0, 150, 20, component::translatable("skyblocker.config.uiAndVisuals.waypoints.renderLine"), |_, value| { WaypointsOptionScreen::update_config(__this_1.get(), Arc::new(move |waypoints| { waypoints.render_line = value.clone(); })); }).unwrap()).unwrap();
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.allow_skipping_waypoints)).with_tooltip(|_| Tooltip::create((component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints.@Tooltip")).get_string())).create(0, 0, 150, 20, component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints"), |_, value| { WaypointsOptionScreen::update_config(__this_2.get(), Arc::new(move |waypoints| { waypoints.allow_skipping_waypoints = value.clone(); })); }).unwrap()).unwrap();
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_3 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.allow_going_backwards)).with_tooltip(|_| Tooltip::create((component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards.@Tooltip")).get_string())).create(0, 0, 150, 20, component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards"), |_, value| { WaypointsOptionScreen::update_config(__this_3.get(), Arc::new(move |waypoints| { waypoints.allow_going_backwards = value.clone(); })); }).unwrap()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_4 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (((((RangedSliderWidget::builder().option_formatter(&component::translatable("skyblocker.config.uiAndVisuals.waypoints.lineWidth"), formatters::FLOAT_NUMBERS.get_mut())).callback(Arc::new(move |value| WaypointsOptionScreen::update_config(__this_4.get(), Arc::new(move |waypoints| { waypoints.line_width = ((value) as f32); }))))).min_max(((1) as f64) as f64, ((15) as f64) as f64)).default_value((waypoints_read_only.line_width) as f64)).step(0.5)).build()).unwrap();
        // TODO(fabric2seer): `.setTooltip(..)` on a value of unknown type
        //   java: adder.addChild(RangedSliderWidget.builder()
        //   java: 				.optionFormatter(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius"), Formatters.FLOAT_NUMBERS)
        //   java: 				.callback(value -> updateConfig(waypoints -> waypoints.waypointActivationRadius = (float) value))
        //   java: 				.minMax(1, 10)
        //   java: 				.defaultValue(waypointsReadOnly.waypointActivationRadius)
        //   java: 				.step(0.5)
        //   java: 				.build()).setTooltip(Tooltip.create(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius.@Tooltip")));
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(&mut SpacerElement::width(0)).unwrap();
        // TODO(fabric2seer): the port's `add_child_spanning` ports this Java method with other parameters (grid, child, column_width, settings for Java's widget, column_width, layout_settings)
        // TODO(fabric2seer): `add_child_spanning` takes 4 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32, settings: LayoutSettings`), Java passes 3
        // rust: adder.add_child_spanning(&mut compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.uiAndVisuals.waypoints.lineColor")), 2, LayoutSettings::defaults().align_horizontally_center().padding_top(4)).unwrap();
        let mut color_layout: LinearLayout = LinearLayout::horizontal();
        // TODO(fabric2seer): the port's `add_child_spanning` ports this Java method with other parameters (grid, child, column_width, settings for Java's widget, column_width, layout_settings)
        // TODO(fabric2seer): `add_child_spanning` takes 4 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32, settings: LayoutSettings`), Java passes 3
        // rust: adder.add_child_spanning(&mut color_layout, 2, LayoutSettings::defaults().align_horizontally_center()).unwrap();
        let mut color_picker_widget: compat::Shared<ColorPickerWidget> = color_layout.add_child({ let __w = compat::WidgetAdapter::widget(ColorPickerWidget::new_5(0, 0, 200, 100, true)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let mut argb_text_input: compat::Shared<ARGBTextInput> = color_layout.add_child({ let __w = compat::WidgetAdapter::widget(ARGBTextInput::new(0, 0, compat::font(), true, true)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_6 = compat::RenderThread::new(__this.clone());
        ColorPickerWidget::set_on_color_change(&color_picker_widget, Arc::new(move |color, mouse_release| {
            ARGBTextInput::set_argb_color(&argb_text_input, color);
            if mouse_release {
                // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
                // rust: WaypointsOptionScreen::update_config(__this_6.get(), Arc::new(move |waypoints| { waypoints.line_color = Color::new(color, true); }));
            }
        }));
        let __this_7 = compat::RenderThread::new(__this.clone());
        ARGBTextInput::set_on_change(&argb_text_input, Arc::new(move |color| {
            ColorPickerWidget::set_argb_color(&color_picker_widget, color);
            // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
            // rust: WaypointsOptionScreen::update_config(__this_7.get(), Arc::new(move |waypoints| { waypoints.line_color = Color::new(color, true); }));
        }));
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: colorPickerWidget.setARGBColor(waypointsReadOnly.lineColor.getRGB());
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: argbTextInput.setARGBColor(waypointsReadOnly.lineColor.getRGB());
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, mcrust::net::minecraft::network::chat::component::literal(__this.lock().borrow().get_title())); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        let __this_w8 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_8) = __this_w8.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_8, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WaypointsOptionScreen::on_close((&__this_8)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        WaypointsOptionScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: private void updateConfig(Consumer<UIAndVisualsConfig.Waypoints> consumer)
    pub fn update_config(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(Waypoints)>) {
        SkyblockerConfigManager::update_only(Arc::new(move |config| { (consumer)(config.ui_and_visuals.waypoints.clone()); }));
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
        SkyblockerConfigManager::update(Arc::new(move |_| {

        }));
    }

}

/// WaypointsOptionScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WaypointsOptionScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WaypointsOptionScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WaypointsOptionScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WaypointsOptionScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WaypointsOptionScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
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

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WaypointsOptionScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
