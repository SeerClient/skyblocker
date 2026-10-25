//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.CollectionsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::player_data::PlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::collection_tiers::CollectionTiers;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::collection_tiers::Report;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::button_widget::ButtonWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::collection_item_widget::CollectionItemWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::collection_item_widget::CollectionItemWidgetBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::minion_widget::MinionWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::data::constants::constant_data::ConstantData;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int VERTICAL_GRID_SPACING = 4;
pub const VERTICAL_GRID_SPACING: i32 = 4;
/// java: private static final int HORIZONTAL_GRID_SPACING = 2;
pub const HORIZONTAL_GRID_SPACING: i32 = 2;
/// java: private static final int SECTION_SPACING = 16;
pub const SECTION_SPACING: i32 = 16;
/// java: private static final int HEADING_CONTENT_SPACING = 8;
pub const HEADING_CONTENT_SPACING: i32 = 8;

/// java: public final class CollectionsPage implements ProfileViewerPage<LoadingInformation>
pub struct CollectionsPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>(); (held in a `Screen`, the port's holder
    /// of game widgets: see `ProfileViewerPageAny::widgets_mut`)
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl CollectionsPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        this
    }

    /// java: private static double calculatePercentage(double amount, double total)
    pub fn calculate_percentage(mut amount: f64, mut total: f64) -> f64 {
        return (if total > (0 as f64) { ((amount / total) * 100.0) } else { ((0) as f64) as f64 });
    }

    /// java: private static void selectTab(int index, List<LayoutElement> tabContentLayouts)
    ///
    /// (`widgets` is the page's widget list the layouts' widget elements index - a press lambda's `screen`)
    pub fn select_tab(mut index: i32, tab_content_layouts: &Vec<LayoutElement>, widgets: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        for i in 0..(tab_content_layouts.len() as i32) {
            let mut should_be_visible: bool = (i == index);
            // java: tabContentLayouts.get(i).visitWidgets(widget -> widget.visible = shouldBeVisible);
            let mut __visited = Vec::new();
            (tab_content_layouts[(i) as usize]).visit_widgets(&mut __visited);
            for (__index, _, _) in __visited {
                widgets.widget_mut(__index).visible = should_be_visible;
            }
        }
    }

    /// java: private LayoutElement buildCollectionLayout(LoadingInformation info, String collectionCategory)
    pub fn build_collection_layout(&self, info: &LoadingInformation, collection_category: &str) -> LayoutElement {
        let mut category_layout: LinearLayout = LinearLayout::horizontal();
        category_layout.add_child(self.add_collection_items(info, collection_category));
        category_layout.add_child(SpacerElement::width(SECTION_SPACING).as_layout_element());
        category_layout.add_child(self.add_minion_items(info, collection_category));
        return LayoutElement::Grid(category_layout.wrapped);
    }

    /// java: private LayoutElement addCollectionItems(LoadingInformation info, String collectionCategory)
    pub fn add_collection_items(&self, info: &LoadingInformation, collection_category: &str) -> LayoutElement {
        let mut collections_layout: LinearLayout = LinearLayout::vertical();
        let mut collections_grid_layout: GridLayout = GridLayout::new_default(); collections_grid_layout.row_spacing(VERTICAL_GRID_SPACING); collections_grid_layout.column_spacing(HORIZONTAL_GRID_SPACING);
        let mut collections_row_helper: RowHelper = collections_grid_layout.create_row_helper(7);
        let mut category_contents: Vec<String> = CollectionTiers::get_collection_category_contents().get(collection_category).cloned().unwrap_or_else(|| vec![]);
        // java: Map<String, CollectionTiers.Report> profileCollections = categoryContents.stream()
        //   java: 				.map(itemId -> Map.entry(itemId, CollectionTiers.getCollectionReport(info.profile(), info.mainMember().id(), itemId)))
        //   java: 				.collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue, (a, _) -> a, LinkedHashMap::new));
        // (a `LinkedHashMap`: insertion order, the first entry of a repeated key kept)
        let mut profile_collections: Vec<(String, Report)> = Vec::new();
        let __main_member_id = info.main_member().id();
        for item_id in category_contents.iter() {
            if !profile_collections.iter().any(|(k, _)| k == item_id) {
                profile_collections.push((item_id.clone(), CollectionTiers::get_collection_report(&info.profile(), &__main_member_id, item_id)));
            }
        }
        let mut total_collections: i32 = (category_contents.len() as i32);
        let mut unlocked: i32 = 0;
        let mut maxed: i32 = 0;
        for entry in profile_collections.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut item_id: String = entry.0.clone();
            let report = &entry.1;
            if report.total() > (0 as i64) {
                unlocked += 1;
            }
            if report.tier() == CollectionTiers::get_max_tier(&item_id) {
                maxed += 1;
            }
            let mut widget: compat::Shared<dyn CollectionItemWidget> = CollectionItemWidgetBase::create(&item_id, info, report);
            collections_row_helper.add_child(&mut collections_grid_layout, crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::WidgetAdapter::widget(compat::handle_as::<CollectionItemWidgetBase, _>(&widget)))).unwrap();
        }
        //   java: String title = WordUtils.capitalizeFully(collectionCategory) + " Collections";
        let mut title: String = format!("{} Collections", compat::capitalize_fully(&collection_category));
        self.add_section_headings(&mut collections_layout, &title, total_collections, unlocked, maxed);
        collections_layout.add_child(LayoutElement::Grid(collections_grid_layout.clone()));
        return LayoutElement::Grid(collections_layout.wrapped);
    }

    /// java: private LayoutElement addMinionItems(LoadingInformation info, String category)
    pub fn add_minion_items(&self, info: &LoadingInformation, category: &str) -> LayoutElement {
        let mut minions_layout: LinearLayout = LinearLayout::vertical();
        let mut minions_grid_layout: GridLayout = GridLayout::new_default(); minions_grid_layout.row_spacing(VERTICAL_GRID_SPACING); minions_grid_layout.column_spacing(HORIZONTAL_GRID_SPACING);
        let mut minions_row_helper: RowHelper = minions_grid_layout.create_row_helper(6);
        let mut minions: Vec<String> = ConstantData::get_minion_constants().categories().get(category).cloned().unwrap_or_else(|| vec![]);
        // java: List<String> craftedMinions = info.profile().members.values().stream()
        //   java: 				.map(member -> member.playerData.craftedMinions)
        //   java: 				.flatMap(Set::stream)
        //   java: 				.toList();
        let __members: Vec<compat::Shared<crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember>> = compat::read(&info.profile(), |__p| __p.members.values().cloned().collect());
        let mut crafted_minions: Vec<String> = __members.iter().flat_map(|member| compat::read(member, |__m| __m.player_data.crafted_minions.iter().cloned().collect::<Vec<String>>())).collect();
        //   java: Map<String, Integer> maxTiers = !NEURepoManager.isLoading() ? NEURepoManager.getConstants().getMisc().getMaxMinionLevel() : Map.of();
        let mut max_tiers: std::collections::HashMap<String, i32> =
            if !(NEURepoManager::is_loading()) { NEURepoManager::get_constants().get_misc().get_max_minion_level().clone() } else { std::collections::HashMap::new() };
        let mut total_minions: i32 = (minions.len() as i32);
        let mut unlocked: i32 = 0;
        let mut maxed: i32 = 0;
        for minion_id in minions.iter() {
            let mut max_tier: i32 = max_tiers.get(&format!("{}_GENERATOR", minion_id)).cloned().unwrap_or_else(|| 11);
            let has_crafted_tier = |tier: i32| PlayerData::has_crafted_minion_tier(&crafted_minions, minion_id, tier);
            // java: List<Integer> tiersCrafted = IntStream.rangeClosed(1, maxTier).filter(hasCraftedTier).boxed().toList();
            let mut tiers_crafted: Vec<i32> = (1..=max_tier).filter(|tier| has_crafted_tier(*tier)).collect();
            // java: OptionalInt lowestUncraftedTier = IntStream.rangeClosed(1, maxTier).filter(hasCraftedTier.negate()).min();
            let mut lowest_uncrafted_tier: Option<i32> = (1..=max_tier).filter(|tier| !has_crafted_tier(*tier)).min();
            if !tiers_crafted.is_empty() {
                unlocked += 1;
            }
            if (tiers_crafted.len() as i32) == max_tier {
                maxed += 1;
            }
            let mut widget: compat::Shared<MinionWidget> = MinionWidget::create(minion_id, max_tier, &tiers_crafted, &lowest_uncrafted_tier);
            minions_row_helper.add_child(&mut minions_grid_layout, crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::WidgetAdapter::widget(widget.clone()))).unwrap();
        }
        //   java: String title = WordUtils.capitalizeFully(category) + " Minions";
        let mut title: String = format!("{} Minions", compat::capitalize_fully(&category));
        self.add_section_headings(&mut minions_layout, &title, total_minions, unlocked, maxed);
        minions_layout.add_child(LayoutElement::Grid(minions_grid_layout.clone()));
        return LayoutElement::Grid(minions_layout.wrapped);
    }

    /// java: private void addSectionHeadings(LinearLayout layout, String title, int total, int unlocked, int maxed)
    pub fn add_section_headings(&self, layout: &mut LinearLayout, title: &str, mut total: i32, mut unlocked: i32, mut maxed: i32) {
        let mut font: Font = compat::font();
        { let __settings = layout.new_cell_settings().align_horizontally_center(); layout.add_child_with(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::string_widget(0, 0, -1, 9, component::literal(title).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).with_style_format(ChatFormatting::Bold).without_shadow())), __settings); }
        let mut unlocked_percentage: f64 = CollectionsPage::calculate_percentage((unlocked) as f64, (total) as f64);
        let mut unlocked_text: String = format!("Unlocked: {}/{} ({}%)", unlocked, total, formatters::FLOAT_NUMBERS.get_mut().format(unlocked_percentage));
        { let __settings = layout.new_cell_settings().align_horizontally_center(); layout.add_child_with(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::string_widget(0, 0, -1, 9, component::literal(&unlocked_text).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).without_shadow())), __settings); }
        let mut maxed_percentage: f64 = CollectionsPage::calculate_percentage((maxed) as f64, (total) as f64);
        let mut maxed_text: String = format!("Maxed: {}/{} ({}%)", maxed, total, formatters::FLOAT_NUMBERS.get_mut().format(maxed_percentage));
        { let __settings = layout.new_cell_settings().align_horizontally_center(); layout.add_child_with(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::string_widget(0, 0, -1, 9, component::literal(&maxed_text).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).without_shadow())), __settings); }
        layout.add_child(SpacerElement::height(HEADING_CONTENT_SPACING).as_layout_element());
    }

}

impl ProfileViewerPageAny for CollectionsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::PAINTING.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Collections");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        CollectionsPage::load_dyn
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.children().to_vec();
    }

    fn widgets_mut(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.widgets
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for CollectionsPage {
    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut page_layout: LinearLayout = LinearLayout::horizontal();
        let mut tab_content_layouts: Arc<Vec<LayoutElement>> = Arc::new(vec![self.build_collection_layout(info, "FARMING"), self.build_collection_layout(info, "MINING"), self.build_collection_layout(info, "COMBAT"), self.build_collection_layout(info, "FISHING"), self.build_collection_layout(info, "FORAGING"), self.build_collection_layout(info, "RIFT")]);
        let mut tab_buttons: Vec<compat::Shared<ButtonWidget>> = vec![ButtonWidget::new(ico::GOLDEN_HOE.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(0, &__layouts, __screen); })) }), ButtonWidget::new(ico::STONE_PICKAXE.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(1, &__layouts, __screen); })) }), ButtonWidget::new(ico::STONE_SWORD.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(2, &__layouts, __screen); })) }), ButtonWidget::new(ico::FISH_ROD.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(3, &__layouts, __screen); })) }), ButtonWidget::new(ico::JUNGLE_SAPLING.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(4, &__layouts, __screen); })) }), ButtonWidget::new(ico::MYCELIUM.get_mut().clone(), { let __layouts = tab_content_layouts.clone(); OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(5, &__layouts, __screen); })) })];
        let mut tab_layout: LinearLayout = LinearLayout::vertical().spacing(1);
        for button in tab_buttons.iter() {
            tab_layout.add_child(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::WidgetAdapter::widget(button.clone())));
        }
        page_layout.add_child_with(LayoutElement::Grid(tab_layout.clone().wrapped), page_layout.new_cell_settings().align_vertically_middle());
        page_layout.add_child(SpacerElement::width(SECTION_SPACING).as_layout_element());
        let mut content_frame: FrameLayout = FrameLayout::new();
        for layout in tab_content_layouts.iter().cloned() {
            content_frame.add_child_with(layout, content_frame.new_child_layout_settings().align_vertically_top());
        }
        page_layout.add_child(LayoutElement::Frame(content_frame.clone()));
        // java: pageLayout.visitWidgets(this.widgets::add); - every widget the layout holds was collected as it was put in
        // it (`layout_widget`); `load` hands them to the page
        // java: selectTab(0, tabContentLayouts);
        crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::with_page_widgets(|__widgets| CollectionsPage::select_tab(0, &tab_content_layouts, __widgets));
        return LayoutElement::Grid(page_layout.wrapped);
    }

}

impl CollectionsPage {
    /// CollectionsPage.load as the dispatch of ProfileViewerPage.load
    pub fn load_dyn(__this: &compat::Shared<dyn ProfileViewerPageAny>, info: &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        // java: return CompletableFuture.completedFuture(info).thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        let __h: compat::Shared<CollectionsPage> = compat::handle_as::<CollectionsPage, _>(__this);
        return compat::LocalFuture::completed_future(info.clone()).then_apply_async(move |__a0| {
            let (__layout, __widgets) = crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::collect_widgets(|| compat::read(&__h, |__t| __t.build_widgets(&__a0)));
            compat::write(&__h, |__t| __t.widgets = __widgets);
            __layout
        });
    }
}
