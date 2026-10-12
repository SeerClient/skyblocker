//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.ElementBasedWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollection;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::sky_block_colors::SkyBlockColors;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::options;
use mcrust::net::minecraft::client::options::Options;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Font txtRend = Minecraft.getInstance().font;
pub static TXT_REND: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());
/// java: private static final List<Element> ERROR_ELEMENTS = List.of(new PlainTextElement(Component.literal("An error occurred! Please check logs.").withColor(0xFFFF0000)));
pub static ERROR_ELEMENTS: compat::MainThread<Vec<compat::Shared<dyn Element>>> = compat::MainThread::new(|| vec![(PlainTextElement::new(Some((component::literal("An error occurred! Please check logs.")).with_color(0xFFFF0000u32 as i32))) as compat::Shared<dyn Element>)]);
/// java: public static final int BORDER_SZE_N = txtRend.lineHeight + 2;
pub static BORDER_SZE_N: LazyLock<i32> = LazyLock::new(|| (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(2));
/// java: public static final int BORDER_SZE_S = 4;
pub const BORDER_SZE_S: i32 = 4;
/// java: public static final int BORDER_SZE_W = 4;
pub const BORDER_SZE_W: i32 = 4;
/// java: public static final int BORDER_SZE_E = 4;
pub const BORDER_SZE_E: i32 = 4;

/// java: public abstract class ElementBasedWidget extends HudWidget implements ElementCollector
#[derive(Clone)]
pub struct ElementBasedWidgetBase {
    /// java: extends HudWidget
    pub base: HudWidgetBase,
    /// java: private @Nullable String lastError = null;
    pub last_error: Option<String>,
    /// java: private final ArrayList<Element> elements = new ArrayList<>();
    pub elements: Vec<compat::Shared<dyn Element>>,
    /// java: private List<Element> configElements;
    pub config_elements: Vec<compat::Shared<dyn Element>>,
    /// java: private final int color;
    pub color: i32,
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private boolean lastRenderedConfig = false;
    pub last_rendered_config: bool,
    /// java: protected boolean drawBorder = true;
    pub draw_border: bool,
    /// java: protected boolean drawTitle = true;
    pub draw_title: bool,
    /// java: protected float backgroundOpacity = 0.75f;
    pub background_opacity: f32,
    /// java: protected boolean roundedCorners = true;
    pub rounded_corners: bool,
}

impl std::ops::Deref for ElementBasedWidgetBase {
    type Target = HudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ElementBasedWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ElementBasedWidgetBase {
    /// java: public ElementBasedWidget(Component title, @Nullable Integer colorValue, Information information) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, mut color_value: i32, mut information: Information, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ElementBasedWidget>, __R)) -> (compat::Shared<dyn ElementBasedWidget>, __R) {
        let __init_title = title.clone();
        let __init_color = (0xFF000000u32 as i32 | (if false /* java: color_value == null: never null here */ { 0 } else { SkyBlockColors::from_vanilla(color_value) }));
        let __init_config_elements = vec![(PlainTextElement::new(Some(title.plain_copy())) as compat::Shared<dyn Element>)];
        let (_, (__this, __r)) = HudWidgetBase::new_in(information.clone(), move |__base| {
            let this = Self {
                base: __base,
                last_error: None,
                elements: Vec::new(),
                config_elements: __init_config_elements,
                color: __init_color,
                title: __init_title,
                last_rendered_config: false,
                draw_border: true,
                draw_title: true,
                background_opacity: 0.75f32,
                rounded_corners: true,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn HudWidget>), (__h, __r))
        });
        ({ let __f = compat::dispatch(&__this, |__t| __t.pack()); __f(&__this, &compat::read(&__this, |__t| __t.element_based_widget_base().elements.clone())) });
        (__this, __r)
    }

}

/// java: the virtual methods of ElementBasedWidget (implemented by ElementBasedWidgetBase and every subclass)
pub trait ElementBasedWidget: HudWidget + ElementCollector {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase;
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase;

    /// java: public final boolean isEmpty() (dispatch: the body for this class)
    fn is_empty(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        <dyn ElementBasedWidget>::is_empty_body
    }

    /// java: public final void update() (dispatch: the body for this class)
    fn update(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn ElementBasedWidget>::update_body
    }

    /// java: protected final void updateConfig() (dispatch: the body for this class)
    fn update_config(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn ElementBasedWidget>::update_config_body
    }

    /// java: public abstract void updateContent(); (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>);

    /// java: protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn ElementBasedWidget>::update_config_content_body
    }

    /// java: public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        <dyn ElementBasedWidget>::should_update_before_rendering_body
    }

    /// java: private void extractInternal(GuiGraphicsExtractor context, Collection<Element> elements, boolean config) (dispatch: the body for this class)
    fn extract_internal(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &mut GuiGraphicsExtractor, &Vec<compat::Shared<dyn Element>>, bool) {
        <dyn ElementBasedWidget>::extract_internal_body
    }

    /// java: private int getBorderSizeNorth() (dispatch: the body for this class)
    fn get_border_size_north(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> i32 {
        <dyn ElementBasedWidget>::get_border_size_north_body
    }

    /// java: private void pack(Collection<Element> elements) (dispatch: the body for this class)
    fn pack(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &Vec<compat::Shared<dyn Element>>) {
        <dyn ElementBasedWidget>::pack_body
    }

    /// java: private void extractHorizontalLine(GuiGraphicsExtractor graphics, int xpos, int ypos, int width) (dispatch: the body for this class)
    fn extract_horizontal_line(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &mut GuiGraphicsExtractor, i32, i32, i32) {
        <dyn ElementBasedWidget>::extract_horizontal_line_body
    }

    /// java: private void extractVerticalLine(GuiGraphicsExtractor graphics, int xpos, int ypos, int height) (dispatch: the body for this class)
    fn extract_vertical_line(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &mut GuiGraphicsExtractor, i32, i32, i32) {
        <dyn ElementBasedWidget>::extract_vertical_line_body
    }

}

impl dyn ElementBasedWidget {
    /// java: public final boolean isEmpty()
    pub fn is_empty_body(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        return __this.lock().borrow().element_based_widget_base().elements.is_empty();
    }

    /// java: public final void update()
    pub fn update_body(__this: &compat::Shared<dyn ElementBasedWidget>) {
        __this.lock().borrow_mut().element_based_widget_base_mut().elements.clear();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ({ let __f = compat::dispatch(__this, |__t| __t.update_content()); __f(__this) });
        }));
        if let Err(e) = __result {
            if false /* java: compat::panic_message(&e) == null: never null here */ || !(Some(&*(compat::panic_message(&e))) == compat::read(__this, |__t| __t.element_based_widget_base().last_error.clone()).as_deref()) {
                { let __v = Some(compat::panic_message(&e)); compat::write(__this, |__t| __t.element_based_widget_base_mut().last_error = __v); }
                // java: LOGGER.error("Failed to update contents of {}", this, e); - `this` prints as Object.toString,
                // the class and `hashCode()` (`getInternalID().hashCode()`)
                let __id = { let __f = compat::dispatch(__this, |__t| __t.get_internal_id()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) };
                logger().error(&format!("Failed to update contents of {}@{:x} {}", __id, compat::string_hash_code(&__id), compat::panic_message(&e)));
            }
            __this.lock().borrow_mut().element_based_widget_base_mut().elements.clear();
            let __h0 = ERROR_ELEMENTS.get_mut().iter().cloned();
            __this.lock().borrow_mut().element_based_widget_base_mut().elements.extend(__h0);
        }
        if !(compat::read(__this, |__t| __t.element_based_widget_base().last_rendered_config)) {
            ({ let __f = compat::dispatch(__this, |__t| __t.pack()); __f(__this, &compat::read(__this, |__t| __t.element_based_widget_base().elements.clone())) });
        }
    }

    /// java: protected final void updateConfig()
    pub fn update_config_body(__this: &compat::Shared<dyn ElementBasedWidget>) {
        let mut collector: compat::Shared<ElementCollection> = ElementCollection::new();
        ({ let __f = compat::dispatch(__this, |__t| __t.update_config_content()); __f(__this, &compat::view(&collector, |a| a as compat::Shared<dyn ElementCollector>)) });
        if !(ElementCollection::get_elements(&collector).is_empty()) {
            { let __v = ElementCollection::get_elements(&collector); compat::write(__this, |__t| __t.element_based_widget_base_mut().config_elements = __v); }
        }
        for element in compat::read(__this, |__t| __t.element_based_widget_base().config_elements.clone()) {
            ({ let __p = compat::read(__this, |__t| __t.hud_widget_base().as_parent()); let __f = compat::dispatch(&element, |__t| __t.set_parent()); __f(&element, __p) });
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.pack()); __f(__this, &compat::read(__this, |__t| __t.element_based_widget_base().config_elements.clone())) });
    }

    /// java: @Override public void updateConfigPreview()
    pub fn update_config_preview_body(__this: &compat::Shared<dyn ElementBasedWidget>) {
        <dyn HudWidget>::update_config_preview_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>));
        ({ let __f = compat::dispatch(__this, |__t| __t.update_config()); __f(__this) });
    }

    /// ElementBasedWidget.updateConfigPreview as the dispatch of HudWidget.updateConfigPreview for the concrete class `__C`
    pub fn update_config_preview_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>))
    }

    /// java: protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_body(__this: &compat::Shared<dyn ElementBasedWidget>, collector: &compat::Shared<dyn ElementCollector>) {
        ({ let __f = compat::dispatch(__this, |__t| __t.update()); __f(__this) });
        for __a0 in compat::read(__this, |__t| __t.element_based_widget_base().elements.clone()) {
            { let (__a0,) = (__a0.clone(),); collector.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// java: public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_body(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        return false;
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn ElementBasedWidget>, context: &mut GuiGraphicsExtractor, mut delta: f32) {
        if { let __f = compat::dispatch(__this, |__t| __t.should_update_before_rendering()); __f(__this) } {
            ({ let __f = compat::dispatch(__this, |__t| __t.update()); __f(__this) });
        }
        if compat::read(__this, |__t| __t.element_based_widget_base().last_rendered_config) {
            { let __v = false; compat::write(__this, |__t| __t.element_based_widget_base_mut().last_rendered_config = __v); }
            ({ let __f = compat::dispatch(__this, |__t| __t.pack()); __f(__this, &compat::read(__this, |__t| __t.element_based_widget_base().elements.clone())) });
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_internal()); __f(__this, context, &compat::read(__this, |__t| __t.element_based_widget_base().elements.clone()), false) });
    }

    /// ElementBasedWidget.extractWidgetRenderState as the dispatch of HudWidget.extractWidgetRenderState for the concrete class `__C`
    pub fn extract_widget_render_state_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), __p0, __p1)
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_for_config_body(__this: &compat::Shared<dyn ElementBasedWidget>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        if !(compat::read(__this, |__t| __t.element_based_widget_base().last_rendered_config)) {
            { let __v = true; compat::write(__this, |__t| __t.element_based_widget_base_mut().last_rendered_config = __v); }
            ({ let __f = compat::dispatch(__this, |__t| __t.pack()); __f(__this, &compat::read(__this, |__t| __t.element_based_widget_base().config_elements.clone())) });
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_internal()); __f(__this, graphics, &compat::read(__this, |__t| __t.element_based_widget_base().config_elements.clone()), true) });
    }

    /// ElementBasedWidget.extractWidgetRenderStateForConfig as the dispatch of HudWidget.extractWidgetRenderStateForConfig for the concrete class `__C`
    pub fn extract_widget_render_state_for_config_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), __p0, __p1)
    }

    /// java: private void extractInternal(GuiGraphicsExtractor context, Collection<Element> elements, boolean config)
    pub fn extract_internal_body(__this: &compat::Shared<dyn ElementBasedWidget>, context: &mut GuiGraphicsExtractor, elements: &Vec<compat::Shared<dyn Element>>, mut config: bool) {
        if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.enable_hud_background && compat::read(__this, |__t| __t.element_based_widget_base().background_opacity) > (0 as f32) {
            // java: Options options = Minecraft.getInstance().options;
            let options = &compat::minecraft().options;
            let mut opacity: f32 = (if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style.is_minimal() { (compat::read(__this, |__t| __t.element_based_widget_base().background_opacity) / (2 as f32)) } else { compat::read(__this, |__t| __t.element_based_widget_base().background_opacity) });
            // java: int textBackgroundColor = options.getBackgroundColor(ARGB.black(opacity));
            let mut text_background_color: i32 = options::background_color_from_int(&options.borrow(), argb::black_f32(opacity));
            if compat::read(__this, |__t| __t.element_based_widget_base().rounded_corners) {
                context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 1, 0, (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(1), compat::read(__this, |__t| __t.element_based_widget_base().h), text_background_color);
                context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 0, 1, 1, (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_sub(1), text_background_color);
                context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(1), 1, compat::read(__this, |__t| __t.element_based_widget_base().w), (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_sub(1), text_background_color);
            } else {
                context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 0, 0, compat::read(__this, |__t| __t.element_based_widget_base().w), compat::read(__this, |__t| __t.element_based_widget_base().h), text_background_color);
            }
        }
        let mut str_height_half: i32 = (if compat::read(__this, |__t| __t.element_based_widget_base().draw_title) { (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_div(2) } else { 0 });
        let mut str_width: i32 = TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.element_based_widget_base().title.clone())));
        let mut str_area_width: i32 = str_width.wrapping_add(4);
        if compat::read(__this, |__t| __t.element_based_widget_base().draw_title) {
            if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) {
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.element_based_widget_base().title.clone())), 8, 2, compat::read(__this, |__t| __t.element_based_widget_base().color), false);
            } else {
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.element_based_widget_base().title.clone())), ((compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(str_width)).wrapping_div(2), 2, compat::read(__this, |__t| __t.element_based_widget_base().color), false);
            }
        }
        if !(SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style.is_minimal()) && compat::read(__this, |__t| __t.element_based_widget_base().draw_border) {
            if compat::read(__this, |__t| __t.element_based_widget_base().draw_title) {
                ({ let __f = compat::dispatch(__this, |__t| __t.extract_horizontal_line()); __f(__this, context, 2, 1i32.wrapping_add(str_height_half), 4) });
                ({ let __f = compat::dispatch(__this, |__t| __t.extract_horizontal_line()); __f(__this, context, (2i32.wrapping_add(str_area_width)).wrapping_add(4), 1i32.wrapping_add(str_height_half), (((compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(4)).wrapping_sub(4)).wrapping_sub(str_area_width)) });
            } else {
                ({ let __f = compat::dispatch(__this, |__t| __t.extract_horizontal_line()); __f(__this, context, 2, 1i32.wrapping_add(str_height_half), (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(4)) });
            }
            ({ let __f = compat::dispatch(__this, |__t| __t.extract_horizontal_line()); __f(__this, context, 2, (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_sub(2), (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(4)) });
            let mut ypos: i32 = (2i32.wrapping_add(str_height_half)).wrapping_add(if compat::read(__this, |__t| __t.element_based_widget_base().rounded_corners) { 0 } else { -1 });
            let mut height: i32 = (((compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_sub(4)).wrapping_sub(str_height_half)).wrapping_add(if compat::read(__this, |__t| __t.element_based_widget_base().rounded_corners) { 0 } else { 2 });
            ({ let __f = compat::dispatch(__this, |__t| __t.extract_vertical_line()); __f(__this, context, 1, ypos, height) });
            ({ let __f = compat::dispatch(__this, |__t| __t.extract_vertical_line()); __f(__this, context, (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_sub(2), ypos, height) });
        }
        let mut y_offs: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_border_size_north()); __f(__this) });
        let mut x_off: i32 = (if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) { BORDER_SZE_W } else { 1 });
        for c in elements.iter() {
            ({ let __f = compat::dispatch(c, |__t| __t.extract_render_state()); __f(c, context, x_off, y_offs) });
            y_offs = y_offs.wrapping_add(({ let __f = compat::dispatch(c, |__t| __t.get_height()); __f(c) }).wrapping_add(element::PAD_L));
        }
    }

    /// java: private int getBorderSizeNorth()
    pub fn get_border_size_north_body(__this: &compat::Shared<dyn ElementBasedWidget>) -> i32 {
        return (if compat::read(__this, |__t| __t.element_based_widget_base().draw_title) { *BORDER_SZE_N } else { 2 });
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_body(__this: &compat::Shared<dyn ElementBasedWidget>, collector: &mut OptionWidgetCollector) {
        <dyn HudWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.widget.element.drawBorder"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.element_based_widget_base_mut().draw_border = __v); } } }), compat::read(__this, |__t| __t.element_based_widget_base().draw_border));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.widget.element.showTitle"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.element_based_widget_base_mut().draw_title = __v); } } }), compat::read(__this, |__t| __t.element_based_widget_base().draw_title));
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.widget.element.roundedCorners"), Arc::new(move |b| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_3), |__t| __t.element_based_widget_base_mut().rounded_corners = __v); } } }), compat::read(__this, |__t| __t.element_based_widget_base().rounded_corners));
        // java: collector.slider(Component.translatable("skyblocker.config.hud.widget.element.backgroundOpacity"), d -> backgroundOpacity = (float) d, backgroundOpacity, 0, 1);
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.slider(&component::translatable("skyblocker.config.hud.widget.element.backgroundOpacity"), Arc::new(move |d: f64| { if let Some(__this_4) = __this_w4.get().upgrade() { compat::write(&__this_4, |__t| __t.element_based_widget_base_mut().background_opacity = d as f32); } }), compat::read(__this, |__t| __t.element_based_widget_base().background_opacity) as f64, 0.0, 1.0);
    }

    /// ElementBasedWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets for the concrete class `__C`
    pub fn get_option_widgets_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_body(__this: &compat::Shared<dyn ElementBasedWidget>, input: &JsonValueInput) {
        <dyn HudWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), input);
        { let __v = input.read_boolean_or("draw_border", true); compat::write(__this, |__t| __t.element_based_widget_base_mut().draw_border = __v); }
        { let __v = input.read_boolean_or("draw_title", true); compat::write(__this, |__t| __t.element_based_widget_base_mut().draw_title = __v); }
        { let __v = input.read_boolean_or("rounded_corners", true); compat::write(__this, |__t| __t.element_based_widget_base_mut().rounded_corners = __v); }
        { let __v = input.read_float_or("background_opacity", 0.75f32); compat::write(__this, |__t| __t.element_based_widget_base_mut().background_opacity = __v); }
    }

    /// ElementBasedWidget.load as the dispatch of HudWidget.load for the concrete class `__C`
    pub fn load_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_body(__this: &compat::Shared<dyn ElementBasedWidget>, output: &mut JsonValueOutput) {
        <dyn HudWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), output);
        output.write_bool("draw_border", compat::read(__this, |__t| __t.element_based_widget_base().draw_border));
        output.write_bool("draw_title", compat::read(__this, |__t| __t.element_based_widget_base().draw_title));
        output.write_bool("rounded_corners", compat::read(__this, |__t| __t.element_based_widget_base().rounded_corners));
        output.write_number("background_opacity", &mcrust::java::lang::number::Number::Float(compat::read(__this, |__t| __t.element_based_widget_base().background_opacity)));
    }

    /// ElementBasedWidget.save as the dispatch of HudWidget.save for the concrete class `__C`
    pub fn save_from<__C: ElementBasedWidget + 'static>(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), __p0)
    }

    /// java: private void pack(Collection<Element> elements)
    pub fn pack_body(__this: &compat::Shared<dyn ElementBasedWidget>, elements: &Vec<compat::Shared<dyn Element>>) {
        { let __v = 0; compat::write(__this, |__t| __t.element_based_widget_base_mut().h = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.element_based_widget_base_mut().w = __v); }
        for c in elements.iter() {
            { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_add(({ let __f = compat::dispatch(c, |__t| __t.get_height()); __f(c) }).wrapping_add(element::PAD_L)); compat::write(__this, |__t| __t.element_based_widget_base_mut().h = __v); }
            { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().w)).max(({ let __f = compat::dispatch(c, |__t| __t.get_width()); __f(c) }).wrapping_add(element::PAD_S)); compat::write(__this, |__t| __t.element_based_widget_base_mut().w = __v); }
        }
        if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) {
            { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_sub((element::PAD_L).wrapping_div(2)); compat::write(__this, |__t| __t.element_based_widget_base_mut().h = __v); }
        }
        { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().h)).wrapping_add(((({ let __f = compat::dispatch(__this, |__t| __t.get_border_size_north()); __f(__this) }).wrapping_add(BORDER_SZE_S)).wrapping_sub(2)).wrapping_add(if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) { 0 } else { -4 })); compat::write(__this, |__t| __t.element_based_widget_base_mut().h = __v); }
        { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().w)).wrapping_add((BORDER_SZE_E.wrapping_add(BORDER_SZE_W)).wrapping_add(if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) { 0 } else { -6 })); compat::write(__this, |__t| __t.element_based_widget_base_mut().w = __v); }
        { let __v = (compat::read(__this, |__t| __t.element_based_widget_base().w)).max((((((BORDER_SZE_W.wrapping_add(BORDER_SZE_E)).wrapping_add(if compat::read(__this, |__t| __t.element_based_widget_base().draw_border) { 0 } else { -6 })).wrapping_add(TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.element_based_widget_base().title.clone()))))).wrapping_add(4)).wrapping_add(4)).wrapping_add(1)); compat::write(__this, |__t| __t.element_based_widget_base_mut().w = __v); }
    }

    /// java: private void extractHorizontalLine(GuiGraphicsExtractor graphics, int xpos, int ypos, int width)
    pub fn extract_horizontal_line_body(__this: &compat::Shared<dyn ElementBasedWidget>, graphics: &mut GuiGraphicsExtractor, mut xpos: i32, mut ypos: i32, mut width: i32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), xpos, ypos, xpos.wrapping_add(width), ypos.wrapping_add(1), compat::read(__this, |__t| __t.element_based_widget_base().color));
    }

    /// java: private void extractVerticalLine(GuiGraphicsExtractor graphics, int xpos, int ypos, int height)
    pub fn extract_vertical_line_body(__this: &compat::Shared<dyn ElementBasedWidget>, graphics: &mut GuiGraphicsExtractor, mut xpos: i32, mut ypos: i32, mut height: i32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), xpos, ypos, xpos.wrapping_add(1), ypos.wrapping_add(height), compat::read(__this, |__t| __t.element_based_widget_base().color));
    }

}

// ElementBasedWidget.updateConfigPreview implements HudWidget.updateConfigPreview: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.extractWidgetRenderState implements HudWidget.extractWidgetRenderState: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.extractWidgetRenderStateForConfig implements HudWidget.extractWidgetRenderStateForConfig: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.getOptionWidgets implements HudWidget.getOptionWidgets: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.load implements HudWidget.load: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.save implements HudWidget.save: it is emitted in each concrete subclass's `impl HudWidget` (Rust has no inherited impls)
// ElementBasedWidget.addElement implements ElementCollector.addElement: it is emitted in each concrete subclass's `impl ElementCollector` (Rust has no inherited impls)
