//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.SkyBlockLevelBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public final class SkyBlockLevelBarWidget extends LevelBarWidget
#[derive(Clone)]
pub struct SkyBlockLevelBarWidget {
    /// java: extends LevelBarWidget
    pub base: LevelBarWidgetBase,
}

impl std::ops::Deref for SkyBlockLevelBarWidget {
    type Target = LevelBarWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyBlockLevelBarWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyBlockLevelBarWidget {
    /// java: private SkyBlockLevelBarWidget(int width, ItemStack icon, Component label, double barFillPercentage, int barFillColour, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut width: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut label: MutableComponent, mut bar_fill_percentage: f64, mut bar_fill_colour: i32, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        let (_, __this) = LevelBarWidgetBase::new_in(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), tooltip_style.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn LevelBarWidget>), __h)
        });
        __this
    }

    /// java: public static LevelBarWidget create(int width, LoadingInformation info)
    pub fn create(mut width: i32, info: &LoadingInformation) -> compat::Shared<dyn LevelBarWidget> {
        let mut level_info: LevelInfo = LevelCalculator::get_skyblock_level(info.member().lock().borrow().levelling.experience);
        let mut level: i32 = level_info.level();
        let mut label: MutableComponent = component::empty().append_str("SB Level ").append(component::literal("[").with_style_format(ChatFormatting::DarkGray)).append(component::literal(&level.to_string()).with_style_format(SkyBlockLevelBarWidget::get_level_colour(level))).append(component::literal("]").with_style_format(ChatFormatting::DarkGray));
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(ico::EXPERIENCE_BOTTLE.get_mut());
        let mut bar_fill_percentage: f64 = LevelBarWidgetBase::get_bar_fill_percentage(&level_info);
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: int barFillColour = Color.CYAN.getRGB();
        let mut bar_fill_colour: i32 = todo!("fabric2seer: the initialiser of `barFillColour` (TODO above)");
        let mut tooltip: Vec<MutableComponent> = LevelBarWidgetBase::build_tooltip(&label, &level_info, info.get_leaderboard_position("skyblockxp"));
        let mut tooltip_style: Identifier = LevelBarWidgetBase::get_tooltip_style(&level_info, 100).unwrap();
        return (SkyBlockLevelBarWidget::new(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), Some(tooltip_style.clone())) as compat::Shared<dyn LevelBarWidget>);
    }

    /// java: private static ChatFormatting getLevelColour(int level)
    pub fn get_level_colour(mut level: i32) -> ChatFormatting {
        if level >= 480 {
            return ChatFormatting::DarkRed;
        } else if level >= 440 {
            return ChatFormatting::Red;
        } else if level >= 400 {
            return ChatFormatting::Gold;
        } else if level >= 360 {
            return ChatFormatting::DarkPurple;
        } else if level >= 320 {
            return ChatFormatting::LightPurple;
        } else if level >= 280 {
            return ChatFormatting::Blue;
        } else if level >= 240 {
            return ChatFormatting::DarkAqua;
        } else if level >= 200 {
            return ChatFormatting::Aqua;
        } else if level >= 160 {
            return ChatFormatting::DarkGreen;
        } else if level >= 140 {
            return ChatFormatting::Green;
        } else if level >= 100 {
            return ChatFormatting::Yellow;
        } else {
            return ChatFormatting::White;
        }
    }

}

impl LevelBarWidget for SkyBlockLevelBarWidget {
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

/// SkyBlockLevelBarWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SkyBlockLevelBarWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// SkyBlockLevelBarWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SkyBlockLevelBarWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), output) }))
    }

}
