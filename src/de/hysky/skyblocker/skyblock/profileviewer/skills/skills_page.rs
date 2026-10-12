//! Ported from de.hysky.skyblocker.skyblock.profileviewer.skills.SkillsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::skills::skill_widget::SkillWidget;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/icon_data_widget.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/icon_data_widget.png"));
/// java: private static final String[] SKILLS = {"Combat", "Mining", "Farming", "Foraging", "Fishing", "Enchanting", "Alchemy", "Taming", "Carpentry", "Catacombs", "Runecraft", "Social"};
pub static SKILLS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Combat".to_string(), "Mining".to_string(), "Farming".to_string(), "Foraging".to_string(), "Fishing".to_string(), "Enchanting".to_string(), "Alchemy".to_string(), "Taming".to_string(), "Carpentry".to_string(), "Catacombs".to_string(), "Runecraft".to_string(), "Social".to_string()]);
/// java: private static final int ROW_GAP = 28;
pub const ROW_GAP: i32 = 28;

/// java: public class SkillsPage implements ProfileViewerPage
#[derive(Clone)]
pub struct SkillsPage {
    /// java: private final JsonObject HYPIXEL_PROFILE;
    pub hypixel_profile: serde_json::Map<String, serde_json::Value>,
    /// java: private final JsonObject PLAYER_PROFILE;
    pub player_profile: serde_json::Map<String, serde_json::Value>,
    /// java: private final List<SkillWidget> skillWidgets = new ArrayList<>();
    pub skill_widgets: Vec<SkillWidget>,
    /// java: private JsonObject skills;
    pub skills: Option<serde_json::Map<String, serde_json::Value>>,
}

impl SkillsPage {
    /// java: public SkillsPage(JsonObject hProfile, JsonObject pProfile)
    pub fn new(mut h_profile: serde_json::Map<String, serde_json::Value>, mut p_profile: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __init_hypixel_profile = h_profile.clone();
        let __init_player_profile = p_profile.clone();
        let __this = compat::shared(Self {
            hypixel_profile: __init_hypixel_profile,
            player_profile: __init_player_profile,
            skill_widgets: Vec::new(),
            skills: None,
        });
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let __v = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&__this.lock().borrow().player_profile, "player_data")), "experience")); compat::write(&__this, |__t| __t.skills = __v); }
            for skill in SKILLS.iter() {
                let __h0 = SkillWidget::new(skill.to_string(), SkillsPage::get_skill_xp(&__this, &format!("SKILL_{}", skill.to_uppercase())), SkillsPage::get_skill_cap(&__this, skill));
                __this.lock().borrow_mut().skill_widgets.push(__h0);
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Error creating widgets."));
        }
        __this
    }

    /// java: private int getSkillCap(String skill)
    pub fn get_skill_cap(__this: &compat::Shared<Self>, skill: &str) -> i32 {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match skill {
                "Farming" => compat::json::get_as_int(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&__this.lock().borrow().player_profile, "jacobs_contest")), "perks")).get("farming_level_cap").cloned().unwrap()),
                "Taming" => ((compat::json::get_as_json_array_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&__this.lock().borrow().player_profile, "pets_data")), "pet_care")), "pet_types_sacrificed")).len() as i32),
                _ => -1,
            };
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return 0;
            }
        }
    }

    /// java: private long getSkillXP(String skill)
    pub fn get_skill_xp(__this: &compat::Shared<Self>, skill: &str) -> i64 {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match skill {
                "SKILL_CATACOMBS" => SkillsPage::get_catacombs_xp(__this),
                "SKILL_SOCIAL" => SkillsPage::get_coop_social_xp(__this),
                "SKILL_RUNECRAFT" => compat::json::get_as_long(&__this.lock().borrow().skills.as_ref().unwrap().get("SKILL_RUNECRAFTING").cloned().unwrap()),
                _ => compat::json::get_as_long(&__this.lock().borrow().skills.as_ref().unwrap().get(skill).cloned().unwrap()),
            };
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return ((0) as i64) as i64;
            }
        }
    }

    /// java: private long getCatacombsXP()
    pub fn get_catacombs_xp(__this: &compat::Shared<Self>) -> i64 {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dungeon_skills: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&__this.lock().borrow().player_profile, "dungeons")), "dungeon_types");
            return compat::json::get_as_long(&(compat::json::get_as_json_object_of(&dungeon_skills, "catacombs")).get("experience").cloned().unwrap());
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return ((0) as i64) as i64;
            }
        }
    }

    /// java: private long getCoopSocialXP()
    pub fn get_coop_social_xp(__this: &compat::Shared<Self>) -> i64 {
        let mut social_xp: i64 = ((0) as i64) as i64;
        let mut members: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&__this.lock().borrow().hypixel_profile, "members");
        for member_id in members.keys().cloned().collect::<Vec<String>>() {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                social_xp = social_xp.wrapping_add(compat::json::get_as_long(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&members, &member_id)), "player_data")), "experience")).get("SKILL_SOCIAL").cloned().unwrap()));
            }));
            if let Err(e) = __result {
                logger().warn(&format!("[Skyblocker Profile Viewer] Error calculating co-op social xp"));
            }
        }
        return social_xp;
    }

}

impl ProfileViewerPage for SkillsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        SkillsPage::extract_render_state_dyn
    }

}

impl SkillsPage {
    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        let mut column2: i32 = root_x.wrapping_add(113);
        for i in 0..(__this.lock().borrow().skill_widgets.len() as i32) {
            let mut x: i32 = (if i < 6 { root_x } else { column2 });
            let mut y: i32 = root_y.wrapping_add((i.wrapping_rem(6)).wrapping_mul(ROW_GAP));
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
            (__this.lock().borrow_mut().skill_widgets[(i) as usize]).extract_render_state(graphics, mouse_x, mouse_y, x, y.wrapping_add(3));
        }
    }

    /// SkillsPage.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

}
