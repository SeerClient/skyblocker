//! Ported from de.hysky.skyblocker.utils.render.gui.CyclingTextureWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::enum_utils::EnumUtils;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final WidgetSprites BUTTON = new WidgetSprites(Identifier.withDefaultNamespace("widget/button"), Identifier.withDefaultNamespace("widget/button_disabled"), Identifier.withDefaultNamespa
pub static BUTTON: compat::MainThread<WidgetSprites> = compat::MainThread::new(|| WidgetSprites::with_disabled(Identifier::with_default_namespace("widget/button"), Identifier::with_default_namespace("widget/button_disabled"), Identifier::with_default_namespace("widget/button_highlighted")));

/// java: public class CyclingTextureWidget<T extends Enum<T> & Supplier<Identifier>> extends AbstractWidget
#[derive(Clone)]
pub struct CyclingTextureWidget<T> {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private Function<T, Component> textSupplier = t -> Component.nullToEmpty(t.name());
    pub text_supplier: Arc<dyn Fn(T) -> MutableComponent>,
    /// java: private Function<T, Tooltip> tooltipSupplier = t -> Tooltip.create(Component.translationArg(textSupplier.apply(t)));
    pub tooltip_supplier: Option<Arc<dyn Fn(T) -> Tooltip>>,
    /// java: private Consumer<T> onCycle = _ -> {};
    pub on_cycle: Arc<dyn Fn(T)>,
    /// java: private T current;
    pub current: T,
}

impl<T: 'static> std::ops::Deref for CyclingTextureWidget<T> {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for CyclingTextureWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// java: the `Supplier<Identifier>` half of `T extends Enum<T> & Supplier<Identifier>` - the value's texture
pub trait IdentifierSupplier {
    /// java: `Identifier get()`
    fn get(&self) -> Identifier;
}

impl<T: Clone + crate::de::hysky::skyblocker::utils::enum_utils::JavaEnum + IdentifierSupplier + 'static> CyclingTextureWidget<T> {
    /// java: public CyclingTextureWidget(int x, int y, int width, int height, T initial)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut initial: T) -> compat::Shared<Self> {
        let __init_current = initial.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, width, height, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            // java: private Function<T, Component> textSupplier = t -> Component.nullToEmpty(t.name());
            text_supplier: Arc::new(|t: T| component::null_to_empty(Some(t.java_name()))),
            tooltip_supplier: None,
            on_cycle: Arc::new(move |_| {

        }),
            current: __init_current,
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // java: private Function<T, Tooltip> tooltipSupplier = t -> Tooltip.create(Component.translationArg(textSupplier.apply(t)));
        // (the lambda reads `this.textSupplier`; it lives in the widget's own field, so the widget is alive whenever it runs)
        { let __v: Option<Arc<dyn Fn(T) -> Tooltip>> = Some(Arc::new(move |t: T| { let __this_1 = __this_w1.get().upgrade().expect("CyclingTextureWidget: tooltipSupplier outlived its widget"); Tooltip::create_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translation_arg_message(&((compat::read(&__this_1, |__t| __t.text_supplier.clone())))(t.clone())))) })); compat::write(&__this, |__t| __t.tooltip_supplier = __v); }
        let __r0 = compat::read(&__this, |__t| __t.tooltip_supplier.clone());
        { let (__q0,) = (((__r0.unwrap()))(initial.clone()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: public void setCycleListener(Consumer<T> onCycle)
    pub fn set_cycle_listener(__this: &compat::Shared<Self>, on_cycle: Arc<dyn Fn(T)>) {
        { let __v = on_cycle.clone(); compat::write(__this, |__t| __t.on_cycle = __v); }
    }

    /// java: public void setTextSupplier(Function<T, Component> textSupplier)
    pub fn set_text_supplier(__this: &compat::Shared<Self>, text_supplier: Arc<dyn Fn(T) -> MutableComponent>) {
        { let __v = text_supplier.clone(); compat::write(__this, |__t| __t.text_supplier = __v); }
        let __r0 = compat::read(__this, |__t| __t.tooltip_supplier.clone());
        { let (__q0,) = (((__r0.unwrap()))(CyclingTextureWidget::get_current(__this)),); __this.lock().borrow_mut().set_tooltip(__q0) };
    }

    /// java: public void setTooltipSupplier(Function<T, Tooltip> tooltipSupplier)
    pub fn set_tooltip_supplier(__this: &compat::Shared<Self>, tooltip_supplier: Arc<dyn Fn(T) -> Tooltip>) {
        { let __v = Some(tooltip_supplier.clone()); compat::write(__this, |__t| __t.tooltip_supplier = __v); }
        { let (__q0,) = ((tooltip_supplier)(CyclingTextureWidget::get_current(__this)),); __this.lock().borrow_mut().set_tooltip(__q0) };
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        { let __v = EnumUtils::cycle(&compat::read(__this, |__t| __t.current.clone())); compat::write(__this, |__t| __t.current = __v); }
        let __r0 = compat::read(__this, |__t| __t.tooltip_supplier.clone());
        { let (__q0,) = (((__r0.unwrap()))(CyclingTextureWidget::get_current(__this)),); __this.lock().borrow_mut().set_tooltip(__q0) };
        ((compat::read(__this, |__t| __t.on_cycle.clone())))(CyclingTextureWidget::get_current(__this));
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let button = BUTTON.get_mut().get(compat::read(__this, |__t| __t.active), __this.lock().borrow().is_hovered_or_focused());
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), button, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        // java: context.blit(RenderPipelines.GUI_TEXTURED, getCurrent().get(), this.getX(), this.getY(), 0, 0, width, height, width, height);
        let (texture, x, y, width, height) = (CyclingTextureWidget::get_current(__this).get(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &texture, x, y, 0.0f32, 0.0f32, width, height, width, height);
        if __this.lock().borrow().is_hovered() {
            // java: context.requestCursor(CursorTypes.POINTING_HAND);
            compat::batch_w_request_cursor(compat::BatchWCursor::PointingHand);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &mut dyn NarrationElementOutput) {
        builder.add_component(NarratedElementType::Title, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(((compat::read(__this, |__t| __t.text_supplier.clone())))(CyclingTextureWidget::get_current(__this))));
    }

    /// java: public T getCurrent()
    pub fn get_current(__this: &compat::Shared<Self>) -> T {
        return compat::read(__this, |__t| __t.current.clone());
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
