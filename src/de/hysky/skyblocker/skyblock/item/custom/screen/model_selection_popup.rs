//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ModelSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::AbstractSelectionPopup;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::AbstractSelectionPopupBase;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::core::component::data_component_patch;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ModelSelectionPopup extends AbstractSelectionPopup<ModelSelectionPopup.Widget>
pub struct ModelSelectionPopup {
    /// java: extends AbstractSelectionPopup
    pub base: AbstractSelectionPopupBase<compat::Shared<Widget>>,
    /// java: private final List<Widget> widgets = ((ModelManagerAccessor) Minecraft.getInstance().getModelManager()).getBakedItemStackModels().keySet().stream().sorted().map
    pub widgets: Vec<compat::Shared<Widget>>,
}

impl std::ops::Deref for ModelSelectionPopup {
    type Target = AbstractSelectionPopupBase<compat::Shared<Widget>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ModelSelectionPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ModelSelectionPopup {
    /// java: protected ModelSelectionPopup(Screen backgroundScreen, Consumer<@Nullable Identifier> onDone)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<Identifier>)>) -> compat::Shared<Self> {
        //   java: super(Component.literal("Select Model"), backgroundScreen, opt -> onDone.accept(opt.map(Widget::getValue).orElse(null)), 20);
        // (open it with `game::set_screen_keeping_parent`: the screen it is opened over is its background screen, which
        // the base keeps a stand-in for - see `AbstractPopupScreenBase::new_in`)
        let __on_done: Arc<dyn Fn(Option<compat::Shared<Widget>>)> = Arc::new(move |opt: Option<compat::Shared<Widget>>| on_done(opt.map(|__w| Widget::get_value(&__w))));
        let (_, __this) = AbstractSelectionPopupBase::new_in(component::literal("Select Model"), background_screen, __on_done, 20, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                widgets: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractSelectionPopup<compat::Shared<Widget>>>), __h)
        });
        // java: private final List<Widget> widgets = ((ModelManagerAccessor) Minecraft.getInstance().getModelManager())
        //   .getBakedItemStackModels().keySet().stream().sorted().map(Widget::new).toList();
        // (the model manager's baked item models are the published bake's `item_models`; the map is ordered by
        // `Identifier.compareTo` already)
        let outer = std::sync::Arc::downgrade(&__this);
        let widgets: Vec<compat::Shared<Widget>> = mcrust::net::minecraft::client::renderer::game_renderer::baked()
            .map(|__models| __models.item_models.baked_item_stack_models.keys().cloned().collect::<Vec<Identifier>>())
            .unwrap_or_default()
            .into_iter()
            .map(|__id| Widget::new(outer.clone(), __id))
            .collect();
        compat::write(&__this, |__t| __t.widgets = widgets);
        __this
    }

}

impl AbstractSelectionPopup<compat::Shared<crate::de::hysky::skyblocker::skyblock::item::custom::screen::model_selection_popup::Widget>> for ModelSelectionPopup {
    fn abstract_selection_popup_base(&self) -> &AbstractSelectionPopupBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::item::custom::screen::model_selection_popup::Widget>> {
        &self.base
    }
    fn abstract_selection_popup_base_mut(&mut self) -> &mut AbstractSelectionPopupBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::item::custom::screen::model_selection_popup::Widget>> {
        &mut self.base
    }

    /// java: @Override protected Collection<Widget> filterWidgets(String input)
    fn filter_widgets(&self, input: &str) -> Vec<compat::Shared<Widget>> {
        // java: return widgets.stream().filter(w -> w.value.toString().contains(input.replace(' ', '_').toLowerCase(Locale.ENGLISH))).toList();
        let needle = input.replace(' ', "_").to_lowercase();
        return self.widgets.iter().cloned().filter(|w| compat::read(w, |__t| __t.value.to_string()).contains(&needle)).collect::<Vec<_>>();
    }

}

impl AbstractPopupScreen for ModelSelectionPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn AbstractSelectionPopup<compat::Shared<Widget>>>::extract_background_from::<Self>
    }

    /// java: @Override public void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup<compat::Shared<Widget>>>::init_from::<Self>
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractSelectionPopup<compat::Shared<Widget>>>::reposition_elements_from::<Self>
    }

}

/// ModelSelectionPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ModelSelectionPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ModelSelectionPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ModelSelectionPopup> {
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

    /// java: @Override public void init()
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

/// java: protected class Widget extends AbstractWidget
#[derive(Clone)]
pub struct Widget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final Identifier value;
    pub value: Identifier,
    /// java: ModelSelectionPopup.this
    pub outer: compat::WeakShared<ModelSelectionPopup>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for Widget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Widget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Widget {
    /// java: @SuppressWarnings("deprecation") public Widget(Identifier identifier)
    pub fn new(outer: compat::WeakShared<ModelSelectionPopup>, mut identifier: Identifier) -> compat::Shared<Self> {
        let __init_value = identifier.clone();
        let __init_icon = mcrust::net::minecraft::world::item::item_stack::ItemStack::with_patch(/* Items.CAT_SPAWN_EGG.builtInRegistryHolder(): the holder's item */ items::items().get("cat_spawn_egg"), 1, &(data_component_patch::builder().set(mcrust::net::minecraft::core::component::data_components::item_model().clone(), mcrust::com::mojang::serialization::obj(identifier.clone()))).build());
        //   java: super(0, 0, 20, 20, Component.literal(identifier.toString()));
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 20, 20, identifier.to_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText, mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Inert),
            icon: __init_icon,
            value: __init_value,
            outer,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Tooltip::create(compat::component_of(__this.lock().borrow().get_message_component()).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: public Identifier getValue()
    pub fn get_value(__this: &compat::Shared<Self>) -> Identifier {
        return compat::read(__this, |__t| __t.value.clone());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.item(&compat::read(__this, |__t| __t.icon.clone()), (__this.lock().borrow().get_x()).wrapping_add(2), (__this.lock().borrow().get_y()).wrapping_add(2));
        // java: if (selectedItem == this)
        let __outer = compat::read(__this, |__t| __t.outer.upgrade());
        let __selected = __outer.is_some_and(|__o| compat::read(&__o, |__t| __t.base.selected_item.as_ref().is_some_and(|__s| std::sync::Arc::ptr_eq(__s, __this))));
        if __selected {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x3000FF00);
        }
        if __this.lock().borrow().is_hovered() {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()), 0x20FFFFFF);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        let _ = (click, doubled);
        // java: ModelSelectionPopup.this.setSelectedItem(this);
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return; };
        compat::write(&outer, |__t| __t.set_selected_item(__this.clone()));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

}

/// Widget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for Widget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// Widget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<Widget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| Widget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| Widget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| Widget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

impl crate::de::hysky::skyblocker::utils::render::gui::abstract_selection_popup::SelectionWidget for compat::Shared<Widget> {
    fn game_widget(&self) -> AbstractWidget {
        compat::WidgetAdapter::widget(self.clone())
    }
}
