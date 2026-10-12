//! Ported from de.hysky.skyblocker.skyblock.WarpAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("warp_autocomplete.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("warp_autocomplete.json"));
/// java: private static final Codec<Object2BooleanMap<String>> MAP_CODEC = CodecUtils.object2BooleanMapCodec(Codec.STRING);
pub static MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, bool>>> = compat::MainThread::new(|| CodecUtils::object2_boolean_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> commandNode;
pub static COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);

/// java: public class WarpAutocomplete
#[derive(Clone)]
pub struct WarpAutocomplete {}

impl WarpAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            // TODO(fabric2seer): static call Object2BooleanMaps.emptyMap(..)
            //   java: Object2BooleanMap<String> warps = Object2BooleanMaps.<String>emptyMap();
            let mut warps: std::collections::HashMap<String, bool> = todo!("fabric2seer: the initialiser of `warps` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String warpsString = Http.sendGetRequest("https://hysky.de/api/locations");
                let mut warps_string: String = todo!("fabric2seer: the initialiser of `warpsString` (TODO above)");
                warps = (MAP_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&warps_string).expect("JsonSyntaxException")))).get_or_throw().unwrap();
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to download warps list"));
            }
            if warps.is_empty() {
                warps = WarpAutocomplete::get_warps_from_file();
            }
            WarpAutocomplete::create_command_node(&warps);
            WarpAutocomplete::save_warps_to_file(&warps);
        })(), compat::executor);
    }

    /// java: private static Object2BooleanMap<String> getWarpsFromFile()
    pub fn get_warps_from_file() -> std::collections::HashMap<String, bool> {
        let mut object: serde_json::Map<String, serde_json::Value>;
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (NoSuchFileException _) catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.fromJson(..)` on a Gson
                //   java: object = SkyblockerMod.GSON.fromJson(reader, JsonObject.class);
            }));
            if let Err(_) = __result {
                // TODO(fabric2seer): static call Object2BooleanMaps.emptyMap(..)
                //   java: return Object2BooleanMaps.emptyMap();
            }
            // TODO(fabric2seer): the second `catch (Exception e)` clause (merged into the first)
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker] Failed to read warp autocomplete file", e);
            //   java: 			return Object2BooleanMaps.emptyMap();
            //   java: 		}
        }
        // TODO(fabric2seer): static call Object2BooleanMaps.emptyMap(..)
        //   java: return MAP_CODEC.parse(JsonOps.INSTANCE, object).result().orElse(Object2BooleanMaps.emptyMap());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void saveWarpsToFile(Object2BooleanMap<String> warps)
    pub fn save_warps_to_file(warps: &std::collections::HashMap<String, bool>) {
        let mut result: Option<serde_json::Value> = (MAP_CODEC.get_mut().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), warps)).result().unwrap();
        if result.is_none() {
            return;
        }
        let mut warps_json: serde_json::Value = result.clone().unwrap();
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(warpsJson, writer);
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to save warps auto complete"));
            }
        }
    }

    /// java: private static void createCommandNode(Object2BooleanMap<String> warps)
    pub fn create_command_node(warps: &std::collections::HashMap<String, bool>) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: commandNode = literal("warp")
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("destination", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(getEligibleWarps(warps), builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				).build();
    }

    /// java: private static Stream<String> getEligibleWarps(Object2BooleanMap<String> warps)
    pub fn get_eligible_warps(warps: &std::collections::HashMap<String, bool>) -> Box<dyn Iterator<Item = String> + '_> {
        // TODO(fabric2seer): Map.object2BooleanEntrySet(..)
        //   java: return warps.object2BooleanEntrySet().stream()
        //   java: 				.filter(WarpAutocomplete::shouldShowWarp)
        //   java: 				.map(Object2BooleanMap.Entry::getKey);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static boolean shouldShowWarp(Object2BooleanMap.Entry<String> entry)
    pub fn should_show_warp(entry: &(String, bool)) -> bool {
        // TODO(fabric2seer): static call RankType.compare(..)
        //   java: return !entry.getBooleanValue() || Utils.isOnBingo() || RankType.compare(Utils.getRank(), PackageRank.MVP_PLUS) >= 0;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
