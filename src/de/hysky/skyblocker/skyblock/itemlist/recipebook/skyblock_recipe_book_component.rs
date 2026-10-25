//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockRecipeBookComponent by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::noop_recipe_book_menu::NoopRecipeBookMenu;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_tab::RecipeTab;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_tab_button::SkyblockRecipeTabButton;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_crafting_tab;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_crafting_tab::SkyblockCraftingTab;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_results::SkyblockRecipeResults;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::upcoming_events_tab::UpcomingEventsTab;
use mcrust::net::minecraft::client::gui::screens::screen::Screen as GuiScreen;
use mcrust::net::minecraft::world::inventory::recipe_book_menu::RecipeBookMenu as _;
use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
use crate::de::hysky::skyblocker::utils::render::gui::cycling_texture_widget::CyclingTextureWidget;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_axis::ScreenAxis;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::recipebook::ghost_slots::GhostSlots;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::RecipeBookComponent;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_collection::RecipeCollection;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::context::context_map::ContextMap;
use mcrust::net::minecraft::world::entity::player::stacked_item_contents::StackedItemContents;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::crafting::display::recipe_display::RecipeDisplay;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: protected static final int IMAGE_WIDTH = RecipeBookComponent.IMAGE_WIDTH;
pub static IMAGE_WIDTH: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::IMAGE_WIDTH);
/// java: private static final int IMAGE_HEIGHT = RecipeBookComponent.IMAGE_HEIGHT;
pub static IMAGE_HEIGHT: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::IMAGE_HEIGHT);
/// java: private static final int OFFSET_X_POSITION = 86;
pub const OFFSET_X_POSITION: i32 = 86;
/// java: private static final int SEARCH_FIELD_WIDTH = 81 + 4 + 8;
pub const SEARCH_FIELD_WIDTH: i32 = (81i32.wrapping_add(4)).wrapping_add(8);
/// java: private static String lastSearch = "";
pub static LAST_SEARCH: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

/// java: public class SkyblockRecipeBookComponent extends RecipeBookComponent<NoopRecipeBookMenu>
///
/// The port's `RecipeBookComponent` keeps the members Java declares `protected` (`searchBox`, `magnifierIconPlacement`,
/// `ignoreTextInput`, `menu`) private and dispatches its own calls, so the members this subclass reads and writes are
/// fields of the subclass, and the inherited bodies that touch them (`keyPressed`, `charTyped`, `keyReleased`,
/// `setVisible`, `tick`) are run here against them. Everything the base owns that the subclass only READS (`visible`,
/// `width`, `height`, `xOffset` through `getXOrigin()`/`getYOrigin()`, the book data) stays the base's.
pub struct SkyblockRecipeBookComponent {
    /// java: extends net.minecraft.client.gui.screens.recipebook.RecipeBookComponent
    pub base: RecipeBookComponent,
    /// java: `RecipeBookComponent.menu` (protected final) - `new NoopRecipeBookMenu(screenHandler.containerId)`
    pub menu: NoopRecipeBookMenu,
    /// java: `RecipeBookComponent.searchBox` (protected, `@Nullable` until the first `initVisuals`): the one widget of
    /// this container, at index 0 (the port's `EditBox` is an `AbstractWidget` kind; its focus is the container's)
    pub search_widgets: GuiScreen,
    /// java: `RecipeBookComponent.magnifierIconPlacement` (protected, `@Nullable`)
    pub magnifier_icon_placement: Option<ScreenRectangle>,
    /// java: `RecipeBookComponent.ignoreTextInput` (private; read by the inherited `charTyped`)
    pub ignore_text_input: bool,
    /// java: `RecipeBookComponent.timesInventoryChanged` (private; read by the inherited `tick`)
    pub times_inventory_changed: i32,
    /// The base's visibility as this object's `initVisuals` last saw it (see `ensure_visuals`)
    pub visuals_visible: bool,
    /// java: private final List<RecipeTab> tabs = List.of( new SkyblockCraftingTab(this, SkyblockCraftingTab.CRAFTING_TABLE, new SkyblockRecipeResults()), new UpcomingEventsTab() );
    pub tabs: Vec<compat::Shared<dyn RecipeTab>>,
    /// java: private final List<Pair<RecipeTab, SkyblockRecipeTabButton>> tabButtons = Lists.newArrayList();
    pub tab_buttons: Vec<(compat::Shared<dyn RecipeTab>, compat::Shared<SkyblockRecipeTabButton>)>,
    /// java: private Pair<RecipeTab, SkyblockRecipeTabButton> currentTab;
    pub current_tab: Option<(compat::Shared<dyn RecipeTab>, compat::Shared<SkyblockRecipeTabButton>)>,
    /// java: protected CyclingTextureWidget<FilterOption> filterOption;
    pub filter_option: Option<compat::Shared<CyclingTextureWidget<FilterOption>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SkyblockRecipeBookComponent {
    type Target = RecipeBookComponent;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockRecipeBookComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockRecipeBookComponent {
    /// java: public SkyblockRecipeBookComponent(AbstractContainerMenu screenHandler)
    pub fn new(mut screen_handler: AbstractContainerMenu) -> compat::Shared<Self> {
        // java: super(new NoopRecipeBookMenu(screenHandler.containerId), List.of());
        // (`RecipeBookComponent(menu, tabInfos)`: the book type is the menu's `getRecipeBookType()` - CRAFTING - and there
        // are no vanilla tabs. The three subclass members the port's base takes as constructor arguments are this
        // class's overrides: `getFilterButtonTextures()` answers null (the base's filter button is never drawn - this
        // class draws the book), `getRecipeFilterName()` null, and `selectMatchingRecipes`/`fillGhostRecipe` are
        // no-ops the base never reaches, the book's recipe page being this class's own tabs.)
        let menu = NoopRecipeBookMenu::new(screen_handler.container_id);
        let base = RecipeBookComponent::new(
            menu.get_recipe_book_type(),
            Vec::new(),
            mcrust::net::minecraft::client::gui::screens::recipebook::crafting_recipe_book_component::filter_button_sprites(),
            "",
            mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::SelectMatchingRecipes::Crafting,
            mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::FillGhostRecipe::Crafting,
            false,
        );
        let __this = compat::shared(Self {
            base,
            menu,
            search_widgets: GuiScreen::new(String::new()),
            magnifier_icon_placement: None,
            ignore_text_input: false,
            times_inventory_changed: 0,
            visuals_visible: false,
            tabs: Vec::new(),
            tab_buttons: Vec::new(),
            current_tab: None,
            filter_option: None,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // java: private final List<RecipeTab> tabs = List.of(new SkyblockCraftingTab(this, SkyblockCraftingTab.CRAFTING_TABLE, new SkyblockRecipeResults()), new UpcomingEventsTab());
        let crafting: compat::Shared<dyn RecipeTab> = compat::shared(SkyblockCraftingTab::new(&__this, skyblock_crafting_tab::CRAFTING_TABLE.get().clone(), SkyblockRecipeResults::new()));
        let events: compat::Shared<dyn RecipeTab> = UpcomingEventsTab::new();
        compat::write(&__this, |__t| __t.tabs = vec![crafting, events]);
        __this
    }

    /// java: @Override public void init(int parentWidth, int parentHeight, Minecraft client, boolean narrow)
    pub fn init(__this: &compat::Shared<Self>, mut parent_width: i32, mut parent_height: i32, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut narrow: bool) {
        // java: super.init(parentWidth, parentHeight, client, narrow);
        // `RecipeBookComponent.init:108-119`: `book = minecraft.player.getRecipeBook()` (the client recipe book the
        // connection holds), `timesInventoryChanged = player.getInventory().getTimesChanged()`, `visible =
        // isVisibleAccordingToBookData()` and, when visible, `initVisuals()` - the base's half here, this class's
        // override below.
        let times_changed: i32 = mcrust::mcrust::modding::game::with_player(|player| {
            use mcrust::net::minecraft::world::entity::player::player::Player as _;
            player.player().get_inventory().get_times_changed()
        })
        .unwrap_or(0);
        if let Some(sinks) = mcrust::net::minecraft::client::multiplayer::client_packet_listener::published_play_sinks() {
            let book = sinks.client_recipe_book.clone();
            compat::write(__this, |__t| __t.base.init(parent_width, parent_height, book, times_changed, &*client, narrow));
        }
        compat::write(__this, |__t| __t.times_inventory_changed = times_changed);
        let visible = compat::read(__this, |__t| __t.base.is_visible());
        compat::write(__this, |__t| __t.visuals_visible = visible);
        if visible {
            SkyblockRecipeBookComponent::init_visuals(__this);
        }
    }

    /// The base's `setVisible(true)` runs `initVisuals()` - this class's override - before it sets the flag. The port's
    /// base runs its own, so a book the base made visible (`toggleVisibility()`, `tick()` following the book data) gets
    /// this class's `initVisuals` here, before the next thing this object does; the base's settings update
    /// (`sendUpdateSettings()`) is sent here too.
    pub fn ensure_visuals(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::send_update_settings(&mut __t.base));
        let (visible, seen) = compat::read(__this, |__t| (__t.base.is_visible(), __t.visuals_visible));
        compat::write(__this, |__t| __t.visuals_visible = visible);
        if visible && !seen {
            SkyblockRecipeBookComponent::init_visuals(__this);
        }
    }

    /// java: `RecipeBookComponent.setVisible(boolean visible)` (protected, inherited): `if (visible) initVisuals();
    /// this.visible = visible; book.setOpen(..); ..; sendUpdateSettings();`
    pub fn set_visible(__this: &compat::Shared<Self>, visible: bool) {
        compat::write(__this, |__t| __t.base.set_visible(visible, &*compat::minecraft()));
        compat::write(__this, |__t| __t.visuals_visible = false);
        SkyblockRecipeBookComponent::ensure_visuals(__this);
    }

    /// java: `RecipeBookComponent.toggleVisibility()` (inherited): `setVisible(!isVisible())`
    pub fn toggle_visibility(__this: &compat::Shared<Self>) {
        let visible = compat::read(__this, |__t| __t.base.is_visible());
        SkyblockRecipeBookComponent::set_visible(__this, !visible);
    }

    /// java: `RecipeBookComponent.tick()` (inherited): follow the book data's visibility, and while visible re-read
    /// the stacked contents when the inventory changed - `updateStackedContents()`, whose `updateCollections` is this
    /// class's override.
    pub fn tick(__this: &compat::Shared<Self>) {
        let (visible, should_be_visible) = compat::read(__this, |__t| (__t.base.is_visible(), __t.base.is_visible_according_to_book_data()));
        if visible != should_be_visible {
            SkyblockRecipeBookComponent::set_visible(__this, should_be_visible);
        }
        if compat::read(__this, |__t| __t.base.is_visible()) {
            let Some(times_changed) = mcrust::mcrust::modding::game::with_player(|player| {
                use mcrust::net::minecraft::world::entity::player::player::Player as _;
                player.player().get_inventory().get_times_changed()
            }) else {
                return;
            };
            if compat::read(__this, |__t| __t.times_inventory_changed) != times_changed {
                // java: this.updateStackedContents(); - `stackedContents` refilled, then `updateCollections(false, isFiltering())`
                let filtering = compat::read(__this, |__t| __t.base.is_filtering());
                SkyblockRecipeBookComponent::update_collections(__this, false, filtering);
                compat::write(__this, |__t| __t.times_inventory_changed = times_changed);
            }
        }
    }

    /// java: @Override protected void initVisuals()
    pub fn init_visuals(__this: &compat::Shared<Self>) {
        // java: this.xOffset = this.widthTooNarrow ? 0 : OFFSET_X_POSITION;
        // (the base's `initVisuals` writes the same value - its `xOffset` is `widthTooNarrow ? 0 : 86` - before this runs;
        // `getXOrigin()` reads it)
        // java: int left = accessor().invokeGetXOrigin(); int top = accessor().invokeGetYOrigin();
        let (left, top) = compat::read(__this, |__t| (__t.base.get_x_origin(), __t.base.get_y_origin()));
        // Init Search Field only once
        if !SkyblockRecipeBookComponent::has_search_box(__this) {
            // java: this.searchBox = new EditBox(this.minecraft.font, left + 25, top + 13, SEARCH_FIELD_WIDTH, 14, Component.translatable("itemGroup.search"));
            let mut search_box = compat::edit_box(left.wrapping_add(25), top.wrapping_add(13), SEARCH_FIELD_WIDTH, 14, component::translatable("itemGroup.search"));
            compat::with_edit_box(&mut search_box, |e| {
                // java: this.searchBox.setMaxLength(60); - the longest Skyblock item name is 55 characters long
                e.set_max_length(60);
                // java: this.searchBox.setTextColor(CommonColors.WHITE);
                e.set_text_color(mcrust::net::minecraft::util::common_colors::WHITE);
            });
            // java: this.searchBox.setValue(lastSearch);
            compat::set_edit_box_value(&mut search_box, &compat::get_state(&LAST_SEARCH));
            // java: this.searchBox.setHint(SEARCH_HINT); - `Component.translatable("gui.recipebook.search_hint")
            // .withStyle(EditBox.SEARCH_HINT_STYLE)`
            compat::with_edit_box(&mut search_box, |e| e.set_hint(&component::translatable(mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::SEARCH_HINT_KEY).get_string(), mcrust::net::minecraft::client::gui::components::edit_box::search_hint_style()));
            compat::write(__this, |__t| __t.search_widgets.add_renderable_widget(search_box));
        }
        // java: this.searchBox.setX(left + 25); this.searchBox.setY(top + 13); this.searchBox.setVisible(true);
        let (search_x, search_y, search_right, search_height) = compat::write(__this, |__t| {
            let search_box = __t.search_widgets.widget_mut(0);
            search_box.set_x(left.wrapping_add(25));
            search_box.set_y(top.wrapping_add(13));
            search_box.visible = true;
            (search_box.get_x(), search_box.get_y(), search_box.get_right(), search_box.get_height())
        });
        // This field's name is misleading, the rectangle is actually the area of the magnifying glass icon rather than
        // the entire search field
        { let __v = Some(ScreenRectangle::of(ScreenAxis::Horizontal, left.wrapping_add(8), search_y, search_x.wrapping_sub(left), search_height)); compat::write(__this, |__t| __t.magnifier_icon_placement = __v); }
        // Init filter option once
        if compat::read(__this, |__t| __t.filter_option.is_none()) {
            // java: this.filterOption = new CyclingTextureWidget<>(this.searchBox.getRight() + 4, this.searchBox.getY(), 14, 14, FilterOption.ALL);
            let filter_option = CyclingTextureWidget::new(search_right.wrapping_add(4), search_y, 14, 14, FilterOption::All);
            { let __v = Some(filter_option.clone()); compat::write(__this, |__t| __t.filter_option = __v); }
            // java: this.filterOption.setCycleListener(this::refilterSearchResults);
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            CyclingTextureWidget::set_cycle_listener(&filter_option, Arc::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { SkyblockRecipeBookComponent::refilter_search_results((&__this_1), __a0.clone()); } }));
            // java: this.filterOption.setTextSupplier(option -> Component.translatable("skyblocker.config.general.itemList.filter." + option.name().toLowerCase(Locale.ENGLISH)));
            CyclingTextureWidget::set_text_supplier(&filter_option, Arc::new(|option: FilterOption| component::translatable(&format!("skyblocker.config.general.itemList.filter.{}", option.name().to_lowercase()))));
        }
        // Always update position of filter option
        let filter_option = compat::read(__this, |__t| __t.filter_option.clone()).unwrap();
        compat::write(&filter_option, |__t| __t.set_x(search_right.wrapping_add(4)));
        compat::write(&filter_option, |__t| __t.set_y(search_y));
        // Setup Tabs
        __this.lock().borrow_mut().tab_buttons.clear();
        for tab in compat::read(__this, |__t| __t.tabs.clone()) {
            let __h0 = (tab.clone(), SkyblockRecipeTabButton::new(({ let __f = compat::dispatch(&tab, |__t| __t.icon()); __f(&tab) })));
            __this.lock().borrow_mut().tab_buttons.push(__h0);
        }
        // Since we clear the tabs when this is called, if a tab is set we need to update the instance
        if let Some(current) = compat::read(__this, |__t| __t.current_tab.clone()) {
            let __v = compat::read(__this, |__t| __t.tab_buttons.clone()).into_iter().find(|button| std::sync::Arc::ptr_eq(&button.0, &current.0));
            compat::write(__this, |__t| __t.current_tab = __v);
        }
        // If there is no current tab, set it to the first one
        if compat::read(__this, |__t| __t.current_tab.is_none()) {
            { let __v = Some(__this.lock().borrow().tab_buttons[0].clone()); compat::write(__this, |__t| __t.current_tab = __v); }
        }
        let current = compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException");
        SkyblockRecipeTabButton::select(&current.1);
        SkyblockRecipeBookComponent::update_tabs(__this, false);
        // Tab Init
        ({ let __r = &current.0; let __f = compat::dispatch(__r, |__t| __t.initialize()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), compat::minecraft(), left, top) });
        // java: this.currentTab.left().updateSearchResults(this.searchBox.getValue().toLowerCase(Locale.ENGLISH), this.filterOption.getCurrent());
        let query: String = SkyblockRecipeBookComponent::search_box_value(__this).to_lowercase();
        ({ let __r = &current.0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &query, CyclingTextureWidget::get_current(&filter_option)) });
    }

    /// java: `this.searchBox != null`
    pub fn has_search_box(__this: &compat::Shared<Self>) -> bool {
        compat::read(__this, |__t| !__t.search_widgets.children().is_empty())
    }

    /// java: `this.searchBox.getValue()`
    pub fn search_box_value(__this: &compat::Shared<Self>) -> String {
        compat::read(__this, |__t| compat::edit_box_value(__t.search_widgets.widget(0)))
    }

    /// java: `this.searchBox.isFocused()`
    pub fn search_box_focused(__this: &compat::Shared<Self>) -> bool {
        compat::read(__this, |__t| __t.search_widgets.get_focused() == Some(0))
    }

    /// java: `this.searchBox.setFocused(focused)`
    pub fn set_search_box_focused(__this: &compat::Shared<Self>, focused: bool) {
        compat::write(__this, |__t| __t.search_widgets.set_focused(if focused { Some(0) } else { None }));
    }

    /// java: `this.searchBox.mouseClicked(click, doubled)` (`AbstractWidget.mouseClicked` -> `EditBox.onClick`, which
    /// moves the cursor to the click)
    pub fn search_box_mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, _doubled: bool) -> bool {
        // (the container's click goes to the child under the cursor - the box, when it is - and focuses it, which the
        // caller's `setFocused(true)` then repeats)
        compat::write(__this, |__t| compat::children_mouse_clicked(&mut __t.search_widgets, click))
    }

    /// java: `recipeBook.searchBox.extractRenderState(graphics, mouseX, mouseY, delta)`
    pub fn extract_search_box(__this: &compat::Shared<Self>, mouse_x: i32, mouse_y: i32) {
        let context = mcrust::mcrust::modding::widgets::WidgetRenderContext::new(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft());
        compat::write(__this, |__t| context.extract_widgets(&mut __t.search_widgets, mouse_x, mouse_y));
    }

    /// java: `recipeBook.filterOption.extractRenderState(graphics, mouseX, mouseY, delta)` (`AbstractWidget`'s): when
    /// visible, the hover state, `extractWidgetRenderState`, then `updateTooltip` - the widget's tooltip, while hovered
    pub fn extract_filter_option(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, delta: f32) {
        let Some(filter_option) = compat::read(__this, |__t| __t.filter_option.clone()) else {
            return;
        };
        if !compat::read(&filter_option, |__t| __t.visible) {
            return;
        }
        compat::write(&filter_option, |__t| __t.set_hovered_from_extract(mouse_x, mouse_y));
        CyclingTextureWidget::extract_widget_render_state(&filter_option, graphics, mouse_x, mouse_y, delta);
        if let Some(tooltip) = compat::read(&filter_option, |__t| if __t.is_hovered() { __t.tooltip.clone() } else { None }) {
            compat::batch_s_set_tooltip_for_next_frame(vec![tooltip]);
        }
    }

    /// java: `recipeBook.filterOption.mouseClicked(click, doubled)` (`AbstractWidget.mouseClicked`): a valid click on
    /// the active widget plays the down sound and runs its `onClick` (the cycle)
    pub fn filter_option_mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let Some(filter_option) = compat::read(__this, |__t| __t.filter_option.clone()) else {
            return false;
        };
        if !compat::read(&filter_option, |__t| __t.base.mouse_clicked(click.button(), click.x(), click.y()).is_some()) {
            return false;
        }
        compat::batch_w_play_down_sound();
        CyclingTextureWidget::on_click(&filter_option, click, doubled);
        true
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        SkyblockRecipeBookComponent::ensure_visuals(__this);
        if __this.lock().borrow().is_visible() {
            // java: int left = accessor().invokeGetXOrigin(); int top = accessor().invokeGetYOrigin();
            let (left, top) = compat::read(__this, |__t| (__t.base.get_x_origin(), __t.base.get_y_origin()));
            // java: graphics.blit(RenderPipelines.GUI_TEXTURED, RECIPE_BOOK_LOCATION, left, top, 1.0f, 1.0f, IMAGE_WIDTH, IMAGE_HEIGHT, 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::recipe_book_location(), left, top, 1.0f32, 1.0f32, *IMAGE_WIDTH, *IMAGE_HEIGHT, 256, 256);
            for tab_button in compat::read(__this, |__t| __t.tab_buttons.clone()) {
                // java: tabButton.right().extractRenderState(graphics, mouseX, mouseY, delta);
                SkyblockRecipeTabButton::extract_render_state(&tab_button.1, graphics, mouse_x, mouse_y, delta);
            }
            ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), graphics, left, top, mouse_x, mouse_y, delta) });
        }
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, @Nullable Slot slot)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut slot: Option<Slot>) {
        if __this.lock().borrow().is_visible() && compat::read(__this, |__t| __t.current_tab.is_some()) {
            ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.extract_tooltip()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), graphics, x, y) });
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        SkyblockRecipeBookComponent::ensure_visuals(__this);
        if __this.lock().borrow().is_visible() && !(mcrust::mcrust::modding::game::with_player(|player| player.is_spectator()).unwrap_or(false)) {
            if { let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.mouse_clicked()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), click, doubled) } {
                return true;
            } else {
                for tab_button in compat::read(__this, |__t| __t.tab_buttons.clone()) {
                    // java: if (tabButton.right().mouseClicked(click, doubled))
                    if SkyblockRecipeTabButton::mouse_clicked(&tab_button.1, click, doubled) {
                        // java: if (currentTab != tabButton) - the same pair object
                        if !compat::read(__this, |__t| __t.current_tab.clone()).is_some_and(|__c| std::sync::Arc::ptr_eq(&__c.0, &tab_button.0) && std::sync::Arc::ptr_eq(&__c.1, &tab_button.1)) {
                            if compat::read(__this, |__t| __t.current_tab.clone()).is_some() {
                                SkyblockRecipeTabButton::unselect(&compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").1);
                            }
                            { let __v = Some(tab_button.clone()); compat::write(__this, |__t| __t.current_tab = __v); }
                            SkyblockRecipeTabButton::select(&compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").1);
                        }
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        SkyblockRecipeBookComponent::ensure_visuals(__this);
        let client = compat::minecraft();
        if client.is_window_active() && compat::read(__this, |__t| __t.current_tab.clone()).is_some() {
            let mouse = &client.mouse_handler;
            let window = client.get_window();
            let mut mouse_x = (mouse.xpos() * (((window.get_gui_scaled_width()) as f64) / ((window.get_screen_width()) as f64)));
            let mut mouse_y = (mouse.ypos() * (((window.get_gui_scaled_height()) as f64) / ((window.get_screen_height()) as f64)));
            if { let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.key_pressed()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), mouse_x, mouse_y, input) } {
                return true;
            }
        }
        // java: return super.keyPressed(input); - `RecipeBookComponent.keyPressed:433-455`
        compat::write(__this, |__t| __t.ignore_text_input = false);
        if !compat::read(__this, |__t| __t.base.is_visible()) || mcrust::mcrust::modding::game::with_player(|player| player.is_spectator()).unwrap_or(false) {
            return false;
        }
        if input.is_escape() && !compat::read(__this, |__t| __t.base.is_offset_next_to_main_gui()) {
            SkyblockRecipeBookComponent::set_visible(__this, false);
            return true;
        }
        if compat::write(__this, |__t| compat::children_key_pressed(&mut __t.search_widgets, input)) {
            SkyblockRecipeBookComponent::check_search_string_update(__this);
            return true;
        }
        let search_visible = compat::read(__this, |__t| __t.search_widgets.widget(0).visible);
        if SkyblockRecipeBookComponent::search_box_focused(__this) && search_visible && !input.is_escape() {
            return true;
        }
        if mcrust::net::minecraft::client::key_mapping::matches("key.chat", input) && !SkyblockRecipeBookComponent::search_box_focused(__this) {
            compat::write(__this, |__t| __t.ignore_text_input = true);
            SkyblockRecipeBookComponent::set_search_box_focused(__this, true);
            return true;
        }
        // java: else if (event.isSelection() && this.lastRecipeCollection != null && this.lastRecipe != null) - the two
        // are set only by the base's own recipe buttons, which this book never shows (its pages are its tabs): null
        return false;
    }

    /// java: `RecipeBookComponent.keyReleased(KeyEvent)` (inherited): `ignoreTextInput = false`
    pub fn key_released(__this: &compat::Shared<Self>, _input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        compat::write(__this, |__t| __t.ignore_text_input = false);
        false
    }

    /// java: `RecipeBookComponent.charTyped(CharacterEvent)` (inherited, `:464-475`)
    pub fn char_typed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        SkyblockRecipeBookComponent::ensure_visuals(__this);
        if compat::read(__this, |__t| __t.ignore_text_input) {
            return false;
        }
        if !compat::read(__this, |__t| __t.base.is_visible()) || mcrust::mcrust::modding::game::with_player(|player| player.is_spectator()).unwrap_or(false) {
            return false;
        }
        if compat::write(__this, |__t| compat::children_char_typed(&mut __t.search_widgets, event)) {
            SkyblockRecipeBookComponent::check_search_string_update(__this);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean hasClickedOutside(double mouseX, double mouseY, int x, int y, int backgroundWidth, int backgroundHeight)
    pub fn has_clicked_outside(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut x: i32, mut y: i32, mut background_width: i32, mut background_height: i32) -> bool {
        SkyblockRecipeBookComponent::ensure_visuals(__this);
        if !(__this.lock().borrow().is_visible()) {
            return true;
        } else {
            let mut bl: bool = (mouse_x < ((x) as f64) || mouse_y < ((y) as f64) || mouse_x >= ((x.wrapping_add(background_width)) as f64) || mouse_y >= ((y.wrapping_add(background_height)) as f64));
            let mut bl2: bool = (((x.wrapping_sub(147)) as f64) < mouse_x && mouse_x < ((x) as f64) && ((y) as f64) < mouse_y && mouse_y < ((y.wrapping_add(background_height)) as f64));
            let __o1 = compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").1.clone();
            return (bl && !bl2 && !(compat::read(&__o1, |__t| __t.is_hovered_or_focused())));
        }
    }

    /// java: @Override protected void updateTabs(boolean filteringCraftable)
    pub fn update_tabs(__this: &compat::Shared<Self>, mut filtering_craftable: bool) {
        // java: int i = accessor().invokeGetXOrigin() - 30; int j = accessor().invokeGetYOrigin() + 3;
        let mut i: i32 = compat::read(__this, |__t| __t.base.get_x_origin()).wrapping_sub(30);
        let mut j: i32 = compat::read(__this, |__t| __t.base.get_y_origin()).wrapping_add(3);
        let mut l: i32 = 0;
        for tab_button in compat::read(__this, |__t| __t.tab_buttons.clone()) {
            let __wa1 = i;
            let __wa2 = j.wrapping_add(27i32.wrapping_mul({ let __v = l; l += 1; __v }));
            let __o1 = tab_button.1.clone();
            compat::write(&__o1, |__t| __t.set_position(__wa1, __wa2));
        }
    }

    /// java: protected void refilterSearchResults(FilterOption filterOption)
    pub fn refilter_search_results(__this: &compat::Shared<Self>, mut filter_option: FilterOption) {
        // java: assert this.searchBox != null;
        debug_assert!(SkyblockRecipeBookComponent::has_search_box(__this));
        compat::set_state(&LAST_SEARCH, SkyblockRecipeBookComponent::search_box_value(__this));
        let mut query: String = SkyblockRecipeBookComponent::search_box_value(__this).to_lowercase();
        // Doesn't trigger the pirate speak check since the query wasn't changed.
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results_3()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), query.clone(), filter_option, true) });
    }

    /// java: @Override protected void checkSearchStringUpdate()
    pub fn check_search_string_update(__this: &compat::Shared<Self>) {
        // java: assert this.searchBox != null;
        debug_assert!(SkyblockRecipeBookComponent::has_search_box(__this));
        compat::set_state(&LAST_SEARCH, SkyblockRecipeBookComponent::search_box_value(__this));
        let mut query: String = SkyblockRecipeBookComponent::search_box_value(__this).to_lowercase();
        FunUtils::recipe_book_easter_egg(&query);
        // java: this.pirateSpeechForThePeople(query); - `RecipeBookComponent.pirateSpeechForThePeople:511-525`: `"excitedze"`
        // switches the language to `en_pt` (`setSelected`, `options.languageCode`, `reloadResourcePacks`, `options.save`)
        mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::request_pirate_speech_for_the_people(&query);
        //Note: The rest of the query checks are implemented by the results class
        let filter_option = compat::read(__this, |__t| __t.filter_option.clone()).unwrap();
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &query, CyclingTextureWidget::get_current(&filter_option)) });
    }

    /// java: private RecipeBookComponentAccessor accessor()
    /// (`(RecipeBookComponentAccessor) this`: the object itself)
    pub fn accessor(__this: &compat::Shared<Self>) -> compat::Shared<Self> {
        return __this.clone();
    }

    /// java: @Override protected void updateCollections(boolean resetCurrentPage, boolean filteringCraftable)
    pub fn update_collections(__this: &compat::Shared<Self>, mut reset_current_page: bool, mut filtering_craftable: bool) {
        // java: assert this.searchBox != null;
        if !SkyblockRecipeBookComponent::has_search_box(__this) || compat::read(__this, |__t| __t.current_tab.is_none()) {
            return;
        }
        let query: String = SkyblockRecipeBookComponent::search_box_value(__this).to_lowercase();
        let filter_option = compat::read(__this, |__t| __t.filter_option.clone()).unwrap();
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &query, CyclingTextureWidget::get_current(&filter_option)) });
    }

    /// java: @Override protected @Nullable WidgetSprites getFilterButtonTextures()
    pub fn get_filter_button_textures(__this: &compat::Shared<Self>) -> Option<WidgetSprites> {
        return None;
    }

    /// java: @Override protected boolean isCraftingSlot(Slot slot)
    pub fn is_crafting_slot(__this: &compat::Shared<Self>, slot: &Slot) -> bool {
        return false;
    }

    /// java: @Override protected void selectMatchingRecipes(RecipeCollection recipeResultCollection, StackedItemContents recipeFinder)
    pub fn select_matching_recipes(__this: &compat::Shared<Self>, recipe_result_collection: &RecipeCollection, recipe_finder: &StackedItemContents) {
    }

    /// java: @Override protected Component getRecipeFilterName()
    pub fn get_recipe_filter_name(__this: &compat::Shared<Self>) -> Option<MutableComponent> {
        return None;
    }

    /// java: @Override protected void fillGhostRecipe(GhostSlots ghostRecipe, RecipeDisplay display, ContextMap context)
    pub fn fill_ghost_recipe(__this: &compat::Shared<Self>, ghost_recipe: &GhostSlots, display: &Arc<dyn RecipeDisplay>, context: &ContextMap) {
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

// (the book's only host is `SkyblockCraftingTableScreen`, whose overrides call these: the port's base does not dispatch to a subclass)

/// `new SkyblockRecipeBookComponent(player.inventoryMenu)` as the object the game's screen holds in place of its
/// vanilla book (`InventoryScreenMixin.skyblocker$replaceRecipeBook`): the game's `RecipeBookComponent` asks this
/// slot first at every entry point, and each one is answered by the overrides above (Java's virtual dispatch).
pub struct SkyblockRecipeBookSubclass {
    pub component: compat::RenderThread<compat::Shared<SkyblockRecipeBookComponent>>,
    /// The mouse of the frame's `extractRenderState` - `extractTooltip(graphics, x, y, slot)`'s `x` / `y`, which
    /// the game's `extractTooltip` is not handed.
    pub mouse: std::sync::Mutex<(i32, i32)>,
}

impl SkyblockRecipeBookSubclass {
    pub fn new(component: compat::Shared<SkyblockRecipeBookComponent>) -> Self {
        Self { component: compat::RenderThread::new(component), mouse: std::sync::Mutex::new((0, 0)) }
    }
}

impl mcrust::mcrust::modding::recipe_books::RecipeBookSubclass for SkyblockRecipeBookSubclass {
    fn init(&self, width: i32, height: i32, _gfx: &mcrust::net::minecraft::client::minecraft::Minecraft, width_too_narrow: bool) -> bool {
        SkyblockRecipeBookComponent::init(self.component.get(), width, height, compat::minecraft(), width_too_narrow);
        true
    }

    fn tick(&self, _gfx: &mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        SkyblockRecipeBookComponent::tick(self.component.get());
        true
    }

    fn toggle_visibility(&self, _gfx: &mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        SkyblockRecipeBookComponent::toggle_visibility(self.component.get());
        true
    }

    fn is_visible(&self) -> Option<bool> {
        Some(compat::read(self.component.get(), |__t| __t.base.is_visible()))
    }

    fn update_screen_position(&self, width: i32, image_width: i32) -> Option<i32> {
        Some(compat::read(self.component.get(), |__t| __t.base.update_screen_position(width, image_width)))
    }

    fn extract_render_state(&self, _gl: &mcrust::lwjgl::opengl::Gl, _gfx: &mcrust::net::minecraft::client::minecraft::Minecraft, mouse_x: i32, mouse_y: i32, a: f32) -> bool {
        *self.mouse.lock().unwrap_or_else(|p| p.into_inner()) = (mouse_x, mouse_y);
        SkyblockRecipeBookComponent::extract_render_state(self.component.get(), &GuiGraphicsExtractor::new(), mouse_x, mouse_y, a);
        true
    }

    fn extract_tooltip(&self, _hovered_slot: Option<usize>) -> Option<Option<mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::BookTooltip>> {
        let (x, y) = *self.mouse.lock().unwrap_or_else(|p| p.into_inner());
        SkyblockRecipeBookComponent::extract_tooltip(self.component.get(), &mut GuiGraphicsExtractor::new(), x, y, None);
        // the override draws its own tooltip
        Some(None)
    }

    fn extract_ghost_recipe(&self) -> bool {
        // `fillGhostRecipe` is a no-op override, so there is never a ghost recipe to draw
        true
    }

    fn fill_ghost_recipe(&self) -> bool {
        // java: @Override protected void fillGhostRecipe(GhostSlots ghostRecipe, RecipeDisplay display, ContextMap context) {}
        true
    }

    fn mouse_clicked(&self, button: i32, x: f64, y: f64, double_click: bool, shift_down: bool) -> Option<bool> {
        let modifiers = if shift_down { mcrust::com::mojang::blaze3d::platform::input_constants::MOD_SHIFT } else { 0 };
        let click = MouseButtonEvent::new(x, y, mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo::new(button, modifiers));
        Some(SkyblockRecipeBookComponent::mouse_clicked(self.component.get(), &click, double_click))
    }

    fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> Option<bool> {
        Some(SkyblockRecipeBookComponent::key_pressed(self.component.get(), event))
    }

    fn key_released(&self) -> Option<bool> {
        // the inherited `keyReleased` reads no field of the event
        Some(SkyblockRecipeBookComponent::key_released(self.component.get(), &mcrust::net::minecraft::client::input::key_event::KeyEvent::new(0, 0, 0)))
    }

    fn char_typed(&self, c: char) -> Option<bool> {
        Some(SkyblockRecipeBookComponent::char_typed(self.component.get(), &mcrust::net::minecraft::client::input::character_event::CharacterEvent::new(c as i32)))
    }

    fn has_clicked_outside(&self, mx: f64, my: f64, left_pos: i32, top_pos: i32, image_width: i32, image_height: i32) -> Option<bool> {
        Some(SkyblockRecipeBookComponent::has_clicked_outside(self.component.get(), mx, my, left_pos, top_pos, image_width, image_height))
    }
}
