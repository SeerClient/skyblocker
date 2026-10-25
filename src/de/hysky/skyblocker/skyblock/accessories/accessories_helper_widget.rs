//! Ported from de.hysky.skyblocker.skyblock.accessories.AccessoriesHelperWidget by fabric2seer.
//!
//! The widget is ONE object (`compat::ObjectWidget`, batch E pattern 7): Java's `this` is what the container screen's
//! widget list holds (`Screens.getWidgets(screen).add(widget)`), and its own widgets (`widgets`: the twenty result
//! buttons, the two page arrows, the page text and the two cycle buttons, in `mainLayout.visitWidgets` order) are the
//! game's widgets in a `Screen` the object owns (`children`), which it draws, hovers and clicks. Its `FrameLayout` is
//! rebuilt from the children's live sizes whenever Java arranges it (batch E pattern 9). A child's call back into this
//! widget (`changePage` from an arrow, `updateFilter(); changePage(0)` from a cycle button) is recorded and run as the
//! widget's own call returns (batch E pattern 8).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_container_solver;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::AccessoriesHelper;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::Accessory;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::line_smoothener::LineSmoothener;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip::ItemTooltip;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_result_button::SkyblockRecipeResultButton;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_result_button::SkyblockRecipeResultButtonBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use compat::ObjectWidget as _;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::loading_dots_text;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_tab_button;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::util::util;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("background");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("background"));
/// java: private static final int BORDER_SIZE = 8;
pub const BORDER_SIZE: i32 = 8;
/// java: private static final int BUTTON_COUNT = 20;
pub const BUTTON_COUNT: i32 = 20;
/// java: private static Filter filter = Filter.ALL;
pub static FILTER: LazyLock<Mutex<Filter>> = LazyLock::new(|| Mutex::new(Filter::All));
/// java: private static int page;
pub static PAGE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static boolean open;
pub static OPEN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean showHighestTierOnly;
pub static SHOW_HIGHEST_TIER_ONLY: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final Component smoothLine = LineSmoothener.createSmoothLine();
pub static SOURCE_SMOOTH_LINE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| LineSmoothener::create_smooth_line());
/// java: private static final Component wikiLine = Component.translatable("skyblocker.accessoryHelper.openWiki").withStyle(ChatFormatting.YELLOW);
pub static SOURCE_WIKI_LINE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.accessoryHelper.openWiki").with_style_format(ChatFormatting::Yellow));

/// `children`' index of `prevPageButton` (`buttons` are 0..20, in order)
const PREV_PAGE_BUTTON: usize = 20;
/// `children`' index of `pageText`
const PAGE_TEXT: usize = 21;
/// `children`' index of `nextPageButton`
const NEXT_PAGE_BUTTON: usize = 22;
/// `children`' index of the filter `CycleButton`
const FILTER_BUTTON: usize = 23;
/// `children`' index of the highest-tier-only `CycleButton`
const TIER_BUTTON: usize = 24;

thread_local! {
    /// `((AbstractContainerScreenAccessor) screen).setX(x)` from the tab button's press: the press runs inside the
    /// game's widget pass, where the container screen is not lent, so the write waits for the screen's next lent call
    /// (`install_screen_hooks`; the screen draws at the new position from the next frame on, as Java's does).
    static PENDING_LEFT_POS: Cell<Option<i32>> = const { Cell::new(None) };
    /// `AccessoriesHelper`'s AFTER_INIT listener called `attachToScreen(screen)` for the screen being initialised.
    static ATTACH_REQUESTED: Cell<bool> = const { Cell::new(false) };
}

/// A call a child makes into the enclosing widget (`AccessoriesHelperWidget.this`) while the widget hands it the
/// input; run, in order, as the widget's call returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildCall {
    /// java: `changePage(offset)` (an `ArrowButton.onClick`)
    ChangePage(i32),
    /// java: `updateFilter(); changePage(0);` (a cycle button's value listener)
    FilterChanged,
}

/// java: class AccessoriesHelperWidget extends AbstractContainerWidget implements HoveredItemStackProvider
pub struct AccessoriesHelperWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget - the object's widget fields (`ObjectWidget`)
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> widgets; - the game's widgets, at the indices above
    pub children: Screen,
    /// java: private final List<ResultButton> buttons = new ArrayList<>(BUTTON_COUNT);
    pub buttons: Vec<compat::Shared<ResultButton>>,
    /// java: private final FrameLayout layout;
    pub layout: FrameLayout,
    /// java: private final ArrowButton prevPageButton = new ArrowButton(false);
    pub prev_page_button: compat::Shared<ArrowButton>,
    /// java: private final ArrowButton nextPageButton = new ArrowButton(true);
    pub next_page_button: compat::Shared<ArrowButton>,
    /// java: private List<AccessoryInfo> accessories = List.of();
    pub accessories: Vec<AccessoryInfo>,
    /// java: private List<MagicPowerSource> displays = List.of();
    pub displays: Vec<compat::Shared<dyn MagicPowerSource>>,
    /// java: private List<RecombobulateSource> recombDisplays = List.of();
    pub recomb_displays: Vec<compat::Shared<RecombobulateSource>>,
    /// java: private boolean refreshWhenDoneLoading = false;
    pub refresh_when_done_loading: bool,
    /// the children's calls into `this` (see `ChildCall`)
    pub child_calls: Rc<RefCell<Vec<ChildCall>>>,
}

impl AccessoriesHelperWidget {
    /// java: AccessoriesHelperWidget()
    pub fn new() -> compat::Shared<Self> {
        let child_calls: Rc<RefCell<Vec<ChildCall>>> = Rc::new(RefCell::new(Vec::new()));
        // java: super(0, 0, 147, 182, CommonComponents.EMPTY, AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(0, 0, 147, 182, FormattedText::from(component::empty()), ScrollbarSettings::default_settings(4.0));
        // java: this.layout = new FrameLayout(getWidth() - BORDER_SIZE * 2, getHeight() - BORDER_SIZE * 2);
        let layout = FrameLayout::of((base.get_width()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)), (base.get_height()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)));
        // java: private final StringWidget pageText = new StringWidget(CommonComponents.EMPTY, Minecraft.getInstance().font).setMaxWidth(30, StringWidget.TextOverflow.SCROLLING);
        let mut page_text: AbstractWidget = compat::string_widget(0, 0, -1, 9, component::empty());
        page_text.set_max_width(30, string_widget::TextOverflow::Scrolling);
        // java: prevPageButton = new ArrowButton(false); nextPageButton = new ArrowButton(true);
        let prev_page_button = ArrowButton::new(false, child_calls.clone());
        let next_page_button = ArrowButton::new(true, child_calls.clone());

        // java: for (int i = 0; i < BUTTON_COUNT; i++) { ResultButton button = new ResultButton(); buttons.add(button); adder.addChild(button); }
        let mut children = Screen::new(String::new());
        let mut buttons: Vec<compat::Shared<ResultButton>> = Vec::with_capacity(BUTTON_COUNT as usize);
        for _ in 0..BUTTON_COUNT {
            let button: compat::Shared<ResultButton> = ResultButton::new();
            children.add_renderable_widget(compat::WidgetAdapter::widget(button.clone()));
            buttons.push(button);
        }
        // java: pageSwitcher.addChild(prevPageButton); pageSwitcher.addChild(new FrameLayout(30, 0)).addChild(pageText); pageSwitcher.addChild(nextPageButton);
        children.add_renderable_widget(compat::WidgetAdapter::widget(prev_page_button.clone()));
        children.add_renderable_widget(page_text);
        children.add_renderable_widget(compat::WidgetAdapter::widget(next_page_button.clone()));

        // java: int filterWidth = layout.getWidth() - 2;
        let filter_width: i32 = (layout.width).wrapping_sub(2);
        // java: mainLayout.addChild(CycleButton.builder(f -> Component.translatable(f.toString()), filter)
        // java: 		.withValues(Filter.values())
        // java: 		.create(0, 0, filterWidth, 16, Component.translatable("skyblocker.accessoryHelper.filter"), (_, v) -> {
        // java: 			filter = v;
        // java: 			updateFilter();
        // java: 			changePage(0);
        // java: 		}));
        let filter_label = |f: &Filter| component::translatable(&f.to_string()).get_string();
        let filter_calls = child_calls.clone();
        let filter_button = compat::cycle_button_widget(
            0,
            0,
            filter_width,
            16,
            &component::translatable("skyblocker.accessoryHelper.filter"),
            cycle_button::builder(filter_label(&compat::get_state(&FILTER))).with_values(Filter::values().iter().map(filter_label).collect()),
            move |index: usize| {
                compat::set_state(&FILTER, Filter::values()[index]);
                filter_calls.borrow_mut().push(ChildCall::FilterChanged);
            },
        );
        children.add_renderable_widget(filter_button);
        // java: mainLayout.addChild(CycleButton.booleanBuilder(Component.translatable("skyblocker.accessoryHelper.highestTierOnly"), Component.translatable("skyblocker.accessoryHelper.allTiers"), showHighestTierOnly)
        // java: 		.displayOnlyValue()
        // java: 		.create(0, 0, filterWidth, 16, CommonComponents.EMPTY, (_, value) -> {
        // java: 			showHighestTierOnly = value;
        // java: 			updateFilter();
        // java: 			changePage(0);
        // java: 		}));
        // (`BOOLEAN_OPTIONS` is `[TRUE, FALSE]`: index 0 is `true`)
        let tier_calls = child_calls.clone();
        let tier_button = compat::cycle_button_widget(
            0,
            0,
            filter_width,
            16,
            &component::empty(),
            cycle_button::boolean_builder(component::translatable("skyblocker.accessoryHelper.highestTierOnly").get_string(), component::translatable("skyblocker.accessoryHelper.allTiers").get_string(), compat::get_state(&SHOW_HIGHEST_TIER_ONLY)).display_only_value(),
            move |index: usize| {
                compat::set_state(&SHOW_HIGHEST_TIER_ONLY, index == 0);
                tier_calls.borrow_mut().push(ChildCall::FilterChanged);
            },
        );
        children.add_renderable_widget(tier_button);

        let mut this = AccessoriesHelperWidget {
            base,
            children,
            buttons,
            layout,
            prev_page_button,
            next_page_button,
            accessories: Vec::new(),
            displays: Vec::new(),
            recomb_displays: Vec::new(),
            refresh_when_done_loading: false,
            child_calls,
        };
        // java: mainLayout.arrangeElements(); updatePageSwitcher();
        this.update_page_switcher();
        compat::shared(this)
    }

    /// java: the constructor's layout - `layout.addChild(LinearLayout.vertical())` holding the button grid (5 columns),
    /// the page switcher (`LinearLayout.horizontal().spacing(2)`, padded 4 vertically) and the two cycle buttons -
    /// built over the children's live sizes (the port's layouts hold a widget's size as it was added)
    fn build_layout(&self) -> FrameLayout {
        let element = |index: usize| {
            let widget = self.children.widget(index);
            LayoutElement::widget(index, widget.get_width(), widget.get_height())
        };
        // java: this.layout = new FrameLayout(getWidth() - BORDER_SIZE * 2, getHeight() - BORDER_SIZE * 2);
        let mut layout = FrameLayout::of((self.base.get_width()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)), (self.base.get_height()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)));
        // java: LinearLayout mainLayout = layout.addChild(LinearLayout.vertical());
        let mut main_layout: LinearLayout = LinearLayout::vertical();
        // java: mainLayout.defaultCellSetting().alignHorizontallyCenter();
        { let __s = main_layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        // java: GridLayout.RowHelper adder = mainLayout.addChild(new GridLayout()).createRowHelper(5);
        let mut grid = GridLayout::new_default();
        let mut adder = grid.create_row_helper(5);
        for i in 0..(BUTTON_COUNT as usize) {
            // java: adder.addChild(button);
            let _ = adder.add_child(&mut grid, element(i));
        }
        main_layout.add_child(LayoutElement::Grid(grid));
        // java: LinearLayout pageSwitcher = mainLayout.addChild(LinearLayout.horizontal().spacing(2), p -> p.paddingVertical(4));
        let mut page_switcher: LinearLayout = LinearLayout::horizontal().spacing(2);
        // java: pageSwitcher.defaultCellSetting().alignVerticallyMiddle();
        { let __s = page_switcher.default_cell_setting(); *__s = (*__s).align_vertically_middle(); }
        // java: pageSwitcher.addChild(prevPageButton);
        page_switcher.add_child(element(PREV_PAGE_BUTTON));
        // java: pageSwitcher.addChild(new FrameLayout(30, 0)).addChild(pageText);
        let mut page_text_frame = FrameLayout::of(30, 0);
        page_text_frame.add_child(element(PAGE_TEXT));
        page_switcher.add_child(LayoutElement::Frame(page_text_frame));
        // java: pageSwitcher.addChild(nextPageButton);
        page_switcher.add_child(element(NEXT_PAGE_BUTTON));
        main_layout.add_child_adjusted(LayoutElement::Grid(page_switcher.wrapped), &mut |p| { *p = p.padding_vertical(4); });
        // java: mainLayout.addChild(<filter cycle button>); mainLayout.addChild(<highest tier cycle button>);
        main_layout.add_child(element(FILTER_BUTTON));
        main_layout.add_child(element(TIER_BUTTON));
        layout.add_child(LayoutElement::Grid(main_layout.wrapped));
        layout
    }

    /// java: `layout.arrangeElements()` at `layout.setX/setY(x/y + BORDER_SIZE)`: the layout rebuilt over the live sizes,
    /// arranged where the widget is, and the positions written onto the children
    fn arrange(&mut self) {
        let mut layout = self.build_layout();
        layout.x = (self.base.get_x()).wrapping_add(BORDER_SIZE);
        layout.y = (self.base.get_y()).wrapping_add(BORDER_SIZE);
        layout.arrange_elements();
        let mut positions: Vec<(usize, i32, i32)> = Vec::new();
        layout.visit_widgets(&mut positions);
        for (index, x, y) in positions {
            self.children.widget_mut(index).set_position(x, y);
        }
        self.layout = layout;
    }

    /// java: static void attachToScreen(ContainerScreen screen) - the container screen whose `init` is returning
    pub fn attach_to_screen(screen: &mut AbstractContainerScreen) {
        if !(compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_accessories_helper)) || !(SkyblockerConfigManager::get().helpers.enable_accessories_helper_widget) {
            return;
        }
        let widget: compat::Shared<AccessoriesHelperWidget> = AccessoriesHelperWidget::new();
        // java: widget.setY((screen.height - widget.getHeight()) / 2);
        let height: i32 = compat::read(&widget, |__t| __t.base.get_height());
        let screen_height: i32 = screen.screen.height;
        compat::write(&widget, |__t| __t.set_y((screen_height.wrapping_sub(height)).wrapping_div(2)));
        // java: Screens.getWidgets(screen).add(widget);
        screen.screen.add_renderable_widget(compat::object_widget(widget.clone(), ""));
        // java: final int previousX = ((AbstractContainerScreenAccessor) screen).getX();
        let previous_x: i32 = screen.left_pos;
        // java: final int offset = Math.max(180 - previousX, 0);
        let offset: i32 = (180i32.wrapping_sub(previous_x)).max(0);
        // `((AbstractContainerScreenAccessor) screen).getY()` - the press reads it; nothing between here and a re-init
        // (which attaches a new widget) writes it
        let top_pos: i32 = screen.top_pos;
        let tab_button: compat::Shared<TabButton> = TabButton::new(Arc::new(move |button: compat::Shared<TabButton>| {
            let toggled: bool = compat::read(&button, |__t| __t.toggled);
            // java: widget.visible = open = toggled;
            compat::set_state(&OPEN, toggled);
            compat::write(&widget, |__t| __t.base.visible = toggled);
            // java: int x = toggled ? previousX + offset : previousX;
            let x: i32 = if toggled { previous_x.wrapping_add(offset) } else { previous_x };
            // java: ((AbstractContainerScreenAccessor) screen).setX(x);
            PENDING_LEFT_POS.with(|p| p.set(Some(x)));
            // java: widget.setX(x - widget.getWidth() - 2);
            let width: i32 = compat::read(&widget, |__t| __t.base.get_width());
            compat::write(&widget, |__t| __t.set_x((x.wrapping_sub(width)).wrapping_sub(2)));
            // java: button.setX((toggled ? widget.getX() : x) - button.getWidth() + 5);
            let (widget_x, widget_y) = compat::read(&widget, |__t| (__t.base.get_x(), __t.base.get_y()));
            let button_width: i32 = compat::read(&button, |__t| __t.base.get_width());
            compat::write(&button, |__t| __t.base.set_x(((if toggled { widget_x } else { x }).wrapping_sub(button_width)).wrapping_add(5)));
            // java: button.setY((toggled ? widget.getY() : ((AbstractContainerScreenAccessor) screen).getY()) + 8);
            compat::write(&button, |__t| __t.base.set_y((if toggled { widget_y } else { top_pos }).wrapping_add(8)));
            if toggled {
                compat::write(&widget, |__t| __t.refresh_data());
            } else {
                // Reset page when you close the helper. keep rest for UX
                compat::set_state(&PAGE, 0);
            }
        }));
        // java: Screens.getWidgets(screen).add(tabButton);
        screen.screen.add_renderable_widget(compat::WidgetAdapter::widget(tab_button.clone()));
        // java: tabButton.setToggled(open);
        TabButton::set_toggled(&tab_button, compat::get_state(&OPEN));
        // the screen is lent here: its `setX` lands at once
        if let Some(x) = PENDING_LEFT_POS.with(|p| p.take()) {
            screen.left_pos = x;
        }
    }

    /// `AccessoriesHelper`'s AFTER_INIT listener: `attachToScreen(genericContainerScreen)` for the screen being
    /// initialised (attached as its `AbstractContainerScreen.init` returns, see `install_screen_hooks`)
    pub fn request_attach() {
        ATTACH_REQUESTED.with(|r| r.set(true));
    }

    /// A screen's AFTER_INIT starts with no `attachToScreen` call of its own.
    pub fn cancel_attach() {
        ATTACH_REQUESTED.with(|r| r.set(false));
    }

    /// The two places the vanilla container screen object is lent to the mod: the RETURN of
    /// `AbstractContainerScreen.init` (where the AFTER_INIT listener that asked for `attachToScreen` has just run, so
    /// the widget is attached to that screen), and the HEAD of its `extractRenderState` (where a `setX` the tab button
    /// made since is written).
    pub fn install_screen_hooks() {
        use mcrust::mcrust::modding::hook::{self, At, CallbackInfo, InjectOptions};
        let attach = hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", At::Return, |ci: &mut CallbackInfo<'_>| {
            // a `setX` left from a previous screen is not this screen's
            PENDING_LEFT_POS.with(|p| p.set(None));
            if !ATTACH_REQUESTED.with(|r| r.replace(false)) {
                return;
            }
            if let Some(screen) = ci.this_mut::<AbstractContainerScreen>() {
                AccessoriesHelperWidget::attach_to_screen(screen);
            }
        });
        if let Err(e) = attach {
            mcrust::mcrust::modding::log::warn_once("skyblocker-accessories-attach", &format!("{e:?}"));
        }
        let set_x = hook::register_injector(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractContents",
            InjectOptions::at(At::Head).priority(900),
            Arc::new(|ci: &mut CallbackInfo<'_>| {
                let Some(x) = PENDING_LEFT_POS.with(|p| p.take()) else {
                    return;
                };
                if let Some(screen) = ci.find_arg_mut::<AbstractContainerScreen>() {
                    screen.left_pos = x;
                }
            }),
        );
        if let Err(e) = set_x {
            mcrust::mcrust::modding::log::warn_once("skyblocker-accessories-set-x", &format!("{e:?}"));
        }
    }

    /// java: private static OptionalDouble getPrice(Accessory acc)
    pub fn get_price(acc: &Accessory) -> Option<f64> {
        let Some(mut stack) = ItemRepository::get_item_stack(&acc.id()) else {
            return None;
        };
        let mut optional_price: Option<f64> = compat::read(&stack, |stack| ItemUtils::get_item_price(stack));
        let mut price: f64;
        if optional_price.is_some() {
            price = optional_price.clone().unwrap();
        } else {
            price = ItemUtils::get_craft_cost(&stack.lock().borrow().get_neu_name());
        }
        if price <= (0 as f64) {
            return None;
        }
        return Some(price);
    }

    /// java: private void refreshData()
    pub fn refresh_data(&mut self) {
        let mut collected_accessories: std::collections::HashSet<Accessory> = AccessoriesHelper::get_collected_accessories();
        // java: this.recombDisplays = collectedAccessories.stream().filter(Accessory::recombobulatable).map(Accessory::id)
        // java: 		.filter(id -> !AccessoriesHelper.isRecombobulated(id)).map(ItemRepository::getItemStack).filter(Objects::nonNull)
        // java: 		.map(SkyblockerStack::getSkyblockRarity).distinct().map(RecombobulateSource::new).toList();
        self.recomb_displays = compat::distinct(collected_accessories.iter().cloned().filter(|__a0| __a0.recombobulatable()).map(|__a0| __a0.id()).filter(|id| !(AccessoriesHelper::is_recombobulated(id))).filter_map(|__a0| ItemRepository::get_item_stack(&__a0)).map(|__a0| compat::read(&__a0, |__t| __t.get_skyblock_rarity()))).map(|__a0| RecombobulateSource::new(__a0)).collect::<Vec<_>>();
        // java: this.accessories = AccessoriesHelper.ACCESSORY_DATA.values().stream()
        // java: 		.filter(accessory -> NEURepoManager.isLoading() || !NEURepoManager.getConstants().getMisc().getIgnoredTalismans().contains(accessory.id())) // Removes admin and rift items
        // java: 		.map(accessory -> { .. }).distinct()
        // java: 		.filter(info -> info.accessory().tier() > info.highestOwned().map(Accessory::tier).orElse(-1))
        // java: 		.toList();
        let accessories: Vec<Accessory> = compat::lock(&accessories_helper::ACCESSORY_DATA).values().cloned().collect();
        let infos = accessories
            .into_iter()
            // Removes admin and rift items (`NEUConstants.getMisc().getIgnoredTalismans()`, the NEU repo port's)
            .filter(|accessory| NEURepoManager::is_loading() || !NEURepoManager::get_constants().get_misc().get_ignored_talismans().contains(&accessory.id()))
            .map(|accessory| {
                if accessory.family().is_some() {
                    let report = AccessoriesHelper::calculate_family_report(&accessory, &collected_accessories);
                    AccessoryInfo::new(accessory, report.highest_collected_in_family(), report.highest_in_family())
                } else {
                    let highest_owned: Option<Accessory> = if collected_accessories.contains(&accessory) { Some(accessory.clone()) } else { None };
                    AccessoryInfo::new(accessory.clone(), highest_owned, accessory)
                }
            });
        self.accessories = compat::distinct(infos).filter(|info| info.accessory().tier() > info.highest_owned().map(|__a0| __a0.tier()).unwrap_or(-1)).collect::<Vec<_>>();
        self.update_filter(); // Update items
        self.change_page(0); // Updates display
    }

    /// java: private void updateFilter()
    pub fn update_filter(&mut self) {
        let mut predicate: Arc<dyn Fn(&AccessoryInfo) -> bool> = match compat::get_state(&FILTER) {
            Filter::All => Arc::new(move |_: &AccessoryInfo| true),
            Filter::Missing => Arc::new(move |info: &AccessoryInfo| info.highest_owned().is_none()),
            Filter::Upgrades => Arc::new(move |info: &AccessoryInfo| (info.highest_owned().is_some() && info.accessory().tier() > info.highest_owned().unwrap().tier())),
        };
        let show_highest_tier_only: bool = compat::get_state(&SHOW_HIGHEST_TIER_ONLY);
        let stream = self
            .accessories
            .iter()
            .filter(|info| predicate(info))
            .filter(|info| !show_highest_tier_only || info.accessory().tier() >= info.highest_in_family().tier())
            .map(|info| Source::new(info.clone()) as compat::Shared<dyn MagicPowerSource>)
            .collect::<Vec<_>>();
        // java: displays = Stream.concat(stream, recombDisplays.stream()).sorted(Comparator.comparingDouble(MagicPowerSource::pricePerMp)).toList();
        self.displays = compat::sorted_by(stream.into_iter().chain(self.recomb_displays.iter().cloned().map(|r| r as compat::Shared<dyn MagicPowerSource>)), |a, b| { let key = |__a0: &compat::Shared<dyn MagicPowerSource>| ({ let __f = compat::dispatch(__a0, |__t| __t.price_per_mp()); __f(__a0) }); key(a).total_cmp(&key(b)) }).collect::<Vec<_>>();
    }

    /// java: private void changePage(int offset)
    pub fn change_page(&mut self, mut offset: i32) {
        // java: page = Math.clamp(page + offset, 0, getPageCount() - 1);
        compat::set_state(&PAGE, ((compat::get_state(&PAGE)).wrapping_add(offset)).clamp(0, (self.get_page_count()).wrapping_sub(1)));
        self.update_page_switcher();
        for i in 0..BUTTON_COUNT {
            let mut j: i32 = i.wrapping_add((compat::get_state(&PAGE)).wrapping_mul(BUTTON_COUNT));
            let button = self.buttons[i as usize].clone();
            if j < (self.displays.len() as i32) {
                ResultButton::set_source(&button, self.displays[j as usize].clone());
            } else {
                let __f = compat::dispatch(&button, |__t| __t.clear_display_stack());
                __f(&compat::view(&button, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>));
            }
            // the button's `active` / `visible` (which `setDisplayStack` / `clearDisplayStack` set) on the game's widget it is
            let (active, visible) = compat::read(&button, |__t| (__t.active, __t.visible));
            let widget = self.children.widget_mut(i as usize);
            widget.active = active;
            widget.visible = visible;
        }
    }

    /// java: private int getPageCount()
    pub fn get_page_count(&self) -> i32 {
        // Math.ceilDiv(x, y) = -Math.floorDiv(-x, y)
        return 1.max(compat::floor_div_i32(-(self.displays.len() as i32), BUTTON_COUNT).wrapping_neg());
    }

    /// java: private void updatePageSwitcher()
    pub fn update_page_switcher(&mut self) {
        // java: pageText.setMessage(Component.translatable("gui.recipebook.page", page + 1, getPageCount()));
        let message = FormattedText::from(component::translatable_args("gui.recipebook.page", vec![FormattedText::of((compat::get_state(&PAGE)).wrapping_add(1).to_string()), FormattedText::of(self.get_page_count().to_string())]));
        // `StringWidget.getWidth:44-55` - `min(maxWidth, font.width(message))` once the message changed
        let text_width: i32 = compat::font().width_of(&message);
        let page_text = self.children.widget_mut(PAGE_TEXT);
        page_text.set_message_text(message);
        page_text.width = string_widget::width(30, text_width);
        // java: prevPageButton.visible = page > 0;
        let prev_visible: bool = compat::get_state(&PAGE) > 0;
        compat::write(&self.prev_page_button, |__t| __t.visible = prev_visible);
        self.children.widget_mut(PREV_PAGE_BUTTON).visible = prev_visible;
        // java: nextPageButton.visible = page < getPageCount() - 1;
        let next_visible: bool = compat::get_state(&PAGE) < (self.get_page_count()).wrapping_sub(1);
        compat::write(&self.next_page_button, |__t| __t.visible = next_visible);
        self.children.widget_mut(NEXT_PAGE_BUTTON).visible = next_visible;
        // java: layout.arrangeElements();
        self.arrange();
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> &[AbstractWidget] {
        self.children.children()
    }

    /// The children's calls into this widget made during the call that is returning (`ChildCall`), in order.
    fn run_child_calls(&mut self) {
        let calls: Vec<ChildCall> = std::mem::take(&mut *self.child_calls.borrow_mut());
        for call in calls {
            match call {
                ChildCall::ChangePage(offset) => self.change_page(offset),
                ChildCall::FilterChanged => {
                    self.update_filter();
                    self.change_page(0);
                }
            }
        }
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return 0.0;
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        // java: return buttons.stream().map(ResultButton::getFocusedItem).filter(Objects::nonNull).findFirst().orElse(null);
        return compat::read(__this, |__t| __t.buttons.clone()).into_iter().filter_map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_focused_item()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn HoveredItemStackProvider>)) })).next();
    }

    /// AccessoriesHelperWidget.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }
}

impl HoveredItemStackProvider for AccessoriesHelperWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem() (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        AccessoriesHelperWidget::get_focused_item_dyn
    }
}

impl compat::ObjectWidget for AccessoriesHelperWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x) { super.setX(x); layout.setX(x + BORDER_SIZE); }
    fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        self.arrange();
    }

    /// java: @Override public void setY(int y) { super.setY(y); layout.setY(y + BORDER_SIZE); }
    fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        self.arrange();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let (x0, y0, w, h) = (self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, TEXTURE, getX(), getY(), getWidth(), getHeight());
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get(), x0, y0, w, h);
        // java: String prevHighlighted = AccessoriesContainerSolver.INSTANCE.highlightedAccessory; (nullable)
        let prev_highlighted: Option<String> = compat::read(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory.clone());
        // java: AccessoriesContainerSolver.INSTANCE.highlightedAccessory = null;
        compat::write(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory = None);
        // java: for (AbstractWidget widget : widgets) widget.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
        if !(ItemRepository::files_imported()) || tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().get_data().is_none() {
            self.refresh_when_done_loading = true;
            let mut x: i32 = x0.wrapping_add(w.wrapping_div(2));
            let mut y: i32 = y0.wrapping_add(h.wrapping_div(4));
            gui_graphics_extractor::centered_text_string(context.gl, context.minecraft, "Loading...", x, y, -1);
            gui_graphics_extractor::centered_text_string(context.gl, context.minecraft, loading_dots_text::get(util::get_millis()), x, y.wrapping_add(10), -1);
        } else if self.refresh_when_done_loading {
            self.refresh_when_done_loading = false;
            self.refresh_data();
        }
        // java: if (!Objects.equals(prevHighlighted, AccessoriesContainerSolver.INSTANCE.highlightedAccessory)) ContainerSolverManager.markHighlightsDirty();
        if prev_highlighted != compat::read(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory.clone()) {
            ContainerSolverManager::mark_highlights_dirty();
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) {}
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {}

    /// java: AbstractContainerWidget.mouseClicked:62-65 - `updateScrolling(event)` (nothing scrolls: `contentHeight()`
    /// is 0) `|| ContainerEventHandler.super.mouseClicked(event, doubleClick)` over `children()`
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, click: &MouseButtonEvent, _doubled: bool) -> bool {
        let consumed: bool = compat::children_mouse_clicked(&mut self.children, click);
        self.run_child_calls();
        consumed
    }

    /// java: AbstractContainerWidget.mouseReleased:67-71 - ContainerEventHandler.mouseReleased over `children()`
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        compat::children_mouse_released(&mut self.children, event);
        self.run_child_calls();
        false
    }

    /// java: ContainerEventHandler.mouseDragged over `children()`
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        compat::children_mouse_dragged(&mut self.children, event, dx, dy);
        self.run_child_calls();
        false
    }

    /// java: AbstractScrollArea.mouseScrolled:29-36 - `true` while visible (the scroll amount stays 0: `scrollRate()` is 0)
    fn mouse_scrolled(&mut self, _widget: &mut AbstractWidget, _x: f64, _y: f64, _scroll_x: f64, _scroll_y: f64) -> bool {
        self.base.visible
    }

    /// java: ContainerEventHandler.keyPressed - the focused child's
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        let consumed: bool = compat::children_key_pressed(&mut self.children, event);
        self.run_child_calls();
        consumed
    }

    /// java: ContainerEventHandler.charTyped - the focused child's
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        let consumed: bool = compat::children_char_typed(&mut self.children, event);
        self.run_child_calls();
        consumed
    }

    /// The children's `graphics.requestCursor(CursorTypes.POINTING_HAND)` (an arrow's, a result button's): the port
    /// asks the screen's widgets after the frame, and this widget's own are asked through it.
    fn handle_cursor(&self, _widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        self.children
            .children()
            .iter()
            .filter(|child| child.visible && child.is_hovered())
            .find_map(|child| child.handle_cursor(cursors))
    }
}

/// java: private class ArrowButton extends AbstractWidget
#[derive(Clone)]
pub struct ArrowButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean next;
    pub next: bool,
    /// java: private final WidgetSprites textures;
    pub textures: WidgetSprites,
    /// java: `AccessoriesHelperWidget.this` - its `changePage(..)`, recorded for the enclosing widget (`ChildCall`)
    pub outer_calls: Rc<RefCell<Vec<ChildCall>>>,
}

impl std::ops::Deref for ArrowButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ArrowButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ArrowButton {
    /// java: ArrowButton(boolean next)
    pub fn new(mut next: bool, outer_calls: Rc<RefCell<Vec<ChildCall>>>) -> compat::Shared<Self> {
        // java: super(0, 0, 12, 17, CommonComponents.EMPTY);
        // java: this.textures = next ? RecipeBookPage.PAGE_FORWARD_SPRITES : RecipeBookPage.PAGE_BACKWARD_SPRITES;
        // (`PAGE_*_SPRITES = new WidgetSprites(sprite, highlighted)`)
        let textures: WidgetSprites = if next {
            WidgetSprites::with_focused(recipe_book_page::page_forward_sprite(), recipe_book_page::page_forward_highlighted_sprite())
        } else {
            WidgetSprites::with_focused(recipe_book_page::page_backward_sprite(), recipe_book_page::page_backward_highlighted_sprite())
        };
        compat::shared(Self {
            base: AbstractWidget::new(0, 0, 12, 17, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            next,
            textures,
            outer_calls,
        })
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: changePage(next ? 1 : -1);
        let (next, outer_calls) = compat::read(__this, |__t| (__t.next, __t.outer_calls.clone()));
        outer_calls.borrow_mut().push(ChildCall::ChangePage(if next { 1 } else { -1 }));
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let (sprite, x, y, w, h) = compat::read(__this, |__t| (__t.textures.get(true, __t.is_hovered()).clone(), __t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &sprite, x, y, w, h);
        // java: if (isHovered()) graphics.requestCursor(CursorTypes.POINTING_HAND); - `handle_cursor` below
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }
}

/// ArrowButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ArrowButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ArrowButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ArrowButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ArrowButton::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ArrowButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ArrowButton::on_click(__h, event, double_click))
    }

    /// java: `if (isHovered()) graphics.requestCursor(CursorTypes.POINTING_HAND);` in `extractWidgetRenderState`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        if widget.is_hovered() { Some(cursors.pointing_hand) } else { None }
    }
}

/// java: private static class TabButton extends ImageButton
pub struct TabButton {
    /// java: extends net.minecraft.client.gui.components.ImageButton (the `AbstractWidget` half)
    pub base: AbstractWidget,
    /// java: ImageButton.sprites
    pub sprites: WidgetSprites,
    /// java: private final Consumer<TabButton> onToggled;
    pub on_toggled: Arc<dyn Fn(compat::Shared<TabButton>)>,
    /// java: private boolean toggled;
    pub toggled: bool,
}

impl std::ops::Deref for TabButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TabButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TabButton {
    /// java: TabButton(Consumer<TabButton> onToggled)
    pub fn new(on_toggled: Arc<dyn Fn(compat::Shared<TabButton>)>) -> compat::Shared<Self> {
        // java: super(35, 27, RecipeBookTabButton.SPRITES, _ -> {}, CommonComponents.EMPTY);
        // (`ImageButton(width, height, sprites, onPress, message)` is at (0, 0); `RecipeBookTabButton.SPRITES` is
        // `new WidgetSprites(TAB, TAB_SELECTED)`; the press is this class's `onPress`)
        compat::shared(Self {
            base: AbstractWidget::new(0, 0, recipe_book_tab_button::WIDTH, recipe_book_tab_button::HEIGHT, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            sprites: WidgetSprites::with_focused(recipe_book_tab_button::tab_sprite(), recipe_book_tab_button::tab_selected_sprite()),
            on_toggled,
            toggled: false,
        })
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(__this: &compat::Shared<Self>, context: &WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let (toggled, bx, by, width, height, hovered, sprite) = compat::read(__this, |__t| (__t.toggled, __t.get_x(), __t.get_y(), __t.width, __t.height, __t.is_hovered(), __t.sprites.get(true, __t.toggled).clone()));
        let mut x: i32 = bx;
        if toggled {
            x = x.wrapping_sub(2);
        }
        // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, this.sprites.get(true, this.toggled), x, this.getY(), this.width, this.height);
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &sprite, x, by, width, height);
        let mut offset: i32 = if toggled { -2 } else { 0 };
        // java: WidgetSprites buttonTextures = this.toggled ? RecipeBookPage.PAGE_FORWARD_SPRITES : RecipeBookPage.PAGE_BACKWARD_SPRITES;
        let button_textures: WidgetSprites = if toggled {
            WidgetSprites::with_focused(recipe_book_page::page_forward_sprite(), recipe_book_page::page_forward_highlighted_sprite())
        } else {
            WidgetSprites::with_focused(recipe_book_page::page_backward_sprite(), recipe_book_page::page_backward_highlighted_sprite())
        };
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), button_textures.get(false, hovered), (bx.wrapping_add(offset)).wrapping_add(9), by.wrapping_add((height.wrapping_sub(17)).wrapping_div(2)), 12, 17);
        // java: if (this.isHovered()) graphics.requestCursor(CursorTypes.POINTING_HAND); - `handle_cursor` below
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        TabButton::set_toggled(__this, !(compat::read(__this, |__t| __t.toggled)));
    }

    /// java: public void setToggled(boolean toggled)
    pub fn set_toggled(__this: &compat::Shared<Self>, mut toggled: bool) {
        { let __v = toggled; compat::write(__this, |__t| __t.toggled = __v); }
        ((compat::read(__this, |__t| __t.on_toggled.clone())))(__this.clone());
    }
}

/// TabButton's widget half, lent by the game for each call
impl compat::WidgetHalf for TabButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// TabButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<TabButton> {
    /// java: AbstractButton.extractWidgetRenderState -> this class's `extractContents`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| TabButton::extract_contents(__h, context, mouse_x, mouse_y, a))
    }

    /// java: AbstractButton.updateWidgetNarration - `defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }

    /// java: AbstractButton.onClick:56-59 - `this.onPress(event)`
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) {
        self.lend(widget, |__h| TabButton::on_press(__h, event))
    }

    /// java: `if (this.isHovered()) graphics.requestCursor(CursorTypes.POINTING_HAND);` in `extractContents`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        if widget.is_hovered() { Some(cursors.pointing_hand) } else { None }
    }
}

/// java: private static class ResultButton extends SkyblockRecipeResultButton implements HoveredItemStackProvider
#[derive(Clone)]
pub struct ResultButton {
    /// java: extends SkyblockRecipeResultButton
    pub base: SkyblockRecipeResultButtonBase,
    /// java: private @Nullable MagicPowerSource source;
    pub source: Option<compat::Shared<dyn MagicPowerSource>>,
}

impl std::ops::Deref for ResultButton {
    type Target = SkyblockRecipeResultButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ResultButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ResultButton {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SkyblockRecipeResultButtonBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                source: None,
            });
            ((__h.clone() as compat::Shared<dyn SkyblockRecipeResultButton>), __h)
        });
        __this
    }

    /// java: private void setSource(MagicPowerSource source)
    pub fn set_source(__this: &compat::Shared<Self>, mut source: compat::Shared<dyn MagicPowerSource>) {
        { let __v = Some(source.clone()); compat::write(__this, |__t| __t.source = __v); }
        ({ let __f = compat::dispatch(__this, |__t| __t.set_display_stack()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>), FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(&source, |__t| __t.icon()); __f(&source) }))) });
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if compat::read(__this, |__t| __t.source.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.source.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.click()); __f(__r) });
        }
    }
}

impl SkyblockRecipeResultButton for ResultButton {
    fn skyblock_recipe_result_button_base(&self) -> &SkyblockRecipeResultButtonBase {
        &self.base
    }
    fn skyblock_recipe_result_button_base_mut(&mut self) -> &mut SkyblockRecipeResultButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>, &GuiGraphicsExtractor, i32, i32, f32) {
        ResultButton::extract_widget_render_state_dyn
    }

    /// java: @Override protected void clearDisplayStack() (dispatch: the body for this class)
    fn clear_display_stack(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>) {
        ResultButton::clear_display_stack_dyn
    }
}

impl HoveredItemStackProvider for ResultButton {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem() (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        ResultButton::get_focused_item_dyn
    }
}

impl ResultButton {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn SkyblockRecipeResultButton>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>), graphics, mouse_x, mouse_y, a);
        let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ({ let __f = compat::dispatch(__this, |__t| __t.get_display_stack()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>)) });
        if __this.lock().borrow().is_hovered() && stack.is_some() && compat::read(__this, |__t| __t.source.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.source.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.extract_tooltip()); __f(__r, graphics, mouse_x, mouse_y) });
            // java: graphics.requestCursor(CursorTypes.POINTING_HAND); - `handle_cursor` below
        }
    }

    /// ResultButton.extractWidgetRenderState as the dispatch of SkyblockRecipeResultButton.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn SkyblockRecipeResultButton>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override protected void clearDisplayStack()
    pub fn clear_display_stack_impl(__this: &compat::Shared<Self>) {
        <dyn SkyblockRecipeResultButton>::clear_display_stack_body(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>));
    }

    /// ResultButton.clearDisplayStack as the dispatch of SkyblockRecipeResultButton.clearDisplayStack
    pub fn clear_display_stack_dyn(__this: &compat::Shared<dyn SkyblockRecipeResultButton>) {
        Self::clear_display_stack_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return (if __this.lock().borrow().is_hovered() { ({ let __f = compat::dispatch(__this, |__t| __t.get_display_stack()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>)) }) } else { None });
    }

    /// ResultButton.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }
}

/// ResultButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ResultButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// ResultButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ResultButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>), &context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>), output) }))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ResultButton::on_click(__h, event, double_click))
    }

    /// java: `if (isHovered() && stack != null && source != null) { ..; graphics.requestCursor(CursorTypes.POINTING_HAND); }`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        let has_source: bool = compat::read(self.object(), |__t| __t.source.is_some());
        if widget.is_hovered() && has_source { Some(cursors.pointing_hand) } else { None }
    }
}

/// java: private record AccessoryInfo(Accessory accessory, Optional<Accessory> highestOwned, Accessory highestInFamily)
#[derive(Clone, Debug, PartialEq)]
pub struct AccessoryInfo {
    /// java: Accessory accessory
    pub accessory: Accessory,
    /// java: Optional<Accessory> highestOwned
    pub highest_owned: Option<Accessory>,
    /// java: Accessory highestInFamily
    pub highest_in_family: Accessory,
}

impl AccessoryInfo {
    /// java: the canonical record constructor
    pub fn new(mut accessory: Accessory, mut highest_owned: Option<Accessory>, mut highest_in_family: Accessory) -> Self {
        Self { accessory, highest_owned, highest_in_family }
    }

    /// java: the record accessor `accessory()`
    pub fn accessory(&self) -> Accessory {
        self.accessory.clone()
    }

    /// java: the record accessor `highestOwned()`
    pub fn highest_owned(&self) -> Option<Accessory> {
        self.highest_owned.clone()
    }

    /// java: the record accessor `highestInFamily()`
    pub fn highest_in_family(&self) -> Accessory {
        self.highest_in_family.clone()
    }

}

/// java: private static class Source implements MagicPowerSource
pub struct Source {
    /// java: private final AccessoryInfo info;
    pub info: AccessoryInfo,
    /// java: private final @Nullable List<FormattedCharSequence> afterSelling;
    pub after_selling: Option<Vec<Seq>>,
    /// java: private @Nullable FlexibleItemStack icon;
    pub icon: Option<compat::Shared<FlexibleItemStack>>,
}

impl Source {
    /// java: private Source(AccessoryInfo info)
    pub fn new(mut info: AccessoryInfo) -> compat::Shared<Self> {
        let __init_info = info.clone();
        let __this = compat::shared(Self {
            info: __init_info,
            after_selling: None,
            icon: None,
        });
        { let __v = info.highest_owned().map(|__a0| __a0.id()).and_then(|__a0| ItemRepository::get_item_stack(&__a0)).and_then(|acc_stack| {
            let mut price_opt: Option<f64> = AccessoriesHelperWidget::get_price(&info.accessory());
            if price_opt.is_none() {
                return None;
            }
            let mut price: Option<f64> = compat::read(&acc_stack, |acc_stack| ItemUtils::get_item_price(acc_stack));
            if price.is_none() {
                return None;
            }
            let mut translatable: MutableComponent = component::translatable_args("skyblocker.accessoryHelper.afterSelling", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ItemTooltip::get_coins_message((price_opt.clone().unwrap() - price.clone().unwrap()), 1)), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(FlexibleItemStack::get_stack_or_throw(&acc_stack).get_hover_name()))]);
            // Font.split = Language.getInstance().getVisualOrder(splitter.splitLines(text, width, Style.EMPTY))
            return Some(mcrust::net::minecraft::locale::language::get_instance().get_visual_order_list(&compat::font().split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(translatable.clone()), 170)));
        }); compat::write(&__this, |__t| __t.after_selling = __v); }
        __this
    }

}

impl MagicPowerSource for Source {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack icon() (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> compat::Shared<FlexibleItemStack> {
        Source::icon_dyn
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn MagicPowerSource>, &GuiGraphicsExtractor, i32, i32) {
        Source::extract_tooltip_dyn
    }

    /// java: @Override public double pricePerMp() (dispatch: the body for this class)
    fn price_per_mp(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> f64 {
        Source::price_per_mp_dyn
    }

    /// java: @Override public void click() (dispatch: the body for this class)
    fn click(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) {
        Source::click_dyn
    }

}

impl Source {
    /// java: @Override public FlexibleItemStack icon()
    pub fn icon_impl(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        // java: return icon != null ? icon : (icon = Optional.ofNullable(ItemRepository.getItemStack(info.accessory().id())).orElse(ItemUtils.getItemIdPlaceholder(info.accessory().id())));
        if let Some(icon) = compat::read(__this, |__t| __t.icon.clone()) {
            return icon;
        }
        let id = compat::read(__this, |__t| __t.info.accessory().id());
        let icon = ItemRepository::get_item_stack(&id).unwrap_or_else(|| ItemUtils::get_item_id_placeholder(&id));
        { let __v = Some(icon.clone()); compat::write(__this, |__t| __t.icon = __v); }
        return icon;
    }

    /// Source.icon as the dispatch of MagicPowerSource.icon
    pub fn icon_dyn(__this: &compat::Shared<dyn MagicPowerSource>) -> compat::Shared<FlexibleItemStack> {
        Self::icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        if compat::read(__this, |__t| __t.icon.clone()).is_none() {
            return;
        }
        if let Some(owned) = __this.lock().borrow().info.highest_owned() {
            { let __v = Some(owned.id()); compat::write(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory = __v); }
        }
        let icon_stack = FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone()).unwrap());
        // java: List<FormattedCharSequence> tooltip = Screen.getTooltipFromItem(client, icon.getStackOrThrow()).stream().map(Component::getVisualOrderText).collect(Util.toMutableList());
        // (`Screen.getTooltipFromItem:238-242` with the client's level and player; the port draws a sequence as the
        // `FormattedText` it was made from)
        let level = mcrust::mcrust::modding::game::level();
        let mut tooltip: Vec<FormattedText> = mcrust::mcrust::modding::game::with_player(|player| abstract_container_screen::get_tooltip_from_item(&icon_stack, level.as_deref(), Some(&*player as &dyn mcrust::net::minecraft::world::entity::player::player::Player), abstract_container_screen::tooltip_flag_from_options()))
            .unwrap_or_else(|| abstract_container_screen::get_tooltip_from_item(&icon_stack, level.as_deref(), None, abstract_container_screen::tooltip_flag_from_options()));
        // java: tooltip.add(smoothLine.getVisualOrderText());
        tooltip.push(FormattedText::from(SOURCE_SMOOTH_LINE.get().clone()));
        let after_selling: Option<Vec<Seq>> = compat::read(__this, |__t| __t.after_selling.clone());
        if let Some(after_selling) = after_selling {
            // java: tooltip.addAll(afterSelling); tooltip.add(FormattedCharSequence.EMPTY);
            tooltip.extend(after_selling.iter().map(|line| compat::sequence_text(&**line)));
            tooltip.push(FormattedText::from(component::empty()));
        }
        // java: tooltip.add(wikiLine.getVisualOrderText());
        tooltip.push(FormattedText::from(SOURCE_WIKI_LINE.get().clone()));
        // java: graphics.setTooltipForNextFrame(client.font, tooltip, mouseX, mouseY, icon.get(DataComponents.TOOLTIP_STYLE));
        let _ = (graphics, mouse_x, mouse_y);
        compat::batch_s_set_tooltip_for_next_frame_styled(tooltip, mcrust::mcrust::modding::widgets::item_tooltip_style(&icon_stack));
    }

    /// Source.extractTooltip as the dispatch of MagicPowerSource.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn MagicPowerSource>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public double pricePerMp()
    pub fn price_per_mp_impl(__this: &compat::Shared<Self>) -> f64 {
        let mut price_opt: Option<f64> = AccessoriesHelperWidget::get_price(&__this.lock().borrow().info.accessory());
        if price_opt.is_none() {
            return f64::MAX;
        }
        let mut price: f64 = price_opt.clone().unwrap();
        let mut original_mp: i32 = 0;
        if __this.lock().borrow().info.highest_owned().is_some() {
            let mut owned_price: Option<f64> = AccessoriesHelperWidget::get_price(&__this.lock().borrow().info.highest_owned().unwrap());
            price = (price - owned_price.clone().unwrap_or(((0) as f64) as f64));
            let mut stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&__this.lock().borrow().info.highest_owned().unwrap().id());
            original_mp = (if stack.is_some() { stack.as_ref().unwrap().lock().borrow().get_skyblock_rarity().get_mp() } else { 0 });
        }
        let Some(mut stack) = ItemRepository::get_item_stack(&__this.lock().borrow().info.accessory().id()) else {
            return f64::MAX;
        };
        let mut mp: i32 = (stack.lock().borrow().get_skyblock_rarity().get_mp()).wrapping_sub(original_mp);
        return (if mp <= 0 { f64::MAX } else { (price / (mp as f64)) });
    }

    /// Source.pricePerMp as the dispatch of MagicPowerSource.pricePerMp
    pub fn price_per_mp_dyn(__this: &compat::Shared<dyn MagicPowerSource>) -> f64 {
        Self::price_per_mp_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void click()
    pub fn click_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.icon.clone()).is_none() {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { WikiLookupManager::open_wiki(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone()).unwrap()), player); });
    }

    /// Source.click as the dispatch of MagicPowerSource.click
    pub fn click_dyn(__this: &compat::Shared<dyn MagicPowerSource>) {
        Self::click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private static class RecombobulateSource implements MagicPowerSource
#[derive(Clone)]
pub struct RecombobulateSource {
    /// java: private final FlexibleItemStack icon;
    pub icon: compat::Shared<FlexibleItemStack>,
    /// java: private final double pricePerMp;
    pub price_per_mp: f64,
    /// java: private final List<Component> tooltip;
    pub tooltip: Vec<MutableComponent>,
}

impl RecombobulateSource {
    /// java: private RecombobulateSource(SkyblockItemRarity rarity)
    pub fn new(mut rarity: SkyblockItemRarity) -> compat::Shared<Self> {
        let __init_icon = ItemRepository::get_item_stack_2("RECOMBOBULATOR_3000", ico::BARRIER.get_mut());
        let mut opt: Option<f64> = ItemUtils::get_item_price_string(Some("RECOMBOBULATOR_3000".to_string()));
        let mut price: f64 = opt.clone().unwrap_or(((6000000) as f64) as f64);
        let mut mp: i32 = (rarity.recombobulate().get_mp()).wrapping_sub(rarity.get_mp());
        let __init_price_per_mp = (if mp <= 0 { f64::MAX } else { (price / (mp as f64)) });
        let __init_tooltip = vec![component::translatable_args("skyblocker.accessoryHelper.recombobulate", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&rarity.name().replace("_", " "))).with_color(rarity.fields().color))]), ItemTooltip::get_coins_message(price, 1)];
        let __this = compat::shared(Self {
            icon: __init_icon,
            price_per_mp: __init_price_per_mp,
            tooltip: __init_tooltip,
        });
        __this
    }

}

impl MagicPowerSource for RecombobulateSource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack icon() (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> compat::Shared<FlexibleItemStack> {
        RecombobulateSource::icon_dyn
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn MagicPowerSource>, &GuiGraphicsExtractor, i32, i32) {
        RecombobulateSource::extract_tooltip_dyn
    }

    /// java: @Override public double pricePerMp() (dispatch: the body for this class)
    fn price_per_mp(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> f64 {
        RecombobulateSource::price_per_mp_dyn
    }

    /// java: @Override public void click() (dispatch: the body for this class)
    fn click(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) {
        RecombobulateSource::click_dyn
    }

}

impl RecombobulateSource {
    /// java: @Override public FlexibleItemStack icon()
    pub fn icon_impl(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return compat::read(__this, |__t| __t.icon.clone());
    }

    /// RecombobulateSource.icon as the dispatch of MagicPowerSource.icon
    pub fn icon_dyn(__this: &compat::Shared<dyn MagicPowerSource>) -> compat::Shared<FlexibleItemStack> {
        Self::icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        // java: Font textRenderer = Minecraft.getInstance().font;
        // java: graphics.setComponentTooltipForNextFrame(textRenderer, tooltip, mouseX, mouseY);
        let _ = (graphics, mouse_x, mouse_y);
        let lines: Vec<FormattedText> = compat::read(__this, |__t| __t.tooltip.iter().map(|line| FormattedText::from(line.clone())).collect());
        compat::batch_s_set_tooltip_for_next_frame(lines);
    }

    /// RecombobulateSource.extractTooltip as the dispatch of MagicPowerSource.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn MagicPowerSource>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public double pricePerMp()
    pub fn price_per_mp_impl(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.price_per_mp);
    }

    /// RecombobulateSource.pricePerMp as the dispatch of MagicPowerSource.pricePerMp
    pub fn price_per_mp_dyn(__this: &compat::Shared<dyn MagicPowerSource>) -> f64 {
        Self::price_per_mp_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void click()
    pub fn click_impl(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { WikiLookupManager::open_wiki_link_name("Recombobulator_3000#Usage", player); });
    }

    /// RecombobulateSource.click as the dispatch of MagicPowerSource.click
    pub fn click_dyn(__this: &compat::Shared<dyn MagicPowerSource>) {
        Self::click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private interface MagicPowerSource
pub trait MagicPowerSource: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: FlexibleItemStack icon(); (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> compat::Shared<FlexibleItemStack>;

    /// java: void extractTooltip(GuiGraphicsExtractor graphics, int mouseX, int mouseY); (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn MagicPowerSource>, &GuiGraphicsExtractor, i32, i32);

    /// java: double pricePerMp(); (dispatch: the body for this class)
    fn price_per_mp(&self) -> fn(&compat::Shared<dyn MagicPowerSource>) -> f64;

    /// java: void click(); (dispatch: the body for this class)
    fn click(&self) -> fn(&compat::Shared<dyn MagicPowerSource>);

}

/// java: private enum Filter
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Filter {
    All,
    Missing,
    Upgrades,
}

impl Filter {
    pub const VALUES: [Filter; 3] = [Filter::All, Filter::Missing, Filter::Upgrades];

    /// java: values()
    pub fn values() -> Vec<Filter> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Filter::All => "ALL",
            Filter::Missing => "MISSING",
            Filter::Upgrades => "UPGRADES",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Filter {
        match name {
            "ALL" => Filter::All,
            "MISSING" => Filter::Missing,
            "UPGRADES" => Filter::Upgrades,
            _ => panic!("IllegalArgumentException: No enum constant Filter.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return format!("skyblocker.accessoryHelper.filter.{}", self.name());
    }

}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Filter::to_string(self))
    }
}
