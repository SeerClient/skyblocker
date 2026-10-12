//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.PlayerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::player_face_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::core::client_asset::Texture as _;

/// java: private static final int SKIN_ICO_DIM = 8;
pub const SKIN_ICO_DIM: i32 = 8;

/// java: public class PlayerElement extends Element
#[derive(Clone)]
pub struct PlayerElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final Component name;
    pub name: Option<MutableComponent>,
    /// java: private final Identifier tex;
    pub tex: Option<Identifier>,
    /// java: private final int iconDim;
    pub icon_dim: i32,
    /// java: private final int textYOffset;
    pub text_y_offset: i32,
}

impl std::ops::Deref for PlayerElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlayerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlayerElement {
    /// java: public PlayerElement(PlayerInfo ple)
    pub fn new(mut ple: PlayerInfo) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `PlayerInfo` (Java holds a reference to the live object); the port's `PlayerInfo` is not a value to copy and the port lends the mod no handle to it
        //   java: this(ple, null, false);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public PlayerElement(PlayerInfo ple, @Nullable Component name, boolean large)
    pub fn new_3(mut ple: PlayerInfo, mut name: Option<MutableComponent>, mut large: bool) -> compat::Shared<Self> {
        let __init_name = Some((if name.is_some() { name.clone().unwrap() } else { (if ple.get_tab_list_display_name().is_some() { ple.get_tab_list_display_name() } else { Some(component::literal("No data").with_style_format(ChatFormatting::Gray)) }).unwrap() }));
        let __init_tex = Some(ple.get_skin().body().texture_path().clone());
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                name: __init_name,
                tex: __init_tex,
                icon_dim: 0,
                text_y_offset: 0,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        { let __v = (if large { ((compat::read(&__this, |__t| __t.ico_dim.clone())))() } else { SKIN_ICO_DIM }); compat::write(&__this, |__t| __t.icon_dim = __v); }
        { let __v = (if large { (if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.compact_widgets { 2 } else { 4 }) } else { 0 }); compat::write(&__this, |__t| __t.text_y_offset = __v); }
        { let __v = ((compat::read(&__this, |__t| __t.icon_dim)).wrapping_add(element::PAD_S)).wrapping_add(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.name.clone()).unwrap()))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = (compat::read(&__this, |__t| __t.icon_dim)).max(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT); compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

}

impl Element for PlayerElement {
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
        PlayerElement::extract_render_state_dyn
    }

}

impl PlayerElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        player_face_extractor::extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.tex.clone()).unwrap(), x, y, compat::read(__this, |__t| __t.icon_dim), true, false, -1);
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.name.clone()).unwrap()), (x.wrapping_add(compat::read(__this, |__t| __t.icon_dim))).wrapping_add(element::PAD_S), y.wrapping_add(compat::read(__this, |__t| __t.text_y_offset)), mcrust::net::minecraft::util::common_colors::WHITE, false);
    }

    /// PlayerElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
