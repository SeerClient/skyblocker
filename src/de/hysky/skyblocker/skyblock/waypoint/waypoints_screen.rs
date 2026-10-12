//! Ported from de.hysky.skyblocker.skyblock.waypoint.WaypointsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::confirm_screen::ConfirmScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::utils::location::Location;

/// java: public class WaypointsScreen extends AbstractWaypointsScreen<Screen>
pub struct WaypointsScreen {
    /// java: extends AbstractWaypointsScreen
    pub base: AbstractWaypointsScreenBase<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for WaypointsScreen {
    type Target = AbstractWaypointsScreenBase<mcrust::net::minecraft::client::gui::screens::screen::Screen>;
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
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(Component.translatable("skyblocker.waypoints.config"), parent, Waypoints.waypointsDeepCopy());
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: private void saveWaypoints()
    pub fn save_waypoints(__this: &compat::Shared<Self>) {
        Waypoints::clear_and_put_all_waypoints(&compat::read(__this, |__t| __t.waypoints.clone()).unwrap());
        Waypoints::save_waypoints(compat::minecraft());
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if !(Waypoints::are_waypoints_equal(&compat::read(__this, |__t| __t.waypoints.clone()).unwrap())) {
            // rust: port gap - the port lends a mod no `ConfirmScreen` (no constructor, no `game::` request for one), so the
            // "discard changes?" prompt cannot be shown (GAPS_batchD.md); the screen stays open, as a cancelled prompt leaves it.
            // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
            // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
            // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            // rust: gui::set_screen(&mut ConfirmScreen::new(Arc::new(move |confirmed_action| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; gui::set_screen((if confirmed_action { compat::read((&__this_1), |__t| __t.parent.clone()) } else { Some((&__this_1).clone()) }).as_mut().unwrap()) }), component::translatable("text.skyblocker.quit_config"), component::translatable("text.skyblocker.quit_config_sure"), component::translatable("text.skyblocker.quit_discard"), mcrust::net::minecraft::network::chat::common_components::gui_cancel()));
        } else {
            // `minecraft.gui.setScreen(parent)`: the screen is opened keeping its parent, which the game's stack hands back.
            mcrust::mcrust::modding::game::set_screen_to_parent();
        }
    }

}

impl AbstractWaypointsScreen<mcrust::net::minecraft::client::gui::screens::screen::Screen> for WaypointsScreen {
    fn abstract_waypoints_screen_base(&self) -> &AbstractWaypointsScreenBase<mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        &self.base
    }
    fn abstract_waypoints_screen_base_mut(&mut self) -> &mut AbstractWaypointsScreenBase<mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init()
    fn init(&mut self) {
        self.init_super();
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.column_spacing(5);
        grid_widget.row_spacing(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // Each button's action reaches this screen through its own handle, the live screen lent to it for the call.
        let __this_w1 = compat::RenderThread::new(self.self_ref.clone());
        let __b1 = button::builder(component::translatable("skyblocker.waypoints.share").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this) = __this_w1.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                // `minecraft.gui.setScreen(new WaypointsShareScreen(this, waypoints))`
                let __waypoints = compat::read(&__this, |__t| __t.waypoints.clone()).unwrap();
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(WaypointsShareScreen::new(__this.clone(), __waypoints))));
            })
        }))).build();
        WaypointsScreen::add_to_grid(self, &mut adder, &mut grid_widget, __b1);
        let __this_w2 = compat::RenderThread::new(self.self_ref.clone());
        let __b2 = button::builder(component::translatable("skyblocker.waypoints.newGroup").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this) = __this_w2.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                compat::write(&__this, |__t| __t.base.with_list(|__h| WaypointsListWidget::add_waypoint_group_after_selected(__h)));
            })
        }))).build();
        WaypointsScreen::add_to_grid(self, &mut adder, &mut grid_widget, __b2);
        let __this_w3 = compat::RenderThread::new(self.self_ref.clone());
        let __b3 = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this) = __this_w3.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || WaypointsScreen::on_close(&__this))
        }))).build();
        WaypointsScreen::add_to_grid(self, &mut adder, &mut grid_widget, __b3);
        let __this_w4 = compat::RenderThread::new(self.self_ref.clone());
        let __b4 = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this) = __this_w4.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                WaypointsScreen::save_waypoints(&__this);
                WaypointsScreen::on_close(&__this);
            })
        }))).build();
        WaypointsScreen::add_to_grid(self, &mut adder, &mut grid_widget, __b4);
        self.layout.as_mut().unwrap().add_to_footer(LayoutElement::Grid(grid_widget.clone()));
        self.layout.as_mut().unwrap().set_footer_height(64);
        // `layout.addToHeader(Button.builder(.."otherOptions"..).build(), p -> p.alignHorizontallyLeft().paddingLeft(10))`
        let __this_w5 = compat::RenderThread::new(self.self_ref.clone());
        let __b5 = button::builder(component::translatable("skyblocker.waypoints.otherOptions").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let Some(__this) = __this_w5.get().upgrade() else { return };
            compat::lend_screen(&__this, __screen, || {
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(WaypointsOptionScreen::new(None))));
            })
        }))).build();
        {
            let (__ww, __wh) = (__b5.get_width(), __b5.get_height());
            let __i = self.base.base.add_widget(__b5);
            let __layout = self.layout.as_mut().unwrap();
            let __settings = __layout.header_frame.new_child_layout_settings().align_horizontally_left().padding_left(10);
            __layout.add_to_header_with(LayoutElement::widget(__i, __ww, __wh), __settings);
        }
        self.update_buttons();
        self.late_init();
    }

    /// `this::islandChanged` for the island dropdown, through this screen's own handle.
    fn island_changed_callback(&self) -> Arc<dyn Fn(Location)> {
        let __this_w = compat::RenderThread::new(self.self_ref.clone());
        Arc::new(move |location| {
            if let Some(__this) = __this_w.get().upgrade() {
                compat::write(&__this, |__t| __t.island_changed(location));
            }
        })
    }

}

impl WaypointsScreen {
    /// `adder.addChild(button)`: the button joins the screen's widget list, the grid holds its index.
    fn add_to_grid(this: &mut WaypointsScreen, adder: &mut RowHelper, grid: &mut GridLayout, button: AbstractWidget) {
        let (__ww, __wh) = (button.get_width(), button.get_height());
        let __i = this.base.base.add_widget(button);
        adder.add_child(grid, LayoutElement::widget(__i, __ww, __wh)).unwrap();
    }
}

/// WaypointsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WaypointsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WaypointsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WaypointsScreen> {
    fn get_title(&self) -> String {
        self.title()
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

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WaypointsScreen::on_close(__h))
    }

}
