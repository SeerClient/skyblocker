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
    /// java: private final List<String> filters; - one list, shared with the `ItemTickList` (its checkboxes write it)
    pub filters: compat::Shared<Vec<String>>,
    /// java: private final List<String> allItems;
    pub all_items: Vec<String>,
    /// `init`'s `itemTickList` (the buttons' lambdas capture it): the list's class object and the game's list widget
    pub item_tick_list: Option<(Arc<ItemTickList<String>>, mcrust::mcrust::modding::widgets::ModWidgetSlot)>,
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
        let __init_parent = parent;
        // java: new ArrayList<>(SkyblockerConfigManager.get().mining.crystalHollows.powderTrackerFilter); // Copy the list so we can undo changes when necessary
        let __init_filters = compat::shared(SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().cloned().collect::<Vec<_>>());
        let __init_all_items = all_items.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::null_to_empty(Some("Powder Mining Tracker Filter Config")).get_string()),
            __widgets: Default::default(),
            parent: __init_parent,
            filters: __init_filters,
            all_items: __init_all_items,
            item_tick_list: None,
        });
        __this
    }

    /// `itemTickList.x(..)`: the list object and its widget's list, lent for `f` (no borrow of this screen held).
    fn with_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&ItemTickList<String>, &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList) -> R) -> R {
        let (class, slot) = compat::read(__this, |__t| __t.item_tick_list.clone()).expect("PowderFilterConfigScreen.itemTickList before init()");
        slot.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| f(&class, &mut l.list)).expect("the item tick list reached from inside its own method")
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: addRenderableOnly((context, _, _, _) -> context.centeredText(..)); - see `extract_render_state`
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        // java: ItemTickList<String> itemTickList = addRenderableWidget(new ItemTickList<>(Minecraft.getInstance(), width, height - 96, 32, 24, filters, allItems).init());
        let (filters, all_items) = compat::read(__this, |__t| (__t.filters.clone(), __t.all_items.clone()));
        let (class, widget) = ItemTickList::new(width, height - 96, 32, 24, filters, all_items);
        let slot = widget.mod_widget().expect("ItemTickList's widget");
        slot.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| class.init(&mut l.list)).expect("ItemTickList.init");
        compat::write(__this, |__t| {
            __t.item_tick_list = Some((class, slot));
            __t.add_renderable_widget(widget);
        });
        //Grid code gratuitously stolen from WaypointsScreen. Same goes for the y and heights above.
        let mut grid_widget: GridLayout = GridLayout::new_default();
        // java: gridWidget.defaultCellSetting().paddingHorizontal(5).paddingVertical(2); - on the layout's own settings
        let __settings = grid_widget.default_cell_setting();
        *__settings = __settings.clone().padding_horizontal(5).padding_vertical(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let reset = button::builder(component::translatable("text.skyblocker.reset").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            // java: filters.clear(); itemTickList.clearAndInit();
            let filters = compat::read(&__this, |__t| __t.filters.clone());
            compat::write(&filters, |f| f.clear());
            PowderFilterConfigScreen::with_list(&__this, |class, list| class.clear_and_init(list));
        })))
        .build();
        PowderFilterConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, reset, 1);

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let undo = button::builder(component::translatable("text.skyblocker.undo").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            // java: filters.clear(); filters.addAll(SkyblockerConfigManager.get().mining.crystalHollows.powderTrackerFilter); itemTickList.clearAndInit();
            let filters = compat::read(&__this, |__t| __t.filters.clone());
            compat::write(&filters, |f| {
                f.clear();
                f.extend(SkyblockerConfigManager::get().mining.crystal_hollows.powder_tracker_filter.iter().cloned());
            });
            PowderFilterConfigScreen::with_list(&__this, |class, list| class.clear_and_init(list));
        })))
        .build();
        PowderFilterConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, undo, 1);

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            PowderFilterConfigScreen::save_filters(&__this);
            PowderFilterConfigScreen::on_close(&__this);
        })))
        .width((mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH * 2) + 10)
        .build();
        PowderFilterConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, done, 2);

        // java: gridWidget.arrangeElements(); FrameLayout.centerInRectangle(gridWidget, 0, this.height - 64, this.width, 64);
        grid_widget.arrange_elements();
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, height - 64, width, 64);
        // java: gridWidget.visitWidgets(this::addRenderableWidget);
        let mut __vw = Vec::new();
        LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw);
        compat::write(__this, |__t| {
            for (__i, __x, __y) in __vw {
                __t.widget_mut(__i).set_position(__x, __y);
                __t.mark_renderable(__i);
            }
        });
    }

    /// `adder.addChild(button[, occupiedColumns])`: the button joins the screen's widgets (by index) and the grid.
    fn add_to_grid(__this: &compat::Shared<Self>, grid: &mut GridLayout, adder: &mut RowHelper, button: mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, columns: i32) {
        let (w, h) = (button.get_width(), button.get_height());
        let index = compat::write(__this, |__t| __t.add_widget(button));
        if columns == 1 {
            adder.add_child(grid, LayoutElement::widget(index, w, h)).expect("GridLayout.addChild");
        } else {
            adder.add_child_width(grid, LayoutElement::widget(index, w, h), columns).expect("GridLayout.addChild");
        }
    }

    /// `addRenderableOnly((context, _, _, _) -> ..)`, the title. (The port's screen keeps no mod renderables; this
    /// draws after the widgets, and the title band, `y < 32`, is outside the list.)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut ScreenRenderContext<'_>) {
        let width = compat::read(__this, |__t| __t.width);
        // java: context.centeredText(minecraft.font, Component.translatable("..screenTitle").withStyle(ChatFormatting.BOLD), width / 2, (32 - minecraft.font.lineHeight) / 2, CommonColors.WHITE);
        let title = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter.screenTitle").with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Bold));
        let line_height = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text(context.gl, context.minecraft, &title, width / 2, (32 - line_height) / 2, mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: public void saveFilters()
    pub fn save_filters(__this: &compat::Shared<Self>) {
        // java: SkyblockerConfigManager.update(config -> config.mining.crystalHollows.powderTrackerFilter = filters);
        let filters = compat::read(__this, |__t| __t.filters.clone());
        SkyblockerConfigManager::update(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.mining.crystal_hollows.powder_tracker_filter = compat::read(&filters, |f| f.clone());
        }));
        // java: PowderMiningTracker.INSTANCE.recalculateAll();
        PowderMiningTracker::recalculate_all(powder_mining_tracker::INSTANCE.get());
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

    /// `addRenderableOnly`'s title (see `PowderFilterConfigScreen::extract_render_state`)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        self.lend_ref(screen, |__h| PowderFilterConfigScreen::extract_render_state(__h, context))
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
