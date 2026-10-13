//! Ported from de.hysky.skyblocker.skyblock.accessories.AccessoriesHelperWidget by fabric2seer.

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
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::cycle_button::CycleButton;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::gui::screens::loading_dots_text;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page::RecipeBookPage;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::util::util;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

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

/// java: class AccessoriesHelperWidget extends AbstractContainerWidget implements HoveredItemStackProvider
pub struct AccessoriesHelperWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: Vec<AbstractWidget>,
    /// java: private final List<ResultButton> buttons = new ArrayList<>(BUTTON_COUNT);
    pub buttons: Vec<compat::Shared<ResultButton>>,
    /// java: private final FrameLayout layout;
    pub layout: Option<FrameLayout>,
    /// java: private final StringWidget pageText = new StringWidget(CommonComponents.EMPTY, Minecraft.getInstance().font).setMaxWidth(30, StringWidget.TextOverflow.SCROLLING
    pub page_text: AbstractWidget,
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
}

impl std::ops::Deref for AccessoriesHelperWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AccessoriesHelperWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AccessoriesHelperWidget {
    /// java: AccessoriesHelperWidget()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 0, 147, 182, CommonComponents.EMPTY, AbstractScrollArea.defaultSettings(4));
        let __this: compat::Shared<Self> = todo!("constructor");
        { let __v = Some(FrameLayout::of((__this.lock().borrow().get_width()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)), (__this.lock().borrow().get_height()).wrapping_sub(BORDER_SIZE.wrapping_mul(2)))); compat::write(&__this, |__t| __t.layout = __v); }
        // `layout.addChild(LinearLayout.vertical())` answers the child itself; the port's layouts hold children by value,
        // so the child is built here and added once it is finished (below, before it is arranged)
        let mut main_layout: LinearLayout = LinearLayout::vertical();
        { let __s = main_layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        // TODO(fabric2seer): `.createRowHelper(..)` on a value of unknown type
        //   java: GridLayout.RowHelper adder = mainLayout.addChild(new GridLayout()).createRowHelper(5);
        let mut adder: RowHelper = todo!("fabric2seer: the initialiser of `adder` (TODO above)");
        for i in 0..BUTTON_COUNT {
            let mut button: compat::Shared<ResultButton> = ResultButton::new();
            let __h0 = button.clone();
            __this.lock().borrow_mut().buttons.push(__h0);
            // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
            // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
            // rust: adder.add_child(&mut button).unwrap();
        }
        // TODO(fabric2seer): static call ImmutableList.builder(..)
        //   java: ImmutableList.Builder<AbstractWidget> builder = ImmutableList.builder();
        let mut builder: crate::unresolved::Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        // java: LinearLayout pageSwitcher = mainLayout.addChild(LinearLayout.horizontal().spacing(2), LayoutSettings::paddingVertical(4)) -
        // the port's layouts hold a child by value (see `main_layout` above), so the switcher is built here and added
        // to `main_layout` once its children are in (below), in the same place among `main_layout`'s children
        let mut page_switcher: LinearLayout = LinearLayout::horizontal().spacing(2);
        { let __s = page_switcher.default_cell_setting(); *__s = (*__s).align_vertically_middle(); }
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: page_switcher.add_child(compat::read(&__this, |__t| __t.prev_page_button.clone()));
        // TODO(fabric2seer): `.addChild(..)` on a value of unknown type
        //   java: pageSwitcher.addChild(new FrameLayout(30, 0)).addChild(pageText);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: page_switcher.add_child(compat::read(&__this, |__t| __t.next_page_button.clone()));
        main_layout.add_child_adjusted(LayoutElement::Grid(page_switcher.wrapped.clone()), &mut |p| { *p = p.padding_vertical(4); });
        let mut filter_width: i32 = (__this.lock().borrow().layout.as_ref().unwrap().get_width()).wrapping_sub(2);
        // TODO(fabric2seer): the port's `builder` ports this Java method with other parameters (default_value for Java's value_stringifier, default_value)
        // TODO(fabric2seer): `builder` takes 1 argument(s) in the port (`default_value: String`), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Function<T, net.minecraft.network.chat.Component>`)
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: main_layout.add_child((cycle_button::builder(|f| component::translatable(&compat::jstring(&f)), compat::get_state(&FILTER))).with_values(Filter::values()).create(0, 0, filter_width, 16, component::translatable("skyblocker.accessoryHelper.filter"), |_, v| {
        // rust: compat::set_state(&FILTER, v.clone());
        // rust: AccessoriesHelperWidget::update_filter(&(*__this_1.get()));
        // rust: AccessoriesHelperWidget::change_page(&(*__this_1.get()), 0);
        // rust: }).unwrap());
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: main_layout.add_child((cycle_button::boolean_builder(component::translatable("skyblocker.accessoryHelper.highestTierOnly").get_string(), component::translatable("skyblocker.accessoryHelper.allTiers").get_string(), compat::get_state(&SHOW_HIGHEST_TIER_ONLY))).display_only_value().create(0, 0, filter_width, 16, mcrust::net::minecraft::network::chat::common_components::empty(), |_, value| {
        // rust: compat::set_state(&SHOW_HIGHEST_TIER_ONLY, value.clone());
        // rust: AccessoriesHelperWidget::update_filter(&(*__this_2.get()));
        // rust: AccessoriesHelperWidget::change_page(&(*__this_2.get()), 0);
        // rust: }).unwrap());
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: mainLayout.visitWidgets(builder::add);
        // TODO(fabric2seer): `.build(..)` on a Builder
        //   java: widgets = builder.build();
        main_layout.arrange_elements();
        { let (__q0,) = (LayoutElement::Grid(main_layout.wrapped.clone()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_child(__q0) };
        AccessoriesHelperWidget::update_page_switcher(&__this);
        __this
    }

    /// java: static void attachToScreen(ContainerScreen screen)
    ///
    /// rust: port gap - the vanilla `ContainerScreen` object (its `leftPos` / `topPos` through
    /// `AbstractContainerScreenAccessor`, its widget list through `Screens.getWidgets`) is not lent to a mod's
    /// `ScreenEvents.AFTER_INIT` listener, so nothing can call this (GAPS_batchG).
    pub fn attach_to_screen(screen: &ContainerScreen) {
        if !(compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_accessories_helper)) || !(SkyblockerConfigManager::get().helpers.enable_accessories_helper_widget) {
            return;
        }
        let mut widget: compat::Shared<AccessoriesHelperWidget> = AccessoriesHelperWidget::new();
        // TODO(fabric2seer): `Screen.height` is a field of the port's `Screen`, which the port's `ContainerScreen` does not hold (no Deref, no `base` field)
        // rust: let __o1 = widget.clone();
        // rust: AccessoriesHelperWidget::set_y(&widget, (screen.height.wrapping_sub(compat::write(&__o1, |__t| __t.get_height()))).wrapping_div(2));
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(screen).add(widget);
        let mut previous_x: i32 = screen.container.left_pos;
        let mut offset: i32 = (180i32.wrapping_sub(previous_x)).max(0);
        // `screen.topPos` is read when the button is pressed; this code never writes it
        let top_pos: i32 = screen.container.top_pos;
        let mut tab_button: compat::Shared<TabButton> = TabButton::new(Arc::new(move |button| {
            let mut toggled: bool = compat::read(&button, |__t| __t.toggled);
            // java: widget.visible = open = toggled;
            compat::set_state(&OPEN, toggled);
            compat::write(&widget, |__t| __t.visible = toggled);
            let mut x: i32 = (if toggled { previous_x.wrapping_add(offset) } else { previous_x });
            // rust: port gap - `((AbstractContainerScreenAccessor) screen).setX(x)` (`screen.leftPos = x`): the live
            // screen is not reachable from the button's press (GAPS_batchG)
            let __o2 = widget.clone();
            AccessoriesHelperWidget::set_x(&widget, (x.wrapping_sub(compat::write(&__o2, |__t| __t.get_width()))).wrapping_sub(2));
            let __o3 = widget.clone();
            let __o4 = button.clone();
            let __o5 = widget.clone();
            let __o6 = button.clone();
            let __wa1 = ((if toggled { compat::write(&__o5, |__t| __t.get_x()) } else { x }).wrapping_sub(compat::write(&__o6, |__t| __t.get_width()))).wrapping_add(5);
            let __o7 = button.clone();
            compat::write(&__o7, |__t| __t.set_x(__wa1));
            let __o8 = widget.clone();
            let __o9 = widget.clone();
            let __wa2 = (if toggled { compat::write(&__o9, |__t| __t.get_y()) } else { top_pos }).wrapping_add(8);
            let __o10 = button.clone();
            compat::write(&__o10, |__t| __t.set_y(__wa2));
            if toggled {
                AccessoriesHelperWidget::refresh_data(&widget);
            } else {
                compat::set_state(&PAGE, 0);
            }
        }));
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(screen).add(tabButton);
        TabButton::set_toggled(&tab_button, compat::get_state(&OPEN));
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
    pub fn refresh_data(__this: &compat::Shared<Self>) {
        let mut collected_accessories: std::collections::HashSet<Accessory> = AccessoriesHelper::get_collected_accessories();
        { let __v = compat::distinct(collected_accessories.iter().cloned().filter(|__a0| __a0.recombobulatable()).map(|__a0| __a0.id()).filter(|id| !(AccessoriesHelper::is_recombobulated(id))).filter_map(|__a0| ItemRepository::get_item_stack(&__a0)).map(|__a0| compat::read(&__a0, |__t| __t.get_skyblock_rarity()))).map(|__a0| RecombobulateSource::new(__a0)).collect::<Vec<_>>(); compat::write(__this, |__t| __t.recomb_displays = __v); }
        // TODO(fabric2seer): `.getMisc(..)` on a NEUConstants
        //   java: this.accessories = AccessoriesHelper.ACCESSORY_DATA.values().stream()
        //   java: 				.filter(accessory -> NEURepoManager.isLoading() || !NEURepoManager.getConstants().getMisc().getIgnoredTalismans().contains(accessory.id())) // Removes admin and rift items
        //   java: 				.map(accessory -> {
        //   java: 					if (accessory.family().isPresent()) {
        //   java: 						AccessoriesHelper.FamilyReport report = AccessoriesHelper.calculateFamilyReport(accessory, collectedAccessories);
        //   java: 						return new AccessoryInfo(accessory, report.highestCollectedInFamily(), report.highestInFamily());
        //   java: 					} else
        //   java: 						return new AccessoryInfo(accessory, collectedAccessories.contains(accessory) ? Optional.of(accessory) : Optional.empty(), accessory);
        //   java: 				}).distinct()
        //   java: 				.filter(info -> info.accessory().tier() > info.highestOwned().map(Accessory::tier).orElse(-1))
        //   java: 				.toList();
        AccessoriesHelperWidget::update_filter(__this);
        AccessoriesHelperWidget::change_page(__this, 0);
    }

    /// java: private void updateFilter()
    pub fn update_filter(__this: &compat::Shared<Self>) {
        let mut predicate: Arc<dyn Fn(AccessoryInfo) -> bool> = match compat::get_state(&FILTER) {
            Filter::All => Arc::new(move |_| true),
            Filter::Missing => Arc::new(move |info| info.highest_owned().is_none()),
            Filter::Upgrades => Arc::new(move |info| (info.highest_owned().is_some() && info.accessory().tier() > info.highest_owned().unwrap().tier())),
            _ => unreachable!("switch expression: no case matched"),
        };
        let show_highest_tier_only: bool = compat::get_state(&SHOW_HIGHEST_TIER_ONLY);
        let mut stream: Box<dyn Iterator<Item = compat::Shared<dyn MagicPowerSource>>> = Box::new(
            compat::read(__this, |__t| __t.accessories.clone())
                .into_iter()
                .filter(move |info| predicate(info.clone()))
                .filter(move |info| !show_highest_tier_only || info.accessory().tier() >= info.highest_in_family().tier())
                .map(|info| Source::new(info) as compat::Shared<dyn MagicPowerSource>),
        );
        { let __v = compat::sorted_by(stream.chain(compat::read(__this, |__t| __t.recomb_displays.clone()).into_iter().map(|r| r as compat::Shared<dyn MagicPowerSource>)), |a, b| { let key = |__a0: &compat::Shared<dyn MagicPowerSource>| ({ let __f = compat::dispatch(__a0, |__t| __t.price_per_mp()); __f(__a0) }); key(a).total_cmp(&key(b)) }).collect::<Vec<_>>(); compat::write(__this, |__t| __t.displays = __v); }
    }

    /// java: private void changePage(int offset)
    pub fn change_page(__this: &compat::Shared<Self>, mut offset: i32) {
        compat::set_state(&PAGE, ((compat::get_state(&PAGE)).wrapping_add(offset)).clamp(0, (AccessoriesHelperWidget::get_page_count(__this)).wrapping_sub(1)));
        AccessoriesHelperWidget::update_page_switcher(__this);
        for i in 0..BUTTON_COUNT {
            let mut j: i32 = i.wrapping_add((compat::get_state(&PAGE)).wrapping_mul(BUTTON_COUNT));
            if j < (__this.lock().borrow().displays.len() as i32) {
                ResultButton::set_source(&__this.lock().borrow().buttons[(i) as usize], __this.lock().borrow().displays[(j) as usize].clone());
            } else {
                ({ let __r = &compat::read(__this, |__t| __t.buttons[(i) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.clear_display_stack()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SkyblockRecipeResultButton>)) });
            }
        }
    }

    /// java: private int getPageCount()
    pub fn get_page_count(__this: &compat::Shared<Self>) -> i32 {
        // Math.ceilDiv(x, y) = -Math.floorDiv(-x, y)
        return 1.max(compat::floor_div_i32(-(__this.lock().borrow().displays.len() as i32), BUTTON_COUNT).wrapping_neg());
    }

    /// java: private void updatePageSwitcher()
    pub fn update_page_switcher(__this: &compat::Shared<Self>) {
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("gui.recipebook.page", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((compat::get_state(&PAGE)).wrapping_add(1).to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(AccessoriesHelperWidget::get_page_count(__this).to_string())])),); __this.lock().borrow_mut().page_text.set_message_text(__q0) };
        { let __v = (compat::get_state(&PAGE) > 0); compat::write(&compat::read(__this, |__t| __t.prev_page_button.clone()), |__t| __t.visible = __v); }
        { let __v = (compat::get_state(&PAGE) < (AccessoriesHelperWidget::get_page_count(__this)).wrapping_sub(1)); compat::write(&compat::read(__this, |__t| __t.next_page_button.clone()), |__t| __t.visible = __v); }
        __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements();
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    ///
    /// The widgets themselves, lent to `f` (Java's list holds the same objects).
    pub fn children<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&Vec<AbstractWidget>) -> R) -> R {
        return compat::read(__this, |__t| f(&__t.widgets));
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        { let (__q0,) = (x.wrapping_add(BORDER_SIZE),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_x(__q0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        { let (__q0,) = (y.wrapping_add(BORDER_SIZE),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_y_translating(__q0) };
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        let mut prev_highlighted: String = compat::read(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory.clone()).unwrap();
        { let __v = None; compat::write(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory = __v); }
        for widget in compat::read(__this, |__t| __t.widgets.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
        if !(ItemRepository::files_imported()) || tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().get_data().is_none() {
            { let __v = true; compat::write(__this, |__t| __t.refresh_when_done_loading = __v); }
            let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2));
            let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add((__this.lock().borrow().get_height()).wrapping_div(4));
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Loading...", x, y, -1);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), loading_dots_text::get(util::get_millis()), x, y.wrapping_add(10), -1);
        } else if compat::read(__this, |__t| __t.refresh_when_done_loading) {
            { let __v = false; compat::write(__this, |__t| __t.refresh_when_done_loading = __v); }
            AccessoriesHelperWidget::refresh_data(__this);
        }
        if !(Some(&*(prev_highlighted)) == compat::read(accessories_container_solver::INSTANCE.get_mut(), |__t| __t.highlighted_accessory.clone()).as_deref()) {
            ContainerSolverManager::mark_highlights_dirty();
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        return 0;
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

impl AccessoriesHelperWidget {
    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return compat::read(__this, |__t| __t.buttons.clone()).into_iter().filter_map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_focused_item()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn HoveredItemStackProvider>)) })).next();
    }

    /// AccessoriesHelperWidget.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, scrollRate, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class ArrowButton extends AbstractWidget
#[derive(Clone)]
pub struct ArrowButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean next;
    pub next: bool,
    /// java: private final WidgetSprites textures;
    pub textures: WidgetSprites,
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
    pub fn new(mut next: bool) -> compat::Shared<Self> {
        let __init_next = next;
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_FORWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        //   java: this.textures = next ? RecipeBookPage.PAGE_FORWARD_SPRITES : RecipeBookPage.PAGE_BACKWARD_SPRITES;
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 12, 17, (&mcrust::net::minecraft::network::chat::common_components::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            next: __init_next,
            textures: todo!("fabric2seer: the field value (TODO above)"),
        });
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `changePage(..)` is a method of the enclosing instance (AccessoriesHelperWidget.this) - an inner class has no outer reference in Rust
        //   java: changePage(next ? 1 : -1);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let __sprite = compat::read(__this, |__t| __t.textures.get(true, __t.is_hovered()).clone());
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &__sprite, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::POINTING_HAND);
        }
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
        self.lend_ref(widget, |__h| ArrowButton::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ArrowButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ArrowButton::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class TabButton extends ImageButton
pub struct TabButton {
    /// java: extends net.minecraft.client.gui.components.ImageButton
    pub base: AbstractWidget,
    /// java: private final Consumer<TabButton> onToggled;
    pub on_toggled: Arc<dyn Fn(compat::Shared<TabButton>)>,
    /// java: private boolean toggled;
    pub toggled: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
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
        let __init_on_toggled = on_toggled.clone();
        // TODO(fabric2seer): static `RecipeBookTabButton.SPRITES` has no Rust counterpart in the port's RecipeBookTabButton file
        //   java: super(35, 27, RecipeBookTabButton.SPRITES, _ -> {}, CommonComponents.EMPTY);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut x: i32 = __this.lock().borrow().get_x();
        if compat::read(__this, |__t| __t.toggled) {
            x = x.wrapping_sub(2);
        }
        // TODO(fabric2seer): `ImageButton.sprites` has no Rust field in the port (searched net.minecraft.client.gui.components.ImageButton and its supertypes)
        // rust: gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), { let (__q0, __q1,) = (true, compat::read(__this, |__t| __t.toggled),); __this.lock().borrow().sprites.get(__q0, __q1) }, x, __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        let mut offset: i32 = (if compat::read(__this, |__t| __t.toggled) { -2 } else { 0 });
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_FORWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_BACKWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // rust: let mut button_textures: WidgetSprites = (if compat::read(__this, |__t| __t.toggled) { RecipeBookPage::PAGE_FORWARD_SPRITES.clone() } else { RecipeBookPage::PAGE_BACKWARD_SPRITES.clone() });
        let mut button_textures: WidgetSprites = todo!("fabric2seer: the initialiser of `buttonTextures` (TODO above)");
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), button_textures.get(false, __this.lock().borrow().is_hovered()), ((__this.lock().borrow().get_x()).wrapping_add(offset)).wrapping_add(9), (__this.lock().borrow().get_y()).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(17)).wrapping_div(2)), 12, 17);
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::POINTING_HAND);
        }
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

// NOTE(fabric2seer): extractContents, onPress override(s) of ImageButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

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
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::POINTING_HAND);
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
        let client = compat::minecraft();
        // TODO(fabric2seer): collect(..) with a collector that is not `Collectors.*`
        //   java: List<FormattedCharSequence> tooltip = Screen.getTooltipFromItem(client, icon.getStackOrThrow()).stream().map(Component::getVisualOrderText).collect(Util.toMutableList());
        let mut tooltip: Vec<Seq> = todo!("fabric2seer: the initialiser of `tooltip` (TODO above)");
        tooltip.push(SOURCE_SMOOTH_LINE.get_mut().get_visual_order_text());
        if compat::read(__this, |__t| __t.after_selling.clone()).is_some() {
            tooltip.extend(__this.lock().borrow().after_selling.as_ref().unwrap().iter().cloned());
            tooltip.push(mcrust::net::minecraft::util::formatted_char_sequence::empty());
        }
        tooltip.push(SOURCE_WIKI_LINE.get_mut().get_visual_order_text());
        // TODO(fabric2seer): the port's `set_tooltip_for_next_frame_lines_styled` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo, style)
        // TODO(fabric2seer): `set_tooltip_for_next_frame_lines_styled` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 5
        // rust: gui_graphics_extractor::set_tooltip_for_next_frame_lines_styled(&mut compat::font(), tooltip.clone(), mouse_x, mouse_y, FlexibleItemStack::get(&compat::read(__this, |__t| __t.icon.clone()).unwrap(), mcrust::net::minecraft::core::component::data_components::tooltip_style()));
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
        let mut text_renderer: Font = compat::font();
        // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
        // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
        // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut text_renderer, compat::read(__this, |__t| __t.tooltip.clone()), mouse_x, mouse_y);
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
