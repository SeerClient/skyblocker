//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.SkillsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget;
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
pub struct SkillsPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>(); (held in a `Screen`, the port's holder
    /// of game widgets: see `ProfileViewerPageAny::widgets_mut`)
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl SkillsPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        this
    }

}

impl ProfileViewerPageAny for SkillsPage {
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

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        SkillsPage::load_dyn
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.children().to_vec();
    }

    fn widgets_mut(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.widgets
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for SkillsPage {
    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut page_layout: LinearLayout = LinearLayout::horizontal();
        page_layout.add_child(layout_widget(compat::WidgetAdapter::widget(RulerWidget::new())));
        let mut left_section_layout: LinearLayout = LinearLayout::vertical().spacing(SPACING);
        left_section_layout.add_child(layout_widget(compat::WidgetAdapter::widget(PlayerWidget::new(info.main_member()))));
        left_section_layout.add_child(layout_widget(compat::WidgetAdapter::widget(SkillsInfoBoxWidget::new(player_widget::WIDTH, 71, info.clone()))));
        page_layout.add_child(LayoutElement::Grid(left_section_layout.clone().wrapped));
        page_layout.add_child(SpacerElement::width(3).as_layout_element());
        let mut skills_area_layout: GridLayout = GridLayout::new_default(); skills_area_layout.row_spacing(SPACING.wrapping_mul(2)); skills_area_layout.column_spacing(SPACING.wrapping_mul(4));
        let mut skills_area_row_helper: RowHelper = skills_area_layout.create_row_helper(2);
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkyBlockLevelBarWidget, _>(&SkyBlockLevelBarWidget::create(LEVEL_BAR_WIDTH, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Combat, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Alchemy, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Mining, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Taming, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Farming, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Hunting, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Foraging, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Carpentry, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Fishing, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Runecrafting, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Enchanting, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<SkillLevelBarWidget, _>(&SkillLevelBarWidget::create(LEVEL_BAR_WIDTH, Skill::Social, info))))).unwrap();
        skills_area_row_helper.add_child(&mut skills_area_layout, layout_widget(compat::WidgetAdapter::widget(DungeonLevelBarWidget::create_catacombs(LEVEL_BAR_WIDTH, info)))).unwrap();
        page_layout.add_child_with(LayoutElement::Grid(skills_area_layout.clone()), page_layout.new_cell_settings().align_vertically_middle());
        // java: layout.visitWidgets(widgets::add); - every widget the layout holds was collected as it was put in it
        // (`layout_widget`); `load` hands them to the page
        return LayoutElement::Grid(page_layout.wrapped);
    }

}

impl SkillsPage {
    /// SkillsPage.load as the dispatch of ProfileViewerPage.load
    pub fn load_dyn(__this: &compat::Shared<dyn ProfileViewerPageAny>, info: &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        // java: return CompletableFuture.completedFuture(info).thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        let __h: compat::Shared<SkillsPage> = compat::handle_as::<SkillsPage, _>(__this);
        return compat::LocalFuture::completed_future(info.clone()).then_apply_async(move |__a0| {
            let (__layout, __widgets) = crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::collect_widgets(|| compat::read(&__h, |__t| __t.build_widgets(&__a0)));
            compat::write(&__h, |__t| __t.widgets = __widgets);
            __layout
        });
    }
}
