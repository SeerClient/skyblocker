//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.TabLayerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::net::minecraft::util::profiling::profiler;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::util::profiling::profiler_filler::ProfilerFiller as _;

/// java: public class TabLayerBuilder extends LayerBuilder
#[derive(Clone)]
pub struct TabLayerBuilder {
    /// java: extends LayerBuilder
    pub base: LayerBuilderBase,
    /// java: private final Set<PositionedWidget> merged = new ObjectLinkedOpenHashSet<>();
    pub merged: std::collections::HashSet<compat::Shared<PositionedWidget>>,
    /// java: private final List<PositionedWidget> tabWidgets = new LinkedList<>();
    pub tab_widgets: Vec<compat::Shared<PositionedWidget>>,
}

impl std::ops::Deref for TabLayerBuilder {
    type Target = LayerBuilderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TabLayerBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TabLayerBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = LayerBuilderBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                merged: std::collections::HashSet::new(),
                tab_widgets: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn LayerBuilder>), __h)
        });
        __this
    }

    /// java: public void clearTab()
    pub fn clear_tab(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().tab_widgets.clear();
        TabLayerBuilder::merge(__this);
    }

    /// java: protected void merge()
    pub fn merge(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().merged.clear();
        let __h0 = compat::read(__this, |__t| __t.widgets.clone()).into_iter();
        __this.lock().borrow_mut().merged.extend(__h0);
        let __h0 = compat::read(__this, |__t| __t.tab_widgets.clone()).into_iter();
        __this.lock().borrow_mut().merged.extend(__h0);
        __this.lock().borrow_mut().rendered_widgets.clear();
        for __a0 in compat::read(__this, |__t| __t.merged.clone()).into_iter().map(|w| compat::read(&w, |__t| __t.widget.clone())) {
            let __h0 = __a0.clone();
            __this.lock().borrow_mut().rendered_widgets.insert(__h0);
        }
    }

    /// java: public void updateTab(Collection<String> ignoredWidgets)
    pub fn update_tab(__this: &compat::Shared<Self>, ignored_widgets: &Vec<String>) {
        profiler::get().push("skyblocker:updateTabWidgetsList");
        let mut current_widgets: std::collections::HashSet<String> = PlayerListManager::get_current_widgets();
        let mut new_tab_widgets: Vec<compat::Shared<PositionedWidget>> = Vec::new();
        for s in current_widgets.iter() {
            let Some(mut widget) = PlayerListManager::get_tab_widget(s) else {
                continue;
            };
            if ignored_widgets.iter().any(|e| *e == ({ let __f = compat::dispatch(&widget, |__t| __t.get_internal_id()); __f(&widget) })) {
                continue;
            }
            let mut e: compat::Shared<PositionedWidget> = PositionedWidget::new(widget.clone(), position_rule::DEFAULT.get_mut().clone());
            { let __v = true; compat::write(&e, |__t| __t.from_tab = __v); }
            new_tab_widgets.push(e.clone());
        }
        if new_tab_widgets == &compat::read(__this, |__t| __t.tab_widgets.clone()) {
            return;
        }
        __this.lock().borrow_mut().tab_widgets.clear();
        let __h0 = new_tab_widgets.iter().cloned();
        __this.lock().borrow_mut().tab_widgets.extend(__h0);
        TabLayerBuilder::merge(__this);
        profiler::get().pop();
    }

}

impl LayerBuilder for TabLayerBuilder {
    fn layer_builder_base(&self) -> &LayerBuilderBase {
        &self.base
    }
    fn layer_builder_base_mut(&mut self) -> &mut LayerBuilderBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void updateList() (dispatch: the body for this class)
    fn update_list(&self) -> fn(&compat::Shared<dyn LayerBuilder>) {
        TabLayerBuilder::update_list_dyn
    }

    /// java: @Override public Set<PositionedWidget> getRendered() (dispatch: the body for this class)
    fn get_rendered(&self) -> fn(&compat::Shared<dyn LayerBuilder>) -> Vec<compat::Shared<PositionedWidget>> {
        TabLayerBuilder::get_rendered_dyn
    }

    /// java: @Override public void updatePositions(int screenWidth, int screenHeight) (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn LayerBuilder>, i32, i32) {
        TabLayerBuilder::update_positions_dyn
    }

}

impl TabLayerBuilder {
    /// java: @Override protected void updateList()
    pub fn update_list_impl(__this: &compat::Shared<Self>) {
        TabLayerBuilder::merge(__this);
        <dyn LayerBuilder>::update_list_body(&compat::view(__this, |a| a as compat::Shared<dyn LayerBuilder>));
    }

    /// TabLayerBuilder.updateList as the dispatch of LayerBuilder.updateList
    pub fn update_list_dyn(__this: &compat::Shared<dyn LayerBuilder>) {
        Self::update_list_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Set<PositionedWidget> getRendered()
    pub fn get_rendered_impl(__this: &compat::Shared<Self>) -> std::collections::HashSet<compat::Shared<PositionedWidget>> {
        return compat::read(__this, |__t| __t.merged.clone());
    }

    /// TabLayerBuilder.getRendered as the dispatch of LayerBuilder.getRendered
    pub fn get_rendered_dyn(__this: &compat::Shared<dyn LayerBuilder>) -> Vec<compat::Shared<PositionedWidget>> {
        Self::get_rendered_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updatePositions(int screenWidth, int screenHeight)
    pub fn update_positions_impl(__this: &compat::Shared<Self>, mut screen_width: i32, mut screen_height: i32) {
        <dyn LayerBuilder>::update_positions_body(&compat::view(__this, |a| a as compat::Shared<dyn LayerBuilder>), screen_width, screen_height);
        if !(__this.lock().borrow().tab_widgets.is_empty()) {
            let mut positioner: compat::Shared<dyn WidgetPositioner> = SkyblockerConfigManager::get().ui_and_visuals.tab_hud.default_positioning.get_new_positioner(0.9f32, screen_height);
            let mut tab_widgets: Vec<compat::Shared<dyn HudWidget>> = ({ let __f = compat::dispatch(__this, |__t| __t.get_rendered()); __f(&compat::view(__this, |a| a as compat::Shared<dyn LayerBuilder>)) }).into_iter().filter(|p| compat::read(p, |__t| __t.from_tab)).map(|p| compat::read(&p, |__t| __t.widget.clone())).collect::<Vec<_>>();
            for __a0 in tab_widgets.iter() {
                ({ let __f = compat::dispatch(&positioner, |__t| __t.position_widget()); __f(&positioner, __a0) });
            }
            let mut dimensions: Vector2i = ({ let __f = compat::dispatch(&positioner, |__t| __t.finalize_positioning()); __f(&positioner) });
            let mut x: i32 = (screen_width.wrapping_sub(dimensions.x)).wrapping_div(2);
            let mut y: i32 = (screen_height.wrapping_sub(dimensions.y)).wrapping_div(2);
            for widget in tab_widgets.iter() {
                // TODO(fabric2seer): `setPosition` is not a method of HudWidget
                //   java: widget.setPosition(..);
            }
        }
    }

    /// TabLayerBuilder.updatePositions as the dispatch of LayerBuilder.updatePositions
    pub fn update_positions_dyn(__this: &compat::Shared<dyn LayerBuilder>, __p0: i32, __p1: i32) {
        Self::update_positions_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
