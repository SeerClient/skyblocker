//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix12MoveWidgetOptions by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::{self, TypeRewriteRule};
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix12MoveWidgetOptions extends ConfigDataFix
pub struct ConfigFix12MoveWidgetOptions {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix12MoveWidgetOptions>,
    /// java: private final Map<String, SubFixer<?, ?>> widgetToSubFixer = Map.of(..)
    widget_to_sub_fixer: std::collections::HashMap<String, SubFixer>,
}

impl ConfigFix12MoveWidgetOptions {
    /// java: public ConfigFix12MoveWidgetOptions(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix12MoveWidgetOptions>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
            widget_to_sub_fixer: ConfigFix12MoveWidgetOptions::widget_to_sub_fixer(self_ref),
        })
    }

    /// java: the initialiser of `widgetToSubFixer` (its lambdas `this::fixX` - `this` is the object being built)
    fn widget_to_sub_fixer(this: &std::sync::Weak<ConfigFix12MoveWidgetOptions>) -> std::collections::HashMap<String, SubFixer> {
        fn method(
            this: &std::sync::Weak<ConfigFix12MoveWidgetOptions>,
            f: fn(&ConfigFix12MoveWidgetOptions, &Dynamic, &Dynamic) -> Dynamic,
        ) -> Arc<dyn Fn(Dynamic, Dynamic) -> Dynamic + Send + Sync> {
            let this = this.clone();
            Arc::new(move |widget_data: Dynamic, previous: Dynamic| match this.upgrade() {
                Some(this) => f(&this, &widget_data, &previous),
                None => widget_data,
            })
        }
        std::collections::HashMap::from([
            (
                "hud_end".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("otherLocations").get("end").result()), method(this, ConfigFix12MoveWidgetOptions::fix_end)),
            ),
            (
                "hud_farming".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("farming").get("farmingHud").result()), method(this, ConfigFix12MoveWidgetOptions::fix_farming)),
            ),
            (
                "hud_fishing".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("helpers").get("fishing").result()), method(this, ConfigFix12MoveWidgetOptions::fix_fishing)),
            ),
            (
                "item_pickup".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("uiAndVisuals").get("itemPickup").result()), method(this, ConfigFix12MoveWidgetOptions::fix_item_pickup)),
            ),
            (
                "players".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("uiAndVisuals").get("tabHud").result()), method(this, ConfigFix12MoveWidgetOptions::fix_player_list)),
            ),
            (
                "active_effects".to_string(),
                SubFixer::new(Arc::new(|dynamic: Dynamic| dynamic.get("uiAndVisuals").get("tabHud").result()), method(this, ConfigFix12MoveWidgetOptions::fix_effects)),
            ),
        ])
    }

    /// java: private <T> Dynamic<T> collect(Dynamic<T> dynamic)
    pub fn collect(&self, dynamic: &Dynamic) -> Dynamic {
        for fixer in self.widget_to_sub_fixer.values() {
            fixer.fetch_data(dynamic);
        }
        return dynamic.clone();
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        let updated: Dynamic = self.fix_version(dynamic).update("configs", |configs| {
            configs.update_map_values(|location| {
                let second = location.1.update_map_values(|layer| {
                    let second = layer.1.update("widgets", |widgets| widgets.update_map_values(|widget| self.fix_widget(widget)));
                    (layer.0, second)
                });
                (location.0, second)
            })
        });
        for fixer in self.widget_to_sub_fixer.values() {
            *fixer.data.lock().unwrap() = None;
        }
        return updated;
    }

    /// java: private static <T> Dynamic<T> lowercase(Dynamic<T> dynamic)
    pub fn lowercase(dynamic: &Dynamic) -> Dynamic {
        return dynamic
            .as_string()
            .result()
            .map(|s| s.to_lowercase()) // java: toLowerCase(Locale.ENGLISH)
            .map(|s| dynamic.create_string(&s))
            .unwrap_or_else(|| dynamic.clone());
    }

    /// java: private Pair<Dynamic<?>, Dynamic<?>> fixWidget(Pair<Dynamic<?>, Dynamic<?>> widget)
    pub fn fix_widget(&self, widget: (Dynamic, Dynamic)) -> (Dynamic, Dynamic) {
        let widget_id: String = widget.0.as_string_or("");
        let Some(fixer) = self.widget_to_sub_fixer.get(&widget_id) else {
            return widget;
        };
        let second = widget.1.update("config", |config| fixer.try_fix(&config));
        return (widget.0, second);
    }

    /// java: private <W, D> Dynamic<W> fixEnd(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_end(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data
            .set_field_if_present("zealot_kills", previous.get("zealotKillsEnabled").result())
            .set_field_if_present("protector_location", previous.get("protectorLocationEnabled").result());
    }

    /// java: private <W, D> Dynamic<W> fixFarming(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_farming(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data
            .set_field_if_present("counter", previous.get("counter").result())
            .set_field_if_present("coins", previous.get("coins").result())
            .set_field_if_present("price_type", previous.get("type").map(|d| ConfigFix12MoveWidgetOptions::lowercase(&d)).result())
            .set_field_if_present("experience", previous.get("experience").result())
            .set_field_if_present("include_seeds_price", previous.get("includeSeedsPrice").result());
    }

    /// java: private <W, D> Dynamic<W> fixFishing(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_fishing(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data
            .set_field_if_present("creature_counter", previous.get("enableSeaCreatureCounter").result())
            .set_field_if_present("fishing_timer", previous.get("enableFishingTimer").result())
            .set_field_if_present("only_barn", previous.get("onlyShowHudInBarn").result());
    }

    /// java: private <W, D> Dynamic<W> fixItemPickup(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_item_pickup(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data
            .set_field_if_present("sack_notifications", previous.get("sackNotifications").result())
            .set_field_if_present("split_sack", previous.get("splitNotifications").result())
            .set_field_if_present("show_item_name", previous.get("showItemName").result())
            .set_field_if_present("lifetime", previous.get("lifetime").result());
    }

    /// java: private <W, D> Dynamic<W> fixEffects(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_effects(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data.set_field_if_present("effects_from_footer", previous.get("effectsFromFooter").result());
    }

    /// java: private <W, D> Dynamic<W> fixPlayerList(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_player_list(&self, widget_data: &Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data.set_field_if_present("name_sorting", previous.get("nameSorting").map(|d| ConfigFix12MoveWidgetOptions::lowercase(&d)).result());
    }
}

impl DataFix for ConfigFix12MoveWidgetOptions {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        let this2 = self.self_ref.clone();
        return type_rewrite_rule::seq2(
            data_fix::fix_type_everywhere_typed_same(
                "ConfigFix12MoveWidgetOptions", // java: getClass().getSimpleName()
                self.get_input_schema()
                    .get_type(config_data_fixer::CONFIG_TYPE.get())
                    .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
                Arc::new(move |typed: Typed| {
                    let Some(this) = this.upgrade() else { return Err("ConfigFix12MoveWidgetOptions: the fix was dropped".to_string()) };
                    typed.update(&dsl::remainder_finder(), &|dynamic| this.collect(&dynamic))
                }),
            ),
            data_fix::fix_type_everywhere_typed_same(
                "ConfigFix12MoveWidgetOptions", // java: getClass().getSimpleName()
                self.get_input_schema()
                    .get_type(config_data_fixer::HUD_WIDGETS_TYPE.get())
                    .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
                Arc::new(move |typed: Typed| {
                    let Some(this) = this2.upgrade() else { return Err("ConfigFix12MoveWidgetOptions: the fix was dropped".to_string()) };
                    typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
                }),
            ),
        );
    }
}

impl ConfigDataFix for ConfigFix12MoveWidgetOptions {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

/// java: private static class SubFixer<W, D>
pub struct SubFixer {
    /// java: private final Function<Dynamic<?>, Optional<Dynamic<D>>> previousDataSupplier;
    previous_data_supplier: Arc<dyn Fn(Dynamic) -> Option<Dynamic> + Send + Sync>,
    /// java: private final BiFunction<Dynamic<W>, Dynamic<D>, Dynamic<W>> fixer;
    fixer: Arc<dyn Fn(Dynamic, Dynamic) -> Dynamic + Send + Sync>,
    /// java: private @Nullable Dynamic<D> data; (written while the rule runs, through `&self`)
    data: std::sync::Mutex<Option<Dynamic>>,
}

impl SubFixer {
    /// java: private SubFixer(Function<Dynamic<?>, Optional<Dynamic<D>>> previousDataSupplier, BiFunction<Dynamic<W>, Dynamic<D>, Dynamic<W>> fixer)
    fn new(
        previous_data_supplier: Arc<dyn Fn(Dynamic) -> Option<Dynamic> + Send + Sync>,
        fixer: Arc<dyn Fn(Dynamic, Dynamic) -> Dynamic + Send + Sync>,
    ) -> Self {
        Self { previous_data_supplier, fixer, data: std::sync::Mutex::new(None) }
    }

    /// java: private Dynamic<?> tryFix(Dynamic<?> widget)
    fn try_fix(&self, widget: &Dynamic) -> Dynamic {
        let data = self.data.lock().unwrap().clone();
        let Some(data) = data else {
            return widget.clone();
        };
        return (self.fixer)(widget.clone(), data);
    }

    /// java: private void fetchData(Dynamic<?> dynamic)
    fn fetch_data(&self, dynamic: &Dynamic) {
        let data = (self.previous_data_supplier)(dynamic.clone());
        *self.data.lock().unwrap() = data;
    }
}
