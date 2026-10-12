//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.ScreenBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_config::ScreenConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::tab_layer_builder::TabLayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;

/// java: public class ScreenBuilder
#[derive(Clone)]
pub struct ScreenBuilderBase {
    /// java: private final LayerBuilder hud;
    pub hud: Option<compat::Shared<dyn LayerBuilder>>,
    /// java: private final TabLayerBuilder tab;
    pub tab: Option<compat::Shared<TabLayerBuilder>>,
    /// java: private final LayerBuilder secondaryTab;
    pub secondary_tab: Option<compat::Shared<dyn LayerBuilder>>,
    /// java: private ScreenConfig config = ScreenConfig.DUMMY;
    pub config: ScreenConfig,
}

impl ScreenBuilderBase {
    /// java: protected ScreenBuilder(LayerBuilder hud, TabLayerBuilder tab, LayerBuilder secondaryTab) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut hud: compat::Shared<dyn LayerBuilder>, mut tab: compat::Shared<TabLayerBuilder>, mut secondary_tab: compat::Shared<dyn LayerBuilder>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ScreenBuilder>, __R)) -> (compat::Shared<dyn ScreenBuilder>, __R) {
        let __init_hud = Some(hud.clone());
        let __init_tab = Some(tab.clone());
        let __init_secondary_tab = Some(secondary_tab.clone());
        let this = Self {
            hud: __init_hud,
            tab: __init_tab,
            secondary_tab: __init_secondary_tab,
            config: screen_config::DUMMY.get_mut().clone(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected ScreenBuilder(LayerBuilder hud, TabLayerBuilder tab, LayerBuilder secondaryTab)
    pub fn new(mut hud: compat::Shared<dyn LayerBuilder>, mut tab: compat::Shared<TabLayerBuilder>, mut secondary_tab: compat::Shared<dyn LayerBuilder>) -> compat::Shared<Self> {
        Self::new_in(hud, tab, secondary_tab, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn ScreenBuilder>), __h)
        }).1
    }

    /// java: public ScreenBuilder() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_0_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn ScreenBuilder>, __R)) -> (compat::Shared<dyn ScreenBuilder>, __R) {
        let (__this, __r) = Self::new_in((LayerBuilderBase::new() as compat::Shared<dyn LayerBuilder>), TabLayerBuilder::new(), (LayerBuilderBase::new() as compat::Shared<dyn LayerBuilder>), __wrap);
        (__this, __r)
    }

    /// java: public ScreenBuilder()
    pub fn new_0() -> compat::Shared<Self> {
        Self::new_0_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn ScreenBuilder>), __h)
        }).1
    }

}

/// java: the virtual methods of ScreenBuilder (implemented by ScreenBuilderBase and every subclass)
pub trait ScreenBuilder: compat::DynAny {
    fn screen_builder_base(&self) -> &ScreenBuilderBase;
    fn screen_builder_base_mut(&mut self) -> &mut ScreenBuilderBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public LayerBuilder get(WidgetManager.ScreenLayer layer) (dispatch: the body for this class)
    fn get(&self) -> fn(&compat::Shared<dyn ScreenBuilder>, ScreenLayer) -> compat::Shared<dyn LayerBuilder> {
        <dyn ScreenBuilder>::get_body
    }

    /// java: public LayerBuilder hud() (dispatch: the body for this class)
    fn hud(&self) -> fn(&compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        <dyn ScreenBuilder>::hud_body
    }

    /// java: public LayerBuilder tab() (dispatch: the body for this class)
    fn tab(&self) -> fn(&compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        <dyn ScreenBuilder>::tab_body
    }

    /// java: public LayerBuilder secondaryTab() (dispatch: the body for this class)
    fn secondary_tab(&self) -> fn(&compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        <dyn ScreenBuilder>::secondary_tab_body
    }

    /// java: public void updateFancyTab() (dispatch: the body for this class)
    fn update_fancy_tab(&self) -> fn(&compat::Shared<dyn ScreenBuilder>) {
        <dyn ScreenBuilder>::update_fancy_tab_body
    }

    /// java: public void clearFancyTab() (dispatch: the body for this class)
    fn clear_fancy_tab(&self) -> fn(&compat::Shared<dyn ScreenBuilder>) {
        <dyn ScreenBuilder>::clear_fancy_tab_body
    }

    /// java: public void setConfig(ScreenConfig config) (dispatch: the body for this class)
    fn set_config(&self) -> fn(&compat::Shared<dyn ScreenBuilder>, ScreenConfig) {
        <dyn ScreenBuilder>::set_config_body
    }

    /// java: public boolean contains(HudWidget widget) (dispatch: the body for this class)
    fn contains(&self) -> fn(&compat::Shared<dyn ScreenBuilder>, &compat::Shared<dyn HudWidget>) -> bool {
        <dyn ScreenBuilder>::contains_body
    }

}

impl dyn ScreenBuilder {
    /// java: public LayerBuilder get(WidgetManager.ScreenLayer layer)
    pub fn get_body(__this: &compat::Shared<dyn ScreenBuilder>, mut layer: ScreenLayer) -> compat::Shared<dyn LayerBuilder> {
        return match layer {
            ScreenLayer::Hud => compat::read(__this, |__t| __t.screen_builder_base().hud.clone()).unwrap(),
            ScreenLayer::MainTab => (compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap() as compat::Shared<dyn LayerBuilder>),
            ScreenLayer::SecondaryTab => compat::read(__this, |__t| __t.screen_builder_base().secondary_tab.clone()).unwrap(),
        };
    }

    /// java: public LayerBuilder hud()
    pub fn hud_body(__this: &compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        return compat::read(__this, |__t| __t.screen_builder_base().hud.clone()).unwrap();
    }

    /// java: public LayerBuilder tab()
    pub fn tab_body(__this: &compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        return (compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap() as compat::Shared<dyn LayerBuilder>);
    }

    /// java: public LayerBuilder secondaryTab()
    pub fn secondary_tab_body(__this: &compat::Shared<dyn ScreenBuilder>) -> compat::Shared<dyn LayerBuilder> {
        return compat::read(__this, |__t| __t.screen_builder_base().secondary_tab.clone()).unwrap();
    }

    /// java: public void updateFancyTab()
    pub fn update_fancy_tab_body(__this: &compat::Shared<dyn ScreenBuilder>) {
        // java: tab.updateTab(config.hiddenTabWidgets());
        let __hidden = compat::read(__this, |__t| __t.screen_builder_base().config.hidden_tab_widgets());
        let __hidden = compat::read(&__hidden, |__s| __s.clone());
        TabLayerBuilder::update_tab(&compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap(), &__hidden);
    }

    /// java: public void clearFancyTab()
    pub fn clear_fancy_tab_body(__this: &compat::Shared<dyn ScreenBuilder>) {
        TabLayerBuilder::clear_tab(&compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap());
    }

    /// java: public void setConfig(ScreenConfig config)
    pub fn set_config_body(__this: &compat::Shared<dyn ScreenBuilder>, mut config: ScreenConfig) {
        { let __v = config.clone(); compat::write(__this, |__t| __t.screen_builder_base_mut().config = __v); }
        ({ let __r = &compat::read(__this, |__t| __t.screen_builder_base().hud.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(__r, config.hud()) });
        ({ let __r = &compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(&compat::view(__r, |a| a as compat::Shared<dyn LayerBuilder>), config.tab()) });
        ({ let __r = &compat::read(__this, |__t| __t.screen_builder_base().secondary_tab.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_config()); __f(__r, config.secondary_tab()) });
    }

    /// java: public boolean contains(HudWidget widget)
    pub fn contains_body(__this: &compat::Shared<dyn ScreenBuilder>, widget: &compat::Shared<dyn HudWidget>) -> bool {
        return ({ let __r = &compat::read(__this, |__t| __t.screen_builder_base().hud.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) } || { let __r = &compat::read(__this, |__t| __t.screen_builder_base().tab.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(&compat::view(__r, |a| a as compat::Shared<dyn LayerBuilder>), widget) } || { let __r = &compat::read(__this, |__t| __t.screen_builder_base().secondary_tab.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.contains()); __f(__r, widget) });
    }

}

impl ScreenBuilder for ScreenBuilderBase {
    fn screen_builder_base(&self) -> &ScreenBuilderBase {
        self
    }
    fn screen_builder_base_mut(&mut self) -> &mut ScreenBuilderBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
