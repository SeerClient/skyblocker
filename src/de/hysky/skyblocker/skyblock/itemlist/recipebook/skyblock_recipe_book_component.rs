//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockRecipeBookComponent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::noop_recipe_book_menu::NoopRecipeBookMenu;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_tab::RecipeTab;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_tab_button::SkyblockRecipeTabButton;
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
pub struct SkyblockRecipeBookComponent {
    /// java: extends net.minecraft.client.gui.screens.recipebook.RecipeBookComponent
    pub base: RecipeBookComponent,
    /// java: private final List<RecipeTab> tabs = List.of( new SkyblockCraftingTab(this, SkyblockCraftingTab.CRAFTING_TABLE, new SkyblockRecipeResults()), new UpcomingEvents
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
        // TODO(fabric2seer): `new` takes 7 argument(s) in the port (`book_type: RecipeBookType, tab_infos: Vec<TabInfo>, filter_button_sprites: WidgetSprites, recipe_filter_name_key: &'static str, select_matching_recipes_body: SelectMatchingRecipes, fill_ghost_recipe_body: FillGhostRecipe, is_furnace_menu: b ..`), Java passes 2
        //   java: super(new NoopRecipeBookMenu(screenHandler.containerId), List.of());
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override public void init(int parentWidth, int parentHeight, Minecraft client, boolean narrow)
    pub fn init(__this: &compat::Shared<Self>, mut parent_width: i32, mut parent_height: i32, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut narrow: bool) {
        // TODO(fabric2seer): `init` takes 6 argument(s) in the port (`width: i32, height: i32, book: Arc<Mutex<ClientRecipeBook>>, times_inventory_changed: i32, gfx: &Minecraft, width_too_narrow: bool`), Java passes 4
        // TODO(fabric2seer): the mod keeps the game's `Minecraft` (Java holds a reference to the live object); the port's `Minecraft` is not a value to copy and the port lends the mod no handle to it
        // rust: { let (__q0, __q1, __q2, __q3,) = (parent_width, parent_height, client.clone(), narrow,); __this.lock().borrow_mut().base.init(__q0, __q1, __q2, __q3) };
    }

    /// java: @Override protected void initVisuals()
    pub fn init_visuals(__this: &compat::Shared<Self>) {
        // java: this.xOffset = this.widthTooNarrow ? 0 : OFFSET_X_POSITION;
        // rust: port gap - `RecipeBookComponent.xOffset` / `widthTooNarrow` (protected) are private to the port's class, so a
        // mod subclass cannot write them (GAPS_batchF.md)
        // TODO(fabric2seer): `.invokeGetXOrigin(..)` on a RecipeBookComponentAccessor
        //   java: int left = accessor().invokeGetXOrigin();
        let mut left: i32 = todo!("fabric2seer: the initialiser of `left` (TODO above)");
        // TODO(fabric2seer): `.invokeGetYOrigin(..)` on a RecipeBookComponentAccessor
        //   java: int top = accessor().invokeGetYOrigin();
        let mut top: i32 = todo!("fabric2seer: the initialiser of `top` (TODO above)");
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        if false /* java: compat::read(__this, |__t| __t.search_box.clone()) == null: never null here */ {
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
            // rust: { let __v = EditBox::new(&compat::font(), component::translatable("itemGroup.search"), left.wrapping_add(25), top.wrapping_add(13), SEARCH_FIELD_WIDTH, 14); compat::write(__this, |__t| __t.search_box = __v); }
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // rust: __this.lock().borrow_mut().search_box.set_max_length(60);
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // rust: __this.lock().borrow_mut().search_box.set_text_color(mcrust::net::minecraft::util::common_colors::WHITE);
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: { let (__q0,) = (&compat::get_state(&LAST_SEARCH),); __this.lock().borrow_mut().search_box.set_value(__q0) };
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // TODO(fabric2seer): `RecipeBookComponent.SEARCH_HINT` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // rust: let __r0 = compat::read(__this, |__t| __t.search_hint.clone());
            // rust: { let (__q0,) = (__r0,); __this.lock().borrow_mut().search_box.set_hint_component(__q0) };
        }
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // TODO(fabric2seer): `set_x` takes 2 argument(s) in the port (`x: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let (__q0,) = (left.wrapping_add(25),); __this.lock().borrow_mut().search_box.set_x(__q0) };
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // TODO(fabric2seer): `set_y` takes 2 argument(s) in the port (`y: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let (__q0,) = (top.wrapping_add(13),); __this.lock().borrow_mut().search_box.set_y(__q0) };
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // TODO(fabric2seer): `EditBox.setVisible` has no Rust method in the port (looked for `set_visible` in EditBox and its supertypes)
        // rust: __this.lock().borrow().search_box.set_visible(true);
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: { let __v = Some(ScreenRectangle::of(ScreenAxis::Horizontal, left.wrapping_add(8), __this.lock().borrow().search_box.get_y(), (__this.lock().borrow().search_box.get_x()).wrapping_sub(left), __this.lock().borrow().search_box.get_height())); compat::write(__this, |__t| __t.magnifier_icon_placement = __v); }
        if compat::read(__this, |__t| __t.filter_option.clone()).is_none() {
            // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // rust: { let __v = Some(CyclingTextureWidget::new((__this.lock().borrow().search_box.get_right()).wrapping_add(4), __this.lock().borrow().search_box.get_y(), 14, 14, FilterOption::All)); compat::write(__this, |__t| __t.filter_option = __v); }
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            CyclingTextureWidget::set_cycle_listener(&compat::read(__this, |__t| __t.filter_option.clone()).unwrap(), Arc::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { SkyblockRecipeBookComponent::refilter_search_results((&__this_1), __a0.clone()); } }));
            // TODO(fabric2seer): `.name(..)` on a T
            //   java: this.filterOption.setTextSupplier(option -> Component.translatable("skyblocker.config.general.itemList.filter." + option.name().toLowerCase(Locale.ENGLISH)));
        }
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: let __wa1 = (__this.lock().borrow().search_box.get_right()).wrapping_add(4);
        // rust: let __o2 = compat::read(__this, |__t| __t.filter_option.clone()).unwrap();
        // rust: compat::write(&__o2, |__t| __t.set_x(__wa1));
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: let __wa2 = __this.lock().borrow().search_box.get_y();
        // rust: let __o3 = compat::read(__this, |__t| __t.filter_option.clone()).unwrap();
        // rust: compat::write(&__o3, |__t| __t.set_y(__wa2));
        __this.lock().borrow_mut().tab_buttons.clear();
        for tab in compat::read(__this, |__t| __t.tabs.clone()) {
            let __h0 = (tab.clone(), SkyblockRecipeTabButton::new(({ let __f = compat::dispatch(&tab, |__t| __t.icon()); __f(&tab) })));
            __this.lock().borrow_mut().tab_buttons.push(__h0);
        }
        if compat::read(__this, |__t| __t.current_tab.clone()).is_some() {
            let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            { let __v = compat::read(__this, |__t| __t.tab_buttons.clone()).into_iter().filter(|button| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; std::sync::Arc::ptr_eq(&button.0, &compat::read((&__this_4), |__t| __t.current_tab.clone()).expect("NullPointerException").0) }).next(); compat::write(__this, |__t| __t.current_tab = __v); }
        }
        if compat::read(__this, |__t| __t.current_tab.clone()).is_none() {
            { let __v = Some(__this.lock().borrow().tab_buttons[0].clone()); compat::write(__this, |__t| __t.current_tab = __v); }
        }
        SkyblockRecipeTabButton::select(&compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").1);
        SkyblockRecipeBookComponent::update_tabs(__this, false);
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.initialize()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), compat::minecraft(), left, top) });
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &__this.lock().borrow().search_box.get_value().to_lowercase(), CyclingTextureWidget::get_current(&compat::read(__this, |__t| __t.filter_option.clone()).unwrap())) });
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        if __this.lock().borrow().is_visible() {
            // TODO(fabric2seer): `.invokeGetXOrigin(..)` on a RecipeBookComponentAccessor
            //   java: int left = accessor().invokeGetXOrigin();
            let mut left: i32 = todo!("fabric2seer: the initialiser of `left` (TODO above)");
            // TODO(fabric2seer): `.invokeGetYOrigin(..)` on a RecipeBookComponentAccessor
            //   java: int top = accessor().invokeGetYOrigin();
            let mut top: i32 = todo!("fabric2seer: the initialiser of `top` (TODO above)");
            // TODO(fabric2seer): `RecipeBookComponent.RECIPE_BOOK_LOCATION` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
            // rust: gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.recipe_book_location.clone()), left, top, 1.0f32, 1.0f32, *IMAGE_WIDTH, *IMAGE_HEIGHT, 256, 256);
            for tab_button in compat::read(__this, |__t| __t.tab_buttons.clone()) {
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: let __wa1 = graphics.clone();
                // rust: let __wa2 = mouse_x;
                // rust: let __wa3 = mouse_y;
                // rust: let __wa4 = delta;
                // rust: let __o1 = tab_button.1.clone();
                // rust: compat::write(&__o1, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
            }
            ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), graphics, left, top, mouse_x, mouse_y, delta) });
        }
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y, @Nullable Slot slot)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut slot: Option<Slot>) {
        if __this.lock().borrow().is_visible() {
            ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.extract_tooltip()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), graphics, x, y) });
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if __this.lock().borrow().is_visible() && !(mcrust::mcrust::modding::game::with_player(|player| player.is_spectator()).unwrap_or(false)) {
            if { let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.mouse_clicked()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), click, doubled) } {
                return true;
            } else {
                for tab_button in compat::read(__this, |__t| __t.tab_buttons.clone()) {
                    // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
                    let __wa1 = click.clone();
                    let __wa2 = doubled;
                    let __o1 = tab_button.1.clone();
                    let __wa3 = click.clone();
                    let __wa4 = doubled;
                    let __o2 = tab_button.1.clone();
                    if compat::write(&__o2, |__t| __t.mouse_clicked(__wa3.button(), __wa3.x(), __wa3.y()).is_some()) {
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
        // TODO(fabric2seer): `key_pressed` takes 6 argument(s) in the port (`event: KeyEvent, matches_chat_key: bool, is_spectator: bool, gfx: &Minecraft, clipboard: &mut dyn ClipboardAccess, handle_place_recipe: &mut dyn FnMut(i32, bool)`), Java passes 1
        // rust: return { let (__q0,) = (input.clone(),); __this.lock().borrow_mut().base.key_pressed(__q0) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean hasClickedOutside(double mouseX, double mouseY, int x, int y, int backgroundWidth, int backgroundHeight)
    pub fn has_clicked_outside(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut x: i32, mut y: i32, mut background_width: i32, mut background_height: i32) -> bool {
        if !(__this.lock().borrow().is_visible()) {
            return true;
        } else {
            let mut bl: bool = (mouse_x < ((x) as f64) || mouse_y < ((y) as f64) || mouse_x >= ((x.wrapping_add(background_width)) as f64) || mouse_y >= ((y.wrapping_add(background_height)) as f64));
            let mut bl2: bool = (((x.wrapping_sub(147)) as f64) < mouse_x && mouse_x < ((x) as f64) && ((y) as f64) < mouse_y && mouse_y < ((y.wrapping_add(background_height)) as f64));
            let __o1 = compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").1.clone();
            return (bl && !bl2 && !(compat::write(&__o1, |__t| __t.is_hovered_or_focused())));
        }
    }

    /// java: @Override protected void updateTabs(boolean filteringCraftable)
    pub fn update_tabs(__this: &compat::Shared<Self>, mut filtering_craftable: bool) {
        // TODO(fabric2seer): `.invokeGetXOrigin(..)` on a RecipeBookComponentAccessor
        //   java: int i = accessor().invokeGetXOrigin() - 30;
        let mut i: i32 = todo!("fabric2seer: the initialiser of `i` (TODO above)");
        // TODO(fabric2seer): `.invokeGetYOrigin(..)` on a RecipeBookComponentAccessor
        //   java: int j = accessor().invokeGetYOrigin() + 3;
        let mut j: i32 = todo!("fabric2seer: the initialiser of `j` (TODO above)");
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
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: compat::set_state(&LAST_SEARCH, __this.lock().borrow().search_box.get_value());
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: let mut query: String = __this.lock().borrow().search_box.get_value().to_lowercase();
        let mut query: String = todo!("fabric2seer: the initialiser of `query` (TODO above)");
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results_3()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), query.clone(), filter_option, true) });
    }

    /// java: @Override protected void checkSearchStringUpdate()
    pub fn check_search_string_update(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: compat::set_state(&LAST_SEARCH, __this.lock().borrow().search_box.get_value());
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: let mut query: String = __this.lock().borrow().search_box.get_value().to_lowercase();
        let mut query: String = todo!("fabric2seer: the initialiser of `query` (TODO above)");
        FunUtils::recipe_book_easter_egg(&query);
        // java: this.pirateSpeechForThePeople(query);
        // rust: port gap - `RecipeBookComponent.pirateSpeechForThePeople` (private in Java too, reached through the mod's
        // accessor) is private to the port's class (GAPS_batchF.md)
        ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &query, CyclingTextureWidget::get_current(&compat::read(__this, |__t| __t.filter_option.clone()).unwrap())) });
    }

    /// java: private RecipeBookComponentAccessor accessor()
    /// (`(RecipeBookComponentAccessor) this`: the object itself)
    pub fn accessor(__this: &compat::Shared<Self>) -> compat::Shared<Self> {
        return __this.clone();
    }

    /// java: @Override protected void updateCollections(boolean resetCurrentPage, boolean filteringCraftable)
    pub fn update_collections(__this: &compat::Shared<Self>, mut reset_current_page: bool, mut filtering_craftable: bool) {
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        debug_assert!(true /* java: compat::read(__this, |__t| __t.search_box.clone()) != null: never null here */);
        // TODO(fabric2seer): `RecipeBookComponent.searchBox` has no Rust field in the port (searched net.minecraft.client.gui.screens.recipebook.RecipeBookComponent and its supertypes)
        // rust: ({ let __r = &compat::read(__this, |__t| __t.current_tab.clone()).expect("NullPointerException").0; let __f = compat::dispatch(__r, |__t| __t.update_search_results()); __f(&compat::view(__r, |a| a as compat::Shared<dyn RecipeAreaDisplay>), &__this.lock().borrow().search_box.get_value().to_lowercase(), CyclingTextureWidget::get_current(&compat::read(__this, |__t| __t.filter_option.clone()).unwrap())) });
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

// NOTE(fabric2seer): checkSearchStringUpdate, extractRenderState, extractTooltip, fillGhostRecipe, getFilterButtonTextures, getRecipeFilterName, hasClickedOutside, init, initVisuals, isCraftingSlot, keyPressed, mouseClicked, selectMatchingRecipes, updateCollections, updateNarration, updateTabs override(s) of RecipeBookComponent - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
