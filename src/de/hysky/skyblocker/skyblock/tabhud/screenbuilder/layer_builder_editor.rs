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
    pub fn add(&mut self, hud_widget: &compat::Shared<dyn HudWidget>, rule: &PositionRule) -> compat::Shared<PositionedWidget> {
        let __h0 = ({ let __f = compat::dispatch(hud_widget, |__t| __t.get_internal_id()); __f(hud_widget) });
        let __h1 = WidgetConfig::new(serde_json::Map::new(), rule.clone());
        self.layer.lock().borrow_mut().layer_builder_base_mut().config.widgets().insert(__h0, __h1);
        let mut positioned_widget: compat::Shared<PositionedWidget> = PositionedWidget::new(hud_widget.clone(), rule.clone());
        let __h0 = positioned_widget.clone();
        self.layer.lock().borrow_mut().layer_builder_base_mut().widgets.push(__h0);
        ({ let __f = compat::dispatch(&self.layer, |__t| __t.update_list()); __f(&self.layer) });
        ({ let __f = compat::dispatch(hud_widget, |__t| __t.update_config_preview()); __f(hud_widget) });
        return positioned_widget;
    }

    /// java: public PositionedWidget add(HudWidget hudWidget)
    pub fn add_1(&mut self, mut hud_widget: compat::Shared<dyn HudWidget>) -> compat::Shared<PositionedWidget> {
        return self.add(&hud_widget, position_rule::DEFAULT.get_mut());
    }

    /// java: public void remove(HudWidget widget)
    pub fn remove(&mut self, widget: &compat::Shared<dyn HudWidget>) {
        let __h0 = &({ let __f = compat::dispatch(widget, |__t| __t.get_internal_id()); __f(widget) });
        self.layer.lock().borrow_mut().layer_builder_base_mut().config.widgets().remove(__h0);
        let __h0 = |w| !(({ let __r = &compat::read(w, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.equals()); __f(__r, widget) }));
        self.layer.lock().borrow_mut().layer_builder_base_mut().widgets.retain(__h0);
        ({ let __f = compat::dispatch(&self.layer, |__t| __t.update_list()); __f(&self.layer) });
    }

    /// java: public void remove(PositionedWidget widget)
    pub fn remove_positioned_widget(&mut self, widget: &compat::Shared<PositionedWidget>) {
        self.remove(&compat::read(widget, |__t| __t.widget.clone()));
    }

    /// java: public void serializeConfig()
    pub fn serialize_config(&self) {
        // TODO(fabric2seer): Map.replaceAll(..)
        //   java: layer.config.widgets().replaceAll((id, widgetConfig) -> {
        //   java: 			PositionedWidget widget = layer.getRendered().stream().filter(w -> w.widget.getInternalID().equals(id)).findFirst().orElse(null);
        //   java: 			if (widget == null) {
        //   java: 				if (widgetConfig.config().isEmpty()) return widgetConfig; // normal, it was removed
        //   java: 				// maybe not normal? not sure how to handle it // TODO
        //   java: 				return widgetConfig;
        //   java: 			}
        //   java: 			JsonObject conf = new JsonObject();
        //   java: 			try (ProblemReporter.ScopedCollector reporter = new ProblemReporter.ScopedCollector(WidgetManager.LOGGER)) {
        //   java: 				widget.widget.save(new JsonValueOutput(reporter, conf));
        //   java: 			}
        //   java: 			return new WidgetConfig(Optional.of(conf), widget.fromTab ? Optional.empty() : Optional.of(widget.rule));
        //   java: 		});
        for widget in ({ let __f = compat::dispatch(&self.layer, |__t| __t.get_rendered()); __f(&self.layer) }) {
            if !(compat::read(&widget, |__t| __t.from_tab)) || self.layer.lock().borrow().layer_builder_base().config.widgets().contains_key(&({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) })) {
                continue;
            }
            let mut conf: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            {
                // TODO(fabric2seer): `logger` (a logger) used as a value
                //   java: ProblemReporter.ScopedCollector reporter = new ProblemReporter.ScopedCollector(WidgetManager.LOGGER);
                let mut reporter: ScopedCollector = todo!("fabric2seer: the initialiser of `reporter` (TODO above)");
                // TODO(fabric2seer): the mod keeps the game's `ProblemReporter$ScopedCollector` (Java holds a reference to the live object); the port's `ScopedCollector` is not a value to copy and the port lends the mod no handle to it
                // rust: ({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, &mut JsonValueOutput::new(reporter.clone(), conf.clone())) });
            }
            let __h0 = ({ let __r = &compat::read(&widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_internal_id()); __f(__r) });
            let __h1 = WidgetConfig::new(Some(conf.clone()), None);
            self.layer.lock().borrow_mut().layer_builder_base_mut().config.widgets().insert(__h0, __h1);
        }
    }

}
