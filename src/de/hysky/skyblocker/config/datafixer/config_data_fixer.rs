//! Ported from de.hysky.skyblocker.config.datafixer.ConfigDataFixer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_fix10_torrhus_canyon_and_safari::ConfigFix10TorrhusCanyonAndSafari;
use crate::de::hysky::skyblocker::config::datafixer::config_fix11_new_hud::ConfigFix11NewHud;
use crate::de::hysky::skyblocker::config::datafixer::config_fix12_move_widget_options::ConfigFix12MoveWidgetOptions;
use crate::de::hysky::skyblocker::config::datafixer::config_fix1::ConfigFix1;
use crate::de::hysky::skyblocker::config::datafixer::config_fix2_quick_nav::ConfigFix2QuickNav;
use crate::de::hysky::skyblocker::config::datafixer::config_fix3_animated_dye_and_item_background::ConfigFix3AnimatedDyeAndItemBackground;
use crate::de::hysky::skyblocker::config::datafixer::config_fix4_chat_rules_object::ConfigFix4ChatRulesObject;
use crate::de::hysky::skyblocker::config::datafixer::config_fix5_chat_rules_separate_outputs::ConfigFix5ChatRulesSeparateOutputs;
use crate::de::hysky::skyblocker::config::datafixer::config_fix6_builders_wand_config::ConfigFix6BuildersWandConfig;
use crate::de::hysky::skyblocker::config::datafixer::config_fix7_farming::ConfigFix7Farming;
use crate::de::hysky::skyblocker::config::datafixer::config_fix8_item_list::ConfigFix8ItemList;
use crate::de::hysky::skyblocker::config::datafixer::config_fix9_event_notifications::ConfigFix9EventNotifications;
use crate::de::hysky::skyblocker::config::datafixer::config_schema::ConfigSchema;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager;
use mcrust::com::mojang::datafixers::data_fixer::DataFixer;
use mcrust::com::mojang::datafixers::data_fixer_builder::DataFixerBuilder;
use mcrust::com::mojang::datafixers::dsl::TypeReference;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public static final String VERSION_KEY = "version";
pub const VERSION_KEY: &str = "version";
/// java: public static final DSL.TypeReference CONFIG_TYPE = () -> "config";
pub static CONFIG_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| Arc::new(move || "config".to_string()));
/// java: public static final DSL.TypeReference CHAT_RULES_TYPE = () -> "chat_rules";
pub static CHAT_RULES_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| Arc::new(move || "chat_rules".to_string()));
/// java: public static final DSL.TypeReference HUD_WIDGETS_TYPE = () -> "hud_widgets";
pub static HUD_WIDGETS_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| Arc::new(move || "hud_widgets".to_string()));
/// java: private static @Nullable DataFixer dataFixer;
pub static DATA_FIXER: compat::MainThread<Option<Box<dyn DataFixer>>> = compat::MainThread::new(|| None);

/// java: public class ConfigDataFixer
#[derive(Clone)]
pub struct ConfigDataFixer {}

impl ConfigDataFixer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static JsonObject apply(DSL.TypeReference type, JsonObject oldConfig)
    pub fn apply(r#type: &TypeReference, old_config: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
        return ConfigDataFixer::apply_type_reference_json_object_int(r#type, old_config, skyblocker_config_manager::CONFIG_VERSION);
    }

    /// java: public static <T> Dynamic<T> apply(DSL.TypeReference type, Dynamic<T> oldConfig)
    pub fn apply_type_reference_dynamic<T: 'static>(r#type: &TypeReference, old_config: &Dynamic) -> Dynamic {
        return ConfigDataFixer::apply_type_reference_dynamic_int(r#type, old_config, skyblocker_config_manager::CONFIG_VERSION);
    }

    /// java: public static <T> Dynamic<T> apply(DSL.TypeReference type, Dynamic<T> oldConfig, int newVersion)
    pub fn apply_type_reference_dynamic_int<T: 'static>(r#type: &TypeReference, old_config: &Dynamic, mut new_version: i32) -> Dynamic {
        let mut start: i64 = compat::millis();
        let mut new_config: Dynamic = ConfigDataFixer::build().update(r#type, old_config.clone(), old_config.get(VERSION_KEY).as_int(1), new_version).unwrap();
        let mut end: i64 = compat::millis();
        logger().info(&format!("[Skyblocker Config Data Fixer] Applied {} datafixers in {} ms!", (r#type).type_name(), end.wrapping_sub(start)));
        return new_config;
    }

    /// java: public static JsonObject apply(DSL.TypeReference type, JsonObject oldConfig, int newVersion)
    pub fn apply_type_reference_json_object_int(r#type: &TypeReference, old_config: &serde_json::Map<String, serde_json::Value>, mut new_version: i32) -> serde_json::Map<String, serde_json::Value> {
        // TODO(fabric2seer): `.getAsJsonObject(..)` on a T
        //   java: return apply(type, new Dynamic<>(JsonOps.INSTANCE, oldConfig), newVersion).getValue().getAsJsonObject();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static DataFixer build()
    pub fn build() -> Box<dyn DataFixer> {
        if DATA_FIXER.get_mut().is_some() {
            return DATA_FIXER.get_mut().clone().unwrap();
        }
        let mut builder: DataFixerBuilder = DataFixerBuilder::new(skyblocker_config_manager::CONFIG_VERSION);
        builder.add_schema(1, |__a0, __a1| ConfigSchema::new(__a0, __a1.clone()));
        let mut schema2: Arc<Schema> = builder.add_schema(2, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix1::new(schema2.clone(), true));
        let mut schema3: Arc<Schema> = builder.add_schema(3, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix2QuickNav::new(schema3.clone(), true));
        let mut schema4: Arc<Schema> = builder.add_schema(4, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix3AnimatedDyeAndItemBackground::new(schema4.clone(), true));
        let mut schema5: Arc<Schema> = builder.add_schema(5, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix4ChatRulesObject::new(schema5.clone(), true));
        let mut schema6: Arc<Schema> = builder.add_schema(6, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix5ChatRulesSeparateOutputs::new(schema6.clone(), true));
        let mut schema7: Arc<Schema> = builder.add_schema(7, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix6BuildersWandConfig::new(schema7.clone(), true));
        let mut schema8: Arc<Schema> = builder.add_schema(8, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix7Farming::new(schema8.clone(), true));
        let mut schema9: Arc<Schema> = builder.add_schema(9, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix8ItemList::new(schema9.clone(), true));
        let mut schema10: Arc<Schema> = builder.add_schema(10, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix9EventNotifications::new(schema10.clone(), true));
        let mut schema11: Arc<Schema> = builder.add_schema(11, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix10TorrhusCanyonAndSafari::new(schema11.clone(), true));
        let mut schema12: Arc<Schema> = builder.add_schema(12, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix11NewHud::new(schema12.clone(), true));
        let mut schema13: Arc<Schema> = builder.add_schema(13, |__a0, __a1| Schema::new(__a0, Some(__a1.clone())));
        builder.add_fixer(ConfigFix12MoveWidgetOptions::new(schema13.clone(), true));
        // TODO(fabric2seer): an assignment used as a value
        //   java: return dataFixer = builder.build().fixer();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <A> Codec<A> createDataFixingCodec(DSL.TypeReference type, Codec<A> baseCodec)
    pub fn create_data_fixing_codec<A: 'static>(r#type: &TypeReference, base_codec: &CodecRef<A>) -> CodecRef<A> {
        // TODO(fabric2seer): `new Codec(..) {..}` (an anonymous class with state or several methods)
        //   java: return new Codec<>() {
        //   java: 			@Override
        //   java: 			public <T> DataResult<T> encode(A input, DynamicOps<T> ops, T prefix) {
        //   java: 				return baseCodec.encode(input, ops, prefix).flatMap(t -> ops.mergeToMap(t, ops.createString(VERSION_KEY), ops.createInt(SkyblockerConfigManager.CONFIG_VERSION)));
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			public <T> DataResult<Pair<A, T>> decode(DynamicOps<T> ops, T input) {
        //   java: 				Dynamic<T> dynamic2 = apply(type, new Dynamic<>(ops, input));
        //   java: 				return baseCodec.decode(dynamic2);
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
