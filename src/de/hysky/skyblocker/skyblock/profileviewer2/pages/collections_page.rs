//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.CollectionsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::player_data::PlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
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
#[derive(Clone)]
pub struct CollectionsPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
}

impl CollectionsPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::new(),
        };
        this
    }

    /// java: private static double calculatePercentage(double amount, double total)
    pub fn calculate_percentage(mut amount: f64, mut total: f64) -> f64 {
        return (if total > (0 as f64) { ((amount / total) * 100.0) } else { ((0) as f64) as f64 });
    }

    /// java: private static void selectTab(int index, List<LayoutElement> tabContentLayouts)
    pub fn select_tab(mut index: i32, tab_content_layouts: &Vec<LayoutElement>) {
        for i in 0..(tab_content_layouts.len() as i32) {
            let mut should_be_visible: bool = (i == index);
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
            // rust: (tab_content_layouts[(i) as usize]).visit_widgets(|widget| { widget.visible = should_be_visible; });
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
        let mut collections_grid_layout: GridLayout = GridLayout::new_default().row_spacing(VERTICAL_GRID_SPACING).column_spacing(HORIZONTAL_GRID_SPACING);
        let mut collections_row_helper: RowHelper = collections_grid_layout.create_row_helper(7);
        let mut category_contents: Vec<String> = CollectionTiers::get_collection_category_contents().get(collection_category).cloned().unwrap_or_else(|| vec![]);
        // TODO(fabric2seer): static field Map.Entry
        //   java: Map<String, CollectionTiers.Report> profileCollections = categoryContents.stream()
        //   java: 				.map(itemId -> Map.entry(itemId, CollectionTiers.getCollectionReport(info.profile(), info.mainMember().id(), itemId)))
        //   java: 				.collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue, (a, _) -> a, LinkedHashMap::new));
        let mut profile_collections: std::collections::HashMap<String, Report> = todo!("fabric2seer: the initialiser of `profileCollections` (TODO above)");
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
            // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
            // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
            // rust: collections_row_helper.add_child(&mut widget).unwrap();
        }
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: String title = WordUtils.capitalizeFully(collectionCategory) + " Collections";
        let mut title: String = todo!("fabric2seer: the initialiser of `title` (TODO above)");
        self.add_section_headings(&mut collections_layout, &title, total_collections, unlocked, maxed);
        collections_layout.add_child(LayoutElement::Grid(collections_grid_layout.clone()));
        return LayoutElement::Grid(collections_layout.wrapped);
    }

    /// java: private LayoutElement addMinionItems(LoadingInformation info, String category)
    pub fn add_minion_items(&self, info: &LoadingInformation, category: &str) -> LayoutElement {
        let mut minions_layout: LinearLayout = LinearLayout::vertical();
        let mut minions_grid_layout: GridLayout = GridLayout::new_default().row_spacing(VERTICAL_GRID_SPACING).column_spacing(HORIZONTAL_GRID_SPACING);
        let mut minions_row_helper: RowHelper = minions_grid_layout.create_row_helper(6);
        let mut minions: Vec<String> = ConstantData::get_minion_constants().categories().get(category).cloned().unwrap_or_else(|| vec![]);
        // TODO(fabric2seer): static call Set.stream(..)
        //   java: List<String> craftedMinions = info.profile().members.values().stream()
        //   java: 				.map(member -> member.playerData.craftedMinions)
        //   java: 				.flatMap(Set::stream)
        //   java: 				.toList();
        let mut crafted_minions: Vec<String> = todo!("fabric2seer: the initialiser of `craftedMinions` (TODO above)");
        // TODO(fabric2seer): `.getMisc(..)` on a NEUConstants
        //   java: Map<String, Integer> maxTiers = !NEURepoManager.isLoading() ? NEURepoManager.getConstants().getMisc().getMaxMinionLevel() : Map.of();
        let mut max_tiers: std::collections::HashMap<String, i32> = todo!("fabric2seer: the initialiser of `maxTiers` (TODO above)");
        let mut total_minions: i32 = (minions.len() as i32);
        let mut unlocked: i32 = 0;
        let mut maxed: i32 = 0;
        for minion_id in minions.iter() {
            let mut max_tier: i32 = max_tiers.get(&format!("{}_GENERATOR", minion_id)).cloned().unwrap_or_else(|| 11);
            let mut has_crafted_tier: Arc<dyn Fn(i32) -> bool> = Arc::new(move |tier| PlayerData::has_crafted_minion_tier(&crafted_minions, minion_id, tier));
            // TODO(fabric2seer): not a lambda
            //   java: List<Integer> tiersCrafted = IntStream.rangeClosed(1, maxTier)
            //   java: 					.filter(hasCraftedTier)
            //   java: 					.boxed()
            //   java: 					.toList();
            let mut tiers_crafted: Vec<i32> = todo!("fabric2seer: the initialiser of `tiersCrafted` (TODO above)");
            // TODO(fabric2seer): not a lambda
            //   java: OptionalInt lowestUncraftedTier = IntStream.rangeClosed(1, maxTier)
            //   java: 					.filter(hasCraftedTier.negate())
            //   java: 					.min();
            let mut lowest_uncrafted_tier: Option<i32> = todo!("fabric2seer: the initialiser of `lowestUncraftedTier` (TODO above)");
            if !tiers_crafted.is_empty() {
                unlocked += 1;
            }
            if (tiers_crafted.len() as i32) == max_tier {
                maxed += 1;
            }
            let mut widget: compat::Shared<MinionWidget> = MinionWidget::create(minion_id, max_tier, &tiers_crafted, &lowest_uncrafted_tier);
            // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
            // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
            // rust: minions_row_helper.add_child(&mut widget).unwrap();
        }
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: String title = WordUtils.capitalizeFully(category) + " Minions";
        let mut title: String = todo!("fabric2seer: the initialiser of `title` (TODO above)");
        self.add_section_headings(&mut minions_layout, &title, total_minions, unlocked, maxed);
        minions_layout.add_child(LayoutElement::Grid(minions_grid_layout.clone()));
        return LayoutElement::Grid(minions_layout.wrapped);
    }

    /// java: private void addSectionHeadings(LinearLayout layout, String title, int total, int unlocked, int maxed)
    pub fn add_section_headings(&self, layout: &mut LinearLayout, title: &str, mut total: i32, mut unlocked: i32, mut maxed: i32) {
        let mut font: Font = compat::font();
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child_with(compat::string_widget(0, 0, -1, 9, component::literal(title).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).with_style_format(ChatFormatting::Bold).without_shadow()), layout.new_cell_settings().align_horizontally_center());
        let mut unlocked_percentage: f64 = CollectionsPage::calculate_percentage((unlocked) as f64, (total) as f64);
        let mut unlocked_text: String = format!("Unlocked: {}/{} ({}%)", unlocked, total, formatters::FLOAT_NUMBERS.get_mut().format(unlocked_percentage));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child_with(compat::string_widget(0, 0, -1, 9, component::literal(&unlocked_text).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).without_shadow()), layout.new_cell_settings().align_horizontally_center());
        let mut maxed_percentage: f64 = CollectionsPage::calculate_percentage((maxed) as f64, (total) as f64);
        let mut maxed_text: String = format!("Maxed: {}/{} ({}%)", maxed, total, formatters::FLOAT_NUMBERS.get_mut().format(maxed_percentage));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child_with(compat::string_widget(0, 0, -1, 9, component::literal(&maxed_text).with_color(mcrust::net::minecraft::util::common_colors::DARK_GRAY).without_shadow()), layout.new_cell_settings().align_horizontally_center());
        layout.add_child(SpacerElement::height(HEADING_CONTENT_SPACING).as_layout_element());
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for CollectionsPage {
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

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info)
    fn load(&self, info: &LoadingInformation) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<LayoutElement> {
        return CompletableFuture::completed_future(info.clone()).then_apply_async(|__a0| self.build_widgets(&__a0), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut page_layout: LinearLayout = LinearLayout::horizontal();
        let mut tab_content_layouts: Vec<LayoutElement> = vec![self.build_collection_layout(info, "FARMING"), self.build_collection_layout(info, "MINING"), self.build_collection_layout(info, "COMBAT"), self.build_collection_layout(info, "FISHING"), self.build_collection_layout(info, "FORAGING"), self.build_collection_layout(info, "RIFT")];
        let mut tab_buttons: Vec<ButtonWidget> = vec![ButtonWidget::new(ico::GOLDEN_HOE.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(0, &tab_content_layouts); }))), ButtonWidget::new(ico::STONE_PICKAXE.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(1, &tab_content_layouts); }))), ButtonWidget::new(ico::STONE_SWORD.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(2, &tab_content_layouts); }))), ButtonWidget::new(ico::FISH_ROD.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(3, &tab_content_layouts); }))), ButtonWidget::new(ico::JUNGLE_SAPLING.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(4, &tab_content_layouts); }))), ButtonWidget::new(ico::MYCELIUM.get_mut().clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { CollectionsPage::select_tab(5, &tab_content_layouts); })))];
        let mut tab_layout: LinearLayout = LinearLayout::vertical().spacing(1);
        for button in tab_buttons.iter() {
            tab_layout.add_child(button.clone());
        }
        page_layout.add_child_with(LayoutElement::Grid(tab_layout.clone().wrapped), page_layout.new_cell_settings().align_vertically_middle());
        page_layout.add_child(SpacerElement::width(SECTION_SPACING).as_layout_element());
        let mut content_frame: FrameLayout = FrameLayout::new();
        for layout in tab_content_layouts.iter().copied() {
            content_frame.add_child_with(layout, content_frame.new_child_layout_settings().align_vertically_top());
        }
        page_layout.add_child(LayoutElement::Frame(content_frame.clone()));
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|__a0| { self.widgets.push(__a0.clone()); });
        CollectionsPage::select_tab(0, &tab_content_layouts);
        return LayoutElement::Grid(page_layout.wrapped);
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.clone();
    }

}
