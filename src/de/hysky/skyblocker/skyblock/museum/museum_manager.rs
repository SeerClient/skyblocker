//! Ported from de.hysky.skyblocker.skyblock.museum.MuseumManager by fabric2seer.
//!
//! The widget is ONE object (`compat::ObjectWidget`, batch E pattern 7): Java's `this` is what the screen's widget list
//! holds (`Screens.getWidgets(screen).add(this)`), and its own widgets - the search field, the four buttons and the
//! twelve donation buttons - are the game's widgets in a `Screen` the object owns (`children`), which it draws,
//! hovers and clicks in Java's order. A child's click is tested on the child itself, as Java's `mouseClicked` calls
//! `child.mouseClicked(..)`; a `Button`'s `onPress` runs as Java's `mouseClicked -> onClick -> onPress` would, inside
//! this object's call.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::item_price;
use crate::de::hysky::skyblocker::skyblock::item::item_price::ItemPrice;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::donation_button::DonationButton;
use crate::de::hysky::skyblocker::skyblock::museum::item_filter::ItemFilter;
use crate::de::hysky::skyblocker::skyblock::museum::item_sorter::ItemSorter;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache::MuseumItemCache;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input_type::InputType;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Font TEXT_RENDERER = CLIENT.font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());
/// java: private static final Identifier BACKGROUND_TEXTURE = SkyblockerMod.id("background");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("background"));
/// java: public static final int BACKGROUND_WIDTH = 147;
pub const BACKGROUND_WIDTH: i32 = 147;
/// java: public static final int BACKGROUND_HEIGHT = 160;
pub const BACKGROUND_HEIGHT: i32 = 160;
/// java: public static final int SPACING = 2;
pub const SPACING: i32 = 2;
/// java: private static final int BUTTON_SIZE = 20;
pub const BUTTON_SIZE: i32 = 20;
/// java: private static final int BUTTONS_PER_PAGE = 12;
pub const BUTTONS_PER_PAGE: i32 = 12;
/// java: private static final ItemSorter ITEM_SORTER = new ItemSorter();
/// (a mutable object behind a `final` field: a render-thread static)
pub static ITEM_SORTER: compat::MainThread<ItemSorter> = compat::MainThread::new(|| ItemSorter::new());
/// java: private static final ItemFilter ITEM_FILTER = new ItemFilter();
pub static ITEM_FILTER: compat::MainThread<ItemFilter> = compat::MainThread::new(|| ItemFilter::new());
/// java: private static String searchQuery = "";
pub static SEARCH_QUERY: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static int currentPage = 0;
pub static CURRENT_PAGE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static List<Donation> donations = new ArrayList<>();
pub static DONATIONS: compat::MainThread<Vec<compat::Shared<Donation>>> = compat::MainThread::new(|| Vec::new());

/// `children`' index of `searchField`
const SEARCH_FIELD: usize = 0;
/// `children`' index of `sortButton`
const SORT_BUTTON: usize = 1;
/// `children`' index of `filterButton`
const FILTER_BUTTON: usize = 2;
/// `children`' index of `nextPageButton`
const NEXT_PAGE_BUTTON: usize = 3;
/// `children`' index of `prevPageButton`
const PREV_PAGE_BUTTON: usize = 4;
/// `children`' index of `donationButtons.get(0)` (the twelve follow in order)
const FIRST_DONATION_BUTTON: usize = 5;

/// java: public class MuseumManager extends AbstractWidget implements HoveredItemStackProvider
pub struct MuseumManager {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget - the object's widget fields (`ObjectWidget`)
    pub base: AbstractContainerWidget,
    /// java: private final EditBox searchField; / private final Button sortButton; / private final Button filterButton;
    /// java: private final ImageButton nextPageButton; / private final ImageButton prevPageButton;
    /// java: (and the `DonationButton`s) - the game's widgets, at the indices above
    pub children: Screen,
    /// java: private final List<Donation> filteredDonations = new ArrayList<>();
    pub filtered_donations: Vec<compat::Shared<Donation>>,
    /// java: private final List<String> excludedDonationIds = new ArrayList<>();
    pub excluded_donation_ids: Vec<String>,
    /// java: private final List<DonationButton> donationButtons = Lists.newArrayListWithCapacity(BUTTONS_PER_PAGE);
    pub donation_buttons: Vec<compat::Shared<DonationButton>>,
    /// java: private @Nullable DonationButton hoveredDonationButton;
    pub hovered_donation_button: Option<compat::Shared<DonationButton>>,
    /// java: private int pageCount = 0;
    pub page_count: i32,
}

impl MuseumManager {
    /// java: public MuseumManager(Screen screen, int x, int y, int backgroundWidth)
    /// (`screen` is the open container screen's `Screen` half: this widget joins its list and takes its focus)
    pub fn new(screen: &mut Screen, mut x: i32, mut y: i32, mut background_width: i32) -> compat::Shared<Self> {
        // java: super(x + backgroundWidth + SPACING, y, BACKGROUND_WIDTH, BACKGROUND_HEIGHT, Component.empty());
        let mut this = MuseumManager {
            base: AbstractContainerWidget::new((x.wrapping_add(background_width)).wrapping_add(SPACING), y, BACKGROUND_WIDTH, BACKGROUND_HEIGHT, FormattedText::from(component::empty())),
            children: Screen::new(String::new()),
            filtered_donations: Vec::new(),
            excluded_donation_ids: Vec::new(),
            donation_buttons: Vec::with_capacity(BUTTONS_PER_PAGE as usize),
            hovered_donation_button: None,
            page_count: 0,
        };
        let (left, top) = (this.base.get_x(), this.base.get_y());

        // Initialize search field
        // java: this.searchField = new EditBox(TEXT_RENDERER, getX() + 25, getY() + 11, 69, 20, Component.empty());
        let mut search_field: AbstractWidget = compat::edit_box(left.wrapping_add(25), top.wrapping_add(11), 69, 20, component::empty());
        compat::with_edit_box(&mut search_field, |e| {
            // java: this.searchField.setMaxLength(60);
            e.set_max_length(60);
            // java: this.searchField.setTextColor(CommonColors.WHITE);
            e.set_text_color(mcrust::net::minecraft::util::common_colors::WHITE);
            // java: this.searchField.setHint(Component.translatable("gui.recipebook.search_hint").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY));
            e.set_hint_component(component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray));
        });
        // java: this.searchField.setVisible(true);
        search_field.visible = true;
        // java: this.searchField.setValue(searchQuery);
        compat::set_edit_box_value(&mut search_field, &compat::get_state(&SEARCH_QUERY));

        // Initialize page navigation buttons
        // java: this.nextPageButton = new ImageButton(getX() + 93, getY() + 133, 12, 17, RecipeBookPage.PAGE_FORWARD_SPRITES, _ -> {});
        // java: this.prevPageButton = new ImageButton(getX() + 38, getY() + 133, 12, 17, RecipeBookPage.PAGE_BACKWARD_SPRITES, _ -> {});
        // (`PAGE_*_SPRITES = new WidgetSprites(sprite, highlighted)`; the press is empty - the page turns in `mouseClicked`)
        let next_page_button = AbstractWidget::new(left.wrapping_add(93), top.wrapping_add(133), 12, 17, mcrust::net::minecraft::client::gui::components::image_button::DEFAULT_MESSAGE.to_string(), WidgetKind::ImageButton(WidgetSprites::with_focused(recipe_book_page::page_forward_sprite(), recipe_book_page::page_forward_highlighted_sprite())), OnPress::Inert);
        let prev_page_button = AbstractWidget::new(left.wrapping_add(38), top.wrapping_add(133), 12, 17, mcrust::net::minecraft::client::gui::components::image_button::DEFAULT_MESSAGE.to_string(), WidgetKind::ImageButton(WidgetSprites::with_focused(recipe_book_page::page_backward_sprite(), recipe_book_page::page_backward_highlighted_sprite())), OnPress::Inert);

        // java: donations = MuseumItemCache.getDonations();
        *DONATIONS.get_mut() = MuseumItemCache::get_donations();
        // java: ITEM_FILTER.updateCategories();
        ITEM_FILTER.get_mut().update_categories();

        // Create donation buttons for pagination
        for i in 0..BUTTONS_PER_PAGE {
            let mut button: compat::Shared<DonationButton> = DonationButton::new((left.wrapping_add(11)).wrapping_add(31i32.wrapping_mul(i.wrapping_rem(4))), (top.wrapping_add(34)).wrapping_add(31i32.wrapping_mul(i.wrapping_div(4))));
            this.donation_buttons.push(button);
        }

        // Initialize sort button
        // java: this.sortButton = Button.builder(Component.empty(), button -> {..}).tooltip(ITEM_SORTER.getTooltip()).pos(getX() + 95, getY() + 11).size(BUTTON_SIZE, BUTTON_SIZE).build();
        // (the press body is `press_sort_button`, run where Java's `mouseClicked` runs `onPress`)
        let sort_button: AbstractWidget = button::builder(component::empty().get_string(), OnPress::Inert).tooltip_text(Some(ITEM_SORTER.get_tooltip().message.clone())).pos(left.wrapping_add(95), top.wrapping_add(11)).size(BUTTON_SIZE, BUTTON_SIZE).build();
        // Initialize filter button
        // java: this.filterButton = Button.builder(Component.empty(), button -> {..}).tooltip(ITEM_FILTER.getTooltip()).pos(getX() + 116, getY() + 11).size(BUTTON_SIZE, BUTTON_SIZE).build();
        let filter_button: AbstractWidget = button::builder(component::empty().get_string(), OnPress::Inert).tooltip_text(Some(ITEM_FILTER.get_tooltip().message.clone())).pos(left.wrapping_add(116), top.wrapping_add(11)).size(BUTTON_SIZE, BUTTON_SIZE).build();

        this.children.add_renderable_widget(search_field);
        this.children.add_renderable_widget(sort_button);
        this.children.add_renderable_widget(filter_button);
        this.children.add_renderable_widget(next_page_button);
        this.children.add_renderable_widget(prev_page_button);
        for button in this.donation_buttons.iter() {
            let index = this.children.add_renderable_widget(compat::WidgetAdapter::widget(button.clone()));
            // the button starts hidden (`DonationButton.resetButton` / `init` set it below)
            this.children.widget_mut(index).visible = compat::read(button, |b| b.visible);
        }

        // java: ITEM_FILTER.applyFilter(donations, filteredDonations); ITEM_SORTER.applySort(filteredDonations); - in place
        ITEM_FILTER.get().apply_filter(DONATIONS.get(), &mut this.filtered_donations);
        ITEM_SORTER.get().apply_sort(&mut this.filtered_donations);
        this.update_search_results(false);

        let __this = compat::shared(this);
        // java: Screens.getWidgets(screen).add(this);
        let index = screen.add_renderable_widget(compat::object_widget(__this.clone(), ""));
        // java: screen.setFocused(this);
        screen.set_focused(Some(index));
        __this
    }

    /// java: protected static @Nullable Donation getDonation(String id)
    pub fn get_donation(id: &str) -> Option<compat::Shared<Donation>> {
        return DONATIONS.get_mut().iter().cloned().filter(|donation| (Donation::get_id(donation) == id)).next();
    }

    /// java: private void updateNavigationButtons()
    pub fn update_navigation_buttons(&mut self) {
        self.children.widget_mut(PREV_PAGE_BUTTON).active = compat::get_state(&CURRENT_PAGE) > 0;
        self.children.widget_mut(NEXT_PAGE_BUTTON).active = compat::get_state(&CURRENT_PAGE) < (self.page_count).wrapping_sub(1);
    }

    /// java: private void updateButtons()
    pub fn update_buttons(&mut self) {
        let mut visible_donations: Vec<compat::Shared<Donation>> = self.filtered_donations.iter().cloned().filter(|donation| !self.excluded_donation_ids.contains(&Donation::get_id(donation))).collect::<Vec<_>>();
        let mut buttons_size: i32 = visible_donations.len() as i32;
        self.page_count = ((buttons_size as f64) / (BUTTONS_PER_PAGE as f64)).ceil() as i32;
        for i in 0..(self.donation_buttons.len() as i32) {
            let mut index: i32 = (compat::get_state(&CURRENT_PAGE)).wrapping_mul(self.donation_buttons.len() as i32).wrapping_add(i);
            let button = self.donation_buttons[i as usize].clone();
            if index < buttons_size && index >= 0 {
                DonationButton::init(&button, visible_donations[index as usize].clone());
            } else {
                DonationButton::reset_button(&button);
            }
            // the button's `visible` (which `init` / `resetButton` set) on the game's widget it is
            self.children.widget_mut(FIRST_DONATION_BUTTON + i as usize).visible = compat::read(&button, |b| b.visible);
        }
        self.update_navigation_buttons();
    }

    /// java: public void updateSearchResults(boolean resetPage)
    pub fn update_search_results(&mut self, mut reset_page: bool) {
        compat::set_state(&SEARCH_QUERY, compat::edit_box_value(self.children.widget(SEARCH_FIELD)));
        self.excluded_donation_ids.clear();
        let search_query: String = compat::get_state(&SEARCH_QUERY);
        for item in DONATIONS.get().iter() {
            let mut searchable_content: String = String::new();
            let mut item_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&Donation::get_id(item));
            if let Some(item_stack) = &item_stack {
                searchable_content.push_str(&compat::component_of(FlexibleItemStack::get_stack_or_throw(item_stack).get_hover_name()).get_string());
                searchable_content.push_str(&ItemUtils::get_concatenated_lore(&FlexibleItemStack::get_stack_or_throw(item_stack)));
            }
            // java: if (item.getSet() != null && !item.getSet().isEmpty())
            if !(Donation::get_set(item).is_empty()) {
                for piece in Donation::get_set(item) {
                    let mut piece_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&piece.0);
                    if let Some(piece_stack) = &piece_stack {
                        searchable_content.push_str(&compat::component_of(FlexibleItemStack::get_stack_or_throw(piece_stack).get_hover_name()).get_string());
                        searchable_content.push_str(&ItemUtils::get_concatenated_lore(&FlexibleItemStack::get_stack_or_throw(piece_stack)));
                    }
                }
            }
            // java: toLowerCase(Locale.ENGLISH)
            if !(searchable_content.to_lowercase().contains(&search_query.to_lowercase())) {
                self.excluded_donation_ids.push(Donation::get_id(item));
            }
        }
        if reset_page {
            compat::set_state(&CURRENT_PAGE, 0);
        }
        self.update_buttons();
    }

    /// java: sortButton's `onPress`: `ITEM_SORTER.cycleSortMode(filteredDonations); button.setTooltip(ITEM_SORTER.getTooltip());
    /// currentPage = 0; updateButtons();`
    fn press_sort_button(&mut self) {
        ITEM_SORTER.get_mut().cycle_sort_mode(&mut self.filtered_donations);
        self.children.widget_mut(SORT_BUTTON).set_tooltip(ITEM_SORTER.get_tooltip());
        compat::set_state(&CURRENT_PAGE, 0);
        self.update_buttons();
    }

    /// java: filterButton's `onPress`: `ITEM_FILTER.cycleFilterMode(donations, filteredDonations);
    /// ITEM_SORTER.applySort(filteredDonations); button.setTooltip(ITEM_FILTER.getTooltip()); currentPage = 0; updateButtons();`
    fn press_filter_button(&mut self) {
        ITEM_FILTER.get_mut().cycle_filter_mode(DONATIONS.get(), &mut self.filtered_donations);
        ITEM_SORTER.get().apply_sort(&mut self.filtered_donations);
        self.children.widget_mut(FILTER_BUTTON).set_tooltip(ITEM_FILTER.get_tooltip());
        compat::set_state(&CURRENT_PAGE, 0);
        self.update_buttons();
    }

    /// java: public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip(&self, mut x: i32, mut y: i32) {
        // Draw the tooltip of the hovered result button if one is hovered over
        if let Some(hovered) = &self.hovered_donation_button {
            let mut tooltip: Vec<MutableComponent> = DonationButton::get_item_tooltip(hovered);
            // java: Identifier tooltipStyle = hoveredDonationButton.getItem().get(DataComponents.TOOLTIP_STYLE); (nullable)
            let mut tooltip_style: Option<Identifier> = (DonationButton::get_item(hovered)).get(mcrust::net::minecraft::core::component::data_components::tooltip_style()).and_then(|o| mcrust::com::mojang::serialization::cast::<Identifier>(&o).cloned());
            // java: graphics.setComponentTooltipForNextFrame(TEXT_RENDERER, tooltip, x, y, tooltipStyle);
            // rust: port gap (GAPS_batchE #1) - the frame's `DeferredTooltip` is not reachable from a mod widget's draw;
            // the port's call is `gui_graphics_extractor::set_component_tooltip_for_next_frame_styled(&mut deferred, lines)`
            let _ = (tooltip, tooltip_style, x, y);
        }
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let hovered = compat::read(__this, |__t| __t.hovered_donation_button.clone())?;
        return DonationButton::get_display_stack(&hovered);
    }

    /// MuseumManager.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }
}

impl HoveredItemStackProvider for MuseumManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem() (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        MuseumManager::get_focused_item_dyn
    }
}

impl compat::ObjectWidget for MuseumManager {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let (x, y, w, h) = (self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        // Render the background texture for the widget
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get(), x, y, w, h);

        let files_imported: bool = ItemRepository::files_imported();
        // Java draws the search field always, the sort / filter buttons while active, and - once the item repository
        // has imported - the donation buttons and the ACTIVE page arrows: the children's `visible` is set to that, and
        // the children are drawn (and hovered) in one pass
        let sort_active: bool = self.children.widget(SORT_BUTTON).active;
        let filter_active: bool = self.children.widget(FILTER_BUTTON).active;
        self.children.widget_mut(SORT_BUTTON).visible = sort_active;
        self.children.widget_mut(FILTER_BUTTON).visible = filter_active;
        let prev_active: bool = self.children.widget(PREV_PAGE_BUTTON).active;
        let next_active: bool = self.children.widget(NEXT_PAGE_BUTTON).active;
        self.children.widget_mut(PREV_PAGE_BUTTON).visible = files_imported && prev_active;
        self.children.widget_mut(NEXT_PAGE_BUTTON).visible = files_imported && next_active;
        for i in 0..self.donation_buttons.len() {
            let visible = files_imported && compat::read(&self.donation_buttons[i], |b| b.visible);
            self.children.widget_mut(FIRST_DONATION_BUTTON + i).visible = visible;
        }
        // java: searchField.extractRenderState(..); sortButton.extractRenderState(..); filterButton.extractRenderState(..);
        // java: resultButton.extractRenderState(..) (each); prevPageButton / nextPageButton.extractRenderState(..)
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);

        if sort_active {
            let sort_button = self.children.widget(SORT_BUTTON);
            let mut icon_x: i32 = (sort_button.get_x()).wrapping_add(((sort_button.get_width()).wrapping_sub(16)).wrapping_div(2));
            let mut icon_y: i32 = (sort_button.get_y()).wrapping_add(((sort_button.get_height()).wrapping_sub(16)).wrapping_div(2));
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = ITEM_SORTER.get_current_sorting_item();
            context.graphics.fake_item(&stack, icon_x, icon_y);
        }
        if filter_active {
            let filter_button = self.children.widget(FILTER_BUTTON);
            let mut icon_x: i32 = (filter_button.get_x()).wrapping_add(((filter_button.get_width()).wrapping_sub(16)).wrapping_div(2));
            let mut icon_y: i32 = (filter_button.get_y()).wrapping_add(((filter_button.get_height()).wrapping_sub(16)).wrapping_div(2));
            context.graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(ico::HOPPER.get()), icon_x, icon_y);
        }

        if files_imported {
            // Render page count if multiple pages exist
            if self.page_count > 1 {
                let mut text: MutableComponent = component::translatable_args("gui.recipebook.page", vec![FormattedText::of((compat::get_state(&CURRENT_PAGE)).wrapping_add(1).to_string()), FormattedText::of(self.page_count.to_string())]);
                let mut width: i32 = TEXT_RENDERER.get_mut().width_of_cached(&FormattedText::from(text.clone()));
                // java: graphics.text(TEXT_RENDERER, text, getX() - width / 2 + 73, getY() + 137, -1, false);
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(context.gl, context.minecraft, &FormattedText::from(text.clone()), (x.wrapping_sub(width.wrapping_div(2))).wrapping_add(73), y.wrapping_add(137), -1, false);
            }
            // java: this.hoveredDonationButton = null; for (..) if (resultButton.visible && resultButton.isHovered()) hoveredDonationButton = resultButton;
            self.hovered_donation_button = None;
            for i in 0..self.donation_buttons.len() {
                let widget = self.children.widget(FIRST_DONATION_BUTTON + i);
                if widget.visible && widget.is_hovered() {
                    self.hovered_donation_button = Some(self.donation_buttons[i].clone());
                }
            }
            self.extract_tooltip(mouse_x, mouse_y);
        } else {
            gui_graphics_extractor::centered_text_string(context.gl, context.minecraft, "Loading...", x.wrapping_add(BACKGROUND_WIDTH.wrapping_div(2)), y.wrapping_add(BACKGROUND_HEIGHT.wrapping_div(2)), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) {}
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {}

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, click: &MouseButtonEvent, doubled: bool) -> bool {
        let (button, x, y) = (click.button(), click.x(), click.y());
        // java: if (this.searchField.mouseClicked(click, doubled)) { this.searchField.setFocused(true); return true; }
        // (`AbstractWidget.mouseClicked` -> `EditBox.onClick`, which moves the cursor to the click)
        if self.children.widget(SEARCH_FIELD).mouse_clicked(button, x, y).is_some() {
            let font = compat::font();
            let shift: bool = mcrust::net::minecraft::client::minecraft::has_shift_down();
            compat::with_edit_box(self.children.widget_mut(SEARCH_FIELD), |e| e.on_click(&font.splitter(), x, shift, doubled));
            self.children.widget_mut(SEARCH_FIELD).set_focused(true, InputType::Mouse);
            return true;
        } else if self.children.widget(NEXT_PAGE_BUTTON).mouse_clicked(button, x, y).is_some() {
            compat::set_state(&CURRENT_PAGE, compat::get_state(&CURRENT_PAGE) + 1);
            self.update_buttons();
            return true;
        } else if self.children.widget(PREV_PAGE_BUTTON).mouse_clicked(button, x, y).is_some() {
            compat::set_state(&CURRENT_PAGE, compat::get_state(&CURRENT_PAGE) - 1);
            self.update_buttons();
            return true;
        } else if self.children.widget(FILTER_BUTTON).mouse_clicked(button, x, y).is_some() {
            // `Button.mouseClicked -> onClick -> onPress`
            self.press_filter_button();
            return true;
        } else if self.children.widget(SORT_BUTTON).mouse_clicked(button, x, y).is_some() {
            self.press_sort_button();
            return true;
        }
        // java: this.searchField.setFocused(false);
        self.children.widget_mut(SEARCH_FIELD).set_focused(false, InputType::Mouse);
        return false;
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, _widget: &mut AbstractWidget, input: &CharacterEvent) -> bool {
        // java: if (this.searchField.charTyped(input)) { updateSearchResults(true); return true; }
        let search_field = self.children.widget_mut(SEARCH_FIELD);
        let (is_focused, active) = (search_field.is_focused(), search_field.active);
        let typed: bool = match char::from_u32(input.codepoint() as u32) {
            Some(c) => {
                let font = compat::font();
                compat::with_edit_box(search_field, |e| e.char_typed(&font.splitter(), c, is_focused, active)).unwrap_or(false)
            }
            None => false,
        };
        if typed {
            self.update_search_results(true);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, input: &KeyEvent) -> bool {
        // java: if ((this.searchField.canConsumeInput() && CLIENT.options.keyInventory.matches(input)) || this.searchField.keyPressed(input)) {..}
        let search_field = self.children.widget_mut(SEARCH_FIELD);
        let (is_focused, active) = (search_field.is_focused(), search_field.is_active());
        let can_consume_input: bool = compat::with_edit_box(search_field, |e| e.can_consume_input(active, is_focused)).unwrap_or(false);
        let consumed: bool = (can_consume_input && key_mapping::matches("key.inventory", input)) || {
            let font = compat::font();
            let active = search_field.active;
            compat::with_edit_box(search_field, |e| {
                let mut memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
                match mcrust::net::minecraft::client::minecraft::GlfwClipboard::published() {
                    Some(mut clipboard) => e.key_pressed(&font.splitter(), input.clone(), is_focused, active, &mut clipboard),
                    None => e.key_pressed(&font.splitter(), input.clone(), is_focused, active, &mut memory),
                }
            })
            .unwrap_or(false)
        };
        if consumed {
            self.update_search_results(true);
            return true;
        }
        if let Some(hovered) = self.hovered_donation_button.clone() {
            let Some(mut hovered_stack) = DonationButton::get_display_stack(&hovered) else {
                return false;
            };
            if mcrust::mcrust::modding::game::with_player(|player| WikiLookupManager::handle_wiki_lookup(&Either::right(hovered_stack.clone()), player, input)).unwrap_or(false) {
                return true;
            }
            // java: if (ItemPrice.ITEM_PRICE_LOOKUP.matches(input)) - the port's key mappings are asked by name
            if key_mapping::matches("key.skyblocker.itemPriceLookup", input) {
                mcrust::mcrust::modding::game::with_player(|player| {
                    ItemPrice::item_price_lookup_local_player_item_stack(player, &hovered_stack);
                });
                return true;
            }
        }
        return false;
    }
}
