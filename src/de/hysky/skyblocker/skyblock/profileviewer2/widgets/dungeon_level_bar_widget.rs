//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.DungeonLevelBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::dungeons::Dungeons;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::level_bar_widget::LevelBarWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
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

/// java: public final class DungeonLevelBarWidget extends LevelBarWidget
#[derive(Clone)]
pub struct DungeonLevelBarWidget {
    /// java: extends LevelBarWidget
    pub base: LevelBarWidgetBase,
}

impl std::ops::Deref for DungeonLevelBarWidget {
    type Target = LevelBarWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonLevelBarWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonLevelBarWidget {
    /// java: private DungeonLevelBarWidget(int width, ItemStack icon, Component label, double barFillPercentage, int barFillColour, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut width: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut label: MutableComponent, mut bar_fill_percentage: f64, mut bar_fill_colour: i32, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        let (_, __this) = LevelBarWidgetBase::new_in(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), tooltip_style.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn LevelBarWidget>), __h)
        });
        __this
    }

    /// java: public static DungeonLevelBarWidget createCatacombs(int width, LoadingInformation info)
    pub fn create_catacombs(mut width: i32, info: &LoadingInformation) -> compat::Shared<DungeonLevelBarWidget> {
        let mut catacombs_xp: i64 = ((compat::read(&info.member().lock().borrow().dungeons.dungeon_types.catacombs, |__t| __t.experience)) as i64);
        let mut level_info: LevelInfo = LevelCalculator::get_skill_level(catacombs_xp, Skill::Catacombs, info);
        let mut label: MutableComponent = component::literal(&format!("Catacombs {}", level_info.level()));
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(ico::CATACOMBS.get_mut());
        let mut bar_fill_percentage: f64 = LevelBarWidgetBase::get_bar_fill_percentage(&level_info);
        let mut bar_fill_colour: i32 = LevelBarWidgetBase::get_bar_fill_colour(&level_info);
        let mut tooltip: Vec<MutableComponent> = LevelBarWidgetBase::build_tooltip(&label, &level_info, info.get_leaderboard_position("catacombs"));
        let mut tooltip_style: Identifier = LevelBarWidgetBase::get_tooltip_style(&level_info, 10).unwrap();
        return DungeonLevelBarWidget::new(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), Some(tooltip_style.clone()));
    }

    /// java: public static DungeonLevelBarWidget createClass(int width, LoadingInformation info, DungeonClass clazz)
    pub fn create_class(mut width: i32, info: &LoadingInformation, mut clazz: DungeonClass) -> compat::Shared<DungeonLevelBarWidget> {
        let mut dungeons: Dungeons = compat::read(&info.member(), |__t| __t.dungeons.clone());
        let mut level_info: LevelInfo = dungeons.get_class_data(clazz).get_level_info(info);
        let mut label: MutableComponent;
        if dungeons.selected_dungeon_class == clazz.api_name() {
            label = (component::literal(&format!("{} {} ★", clazz.display_name(), level_info.level()))).with_style_format(ChatFormatting::Green);
        } else {
            label = component::literal(&format!("{} {}", clazz.display_name(), level_info.level()));
        }
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&clazz.icon());
        let mut bar_fill_percentage: f64 = LevelBarWidgetBase::get_bar_fill_percentage(&level_info);
        let mut bar_fill_colour: i32 = LevelBarWidgetBase::get_bar_fill_colour(&level_info);
        let mut tooltip: Vec<MutableComponent> = LevelBarWidgetBase::build_tooltip(&label, &level_info, info.get_leaderboard_position(&format!("{}-xp", clazz.api_name())));
        let mut tooltip_style: Identifier = LevelBarWidgetBase::get_tooltip_style(&level_info, 10).unwrap();
        return DungeonLevelBarWidget::new(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), Some(tooltip_style.clone()));
    }

    /// java: public static DungeonLevelBarWidget createClassAverage(int width, LoadingInformation info)
    pub fn create_class_average(mut width: i32, info: &LoadingInformation) -> compat::Shared<DungeonLevelBarWidget> {
        let mut level_info: LevelInfo = info.member().lock().borrow().dungeons.get_class_average(info);
        let mut label: MutableComponent = component::literal(&format!("Class Avg {}", formatters::FLOAT_NUMBERS.get_mut().format(level_info.level_with_progress())));
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ItemRepository::get_item_stack_2("SPIRIT_WING", ico::BARRIER.get_mut()));
        let mut bar_fill_percentage: f64 = LevelBarWidgetBase::get_bar_fill_percentage(&level_info);
        let mut bar_fill_colour: i32 = LevelBarWidgetBase::get_bar_fill_colour(&level_info);
        let mut tooltip: Vec<MutableComponent> = LevelBarWidgetBase::build_tooltip(&label, &level_info, elite_leaderboards::NO_POSITION);
        let mut tooltip_style: Identifier = LevelBarWidgetBase::get_tooltip_style(&level_info, 10).unwrap();
        return DungeonLevelBarWidget::new(width, icon.clone(), label.clone(), bar_fill_percentage, bar_fill_colour, tooltip.clone(), Some(tooltip_style.clone()));
    }

}

impl LevelBarWidget for DungeonLevelBarWidget {
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

/// DungeonLevelBarWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for DungeonLevelBarWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// DungeonLevelBarWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<DungeonLevelBarWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn LevelBarWidget>), output) }))
    }

}
