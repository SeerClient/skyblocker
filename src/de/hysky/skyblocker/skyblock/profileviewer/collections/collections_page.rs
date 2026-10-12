//! Ported from de.hysky.skyblocker.skyblock.profileviewer.collections.CollectionsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::collections::generic_category::GenericCategory;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::sub_page_select_button::SubPageSelectButton;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final String[] COLLECTION_CATEGORIES = {"MINING", "FARMING", "COMBAT", "FISHING", "FORAGING", "RIFT"};
pub static COLLECTION_CATEGORIES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["MINING".to_string(), "FARMING".to_string(), "COMBAT".to_string(), "FISHING".to_string(), "FORAGING".to_string(), "RIFT".to_string()]);
/// java: private static final int TOTAL_HEIGHT = 165;
pub const TOTAL_HEIGHT: i32 = 165;
/// java: private static final Map<String, FlexibleItemStack> ICON_MAP = Map.ofEntries( Map.entry("MINING", Ico.STONE_PICKAXE), Map.entry("FARMING", Ico.GOLDEN_HOE), Map.entry("COMBAT", Ico.STONE_SWORD), Map.en
pub static ICON_MAP: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("MINING".to_string(), ico::STONE_PICKAXE.get_mut().clone()), ("FARMING".to_string(), ico::GOLDEN_HOE.get_mut().clone()), ("COMBAT".to_string(), ico::STONE_SWORD.get_mut().clone()), ("FISHING".to_string(), ico::FISH_ROD.get_mut().clone()), ("FORAGING".to_string(), ico::JUNGLE_SAPLING.get_mut().clone()), ("RIFT".to_string(), ico::MYCELIUM.get_mut().clone())]));
/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class CollectionsPage implements ProfileViewerPage
#[derive(Clone)]
pub struct CollectionsPage {
    /// java: private final @Nullable GenericCategory[] collections = new GenericCategory[COLLECTION_CATEGORIES.length];
    pub collections: Option<Vec<compat::Shared<GenericCategory>>>,
    /// java: private final List<SubPageSelectButton> collectionSelectButtons = new ArrayList<>();
    pub collection_select_buttons: Vec<compat::Shared<SubPageSelectButton>>,
    /// java: private int activePage = 0;
    pub active_page: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl CollectionsPage {
    /// java: public CollectionsPage(JsonObject hProfile, JsonObject pProfile)
    pub fn new(mut h_profile: serde_json::Map<String, serde_json::Value>, mut p_profile: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            collections: Some((0..((COLLECTION_CATEGORIES.len() as i32)) as usize).map(|_| Default::default()).collect::<Vec<_>>()),
            collection_select_buttons: Vec::new(),
            active_page: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        for i in 0..(COLLECTION_CATEGORIES.len() as i32) {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let __h0 = SubPageSelectButton::new((__this.clone() as compat::Shared<dyn ProfileViewerPage>), -100, 0, i, ICON_MAP.get_mut().get(&COLLECTION_CATEGORIES[(i) as usize]).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone()));
                __this.lock().borrow_mut().collection_select_buttons.push(__h0);
                __this.lock().borrow_mut().collections.as_mut().unwrap()[(i) as usize] = GenericCategory::new(h_profile.clone(), p_profile.clone(), COLLECTION_CATEGORIES[(i) as usize].clone());
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Profile Viewer] Error creating Collections Page"));
            }
        }
        __this
    }

}

impl ProfileViewerPage for CollectionsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        CollectionsPage::extract_render_state_dyn
    }

    /// java: public void onNavButtonClick(SubPageSelectButton selectButton) (dispatch: the body for this class)
    fn on_nav_button_click(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &compat::Shared<SubPageSelectButton>) {
        CollectionsPage::on_nav_button_click_dyn
    }

    /// java: @Override public List<AbstractWidget> getButtons() (dispatch: the body for this class)
    fn get_buttons(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        CollectionsPage::get_buttons_dyn
    }

    /// java: @Override public void markWidgetsAsVisible() (dispatch: the body for this class)
    fn mark_widgets_as_visible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        CollectionsPage::mark_widgets_as_visible_dyn
    }

    /// java: @Override public void markWidgetsAsInvisible() (dispatch: the body for this class)
    fn mark_widgets_as_invisible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        CollectionsPage::mark_widgets_as_invisible_dyn
    }

}

impl CollectionsPage {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32, mut root_x: i32, mut root_y: i32) {
        let mut starting_y: i32 = root_y.wrapping_add((TOTAL_HEIGHT.wrapping_sub((__this.lock().borrow().collection_select_buttons.len() as i32).wrapping_mul(21))).wrapping_div(2));
        for i in 0..(__this.lock().borrow().collection_select_buttons.len() as i32) {
            let __wa1 = root_x;
            let __o1 = __this.lock().borrow().collection_select_buttons[(i) as usize].clone();
            compat::write(&__o1, |__t| __t.set_x(__wa1));
            let __wa2 = starting_y.wrapping_add(i.wrapping_mul(21));
            let __o2 = __this.lock().borrow().collection_select_buttons[(i) as usize].clone();
            compat::write(&__o2, |__t| __t.set_y(__wa2));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa3 = graphics.clone();
            // rust: let __wa4 = mouse_x;
            // rust: let __wa5 = mouse_y;
            // rust: let __wa6 = a;
            // rust: let __o3 = __this.lock().borrow().collection_select_buttons[(i) as usize].clone();
            // rust: compat::write(&__o3, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6));
        }
        if false /* java: __this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] == null: never null here */ {
            // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
            //   java: graphics.text(textRenderer, "No data...", rootX + 92, rootY + 72, Color.DARK_GRAY.getRGB(), false);
            return;
        }
        ({ let __r = &__this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_visible()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ProfileViewerPage>)) });
        ({ let __r = &__this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.extract_render_state()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ProfileViewerPage>), graphics, mouse_x, mouse_y, a, root_x.wrapping_add(35), root_y.wrapping_add(6)) });
    }

    /// CollectionsPage.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: public void onNavButtonClick(SubPageSelectButton selectButton)
    pub fn on_nav_button_click_impl(__this: &compat::Shared<Self>, select_button: &compat::Shared<SubPageSelectButton>) {
        if true /* java: __this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize] != null: never null here */ {
            ({ let __r = &__this.lock().borrow().collections.as_ref().unwrap()[(compat::read(__this, |__t| __t.active_page)) as usize]; let __f = compat::dispatch(__r, |__t| __t.mark_widgets_as_invisible()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ProfileViewerPage>)) });
        }
        for button in compat::read(__this, |__t| __t.collection_select_buttons.clone()) {
            SubPageSelectButton::set_toggled(&button, false);
        }
        { let __v = SubPageSelectButton::get_index(select_button); compat::write(__this, |__t| __t.active_page = __v); }
        SubPageSelectButton::set_toggled(select_button, true);
    }

    /// CollectionsPage.onNavButtonClick as the dispatch of ProfileViewerPage.onNavButtonClick
    pub fn on_nav_button_click_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &compat::Shared<SubPageSelectButton>) {
        Self::on_nav_button_click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public List<AbstractWidget> getButtons()
    pub fn get_buttons_impl(__this: &compat::Shared<Self>) -> Option<Vec<AbstractWidget>> {
        let mut clickable_widgets: Vec<AbstractWidget> = compat::read(__this, |__t| __t.collection_select_buttons.clone()).into_iter().collect::<Vec<_>>();
        for page in __this.lock().borrow().collections.as_ref().unwrap().iter() {
            if true /* java: page != null: never null here */ && ({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).is_some() {
                clickable_widgets.extend(({ let __f = compat::dispatch(page, |__t| __t.get_buttons()); __f(page) }).unwrap().iter().cloned());
            }
        }
        return Some(clickable_widgets);
    }

    /// CollectionsPage.getButtons as the dispatch of ProfileViewerPage.getButtons
    pub fn get_buttons_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        Self::get_buttons_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsVisible()
    pub fn mark_widgets_as_visible_impl(__this: &compat::Shared<Self>) {
        for button in compat::read(__this, |__t| __t.collection_select_buttons.clone()) {
            { let __v = true; compat::write(&button, |__t| __t.visible = __v); }
            { let __v = true; compat::write(&button, |__t| __t.active = __v); }
        }
    }

    /// CollectionsPage.markWidgetsAsVisible as the dispatch of ProfileViewerPage.markWidgetsAsVisible
    pub fn mark_widgets_as_visible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_visible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsInvisible()
    pub fn mark_widgets_as_invisible_impl(__this: &compat::Shared<Self>) {
        for button in compat::read(__this, |__t| __t.collection_select_buttons.clone()) {
            { let __v = false; compat::write(&button, |__t| __t.visible = __v); }
            { let __v = false; compat::write(&button, |__t| __t.active = __v); }
        }
    }

    /// CollectionsPage.markWidgetsAsInvisible as the dispatch of ProfileViewerPage.markWidgetsAsInvisible
    pub fn mark_widgets_as_invisible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_invisible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
