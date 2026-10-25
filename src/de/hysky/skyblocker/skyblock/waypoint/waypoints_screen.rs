//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen::AbstractWaypointsScreen;
use crate::de::hysky::skyblocker::skyblock::waypoint::abstract_waypoints_screen::AbstractWaypointsScreenBase;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints::Waypoints;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_list_widget::WaypointsListWidget;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_option_screen::WaypointsOptionScreen;
use crate::de::hysky::skyblocker::skyblock::waypoint::waypoints_share_screen::WaypointsShareScreen;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class WaypointsScreen extends AbstractWaypointsScreen<Screen>
pub struct WaypointsScreen {
    /// java: extends AbstractWaypointsScreen
    pub base: AbstractWaypointsScreenBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for WaypointsScreen {
    type Target = AbstractWaypointsScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaypointsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaypointsScreen {
    /// java: public WaypointsScreen(Screen parent)
    pub fn new(parent: Screen) -> compat::Shared<Self> {
        // java: super(Component.translatable("skyblocker.waypoints.config"), parent, Waypoints.waypointsDeepCopy());
        let __this = compat::shared(Self {
            base: AbstractWaypointsScreenBase::new_3(component::translatable("skyblocker.waypoints.config"), Some(std::rc::Rc::new(parent) as std::rc::Rc<dyn std::any::Any>), Waypoints::waypoints_deep_copy()),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// The object as the class it extends (`AbstractWaypointsScreen<?>`), for the bodies the base class declares.
    fn as_abstract(__this: &compat::Shared<Self>) -> compat::Shared<dyn AbstractWaypointsScreen> {
        __this.clone()
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let __abstract = WaypointsScreen::as_abstract(__this);
        // java: super.init();
        <dyn AbstractWaypointsScreen>::init_super(&__abstract);
        // java: GridLayout gridWidget = new GridLayout().columnSpacing(5).rowSpacing(2);
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.column_spacing(5);
        grid_widget.row_spacing(2);
        // java: GridLayout.RowHelper adder = gridWidget.createRowHelper(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // Each button's action reaches this screen through its own handle, the live screen lent to it for the call.
        // java: adder.addChild(Button.builder(Component.translatable("skyblocker.waypoints.share"), _ -> minecraft.gui.setScreen(new WaypointsShareScreen(this, waypoints))).build());
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __b1 = button::builder(component::translatable("skyblocker.waypoints.share").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w1.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                let __waypoints = compat::read(&__this, |__t| __t.waypoints.clone()).unwrap();
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(WaypointsShareScreen::new(__this.clone(), __waypoints))));
            })
        }))).build();
        WaypointsScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __b1);
        // java: adder.addChild(Button.builder(Component.translatable("skyblocker.waypoints.newGroup"), _ -> waypointsListWidget.addWaypointGroupAfterSelected()).build());
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __b2 = button::builder(component::translatable("skyblocker.waypoints.newGroup").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w2.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                <dyn AbstractWaypointsScreen>::with_list(&WaypointsScreen::as_abstract(&__this), |__h| WaypointsListWidget::add_waypoint_group_after_selected(__h));
            })
        }))).build();
        WaypointsScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __b2);
        // java: adder.addChild(Button.builder(CommonComponents.GUI_CANCEL, _ -> onClose()).build());
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __b3 = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w3.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || WaypointsScreen::on_close(&__this))
        }))).build();
        WaypointsScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __b3);
        // java: adder.addChild(Button.builder(CommonComponents.GUI_DONE, _ -> { saveWaypoints(); onClose(); }).build());
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __b4 = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w4.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                WaypointsScreen::save_waypoints(&__this);
                WaypointsScreen::on_close(&__this);
            })
        }))).build();
        WaypointsScreen::add_to_grid(__this, &mut adder, &mut grid_widget, __b4);
        // java: layout.addToFooter(gridWidget); layout.setFooterHeight(64);
        compat::write(__this, |__t| {
            let __layout = __t.base.layout.as_mut().unwrap();
            __layout.add_to_footer(LayoutElement::Grid(grid_widget));
            __layout.set_footer_height(64);
        });
        // java: layout.addToHeader(Button.builder(Component.translatable("skyblocker.waypoints.otherOptions"), _ ->
        //   minecraft.gui.setScreen(new WaypointsOptionScreen(this))).build(), p -> p.alignHorizontallyLeft().paddingLeft(10));
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __b5 = button::builder(component::translatable("skyblocker.waypoints.otherOptions").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut Screen, __index: usize| {
            let Some(__this) = __this_w5.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                // (the option screen's parent is this screen, which the game's stack keeps)
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(WaypointsOptionScreen::new(None))));
            })
        }))).build();
        compat::write(__this, |__t| {
            let (__ww, __wh) = (__b5.get_width(), __b5.get_height());
            let __i = __t.base.base.add_widget(__b5);
            let __layout = __t.base.layout.as_mut().unwrap();
            let __settings = __layout.header_frame.new_child_layout_settings().align_horizontally_left().padding_left(10);
            __layout.add_to_header_with(LayoutElement::widget(__i, __ww, __wh), __settings);
        });
        // java: updateButtons();
        <dyn AbstractWaypointsScreen>::update_buttons(&__abstract);
        // java: super.lateInit();
        <dyn AbstractWaypointsScreen>::late_init(&__abstract);
    }

    /// `adder.addChild(button)`: the button joins the screen's widget list, the grid holds its index.
    fn add_to_grid(__this: &compat::Shared<Self>, adder: &mut RowHelper, grid: &mut GridLayout, button: AbstractWidget) {
        let (__ww, __wh) = (button.get_width(), button.get_height());
        let __i = compat::write(__this, |__t| __t.base.base.add_widget(button));
        adder.add_child(grid, LayoutElement::widget(__i, __ww, __wh)).unwrap();
    }

    /// java: private void saveWaypoints()
    pub fn save_waypoints(__this: &compat::Shared<Self>) {
        // java: Waypoints.clearAndPutAllWaypoints(waypoints);
        Waypoints::clear_and_put_all_waypoints(&compat::read(__this, |__t| __t.waypoints.clone()).unwrap());
        // java: Waypoints.saveWaypoints(minecraft);
        Waypoints::save_waypoints(compat::minecraft());
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if !(Waypoints::are_waypoints_equal(&compat::read(__this, |__t| __t.waypoints.clone()).unwrap())) {
            // java: minecraft.gui.setScreen(new ConfirmScreen(confirmedAction -> minecraft.gui.setScreen(confirmedAction ? parent : this),
            //   Component.translatable("text.skyblocker.quit_config"), Component.translatable("text.skyblocker.quit_config_sure"),
            //   Component.translatable("text.skyblocker.quit_discard"), CommonComponents.GUI_CANCEL));
            // (the prompt is opened over this screen, which the game's stack keeps: `setScreen(this)` is one step back to
            // it, `setScreen(parent)` two)
            let popup = compat::ConfirmScreen::new(
                |confirmed_action: bool| {
                    mcrust::mcrust::modding::game::set_screen_to_parent();
                    if confirmed_action {
                        mcrust::mcrust::modding::game::set_screen_to_parent();
                    }
                },
                component::translatable("text.skyblocker.quit_config"),
                component::translatable("text.skyblocker.quit_config_sure"),
                component::translatable("text.skyblocker.quit_discard"),
                mcrust::net::minecraft::network::chat::common_components::gui_cancel(),
            );
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(popup));
        } else {
            // java: minecraft.gui.setScreen(parent); - the screen is opened keeping its parent, which the game's stack hands back.
            mcrust::mcrust::modding::game::set_screen_to_parent();
        }
    }
}

impl AbstractWaypointsScreen for WaypointsScreen {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase {
        &self.base
    }
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// WaypointsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WaypointsScreen {
    fn screen_half(&mut self) -> &mut Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WaypointsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WaypointsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| WaypointsScreen::init(__h))
    }

    /// java: AbstractWaypointsScreen.repositionElements (it does not rebuild the widgets)
    fn reposition_elements(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::reposition_elements(&WaypointsScreen::as_abstract(__h)))
    }

    /// java: Screen.tick (the base body), and the `updateButtons()` a click asked for
    fn tick(&mut self, screen: &mut Screen) {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::tick(&WaypointsScreen::as_abstract(__h)))
    }

    /// java: Screen.extractRenderState (the base body) - the island dropdown and the popup container last
    fn extract_render_state(&mut self, screen: &Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| <dyn AbstractWaypointsScreen>::extract_render_state(&WaypointsScreen::as_abstract(__h), context, mouse_x, mouse_y, a))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: AbstractWaypointsScreen.mouseClicked
    fn mouse_clicked(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_clicked(&WaypointsScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button), false))
    }

    /// java: Screen.mouseReleased - the popup container while it is dragged
    fn mouse_released(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_released(&WaypointsScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button)))
    }

    /// java: Screen.mouseDragged - the popup container while it is dragged
    fn mouse_dragged(&mut self, screen: &mut Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_dragged(&WaypointsScreen::as_abstract(__h), &abstract_waypoints_screen::click_event(x, y, button), dx, dy))
    }

    /// java: AbstractWaypointsScreen.mouseScrolled
    fn mouse_scrolled(&mut self, screen: &mut Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| <dyn AbstractWaypointsScreen>::mouse_scrolled(&WaypointsScreen::as_abstract(__h), x, y, scroll_x, scroll_y))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WaypointsScreen::on_close(__h))
    }
}
