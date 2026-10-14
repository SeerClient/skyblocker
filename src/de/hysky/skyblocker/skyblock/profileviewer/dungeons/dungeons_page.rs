//! Ported from de.hysky.skyblocker.skyblock.profileviewer.dungeons.DungeonsPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::dungeons::dungeon_class_widget::DungeonClassWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer::dungeons::dungeon_floor_runs_widget::DungeonFloorRunsWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer::dungeons::dungeon_header_widget::DungeonHeaderWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer::dungeons::dungeon_misc_stats_widgets::DungeonMiscStatsWidgets;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: private static final String[] CLASSES = {"Healer", "Mage", "Berserk", "Archer", "Tank"};
pub static CLASSES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Healer".to_string(), "Mage".to_string(), "Berserk".to_string(), "Archer".to_string(), "Tank".to_string()]);

/// java: public class DungeonsPage implements ProfileViewerPage
#[derive(Clone)]
pub struct DungeonsPage {
    /// java: private final DungeonHeaderWidget dungeonHeaderWidget;
    pub dungeon_header_widget: DungeonHeaderWidget,
    /// java: private final List<DungeonClassWidget> dungeonClassWidgetsList = new ArrayList<>();
    pub dungeon_class_widgets_list: Vec<DungeonClassWidget>,
    /// java: private final DungeonFloorRunsWidget dungeonFloorRunsWidget;
    pub dungeon_floor_runs_widget: DungeonFloorRunsWidget,
    /// java: private final DungeonMiscStatsWidgets dungeonMiscStatsWidgets;
    pub dungeon_misc_stats_widgets: DungeonMiscStatsWidgets,
}

impl DungeonsPage {
    /// java: public DungeonsPage(JsonObject pProfile)
    pub fn new(mut p_profile: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __init_dungeon_header_widget = DungeonHeaderWidget::new(p_profile.clone(), CLASSES.clone());
        let __init_dungeon_floor_runs_widget = DungeonFloorRunsWidget::new(p_profile.clone());
        let __init_dungeon_misc_stats_widgets = DungeonMiscStatsWidgets::new(p_profile.clone());
        let __this = compat::shared(Self {
            dungeon_header_widget: __init_dungeon_header_widget,
            dungeon_class_widgets_list: Vec::new(),
            dungeon_floor_runs_widget: __init_dungeon_floor_runs_widget,
            dungeon_misc_stats_widgets: __init_dungeon_misc_stats_widgets,
        });
        for element in CLASSES.iter() {
            let __h0 = DungeonClassWidget::new(element.to_string(), p_profile.clone());
            __this.lock().borrow_mut().dungeon_class_widgets_list.push(__h0);
        }
        __this
    }

}

impl ProfileViewerPage for DungeonsPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        DungeonsPage::extract_render_state_dyn
    }

}

impl DungeonsPage {
    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32, mut root_x: i32, mut root_y: i32) {
        __this.lock().borrow().dungeon_header_widget.extract_render_state(graphics, root_x, root_y);
        let __h0 = root_x.wrapping_add(113);
        let __h1 = root_y.wrapping_add(56);
        __this.lock().borrow().dungeon_floor_runs_widget.extract_render_state(graphics, mouse_x, mouse_y, __h0, __h1);
        let __h0 = root_x.wrapping_add(113);
        __this.lock().borrow().dungeon_misc_stats_widgets.extract_render_state(graphics, __h0, root_y);
        for i in { 0..(__this.lock().borrow().dungeon_class_widgets_list.len() as i32) } {
            (__this.lock().borrow_mut().dungeon_class_widgets_list[(i) as usize]).extract_render_state(graphics, mouse_x, mouse_y, root_x, (root_y.wrapping_add(28)).wrapping_add(i.wrapping_mul(28)));
        }
    }

    /// DungeonsPage.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

}
