//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix11NewHud by fabric2seer.

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

/// java: private static final Map<String, String[]> LOCATION_WIDGETS = Map.ofEntries( Map.entry("garden", new String[]{"hud_farming", "sweepDetails"}), Map.entry("hub", new String[]{"sweepDetails", "hud_slayer
pub static LOCATION_WIDGETS: LazyLock<std::collections::HashMap<String, Vec<String>>> = LazyLock::new(|| std::collections::HashMap::from([("garden".to_string(), vec!["hud_farming".to_string(), "sweepDetails".to_string()]), ("hub".to_string(), vec!["sweepDetails".to_string(), "hud_slayer".to_string(), "hud_fishing".to_string()]), ("foraging_1".to_string(), vec!["sweepDetails".to_string(), "hud_slayer".to_string(), "hud_fishing".to_string()]), ("foraging_2".to_string(), vec!["sweepDetails".to_string(), "Lasso HUD".to_string(), "hud_treeprogress".to_string(), "hud_fishing".to_string()]), ("foraging_3".to_string(), vec!["sweepDetails".to_string(), "Lasso HUD".to_string(), "hud_treeprogress".to_string(), "hud_fishing".to_string()]), ("crystal_hollows".to_string(), vec!["hud_crystals".to_string(), "powder_mining_tracker".to_string(), "hud_fishing".to_string()]), ("combat_3".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("combat_1".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("rift".to_string(), vec!["hud_slayer".to_string()]), ("crimson_isle".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("lotus_atoll".to_string(), vec!["hud_fishing".to_string()]), ("fishing_1".to_string(), vec!["hud_fishing".to_string()])]));

/// java: public class ConfigFix11NewHud extends ConfigDataFix
pub struct ConfigFix11NewHud {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix11NewHud>,
    /// java: private @Nullable Dynamic<?> mainConfig;
    main_config: std::sync::Mutex<Option<Dynamic>>,
}

impl ConfigFix11NewHud {
    /// java: public ConfigFix11NewHud(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix11NewHud>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
            main_config: std::sync::Mutex::new(None),
        })
    }

    /// java: private <T> Dynamic<T> collect(Dynamic<T> dynamic)
    pub fn collect(&self, dynamic: &Dynamic) -> Dynamic {
        // Fix enable pickobulus helper
        let dynamic = self.fix_version(dynamic).update("mining", |mining| {
            mining
                .update("pickobulusHelper", |pickobulus_helper| {
                    pickobulus_helper.set_field_if_present("enablePickobulusHelper", mining.get("enablePickobulusHelper").result())
                })
                .remove("enablePickobulusHelper")
        });

        *self.main_config.lock().unwrap() = Some(dynamic.clone());
        return dynamic;
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        let updated: Dynamic = self
            .fix_version(dynamic)
            .rename_and_fix_field("positions", "configs", |locations| self.fix_widgets(locations))
            .set(
                "copies",
                &dynamic.create_map(vec![
                    (dynamic.create_string("hud"), dynamic.empty_map()),
                    (dynamic.create_string("tab"), dynamic.empty_map()),
                    (dynamic.create_string("secondary_tab"), dynamic.empty_map()),
                ]),
            );
        *self.main_config.lock().unwrap() = None;
        return updated;
    }

    /// Fixes the map of skyblock locations to widgets.
    /// java: private <T> Dynamic<T> fixWidgets(Dynamic<T> locations)
    pub fn fix_widgets(&self, mut locations: Dynamic) -> Dynamic {
        // it was possible for widgets to be enabled but to not have a position, add them if they
        for (key, value) in LOCATION_WIDGETS.iter() {
            if locations.get(key).result().is_none() {
                locations = locations.set(key, &locations.empty_map());
            }
            locations = locations.update(key, |location| self.fix_missing_widgets_for_location(value, location));
        }
        return locations.update_map_values(|location| self.fix_widgets_for_location(location));
    }

    /// Fixes widgets that are enabled but don't have a position configured for this location.
    /// It would default to the top left in the old system.
    /// java: private <T> Dynamic<T> fixMissingWidgetsForLocation(String[] missingWidgets, Dynamic<T> location)
    pub fn fix_missing_widgets_for_location(&self, missing_widgets: &[String], mut location: Dynamic) -> Dynamic {
        let mut last_added: Option<String> = None;
        for widget in missing_widgets.iter() {
            if self.is_enabled(widget) && location.get(widget).result().is_none() {
                location = location.set(widget, &ConfigFix11NewHud::create_default_widget(&location, last_added.clone()));
                last_added = Some(widget.clone());
            }
        }
        return location;
    }

    /// Fixes widgets in this skyblock location and returns a map of layers to widgets.
    /// java: private Pair<Dynamic<?>, Dynamic<?>> fixWidgetsForLocation(Pair<Dynamic<?>, Dynamic<?>> location)
    pub fn fix_widgets_for_location(&self, location: (Dynamic, Dynamic)) -> (Dynamic, Dynamic) {
        let location_id: Dynamic = location.0;
        // java: new HashMap<>(Map.of(..)) keyed by Dynamic.equals - a list with equality lookup
        let mut layers: Vec<(Dynamic, Dynamic)> = vec![
            (location_id.create_string("hud"), location_id.empty_map()),
            (location_id.create_string("tab"), location_id.empty_map()),
            (location_id.create_string("secondary_tab"), location_id.empty_map()),
        ];
        // java: getMapValues().getOrThrow()
        let widgets = location.1.get_map_values().get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}"));
        for (widget_id, widget) in widgets {
            self.fix_widget_and_layer(ConfigFix11NewHud::fix_widget_id(&widget_id).as_string_or(""), &widget, &mut layers);
        }
        // java: layers.replaceAll((_, widgets) -> locationId.emptyMap().set("widgets", widgets));
        for layer in layers.iter_mut() {
            layer.1 = location_id.empty_map().set("widgets", &layer.1);
        }
        let map = location_id.create_map(layers);
        return (location_id, map);
    }

    /// java: private <T> void fixWidgetAndLayer(String widgetIdNew, Dynamic<T> widget, Map<Dynamic<?>, Dynamic<?>> layers)
    pub fn fix_widget_and_layer(&self, widget_id_new: String, widget: &Dynamic, layers: &mut Vec<(Dynamic, Dynamic)>) {
        if !self.is_enabled(&widget_id_new) {
            return;
        }
        // java: layers.computeIfPresent(fixWidgetLayer(widget, widgetIdNew), (_, widgets) -> widgets.set(widgetIdNew, fixWidget(widget)));
        let key = ConfigFix11NewHud::fix_widget_layer(widget, &widget_id_new);
        if let Some(entry) = layers.iter_mut().find(|(k, _)| *k == key) {
            entry.1 = entry.1.set(&widget_id_new, &ConfigFix11NewHud::fix_widget(widget));
        }
    }

    /// java: private static <T> Dynamic<T> fixWidgetId(Dynamic<T> widgetId)
    pub fn fix_widget_id(widget_id: &Dynamic) -> Dynamic {
        return match widget_id.as_string_or("").as_str() {
            "sweepDetails" => widget_id.create_string("sweep_details"),
            "Lasso HUD" => widget_id.create_string("hud_lasso"),
            "Dungeon Splits" => widget_id.create_string("dungeon_splits"),
            "Item Pickup" => widget_id.create_string("item_pickup"),
            _ => widget_id.clone(),
        };
    }

    /// Returns the layer the widget should be on.
    /// java: private static <T> Dynamic<T> fixWidgetLayer(Dynamic<T> widget, String widgetId)
    pub fn fix_widget_layer(widget: &Dynamic, widget_id: &str) -> Dynamic {
        let layer: &str = match widget.get("layer").as_string_or("DEFAULT").as_str() {
            "HUD" => "hud",
            "MAIN_TAB" => "tab",
            "SECONDARY_TAB" => "secondary_tab",
            _ => {
                if widget_id.contains("hud") || widget_id == "sweep_details" || widget_id == "powder_mining_tracker" || widget_id == "dungeon_splits" || widget_id == "item_pickup" {
                    "hud"
                } else {
                    "tab"
                }
            }
        };
        return widget.create_string(layer);
    }

    /// java: private static <T> Dynamic<T> fixWidget(Dynamic<T> widget)
    pub fn fix_widget(widget: &Dynamic) -> Dynamic {
        return widget
            .empty_map()
            .set("config", &widget.empty_map())
            .set("position", &widget.remove("layer").replace_field("parent", "parent", ConfigFix11NewHud::fix_widget_parent(widget)));
    }

    /// Returns the parent of the widget or empty if the parent is the screen.
    /// java: private static <T> Optional<Dynamic<T>> fixWidgetParent(Dynamic<T> widget)
    pub fn fix_widget_parent(widget: &Dynamic) -> Option<Dynamic> {
        return if widget.get("parent").as_string_or("screen") == "screen" {
            None
        } else {
            widget.get("parent").map(|parent| ConfigFix11NewHud::fix_widget_id(&parent)).result()
        };
    }

    /// java: private static <T> Dynamic<T> createDefaultWidget(Dynamic<T> base, @Nullable String parent)
    pub fn create_default_widget(base: &Dynamic, parent: Option<String>) -> Dynamic {
        return base
            .empty_map()
            .set_field_if_present("parent", parent.as_ref().map(|parent| base.create_string(parent)))
            .set("parent_anchor", &base.empty_map().set("v", &base.create_string(if parent.is_none() { "TOP" } else { "BOTTOM" })).set("h", &base.create_string("LEFT")))
            .set("this_anchor", &base.empty_map().set("v", &base.create_string("TOP")).set("h", &base.create_string("LEFT")))
            .set("relative_x", &base.create_int(if parent.is_none() { 5 } else { 0 }))
            .set("relative_y", &base.create_int(if parent.is_none() { 5 } else { 2 }));
    }

    /// java: private boolean isEnabled(String widgetId)
    pub fn is_enabled(&self, widget_id: &str) -> bool {
        let guard = self.main_config.lock().unwrap();
        let Some(main_config) = guard.as_ref() else {
            return true;
        };
        return match widget_id {
            "dungeon_splits" | "Dungeon Splits" => main_config.get("dungeons").get("dungeonSplits").as_boolean_or(true),
            "hud_farming" => main_config.get("farming").get("farmingHud").get("enabled").as_boolean_or(true),
            "hud_treeprogress" => main_config.get("foraging").get("moongladeMarsh").get("enableTreeBreakProgress").as_boolean_or(true),
            "sweep_details" | "sweepDetails" => main_config.get("foraging").get("moongladeMarsh").get("enableSweepDetailsWidget").as_boolean_or(true),
            "hud_fishing" => main_config.get("helpers").get("fishing").get("enableFishingHud").as_boolean_or(true),
            "hud_lasso" | "Lasso HUD" => main_config.get("hunting").get("lassoHud").get("enabled").as_boolean_or(true),
            "powder_mining_tracker" => main_config.get("mining").get("crystalHollows").get("enablePowderTracker").as_boolean_or(true),
            "hud_crystals" => main_config.get("mining").get("crystalsHud").get("enabled").as_boolean_or(true),
            "hud_pickobulus" => main_config.get("mining").get("pickobulusHelper").get("enablePickobulusHud").as_boolean_or(true),
            "hud_end" => main_config.get("otherLocations").get("end").get("hudEnabled").as_boolean_or(true),
            "hud_slayer" => main_config.get("slayers").get("enableHud").as_boolean_or(true),
            "item_pickup" | "Item Pickup" => main_config.get("uiAndVisuals").get("itemPickup").get("enabled").as_boolean_or(true),
            _ => true,
        };
    }
}

impl DataFix for ConfigFix11NewHud {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        let this2 = self.self_ref.clone();
        return type_rewrite_rule::seq2(
            data_fix::fix_type_everywhere_typed_same(
                "ConfigFix11NewHud", // java: getClass().getSimpleName()
                self.get_input_schema()
                    .get_type(config_data_fixer::CONFIG_TYPE.get())
                    .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
                Arc::new(move |typed: Typed| {
                    let Some(this) = this.upgrade() else { return Err("ConfigFix11NewHud: the fix was dropped".to_string()) };
                    typed.update(&dsl::remainder_finder(), &|dynamic| this.collect(&dynamic))
                }),
            ),
            data_fix::fix_type_everywhere_typed_same(
                "ConfigFix11NewHud", // java: getClass().getSimpleName()
                self.get_input_schema()
                    .get_type(config_data_fixer::HUD_WIDGETS_TYPE.get())
                    .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
                Arc::new(move |typed: Typed| {
                    let Some(this) = this2.upgrade() else { return Err("ConfigFix11NewHud: the fix was dropped".to_string()) };
                    typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
                }),
            ),
        );
    }
}

impl ConfigDataFix for ConfigFix11NewHud {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}
