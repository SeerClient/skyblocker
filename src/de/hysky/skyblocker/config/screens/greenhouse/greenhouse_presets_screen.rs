//! Ported from de.hysky.skyblocker.config.screens.greenhouse.GreenhousePresetsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class GreenhousePresetsScreen extends Screen
pub struct GreenhousePresetsScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private PresetsList presetsList;
    pub presets_list: Option<compat::WidgetRef>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for GreenhousePresetsScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GreenhousePresetsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GreenhousePresetsScreen {
    /// java: public GreenhousePresetsScreen(@Nullable Screen parent)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.title").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            layout: None,
            parent: __init_parent,
            presets_list: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: public GreenhousePresetsScreen()
    pub fn new_0() -> compat::Shared<Self> {
        let __this = Self::new(None);
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `HeaderAndFooterLayout.addTitleHeader` has no Rust method in the port (looked for `add_title_header` in HeaderAndFooterLayout and its supertypes)
        // rust: let __h0 = __this.lock().borrow().get_title();
        // rust: let __h1 = &compat::font();
        // rust: __this.lock().borrow().layout.as_ref().unwrap().add_title_header(__h0, __h1);
        { let __v = (Some(PresetsList::new(compat::minecraft(), compat::read(__this, |__t| __t.width), __this.lock().borrow().layout.as_ref().unwrap().get_content_height()))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.presets_list = __v; }); }
        { let (__q0,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.presets_list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        // TODO(fabric2seer): `.forEach(..)` on a T
        //   java: GreenhousePaste.PRESETS_DATA.getData().forEach((name, link) -> presetsList.addEntry(new PresetEntry(name, link)));
        let mut footer: LinearLayout = LinearLayout::horizontal().spacing(8);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer.add_child({ let __w = (button::builder(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.addPreset").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { let __wa2 = PresetEntry::new_0(); compat::write((&__this_1), |__t| { let __s = &mut *__t; let __r = __s.presets_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<PresetsList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || PresetsList::add_entry(__h, &__wa2)) })).expect("a mod list reached from inside its own method")) }); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer.add_child({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { GreenhousePresetsScreen::on_close((&__this_2)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(footer.clone().wrapped),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        GreenhousePresetsScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __wa1 = compat::read(__this, |__t| __t.width);
        let __wa2 = compat::read(__this, |__t| __t.layout.clone());
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.presets_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.update_size(__wa1, __wa2.as_ref().unwrap()))).expect("a mod list reached from inside its own method")) });
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.clear(..)` on a T
        //   java: GreenhousePaste.PRESETS_DATA.getData().clear();
        for entry in compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.presets_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| compat::list_entries::<PresetEntry>(__l))).expect("a mod list reached from inside its own method")) }) {
            let mut name: String = entry.lock().borrow().name_box.as_ref().unwrap().get_value();
            let mut link: String = entry.lock().borrow().link_box.as_ref().unwrap().get_value();
            if compat::is_blank(&name) || compat::is_blank(&link) {
                continue;
            }
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: GreenhousePaste.PRESETS_DATA.getData().put(name.trim(), link.trim());
        }
        greenhouse_paste::PRESETS_DATA.get_mut().lock().borrow().save();
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// GreenhousePresetsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for GreenhousePresetsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// GreenhousePresetsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<GreenhousePresetsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| GreenhousePresetsScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| GreenhousePresetsScreen::reposition_elements(__h))
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
        self.call(|__h| GreenhousePresetsScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class PresetsList extends ContainerObjectSelectionList<PresetEntry>
pub struct PresetsList {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for PresetsList {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for PresetsList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl PresetsList {
    /// java: private PresetsList(Minecraft minecraft, int width, int height)
    pub fn new(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, 0, 24),
        });
        __this
    }

    /// java: @Override protected void removeEntry(PresetEntry entry)
    pub fn remove_entry(__this: &compat::Shared<Self>, entry: &compat::Shared<PresetEntry>) {
        let __entry = entry.clone();
        compat::write(__this, |__t| { let __l = &mut __t.base; { if let Some(__i) = compat::entry_index::<PresetEntry>(__l, &__entry) { __l.remove_entry(__i); } } });
    }

    /// java: @Override protected int addEntry(PresetEntry entry)
    pub fn add_entry(__this: &compat::Shared<Self>, entry: &compat::Shared<PresetEntry>) -> i32 {
        let __entry = entry.clone();
        return compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 360;
    }

}

/// PresetsList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for PresetsList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// PresetsList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<PresetsList> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| PresetsList::get_row_width(__h))
    }

}

// NOTE(fabric2seer): addEntry, getRowWidth, removeEntry override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class PresetEntry extends ContainerObjectSelectionList.Entry<PresetEntry>
pub struct PresetEntry {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final EditBox nameBox;
    pub name_box: Option<EditBox>,
    /// java: private final EditBox linkBox;
    pub link_box: Option<EditBox>,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: Vec<AbstractWidget>,
    /// java: private final LinearLayout layout = LinearLayout.horizontal().spacing(8);
    pub layout: LinearLayout,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for PresetEntry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PresetEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PresetEntry {
    /// java: private PresetEntry(String name, String link)
    pub fn new(mut name: String, mut link: String) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            name_box: None,
            link_box: None,
            widgets: Default::default(),
            layout: LinearLayout::horizontal().spacing(8),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): unknown name `font`
        //   java: nameBox = layout.addChild(new EditBox(font, Component.empty()));
        __this.lock().borrow_mut().name_box.as_mut().unwrap().set_width(100);
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: __this.lock().borrow_mut().name_box.as_mut().unwrap().set_value(&name);
        __this.lock().borrow_mut().name_box.as_mut().unwrap().set_max_length(64);
        { let (__q0,) = (component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.name"),); __this.lock().borrow_mut().name_box.as_mut().unwrap().set_hint_component(__q0) };
        // TODO(fabric2seer): unknown name `font`
        //   java: linkBox = layout.addChild(new EditBox(font, Component.empty()));
        __this.lock().borrow_mut().link_box.as_mut().unwrap().set_width(180);
        __this.lock().borrow_mut().link_box.as_mut().unwrap().set_max_length(2048);
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: __this.lock().borrow_mut().link_box.as_mut().unwrap().set_value(&link);
        { let (__q0,) = (component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.link"),); __this.lock().borrow_mut().link_box.as_mut().unwrap().set_hint_component(__q0) };
        // TODO(fabric2seer): `presetsList` is a field of the enclosing instance (GreenhousePresetsScreen.this) - an inner class has no outer reference in Rust
        //   java: layout.addChild(Button.builder(Component.translatable("gui.remove"), _ -> presetsList.removeEntry(this)).width(80).build());
        { let __v = Vec::new(); compat::write(&__this, |__t| __t.widgets = __v); }
        { __this.lock().borrow_mut().layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(&__this, |__t| __t.__row.widget_mut(__i).set_position(__x, __y)); } };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(&__this, |__t| __t.__row.mark_renderable(__i)); } };
        __this
    }

    /// java: private PresetEntry()
    pub fn new_0() -> compat::Shared<Self> {
        let __this = Self::new("".to_string(), "".to_string());
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.widgets.clone());
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: { let (__q0, __q1,) = ((__this.lock().borrow().get_content_x_middle()).wrapping_sub((__this.lock().borrow().layout.get_width()).wrapping_div(2)), (__this.lock().borrow().get_content_y_middle()).wrapping_sub((__this.lock().borrow().layout.get_height()).wrapping_div(2)),); __this.lock().borrow_mut().layout.set_position(__q0, __q1) };
        for widget in compat::read(__this, |__t| __t.widgets.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.widgets.clone());
    }

}

/// PresetEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for PresetEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// PresetEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<PresetEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| PresetEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

// NOTE(fabric2seer): children, extractContent, narratables override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
