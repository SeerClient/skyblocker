//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.SkillTreeNodeWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int SIZE = GuiRenderer.DEFAULT_ITEM_SIZE + 2;
pub static SIZE: LazyLock<i32> = LazyLock::new(|| (mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE).wrapping_add(2));

/// java: public final class SkillTreeNodeWidget extends AbstractWidget
#[derive(Clone)]
pub struct SkillTreeNodeWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final List<Component> tooltip;
    pub tooltip: Vec<MutableComponent>,
    /// java: private final Identifier tooltipStyle;
    pub tooltip_style: Identifier,
}

impl std::ops::Deref for SkillTreeNodeWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkillTreeNodeWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkillTreeNodeWidget {
    /// java: public SkillTreeNodeWidget(ItemStack icon, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        let __init_icon = icon.clone();
        let __init_tooltip = tooltip.clone();
        let __init_tooltip_style = tooltip_style.clone().unwrap();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, *SIZE, *SIZE, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            icon: __init_icon,
            tooltip: __init_tooltip,
            tooltip_style: __init_tooltip_style,
        });
        { let __v = false; compat::write(&__this, |__t| __t.active = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(1);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(1);
        graphics.item(&compat::read(__this, |__t| __t.icon.clone()), x, y);
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame_styled` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo, style)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame_styled` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 5
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame_styled(&mut compat::font(), compat::read(__this, |__t| __t.tooltip.clone()), mouse_x, mouse_y, compat::read(__this, |__t| __t.tooltip_style.clone()));
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// SkillTreeNodeWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SkillTreeNodeWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// SkillTreeNodeWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SkillTreeNodeWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| SkillTreeNodeWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| SkillTreeNodeWidget::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, shouldTakeFocusAfterInteraction, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
