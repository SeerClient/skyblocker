//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.CatacombsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::skills_page;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::croesus_widget::CroesusWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::daily_runs_widget::DailyRunsWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::dungeon_level_bar_widget::DungeonLevelBarWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::dungeon_runs_widget::DungeonRunsWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::dungeon_secrets_widget::DungeonSecretsWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::floor_runs_widget::FloorRunsWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int VERTICAL_SPACING = 2;
pub const VERTICAL_SPACING: i32 = 2;
/// java: private static final int HORIZONTAL_SPACING = VERTICAL_SPACING * 2;
pub static HORIZONTAL_SPACING: LazyLock<i32> = LazyLock::new(|| VERTICAL_SPACING.wrapping_mul(2));

/// java: public final class CatacombsPage implements ProfileViewerPage<LoadingInformation>
#[derive(Clone)]
pub struct CatacombsPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
}

impl CatacombsPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::new(),
        };
        this
    }

}

impl ProfileViewerPageAny for CatacombsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::CATACOMBS.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Catacombs");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        CatacombsPage::load_dyn
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.clone();
    }

    /// java: @Override public boolean centred()
    fn centred(&self) -> bool {
        return true;
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for CatacombsPage {
    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut page_layout: LinearLayout = LinearLayout::horizontal().spacing(*HORIZONTAL_SPACING);
        let mut levels_layout: LinearLayout = LinearLayout::vertical().spacing(VERTICAL_SPACING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_catacombs(skills_page::LEVEL_BAR_WIDTH, info));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class_average(skills_page::LEVEL_BAR_WIDTH, info));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class(skills_page::LEVEL_BAR_WIDTH, info, DungeonClass::Healer));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class(skills_page::LEVEL_BAR_WIDTH, info, DungeonClass::Mage));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class(skills_page::LEVEL_BAR_WIDTH, info, DungeonClass::Berserk));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class(skills_page::LEVEL_BAR_WIDTH, info, DungeonClass::Archer));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: levels_layout.add_child(DungeonLevelBarWidget::create_class(skills_page::LEVEL_BAR_WIDTH, info, DungeonClass::Tank));
        page_layout.add_child_with(LayoutElement::Grid(levels_layout.clone().wrapped), page_layout.new_cell_settings().align_horizontally_center());
        let mut stats_layout: LinearLayout = LinearLayout::vertical().spacing(VERTICAL_SPACING);
        let mut row1: LinearLayout = LinearLayout::horizontal().spacing(*HORIZONTAL_SPACING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: row1.add_child(DungeonSecretsWidget::new(skills_page::LEVEL_BAR_WIDTH, info.clone()));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: row1.add_child(CroesusWidget::new(skills_page::LEVEL_BAR_WIDTH, info.clone()));
        stats_layout.add_child(LayoutElement::Grid(row1.clone().wrapped));
        let mut row2: LinearLayout = LinearLayout::horizontal().spacing(*HORIZONTAL_SPACING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: row2.add_child(DungeonRunsWidget::new(skills_page::LEVEL_BAR_WIDTH, info.clone()));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: row2.add_child(DailyRunsWidget::new(skills_page::LEVEL_BAR_WIDTH, info.clone()));
        stats_layout.add_child(LayoutElement::Grid(row2.clone().wrapped));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: stats_layout.add_child(FloorRunsWidget::new(((skills_page::LEVEL_BAR_WIDTH).wrapping_mul(2)).wrapping_add(*HORIZONTAL_SPACING), 94, info.clone()));
        page_layout.add_child_with(LayoutElement::Grid(stats_layout.clone().wrapped), page_layout.new_cell_settings().align_horizontally_center());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|__a0| { self.widgets.push(__a0.clone()); });
        return LayoutElement::Grid(page_layout.wrapped);
    }

}

impl CatacombsPage {
    /// CatacombsPage.load as the dispatch of ProfileViewerPage.load
    pub fn load_dyn(__this: &compat::Shared<dyn ProfileViewerPageAny>, info: &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        // java: return CompletableFuture.completedFuture(info).thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        let __h: compat::Shared<CatacombsPage> = compat::handle_as::<CatacombsPage, _>(__this);
        return compat::LocalFuture::completed_future(info.clone()).then_apply_async(move |__a0| compat::read(&__h, |__t| __t.build_widgets(&__a0)));
    }
}
