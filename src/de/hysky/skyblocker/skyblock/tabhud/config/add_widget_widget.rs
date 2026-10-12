//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.AddWidgetWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int MAX_ENTRIES = 10;
pub const MAX_ENTRIES: i32 = 10;

/// java: public class AddWidgetWidget extends AbstractSelectionList<AddWidgetWidget.Entry>
#[derive(Clone)]
pub struct AddWidgetWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList
    pub base: AbstractSelectionList,
    /// java: private final Consumer<HudWidget> widgetConsumer;
    pub widget_consumer: Arc<dyn Fn(compat::Shared<dyn HudWidget>)>,
}

impl std::ops::Deref for AddWidgetWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AddWidgetWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AddWidgetWidget {
    /// java: public AddWidgetWidget(Minecraft client, Consumer<HudWidget> widgetConsumer)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, widget_consumer: Arc<dyn Fn(compat::Shared<dyn HudWidget>)>) -> Self {
        let __init_widget_consumer = widget_consumer.clone();
        // TODO(fabric2seer): `new` takes 4 argument(s) in the port (`width: i32, height: i32, y: i32, default_entry_height: i32`), Java passes 5
        //   java: super(client, 10, 10, 0, 12);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        // rust: this.visible = false;
        this
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor context)
    pub fn extract_list_background(&self, context: &mut GuiGraphicsExtractor) {
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), argb::black_f32(0.6f32));
        if self.scroll_amount() > (0 as f64) {
            for x in 0..self.get_width() {
                if x.wrapping_rem(2) == 0 {
                    context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x().wrapping_add(x), self.get_y().wrapping_sub(1), (self.get_x().wrapping_add(x)).wrapping_add(1), self.get_y(), -1);
                }
            }
        }
        if self.scroll_amount() < (self.max_scroll_amount() as f64) {
            for x in 0..self.get_width() {
                if x.wrapping_rem(2) == 0 {
                    context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x().wrapping_add(x), self.get_y().wrapping_add(self.get_height()), (self.get_x().wrapping_add(x)).wrapping_add(1), (self.get_y().wrapping_add(self.get_height())).wrapping_add(1), -1);
                }
            }
        }
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&self, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // TODO(fabric2seer): `AbstractSelectionList.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in AbstractSelectionList and its supertypes)
        // rust: self.base.extract_widget_render_state(context, mouse_x, mouse_y, delta_ticks);
        if mouse_x < self.get_x().wrapping_sub(20) || mouse_y < self.get_y().wrapping_sub(20) || mouse_x > self.get_right().wrapping_add(20) || mouse_y > self.get_bottom().wrapping_add(20) {
            // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
            // rust: self.visible = false;
        }
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor context)
    pub fn extract_list_separators(&self, context: &GuiGraphicsExtractor) {
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // rust: port gap (GAPS_batchA #8) - `AbstractWidget.visible` on a selection list: the port's
        // `AbstractSelectionList` holds no `visible`
        // rust: if !self.visible {
        // rust:     return false;
        // rust: }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return self.base.mouse_clicked(click.clone(), doubled);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public void openWith(List<HudWidget> widgets)
    pub fn open_with(&mut self, widgets: &Vec<compat::Shared<dyn HudWidget>>) {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        // rust: self.visible = true;
        // rust: port gap (GAPS_batchC #C5) - this list subclass sits on the port's plain `AbstractSelectionList`, whose
        // rows are the game's own `Entry` values: the mod's row objects (`Entry::new`) join a list only through the
        // `ModSelectionList` / `compat::EntryAdapter` path, which this class is not built on
        // rust: self.replace_entries(compat::sorted_by(widgets.iter().cloned(), |a, b| { let key = |w| ({ let __f = compat::dispatch(w, |__t| __t.get_information()); __f(w) }).display_name().unwrap().get_string(); key(a).cmp(&key(b)) }).map(|__a0| Entry::new(__a0.clone())).collect::<Vec<_>>());
        { let __s0 = (((widgets.len() as i32).min(MAX_ENTRIES)).wrapping_mul(self.default_entry_height)).wrapping_add(4); self.set_height(__s0) };
        { let __s0 = (widgets.iter().cloned().map(|entry| compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(({ let __f = compat::dispatch(&entry, |__t| __t.get_information()); __f(&entry) }).display_name().unwrap()))).max().unwrap_or(100)).wrapping_add(if self.scrollable() { 6 } else { 4 }); self.set_width(__s0) };
    }

    /// java: @Override public int getRowLeft()
    pub fn get_row_left(&self) -> i32 {
        return self.get_x();
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(&self) -> i32 {
        return (if self.scrollable() { self.width.wrapping_sub(2) } else { self.width });
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor context, int mouseX, int mouseY)
    pub fn extract_scrollbar(&self, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        if self.scrollable() {
            let mut x: i32 = self.scroll_bar_x();
            let mut y: i32 = self.scroll_bar_y();
            let mut h: i32 = self.scroller_height();
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, y, x.wrapping_add(2), y.wrapping_add(h), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(&self) -> i32 {
        return self.get_right().wrapping_sub(2);
    }

}

// NOTE(fabric2seer): extractListBackground, extractListSeparators, extractScrollbar, extractWidgetRenderState, getRowLeft, getRowWidth, mouseClicked, scrollBarX, updateWidgetNarration override(s) of AbstractSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: protected class Entry extends AbstractSelectionList.Entry<de.hysky.skyblocker.skyblock.tabhud.config.AddWidgetWidget.Entry>
pub struct Entry {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final HudWidget hudWidget;
    pub hud_widget: compat::Shared<dyn HudWidget>,
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
    pub fn new(mut widget: compat::Shared<dyn HudWidget>) -> compat::Shared<Self> {
        let __init_hud_widget = widget.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            hud_widget: __init_hud_widget,
        });
        __this
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float tickProgress)
    pub fn extract_content(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut tick_progress: f32) {
        if hovered {
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), argb::white_f32(0.1f32));
        }
        // TODO(fabric2seer): the port's `text_renderer` ports this Java method with other parameters (mouse_x, mouse_y for Java's )
        // TODO(fabric2seer): `text_renderer` takes 2 argument(s) in the port (`mouse_x: f64, mouse_y: f64`), Java passes 0
        // TODO(fabric2seer): `ActiveTextCollector.accept` has no Rust method in the port (looked for `accept` in ActiveTextCollector and its supertypes)
        // rust: gui_graphics_extractor::text_renderer().accept((__this.lock().borrow().get_x()).wrapping_add(2), (__this.lock().borrow().get_y()).wrapping_add(2), &({ let __r = &compat::read(__this, |__t| __t.hud_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_information()); __f(__r) }).display_name().unwrap());
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `widgetConsumer` is a field of the enclosing instance (AddWidgetWidget.this) - an inner class has no outer reference in Rust
        //   java: widgetConsumer.accept(hudWidget);
        // TODO(fabric2seer): assignment to unknown `visible`
        //   java: visible = false;
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
        self.lend(Some(list.list.entry(index)), |__h| Entry::extract_content(__h, &mut context.graphics, mouse_x, mouse_y, hovered, a));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| Entry::mouse_clicked(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractContent, mouseClicked override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
