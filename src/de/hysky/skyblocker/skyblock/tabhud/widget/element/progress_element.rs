//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.ProgressElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::sky_block_colors::SkyBlockColors;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int BAR_WIDTH = 100;
pub const BAR_WIDTH: i32 = 100;
/// java: private static final int BAR_HEIGHT = txtRend.lineHeight + 3;
pub static BAR_HEIGHT: LazyLock<i32> = LazyLock::new(|| (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(3));
/// java: private static final int ICO_OFFS = 4;
pub const ICO_OFFS: i32 = 4;
/// java: private static final int COL_BG_BAR = 0xF0101010;
pub const COL_BG_BAR: i32 = 0xF0101010u32 as i32;

/// java: class ProgressElement extends Element
#[derive(Clone)]
pub struct ProgressElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final @Nullable FlexibleItemStack ico;
    pub ico: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final Component desc, bar;
    pub desc: Option<MutableComponent>,
    /// java: private final Component desc, bar;
    pub bar: Option<MutableComponent>,
    /// java: private final float pcnt;
    pub pcnt: f32,
    /// java: private final int color;
    pub color: i32,
    /// java: private final boolean colorIsBright;
    pub color_is_bright: bool,
    /// java: private final int barW;
    pub bar_w: i32,
}

impl std::ops::Deref for ProgressElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProgressElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProgressElement {
    /// java: ProgressElement(@Nullable FlexibleItemStack ico, @Nullable Component description, @Nullable Component bar, float percent, int color)
    pub fn new(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut description: Option<MutableComponent>, mut bar: Option<MutableComponent>, mut percent: f32, mut color: i32) -> compat::Shared<Self> {
        let mut show_icons: bool = SkyblockerConfigManager::get().ui_and_visuals.tab_hud.display_icons;
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                ico: None,
                desc: None,
                bar: None,
                pcnt: 0.0,
                color: 0,
                color_is_bright: false,
                bar_w: 0,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        if description.is_none() || bar.is_none() {
            { let __v = (if show_icons { Some(ico::BARRIER.get_mut().clone()) } else { None }); compat::write(&__this, |__t| __t.ico = __v); }
            { let __v = Some(component::literal("No data").with_style_format(ChatFormatting::Gray)); compat::write(&__this, |__t| __t.desc = __v); }
            { let __v = Some((component::literal("---")).with_style_format(ChatFormatting::Gray)); compat::write(&__this, |__t| __t.bar = __v); }
            { let __v = 100.0f32; compat::write(&__this, |__t| __t.pcnt = __v); }
            { let __v = (0xFF000000u32 as i32 | mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_GRAY.get_value()); compat::write(&__this, |__t| __t.color = __v); }
        } else {
            { let __v = (if show_icons { (if ico.is_none() { Some(ico::BARRIER.get_mut().clone()) } else { ico.clone() }) } else { None }); compat::write(&__this, |__t| __t.ico = __v); }
            { let __v = description.clone(); compat::write(&__this, |__t| __t.desc = __v); }
            { let __v = bar.clone(); compat::write(&__this, |__t| __t.bar = __v); }
            { let __v = percent.clamp(0.0f32, 100.0f32); compat::write(&__this, |__t| __t.pcnt = __v); }
            { let __v = (0xFF000000u32 as i32 | SkyBlockColors::from_vanilla(color)); compat::write(&__this, |__t| __t.color = __v); }
        }
        { let __v = BAR_WIDTH; compat::write(&__this, |__t| __t.bar_w = __v); }
        { let __v = ((if show_icons { ((compat::read(&__this, |__t| __t.ico_dim.clone())))() } else { 0 }).wrapping_add(element::PAD_L)).wrapping_add((compat::read(&__this, |__t| __t.bar_w)).max(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.desc.clone()).unwrap())))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = ((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(element::PAD_S)).wrapping_add(2)).wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add(2); compat::write(&__this, |__t| __t.height = __v); }
        { let __v = ColorUtils::is_bright(compat::read(&__this, |__t| __t.color)); compat::write(&__this, |__t| __t.color_is_bright = __v); }
        __this
    }

    /// java: ProgressElement(@Nullable FlexibleItemStack ico, @Nullable Component description, @Nullable Component bar, float percent)
    pub fn new2(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut description: Option<MutableComponent>, mut bar: Option<MutableComponent>, mut percent: f32) -> compat::Shared<Self> {
        let __this = Self::new(ico.clone(), description.clone(), bar.clone(), percent, ColorUtils::percent_to_color(percent));
        __this
    }

    /// java: ProgressElement(@Nullable FlexibleItemStack ico, @Nullable Component description, float percent, int color)
    pub fn new3(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut description: Option<MutableComponent>, mut percent: f32, mut color: i32) -> compat::Shared<Self> {
        let __this = Self::new(ico.clone(), description.clone(), Some(component::null_to_empty(Some(&compat::string_format("%.2f%%", &[percent.into()])))), percent, color);
        __this
    }

    /// java: ProgressElement(@Nullable FlexibleItemStack ico, @Nullable Component description, float percent)
    pub fn new_3(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut description: Option<MutableComponent>, mut percent: f32) -> compat::Shared<Self> {
        let __this = Self::new3(ico.clone(), description.clone(), percent, ColorUtils::percent_to_color(percent));
        __this
    }

    /// java: ProgressElement()
    pub fn new_0() -> compat::Shared<Self> {
        let __this = Self::new(None, None, None, ((100) as f32) as f32, 0);
        __this
    }

}

impl Element for ProgressElement {
    fn element_base(&self) -> &ElementBase {
        &self.base
    }
    fn element_base_mut(&mut self) -> &mut ElementBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn Element>, &mut GuiGraphicsExtractor, i32, i32) {
        ProgressElement::extract_render_state_dyn
    }

}

impl ProgressElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut component_x: i32 = x.wrapping_add(element::PAD_L);
        if compat::read(__this, |__t| __t.ico.clone()).is_some() {
            ({ let __f = compat::dispatch(__this, |__t| __t.extract_icon()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Element>), graphics, &compat::read(__this, |__t| __t.ico.clone()).unwrap(), x, y.wrapping_add(ICO_OFFS)) });
            component_x = component_x.wrapping_add(((compat::read(__this, |__t| __t.ico_dim.clone())))());
        }
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.desc.clone()).unwrap()), component_x, y, mcrust::net::minecraft::util::common_colors::WHITE, false);
        let mut bar_y: i32 = (y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add(element::PAD_S);
        let mut end_offs_x: i32 = ((((compat::read(__this, |__t| __t.bar_w) as f32) * (compat::read(__this, |__t| __t.pcnt) / 100.0f32))) as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), component_x.wrapping_add(end_offs_x), bar_y, component_x.wrapping_add(compat::read(__this, |__t| __t.bar_w)), bar_y.wrapping_add(*BAR_HEIGHT), COL_BG_BAR);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), component_x, bar_y, component_x.wrapping_add(end_offs_x), bar_y.wrapping_add(*BAR_HEIGHT), compat::read(__this, |__t| __t.color));
        let mut text_width: i32 = element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.bar.clone()).unwrap()));
        let mut text_dark: bool = (end_offs_x >= text_width.wrapping_add(4) && compat::read(__this, |__t| __t.color_is_bright));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.bar.clone()).unwrap()), component_x.wrapping_add(3), bar_y.wrapping_add(2), (if text_dark { mcrust::net::minecraft::util::common_colors::BLACK } else { mcrust::net::minecraft::util::common_colors::WHITE }), !text_dark);
    }

    /// ProgressElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
