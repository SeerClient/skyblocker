//! Ported from de.hysky.skyblocker.utils.render.gui.SoundSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Component YES_TEXT = CommonComponents.GUI_YES.copy().withStyle(ChatFormatting.GREEN);
pub static YES_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| compat::component_of(mcrust::net::minecraft::network::chat::common_components::gui_yes()).with_style_format(ChatFormatting::Green));
/// java: private static final Component NO_TEXT = CommonComponents.GUI_NO.copy().withStyle(ChatFormatting.RED);
pub static NO_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| compat::component_of(mcrust::net::minecraft::network::chat::common_components::gui_no()).with_style_format(ChatFormatting::Red));

/// java: public class SoundSelectionPopup extends AbstractPopupScreen
pub struct SoundSelectionPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final List<AbstractWidget> filteredWidgets = new ObjectArrayList<>(); - the game's widgets (over
    /// `SoundWidget` objects), held in a screen of the popup's own: the list container draws them and hands them
    /// their clicks (`ListContainer.children()`)
    pub filtered_widgets: Screen,
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
    pub widgets_container: Option<compat::Shared<ListContainer>>,
    /// the search field (`init`'s local, read by the advanced toggle)
    pub search_field: Option<compat::WidgetRef>,
    /// the list container's index in the screen's widgets
    pub container_index: Option<usize>,
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
    ///
    /// (open it with `game::set_screen_keeping_parent`: the screen it is opened over is its background screen, which
    /// the base keeps a stand-in for - see `AbstractPopupScreenBase::new_in`)
    pub fn new(background_screen: Screen, on_done: Arc<dyn Fn(Option<SoundEvent>)>) -> compat::Shared<Self> {
        // java: super(Component.literal("Select Sound"), backgroundScreen);
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Select Sound"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                filtered_widgets: Screen::new(String::new()),
                on_done,
                selected_sound: None,
                advanced: false,
                grid_widget: GridLayout::new_default(),
                list_layout: LinearLayout::new(0, 0, mcrust::net::minecraft::client::gui::layouts::linear_layout::Orientation::Vertical),
                done_button: None,
                widgets_container: None,
                search_field: None,
                container_index: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        __this
    }

    /// java: private @Nullable Component getSoundName(SoundEvent sound)
    ///
    /// (`sound` is the registry's own `SoundEvent` object - `getKey` finds an element by identity)
    pub fn get_sound_name(__this: &compat::Shared<Self>, sound: &mcrust::com::mojang::serialization::Obj) -> Option<MutableComponent> {
        let key: String = mcrust::net::minecraft::core::registries::built_in_registries::sound_event().get_key(sound).unwrap().to_short_language_key();
        //first check for translation
        let translation: MutableComponent = component::translatable_with_fallback(&format!("subtitles.{}", key), Some("null"));
        if !(translation.get_string() == "null") {
            return Some(translation);
        }
        //convert note block sounds
        if key.contains("note_block") {
            let split: Vec<String> = compat::regex::split(&key, "\\.");
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
        // java: filteredWidgets.clear();
        compat::write(__this, |__t| __t.filtered_widgets.truncate_widgets(0));
        // java: for (SoundEvent soundEvent : BuiltInRegistries.SOUND_EVENT)
        for sound_event in mcrust::net::minecraft::core::registries::built_in_registries::sound_event().iterator() {
            let translation: Option<MutableComponent> = SoundSelectionPopup::get_sound_name(__this, &sound_event);
            //filter sounds
            if let Some(translation) = translation && translation.get_string().to_lowercase().contains(&input.to_lowercase()) {
                let Some(sound) = mcrust::com::mojang::serialization::cast::<SoundEvent>(&sound_event).cloned() else { continue; };
                // java: AbstractWidget widget = new SoundWidget(translation, soundEvent); filteredWidgets.add(widget);
                let widget = SoundWidget::new(std::sync::Arc::downgrade(__this), translation, sound);
                let __w = compat::WidgetAdapter::widget(widget);
                compat::write(__this, |__t| { __t.filtered_widgets.add_renderable_widget(__w); });
            }
        }
        SoundSelectionPopup::recreate_list(__this);
    }

    /// java: private void recreateList()
    pub fn recreate_list(__this: &compat::Shared<Self>) {
        let list_x: i32 = __this.lock().borrow().list_layout.wrapped.x;
        let list_y: i32 = __this.lock().borrow().list_layout.wrapped.y;
        // java: listLayout = new LinearLayout(0, 0, LinearLayout.Orientation.VERTICAL); filteredWidgets.forEach(listLayout::addChild);
        // listLayout.arrangeElements(); listLayout.setPosition(listX, listY);
        compat::write(__this, |__t| {
            let mut layout = LinearLayout::new(list_x, list_y, mcrust::net::minecraft::client::gui::layouts::linear_layout::Orientation::Vertical);
            for __i in 0..__t.filtered_widgets.children().len() {
                let __w = __t.filtered_widgets.widget(__i);
                layout.add_child(LayoutElement::widget(__i, __w.get_width(), __w.get_height()));
            }
            layout.arrange_elements();
            __t.list_layout = layout;
        });
        SoundSelectionPopup::place_list(__this);
        // java: if (widgetsContainer != null) widgetsContainer.refreshScrollAmount();
        if let Some(container) = compat::read(__this, |__t| __t.widgets_container.clone()) {
            let content_height = compat::read(__this, |__t| __t.list_layout.get_height());
            compat::write(&container, |__t| __t.area.refresh_scroll_amount(content_height));
        }
    }

    /// The filtered widgets at the places `listLayout` gives them (its x and y as of now).
    pub fn place_list(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| {
            __t.list_layout.arrange_elements();
            let mut placed: Vec<(usize, i32, i32)> = Vec::new();
            LayoutElement::Grid(__t.list_layout.wrapped.clone()).visit_widgets(&mut placed);
            for (__i, __x, __y) in placed {
                if __i < __t.filtered_widgets.children().len() {
                    __t.filtered_widgets.widget_mut(__i).set_position(__x, __y);
                }
            }
        });
    }

    /// `doneButton.active = true` (a `SoundWidget`'s click, which runs between the game's calls on the popup)
    fn activate_done_button(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            if let Some(__r) = __s.done_button.as_mut() {
                __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.active = true);
            }
        });
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
    /// `addRenderableWidget(adder.addChild(widget[, columns]))`
    fn add_to_grid(__this: &compat::Shared<Self>, adder: &mut RowHelper, widget: AbstractWidget, columns: i32) -> usize {
        let (__w, __h) = (widget.get_width(), widget.get_height());
        let __i = compat::write(__this, |__t| __t.add_renderable_widget(widget));
        compat::write(__this, |__t| { let _ = adder.add_child_width(&mut __t.grid_widget, LayoutElement::widget(__i, __w, __h), columns); });
        __i
    }

    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.grid_widget = GridLayout::new_default());
        let mut adder: RowHelper = __this.lock().borrow().grid_widget.create_row_helper(2);
        // java: EditBox searchField = new EditBox(Minecraft.getInstance().font, 200, 20, Component.empty());
        let mut search_field = compat::edit_box(0, 0, 200, 20, component::empty());
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        compat::with_edit_box(&mut search_field, |__e| {
            // java: searchField.setHint(Component.translatable("gui.recipebook.search_hint").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY));
            __e.set_hint_component(FormattedText::from(component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray)));
            // java: searchField.setResponder(this::filterSounds);
            __e.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return; }; SoundSelectionPopup::filter_sounds(&__this_1, __a0); })));
        });
        // java: addRenderableWidget(adder.addChild(searchField));
        let mut __search_ref = compat::WidgetRef::value(search_field);
        let (__si, __sw, __sh) = compat::write(__this, |__t| { let __s = &mut __t.base; let __i = __s.__widgets.add(&mut __s.base, &mut __search_ref, true); let __w = __s.base.widget(__i); (__i, __w.get_width(), __w.get_height()) });
        compat::write(__this, |__t| { __t.search_field = Some(__search_ref); let _ = adder.add_child(&mut __t.grid_widget, LayoutElement::widget(__si, __sw, __sh)); });
        // java: CycleButton<Boolean> toggleAdvanced = CycleButton.booleanBuilder(YES_TEXT, NO_TEXT, advanced)
        //   .withTooltip(_ -> Tooltip.create(..advanced.@Tooltip)).create(0, 0, 100, 20, ..advanced.., (_, _) -> { advanced = !advanced; filterSounds(searchField.getValue()); });
        let __builder = cycle_button::boolean_builder(YES_TEXT.get().get_string(), NO_TEXT.get().get_string(), compat::read(__this, |__t| __t.advanced))
            .with_tooltip(cycle_button::TooltipSupplier::Constant(FormattedText::from(component::translatable("skyblocker.utils.render.gui.soundSelectionPopup.advanced.@Tooltip"))));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let toggle_advanced = compat::cycle_button_widget(0, 0, 100, 20, &component::translatable("skyblocker.utils.render.gui.soundSelectionPopup.advanced"), __builder, move |_: usize| {
            let Some(__this_2) = __this_w2.get().upgrade() else { return; };
            compat::write(&__this_2, |__t| __t.advanced = !__t.advanced);
            let Some(mut __r) = compat::read(&__this_2, |__t| __t.search_field.clone()) else { return; };
            let value = compat::write(&__this_2, |__t| { let __s = &mut __t.base; __s.__widgets.with(&mut __s.base, &mut __r, |__w| compat::edit_box_value(__w)) });
            SoundSelectionPopup::filter_sounds(&__this_2, &value);
        });
        // java: addRenderableWidget(adder.addChild(toggleAdvanced));
        SoundSelectionPopup::add_to_grid(__this, &mut adder, toggle_advanced, 1);
        // java: widgetsContainer = new ListContainer(0, 0, 400, this.height / 2);
        let container = ListContainer::new(std::sync::Arc::downgrade(__this), 0, 0, 400, (compat::read(__this, |__t| __t.height)).wrapping_div(2));
        compat::write(__this, |__t| __t.widgets_container = Some(container.clone()));
        // java: addRenderableWidget(adder.addChild(widgetsContainer, 2));
        let __ci = SoundSelectionPopup::add_to_grid(__this, &mut adder, compat::object_widget(container, "List"), 2);
        compat::write(__this, |__t| __t.container_index = Some(__ci));
        // java: addRenderableWidget(adder.addChild(Button.builder(CommonComponents.GUI_CANCEL, _ -> { onClose(); onDone.accept(null); }).build()));
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let cancel = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut Screen, _: usize| {
            let Some(__this_3) = __this_w3.get().upgrade() else { return; };
            ({ let __f = compat::dispatch(&__this_3, |__t| __t.on_close()); __f(&compat::view(&__this_3, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
            (compat::read(&__this_3, |__t| __t.on_done.clone()))(None);
        }))).build();
        SoundSelectionPopup::add_to_grid(__this, &mut adder, cancel, 1);
        // java: doneButton = Button.builder(CommonComponents.GUI_DONE, _ -> { onClose(); onDone.accept(selectedSound); }).build();
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut Screen, _: usize| {
            let Some(__this_4) = __this_w4.get().upgrade() else { return; };
            ({ let __f = compat::dispatch(&__this_4, |__t| __t.on_close()); __f(&compat::view(&__this_4, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
            let (on_done, selected) = compat::read(&__this_4, |__t| (__t.on_done.clone(), __t.selected_sound.clone()));
            on_done(selected);
        }))).build();
        // java: doneButton.active = false;
        done.active = false;
        // java: addRenderableWidget(adder.addChild(doneButton));
        let mut __done_ref = compat::WidgetRef::value(done);
        let (__di, __dw, __dh) = compat::write(__this, |__t| { let __s = &mut __t.base; let __i = __s.__widgets.add(&mut __s.base, &mut __done_ref, true); let __w = __s.base.widget(__i); (__i, __w.get_width(), __w.get_height()) });
        compat::write(__this, |__t| { __t.done_button = Some(__done_ref); let _ = adder.add_child(&mut __t.grid_widget, LayoutElement::widget(__di, __dw, __dh)); });
        // java: gridWidget.arrangeElements(); repositionElements(); filterSounds("");
        compat::write(__this, |__t| __t.grid_widget.arrange_elements());
        ({ let __f = compat::dispatch(__this, |__t| __t.reposition_elements()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        SoundSelectionPopup::filter_sounds(__this, "");
    }

    /// SoundSelectionPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        // java: gridWidget.setPosition((width - gridWidget.getWidth()) / 2, (height - gridWidget.getHeight()) / 2);
        // (the grid's widgets are the screen's: they take the places the grid gives them; the list container moves
        // the list with it - its `setX` / `setY`)
        let placed: Vec<(usize, i32, i32)> = compat::write(__this, |__t| {
            let (width, height) = (__t.width, __t.height);
            __t.grid_widget.x = (width.wrapping_sub(__t.grid_widget.get_width())).wrapping_div(2);
            __t.grid_widget.y = (height.wrapping_sub(__t.grid_widget.get_height())).wrapping_div(2);
            __t.grid_widget.arrange_elements();
            let mut placed: Vec<(usize, i32, i32)> = Vec::new();
            LayoutElement::Grid(__t.grid_widget.clone()).visit_widgets(&mut placed);
            placed
        });
        for (__i, __x, __y) in placed {
            compat::write(__this, |__t| { if __i < __t.children().len() { __t.widget_mut(__i).set_position(__x, __y); } });
        }
        if let Some(container) = compat::read(__this, |__t| __t.widgets_container.clone()) {
            // the container's box as the grid placed it (`ListContainer.setX` / `setY`)
            if let Some(index) = compat::read(__this, |__t| __t.container_index) {
                let (x, y) = compat::read(__this, |__t| (__t.widget(index).get_x(), __t.widget(index).get_y()));
                ListContainer::move_to(&container, x, y);
            }
        }
    }

    /// SoundSelectionPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        let (__x, __y, __w, __h) = compat::read(__this, |__t| (__t.grid_widget.get_x(), __t.grid_widget.get_y(), __t.grid_widget.get_width(), __t.grid_widget.get_height()));
        AbstractPopupScreenBase::extract_popup_background(graphics, __x, __y, __w, __h);
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
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

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
///
/// (an inner class: its children - `filteredWidgets` - and `listLayout` are the enclosing popup's, reached through
/// `outer`; the popup is not borrowed while the game calls this widget)
pub struct ListContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// AbstractScrollArea's scroll state (`AbstractScrollArea.defaultSettings(8)`, `scrollRate()` 15)
    pub area: ScrollArea,
    /// java: SoundSelectionPopup.this
    pub outer: compat::WeakShared<SoundSelectionPopup>,
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
    pub fn new(outer: compat::WeakShared<SoundSelectionPopup>, x: i32, y: i32, width: i32, height: i32) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.literal("List"), AbstractScrollArea.defaultSettings(8));
        let mut settings = ScrollbarSettings::default_settings(8.0);
        settings.scroll_rate = ListContainer::scroll_rate();
        compat::shared(ListContainer {
            base: AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, height, FormattedText::from(component::literal("List")), ScrollbarSettings::default_settings(8.0)),
            area: ScrollArea::new(y, height, settings),
            outer,
        })
    }

    /// java: @Override protected int contentHeight() - `listLayout.getHeight()`
    pub fn content_height(&self) -> i32 {
        self.outer.upgrade().map_or(0, |__o| compat::read(&__o, |__t| __t.list_layout.get_height()))
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate() -> f64 {
        return 15.0;
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&mut self, scroll_y: f64) {
        // java: super.setScrollAmount(scrollY); listLayout.setY(getY() - (int) scrollAmount());
        let content_height = self.content_height();
        self.area.set_scroll_amount(content_height, scroll_y);
        self.move_list();
    }

    /// `listLayout.setX(getX()); listLayout.setY(getY() - (int) scrollAmount())` - the list follows the box
    fn move_list(&self) {
        let Some(outer) = self.outer.upgrade() else { return; };
        let (x, y) = (self.base.get_x(), self.base.get_y() - self.area.scroll_amount() as i32);
        compat::write(&outer, |__t| { __t.list_layout.wrapped.x = x; __t.list_layout.wrapped.y = y; });
        SoundSelectionPopup::place_list(&outer);
    }

    /// `setX(x); setY(y)` from the popup that placed the container's widget
    pub fn move_to(__this: &compat::Shared<Self>, x: i32, y: i32) {
        compat::write(__this, |__t| {
            __t.base.set_x(x);
            __t.base.set_y(y);
            __t.area.y = y;
            __t.move_list();
        });
    }

    /// java: private boolean isVisible(AbstractWidget widget)
    pub fn is_visible(&self, widget: &AbstractWidget) -> bool {
        return (widget.get_bottom() >= self.get_y() && widget.get_y() < self.get_bottom());
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `filteredWidgets`: the popup's widget
    /// screen, lent to `f` (taken out of the popup for the call: a child's click reaches the popup)
    fn with_children<R>(&self, f: impl FnOnce(&mut Screen) -> R) -> Option<R> {
        let outer = self.outer.upgrade()?;
        let mut children = compat::write(&outer, |__t| std::mem::take(&mut __t.filtered_widgets));
        let out = f(&mut children);
        compat::write(&outer, |__t| {
            // a child's click may have refiltered the list meanwhile: the newer list stays
            if __t.filtered_widgets.children().is_empty() {
                __t.filtered_widgets = children;
            }
        });
        Some(out)
    }
}

impl compat::ObjectWidget for ListContainer {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        // java: super.setX(x); listLayout.setX(x);
        self.base.set_x(x);
        self.move_list();
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        // java: super.setY(y); listLayout.setY(y);
        self.base.set_y(y);
        self.area.y = y;
        self.move_list();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        // java: graphics.enableScissor(getX(), getY(), getRight(), getBottom());
        // java: for (AbstractWidget widget : filteredWidgets) if (isVisible(widget)) widget.extractRenderState(graphics, mouseX, mouseY, a);
        // (the scissor is each child's `clip`, which the widget pass pushes around that child's draw)
        let (left, top, right, bottom) = (self.base.get_x(), self.base.get_y(), self.base.get_right(), self.base.get_bottom());
        self.with_children(|children| {
            for __i in 0..children.children().len() {
                let __w = children.widget_mut(__i);
                __w.visible = __w.get_bottom() >= top && __w.get_y() < bottom;
                __w.clip = Some((left, top, right, bottom));
            }
            context.extract_widgets(children, mouse_x, mouse_y);
        });
        // java: extractScrollbar(graphics, mouseX, mouseY); graphics.disableScissor();
        let content_height = self.content_height();
        compat::extract_scrollbar(context, &self.area, content_height, self.base.get_right() - self.area.scrollbar_width());
    }

    /// AbstractContainerWidget.mouseClicked - the scroll bar, then `ContainerEventHandler.mouseClicked` over `children()`
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !self.base.is_active() || !self.base.visible {
            return false;
        }
        let content_height = self.content_height();
        let scroll_bar_x = self.base.get_right() - self.area.scrollbar_width();
        let scrolling = self.area.update_scrolling(content_height, scroll_bar_x, event.button(), event.x(), event.y());
        let taken = self.with_children(|children| compat::children_mouse_clicked(children, event)).unwrap_or(false);
        taken || scrolling
    }

    /// ContainerEventHandler.mouseReleased
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        self.area.on_release();
        self.with_children(|children| compat::children_mouse_released(children, event));
        false
    }

    /// AbstractScrollArea.mouseDragged (the scroll bar) / ContainerEventHandler.mouseDragged
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let content_height = self.content_height();
        if self.area.mouse_dragged(content_height, event.y(), dy) {
            self.move_list();
            return true;
        }
        self.with_children(|children| compat::children_mouse_dragged(children, event, dx, dy));
        false
    }

    /// AbstractScrollArea.mouseScrolled - `setScrollAmount(scrollAmount() - scrollY * scrollRate())`
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, _x: f64, _y: f64, _scroll_x: f64, scroll_y: f64) -> bool {
        if !self.base.visible {
            return false;
        }
        let amount = self.area.scroll_amount() - scroll_y * self.area.scroll_rate();
        self.set_scroll_amount(amount);
        true
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput narrationElementOutput)
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {
    }
}

/// java: private class SoundWidget extends StringWidget
pub struct SoundWidget {
    /// java: extends net.minecraft.client.gui.components.StringWidget
    pub base: AbstractWidget,
    /// java: SoundEvent sound;
    pub sound: SoundEvent,
    /// java: Component name;
    pub name: MutableComponent,
    /// java: SoundSelectionPopup.this
    pub outer: compat::WeakShared<SoundSelectionPopup>,
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
    pub fn new(outer: compat::WeakShared<SoundSelectionPopup>, name: MutableComponent, sound: SoundEvent) -> compat::Shared<Self> {
        // java: super(name, font); - `StringWidget(0, 0, font.width(name), 9, name, font)`; `getHeight()` is
        // overridden to `super.getHeight() + 10`, which is the box the layout and the clicks see
        let text = FormattedText::from(name.clone());
        let width = compat::font().width_of(&text);
        let mut base = AbstractWidget::new(0, 0, width, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT + 10, name.get_string(), WidgetKind::StringWidget, OnPress::Inert);
        base.set_message_text(text);
        // java: this.active = true;
        base.active = true;
        compat::shared(SoundWidget { base, sound, name, outer })
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        // (the widget's box is built with the extra 10)
        self.base.get_height()
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        // java: super.extractWidgetRenderState(graphics, mouseX, mouseY, a); - StringWidget: the message, centred in the height
        let (x, y, right, bottom, height, message, hovered, sound) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_right(), __t.get_bottom(), __t.get_height(), __t.message_text.clone(), __t.is_hovered(), __t.sound.clone()));
        let gl = mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl();
        if let Some(message) = message {
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(gl, &*compat::minecraft(), &message, x, y + (height - mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) / 2, -1, true);
        }
        let selected = compat::read(__this, |__t| __t.outer.upgrade()).is_some_and(|__o| compat::read(&__o, |__t| __t.selected_sound.as_ref() == Some(&sound)));
        if selected {
            graphics.fill(gl, x, y, right, bottom, 0x3000FF00);
        } else if hovered {
            graphics.fill(gl, x, y, right, bottom, 0x20FFFFFF);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return; };
        // java: selectedSound = sound;
        let sound = compat::read(__this, |__t| __t.sound.clone());
        compat::write(&outer, |__t| __t.selected_sound = Some(sound.clone()));
        // java: if (doneButton != null) doneButton.active = true;
        SoundSelectionPopup::activate_done_button(&outer);
        // java: minecraft.getSoundManager().stop(); minecraft.getSoundManager().play(SimpleSoundInstance.forUI(sound, 1.0f));
        mcrust::mcrust::modding::game::stop_all_sounds();
        mcrust::mcrust::modding::game::play_ui_sound(&sound, 1.0);
        // java: super.onClick(click, doubled); - empty
    }

}

/// SoundWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SoundWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// SoundWidget as the game's widget (`filteredWidgets.add(widget)`)
impl ModWidget for compat::WidgetAdapter<SoundWidget> {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| SoundWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// StringWidget.updateWidgetNarration - empty
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| SoundWidget::on_click(__h, event, double_click))
    }
}
