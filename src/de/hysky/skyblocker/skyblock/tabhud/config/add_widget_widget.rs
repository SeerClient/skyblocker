//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.AddWidgetWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::mcrust::modding::widgets::ListKind;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int MAX_ENTRIES = 10;
pub const MAX_ENTRIES: i32 = 10;

/// The row a click chose (`Entry.mouseClicked`: `widgetConsumer.accept(hudWidget); visible = false;`), handed to the
/// enclosing list after the click - the row runs while the list object is borrowed for the click.
type Chosen = std::rc::Rc<std::cell::RefCell<Option<compat::Shared<dyn HudWidget>>>>;

/// java: public class AddWidgetWidget extends AbstractSelectionList<AddWidgetWidget.Entry>
pub struct AddWidgetWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList (its `visible` / `active` are this
    /// list's own fields, as Java's `AbstractWidget` fields)
    pub base: ModSelectionList,
    /// java: private final Consumer<HudWidget> widgetConsumer;
    pub widget_consumer: Arc<dyn Fn(compat::Shared<dyn HudWidget>)>,
    /// what a row's click asks of `AddWidgetWidget.this`
    pub chosen: Chosen,
}

impl std::ops::Deref for AddWidgetWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for AddWidgetWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl AddWidgetWidget {
    /// java: public AddWidgetWidget(Minecraft client, Consumer<HudWidget> widgetConsumer)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, widget_consumer: Arc<dyn Fn(compat::Shared<dyn HudWidget>)>) -> compat::Shared<Self> {
        // java: super(client, 10, 10, 0, 12); - (width, height, y, itemHeight); the class's overrides of the list's own
        // methods are installed with it
        let _ = client;
        let mut base = ModSelectionList::with_class(ListKind::Object, 10, 10, 0, 12, Arc::new(AddWidgetWidgetClass));
        // java: visible = false;
        base.visible = false;
        compat::shared(Self { base, widget_consumer, chosen: std::rc::Rc::new(std::cell::RefCell::new(None)) })
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// `AbstractWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(x, y)` (`ContainerEventHandler.getChildAt`
    /// asks it of the screen's children)
    pub fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.visible && self.base.active && self.base.list.is_mouse_over(mouse_x, mouse_y)
    }

    /// `AbstractWidget.extractRenderState` - `if (visible) { isHovered = ..; extractWidgetRenderState(..) }`
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, delta_ticks: f32) {
        if !compat::read(__this, |__t| __t.base.visible) {
            return;
        }
        AddWidgetWidget::extract_widget_render_state(__this, context, mouse_x, mouse_y, delta_ticks);
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // java: super.extractWidgetRenderState(context, mouseX, mouseY, deltaTicks); - `AbstractSelectionList`'s, with this
        // class's overrides (`AddWidgetWidgetClass`). The live list is taken out of the object for the draw (its rows'
        // draws do not reach the object) and put back after it.
        let mut __live = compat::write(__this, |__t| std::mem::replace(&mut __t.base, ModSelectionList::new(ListKind::Object, 0, 0, 0, 0)));
        let (x, y, w, h) = (__live.list.x, __live.list.y, __live.list.width, __live.list.height);
        let mut __widget = AbstractWidget::new(x, y, w, h, String::new(), WidgetKind::Button, OnPress::Inert);
        __widget.visible = __live.visible;
        __widget.active = __live.active;
        __live.extract_widget_render_state(&__widget, context, mouse_x, mouse_y, delta_ticks);
        compat::write(__this, |__t| __t.base = __live);
        let (gx, gy, right, bottom) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_right(), __t.get_bottom()));
        if mouse_x < gx.wrapping_sub(20) || mouse_y < gy.wrapping_sub(20) || mouse_x > right.wrapping_add(20) || mouse_y > bottom.wrapping_add(20) {
            compat::write(__this, |__t| __t.base.visible = false);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !compat::read(__this, |__t| __t.base.visible) {
            return false;
        }
        // java: return super.mouseClicked(click, doubled);
        let __answer = compat::write(__this, |__t| __t.base.mouse_clicked_base(click, doubled));
        // the row's `widgetConsumer.accept(hudWidget); visible = false;`, made inside the click above
        let __chosen = compat::read(__this, |__t| __t.chosen.borrow_mut().take());
        if let Some(__widget) = __chosen {
            let __consumer = compat::read(__this, |__t| __t.widget_consumer.clone());
            (__consumer)(__widget);
            compat::write(__this, |__t| __t.base.visible = false);
        }
        __answer
    }

    /// `AbstractContainerWidget.mouseReleased` (the scrollbar drag ends)
    pub fn mouse_released(__this: &compat::Shared<Self>, click: &MouseButtonEvent) -> bool {
        compat::write(__this, |__t| __t.base.mouse_released_base(click))
    }

    /// `AbstractContainerWidget.mouseDragged` (the scrollbar drag)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::write(__this, |__t| __t.base.mouse_dragged_base(click, dx, dy))
    }

    /// `AbstractScrollArea.mouseScrolled`
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, _mouse_x: f64, _mouse_y: f64, _horizontal_amount: f64, vertical_amount: f64) -> bool {
        if !compat::read(__this, |__t| __t.base.visible) {
            return false;
        }
        compat::write(__this, |__t| __t.base.mouse_scrolled_base(vertical_amount))
    }

    /// java: public void openWith(List<HudWidget> widgets)
    pub fn open_with(__this: &compat::Shared<Self>, widgets: &Vec<compat::Shared<dyn HudWidget>>) {
        compat::write(__this, |__t| __t.base.visible = true);
        // java: replaceEntries(widgets.stream().sorted(Comparator.comparing(w -> w.getInformation().displayName().getString())).map(Entry::new).toList());
        let __name = |w: &compat::Shared<dyn HudWidget>| ({ let __f = compat::dispatch(w, |__t| __t.get_information()); __f(w) }).display_name().map(|n| n.get_string()).unwrap_or_default();
        let mut __sorted: Vec<compat::Shared<dyn HudWidget>> = widgets.clone();
        // (a stable sort, as `Stream.sorted` is; `String.compareTo` is UTF-16 order)
        __sorted.sort_by(|a, b| compat::compare_to(&__name(a), &__name(b)).cmp(&0));
        let __chosen = compat::read(__this, |__t| __t.chosen.clone());
        compat::write(__this, |__t| {
            __t.base.clear_entries();
            for __w in __sorted.iter() {
                __t.base.add_entry(Box::new(compat::EntryAdapter::new(Entry::new(__w.clone(), __chosen.clone()))));
            }
        });
        // 2 pixels padding below and above the entries
        let __h = (((widgets.len() as i32).min(MAX_ENTRIES)).wrapping_mul(compat::read(__this, |__t| __t.default_entry_height))).wrapping_add(4);
        compat::write(__this, |__t| __t.set_height(__h));
        // 2 pixels padding on each side + 2 pixels for the scrollbar
        let __max = widgets.iter().map(|entry| compat::font().width_of_cached(&FormattedText::from(({ let __f = compat::dispatch(entry, |__t| __t.get_information()); __f(entry) }).display_name().unwrap()))).max().unwrap_or(100);
        let __scrollable = compat::read(__this, |__t| __t.scrollable());
        compat::write(__this, |__t| __t.set_width(__max.wrapping_add(if __scrollable { 6 } else { 4 })));
    }

}

/// AddWidgetWidget's overrides of the list's own methods, as the port's list bodies ask them (`widgets::ModList`).
/// Every one reads only the list it is handed (`this`'s box, its scroll state).
struct AddWidgetWidgetClass;

impl ModList for AddWidgetWidgetClass {
    /// java: @Override public int getRowLeft()
    fn get_row_left(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        list.get_x()
    }

    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        if list.scrollable() { list.width.wrapping_sub(2) } else { list.width }
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        list.get_right().wrapping_sub(2)
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor context, int mouseX, int mouseY)
    fn extract_scrollbar(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _super_extract_scrollbar: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        if list.scrollable() {
            let mut x: i32 = list.get_right().wrapping_sub(2);
            let mut y: i32 = list.scroll_bar_y();
            let mut h: i32 = list.scroller_height();
            context.graphics.fill(context.gl, x, y, x.wrapping_add(2), y.wrapping_add(h), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor context) - empty
    fn extract_list_separators(&self, _list: &AbstractSelectionList, _context: &mut WidgetRenderContext<'_>, _super_extract_list_separators: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {}

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor context)
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, _super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let (x, y, w, h) = (list.get_x(), list.get_y(), list.get_width(), list.get_height());
        context.graphics.fill(context.gl, x, y, list.get_right(), list.get_bottom(), argb::black_f32(0.6f32));
        if list.scroll_amount() > 0.0 {
            for i in 0..w {
                if i.wrapping_rem(2) == 0 {
                    context.graphics.fill(context.gl, x.wrapping_add(i), y.wrapping_sub(1), (x.wrapping_add(i)).wrapping_add(1), y, -1);
                }
            }
        }
        if list.scroll_amount() < (list.max_scroll_amount() as f64) {
            for i in 0..w {
                if i.wrapping_rem(2) == 0 {
                    context.graphics.fill(context.gl, x.wrapping_add(i), y.wrapping_add(h), (x.wrapping_add(i)).wrapping_add(1), (y.wrapping_add(h)).wrapping_add(1), -1);
                }
            }
        }
    }
}

/// java: protected class Entry extends AbstractSelectionList.Entry<de.hysky.skyblocker.skyblock.tabhud.config.AddWidgetWidget.Entry>
pub struct Entry {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final HudWidget hudWidget;
    pub hud_widget: compat::Shared<dyn HudWidget>,
    /// `AddWidgetWidget.this` - what the row's click hands the enclosing list
    pub outer_chosen: Chosen,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for Entry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Entry {
    /// java: private Entry(HudWidget widget)
    pub fn new(mut widget: compat::Shared<dyn HudWidget>, outer_chosen: Chosen) -> compat::Shared<Self> {
        compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            hud_widget: widget,
            outer_chosen,
        })
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float tickProgress)
    pub fn extract_content(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut tick_progress: f32) {
        let (x, y, w, h) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
        if hovered {
            context.graphics.fill(context.gl, x, y, x.wrapping_add(w), y.wrapping_add(h), argb::white_f32(0.1f32));
        }
        // java: context.textRenderer().accept(getX() + 2, getY() + 2, hudWidget.getInformation().displayName());
        // (`GuiGraphicsExtractor.textRenderer()` - white, with the drop shadow)
        let __name = { let __r = compat::read(__this, |__t| __t.hud_widget.clone()); let __f = compat::dispatch(&__r, |__t| __t.get_information()); __f(&__r) }.display_name().unwrap();
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(context.gl, context.minecraft, &FormattedText::from(__name), x.wrapping_add(2), y.wrapping_add(2), -1, true);
        let _ = (mouse_x, mouse_y, tick_progress);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // java: widgetConsumer.accept(hudWidget); visible = false; - `AddWidgetWidget.this`'s, run by the list once the
        // click returns (see `AddWidgetWidget::mouse_clicked`)
        let (__chosen, __widget) = compat::read(__this, |__t| (__t.outer_chosen.clone(), __t.hud_widget.clone()));
        *__chosen.borrow_mut() = Some(__widget);
        let _ = (click, doubled);
        return true;
    }

}

/// Entry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for Entry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Entry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<Entry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float tickProgress)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| Entry::extract_content(__h, context, mouse_x, mouse_y, hovered, a));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| Entry::mouse_clicked(__h, event, double_click))
    }

}
