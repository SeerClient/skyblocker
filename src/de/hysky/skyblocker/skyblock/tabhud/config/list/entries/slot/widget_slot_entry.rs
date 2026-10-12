//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.entries.slot.WidgetSlotEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntryBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntryBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_element_list;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToast;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class WidgetSlotEntry extends WidgetsListSlotEntry
pub struct WidgetSlotEntry {
    /// java: extends WidgetsListSlotEntry
    pub base: WidgetsListSlotEntryBase,
    /// java: private final Button editButton;
    pub edit_button: Option<compat::WidgetRef>,
    /// java: private final State state;
    pub state: Option<State>,
    /// java: private final Button enableButton;
    pub enable_button: Option<compat::WidgetRef>,
    /// java: private final boolean alwaysEnabled;
    pub always_enabled: bool,
}

impl std::ops::Deref for WidgetSlotEntry {
    type Target = WidgetsListSlotEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetSlotEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetSlotEntry {
    /// java: public WidgetSlotEntry(WidgetsListScreen parent, int slotId, ItemStack icon)
    pub fn new(mut parent: compat::Shared<WidgetsListScreen>, mut slot_id: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        let (_, __this) = WidgetsListSlotEntryBase::new_in(parent.clone(), slot_id, icon.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                edit_button: None,
                state: None,
                enable_button: None,
                always_enabled: false,
            });
            ((__h.clone() as compat::Shared<dyn WidgetsListSlotEntry>), __h)
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder(component::literal("EDIT").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            WidgetsListScreen::click_and_wait_for_server(&compat::read(&__this_1, |__t| __t.parent.clone()), compat::read(&__this_1, |__t| __t.slot_id), 1);
            WidgetsListScreen::reset_scroll_on_load(&compat::read(&__this_1, |__t| __t.parent.clone()));
        } }) })))).size(32, 12).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.edit_button = __v; }); }
        let mut string: String = compat::trim(&compat::component_of(icon.get_hover_name()).get_string());
        if string.starts_with("✔") {
            { let __v = Some(State::Enabled); compat::write(&__this, |__t| __t.state = __v); }
        } else if string.starts_with("✖") {
            { let __v = Some(State::Disabled); compat::write(&__this, |__t| __t.state = __v); }
        } else {
            { let __v = Some(State::Locked); compat::write(&__this, |__t| __t.state = __v); }
        }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = (Some((button::builder((if compat::read(&__this, |__t| __t.state.clone()).unwrap() == State::Enabled { widgets_list_entry::ENABLED_TEXT.get_mut().clone() } else { widgets_list_entry::DISABLED_TEXT.get_mut().clone() }).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_row(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server(&compat::read(&__this_2, |__t| __t.parent.clone()), compat::read(&__this_2, |__t| __t.slot_id), 0); } }) })))).size(64, 12).build())).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.enable_button = __v; }); }
        { let __v = ItemUtils::get_lore_line_if(&icon, Arc::new(move |s| s.to_lowercase().contains("always enable"))).is_some(); compat::write(&__this, |__t| __t.always_enabled = __v); }
        __this
    }

    /// java: private static void addToast(Component message)
    pub fn add_toast(message: &MutableComponent) {
        // TODO(fabric2seer): `new` takes 4 argument(s) in the port (`font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 3
        // rust: compat::minecraft().gui.toast_manager().borrow_mut().add_toast(SystemToast::new(widgets_list_screen::SYSTEM_TOAST_ID.get_mut(), message.clone(), None /* java: null */));
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    /// (the game asks the row for its widgets, `ModListEntry::children`; this answers them as values)
    pub fn children(__this: &compat::Shared<Self>) -> Vec<mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget> {
        return (if compat::read(__this, |__t| __t.always_enabled) { vec![compat::write(__this, |__t| { let __s = &mut *__t; match __s.edit_button.as_mut() { Some(__r) => Some(__s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()] } else { vec![compat::write(__this, |__t| { let __s = &mut *__t; match __s.edit_button.as_mut() { Some(__r) => Some(__s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(__this, |__t| { let __s = &mut *__t; match __s.enable_button.as_mut() { Some(__r) => Some(__s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()] });
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        let mut text_y: i32 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(9)).wrapping_div(2));
        let mut text_renderer: Font = compat::font();
        ({ let __f = compat::dispatch(__this, |__t| WidgetsListSlotEntry::extract_icon_and_text(__t)); __f(&compat::view(__this, |a| a as compat::Shared<dyn WidgetsListSlotEntry>), graphics, __this.lock().borrow().get_y(), __this.lock().borrow().get_x(), __this.lock().borrow().get_height()) });
        if compat::read(__this, |__t| __t.state.clone()).unwrap() != State::Locked {
            let __wa1 = ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(40);
            let __wa2 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(12)).wrapping_div(2));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.edit_button.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.set_position(__wa1, __wa2)) });
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa3 = graphics.clone();
            // rust: let __wa4 = mouse_x;
            // rust: let __wa5 = mouse_y;
            // rust: let __wa6 = delta_ticks;
            // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.edit_button.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6)) });
            if !(compat::read(__this, |__t| __t.always_enabled)) {
                let __wa7 = ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(110);
                let __wa8 = (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(12)).wrapping_div(2));
                compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.enable_button.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.set_position(__wa7, __wa8)) });
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: let __wa9 = graphics.clone();
                // rust: let __wa10 = mouse_x;
                // rust: let __wa11 = mouse_y;
                // rust: let __wa12 = delta_ticks;
                // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.enable_button.as_mut().unwrap(); __s.base.base.__widgets.with(&mut __s.base.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa10, __wa11, __wa12)) });
            }
        } else {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("LOCKED"), ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(50), text_y, mcrust::net::minecraft::util::common_colors::RED, true);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut doubled: bool) -> bool {
        // java: super.mouseClicked(event, doubled) - `ContainerObjectSelectionList.Entry.mouseClicked`: `ContainerEventHandler
        // .mouseClicked` over the children, the row's widgets (lent to this object for the call, `__row`); true when a
        // child is under the cursor (the body `ModListEntry::mouse_clicked` runs for a row with children)
        let __super_clicked: bool = compat::write(__this, |__t| {
            let __row = &mut __t.base.base.__row;
            let __hit = __row.get_child_at(event.x(), event.y()).is_some();
            __row.mouse_clicked(event.button(), event.x(), event.y());
            __hit
        });
        if __super_clicked || WidgetsListScreen::is_waiting_for_server(&compat::read(__this, |__t| __t.parent.clone())) {
            return true;
        }
        let mut is_select: bool = (event.button() == 0);
        if is_select && compat::read(__this, |__t| __t.state.clone()).unwrap() != State::Enabled {
            WidgetSlotEntry::add_toast(&component::translatable("skyblocker.uiAndVisuals.tabHud.widgetsScreen.toast.mustEnableWidget"));
            return false;
        }
        let mut relative_position: i32 = ((compat::read(__this, |__t| __t.slot_id)).wrapping_sub(18)).wrapping_sub(1);
        relative_position = relative_position.wrapping_sub(2i32.wrapping_mul(relative_position.wrapping_div(9)));
        if compat::get_state(&widgets_element_list::EDITING_POSITION) == relative_position {
            return false;
        }
        if is_select && relative_position == 0 && !(compat::get_state(&widgets_element_list::IS_ON_SECOND_PAGE)) {
            WidgetSlotEntry::add_toast(&component::translatable("skyblocker.uiAndVisuals.tabHud.widgetsScreen.toast.unselectableWidget"));
            return false;
        } else if !is_select && relative_position > compat::get_state(&widgets_element_list::MAX_POSITION) {
            WidgetSlotEntry::add_toast(&component::translatable("skyblocker.uiAndVisuals.tabHud.widgetsScreen.toast.cannotMoveHere"));
            return true;
        }
        let mut is_greater: bool = (compat::get_state(&widgets_element_list::EDITING_POSITION) > relative_position);
        if is_select {
            WidgetsListScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, (if is_greater { 1 } else { 0 }));
        } else {
            WidgetsListScreen::shift_click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, (if is_greater { 1 } else { 0 }));
        }
        let mut remaining_clicks: i32 = (((compat::get_state(&widgets_element_list::EDITING_POSITION)).wrapping_sub(relative_position)).wrapping_abs()).wrapping_sub(1);
        if remaining_clicks == 0 {
            return true;
        }
        if is_select {
            WidgetSlotEntry::add_toast(&component::translatable_args("skyblocker.uiAndVisuals.tabHud.widgetsScreen.toast.remainingClicksToSelect", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(remaining_clicks.to_string())]));
        } else {
            WidgetSlotEntry::add_toast(&component::translatable_args("skyblocker.uiAndVisuals.tabHud.widgetsScreen.toast.remainingClicksToMove", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(remaining_clicks.to_string())]));
        }
        return true;
    }

    /// java: public State getState()
    pub fn get_state(__this: &compat::Shared<Self>) -> State {
        return compat::read(__this, |__t| __t.state.clone()).unwrap();
    }

}

impl WidgetsListSlotEntry for WidgetSlotEntry {
    fn widgets_list_slot_entry_base(&self) -> &WidgetsListSlotEntryBase {
        &self.base
    }
    fn widgets_list_slot_entry_base_mut(&mut self) -> &mut WidgetsListSlotEntryBase {
        &mut self.base
    }

}

impl WidgetsListEntry for WidgetSlotEntry {
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
        WidgetSlotEntry::extract_tooltip_dyn
    }

}

impl WidgetSlotEntry {
    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, int entryWidth, int entryHeight, int mouseX, int mouseY)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut entry_width: i32, mut entry_height: i32, mut mouse_x: i32, mut mouse_y: i32) {
        if mouse_x >= x && mouse_x <= (x.wrapping_add(entry_width)).wrapping_sub(110) && mouse_y >= y && mouse_y <= y.wrapping_add(entry_height) {
            let mut lore: Vec<MutableComponent> = ItemUtils::get_lore(&compat::read(__this, |__t| __t.icon.clone()));
            if compat::read(__this, |__t| __t.always_enabled) {
                lore = lore[(0) as usize..(((lore.len() as i32).wrapping_sub(2)).max(0)) as usize].to_vec();
            } else if compat::read(__this, |__t| __t.state.clone()).unwrap() != State::Locked {
                lore = lore[(0) as usize..(((lore.len() as i32).wrapping_sub(3)).max(0)) as usize].to_vec();
            }
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut compat::font(), lore.clone(), mouse_x, mouse_y);
        }
    }

    /// WidgetSlotEntry.extractTooltip as the dispatch of WidgetsListEntry.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn WidgetsListEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: i32, __p5: i32, __p6: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5, __p6)
    }

}

/// WidgetSlotEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for WidgetSlotEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// WidgetSlotEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<WidgetSlotEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| WidgetSlotEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| WidgetSlotEntry::mouse_clicked(__h, event, double_click))
    }

}

/// java: public enum State
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum State {
    Enabled,
    Disabled,
    Locked,
}

impl State {
    pub const VALUES: [State; 3] = [State::Enabled, State::Disabled, State::Locked];

    /// java: values()
    pub fn values() -> Vec<State> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            State::Enabled => "ENABLED",
            State::Disabled => "DISABLED",
            State::Locked => "LOCKED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> State {
        match name {
            "ENABLED" => State::Enabled,
            "DISABLED" => State::Disabled,
            "LOCKED" => State::Locked,
            _ => panic!("IllegalArgumentException: No enum constant State.{name}"),
        }
    }

}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
