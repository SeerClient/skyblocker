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
use mcrust::com::mojang::serialization::Obj;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::decoder::Decoder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::encoder::Encoder;
use mcrust::com::mojang::serialization::json_ops::JsonOps;

/// java: public static final String VERSION_KEY = "version";
pub const VERSION_KEY: &str = "version";
/// java: public static final DSL.TypeReference CONFIG_TYPE = () -> "config";
pub static CONFIG_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| TypeReference("config"));
/// java: public static final DSL.TypeReference CHAT_RULES_TYPE = () -> "chat_rules";
pub static CHAT_RULES_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| TypeReference("chat_rules"));
/// java: public static final DSL.TypeReference HUD_WIDGETS_TYPE = () -> "hud_widgets";
pub static HUD_WIDGETS_TYPE: compat::MainThread<TypeReference> = compat::MainThread::new(|| TypeReference("hud_widgets"));
/// java: private static @Nullable DataFixer dataFixer;
pub static DATA_FIXER: compat::MainThread<Option<Arc<dyn DataFixer>>> = compat::MainThread::new(|| None);

/// java: public class ConfigDataFixer
#[derive(Clone)]
pub struct ConfigDataFixer {}

impl ConfigDataFixer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static JsonObject apply(DSL.TypeReference type, JsonObject oldConfig)
    pub fn apply(r#type: &TypeReference, old_config: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
        return ConfigDataFixer::apply_type_reference_json_object_int(r#type, old_config, skyblocker_config_manager::CONFIG_VERSION);
    }

    /// java: public static <T> Dynamic<T> apply(DSL.TypeReference type, Dynamic<T> oldConfig)
    pub fn apply_type_reference_dynamic(r#type: &TypeReference, old_config: &Dynamic) -> Dynamic {
        return ConfigDataFixer::apply_type_reference_dynamic_int(r#type, old_config, skyblocker_config_manager::CONFIG_VERSION);
    }

    /// java: public static <T> Dynamic<T> apply(DSL.TypeReference type, Dynamic<T> oldConfig, int newVersion)
    pub fn apply_type_reference_dynamic_int(r#type: &TypeReference, old_config: &Dynamic, new_version: i32) -> Dynamic {
        let start: i64 = compat::millis();

        // java: DataFixer.update throws what the port answers as `Err`
        let new_config: Dynamic = ConfigDataFixer::build()
            .update(r#type, old_config.clone(), old_config.get(VERSION_KEY).as_int(1), new_version)
            .unwrap_or_else(|e| panic!("{e}"));

        let end: i64 = compat::millis();
        logger().info(&format!("[Skyblocker Config Data Fixer] Applied {} datafixers in {} ms!", r#type.type_name(), end.wrapping_sub(start)));
        return new_config;
    }

    /// java: public static JsonObject apply(DSL.TypeReference type, JsonObject oldConfig, int newVersion)
    pub fn apply_type_reference_json_object_int(r#type: &TypeReference, old_config: &serde_json::Map<String, serde_json::Value>, new_version: i32) -> serde_json::Map<String, serde_json::Value> {
        // java: new Dynamic<>(JsonOps.INSTANCE, oldConfig)
        let old = Dynamic::new_with(JsonOps::instance(), Some(compat::json_to_port(&serde_json::Value::Object(old_config.clone()))));
        let fixed = ConfigDataFixer::apply_type_reference_dynamic_int(r#type, &old, new_version);
        // java: .getValue().getAsJsonObject() - IllegalStateException for a non-object
        return match compat::json_from_port(fixed.get_value()) {
            serde_json::Value::Object(object) => object,
            other => panic!("java.lang.IllegalStateException: Not a JSON Object: {other}"),
        };
    }

    /// java: private static DataFixer build()
    pub fn build() -> Arc<dyn DataFixer> {
        if let Some(data_fixer) = DATA_FIXER.get().as_ref() {
            return data_fixer.clone();
        }
        let mut builder: DataFixerBuilder = DataFixerBuilder::new(skyblocker_config_manager::CONFIG_VERSION);

        builder.add_schema(1, ConfigSchema::new);
        let schema2: Arc<Schema> = builder.add_schema(2, Schema::new);
        builder.add_fixer(ConfigFix1::new(schema2, true));
        let schema3: Arc<Schema> = builder.add_schema(3, Schema::new);
        builder.add_fixer(ConfigFix2QuickNav::new(schema3, true));
        let schema4: Arc<Schema> = builder.add_schema(4, Schema::new);
        builder.add_fixer(ConfigFix3AnimatedDyeAndItemBackground::new(schema4, true));
        let schema5: Arc<Schema> = builder.add_schema(5, Schema::new);
        builder.add_fixer(ConfigFix4ChatRulesObject::new(schema5, true));
        let schema6: Arc<Schema> = builder.add_schema(6, Schema::new);
        builder.add_fixer(ConfigFix5ChatRulesSeparateOutputs::new(schema6, true));
        let schema7: Arc<Schema> = builder.add_schema(7, Schema::new);
        builder.add_fixer(ConfigFix6BuildersWandConfig::new(schema7, true));
        let schema8: Arc<Schema> = builder.add_schema(8, Schema::new);
        builder.add_fixer(ConfigFix7Farming::new(schema8, true));
        let schema9: Arc<Schema> = builder.add_schema(9, Schema::new);
        builder.add_fixer(ConfigFix8ItemList::new(schema9, true));
        let schema10: Arc<Schema> = builder.add_schema(10, Schema::new);
        builder.add_fixer(ConfigFix9EventNotifications::new(schema10, true));
        let schema11: Arc<Schema> = builder.add_schema(11, Schema::new);
        builder.add_fixer(ConfigFix10TorrhusCanyonAndSafari::new(schema11, true));
        let schema12: Arc<Schema> = builder.add_schema(12, Schema::new);
        builder.add_fixer(ConfigFix11NewHud::new(schema12, true));
        let schema13: Arc<Schema> = builder.add_schema(13, Schema::new);
        builder.add_fixer(ConfigFix12MoveWidgetOptions::new(schema13, true));

        // java: return dataFixer = builder.build().fixer();
        let data_fixer: Arc<dyn DataFixer> = builder.build().fixer();
        *DATA_FIXER.get_mut() = Some(data_fixer.clone());
        return data_fixer;
    }

    /// java: public static <A> Codec<A> createDataFixingCodec(DSL.TypeReference type, Codec<A> baseCodec)
    pub fn create_data_fixing_codec<A: 'static>(r#type: &TypeReference, base_codec: &CodecRef<A>) -> CodecRef<A> {
        return Arc::new(DataFixingCodec { r#type: *r#type, base_codec: base_codec.clone() });
    }
}

/// java: the anonymous `new Codec<>() {..}` of createDataFixingCodec (its captures `type` and `baseCodec`)
struct DataFixingCodec<A: 'static> {
    r#type: TypeReference,
    base_codec: CodecRef<A>,
}

impl<A: 'static> Encoder<A> for DataFixingCodec<A> {
    /// java: public <T> DataResult<T> encode(A input, DynamicOps<T> ops, T prefix)
    fn encode(&self, input: &A, ops: &Ops, prefix: &Obj) -> DataResult<Obj> {
        return self
            .base_codec
            .encode(input, ops, prefix)
            .flat_map(|t| ops.merge_to_map(&t, &ops.create_string(VERSION_KEY), &ops.create_int(skyblocker_config_manager::CONFIG_VERSION)));
    }
}

impl<A: 'static> Decoder<A> for DataFixingCodec<A> {
    /// java: public <T> DataResult<Pair<A, T>> decode(DynamicOps<T> ops, T input)
    fn decode(&self, ops: &Ops, input: &Obj) -> DataResult<(A, Obj)> {
        let dynamic2: Dynamic = ConfigDataFixer::apply_type_reference_dynamic(&self.r#type, &Dynamic::new_with(ops.clone(), Some(input.clone())));
        return self.base_codec.decode_dynamic(&dynamic2);
    }
}
