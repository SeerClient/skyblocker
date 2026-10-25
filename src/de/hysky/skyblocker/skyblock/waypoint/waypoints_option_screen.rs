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
    /// `None`: the screen is opened with `set_screen_keeping_parent`, and `onClose` returns to that parent through the
    /// game's screen stack (`set_screen_to_parent`), as the mod's other screens do.
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
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
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.waypoints.waypointsOptions").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent,
            layout: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let mut grid: GridLayout = GridLayout::new_default();
        grid.spacing(2);
        // java: layout.addToContents(grid) - the port's layout holds the grid BY VALUE, so it is added once its rows are
        // in (below); Java adds the reference first and fills it after, which arranges the same.
        let mut adder: RowHelper = grid.create_row_helper(2);
        let waypoints_read_only = &SkyblockerConfigManager::get().ui_and_visuals.waypoints;
        // java: adder.addChild(CycleButton.booleanBuilder(CommonComponents.GUI_YES, CommonComponents.OPTION_OFF, waypointsReadOnly.renderLine)
        //   .create(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.renderLine"), (_, value) -> updateConfig(waypoints -> waypoints.renderLine = value)));
        {
            let __builder = cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.render_line);
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.uiAndVisuals.waypoints.renderLine"), __builder, move |__index: usize| {
                let Some(__this_1) = __this_w.get().upgrade() else { return; };
                let value = __index == 0;
                WaypointsOptionScreen::update_config(&__this_1, Arc::new(move |waypoints: &mut Waypoints| { waypoints.render_line = value; }));
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            adder.add_child(&mut grid, __e).unwrap();
        }
        // java: adder.addChild(CycleButton.booleanBuilder(CommonComponents.GUI_YES, CommonComponents.OPTION_OFF, waypointsReadOnly.allowSkippingWaypoints).withTooltip(..)
        //   .create(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints"), (_, value) -> updateConfig(waypoints -> waypoints.allowSkippingWaypoints = value)));
        {
            let __builder = cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.allow_skipping_waypoints)
                .with_tooltip(cycle_button::TooltipSupplier::Constant(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints.@Tooltip"))));
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints"), __builder, move |__index: usize| {
                let Some(__this_1) = __this_w.get().upgrade() else { return; };
                let value = __index == 0;
                WaypointsOptionScreen::update_config(&__this_1, Arc::new(move |waypoints: &mut Waypoints| { waypoints.allow_skipping_waypoints = value; }));
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            adder.add_child(&mut grid, __e).unwrap();
        }
        // java: adder.addChild(CycleButton.booleanBuilder(CommonComponents.GUI_YES, CommonComponents.OPTION_OFF, waypointsReadOnly.allowGoingBackwards).withTooltip(..)
        //   .create(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards"), (_, value) -> updateConfig(waypoints -> waypoints.allowGoingBackwards = value)));
        {
            let __builder = cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::option_off().get_string(), waypoints_read_only.allow_going_backwards)
                .with_tooltip(cycle_button::TooltipSupplier::Constant(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards.@Tooltip"))));
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __w = compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards"), __builder, move |__index: usize| {
                let Some(__this_1) = __this_w.get().upgrade() else { return; };
                let value = __index == 0;
                WaypointsOptionScreen::update_config(&__this_1, Arc::new(move |waypoints: &mut Waypoints| { waypoints.allow_going_backwards = value; }));
            });
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            adder.add_child(&mut grid, __e).unwrap();
        }
        // java: adder.addChild(RangedSliderWidget.builder().optionFormatter(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.lineWidth"), Formatters.FLOAT_NUMBERS)
        //   .callback(value -> updateConfig(waypoints -> waypoints.lineWidth = (float) value)).minMax(1, 15).defaultValue(waypointsReadOnly.lineWidth).step(0.5).build());
        {
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __slider: compat::Shared<RangedSliderWidget> = RangedSliderWidget::builder()
                .option_formatter(&component::translatable("skyblocker.config.uiAndVisuals.waypoints.lineWidth"), formatters::FLOAT_NUMBERS.get_mut())
                .callback(Arc::new(move |value: f64| {
                    let Some(__this_1) = __this_w.get().upgrade() else { return; };
                    WaypointsOptionScreen::update_config(&__this_1, Arc::new(move |waypoints: &mut Waypoints| { waypoints.line_width = value as f32; }));
                }))
                .min_max(1.0, 15.0)
                .default_value((waypoints_read_only.line_width) as f64)
                .step(0.5)
                .build();
            let mut __w = RangedSliderWidget::widget(__slider);
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            adder.add_child(&mut grid, __e).unwrap();
        }
        // java: adder.addChild(RangedSliderWidget.builder().optionFormatter(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius"), Formatters.FLOAT_NUMBERS)
        //   .callback(value -> updateConfig(waypoints -> waypoints.waypointActivationRadius = (float) value)).minMax(1, 10).defaultValue(waypointsReadOnly.waypointActivationRadius).step(0.5).build()).setTooltip(..);
        {
            let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __slider: compat::Shared<RangedSliderWidget> = RangedSliderWidget::builder()
                .option_formatter(&component::translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius"), formatters::FLOAT_NUMBERS.get_mut())
                .callback(Arc::new(move |value: f64| {
                    let Some(__this_1) = __this_w.get().upgrade() else { return; };
                    WaypointsOptionScreen::update_config(&__this_1, Arc::new(move |waypoints: &mut Waypoints| { waypoints.waypoint_activation_radius = value as f32; }));
                }))
                .min_max(1.0, 10.0)
                .default_value((waypoints_read_only.waypoint_activation_radius) as f64)
                .step(0.5)
                .build();
            let mut __w = RangedSliderWidget::widget(__slider);
            __w.set_tooltip(Tooltip::create(component::translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius.@Tooltip").get_string()));
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __e = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            adder.add_child(&mut grid, __e).unwrap();
        }
        // java: adder.addChild(SpacerElement.width(0));
        { let __spacer = SpacerElement::width(0); adder.add_child(&mut grid, LayoutElement::Spacer { x: __spacer.get_x(), y: __spacer.get_y(), width: __spacer.get_width(), height: __spacer.get_height() }).unwrap(); }
        {
            // `adder.addChild(new StringWidget(..), 2, gridWidget.newCellSettings().alignHorizontallyCenter().paddingTop(4))`
            let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.uiAndVisuals.waypoints.lineColor"));
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __q0 = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            let __settings = grid.new_cell_settings().align_horizontally_center().padding_top(4);
            adder.add_child_spanning(&mut grid, __q0, 2, __settings).unwrap();
        }
        let mut color_layout: LinearLayout = LinearLayout::horizontal();
        // java: adder.addChild(colorLayout, 2, LayoutSettings.defaults().alignHorizontallyCenter()); - added below, once filled (by value)
        // `colorLayout.addChild(new ColorPickerWidget(..))` answers the widget: the object is kept, its game widget joins the screen.
        let mut color_picker_widget: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new_5(0, 0, 200, 100, true);
        color_layout.add_child({ let __w = compat::WidgetAdapter::widget(color_picker_widget.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let mut argb_text_input: compat::Shared<ARGBTextInput> = ARGBTextInput::new(0, 0, compat::font(), true, true);
        color_layout.add_child({ let __w = compat::WidgetAdapter::widget(argb_text_input.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        // `adder.addChild(colorLayout, 2, gridWidget.newCellSettings().alignHorizontallyCenter())`: added once filled (by value).
        { let __settings = grid.new_cell_settings().align_horizontally_center(); adder.add_child_spanning(&mut grid, LayoutElement::Grid(color_layout.clone().wrapped), 2, __settings).unwrap(); }
        { let (__q0,) = (LayoutElement::Grid(grid.clone()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // The two widgets' callbacks reach each other: each holds the other weakly (Java's lambdas hold references
        // the GC can collect together; strong handles would keep both alive for ever).
        let __argb_text_input_w = std::sync::Arc::downgrade(&argb_text_input);
        ColorPickerWidget::set_on_color_change(&color_picker_widget, Arc::new(move |color, mouse_release| {
            let Some(argb_text_input) = __argb_text_input_w.upgrade() else { return };
            ARGBTextInput::set_argb_color(&argb_text_input, color);
            if mouse_release {
                let Some(__this_6) = __this_w6.get().upgrade() else { return };
                // `new Color(color, true)` (the alpha bits kept)
                WaypointsOptionScreen::update_config(&__this_6, Arc::new(move |waypoints: &mut Waypoints| { waypoints.line_color = Color::from_rgba(color, true); }));
            }
        }));
        let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __color_picker_widget_w = std::sync::Arc::downgrade(&color_picker_widget);
        ARGBTextInput::set_on_change(&argb_text_input, Arc::new(move |color| {
            let Some(color_picker_widget) = __color_picker_widget_w.upgrade() else { return };
            ColorPickerWidget::set_argb_color(&color_picker_widget, color);
            let Some(__this_7) = __this_w7.get().upgrade() else { return };
            // `new Color(color, true)` (the alpha bits kept)
            WaypointsOptionScreen::update_config(&__this_7, Arc::new(move |waypoints: &mut Waypoints| { waypoints.line_color = Color::from_rgba(color, true); }));
        }));
        ColorPickerWidget::set_argb_color(&color_picker_widget, waypoints_read_only.line_color.get_rgb());
        ARGBTextInput::set_argb_color(&argb_text_input, waypoints_read_only.line_color.get_rgb());
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, mcrust::net::minecraft::network::chat::component::literal(__this.lock().borrow().get_title())); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        let __this_w8 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_8) = __this_w8.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_8, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WaypointsOptionScreen::on_close((&__this_8)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        WaypointsOptionScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: private void updateConfig(Consumer<UIAndVisualsConfig.Waypoints> consumer)
    /// (the consumer writes the config's own object, as Java's does - not a copy of it)
    pub fn update_config(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(&mut Waypoints)>) {
        SkyblockerConfigManager::update_only(Arc::new(move |config| { (consumer)(&mut config.ui_and_visuals.waypoints); }));
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

