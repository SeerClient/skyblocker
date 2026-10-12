//! Ported from de.hysky.skyblocker.utils.render.gui.SoundSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::cycle_button::CycleButton;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Component YES_TEXT = CommonComponents.GUI_YES.copy().withStyle(ChatFormatting.GREEN);
pub static YES_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| mcrust::net::minecraft::network::chat::common_components::gui_yes().copy().with_style_format(ChatFormatting::Green));
/// java: private static final Component NO_TEXT = CommonComponents.GUI_NO.copy().withStyle(ChatFormatting.RED);
pub static NO_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| mcrust::net::minecraft::network::chat::common_components::gui_no().copy().with_style_format(ChatFormatting::Red));

/// java: public class SoundSelectionPopup extends AbstractPopupScreen
pub struct SoundSelectionPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final List<AbstractWidget> filteredWidgets = new ObjectArrayList<>();
    pub filtered_widgets: Vec<AbstractWidget>,
    /// java: private final Consumer<@Nullable SoundEvent> onDone;
    pub on_done: Arc<dyn Fn(Option<SoundEvent>)>,
    /// java: private @Nullable SoundEvent selectedSound = null;
    pub selected_sound: Option<SoundEvent>,
    /// java: private boolean advanced = false;
    pub advanced: bool,
    /// java: private final GridLayout gridWidget = new GridLayout();
    pub grid_widget: GridLayout,
    /// java: private LinearLayout listLayout = new LinearLayout(0, 0, LinearLayout.Orientation.VERTICAL);
    pub list_layout: LinearLayout,
    /// java: private @Nullable Button doneButton;
    pub done_button: Option<compat::WidgetRef>,
    /// java: private @Nullable ListContainer widgetsContainer;
    pub widgets_container: Option<ListContainer>,
}

impl std::ops::Deref for SoundSelectionPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SoundSelectionPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SoundSelectionPopup {
    /// java: public SoundSelectionPopup(Screen backgroundScreen, Consumer<@Nullable SoundEvent> onDone)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<SoundEvent>)>) -> compat::Shared<Self> {
        let __init_on_done = on_done.clone();
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(Component.literal("Select Sound"), backgroundScreen);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: private @Nullable Component getSoundName(SoundEvent sound)
    pub fn get_sound_name(__this: &compat::Shared<Self>, sound: &SoundEvent) -> Option<MutableComponent> {
        let mut key: String = mcrust::net::minecraft::core::registries::built_in_registries::sound_event().get_key(sound).unwrap().to_short_language_key();
        let mut translation: MutableComponent = component::translatable_with_fallback(&format!("subtitles.{}", key), Some("null"));
        if !(translation.get_string() == "null") {
            return Some(translation);
        }
        if key.contains("note_block") {
            let mut split: Vec<String> = compat::regex::split(&key, "\\.");
            return Some(component::literal(&format!("note block {}", (split[((split.len() as i32).wrapping_sub(1)) as usize]).replace("_", " "))));
        }
        if compat::read(__this, |__t| __t.advanced) {
            return Some(component::literal(&key));
        } else {
            return None;
        }
    }

    /// java: private void filterSounds(String input)
    pub fn filter_sounds(__this: &compat::Shared<Self>, input: &str) {
        __this.lock().borrow_mut().filtered_widgets.clear();
        for sound_event in mcrust::net::minecraft::core::registries::built_in_registries::sound_event().iter() {
            let mut translation: Option<MutableComponent> = SoundSelectionPopup::get_sound_name(__this, sound_event);
            if translation.is_some() && translation.as_ref().unwrap().get_string().to_lowercase().contains(&input.to_lowercase()) {
                let mut widget: AbstractWidget = SoundWidget::new(translation.clone().unwrap(), sound_event.clone());
                let __h0 = widget.clone();
                __this.lock().borrow_mut().filtered_widgets.push(__h0);
            }
        }
        SoundSelectionPopup::recreate_list(__this);
    }

    /// java: private void recreateList()
    pub fn recreate_list(__this: &compat::Shared<Self>) {
        let mut list_x: i32 = __this.lock().borrow().list_layout.get_x();
        let mut list_y: i32 = __this.lock().borrow().list_layout.get_y();
        // TODO(fabric2seer): field `VERTICAL` of a value of unknown type
        //   java: listLayout = new LinearLayout(0, 0, LinearLayout.Orientation.VERTICAL);
        for __a0 in compat::read(__this, |__t| __t.filtered_widgets.clone()) {
            { let (__q0,) = ({ let __w = __a0.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().list_layout.add_child(__q0) };
        }
        { __this.lock().borrow_mut().list_layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().list_layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().list_layout.set_position(list_x, list_y);
        if compat::read(__this, |__t| __t.widgets_container.clone()).is_some() {
            // TODO(fabric2seer): `refresh_scroll_amount` takes 1 argument(s) in the port (`content_height: i32`), Java passes 0
            // TODO(fabric2seer): `ScrollArea::refresh_scroll_amount` ports `refreshScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
            // rust: __this.lock().borrow_mut().widgets_container.as_mut().unwrap().refresh_scroll_amount();
        }
    }

}

impl AbstractPopupScreen for SoundSelectionPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        SoundSelectionPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        SoundSelectionPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        SoundSelectionPopup::extract_background_dyn
    }

}

impl SoundSelectionPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        let mut adder: RowHelper = __this.lock().borrow().grid_widget.create_row_helper(2);
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut search_field: EditBox = EditBox::new(&compat::font(), 200, 20, component::empty());
        let mut search_field: EditBox = todo!("fabric2seer: the initialiser of `searchField` (TODO above)");
        search_field.set_hint_component(component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray));
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        search_field.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { SoundSelectionPopup::filter_sounds((&__this_1), __a0); } })));
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: { let (__q0,) = (adder.add_child(&mut search_field).unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: let mut toggle_advanced: CycleButton = (cycle_button::boolean_builder(YES_TEXT.get_mut().get_string(), NO_TEXT.get_mut().get_string(), compat::read(__this, |__t| __t.advanced))).with_tooltip(|_| Tooltip::create((component::translatable("skyblocker.utils.render.gui.soundSelectionPopup.advanced.@Tooltip")).get_string())).create(0, 0, 100, 20, component::translatable("skyblocker.utils.render.gui.soundSelectionPopup.advanced"), |_, _| {
        // rust: { let __v = !(compat::read(__this_2.get(), |__t| __t.advanced)); compat::write(__this_2.get(), |__t| __t.advanced = __v); }
        // rust: SoundSelectionPopup::filter_sounds(__this_2.get(), &search_field.get_value());
        // rust: }).unwrap();
        let mut toggle_advanced: CycleButton = todo!("fabric2seer: the initialiser of `toggleAdvanced` (TODO above)");
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: { let (__q0,) = (adder.add_child(&mut toggle_advanced).unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        { let __v = Some(ListContainer::new(0, 0, 400, (compat::read(__this, |__t| __t.height)).wrapping_div(2))); compat::write(__this, |__t| __t.widgets_container = __v); }
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __r0 = compat::read(__this, |__t| __t.widgets_container.clone());
        // rust: { let (__q0,) = (adder.add_child_width(__r0.as_mut().unwrap(), 2).unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: { let (__q0,) = (adder.add_child(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
        // rust: ({ let __f = compat::dispatch((&__this_3), |__t| __t.on_close()); __f(&compat::view((&__this_3), |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        // rust: ((compat::read((&__this_3), |__t| __t.on_done.clone())))(None);
        // rust: } }) })))).build()).unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            ({ let __f = compat::dispatch((&__this_4), |__t| __t.on_close()); __f(&compat::view((&__this_4), |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
            ((compat::read((&__this_4), |__t| __t.on_done.clone())))(compat::read((&__this_4), |__t| __t.selected_sound.clone()));
        } }) })))).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.done_button = __v; }); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.done_button.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.active = __v) }); }
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: { let (__q0,) = (adder.add_child(compat::write(__this, |__t| { let __s = &mut *__t; match __s.done_button.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap()).unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        { __this.lock().borrow_mut().grid_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().grid_widget.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        ({ let __f = compat::dispatch(__this, |__t| __t.reposition_elements()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        SoundSelectionPopup::filter_sounds(__this, "");
    }

    /// SoundSelectionPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `GridLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.width);
        // rust: let __r1 = compat::read(__this, |__t| __t.height);
        // rust: { let (__q0, __q1,) = (((__r0).wrapping_sub(__this.lock().borrow().grid_widget.get_width())).wrapping_div(2), ((__r1).wrapping_sub(__this.lock().borrow().grid_widget.get_height())).wrapping_div(2),); __this.lock().borrow_mut().grid_widget.set_position(__q0, __q1) };
    }

    /// SoundSelectionPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().grid_widget.get_x(), __this.lock().borrow().grid_widget.get_y(), __this.lock().borrow().grid_widget.get_width(), __this.lock().borrow().grid_widget.get_height());
    }

    /// SoundSelectionPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// SoundSelectionPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for SoundSelectionPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// SoundSelectionPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<SoundSelectionPopup> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
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
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}

/// java: private class ListContainer extends AbstractContainerWidget
#[derive(Clone)]
pub struct ListContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
}

impl std::ops::Deref for ListContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ListContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ListContainer {
    /// java: ListContainer(int x, int y, int width, int height)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> Self {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, width, height, Component.literal("List"), AbstractScrollArea.defaultSettings(8));
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        // TODO(fabric2seer): `listLayout` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: return listLayout.getHeight();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((15) as f64) as f64;
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&self, mut scroll_y: f64) {
        // TODO(fabric2seer): `set_scroll_amount` takes 2 argument(s) in the port (`content_height: i32, scroll_amount: f64`), Java passes 1
        // TODO(fabric2seer): `ScrollArea::set_scroll_amount` ports `setScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: self.base.set_scroll_amount(scroll_y);
        // TODO(fabric2seer): `listLayout` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: listLayout.setY(getY() - (int) scrollAmount());
    }

    /// java: private boolean isVisible(AbstractWidget widget)
    pub fn is_visible(&self, widget: &AbstractWidget) -> bool {
        return (widget.get_bottom() >= self.get_y() && widget.get_y() < self.get_bottom());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.enable_scissor(self.get_x(), self.get_y(), self.get_right(), self.get_bottom());
        // TODO(fabric2seer): `filteredWidgets` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: for (AbstractWidget widget : filteredWidgets) {
        //   java: 				if (isVisible(widget)) widget.extractRenderState(graphics, mouseX, mouseY, a);
        //   java: 			}
        // TODO(fabric2seer): `AbstractContainerWidget.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in AbstractContainerWidget and its supertypes)
        // rust: self.extract_scrollbar(graphics, mouse_x, mouse_y);
        graphics.disable_scissor();
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&self, mut x: i32) {
        self.base.set_x(x);
        // TODO(fabric2seer): `listLayout` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: listLayout.setX(x);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&self, mut y: i32) {
        self.base.set_y(y);
        // TODO(fabric2seer): `listLayout` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: listLayout.setY(y);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput narrationElementOutput)
    pub fn update_widget_narration(&self, narration_element_output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        // TODO(fabric2seer): `filteredWidgets` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: return filteredWidgets;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, scrollRate, setScrollAmount, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class SoundWidget extends StringWidget
pub struct SoundWidget {
    /// java: extends net.minecraft.client.gui.components.StringWidget
    pub base: AbstractWidget,
    /// java: SoundEvent sound;
    pub sound: SoundEvent,
    /// java: @SuppressWarnings("unused") Component name;
    pub name: MutableComponent,
}

impl std::ops::Deref for SoundWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SoundWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SoundWidget {
    /// java: SoundWidget(Component name, SoundEvent sound)
    pub fn new(mut name: MutableComponent, mut sound: SoundEvent) -> Self {
        let __init_name = name.clone();
        let __init_sound = sound.clone();
        // TODO(fabric2seer): unknown name `font`
        //   java: super(name, font);
        let mut this: Self = todo!("constructor");
        this.active = true;
        this
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return self.base.get_height().wrapping_add(10);
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `StringWidget.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in StringWidget and its supertypes)
        // rust: self.base.extract_widget_render_state(graphics, mouse_x, mouse_y, a);
        // TODO(fabric2seer): the condition: `selectedSound` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: selectedSound == sound
        if todo!("fabric2seer: the condition (TODO above)") {
            graphics.fill_additive(&mcrust::net::minecraft::client::renderer::render_pipelines::gui(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), 0x3000FF00);
        } else if self.is_hovered {
            graphics.fill_additive(&mcrust::net::minecraft::client::renderer::render_pipelines::gui(), self.get_x(), self.get_y(), self.get_right(), self.get_bottom(), 0x20FFFFFF);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(&self, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `selectedSound` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: selectedSound = sound;
        // TODO(fabric2seer): the condition: `doneButton` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
        //   java: doneButton != null
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `doneButton` is a field of the enclosing instance (SoundSelectionPopup.this) - an inner class has no outer reference in Rust
            //   java: doneButton.active = true;
        }
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: minecraft.getSoundManager().stop();
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: minecraft.getSoundManager().play(SimpleSoundInstance.forUI(sound, 1.0f));
        // TODO(fabric2seer): `StringWidget.onClick` has no Rust method in the port (looked for `on_click` in StringWidget and its supertypes)
        // rust: self.base.on_click(click, doubled);
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, getHeight, onClick override(s) of StringWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
