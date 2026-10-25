//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.OptionDropdownWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::FinderSettingsContainer;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::OptionWidget;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;

/// java: private static final int CLOSED_HEIGHT = 35;
pub const CLOSED_HEIGHT: i32 = 35;

/// java: public class OptionDropdownWidget extends AbstractSelectionList<OptionDropdownWidget.AbstractEntry>
pub struct OptionDropdownWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList - the game's list with its rows (`Entry.object`)
    /// and this class's own overrides of the list's methods (`DropdownListClass`)
    pub base: ModSelectionList,
    /// The list's `AbstractWidget` half (`AbstractSelectionList extends AbstractWidget`) the game's list draw is handed:
    /// this list is drawn by its container, not as a widget of the screen, so its box is copied in before each draw.
    pub __widget: mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
    /// java: private final int slotId;
    pub slot_id: i32,
    /// java: private final Component name;
    pub name: std::option::Option<MutableComponent>,
    /// java: private final int maxHeight;
    pub max_height: i32,
    /// java: private final PartyFinderScreen screen;
    pub screen: compat::Shared<PartyFinderScreen>,
    /// java: private final Header header = new Header();
    pub header: compat::Shared<Header>,
    /// java: private int backButtonId = -1;
    pub back_button_id: i32,
    /// java: private @Nullable Option selectedOption;
    pub selected_option: std::option::Option<compat::Shared<Option>>,
    /// java: private boolean isOpen = false;
    pub is_open: bool,
    /// java: private float animationProgress = 0f;
    pub animation_progress: f32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for OptionDropdownWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for OptionDropdownWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl OptionDropdownWidget {
    /// java: public OptionDropdownWidget(PartyFinderScreen screen, Component name, int x, int y, int width, int maxHeight, int slotId)
    pub fn new(mut screen: compat::Shared<PartyFinderScreen>, mut name: MutableComponent, mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut slot_id: i32) -> compat::Shared<Self> {
        let __init_max_height = max_height;
        let __init_screen = screen.clone();
        let __init_slot_id = slot_id;
        //   java: super(screen.getClient(), width, CLOSED_HEIGHT, y, 15); - `AbstractSelectionList(minecraft, width, height, y, itemHeight)`
        let __this = compat::shared(Self {
            base: ModSelectionList::with_class(mcrust::mcrust::modding::widgets::ListKind::Object, width, CLOSED_HEIGHT, y, 15, std::sync::Arc::new(DropdownListClass)),
            __widget: compat::string_widget(0, 0, 0, 0, component::empty()),
            slot_id: __init_slot_id,
            name: None,
            max_height: __init_max_height,
            screen: __init_screen,
            // java: private final Header header = new Header(); - the inner class's enclosing instance is set below
            header: Header::new(std::sync::Weak::new()),
            back_button_id: -1,
            selected_option: None,
            is_open: false,
            animation_progress: 0f32,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __header = compat::read(&__this, |__t| __t.header.clone()); compat::write(&__header, |__t| __t.outer = std::sync::Arc::downgrade(&__this)); }
        __this.lock().borrow_mut().set_x(x);
        { let __v = Some(name.clone()); compat::write(&__this, |__t| __t.name = __v); }
        let __r0 = compat::read(&__this, |__t| __t.header.clone());
        { let (__q0,) = (row_entry(__r0),); __this.lock().borrow_mut().add_entry(__q0) };
        __this
    }

    /// java: @Override public int getRowLeft()
    pub fn get_row_left(__this: &compat::Shared<Self>) -> i32 {
        return (__this.lock().borrow().get_x()).wrapping_add(2);
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return (OptionDropdownWidget::get_row_left(__this)).wrapping_add(OptionDropdownWidget::get_row_width(__this));
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return (__this.lock().borrow().get_width()).wrapping_sub(6);
    }

    /// java: public void setSelectedOption(OptionDropdownWidget.Option entry)
    pub fn set_selected_option(__this: &compat::Shared<Self>, mut entry: compat::Shared<Option>) {
        { let __v = Some(entry.clone()); compat::write(__this, |__t| __t.selected_option = __v); }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !(FinderSettingsContainer::can_interact(&PartyFinderScreen::get_settings_container(&compat::read(__this, |__t| __t.screen.clone())), Some(&OptionWidget::Dropdown(__this.clone())))) {
            return false;
        }
        if compat::read(__this, |__t| __t.is_open) {
            if !({ let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().is_mouse_over(__q0, __q1) }) && compat::read(__this, |__t| __t.back_button_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.screen.clone()), compat::read(__this, |__t| __t.back_button_id));
                return true;
            }
        }
        // java: return super.mouseClicked(click, doubled); - `AbstractContainerWidget.mouseClicked` over the rows. The rows'
        // own `mouseClicked` read and write this object, so the list is taken out for the call (`with_list_lent`).
        return OptionDropdownWidget::with_list_lent(__this, |list| list.mouse_clicked_base(click, doubled));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// (`context` is the frame's render context the container's draw is handed: `super.extractWidgetRenderState` is the
    /// game's list draw, which takes it)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if compat::read(__this, |__t| __t.is_open) {
            if compat::read(__this, |__t| __t.animation_progress) < (1 as f32) {
                { let __v = (compat::read(__this, |__t| __t.animation_progress) + (a * 0.5f32)); compat::write(__this, |__t| __t.animation_progress = __v); }
            } else if compat::read(__this, |__t| __t.animation_progress) != (1 as f32) {
                { let __v = ((1) as f32) as f32; compat::write(__this, |__t| __t.animation_progress = __v); }
            }
        } else {
            { let __v = ((0) as f32) as f32; compat::write(__this, |__t| __t.animation_progress = __v); }
        }
        if compat::get_state(&party_finder_screen::DEBUG) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.slot_id).to_string()), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_sub(10), mcrust::net::minecraft::util::common_colors::RED, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.back_button_id).to_string()), (__this.lock().borrow().get_x()).wrapping_add(50), (__this.lock().borrow().get_y()).wrapping_sub(10), mcrust::net::minecraft::util::common_colors::RED, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::fstr(compat::read(__this, |__t| __t.animation_progress) as f64)), (__this.lock().borrow().get_x()).wrapping_sub(10), __this.lock().borrow().get_y(), mcrust::net::minecraft::util::common_colors::GREEN, true);
        }
        if compat::read(__this, |__t| __t.is_open) {
            let mut list_height: i32 = (__this.lock().borrow().get_height()).min(((__this.lock().borrow().content_height()).wrapping_sub(Header::get_height(&compat::read(__this, |__t| __t.header.clone())))).wrapping_sub(4));
            let mut opened_list_height: i32 = (if compat::read(__this, |__t| __t.is_open) { ((((list_height as f32) * compat::read(__this, |__t| __t.animation_progress))) as i32) } else { ((((list_height as f32) * ((1 as f32) - compat::read(__this, |__t| __t.animation_progress)))) as i32) });
            let __o1 = compat::read(__this, |__t| __t.header.clone());
            let __o2 = compat::read(__this, |__t| __t.header.clone());
            let __o3 = compat::read(__this, |__t| __t.header.clone());
            let __o4 = compat::read(__this, |__t| __t.header.clone());
            let __o5 = compat::read(__this, |__t| __t.header.clone());
            let __o6 = compat::read(__this, |__t| __t.header.clone());
            let __o7 = compat::read(__this, |__t| __t.header.clone());
            let __o8 = compat::read(__this, |__t| __t.header.clone());
            let __o9 = compat::read(__this, |__t| __t.header.clone());
            let __o10 = compat::read(__this, |__t| __t.header.clone());
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (compat::write(&__o7, |__t| __t.get_y())).wrapping_add(Header::get_height(&compat::read(__this, |__t| __t.header.clone()))), ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(1), ((compat::write(&__o10, |__t| __t.get_y())).wrapping_add(opened_list_height)).wrapping_add(Header::get_height(&compat::read(__this, |__t| __t.header.clone()))), 0xFFF0F0F0u32 as i32);
            let __o11 = compat::read(__this, |__t| __t.header.clone());
            let __o12 = compat::read(__this, |__t| __t.header.clone());
            let __o13 = compat::read(__this, |__t| __t.header.clone());
            let __o14 = compat::read(__this, |__t| __t.header.clone());
            let __o15 = compat::read(__this, |__t| __t.header.clone());
            let __o16 = compat::read(__this, |__t| __t.header.clone());
            let __o17 = compat::read(__this, |__t| __t.header.clone());
            let __o18 = compat::read(__this, |__t| __t.header.clone());
            let __o19 = compat::read(__this, |__t| __t.header.clone());
            let __o20 = compat::read(__this, |__t| __t.header.clone());
            let __o21 = compat::read(__this, |__t| __t.header.clone());
            let __o22 = compat::read(__this, |__t| __t.header.clone());
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), ((compat::write(&__o19, |__t| __t.get_y())).wrapping_add(Header::get_height(&compat::read(__this, |__t| __t.header.clone())))).wrapping_add(1), ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(2), (((compat::write(&__o22, |__t| __t.get_y())).wrapping_add(opened_list_height)).wrapping_add(Header::get_height(&compat::read(__this, |__t| __t.header.clone())))).wrapping_sub(1), mcrust::net::minecraft::util::common_colors::BLACK);
        }
        // java: super.extractWidgetRenderState(graphics, mouseX, mouseY, a); - the game's list draw (`ModSelectionList`'s
        // `extractWidgetRenderState`), handed the list's box; the rows' `extractContent` read this object, so the list is
        // taken out for the draw (`with_list_lent`)
        let __widget_box = compat::write(__this, |__t| {
            let (x, y, w, h, visible, active) = (__t.base.list.get_x(), __t.base.list.get_y(), __t.base.list.get_width(), __t.base.list.get_height(), __t.base.visible, __t.base.active);
            __t.__widget.set_position(x, y);
            __t.__widget.width = w;
            __t.__widget.height = h;
            __t.__widget.visible = visible;
            __t.__widget.active = active;
            std::mem::replace(&mut __t.__widget, compat::string_widget(0, 0, 0, 0, component::empty()))
        });
        OptionDropdownWidget::with_list_lent(__this, |list| {
            use mcrust::mcrust::modding::widgets::ModWidget as _;
            list.extract_widget_render_state(&__widget_box, context, mouse_x, mouse_y, a);
        });
        compat::write(__this, |__t| __t.__widget = __widget_box);
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    pub fn extract_list_separators(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    pub fn extract_list_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: public void open(List<Option> entries, int backButtonId)
    pub fn open(__this: &compat::Shared<Self>, entries: &Vec<compat::Shared<Option>>, mut back_button_id: i32) {
        if compat::read(__this, |__t| __t.is_open) {
            return;
        }
        { let __v = true; compat::write(__this, |__t| __t.is_open = __v); }
        { let __v = compat::read(__this, |__t| __t.max_height); compat::write(__this, |__t| __t.height = __v); }
        { let __v = 0.0f32; compat::write(__this, |__t| __t.animation_progress = __v); }
        // java: clearEntriesExcept(header); - the header is the list's first row (the constructor adds it, every clear
        // keeps it)
        compat::write(__this, |__t| __t.base.list.clear_entries_except(0));
        for __a0 in entries.iter() {
            { let (__q0,) = (row_entry(__a0.clone()),); __this.lock().borrow_mut().add_entry(__q0) };
        }
        { let __v = back_button_id; compat::write(__this, |__t| __t.back_button_id = __v); }
    }

    /// java: public void close()
    pub fn close(__this: &compat::Shared<Self>) {
        if !(compat::read(__this, |__t| __t.is_open)) {
            return;
        }
        { let __v = false; compat::write(__this, |__t| __t.is_open = __v); }
        { let __v = CLOSED_HEIGHT; compat::write(__this, |__t| __t.height = __v); }
        // java: clearEntriesExcept(header); - the header is the list's first row (the constructor adds it, every clear
        // keeps it)
        compat::write(__this, |__t| __t.base.list.clear_entries_except(0));
    }

    /// The list, taken out of this object for `f` (a dropdown with this one's box stands in): the rows' own methods, which
    /// the list's bodies call, read and write this object (`OptionDropdownWidget.this`), which the call must not hold.
    fn with_list_lent<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut ModSelectionList) -> R) -> R {
        let mut list: ModSelectionList = compat::write(__this, |__t| {
            let (x, y, w, h) = (__t.base.list.get_x(), __t.base.list.get_y(), __t.base.list.get_width(), __t.base.list.get_height());
            let mut stand_in = ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Object, w, h, y, 15);
            stand_in.list.set_x(x);
            std::mem::replace(&mut __t.base, stand_in)
        });
        let answer = f(&mut list);
        compat::write(__this, |__t| __t.base = list);
        answer
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    pub fn entries_can_be_selected(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// `OptionDropdownWidget`'s overrides of the list's own methods (`getRowLeft`, `getRowWidth`, `scrollBarX`,
/// `entriesCanBeSelected`, `extractListBackground`, `extractListSeparators`): the game's list asks them where its bodies
/// call the virtual. Their bodies read only the list's box (`getX()`, `getWidth()`), which `list` is - so they are
/// answered from it rather than through the object, which the list's own calls may be holding.
struct DropdownListClass;

impl mcrust::mcrust::modding::widgets::ModList for DropdownListClass {
    /// java: @Override public int getRowLeft() - `getX() + 2`
    fn get_row_left(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        let _ = super_answer;
        list.get_x().wrapping_add(2)
    }

    /// java: @Override public int getRowWidth() - `getWidth() - 6`
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        let _ = super_answer;
        list.get_width().wrapping_sub(6)
    }

    /// java: @Override protected int scrollBarX() - `getRowLeft() + getRowWidth()`
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        let _ = super_answer;
        list.get_x().wrapping_add(2).wrapping_add(list.get_width().wrapping_sub(6))
    }

    /// java: @Override protected boolean entriesCanBeSelected() - `false`
    fn entries_can_be_selected(&self, list: &AbstractSelectionList, super_answer: bool) -> bool {
        let _ = (list, super_answer);
        false
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics) - empty
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = (list, context, super_extract_list_background);
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics) - empty
    fn extract_list_separators(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_separators: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = (list, context, super_extract_list_separators);
    }
}

/// java: private class Header extends AbstractEntry
pub struct Header {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: OptionDropdownWidget.this
    pub outer: compat::WeakShared<OptionDropdownWidget>,
}

impl std::ops::Deref for Header {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Header {
    /// java: the implicit no-argument constructor (`outer` is the enclosing `OptionDropdownWidget.this`)
    pub fn new(outer: compat::WeakShared<OptionDropdownWidget>) -> compat::Shared<Self> {
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                outer,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        __this
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return true; };
        let screen = compat::read(&outer, |__t| __t.screen.clone());
        //   java: if (screen.isWaitingForServer()) return false;
        if PartyFinderScreen::is_waiting_for_server(&screen) {
            return false;
        }
        //   java: if (isOpen) { if (backButtonId != -1) screen.clickAndWaitForServer(backButtonId); }
        if compat::read(&outer, |__t| __t.is_open) {
            let back_button_id = compat::read(&outer, |__t| __t.back_button_id);
            if back_button_id != -1 {
                PartyFinderScreen::click_and_wait_for_server(&screen, back_button_id);
            }
        } else {
            //   java: animationProgress = 0f; screen.clickAndWaitForServer(slotId); screen.partyFinderButton.active = false;
            compat::write(&outer, |__t| __t.animation_progress = 0f32);
            PartyFinderScreen::click_and_wait_for_server(&screen, compat::read(&outer, |__t| __t.slot_id));
            compat::write(&screen, |__t| { let __s = &mut *__t; if let Some(__r) = __s.party_finder_button.as_mut() { __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = false) } });
        }
        return true;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(__this: &compat::Shared<Self>) -> i32 {
        return 25;
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let mut x: i32 = __this.lock().borrow().get_x();
        let mut y: i32 = __this.lock().borrow().get_y();
        let outer = compat::read(__this, |__t| __t.outer.upgrade());
        //   java: graphics.text(Minecraft.getInstance().font, name, x, y + 1, 0xFFD0D0D0, false);
        if let Some(name) = outer.as_ref().and_then(|__o| compat::read(__o, |__t| __t.name.clone())) {
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(name), x, y.wrapping_add(1), 0xFFD0D0D0u32 as i32, false);
        }
        let mut offset: i32 = 10;
        let outer_width: i32 = outer.as_ref().map_or(0, |__o| compat::read(__o, |__t| __t.get_width()));
        //   java: graphics.fill(x - 2, y + offset, x - 3 + OptionDropdownWidget.this.getWidth(), y + 15 + offset, 0xFFF0F0F0);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x.wrapping_sub(2), y.wrapping_add(offset), x.wrapping_sub(3).wrapping_add(outer_width), y.wrapping_add(15).wrapping_add(offset), 0xFFF0F0F0u32 as i32);
        //   java: graphics.fill(x - 1, y + 1 + offset, x - 3 + OptionDropdownWidget.this.getWidth() - 1, y + 14 + offset, CommonColors.BLACK);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x.wrapping_sub(1), y.wrapping_add(1).wrapping_add(offset), x.wrapping_sub(3).wrapping_add(outer_width).wrapping_sub(1), y.wrapping_add(14).wrapping_add(offset), mcrust::net::minecraft::util::common_colors::BLACK);
        //   java: if (selectedOption != null)
        if let Some(selected_option) = outer.as_ref().and_then(|__o| compat::read(__o, |__t| __t.selected_option.clone())) {
            //   java: graphics.text(Minecraft.getInstance().font, selectedOption.message, x + 2, y + 3 + offset, CommonColors.WHITE, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(&selected_option, |__t| __t.message.clone())), x.wrapping_add(2), (y.wrapping_add(3)).wrapping_add(offset), mcrust::net::minecraft::util::common_colors::WHITE, true);
        } else {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("???"), x.wrapping_add(2), (y.wrapping_add(3)).wrapping_add(offset), mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
    }

}

impl AbstractEntry for Header {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// Header's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for Header {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Header as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<Header> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| Header::extract_content(__h, &mut context.graphics, mouse_x, mouse_y, hovered, a));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| Header::mouse_clicked(__h, event, double_click))
    }

}

/// java: public class Option extends AbstractEntry
pub struct Option {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: private final String message;
    pub message: String,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final int optionSlotId;
    pub option_slot_id: i32,
    /// java: OptionDropdownWidget.this (`dropdownWidget.new Option(..)`)
    pub outer: compat::WeakShared<OptionDropdownWidget>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for Option {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Option {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Option {
    /// java: public Option(String message, @Nullable ItemStack icon, int slotId)
    ///
    /// (`outer` is the enclosing instance: Java's `dropdownWidget.new Option(..)`)
    pub fn new(outer: &compat::Shared<OptionDropdownWidget>, mut message: String, mut icon: std::option::Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut slot_id: i32) -> compat::Shared<Self> {
        let __init_outer = std::sync::Arc::downgrade(outer);
        let __init_message = message.clone();
        let __init_icon = icon.clone().unwrap();
        let __init_option_slot_id = slot_id;
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                message: __init_message,
                icon: __init_icon,
                option_slot_id: __init_option_slot_id,
                outer: __init_outer,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        __this
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        let mut icon_y: i32 = (__this.lock().borrow().get_y()).wrapping_add(1);
        gui_graphics_extractor::pose_translate((__this.lock().borrow().get_x()) as f32, (icon_y) as f32);
        gui_graphics_extractor::pose_scale((0.8f32) as f32, (0.8f32) as f32);
        gui_graphics_extractor::pose_translate(((-__this.lock().borrow().get_x())) as f32, ((-icon_y)) as f32);
        graphics.item(&compat::read(__this, |__t| __t.icon.clone()), __this.lock().borrow().get_x(), icon_y);
        gui_graphics_extractor::pop_pose(__pose_1);
        if compat::get_state(&party_finder_screen::DEBUG) {
            //   java: graphics.text(minecraft.font, String.valueOf(optionSlotId), this.getX() + 8, this.getY(), CommonColors.RED, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.option_slot_id).to_string()), (__this.lock().borrow().get_x()).wrapping_add(8), __this.lock().borrow().get_y(), mcrust::net::minecraft::util::common_colors::RED, true);
        }
        let mut text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.message.clone()))).with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(hovered)));
        //   java: minecraft.font.width(text) >= this.getWidth() - 14
        let __text = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone());
        let (__x, __y, __width) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width()));
        if mcrust::net::minecraft::client::gui::font::width(compat::minecraft().font.splitter().string_width_formatted(&__text)) >= __width.wrapping_sub(14) {
            //   java: graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(text, this.getX() + 14, this.getX() + this.getWidth(), getY() + 3, this.getY() + 3 + minecraft.font.lineHeight);
            accept_scrolling_with_default_center(&__text, __x.wrapping_add(14), __x.wrapping_add(__width), __y.wrapping_add(3), __y.wrapping_add(3).wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT));
        } else {
            //   java: graphics.text(minecraft.font, text, this.getX() + 14, this.getY() + 3, CommonColors.WHITE, false);
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__text, __x.wrapping_add(14), __y.wrapping_add(3), mcrust::net::minecraft::util::common_colors::WHITE, false);
        }
    }

    /// java: @Override public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        // if (o == null || getClass() != o.getClass()) return false; - an object of this class is held by its handle
        let Some(that) = o.downcast_ref::<compat::Shared<Option>>() else {
            return false;
        };
        // if (this == o) return true;
        if self.self_ref.upgrade().is_some_and(|this| std::sync::Arc::ptr_eq(&this, that)) {
            return true;
        }
        // Option that = (Option) o; return message.equals(that.message);
        return (self.message == compat::read(that, |__t| __t.message.clone()));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return true; };
        let screen = compat::read(&outer, |__t| __t.screen.clone());
        //   java: if (screen.isWaitingForServer()) return false;
        if PartyFinderScreen::is_waiting_for_server(&screen) {
            return false;
        }
        if click.button() == 1 {
            //   java: screen.clickAndWaitForServer(this.optionSlotId);
            PartyFinderScreen::click_and_wait_for_server(&screen, compat::read(__this, |__t| __t.option_slot_id));
            //   java: setSelectedOption(this);
            OptionDropdownWidget::set_selected_option(&outer, __this.clone());
        }
        return true;
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return compat::string_hash_code(&self.message);
    }

}

impl AbstractEntry for Option {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// Option's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for Option {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Option as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<Option> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| Option::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| Option::mouse_clicked(__h, event, double_click))
    }

}

/// java: abstract static class AbstractEntry extends AbstractSelectionList.Entry<AbstractEntry>
pub struct AbstractEntryBase {
    /// java: extends net.minecraft.client.gui.components.AbstractSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractEntry>, __R)) -> (compat::Shared<dyn AbstractEntry>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractEntry (implemented by AbstractEntryBase and every subclass)
pub trait AbstractEntry: compat::DynAny {
    fn abstract_entry_base(&self) -> &AbstractEntryBase;
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

}

/// AbstractEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.abstract_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<Header, _>(__h) { return Header::extract_content(&*__c, &mut context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<Option, _>(__h) { return Option::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); }  });
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<Header, _>(__h) { return Header::mouse_clicked(&*__c, event, double_click); } if let Some(__c) = compat::try_view::<Option, _>(__h) { return Option::mouse_clicked(&*__c, event, double_click); } false })
    }

}

/// `graphics.textRenderer(HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(message, left, right, top, bottom)`
/// from a draw that holds only the extractor: the scrolling placement and the clipped draw, as
/// `compat::accept_scrolling_with_default_center` does with a widget render context
fn accept_scrolling_with_default_center(message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, left: i32, right: i32, top: i32, bottom: i32) {
    let minecraft = &*compat::minecraft();
    let width = mcrust::net::minecraft::client::gui::font::width(minecraft.font.splitter().string_width_formatted(message));
    let placement = mcrust::net::minecraft::client::gui::active_text_collector::accept_scrolling_with_default_center(left, right, top, bottom, width, mcrust::net::minecraft::util::util::get_millis());
    gui_graphics_extractor::draw_clipped_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), minecraft, placement.x, placement.y, message, -1, &mcrust::net::minecraft::network::chat::style::Style::EMPTY, placement.scissor, None, minecraft.window.get_gui_scale(), minecraft.window.get_height());
}

/// java: `addEntry(row)` on the list's `AbstractSelectionList` half - the port's `Entry` is the row's box, and the
/// row OBJECT rides on it (`Entry.object`), as every mod list's rows do.
fn row_entry<E: ?Sized + 'static>(row: compat::Shared<E>) -> mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry
where
    E: compat::RowHalf,
    compat::EntryAdapter<E>: ModListEntry,
{
    let mut entry = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0);
    entry.object = Some(mcrust::mcrust::modding::widgets::ModListEntrySlot::new(Box::new(compat::EntryAdapter::new(row))));
    entry
}
