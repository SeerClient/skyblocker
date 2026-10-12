//! Ported from de.hysky.skyblocker.skyblock.museum.MuseumManager by fabric2seer.

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
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page::RecipeBookPage;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
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

/// java: public class MuseumManager extends AbstractWidget implements HoveredItemStackProvider
pub struct MuseumManager {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ImageButton nextPageButton;
    pub next_page_button: Option<AbstractWidget>,
    /// java: private final ImageButton prevPageButton;
    pub prev_page_button: Option<AbstractWidget>,
    /// java: private final EditBox searchField;
    pub search_field: Option<EditBox>,
    /// java: private final List<Donation> filteredDonations = new ArrayList<>();
    pub filtered_donations: Vec<compat::Shared<Donation>>,
    /// java: private final List<String> excludedDonationIds = new ArrayList<>();
    pub excluded_donation_ids: Vec<String>,
    /// java: private final List<DonationButton> donationButtons = Lists.newArrayListWithCapacity(BUTTONS_PER_PAGE);
    pub donation_buttons: Vec<compat::Shared<DonationButton>>,
    /// java: private final Button filterButton;
    pub filter_button: Option<AbstractWidget>,
    /// java: private final Button sortButton;
    pub sort_button: Option<AbstractWidget>,
    /// java: private @Nullable DonationButton hoveredDonationButton;
    pub hovered_donation_button: Option<compat::Shared<DonationButton>>,
    /// java: private int pageCount = 0;
    pub page_count: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for MuseumManager {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MuseumManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MuseumManager {
    /// java: public MuseumManager(Screen screen, int x, int y, int backgroundWidth)
    pub fn new(mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut x: i32, mut y: i32, mut background_width: i32) -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `donationButtons`: static call Lists.newArrayListWithCapacity(..)
        //   java: private final List<DonationButton> donationButtons = Lists.newArrayListWithCapacity(BUTTONS_PER_PAGE);
        let __this = compat::shared(Self {
            base: AbstractWidget::new((x.wrapping_add(background_width)).wrapping_add(SPACING), y, BACKGROUND_WIDTH, BACKGROUND_HEIGHT, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            next_page_button: None,
            prev_page_button: None,
            search_field: None,
            filtered_donations: Vec::new(),
            excluded_donation_ids: Vec::new(),
            donation_buttons: Default::default(),
            filter_button: None,
            sort_button: None,
            hovered_donation_button: None,
            page_count: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
        // rust: { let __v = Some(EditBox::new(TEXT_RENDERER.get_mut(), component::empty(), (__this.lock().borrow().get_x()).wrapping_add(25), (__this.lock().borrow().get_y()).wrapping_add(11), 69, 20)); compat::write(&__this, |__t| __t.search_field = __v); }
        __this.lock().borrow_mut().search_field.as_mut().unwrap().set_max_length(60);
        // TODO(fabric2seer): `EditBox.setVisible` has no Rust method in the port (looked for `set_visible` in EditBox and its supertypes)
        // rust: __this.lock().borrow().search_field.as_ref().unwrap().set_visible(true);
        __this.lock().borrow_mut().search_field.as_mut().unwrap().set_text_color(mcrust::net::minecraft::util::common_colors::WHITE);
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: { let (__q0,) = (&compat::get_state(&SEARCH_QUERY),); __this.lock().borrow_mut().search_field.as_mut().unwrap().set_value(__q0) };
        { let (__q0,) = (component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray),); __this.lock().borrow_mut().search_field.as_mut().unwrap().set_hint_component(__q0) };
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_FORWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        // rust: { let __v = Some(AbstractWidget::new((__this.lock().borrow().get_x()).wrapping_add(93), (__this.lock().borrow().get_y()).wrapping_add(133), 12, 17, RecipeBookPage::PAGE_FORWARD_SPRITES.clone(), Arc::new(move |_| {
        // rust: 
        // rust: }))); compat::write(&__this, |__t| __t.next_page_button = __v); }
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_BACKWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        // rust: { let __v = Some(AbstractWidget::new((__this.lock().borrow().get_x()).wrapping_add(38), (__this.lock().borrow().get_y()).wrapping_add(133), 12, 17, RecipeBookPage::PAGE_BACKWARD_SPRITES.clone(), Arc::new(move |_| {
        // rust: 
        // rust: }))); compat::write(&__this, |__t| __t.prev_page_button = __v); }
        *DONATIONS.get_mut() = MuseumItemCache::get_donations();
        ITEM_FILTER.get_mut().update_categories();
        for i in 0..BUTTONS_PER_PAGE {
            let mut button: compat::Shared<DonationButton> = DonationButton::new(((__this.lock().borrow().get_x()).wrapping_add(11)).wrapping_add(31i32.wrapping_mul(i.wrapping_rem(4))), ((__this.lock().borrow().get_y()).wrapping_add(34)).wrapping_add(31i32.wrapping_mul(i.wrapping_div(4))));
            let __h0 = button.clone();
            __this.lock().borrow_mut().donation_buttons.push(__h0);
        }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            // java: ITEM_SORTER.cycleSortMode(filteredDonations); - sorts the list in place
            compat::write(&__this_1, |__t| ITEM_SORTER.get_mut().cycle_sort_mode(&mut __t.filtered_donations));
            (__screen.widget_mut(__index)).set_tooltip(ITEM_SORTER.get_tooltip());
            compat::set_state(&CURRENT_PAGE, 0);
            MuseumManager::update_buttons(&__this_1);
        } })))).tooltip_text(Some(ITEM_SORTER.get_tooltip().message.clone())).pos((__this.lock().borrow().get_x()).wrapping_add(95), (__this.lock().borrow().get_y()).wrapping_add(11)).size(BUTTON_SIZE, BUTTON_SIZE).build()); compat::write(&__this, |__t| __t.sort_button = __v); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((button::builder(component::empty().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
            // java: ITEM_FILTER.cycleFilterMode(donations, filteredDonations); ITEM_SORTER.applySort(filteredDonations); - in place
            compat::write(&__this_2, |__t| ITEM_FILTER.get_mut().cycle_filter_mode(DONATIONS.get(), &mut __t.filtered_donations));
            compat::write(&__this_2, |__t| ITEM_SORTER.get().apply_sort(&mut __t.filtered_donations));
            (__screen.widget_mut(__index)).set_tooltip(ITEM_FILTER.get_tooltip());
            compat::set_state(&CURRENT_PAGE, 0);
            MuseumManager::update_buttons(&__this_2);
        } })))).tooltip_text(Some(ITEM_FILTER.get_tooltip().message.clone())).pos((__this.lock().borrow().get_x()).wrapping_add(116), (__this.lock().borrow().get_y()).wrapping_add(11)).size(BUTTON_SIZE, BUTTON_SIZE).build()); compat::write(&__this, |__t| __t.filter_button = __v); }
        // java: ITEM_FILTER.applyFilter(donations, filteredDonations); ITEM_SORTER.applySort(filteredDonations); - in place
        compat::write(&__this, |__t| ITEM_FILTER.get().apply_filter(DONATIONS.get(), &mut __t.filtered_donations));
        compat::write(&__this, |__t| ITEM_SORTER.get().apply_sort(&mut __t.filtered_donations));
        MuseumManager::update_search_results(&__this, false);
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(screen).add(this);
        // TODO(fabric2seer): `Screen.setFocused` has no Rust method in the port (looked for `set_focused` in Screen and its supertypes)
        // rust: screen.set_focused(&__this);
        __this
    }

    /// java: protected static @Nullable Donation getDonation(String id)
    pub fn get_donation(id: &str) -> Option<compat::Shared<Donation>> {
        return DONATIONS.get_mut().iter().cloned().filter(|donation| (Donation::get_id(donation) == id)).next();
    }

    /// java: private void updateNavigationButtons()
    pub fn update_navigation_buttons(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().prev_page_button.as_mut().unwrap().active = (compat::get_state(&CURRENT_PAGE) > 0);
        let __r0 = compat::read(__this, |__t| __t.page_count);
        __this.lock().borrow_mut().next_page_button.as_mut().unwrap().active = (compat::get_state(&CURRENT_PAGE) < (__r0).wrapping_sub(1));
    }

    /// java: private void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut visible_donations: Vec<compat::Shared<Donation>> = compat::read(__this, |__t| __t.filtered_donations.clone()).into_iter().filter(|donation| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; !((&__this_1).lock().borrow().excluded_donation_ids.iter().any(|e| *e == Donation::get_id(donation))) }).collect::<Vec<_>>();
        let mut buttons_size: i32 = (visible_donations.len() as i32);
        { let __v = (((((buttons_size) as f64) / (BUTTONS_PER_PAGE as f64)).ceil()) as i32); compat::write(__this, |__t| __t.page_count = __v); }
        for i in 0..(__this.lock().borrow().donation_buttons.len() as i32) {
            let mut index: i32 = ((compat::get_state(&CURRENT_PAGE)).wrapping_mul(__this.lock().borrow().donation_buttons.len() as i32)).wrapping_add(i);
            if index < buttons_size {
                DonationButton::init(&__this.lock().borrow().donation_buttons[(i) as usize], visible_donations[(index) as usize].clone());
            } else {
                DonationButton::reset_button(&__this.lock().borrow().donation_buttons[(i) as usize]);
            }
        }
        MuseumManager::update_navigation_buttons(__this);
    }

    /// java: public void updateSearchResults(boolean resetPage)
    pub fn update_search_results(__this: &compat::Shared<Self>, mut reset_page: bool) {
        compat::set_state(&SEARCH_QUERY, __this.lock().borrow().search_field.as_ref().unwrap().get_value());
        __this.lock().borrow_mut().excluded_donation_ids.clear();
        for item in DONATIONS.get_mut().iter() {
            let mut searchable_content: String = String::new();
            let mut item_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&Donation::get_id(item));
            if let Some(item_stack) = &item_stack {
                { searchable_content.push_str(&compat::component_of(FlexibleItemStack::get_stack_or_throw(item_stack).get_hover_name()).get_string()); searchable_content.push_str(&ItemUtils::get_concatenated_lore(&FlexibleItemStack::get_stack_or_throw(item_stack))); };
            }
            if true /* java: Donation::get_set(item) != null: never null here */ && !(Donation::get_set(item).is_empty()) {
                for piece in Donation::get_set(item) {
                    let mut piece_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&piece.0);
                    if let Some(piece_stack) = &piece_stack {
                        { searchable_content.push_str(&compat::component_of(FlexibleItemStack::get_stack_or_throw(piece_stack).get_hover_name()).get_string()); searchable_content.push_str(&ItemUtils::get_concatenated_lore(&FlexibleItemStack::get_stack_or_throw(piece_stack))); };
                    }
                }
            }
            if !(searchable_content.clone().to_lowercase().contains(&compat::lock(&SEARCH_QUERY).to_lowercase())) {
                let __h0 = Donation::get_id(item);
                __this.lock().borrow_mut().excluded_donation_ids.push(__h0);
            }
        }
        if reset_page {
            compat::set_state(&CURRENT_PAGE, 0);
        }
        MuseumManager::update_buttons(__this);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().search_field.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        if __this.lock().borrow().sort_button.as_ref().unwrap().active {
            let mut icon_x: i32 = (__this.lock().borrow().sort_button.as_ref().unwrap().get_x()).wrapping_add(((__this.lock().borrow().sort_button.as_ref().unwrap().get_width()).wrapping_sub(16)).wrapping_div(2));
            let mut icon_y: i32 = (__this.lock().borrow().sort_button.as_ref().unwrap().get_y()).wrapping_add(((__this.lock().borrow().sort_button.as_ref().unwrap().get_height()).wrapping_sub(16)).wrapping_div(2));
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = ITEM_SORTER.get_current_sorting_item();
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().sort_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
            graphics.fake_item(&stack, icon_x, icon_y);
        }
        if __this.lock().borrow().filter_button.as_ref().unwrap().active {
            let mut icon_x: i32 = (__this.lock().borrow().filter_button.as_ref().unwrap().get_x()).wrapping_add(((__this.lock().borrow().filter_button.as_ref().unwrap().get_width()).wrapping_sub(16)).wrapping_div(2));
            let mut icon_y: i32 = (__this.lock().borrow().filter_button.as_ref().unwrap().get_y()).wrapping_add(((__this.lock().borrow().filter_button.as_ref().unwrap().get_height()).wrapping_sub(16)).wrapping_div(2));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().filter_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
            graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(ico::HOPPER.get_mut()), icon_x, icon_y);
        }
        if ItemRepository::files_imported() {
            if compat::read(__this, |__t| __t.page_count) > 1 {
                let mut text: MutableComponent = component::translatable_args("gui.recipebook.page", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((compat::get_state(&CURRENT_PAGE)).wrapping_add(1).to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.page_count).to_string())]);
                let mut width: i32 = TEXT_RENDERER.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), ((__this.lock().borrow().get_x()).wrapping_sub(width.wrapping_div(2))).wrapping_add(73), (__this.lock().borrow().get_y()).wrapping_add(137), -1, false);
            }
            { let __v = None; compat::write(__this, |__t| __t.hovered_donation_button = __v); }
            for result_button in compat::read(__this, |__t| __t.donation_buttons.clone()) {
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: let __wa1 = graphics.clone();
                // rust: let __wa2 = mouse_x;
                // rust: let __wa3 = mouse_y;
                // rust: let __wa4 = a;
                // rust: let __o1 = result_button.clone();
                // rust: compat::write(&__o1, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
                let __o2 = result_button.clone();
                let __o3 = result_button.clone();
                if compat::read(&result_button, |__t| __t.visible) && compat::write(&__o3, |__t| __t.is_hovered()) {
                    { let __v = Some(result_button.clone()); compat::write(__this, |__t| __t.hovered_donation_button = __v); }
                }
            }
            if __this.lock().borrow().prev_page_button.as_ref().unwrap().active {
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().prev_page_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
            }
            if __this.lock().borrow().next_page_button.as_ref().unwrap().active {
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().next_page_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
            }
            MuseumManager::extract_tooltip(__this, graphics, mouse_x, mouse_y);
        } else {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Loading...", (__this.lock().borrow().get_x()).wrapping_add(BACKGROUND_WIDTH.wrapping_div(2)), (__this.lock().borrow().get_y()).wrapping_add(BACKGROUND_HEIGHT.wrapping_div(2)), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        if compat::read(__this, |__t| __t.hovered_donation_button.clone()).is_some() {
            let mut tooltip: Vec<MutableComponent> = DonationButton::get_item_tooltip(&compat::read(__this, |__t| __t.hovered_donation_button.clone()).unwrap());
            // java: Identifier tooltipStyle = hoveredDonationButton.getItem().get(DataComponents.TOOLTIP_STYLE); (nullable)
            let mut tooltip_style: Option<Identifier> = (DonationButton::get_item(&compat::read(__this, |__t| __t.hovered_donation_button.clone()).unwrap())).get(mcrust::net::minecraft::core::component::data_components::tooltip_style()).and_then(|o| mcrust::com::mojang::serialization::cast::<Identifier>(&o).cloned());
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame_styled` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo, style)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame_styled` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 5
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame_styled(TEXT_RENDERER.get_mut(), tooltip.clone(), x, y, tooltip_style.clone());
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        // TODO(fabric2seer): `AbstractWidget::mouse_clicked` ports `mouseClicked`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // java: if (searchField.mouseClicked(click, doubled)) { searchField.setFocused(true); return true; }
        // rust: port gap (GAPS_batchA #3) - the port's EditBox answers a click through `on_click(splitter, ..)` with its focus
        // and active state held by the screen's widget tree; a mod widget's own EditBox field has neither, so its click is not tested
        let search_field_clicked: bool = false;
        if search_field_clicked {
            __this.lock().borrow_mut().search_field.as_mut().unwrap().set_focused_at(true, mcrust::net::minecraft::util::util::get_millis());
            return true;
        } else if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().next_page_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() || { let _ = __q1; false } } {
            compat::set_state(&CURRENT_PAGE, compat::get_state(&CURRENT_PAGE) + 1);
            MuseumManager::update_buttons(__this);
            return true;
        } else if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().prev_page_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() || { let _ = __q1; false } } {
            compat::set_state(&CURRENT_PAGE, compat::get_state(&CURRENT_PAGE) - 1);
            MuseumManager::update_buttons(__this);
            return true;
        } else if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().filter_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() || { let _ = __q1; false } } {
            // rust: port gap (GAPS_batchB L3) - Java's mouseClicked also runs the button's onPress; the port's OnPress::Lambda
            // runs against the screen that holds the button, and this button is a field of a mod widget, not a screen widget
            return true;
        } else if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().sort_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() || { let _ = __q1; false } } {
            // rust: port gap (GAPS_batchB L3) - the onPress lambda is not run (see the filter button above)
            return true;
        }
        // java: searchField.setFocused(false);
        __this.lock().borrow_mut().search_field.as_mut().unwrap().set_focused_at(false, mcrust::net::minecraft::util::util::get_millis());
        return false;
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    pub fn char_typed(__this: &compat::Shared<Self>, input: &CharacterEvent) -> bool {
        // java: if (searchField.charTyped(input)) {..}
        // rust: port gap (GAPS_batchA #3) - EditBox.charTyped takes the font's splitter and the focus / active state the
        // screen's widget tree holds; this EditBox is a field of a mod widget
        let search_field_typed: bool = { let _ = input; false };
        if search_field_typed {
            MuseumManager::update_search_results(__this, true);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // java: if (searchField.canConsumeInput() && CLIENT.options.keyInventory.matches(input) || searchField.keyPressed(input)) {..}
        // rust: port gap (GAPS_batchA #3) - canConsumeInput / keyPressed need the focus / active state (and keyPressed the
        // splitter and clipboard) that the screen's widget tree holds; this EditBox is a field of a mod widget
        let search_field_consumes: bool = false;
        if (search_field_consumes && key_mapping::matches("key.inventory", input)) || search_field_consumes {
            MuseumManager::update_search_results(__this, true);
            return true;
        }
        if compat::read(__this, |__t| __t.hovered_donation_button.clone()).is_some() {
            let Some(mut hovered_stack) = DonationButton::get_display_stack(&compat::read(__this, |__t| __t.hovered_donation_button.clone()).unwrap()) else {
                return false;
            };
            if mcrust::mcrust::modding::game::with_player(|player| WikiLookupManager::handle_wiki_lookup(&Either::right(hovered_stack.clone()), player, input)).unwrap_or(false) {
                return true;
            }
            if item_price::ITEM_PRICE_LOOKUP.get_mut().matches(input) {
                mcrust::mcrust::modding::game::with_player(|player| { ItemPrice::item_price_lookup_local_player_item_stack(player, &hovered_stack); });
                return true;
            }
        }
        return false;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
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

impl MuseumManager {
    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        if compat::read(__this, |__t| __t.hovered_donation_button.clone()).is_none() {
            return None;
        }
        return DonationButton::get_display_stack(&compat::read(__this, |__t| __t.hovered_donation_button.clone()).unwrap());
    }

    /// MuseumManager.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// MuseumManager's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for MuseumManager {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// MuseumManager as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<MuseumManager> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| MuseumManager::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| MuseumManager::update_widget_narration(__h, output))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(widget, |__h| MuseumManager::mouse_clicked(__h, event, double_click))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(widget, |__h| MuseumManager::key_pressed(__h, event))
    }

    /// java: @Override public boolean charTyped(CharacterEvent input)
    fn char_typed(&mut self, widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        self.lend(widget, |__h| MuseumManager::char_typed(__h, event))
    }

}

// NOTE(fabric2seer): charTyped, extractWidgetRenderState, keyPressed, mouseClicked, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
