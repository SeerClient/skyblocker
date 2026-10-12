//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.InventoryPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::inventory::Inventory;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::backpack_item_loader::BackpackItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::pets_inventory_item_loader::PetsInventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::wardrobe_inventory_item_loader::WardrobeInventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::player_inventory::PlayerInventory;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::sub_page_select_button::SubPageSelectButton;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final String[] INVENTORY_PAGES = {"inventory", "enderchest", "backpack", "wardrobe", "pets", "accessoryBag"};
pub static INVENTORY_PAGES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["inventory".to_string(), "enderchest".to_string(), "backpack".to_string(), "wardrobe".to_string(), "pets".to_string(), "accessoryBag".to_string()]);
/// java: private static final int TOTAL_HEIGHT = 165;
pub const TOTAL_HEIGHT: i32 = 165;
/// java: private static final Map<String, FlexibleItemStack> ICON_MAP = Map.ofEntries( Map.entry("wardrobe", Ico.L_CHESTPLATE), Map.entry("inventory", Ico.CHEST), Map.entry("backpack", ProfileViewerUtils.creat
pub static ICON_MAP: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("wardrobe".to_string(), ico::L_CHESTPLATE.get_mut().clone()), ("inventory".to_string(), ico::CHEST.get_mut().clone()), ("backpack".to_string(), ProfileViewerUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHBzOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzYyZjNiM2EwNTQ4MWNkZTc3MjQwMDA1YzBkZGNlZTFjMDY5ZTU1MDRhNjJjZTA5Nzc4NzlmNTVhMzkzOTYxNDYifX19")), ("pets".to_string(), ico::BONE.get_mut().clone()), ("enderchest".to_string(), ico::E_CHEST.get_mut().clone()), ("accessoryBag".to_string(), ProfileViewerUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvOTYxYTkxOGMwYzQ5YmE4ZDA1M2U1MjJjYjkxYWJjNzQ2ODkzNjdiNGQ4YWEwNmJmYzFiYTkxNTQ3MzA5ODVmZiJ9fX0="))]));
/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class InventoryPage implements ProfileViewerPage
#[derive(Clone)]
pub struct InventoryPage {
    /// java: private final @Nullable ProfileViewerPage[] inventorySubPages = new ProfileViewerPage[6];
    pub inventory_sub_pages: Option<Vec<compat::Shared<dyn ProfileViewerPage>>>,
    /// java: private final List<SubPageSelectButton> inventorySelectButtons = new ArrayList<>();
    pub inventory_select_buttons: Vec<compat::Shared<SubPageSelectButton>>,
    /// java: private int activePage = 0;
    pub active_page: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl InventoryPage {
    /// java: public InventoryPage(JsonObject pProfile)
    pub fn new(mut p_profile: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            inventory_sub_pages: Some((0..(6) as usize).map(|_| Default::default()).collect::<Vec<_>>()),
            inventory_select_buttons: Vec::new(),
            active_page: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        for i in 0..(INVENTORY_PAGES.len() as i32) {
            let __h0 = SubPageSelectButton::new((__this.clone() as compat::Shared<dyn ProfileViewerPage>), -100, 0, i, ICON_MAP.get_mut().get(&INVENTORY_PAGES[(i) as usize]).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone()));
            __this.lock().borrow_mut().inventory_select_buttons.push(__h0);
        }
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let mut inventory_data: Option<serde_json::Map<String, serde_json::Value>> = Some(compat::json::get_as_json_object_of(&p_profile, "inventory"));
            let Some(mut inventory_data) = inventory_data.clone() else {
                return __this.clone();
            };
            __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(0) as usize] = (PlayerInventory::new(inventory_data.clone()) as compat::Shared<dyn ProfileViewerPage>);
            if compat::json::has(&inventory_data, "ender_chest_contents") {
                __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(1) as usize] = (Inventory::new(INVENTORY_PAGES[(1) as usize].clone(), (5, 9), compat::json::get_as_json_object_of(&inventory_data, "ender_chest_contents")) as compat::Shared<dyn ProfileViewerPage>);
            }
            if compat::json::has(&inventory_data, "backpack_contents") {
                __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(2) as usize] = (Inventory::new_4(INVENTORY_PAGES[(2) as usize].clone(), (5, 9), compat::json::get_as_json_object_of(&inventory_data, "backpack_contents"), (BackpackItemLoader::new() as compat::Shared<dyn ItemLoader>)) as compat::Shared<dyn ProfileViewerPage>);
            }
            if compat::json::has(&inventory_data, "wardrobe_contents") {
                __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(3) as usize] = (Inventory::new_4(INVENTORY_PAGES[(3) as usize].clone(), (4, 9), compat::json::get_as_json_object_of(&inventory_data, "wardrobe_contents"), (WardrobeInventoryItemLoader::new(inventory_data.clone()) as compat::Shared<dyn ItemLoader>)) as compat::Shared<dyn ProfileViewerPage>);
            }
            __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(4) as usize] = (Inventory::new_4(INVENTORY_PAGES[(4) as usize].clone(), (4, 9), p_profile.clone(), (PetsInventoryItemLoader::new() as compat::Shared<dyn ItemLoader>)) as compat::Shared<dyn ProfileViewerPage>);
            if compat::json::has(&inventory_data, "bag_contents") && compat::json::has(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag") {
                __this.lock().borrow_mut().inventory_sub_pages.as_mut().unwrap()[(5) as usize] = (Inventory::new(INVENTORY_PAGES[(5) as usize].clone(), (5, 9), compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&inventory_data, "bag_contents")), "talisman_bag")) as compat::Shared<dyn ProfileViewerPage>);
            }
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			ProfileViewerScreen.LOGGER.error("[Skyblocker Profile Viewer] Error while loading inventory data: ", e);
        //   java: 		}
        __this
    }

}

impl ProfileViewerPage for InventoryPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        InventoryPage::extract_render_state_dyn
    }

    /// java: public void onNavButtonClick(SubPageSelectButton clickedButton) (dispatch: the body for this class)
    fn on_nav_button_click(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &compat::Shared<SubPageSelectButton>) {
        InventoryPage::on_nav_button_click_dyn
    }

    /// java: @Override public List<AbstractWidget> getButtons() (dispatch: the body for this class)
    fn get_buttons(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        InventoryPage::get_buttons_dyn
    }

    /// java: @Override public void markWidgetsAsVisible() (dispatch: the body for this class)
    fn mark_widgets_as_visible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        InventoryPage::mark_widgets_as_visible_dyn
    }

    /// java: @Override public void markWidgetsAsInvisible() (dispatch: the body for this class)
    fn mark_widgets_as_invisible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        InventoryPage::mark_widgets_as_invisible_dyn
    }

}

impl InventoryPage {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        let mut starting_y: i32 = root_y.wrapping_add((TOTAL_HEIGHT.wrapping_sub((__this.lock().borrow().inventory_select_buttons.len() as i32).wrapping_mul(21))).wrapping_div(2));
        for i in 0..(__this.lock().borrow().inventory_select_buttons.len() as i32) {
            let __wa1 = root_x;
            let __o1 = __this.lock().borrow().inventory_select_buttons[(i) as usize].clone();
            compat::write(&__o1, |__t| __t.set_x(__wa1));
            let __wa2 = starting_y.wrapping_add(i.wrapping_mul(21));
            let __o2 = __this.lock().borrow().inventory_select_buttons[(i) as usize].clone();
            compat::write(&__o2, |__t| __t.set_y(__wa2));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa3 = graphics.clone();
            // rust: let __wa4 = mouse_x;
            // rust: let __wa5 = mouse_y;
            // rust: let __wa6 = delta;
            // rust: let __o3 = __this.lock().borrow().inventory_select_buttons[(i) as usize].clone();
            // rust: compat::write(&__o3, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6));
        }
        if false /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] == null: never null here */ {
            // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
            //   java: graphics.text(textRenderer, "No data...", rootX + 92, rootY + 72, Color.DARK_GRAY.getRGB(), false);
            return;
        }
        ({ let __r = &__this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_visible()); __f(__r) });
        ({ let __r = &__this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(__r, graphics, mouse_x, mouse_y, delta, root_x.wrapping_add(35), root_y.wrapping_add(6)) });
    }

    /// InventoryPage.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: public void onNavButtonClick(SubPageSelectButton clickedButton)
    pub fn on_nav_button_click_impl(__this: &compat::Shared<Self>, clicked_button: &compat::Shared<SubPageSelectButton>) {
        if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_invisible()); __f(__r) });
        }
        for button in compat::read(__this, |__t| __t.inventory_select_buttons.clone()) {
            SubPageSelectButton::set_toggled(&button, false);
        }
        { let __v = SubPageSelectButton::get_index(clicked_button); compat::write(__this, |__t| __t.active_page = __v); }
        SubPageSelectButton::set_toggled(clicked_button, true);
    }

    /// InventoryPage.onNavButtonClick as the dispatch of ProfileViewerPage.onNavButtonClick
    pub fn on_nav_button_click_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &compat::Shared<SubPageSelectButton>) {
        Self::on_nav_button_click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public List<AbstractWidget> getButtons()
    pub fn get_buttons_impl(__this: &compat::Shared<Self>) -> Option<Vec<AbstractWidget>> {
        let mut clickable_widgets: Vec<AbstractWidget> = compat::read(__this, |__t| __t.inventory_select_buttons.clone()).into_iter().collect::<Vec<_>>();
        for page in __this.lock().borrow().inventory_sub_pages.as_ref().unwrap().iter() {
            if true /* java: page != null: never null here */ && ({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).is_some() {
                clickable_widgets.extend(({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).unwrap().iter().cloned());
            }
        }
        return Some(clickable_widgets);
    }

    /// InventoryPage.getButtons as the dispatch of ProfileViewerPage.getButtons
    pub fn get_buttons_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        Self::get_buttons_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsVisible()
    pub fn mark_widgets_as_visible_impl(__this: &compat::Shared<Self>) {
        if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_visible()); __f(__r) });
        }
        for button in compat::read(__this, |__t| __t.inventory_select_buttons.clone()) {
            { let __v = true; compat::write(&button, |__t| __t.visible = __v); }
            { let __v = true; compat::write(&button, |__t| __t.active = __v); }
        }
    }

    /// InventoryPage.markWidgetsAsVisible as the dispatch of ProfileViewerPage.markWidgetsAsVisible
    pub fn mark_widgets_as_visible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_visible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsInvisible()
    pub fn mark_widgets_as_invisible_impl(__this: &compat::Shared<Self>) {
        if true /* java: __this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().inventory_sub_pages.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_invisible()); __f(__r) });
        }
        for button in compat::read(__this, |__t| __t.inventory_select_buttons.clone()) {
            { let __v = false; compat::write(&button, |__t| __t.visible = __v); }
            { let __v = false; compat::write(&button, |__t| __t.active = __v); }
        }
    }

    /// InventoryPage.markWidgetsAsInvisible as the dispatch of ProfileViewerPage.markWidgetsAsInvisible
    pub fn mark_widgets_as_invisible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_invisible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
