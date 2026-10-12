//! Ported from de.hysky.skyblocker.skyblock.auction.widgets.RarityWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler::SlotClickHandler;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier LEFT_TEXTURE = SkyblockerMod.id("auctions_gui/rarity_widget/left");
pub static LEFT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("auctions_gui/rarity_widget/left"));
/// java: private static final Identifier LEFT_HOVER_TEXTURE = SkyblockerMod.id("auctions_gui/rarity_widget/left_hover");
pub static LEFT_HOVER_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("auctions_gui/rarity_widget/left_hover"));
/// java: private static final Identifier RIGHT_TEXTURE = SkyblockerMod.id("auctions_gui/rarity_widget/right");
pub static RIGHT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("auctions_gui/rarity_widget/right"));
/// java: private static final Identifier RIGHT_HOVER_TEXTURE = SkyblockerMod.id("auctions_gui/rarity_widget/right_hover");
pub static RIGHT_HOVER_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("auctions_gui/rarity_widget/right_hover"));
/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("auctions_gui/rarity_widget/background");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("auctions_gui/rarity_widget/background"));

/// java: public class RarityWidget extends AbstractWidget
#[derive(Clone)]
pub struct RarityWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final SlotClickHandler onClick;
    pub on_click: compat::Shared<dyn SlotClickHandler>,
    /// java: private int slotId = -1;
    pub slot_id: i32,
    /// java: private List<Component> tooltip = List.of();
    pub tooltip: Vec<MutableComponent>,
    /// java: private String current = "?";
    pub current: String,
    /// java: private int color = 0xFFEAEAEA;
    pub color: i32,
}

impl std::ops::Deref for RarityWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RarityWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RarityWidget {
    /// java: public RarityWidget(int x, int y, SlotClickHandler onClick)
    pub fn new(mut x: i32, mut y: i32, mut on_click: compat::Shared<dyn SlotClickHandler>) -> compat::Shared<Self> {
        let __init_on_click = on_click.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 48, 11, (&(component::literal("rarity selector thing, hi mom")).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            on_click: __init_on_click,
            slot_id: -1,
            tooltip: vec![],
            current: "?".to_string(),
            color: 0xFFEAEAEAu32 as i32,
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((__this.lock().borrow().get_x()) as f32, (__this.lock().borrow().get_y()) as f32);
        let mut on_left_arrow: bool = RarityWidget::is_on_left_arrow(__this, (mouse_x) as f64);
        let mut on_right_arrow: bool = RarityWidget::is_on_right_arrow(__this, (mouse_x) as f64);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), 6, 0, 36, 11);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if on_left_arrow { LEFT_HOVER_TEXTURE.get_mut().clone() } else { LEFT_TEXTURE.get_mut().clone() }), 0, 0, 6, 11);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if on_right_arrow { RIGHT_HOVER_TEXTURE.get_mut().clone() } else { RIGHT_TEXTURE.get_mut().clone() }), 42, 0, 6, 11);
        let mut text_renderer: Font = compat::font();
        let mut text_width: i32 = text_renderer.width(&compat::read(__this, |__t| __t.current.clone()));
        if text_width > 34 {
            let mut scale: f32 = (34.0f32 / (text_width as f32));
            let __pose_2 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((0.0f32) as f32, (5.5f32) as f32);
            gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.current.clone()), ((((24 as f32) / scale)) as i32), (-mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_div(2), compat::read(__this, |__t| __t.color));
            gui_graphics_extractor::pop_pose(__pose_2);
        } else {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.current.clone()), 24, 2, compat::read(__this, |__t| __t.color));
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        if !on_left_arrow && !on_right_arrow && __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut text_renderer, compat::read(__this, |__t| __t.tooltip.clone()), mouse_x, mouse_y);
        }
    }

    /// java: private boolean isOnRightArrow(double mouseX)
    pub fn is_on_right_arrow(__this: &compat::Shared<Self>, mut mouse_x: f64) -> bool {
        return (__this.lock().borrow().is_hovered() && (mouse_x - (__this.lock().borrow().get_x() as f64)) > (40 as f64));
    }

    /// java: private boolean isOnLeftArrow(double mouseX)
    pub fn is_on_left_arrow(__this: &compat::Shared<Self>, mut mouse_x: f64) -> bool {
        return (__this.lock().borrow().is_hovered() && (mouse_x - (__this.lock().borrow().get_x() as f64)) < (7 as f64));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: public void setSlotId(int slotId)
    pub fn set_slot_id(__this: &compat::Shared<Self>, mut slot_id: i32) {
        { let __v = slot_id; compat::write(__this, |__t| __t.slot_id = __v); }
    }

    /// java: public void setText(List<Component> tooltip, String current)
    pub fn set_text(__this: &compat::Shared<Self>, mut tooltip: Vec<MutableComponent>, mut current: String) {
        { let __v = tooltip.clone(); compat::write(__this, |__t| __t.tooltip = __v); }
        { let __v = current.clone(); compat::write(__this, |__t| __t.current = __v); }
        { let __v = argb::opaque(SkyblockItemRarity::contains_name(&current.to_uppercase()).map(|r| r.fields().color).unwrap_or(mcrust::net::minecraft::util::common_colors::GRAY)); compat::write(__this, |__t| __t.color = __v); }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if compat::read(__this, |__t| __t.slot_id) == -1 {
            return;
        }
        if RarityWidget::is_on_left_arrow(__this, click.x()) {
            ({ let __r = &compat::read(__this, |__t| __t.on_click.clone()); let __f = compat::dispatch(__r, |__t| __t.click()); __f(__r, compat::read(__this, |__t| __t.slot_id), 1) });
        } else if RarityWidget::is_on_right_arrow(__this, click.x()) {
            ({ let __r = &compat::read(__this, |__t| __t.on_click.clone()); let __f = compat::dispatch(__r, |__t| __t.click()); __f(__r, compat::read(__this, |__t| __t.slot_id), 0) });
        }
    }

}

/// RarityWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for RarityWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// RarityWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<RarityWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| RarityWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| RarityWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| RarityWidget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
