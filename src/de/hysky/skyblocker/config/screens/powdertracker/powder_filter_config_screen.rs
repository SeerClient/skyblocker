//! Ported from de.hysky.skyblocker.config.screens.powdertracker.PowderFilterConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::screens::powdertracker::item_tick_list::ItemTickList;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::powder_mining_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::powder_mining_tracker::PowderMiningTracker;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class PowderFilterConfigScreen extends Screen
pub struct PowderFilterConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private final List<String> filters;
    pub filters: Vec<String>,
    /// java: private final List<String> allItems;
    pub all_items: Vec<String>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for PowderFilterConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PowderFilterConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PowderFilterConfigScreen {
    /// java: public PowderFilterConfigScreen(@Nullable Screen parent, List<String> allItems)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut all_items: Vec<String>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __init_filters = SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().cloned().collect::<Vec<_>>();
        let __init_all_items = all_items.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::null_to_empty(Some("Powder Mining Tracker Filter Config")).get_string()),
            __widgets: Default::default(),
            parent: __init_parent,
            filters: __init_filters,
            all_items: __init_all_items,
        });
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `Screen.addRenderableOnly` has no Rust method in the port (looked for `add_renderable_only` in Screen and its supertypes)
        // rust: __this.lock().borrow().add_renderable_only(&Arc::new(move |context, _, _, _| {
        // rust: // TODO(fabric2seer): `.centeredText(..)` on a value of unknown type
        // rust: //   java: context.centeredText(minecraft.font, Component.translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter.screenTitle").withStyle(ChatFormatting.BOLD), width / 2, (32 - minecraft.font.lineHeight) / 2, CommonColors.WHITE);
        // rust: }));
        let __r0 = compat::read(__this, |__t| __t.width);
        let __r1 = compat::read(__this, |__t| __t.height);
        let __r2 = compat::read(__this, |__t| __t.filters.clone());
        let __r3 = compat::read(__this, |__t| __t.all_items.clone());
        let mut item_tick_list: compat::Shared<ItemTickList<String>> = { let (__q0,) = (compat::attach_list(ItemTickList::init(&ItemTickList::new(compat::minecraft(), __r0, (__r1).wrapping_sub(96), 32, 24, __r2, __r3))),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.default_cell_setting().padding_horizontal(5).padding_vertical(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(component::translatable("text.skyblocker.reset").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: __this_1.get().lock().borrow_mut().filters.clear();
        // rust: ItemTickList::clear_and_init(&item_tick_list);
        // rust: } }) })))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(component::translatable("text.skyblocker.undo").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: __this_2.get().lock().borrow_mut().filters.clear();
        // rust: let __h0 = SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().cloned();
        // rust: __this_2.get().lock().borrow_mut().filters.extend(__h0);
        // rust: ItemTickList::clear_and_init(&item_tick_list);
        // rust: } }) })))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_3 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child_width(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_3.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: PowderFilterConfigScreen::save_filters(__this_3.get());
        // rust: PowderFilterConfigScreen::on_close(__this_3.get());
        // rust: } }) })))).width(((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH).wrapping_mul(2)).wrapping_add(10)).build(), 2).unwrap();
        { grid_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, (compat::read(__this, |__t| __t.height)).wrapping_sub(64), compat::read(__this, |__t| __t.width), 64);
        { let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: public void saveFilters()
    pub fn save_filters(__this: &compat::Shared<Self>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update(Arc::new(move |config| { config.mining.crystal_hollows.powder_tracker_filter = compat::read(__this_1.get(), |__t| __t.filters.clone()); }));
        PowderMiningTracker::recalculate_all(powder_mining_tracker::INSTANCE.get_mut());
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// PowderFilterConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for PowderFilterConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// PowderFilterConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<PowderFilterConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PowderFilterConfigScreen::init(__h))
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
        self.call(|__h| PowderFilterConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
