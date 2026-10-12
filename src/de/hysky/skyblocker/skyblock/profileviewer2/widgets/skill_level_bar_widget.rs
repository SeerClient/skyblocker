//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.SkillLevelBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidgetBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final int NORMAL_LEVEL_DIVISOR = 10;
pub const NORMAL_LEVEL_DIVISOR: i32 = 10;
/// java: private static final int COSMETIC_LEVEL_DIVISOR = 5;
pub const COSMETIC_LEVEL_DIVISOR: i32 = 5;

/// java: public final class SkillLevelBarWidget extends LevelBarWidget
#[derive(Clone)]
pub struct SkillLevelBarWidget {
    /// java: extends LevelBarWidget
    pub base: LevelBarWidgetBase,
}

impl std::ops::Deref for SkillLevelBarWidget {
    type Target = LevelBarWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkillLevelBarWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkillLevelBarWidget {
    /// java: private SkillLevelBarWidget(int width, ItemStack icon, Component label, double barFillPercentage, int barFillColour, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut width: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut label: MutableComponent, mut bar_fill_percentage: f64, mut bar_fill_colour: i32, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        let (_, __this) = LevelBarWidgetBase::new_in(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), tooltip_style.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn LevelBarWidget>), __h)
        });
        __this
    }

    /// java: public static LevelBarWidget create(int width, Skill skill, LoadingInformation info)
    pub fn create(mut width: i32, mut skill: Skill, info: &LoadingInformation) -> compat::Shared<dyn LevelBarWidget> {
        let mut level_info: LevelInfo = info.member().lock().borrow().player_data.get_skill_level(skill, info);
        let mut name: String = skill.get_friendly_name();
        let mut leaderboard_id: String = name.to_lowercase();
        let mut is_cosmetic: bool = (skill == Skill::Runecrafting || skill == Skill::Social);
        let mut label: MutableComponent = component::literal(&format!("{} {}", name, level_info.level()));
        let mut bar_fill_percentage: f64 = LevelBarWidgetBase::get_bar_fill_percentage(&level_info);
        let mut bar_fill_colour: i32 = LevelBarWidgetBase::get_bar_fill_colour(&level_info);
        let mut tooltip: Vec<MutableComponent> = LevelBarWidgetBase::build_tooltip(&label, &level_info, info.get_leaderboard_position(&leaderboard_id));
        let mut tooltip_style: Identifier = LevelBarWidgetBase::get_tooltip_style(&level_info, (if is_cosmetic { COSMETIC_LEVEL_DIVISOR } else { NORMAL_LEVEL_DIVISOR })).unwrap();
        return (SkillLevelBarWidget::new(width, FlexibleItemStack::get_stack_or_throw(&skill.get_icon()), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), Some(tooltip_style.clone())) as compat::Shared<dyn LevelBarWidget>);
    }

}

impl LevelBarWidget for SkillLevelBarWidget {
    fn level_bar_widget_base(&self) -> &LevelBarWidgetBase {
        &self.base
    }
    fn level_bar_widget_base_mut(&mut self) -> &mut LevelBarWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// SkillLevelBarWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SkillLevelBarWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// SkillLevelBarWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SkillLevelBarWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), output) }))
    }

}
