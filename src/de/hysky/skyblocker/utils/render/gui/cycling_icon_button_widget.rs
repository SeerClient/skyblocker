//! Ported from de.hysky.skyblocker.utils.render.gui.CyclingIconButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class CyclingIconButtonWidget<T> extends AbstractButton
pub struct CyclingIconButtonWidget<T> {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: private final Function<T, Icon> valueToIcon;
    pub value_to_icon: Option<Arc<dyn Fn(T) -> Icon>>,
    /// java: private final boolean showText;
    pub show_text: bool,
    /// java: private int index;
    pub index: i32,
    /// java: private final Function<T, Component> valueToText;
    pub value_to_text: Option<Arc<dyn Fn(T) -> MutableComponent>>,
    /// java: private final T[] values;
    pub values: Vec<T>,
    /// java: private final Consumer<T> callback;
    pub callback: Option<Arc<dyn Fn(T)>>,
    /// java: private final Function<T, Tooltip> tooltipFactory;
    pub tooltip_factory: Option<Arc<dyn Fn(T) -> Tooltip>>,
    /// java: private Icon currentIcon;
    pub current_icon: Option<Icon>,
}

impl<T: 'static> std::ops::Deref for CyclingIconButtonWidget<T> {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for CyclingIconButtonWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> CyclingIconButtonWidget<T> {
    /// java: public CyclingIconButtonWidget(int width, int height, T value, T[] values, Function<T, Component> valueToText, Function<T, Icon> valueToIcon, Function<T, Tooltip> tooltipFactory, Consumer<T> callback,
    pub fn new(mut width: i32, mut height: i32, mut value: T, mut values: Vec<T>, value_to_text: Arc<dyn Fn(T) -> MutableComponent>, value_to_icon: Arc<dyn Fn(T) -> Icon>, tooltip_factory: Arc<dyn Fn(T) -> Tooltip>, callback: Arc<dyn Fn(T)>, mut show_text: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): static call ArrayUtils.indexOf(..)
        //   java: this.index = ArrayUtils.indexOf(values, value);
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, width, height, (&(value_to_text)(value.clone()).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            value_to_icon: None,
            show_text: false,
            index: todo!("fabric2seer: the field value (TODO above)"),
            value_to_text: None,
            values: Default::default(),
            callback: None,
            tooltip_factory: None,
            current_icon: None,
        });
        debug_assert!(compat::read(&__this, |__t| __t.index) >= 0);
        { let __v = values.clone(); compat::write(&__this, |__t| __t.values = __v); }
        { let __v = Some(value_to_text.clone()); compat::write(&__this, |__t| __t.value_to_text = __v); }
        { let __v = Some(value_to_icon.clone()); compat::write(&__this, |__t| __t.value_to_icon = __v); }
        { let __v = Some(callback.clone()); compat::write(&__this, |__t| __t.callback = __v); }
        { let __v = Some(tooltip_factory.clone()); compat::write(&__this, |__t| __t.tooltip_factory = __v); }
        { let __v = show_text; compat::write(&__this, |__t| __t.show_text = __v); }
        { let __v = Some((value_to_icon)(value.clone())); compat::write(&__this, |__t| __t.current_icon = __v); }
        { let (__q0,) = ((tooltip_factory)(value.clone()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: public CyclingIconButtonWidget(int width, int height, T value, T[] values, Function<T, Icon> valueToIcon, Function<T, Tooltip> tooltipFactory, Consumer<T> callback)
    pub fn new_7(mut width: i32, mut height: i32, mut value: T, mut values: Vec<T>, value_to_icon: Arc<dyn Fn(T) -> Icon>, tooltip_factory: Arc<dyn Fn(T) -> Tooltip>, callback: Arc<dyn Fn(T)>) -> compat::Shared<Self> {
        let __this = Self::new(width, height, value.clone(), values.clone(), Arc::new(move |o| component::literal(&compat::jstring(&o))), value_to_icon.clone(), tooltip_factory.clone(), callback.clone(), false);
        __this
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        { let __v = ((compat::read(__this, |__t| __t.index)).wrapping_add(1)).wrapping_rem(__this.lock().borrow().values.len() as i32); compat::write(__this, |__t| __t.index = __v); }
        let new_value = &__this.lock().borrow().values[(compat::read(__this, |__t| __t.index)) as usize];
        ((compat::read(__this, |__t| __t.callback.clone()).unwrap()))(new_value.clone());
        let __r0 = compat::read(__this, |__t| __t.value_to_text.clone());
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(((__r0.unwrap()))(new_value.clone())),); __this.lock().borrow_mut().set_message_text(__q0) };
        { let __v = Some(((compat::read(__this, |__t| __t.value_to_icon.clone()).unwrap()))(new_value.clone())); compat::write(__this, |__t| __t.current_icon = __v); }
        let __r0 = compat::read(__this, |__t| __t.tooltip_factory.clone());
        { let (__q0,) = (((__r0.unwrap()))(new_value.clone()),); __this.lock().borrow_mut().set_tooltip(__q0) };
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_contents(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // TODO(fabric2seer): `AbstractButton.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in AbstractButton and its supertypes)
        // rust: __this.lock().borrow().extract_default_sprite(context);
        let mut x: i32 = (if compat::read(__this, |__t| __t.show_text) { (((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(__this.lock().borrow().current_icon.as_ref().unwrap().width())).wrapping_sub(2) } else { ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2))).wrapping_sub((__this.lock().borrow().current_icon.as_ref().unwrap().width()).wrapping_div(2)) });
        let mut y: i32 = ((__this.lock().borrow().get_y()).wrapping_add((__this.lock().borrow().get_height()).wrapping_div(2))).wrapping_sub((__this.lock().borrow().current_icon.as_ref().unwrap().height()).wrapping_div(2));
        gui_graphics_extractor::blit_sprite_identifier_with_alpha(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &__this.lock().borrow().current_icon.as_ref().unwrap().id(), x, y, __this.lock().borrow().current_icon.as_ref().unwrap().width(), __this.lock().borrow().current_icon.as_ref().unwrap().height(), compat::read(__this, |__t| __t.alpha));
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: this.extractDefaultLabel(context.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE));
    }

    /// java: @Override public void extractDefaultLabel(ActiveTextCollector drawer)
    pub fn extract_default_label(__this: &compat::Shared<Self>, drawer: &dyn ActiveTextCollector) {
        if !(compat::read(__this, |__t| __t.show_text)) {
            return;
        }
        let mut x1: i32 = (__this.lock().borrow().get_x()).wrapping_add(2);
        let mut x2: i32 = (((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(__this.lock().borrow().current_icon.as_ref().unwrap().width())).wrapping_sub(4);
        let mut x_center: i32 = (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2));
        // TODO(fabric2seer): the port's `accept_scrolling` ports this Java method with other parameters (center_x, left, right, top, bottom, line_width, time_millis for Java's message, center_x, left, right, top, bottom)
        // TODO(fabric2seer): `accept_scrolling` takes 7 argument(s) in the port (`center_x: i32, left: i32, right: i32, top: i32, bottom: i32, line_width: i32, time_millis: i64`), Java passes 6
        // rust: active_text_collector::accept_scrolling(compat::component_of(__this.lock().borrow().get_message_component()), x_center, x1, x2, __this.lock().borrow().get_y(), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

// NOTE(fabric2seer): extractContents, extractDefaultLabel, onPress, updateWidgetNarration override(s) of AbstractButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public record Icon(Identifier id, int width, int height)
#[derive(Clone, Debug, PartialEq)]
pub struct Icon {
    /// java: Identifier id
    pub id: Identifier,
    /// java: int width
    pub width: i32,
    /// java: int height
    pub height: i32,
}

impl Icon {
    /// java: the canonical record constructor
    pub fn new(mut id: Identifier, mut width: i32, mut height: i32) -> Self {
        Self { id, width, height }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> Identifier {
        self.id.clone()
    }

    /// java: the record accessor `width()`
    pub fn width(&self) -> i32 {
        self.width
    }

    /// java: the record accessor `height()`
    pub fn height(&self) -> i32 {
        self.height
    }

}
