//! Ported from de.hysky.skyblocker.skyblock.profileviewer.slayers.SlayersPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::slayers::slayer_widget::SlayerWidget;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: private static final String[] SLAYERS = {"Zombie", "Spider", "Wolf", "Enderman", "Vampire", "Blaze"};
pub static SLAYERS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Zombie".to_string(), "Spider".to_string(), "Wolf".to_string(), "Enderman".to_string(), "Vampire".to_string(), "Blaze".to_string()]);
/// java: private static final int ROW_GAP = 28;
pub const ROW_GAP: i32 = 28;

/// java: public class SlayersPage implements ProfileViewerPage
#[derive(Clone)]
pub struct SlayersPage {
    /// java: private final List<SlayerWidget> slayerWidgets = new ArrayList<>();
    pub slayer_widgets: Vec<SlayerWidget>,
}

impl SlayersPage {
    /// java: public SlayersPage(JsonObject pProfile)
    pub fn new(mut p_profile: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            slayer_widgets: Vec::new(),
        });
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for slayer in SLAYERS.iter() {
                let __h0 = SlayerWidget::new(slayer.to_string(), SlayersPage::get_slayer_xp(&__this, &slayer.to_lowercase(), &p_profile), p_profile.clone());
                __this.lock().borrow_mut().slayer_widgets.push(__h0);
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Error creating slayer widgets"));
        }
        __this
    }

    /// java: private long getSlayerXP(String slayer, JsonObject pProfile)
    pub fn get_slayer_xp(__this: &compat::Shared<Self>, slayer: &str, p_profile: &serde_json::Map<String, serde_json::Value>) -> i64 {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return compat::json::get_as_long(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&p_profile, "slayer")), "slayer_bosses")), slayer)).get("xp").cloned().unwrap());
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return ((0) as i64) as i64;
            }
        }
    }

}

impl ProfileViewerPage for SlayersPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        SlayersPage::extract_render_state_dyn
    }

}

impl SlayersPage {
    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        for i in 0..(__this.lock().borrow().slayer_widgets.len() as i32) {
            (__this.lock().borrow().slayer_widgets[(i) as usize]).extract_render_state(graphics, mouse_x, mouse_y, root_x, root_y.wrapping_add(i.wrapping_mul(ROW_GAP)));
        }
    }

    /// SlayersPage.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

}
