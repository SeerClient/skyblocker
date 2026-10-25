//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.entries.slot.DefaultSlotEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntryBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntryBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class DefaultSlotEntry extends WidgetsListSlotEntry
pub struct DefaultSlotEntry {
    /// java: extends WidgetsListSlotEntry
    pub base: WidgetsListSlotEntryBase,
    /// java: private final Button leftClick;
    pub left_click: Option<compat::WidgetRef>,
    /// java: private final Button rightClick;
    pub right_click: Option<compat::WidgetRef>,
}

impl std::ops::Deref for DefaultSlotEntry {
    type Target = WidgetsListSlotEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DefaultSlotEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DefaultSlotEntry {
    /// java: public DefaultSlotEntry(WidgetsListScreen parent, int slotId, ItemStack icon)
    pub fn new(mut parent: compat::Shared<WidgetsListScreen>, mut slot_id: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        let (_, __this) = WidgetsListSlotEntryBase::new_in(parent.clone(), slot_id, icon.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                left_click: None,
                right_click: None,
            });
            ((__h.clone() as compat::Shared<dyn WidgetsListSlotEntry>), __h)
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder(component::literal("LEFT").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server(&compat::read(&__this_1, |__t| __t.parent.clone()), compat::read(&__this_1, |__t| __t.slot_id), 0); } }) })))).size(32, 12).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.left_click = __v; }); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder(component::literal("RIGHT").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server(&compat::read(&__this_2, |__t| __t.parent.clone()), compat::read(&__this_2, |__t| __t.slot_id), 1); } }) })))).size(32, 12).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.right_click = __v; }); }
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    /// (the game asks the row for its widgets, `ModListEntry::children`; this answers them as values)
    pub fn children(__this: &compat::Shared<Self>) -> Vec<mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget> {
        return vec![compat::write(__this, |__t| { let __s = &mut *__t; match __s.left_click.as_mut() { Some(__r) => Some(__s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(__this, |__t| { let __s = &mut *__t; match __s.right_click.as_mut() { Some(__r) => Some(__s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()];
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        ({ let __f = compat::dispatch(__this, |__t| WidgetsListSlotEntry::extract_icon_and_text(__t)); __f(&compat::view(__this, |a| a as compat::Shared<dyn WidgetsListSlotEntry>), graphics, __this.lock().borrow().get_y(), __this.lock().borrow().get_x(), __this.lock().borrow().get_height()) });
        let __wa1 = ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(40);
        let __wa2 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(12)).wrapping_div(2));
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.right_click.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.set_position(__wa1, __wa2)) });
        // java: right_click.extractRenderState(graphics, mouseX, mouseY, deltaTicks); - the row's widgets are drawn by
        // its `compat::EntryAdapter` right after this method returns (`ModListEntry::extract_content`)
        let __wa7 = ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(80);
        let __wa8 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(12)).wrapping_div(2));
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.left_click.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.set_position(__wa7, __wa8)) });
        // java: left_click.extractRenderState(graphics, mouseX, mouseY, deltaTicks); - the row's widgets are drawn by
        // its `compat::EntryAdapter` right after this method returns (`ModListEntry::extract_content`)
    }

}

impl WidgetsListSlotEntry for DefaultSlotEntry {
    fn widgets_list_slot_entry_base(&self) -> &WidgetsListSlotEntryBase {
        &self.base
    }
    fn widgets_list_slot_entry_base_mut(&mut self) -> &mut WidgetsListSlotEntryBase {
        &mut self.base
    }

}

impl WidgetsListEntry for DefaultSlotEntry {
    fn widgets_list_entry_base(&self) -> &WidgetsListEntryBase {
        &self.base.base
    }
    fn widgets_list_entry_base_mut(&mut self) -> &mut WidgetsListEntryBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, int entryWidth, int entryHeight, int mouseX, int mouseY) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn WidgetsListEntry>, &mut GuiGraphicsExtractor, i32, i32, i32, i32, i32, i32) {
        DefaultSlotEntry::extract_tooltip_dyn
    }

}

impl DefaultSlotEntry {
    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, int entryWidth, int entryHeight, int mouseX, int mouseY)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut entry_width: i32, mut entry_height: i32, mut mouse_x: i32, mut mouse_y: i32) {
        if mouse_x >= x && mouse_x <= (x.wrapping_add(entry_width)).wrapping_sub(80) && mouse_y >= y && mouse_y <= y.wrapping_add(entry_height) {
            let mut lore: Vec<MutableComponent> = ItemUtils::get_lore(&compat::read(__this, |__t| __t.icon.clone()));
            // java: graphics.setComponentTooltipForNextFrame(minecraft.font, lines, mouseX, mouseY);
            compat::batch_s_set_tooltip_for_next_frame((lore[(0) as usize..(((lore.len() as i32).wrapping_sub(2)).max(0)) as usize].to_vec()).into_iter().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect());
        }
    }

    /// DefaultSlotEntry.extractTooltip as the dispatch of WidgetsListEntry.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn WidgetsListEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: i32, __p5: i32, __p6: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5, __p6)
    }

}

/// DefaultSlotEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for DefaultSlotEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// DefaultSlotEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<DefaultSlotEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| DefaultSlotEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}
