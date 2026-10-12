//! Ported from de.hysky.skyblocker.skyblock.ChaptersAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static List<String> locations = List.of();
pub static LOCATIONS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> singularCommandNode;
pub static SINGULAR_COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> pluralCommandNode;
pub static PLURAL_COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<ChaptersData> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.listOf().fieldOf("locations").forGetter(ChaptersData::locations)
//   java: 		).apply(instance, ChaptersData::new));
/// java: private static final Codec<ChaptersData> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.listOf().fieldOf("locations").forGetter(ChaptersData::locations) ).apply(instance, C
pub static CHAPTERS_DATA_CODEC: compat::MainThread<CodecRef<ChaptersData>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public final class ChaptersAutocomplete
#[derive(Clone)]
pub struct ChaptersAutocomplete {}

impl ChaptersAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/chapters");
                let mut json: String = todo!("fabric2seer: the initialiser of `json` (TODO above)");
                let mut data: ChaptersData = (CHAPTERS_DATA_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")))).get_or_throw().unwrap();
                compat::set_state(&LOCATIONS, data.locations.clone());
                *SINGULAR_COMMAND_NODE.get_mut() = Some(ChaptersAutocomplete::create_command_node("chapter"));
                *PLURAL_COMMAND_NODE.get_mut() = Some(ChaptersAutocomplete::create_command_node("chapters"));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Chapters Autocomplete] Failed to load Chapters data..."));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command)
    pub fn create_command_node(command: &str) -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("island", StringArgumentType.string())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(locations, builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				).build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: private record ChaptersData(List<String> locations)
#[derive(Clone, Debug, PartialEq)]
pub struct ChaptersData {
    /// java: List<String> locations
    pub locations: Vec<String>,
}

impl ChaptersData {
    /// java: the canonical record constructor
    pub fn new(mut locations: Vec<String>) -> Self {
        Self { locations }
    }

    /// java: the record accessor `locations()`
    pub fn locations(&self) -> Vec<String> {
        self.locations.clone()
    }

}
