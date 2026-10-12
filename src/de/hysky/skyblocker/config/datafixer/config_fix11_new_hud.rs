//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix11NewHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::util::pair::Pair;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: private static final Map<String, String[]> LOCATION_WIDGETS = Map.ofEntries( Map.entry("garden", new String[]{"hud_farming", "sweepDetails"}), Map.entry("hub", new String[]{"sweepDetails", "hud_slayer
pub static LOCATION_WIDGETS: LazyLock<std::collections::HashMap<String, Vec<String>>> = LazyLock::new(|| std::collections::HashMap::from([("garden".to_string(), vec!["hud_farming".to_string(), "sweepDetails".to_string()]), ("hub".to_string(), vec!["sweepDetails".to_string(), "hud_slayer".to_string(), "hud_fishing".to_string()]), ("foraging_1".to_string(), vec!["sweepDetails".to_string(), "hud_slayer".to_string(), "hud_fishing".to_string()]), ("foraging_2".to_string(), vec!["sweepDetails".to_string(), "Lasso HUD".to_string(), "hud_treeprogress".to_string(), "hud_fishing".to_string()]), ("foraging_3".to_string(), vec!["sweepDetails".to_string(), "Lasso HUD".to_string(), "hud_treeprogress".to_string(), "hud_fishing".to_string()]), ("crystal_hollows".to_string(), vec!["hud_crystals".to_string(), "powder_mining_tracker".to_string(), "hud_fishing".to_string()]), ("combat_3".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("combat_1".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("rift".to_string(), vec!["hud_slayer".to_string()]), ("crimson_isle".to_string(), vec!["hud_slayer".to_string(), "hud_fishing".to_string()]), ("lotus_atoll".to_string(), vec!["hud_fishing".to_string()]), ("fishing_1".to_string(), vec!["hud_fishing".to_string()])]));

/// java: public class ConfigFix11NewHud extends ConfigDataFix
pub struct ConfigFix11NewHud {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: private @Nullable Dynamic<?> mainConfig;
    pub main_config: Option<Dynamic>,
}

impl std::ops::Deref for ConfigFix11NewHud {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix11NewHud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix11NewHud {
    /// java: public ConfigFix11NewHud(Schema outputSchema, boolean changesType)
    pub fn new(mut output_schema: Arc<Schema>, mut changes_type: bool) -> compat::Shared<Self> {
        let (_, __this) = ConfigDataFixBase::new_in(output_schema.clone(), changes_type, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                main_config: None,
            });
            ((__h.clone() as compat::Shared<dyn ConfigDataFix>), __h)
        });
        __this
    }

    /// java: private static <T> Dynamic<T> fixWidgetId(Dynamic<T> widgetId)
    pub fn fix_widget_id<T: 'static>(widget_id: &Dynamic) -> Dynamic {
        return match widget_id.as_string_or("").as_str() {
            "sweepDetails" => widget_id.create_string("sweep_details"),
            "Lasso HUD" => widget_id.create_string("hud_lasso"),
            "Dungeon Splits" => widget_id.create_string("dungeon_splits"),
            "Item Pickup" => widget_id.create_string("item_pickup"),
            _ => widget_id.clone(),
        };
    }

    /// java: private static <T> Dynamic<T> fixWidgetLayer(Dynamic<T> widget, String widgetId)
    pub fn fix_widget_layer<T: 'static>(widget: &Dynamic, widget_id: &str) -> Dynamic {
        let mut layer: String = match widget.get("layer").as_string_or("DEFAULT").as_str() {
            "HUD" => "hud".to_string(),
            "MAIN_TAB" => "tab".to_string(),
            "SECONDARY_TAB" => "secondary_tab".to_string(),
            _ => (if widget_id.contains("hud") || widget_id == "sweep_details" || widget_id == "powder_mining_tracker" || widget_id == "dungeon_splits" || widget_id == "item_pickup" { "hud" } else { "tab" }).to_string(),
        };
        return widget.create_string(&layer);
    }

    /// java: private static <T> Dynamic<T> fixWidget(Dynamic<T> widget)
    pub fn fix_widget<T: 'static>(widget: &mut Dynamic) -> Dynamic {
        return (widget.empty_map().set("config", &widget.empty_map())).set("position", &widget.remove("layer").replace_field("parent", "parent", ConfigFix11NewHud::fix_widget_parent(widget)));
    }

    /// java: private static <T> Optional<Dynamic<T>> fixWidgetParent(Dynamic<T> widget)
    pub fn fix_widget_parent<T: 'static>(widget: &Dynamic) -> Option<Dynamic> {
        return (if widget.get("parent").as_string_or("screen") == "screen" { None } else { (widget.get("parent").map(|__a0| ConfigFix11NewHud::fix_widget_id(&__a0))).result() }).unwrap();
    }

    /// java: private static <T> Dynamic<T> createDefaultWidget(Dynamic<T> base, @Nullable String parent)
    pub fn create_default_widget<T: 'static>(base: &Dynamic, mut parent: Option<String>) -> Dynamic {
        return (base.empty_map().set_field_if_present("parent", parent.clone().map(|__a0| base.create_string(&__a0)))).set("parent_anchor", &(base.empty_map().set("v", &base.create_string((if parent.is_none() { "TOP" } else { "BOTTOM" })))).set("h", &base.create_string("LEFT"))).set("this_anchor", &(base.empty_map().set("v", &base.create_string("TOP"))).set("h", &base.create_string("LEFT"))).set("relative_x", &base.create_int((if parent.is_none() { 5 } else { 0 }))).set("relative_y", &base.create_int((if parent.is_none() { 5 } else { 2 })));
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    pub fn make_rule(__this: &compat::Shared<Self>) -> TypeRewriteRule {
        // TODO(fabric2seer): `.getSimpleName(..)` on a value of unknown type
        //   java: return TypeRewriteRule.seq(fixTypeEverywhereTyped(
        //   java: 						getClass().getSimpleName(),
        //   java: 						getInputSchema().getType(ConfigDataFixer.CONFIG_TYPE),
        //   java: 						typed -> typed.update(DSL.remainderFinder(), this::collect)),
        //   java: 				fixTypeEverywhereTyped(
        //   java: 						getClass().getSimpleName(),
        //   java: 						getInputSchema().getType(ConfigDataFixer.HUD_WIDGETS_TYPE),
        //   java: 						typed -> typed.update(DSL.remainderFinder(), this::fix))
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <T> Dynamic<T> collect(Dynamic<T> dynamic)
    pub fn collect<T: 'static>(__this: &compat::Shared<Self>, mut dynamic: Dynamic) -> Dynamic {
        dynamic = { let (__q0, __q1,) = ("mining", |mining| (mining.update("pickobulusHelper", |pickobulus_helper| pickobulus_helper.set_field_if_present("enablePickobulusHelper", mining.get("enablePickobulusHelper").result()))).remove("enablePickobulusHelper"),); __this.lock().borrow().fix_version(&mut dynamic).update(__q0, __q1) };
        { let __v = Some(dynamic.clone()); compat::write(__this, |__t| __t.main_config = __v); }
        return dynamic;
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): `Dynamic.createMap` has no Rust method in the port (looked for `create_map` in Dynamic and its supertypes)
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: let mut updated: Dynamic = { let (__q0, __q1,) = ("copies", &dynamic.create_map(&std::collections::HashMap::from([(dynamic.create_string("hud"), dynamic.empty_map()), (dynamic.create_string("tab"), dynamic.empty_map()), (dynamic.create_string("secondary_tab"), dynamic.empty_map())])),); ({ let (__q0, __q1, __q2,) = ("positions", "configs", |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix11NewHud::fix_widgets((&__this_1), __a0.clone()) },); __this.lock().borrow().fix_version(dynamic).rename_and_fix_field(__q0, __q1, __q2) }).set(__q0, __q1) };
        let mut updated: Dynamic = todo!("fabric2seer: the initialiser of `updated` (TODO above)");
        { let __v = None; compat::write(__this, |__t| __t.main_config = __v); }
        return updated;
    }

    /// java: private <T> Dynamic<T> fixWidgets(Dynamic<T> locations)
    pub fn fix_widgets<T: 'static>(__this: &compat::Shared<Self>, mut locations: Dynamic) -> Dynamic {
        for entry in LOCATION_WIDGETS.iter().map(|(k, v)| (k.clone(), v.clone())) {
            if locations.get(&entry.0).result().is_none() {
                locations = locations.set(&entry.0, &locations.empty_map());
            }
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            locations = locations.update(&entry.0, |location| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix11NewHud::fix_missing_widgets_for_location((&__this_1), &entry.1, location.clone()) });
        }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        return locations.update_map_values(|__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; ConfigFix11NewHud::fix_widgets_for_location((&__this_2), &__a0) });
    }

    /// java: private <T> Dynamic<T> fixMissingWidgetsForLocation(String[] missingWidgets, Dynamic<T> location)
    pub fn fix_missing_widgets_for_location<T: 'static>(__this: &compat::Shared<Self>, missing_widgets: &Vec<String>, mut location: Dynamic) -> Dynamic {
        let mut last_added: Option<String> = None;
        for widget in missing_widgets.iter() {
            if ConfigFix11NewHud::is_enabled(__this, widget) && location.get(widget).result().is_none() {
                location = location.set(widget, &ConfigFix11NewHud::create_default_widget(&location, last_added.clone()));
                last_added = Some(widget.to_string());
            }
        }
        return location;
    }

    /// java: private Pair<Dynamic<?>, Dynamic<?>> fixWidgetsForLocation(Pair<Dynamic<?>, Dynamic<?>> location)
    pub fn fix_widgets_for_location(__this: &compat::Shared<Self>, location: &(Dynamic, Dynamic)) -> (Dynamic, Dynamic) {
        let location_id = &location.0;
        let mut layers: std::collections::HashMap<Dynamic, Dynamic> = std::collections::HashMap::from([(location_id.create_string("hud"), location_id.empty_map()), (location_id.create_string("tab"), location_id.empty_map()), (location_id.create_string("secondary_tab"), location_id.empty_map())]);
        for (widget_id, widget) in location.1.get_map_values().get_or_throw().unwrap().iter() {
            ConfigFix11NewHud::fix_widget_and_layer(__this, ConfigFix11NewHud::fix_widget_id(widget_id).as_string_or(""), &mut widget, &mut layers);
        }
        // TODO(fabric2seer): Map.replaceAll(..)
        //   java: layers.replaceAll((_, widgets) -> locationId.emptyMap().set("widgets", widgets));
        // TODO(fabric2seer): `Dynamic.createMap` has no Rust method in the port (looked for `create_map` in Dynamic and its supertypes)
        // rust: return Pair::of(location_id.clone(), location_id.create_map(&layers));
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private <T> void fixWidgetAndLayer(String widgetIdNew, Dynamic<T> widget, Map<Dynamic<?>, Dynamic<?>> layers)
    pub fn fix_widget_and_layer<T: 'static>(__this: &compat::Shared<Self>, mut widget_id_new: String, widget: &mut Dynamic, layers: &mut std::collections::HashMap<Dynamic, Dynamic>) {
        if !(ConfigFix11NewHud::is_enabled(__this, &widget_id_new)) {
            return;
        }
        // TODO(fabric2seer): Map.computeIfPresent(..)
        //   java: layers.computeIfPresent(
        //   java: 				fixWidgetLayer(widget, widgetIdNew),
        //   java: 				(_, widgets) -> widgets.set(widgetIdNew, fixWidget(widget))
        //   java: 		);
    }

    /// java: private boolean isEnabled(String widgetId)
    pub fn is_enabled(__this: &compat::Shared<Self>, widget_id: &str) -> bool {
        if compat::read(__this, |__t| __t.main_config.clone()).is_none() {
            return true;
        }
        return match widget_id.as_str() {
            "dungeon_splits" | "Dungeon Splits" => __this.lock().borrow().main_config.as_ref().unwrap().get("dungeons").get("dungeonSplits").as_boolean_or(true),
            "hud_farming" => __this.lock().borrow().main_config.as_ref().unwrap().get("farming").get("farmingHud").get("enabled").as_boolean_or(true),
            "hud_treeprogress" => __this.lock().borrow().main_config.as_ref().unwrap().get("foraging").get("moongladeMarsh").get("enableTreeBreakProgress").as_boolean_or(true),
            "sweep_details" | "sweepDetails" => __this.lock().borrow().main_config.as_ref().unwrap().get("foraging").get("moongladeMarsh").get("enableSweepDetailsWidget").as_boolean_or(true),
            "hud_fishing" => __this.lock().borrow().main_config.as_ref().unwrap().get("helpers").get("fishing").get("enableFishingHud").as_boolean_or(true),
            "hud_lasso" | "Lasso HUD" => __this.lock().borrow().main_config.as_ref().unwrap().get("hunting").get("lassoHud").get("enabled").as_boolean_or(true),
            "powder_mining_tracker" => __this.lock().borrow().main_config.as_ref().unwrap().get("mining").get("crystalHollows").get("enablePowderTracker").as_boolean_or(true),
            "hud_crystals" => __this.lock().borrow().main_config.as_ref().unwrap().get("mining").get("crystalsHud").get("enabled").as_boolean_or(true),
            "hud_pickobulus" => __this.lock().borrow().main_config.as_ref().unwrap().get("mining").get("pickobulusHelper").get("enablePickobulusHud").as_boolean_or(true),
            "hud_end" => __this.lock().borrow().main_config.as_ref().unwrap().get("otherLocations").get("end").get("hudEnabled").as_boolean_or(true),
            "hud_slayer" => __this.lock().borrow().main_config.as_ref().unwrap().get("slayers").get("enableHud").as_boolean_or(true),
            "item_pickup" | "Item Pickup" => __this.lock().borrow().main_config.as_ref().unwrap().get("uiAndVisuals").get("itemPickup").get("enabled").as_boolean_or(true),
            _ => true,
        };
    }

}

impl ConfigDataFix for ConfigFix11NewHud {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
    fn config_data_fix_base_mut(&mut self) -> &mut ConfigDataFixBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
