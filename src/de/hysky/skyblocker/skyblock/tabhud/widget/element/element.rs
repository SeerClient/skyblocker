//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.Element by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;

/// java: public static final int PAD_S = 2;
pub const PAD_S: i32 = 2;
/// java: public static final int PAD_L = 2;
pub const PAD_L: i32 = 2;
/// java: static final Font txtRend = Minecraft.getInstance().font;
pub static TXT_REND: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public abstract class Element
#[derive(Clone)]
pub struct ElementBase {
    /// java: public final Supplier<Integer> ICO_DIM = () -> SkyblockerConfigManager.get().uiAndVisuals.tabHud.compactWidgets ? 12 : 16;
    pub ico_dim: Arc<dyn Fn() -> i32>,
    /// java: int width, height;
    pub width: i32,
    /// java: int width, height;
    pub height: i32,
    /// java: private LayoutElement parent = SpacerElement.width(0);
    pub parent: LayoutElement,
}

impl ElementBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn Element>, __R)) -> (compat::Shared<dyn Element>, __R) {
        let this = Self {
            ico_dim: Arc::new(move || (if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.compact_widgets { 12 } else { 16 })),
            width: 0,
            height: 0,
            parent: SpacerElement::width(0).as_layout_element(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of Element (implemented by ElementBase and every subclass)
pub trait Element: compat::DynAny {
    fn element_base(&self) -> &ElementBase;
    fn element_base_mut(&mut self) -> &mut ElementBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void extractRenderState(GuiGraphicsExtractor graphics, int x, int y); (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn Element>, &mut GuiGraphicsExtractor, i32, i32);

    /// java: public void setParent(LayoutElement parent) (dispatch: the body for this class)
    fn set_parent(&self) -> fn(&compat::Shared<dyn Element>, LayoutElement) {
        <dyn Element>::set_parent_body
    }

    /// java: public LayoutElement getParent() (dispatch: the body for this class)
    fn get_parent(&self) -> fn(&compat::Shared<dyn Element>) -> LayoutElement {
        <dyn Element>::get_parent_body
    }

    /// java: public int getWidth() (dispatch: the body for this class)
    fn get_width(&self) -> fn(&compat::Shared<dyn Element>) -> i32 {
        <dyn Element>::get_width_body
    }

    /// java: public int getHeight() (dispatch: the body for this class)
    fn get_height(&self) -> fn(&compat::Shared<dyn Element>) -> i32 {
        <dyn Element>::get_height_body
    }

    /// java: public void extractIcon(GuiGraphicsExtractor graphics, FlexibleItemStack icon, int x, int y) (dispatch: the body for this class)
    fn extract_icon(&self) -> fn(&compat::Shared<dyn Element>, &GuiGraphicsExtractor, &compat::Shared<FlexibleItemStack>, i32, i32) {
        <dyn Element>::extract_icon_body
    }

}

impl dyn Element {
    /// java: public void setParent(LayoutElement parent)
    pub fn set_parent_body(__this: &compat::Shared<dyn Element>, mut parent: LayoutElement) {
        { let __v = parent; compat::write(__this, |__t| __t.element_base_mut().parent = __v); }
    }

    /// java: public LayoutElement getParent()
    pub fn get_parent_body(__this: &compat::Shared<dyn Element>) -> LayoutElement {
        return compat::read(__this, |__t| __t.element_base().parent);
    }

    /// java: public int getWidth()
    pub fn get_width_body(__this: &compat::Shared<dyn Element>) -> i32 {
        return compat::read(__this, |__t| __t.element_base().width);
    }

    /// java: public int getHeight()
    pub fn get_height_body(__this: &compat::Shared<dyn Element>) -> i32 {
        return compat::read(__this, |__t| __t.element_base().height);
    }

    /// java: public void extractIcon(GuiGraphicsExtractor graphics, FlexibleItemStack icon, int x, int y)
    pub fn extract_icon_body(__this: &compat::Shared<dyn Element>, graphics: &GuiGraphicsExtractor, icon: &compat::Shared<FlexibleItemStack>, mut x: i32, mut y: i32) {
        let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = FlexibleItemStack::get_stack(icon);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        if let Some(stack) = &stack {
            // TODO(fabric2seer): Matrix3x2fStack.scale
            //   java: graphics.pose().scale((float) ICO_DIM.get() / 16);
            graphics.item(stack, 0, 0);
        } else {
            // TODO(fabric2seer): Matrix3x2fStack.scale
            //   java: graphics.pose().scale(2f);
            // TODO(fabric2seer): Matrix3x2fStack.scale
            //   java: graphics.pose().scale((float) ICO_DIM.get() / 16);
            gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ItemUtils::get_icon(icon)), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE, false);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

}
