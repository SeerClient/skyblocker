//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix12MoveWidgetOptions by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::util::pair::Pair;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix12MoveWidgetOptions extends ConfigDataFix
pub struct ConfigFix12MoveWidgetOptions {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: private final Map<String, SubFixer<?, ?>> widgetToSubFixer = Map.of( "hud_end", new SubFixer<>( dynamic -> dynamic.get("otherLocations").get("end").result(), th
    pub widget_to_sub_fixer: std::collections::HashMap<String, SubFixer<compat::Object, compat::Object>>,
}

impl std::ops::Deref for ConfigFix12MoveWidgetOptions {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix12MoveWidgetOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix12MoveWidgetOptions {
    /// java: public ConfigFix12MoveWidgetOptions(Schema outputSchema, boolean changesType)
    pub fn new(mut output_schema: Arc<Schema>, mut changes_type: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `widgetToSubFixer`: instance method `fixEnd` called from a static context
        //   java: private final Map<String, SubFixer<?, ?>> widgetToSubFixer = Map.of(
        //   java: 			"hud_end", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("otherLocations").get("end").result(),
        //   java: 					this::fixEnd
        //   java: 			),
        //   java: 			"hud_farming", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("farming").get("farmingHud").result(),
        //   java: 					this::fixFarming
        //   java: 			),
        //   java: 			"hud_fishing", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("helpers").get("fishing").result(),
        //   java: 					this::fixFishing
        //   java: 			),
        //   java: 			"item_pickup", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("uiAndVisuals").get("itemPickup").result(),
        //   java: 					this::fixItemPickup
        //   java: 			),
        //   java: 			"players", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("uiAndVisuals").get("tabHud").result(),
        //   java: 					this::fixPlayerList
        //   java: 			),
        //   java: 			"active_effects", new SubFixer<>(
        //   java: 					dynamic -> dynamic.get("uiAndVisuals").get("tabHud").result(),
        //   java: 					this::fixEffects
        //   java: 			)
        //   java: 	);
        let (_, __this) = ConfigDataFixBase::new_in(output_schema.clone(), changes_type, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                widget_to_sub_fixer: Default::default(),
            });
            ((__h.clone() as compat::Shared<dyn ConfigDataFix>), __h)
        });
        __this
    }

    /// java: private static <T> Dynamic<T> lowercase(Dynamic<T> dynamic)
    pub fn lowercase<T: 'static>(dynamic: &Dynamic) -> Dynamic {
        // TODO(fabric2seer): Optional.toLowerCase
        //   java: return dynamic.asString().result()
        //   java: 				.map(s -> s.toLowerCase(Locale.ENGLISH))
        //   java: 				.map(dynamic::createString)
        //   java: 				.orElse(dynamic);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    pub fn make_rule(__this: &compat::Shared<Self>) -> TypeRewriteRule {
        // TODO(fabric2seer): `.getSimpleName(..)` on a value of unknown type
        //   java: return TypeRewriteRule.seq(
        //   java: 				fixTypeEverywhereTyped(
        //   java: 						getClass().getSimpleName(),
        //   java: 						getInputSchema().getType(ConfigDataFixer.CONFIG_TYPE),
        //   java: 						typed -> typed.update(DSL.remainderFinder(), this::collect)
        //   java: 				),
        //   java: 				fixTypeEverywhereTyped(
        //   java: 						getClass().getSimpleName(),
        //   java: 						getInputSchema().getType(ConfigDataFixer.HUD_WIDGETS_TYPE),
        //   java: 						typed -> typed.update(DSL.remainderFinder(), this::fix)
        //   java: 				)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <T> Dynamic<T> collect(Dynamic<T> dynamic)
    pub fn collect<T: 'static>(__this: &compat::Shared<Self>, dynamic: &Dynamic) -> Dynamic {
        for fixer in __this.lock().borrow().widget_to_sub_fixer.values().cloned() {
            fixer.fetch_data(dynamic);
        }
        return dynamic.clone();
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut updated: Dynamic = { let (__q0, __q1,) = ("configs", |configs| configs.update_map_values(|location| Pair::of(location.0.clone(), location.1.update_map_values(|layer| Pair::of(layer.0.clone(), layer.1.update("widgets", |widgets| widgets.update_map_values(|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix12MoveWidgetOptions::fix_widget((&__this_1), &__a0) })))))),); __this.lock().borrow().fix_version(dynamic).update(__q0, __q1) };
        for fixer in __this.lock().borrow().widget_to_sub_fixer.values().cloned() {
            fixer.data = None;
        }
        return updated;
    }

    /// java: private Pair<Dynamic<?>, Dynamic<?>> fixWidget(Pair<Dynamic<?>, Dynamic<?>> widget)
    pub fn fix_widget(__this: &compat::Shared<Self>, widget: &(Dynamic, Dynamic)) -> (Dynamic, Dynamic) {
        let mut widget_id: String = widget.0.as_string_or("");
        let Some(mut fixer) = __this.lock().borrow().widget_to_sub_fixer.get(&widget_id).cloned() else {
            return widget.clone();
        };
        return Pair::of(widget.0.clone(), widget.1.update("config", |__a0| fixer.try_fix(&__a0)));
    }

    /// java: private <W, D> Dynamic<W> fixEnd(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_end<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return (widget_data.set_field_if_present("zealot_kills", previous.get("zealotKillsEnabled").result())).set_field_if_present("protector_location", previous.get("protectorLocationEnabled").result());
    }

    /// java: private <W, D> Dynamic<W> fixFarming(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_farming<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return (widget_data.set_field_if_present("counter", previous.get("counter").result())).set_field_if_present("coins", previous.get("coins").result()).set_field_if_present("price_type", (previous.get("type").map(|__a0| ConfigFix12MoveWidgetOptions::lowercase(&__a0))).result()).set_field_if_present("experience", previous.get("experience").result()).set_field_if_present("include_seeds_price", previous.get("includeSeedsPrice").result());
    }

    /// java: private <W, D> Dynamic<W> fixFishing(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_fishing<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return (widget_data.set_field_if_present("creature_counter", previous.get("enableSeaCreatureCounter").result())).set_field_if_present("fishing_timer", previous.get("enableFishingTimer").result()).set_field_if_present("only_barn", previous.get("onlyShowHudInBarn").result());
    }

    /// java: private <W, D> Dynamic<W> fixItemPickup(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_item_pickup<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return (widget_data.set_field_if_present("sack_notifications", previous.get("sackNotifications").result())).set_field_if_present("split_sack", previous.get("splitNotifications").result()).set_field_if_present("show_item_name", previous.get("showItemName").result()).set_field_if_present("lifetime", previous.get("lifetime").result());
    }

    /// java: private <W, D> Dynamic<W> fixEffects(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_effects<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data.set_field_if_present("effects_from_footer", previous.get("effectsFromFooter").result());
    }

    /// java: private <W, D> Dynamic<W> fixPlayerList(Dynamic<W> widgetData, Dynamic<D> previous)
    pub fn fix_player_list<W: 'static, D: 'static>(__this: &compat::Shared<Self>, widget_data: &mut Dynamic, previous: &Dynamic) -> Dynamic {
        return widget_data.set_field_if_present("name_sorting", (previous.get("nameSorting").map(|__a0| ConfigFix12MoveWidgetOptions::lowercase(&__a0))).result());
    }

}

impl ConfigDataFix for ConfigFix12MoveWidgetOptions {
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

/// java: private static class SubFixer<W, D>
#[derive(Clone)]
pub struct SubFixer<W, D> {
    /// java: private final Function<Dynamic<?>, Optional<Dynamic<D>>> previousDataSupplier;
    pub previous_data_supplier: Arc<dyn Fn(Dynamic) -> Option<Dynamic>>,
    /// java: private final BiFunction<Dynamic<W>, Dynamic<D>, Dynamic<W>> fixer;
    pub fixer: Arc<dyn Fn(Dynamic, Dynamic) -> Dynamic>,
    /// java: private @Nullable Dynamic<D> data;
    pub data: Option<Dynamic>,
}

impl<W: Clone + 'static, D: Clone + 'static> SubFixer<W, D> {
    /// java: private SubFixer(Function<Dynamic<?>, Optional<Dynamic<D>>> previousDataSupplier, BiFunction<Dynamic<W>, Dynamic<D>, Dynamic<W>> fixer)
    pub fn new(previous_data_supplier: Arc<dyn Fn(Dynamic) -> Option<Dynamic>>, fixer: Arc<dyn Fn(Dynamic, Dynamic) -> Dynamic>) -> Self {
        let __init_previous_data_supplier = previous_data_supplier.clone();
        let __init_fixer = fixer.clone();
        let mut this = Self {
            previous_data_supplier: __init_previous_data_supplier,
            fixer: __init_fixer,
            data: None,
        };
        this
    }

    /// java: @SuppressWarnings("unchecked") private Dynamic<?> tryFix(Dynamic<?> widget)
    pub fn try_fix(&self, widget: &Dynamic) -> Dynamic {
        if self.data.is_none() {
            return widget.clone();
        }
        return (self.fixer)(widget.clone(), self.data.clone());
    }

    /// java: private void fetchData(Dynamic<?> dynamic)
    pub fn fetch_data(&mut self, dynamic: &Dynamic) {
        self.data = (self.previous_data_supplier)(dynamic.clone());
    }

}
