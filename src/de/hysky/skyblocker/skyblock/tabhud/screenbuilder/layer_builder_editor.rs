//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.LayerBuilderEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_config::WidgetConfig;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::net::minecraft::util::problem_reporter::ScopedCollector;

/// java: public class LayerBuilderEditor
#[derive(Clone)]
pub struct LayerBuilderEditor {
    /// java: private final LayerBuilder layer;
    pub layer: compat::Shared<dyn LayerBuilder>,
}

impl LayerBuilderEditor {
    /// java: LayerBuilderEditor(LayerBuilder layer)
    pub fn new(mut layer: compat::Shared<dyn LayerBuilder>) -> Self {
        let __init_layer = layer.clone();
        let mut this = Self {
            layer: __init_layer,
        };
        this
    }

    /// java: public PositionedWidget add(HudWidget hudWidget, PositionRule rule)
    pub fn add(&self, hud_widget: &compat::Shared<dyn HudWidget>, rule: &PositionRule) -> compat::Shared<PositionedWidget> {
        // java: layer.config.widgets().put(hudWidget.getInternalID(), new WidgetConfig(new JsonObject(), rule));
        let __h0 = ({ let __f = compat::dispatch(hud_widget, |__t| __t.get_internal_id()); __f(hud_widget) });
        let __h1 = WidgetConfig::new(serde_json::Map::new(), rule.clone());
        let __widgets = compat::read(&self.layer, |__t| __t.layer_builder_base().config.widgets());
        compat::write(&__widgets, |__m| __m.insert(__h0, __h1));
        let mut positioned_widget: compat::Shared<PositionedWidget> = PositionedWidget::new(hud_widget.clone(), rule.clone());
        let __h0 = positioned_widget.clone();
        compat::write(&self.layer, |__t| __t.layer_builder_base_mut().widgets.push(__h0));
        ({ let __f = compat::dispatch(&self.layer, |__t| __t.update_list()); __f(&self.layer) });
        ({ let __f = compat::dispatch(hud_widget, |__t| __t.update_config_preview()); __f(hud_widget) });
        return positioned_widget;
    }

    /// java: public PositionedWidget add(HudWidget hudWidget)
    pub fn add_1(&self, hud_widget: compat::Shared<dyn HudWidget>) -> compat::Shared<PositionedWidget> {
        return self.add(&hud_widget, position_rule::DEFAULT.get());
    }

    /// java: public void remove(HudWidget widget)
    pub fn remove(&self, widget: &compat::Shared<dyn HudWidget>) {
        // java: layer.config.widgets().remove(widget.getInternalID());
        let __h0 = ({ let __f = compat::dispatch(widget, |__t| __t.get_internal_id()); __f(widget) });
        let __widgets = compat::read(&self.layer, |__t| __t.layer_builder_base().config.widgets());
        compat::write(&__widgets, |__m| __m.remove(&__h0));
        // java: layer.widgets.removeIf(w -> w.widget.equals(widget));
        let __kept: Vec<compat::Shared<PositionedWidget>> = compat::read(&self.layer, |__t| __t.layer_builder_base().widgets.clone())
            .into_iter()
            .filter(|w: &compat::Shared<PositionedWidget>| { let __r = compat::read(w, |__t| __t.widget.clone()); !({ let __f = compat::dispatch(&__r, |__t| __t.equals()); __f(&__r, widget) }) })
            .collect();
        compat::write(&self.layer, |__t| __t.layer_builder_base_mut().widgets = __kept);
        ({ let __f = compat::dispatch(&self.layer, |__t| __t.update_list()); __f(&self.layer) });
    }

    /// java: public void remove(PositionedWidget widget)
    pub fn remove_positioned_widget(&self, widget: &compat::Shared<PositionedWidget>) {
        self.remove(&compat::read(widget, |__t| __t.widget.clone()));
    }

    /// `widget.widget.save(new JsonValueOutput(reporter, conf))` in a `ScopedCollector` (WidgetManager.LOGGER): the object
    fn save_widget(widget: &compat::Shared<PositionedWidget>) -> serde_json::Map<String, serde_json::Value> {
        let reporter: ScopedCollector = ScopedCollector::new("de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager");
        let mut output = JsonValueOutput::new(reporter.reporter(), serde_json::Map::new());
        ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, &mut output) });
        output.output.unwrap_or_default()
    }

    /// java: public void serializeConfig()
    pub fn serialize_config(&self) {
        let __widgets = compat::read(&self.layer, |__t| __t.layer_builder_base().config.widgets());
        let __rendered = ({ let __f = compat::dispatch(&self.layer, |__t| __t.get_rendered()); __f(&self.layer) });
        // java: layer.config.widgets().replaceAll((id, widgetConfig) -> {..});
        let __ids: Vec<String> = compat::read(&__widgets, |__m| __m.keys().cloned().collect());
        for id in __ids {
            let widget: Option<compat::Shared<PositionedWidget>> = __rendered.iter().find(|w| { let __r = compat::read(w, |__t| __t.widget.clone()); ({ let __f = compat::dispatch(&__r, |__t| __t.get_internal_id()); __f(&__r) }) == id }).cloned();
            let Some(widget) = widget else {
                // java: if (widgetConfig.config().isEmpty()) return widgetConfig; return widgetConfig;
                continue;
            };
            let conf = LayerBuilderEditor::save_widget(&widget);
            let (from_tab, rule) = compat::read(&widget, |__t| (__t.from_tab, __t.rule.clone()));
            // java: return new WidgetConfig(Optional.of(conf), widget.fromTab ? Optional.empty() : Optional.of(widget.rule));
            let __config = WidgetConfig::new_canonical(Some(conf), if from_tab { None } else { Some(rule) });
            compat::write(&__widgets, |__m| __m.insert(id, __config));
        }
        for widget in __rendered.iter() {
            let __id = ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) });
            if !(compat::read(widget, |__t| __t.from_tab)) || compat::read(&__widgets, |__m| __m.contains_key(&__id)) {
                continue;
            }
            let conf = LayerBuilderEditor::save_widget(widget);
            // java: layer.config.widgets().put(widget.widget.getInternalID(), new WidgetConfig(Optional.of(conf), Optional.empty()));
            let __config = WidgetConfig::new_canonical(Some(conf), None);
            compat::write(&__widgets, |__m| __m.insert(__id, __config));
        }
    }

}
