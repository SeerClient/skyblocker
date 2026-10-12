//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.SkillsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::dungeon_level_bar_widget::DungeonLevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::player_widget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::player_widget::PlayerWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::ruler_widget::RulerWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::skill_level_bar_widget::SkillLevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::skills_info_box_widget::SkillsInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::sky_block_level_bar_widget::SkyBlockLevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int SPACING = 2;
pub const SPACING: i32 = 2;
/// java: protected static final int LEVEL_BAR_WIDTH = 105;
pub const LEVEL_BAR_WIDTH: i32 = 105;

/// java: public final class SkillsPage implements ProfileViewerPage<LoadingInformation>
#[derive(Clone)]
pub struct SkillsPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
}

impl SkillsPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::new(),
        };
        this
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for SkillsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::IRON_SWORD.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Skills");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info)
    fn load(&self, info: &LoadingInformation) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<LayoutElement> {
        return CompletableFuture::completed_future(info.clone()).then_apply_async(|__a0| self.build_widgets(&__a0), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut page_layout: LinearLayout = LinearLayout::horizontal();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: page_layout.add_child(RulerWidget::new());
        let mut left_section_layout: LinearLayout = LinearLayout::vertical().spacing(SPACING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: left_section_layout.add_child(PlayerWidget::new(info.main_member()));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: left_section_layout.add_child(SkillsInfoBoxWidget::new(player_widget::WIDTH, 71, info.clone()));
        page_layout.add_child(LayoutElement::Grid(left_section_layout.clone().wrapped));
        page_layout.add_child(SpacerElement::width(3).as_layout_element());
        let mut skills_area_layout: GridLayout = GridLayout::new_default().row_spacing(SPACING.wrapping_mul(2)).column_spacing(SPACING.wrapping_mul(4));
        let mut skills_area_row_helper: RowHelper = skills_area_layout.create_row_helper(2);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkyBlockLevelBarWidget::create(LEVEL_BAR_WIDTH, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Combat, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Alchemy, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Mining, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Taming, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Farming, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Hunting, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Foraging, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Carpentry, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Fishing, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Runecrafting, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Enchanting, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Social, info)).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: skills_area_row_helper.add_child(&mut DungeonLevelBarWidget::create_catacombs(LEVEL_BAR_WIDTH, info)).unwrap();
        page_layout.add_child_with(LayoutElement::Grid(skills_area_layout.clone()), page_layout.new_cell_settings().align_vertically_middle());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|__a0| { self.widgets.push(__a0.clone()); });
        return LayoutElement::Grid(page_layout.wrapped);
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.clone();
    }

}
