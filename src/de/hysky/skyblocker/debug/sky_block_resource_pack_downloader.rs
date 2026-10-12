//! Ported from de.hysky.skyblocker.debug.SkyBlockResourcePackDownloader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::file_utils::FileUtils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::server::packs::pack_type::PackType;
use mcrust::net::minecraft::shared_constants;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final int CURRENT_RESOURCE_PACK_VERSION = SharedConstants.getCurrentVersion().packVersion(PackType.CLIENT_RESOURCES).major();
pub static CURRENT_RESOURCE_PACK_VERSION: LazyLock<i32> = LazyLock::new(|| shared_constants::get_current_version().pack_version(PackType::ClientResources).major());
/// java: private static final String HYPIXEL_RESOURCE_PACKS_API = "https://api.hypixel.net/v2/resources/packs";
pub const HYPIXEL_RESOURCE_PACKS_API: &str = "https://api.hypixel.net/v2/resources/packs";
/// java: private static final String SKYBLOCK_RESOURCE_PACK_ID = "SkyBlock";
pub const SKYBLOCK_RESOURCE_PACK_ID: &str = "SkyBlock";
/// java: private static final Codec<HypixelPack> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("id").forGetter(HypixelPack::id), Codec.LONG.fieldOf("lastUpdated").forGetter
pub static HYPIXEL_PACK_CODEC: compat::MainThread<CodecRef<HypixelPack>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &HypixelPack| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &HypixelPack| o.last_updated(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "lastUpdated")), record_codec_builder::of(|o: &HypixelPack| o.deploy_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "deployId")), record_codec_builder::of(|o: &HypixelPack| o.versions(), codec::field_of(codec::list(HYPIXEL_PACK_VERSION_CODEC.get_mut().clone()), "versions")), move |__a0, __a1, __a2, __a3| HypixelPack::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));
/// java: private static final Codec<List<HypixelPack>> LIST_CODEC = CODEC.listOf();
pub static HYPIXEL_PACK_LIST_CODEC: compat::MainThread<CodecRef<Vec<HypixelPack>>> = compat::MainThread::new(|| codec::list(HYPIXEL_PACK_CODEC.get_mut().clone()));
/// java: private static final Codec<HypixelPackVersion> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("packFormat").forGetter(HypixelPackVersion::packFormat), Codec.STRING.fie
pub static HYPIXEL_PACK_VERSION_CODEC: compat::MainThread<CodecRef<HypixelPackVersion>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &HypixelPackVersion| o.pack_format(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "packFormat")), record_codec_builder::of(|o: &HypixelPackVersion| o.hash(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "hash")), record_codec_builder::of(|o: &HypixelPackVersion| o.url(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "url")), move |__a0, __a1, __a2| HypixelPackVersion::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public class SkyBlockResourcePackDownloader
#[derive(Clone)]
pub struct SkyBlockResourcePackDownloader {}

impl SkyBlockResourcePackDownloader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && SkyblockerConfigManager::get().debug.skyblock_resource_pack.download_resource_pack {
            SkyBlockResourcePackDownloader::download_resource_pack();
        }
    }

    /// java: public static void downloadResourcePack()
    pub fn download_resource_pack() {
        CompletableFuture::<()>::run_async(move || (|| { SkyBlockResourcePackDownloader::download_resource_pack_internal(); })(), compat::executor);
    }

    /// java: private static boolean downloadResourcePackInternal()
    pub fn download_resource_pack_internal() -> bool {
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            // TODO(fabric2seer): `Minecraft.getResourcePackDirectory` has no Rust method in the port (looked for `get_resource_pack_directory` in Minecraft and its supertypes)
            // rust: let mut resource_pack_destination: std::path::PathBuf = compat::minecraft().get_resource_pack_directory().join("Hypixel SkyBlock");
            let mut resource_pack_destination: std::path::PathBuf = todo!("fabric2seer: the initialiser of `resourcePackDestination` (TODO above)");
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String packsResponse = Http.sendGetRequest(HYPIXEL_RESOURCE_PACKS_API);
            let mut packs_response: String = todo!("fabric2seer: the initialiser of `packsResponse` (TODO above)");
            let mut packs: Vec<HypixelPack> = (HYPIXEL_PACK_LIST_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&packs_response).expect("JsonSyntaxException")))).get("packs").cloned().unwrap()))).get_or_throw().unwrap();
            let mut skyblock_resource_pack: HypixelPack = packs.iter().cloned().filter(|pack| (pack.id() == SKYBLOCK_RESOURCE_PACK_ID)).next().unwrap();
            // TODO(fabric2seer): Files.getLastModifiedTime
            //   java: boolean shouldUpdatePack = !Files.exists(resourcePackDestination) || skyblockResourcePack.lastUpdated() > Files.getLastModifiedTime(resourcePackDestination).toMillis();
            let mut should_update_pack: bool = todo!("fabric2seer: the initialiser of `shouldUpdatePack` (TODO above)");
            let mut pack_url: Option<String> = skyblock_resource_pack.versions().into_iter().filter(|version| (version.pack_format() == *CURRENT_RESOURCE_PACK_VERSION)).map(|__a0| __a0.url()).next();
            if should_update_pack && pack_url.is_some() {
                FileUtils::recursive_delete(&resource_pack_destination);
                // TODO(fabric2seer): `Http.downloadContent` is not a static method of Http
                //   java: InputStream packInputStream = Http.downloadContent(packUrl.get());
                let mut pack_input_stream: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `packInputStream` (TODO above)");
                SkyBlockResourcePackDownloader::extract_resource_pack(&resource_pack_destination, &pack_input_stream);
                // TODO(fabric2seer): static call FileTime.fromMillis(..)
                //   java: Files.setLastModifiedTime(resourcePackDestination, FileTime.fromMillis(skyblockResourcePack.lastUpdated()));
                logger().info("[Skyblocker Resource Pack Downloader] Successfully downloaded the SkyBlock pack!");
                return true;
            } else if pack_url.is_none() {
                logger().warn(&format!("[Skyblocker Resource Pack Downloader] Could not find a version of the SkyBlock pack for format {}.", *CURRENT_RESOURCE_PACK_VERSION));
            }
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Resource Pack Downloader] Could not download the SkyBlock pack.", e);
        //   java: 		}
        return false;
    }

    /// java: private static void extractResourcePack(Path packDirectory, InputStream packInputStream) throws Exception
    pub fn extract_resource_pack(pack_directory: &std::path::PathBuf, pack_input_stream: &crate::unresolved::InputStream) {
        {
            // TODO(fabric2seer): `new ZipInputStream(..)`
            //   java: ZipInputStream zipInputStream = new ZipInputStream(packInputStream);
            let mut zip_input_stream: crate::unresolved::ZipInputStream = todo!("fabric2seer: the initialiser of `zipInputStream` (TODO above)");
            let mut entry: crate::unresolved::ZipEntry;
            // TODO(fabric2seer): the condition: `.getNextEntry(..)` on a ZipInputStream
            //   java: (entry = zipInputStream.getNextEntry()) != null
            while todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): `.getName(..)` on a ZipEntry
                //   java: Path outputPath = packDirectory.resolve(entry.getName()).normalize();
                let mut output_path: std::path::PathBuf = todo!("fabric2seer: the initialiser of `outputPath` (TODO above)");
                if !(output_path.starts_with(pack_directory)) {
                    // TODO(fabric2seer): `.getName(..)` on a ZipEntry
                    //   java: throw new IllegalStateException("Malicious ZIP entry detected: " + entry.getName());
                }
                // TODO(fabric2seer): the condition: `.isDirectory(..)` on a ZipEntry
                //   java: entry.isDirectory(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    std::fs::create_dir_all(&output_path).expect("IOException");
                } else {
                    std::fs::create_dir_all(&output_path.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
                    // TODO(fabric2seer): static field StandardCopyOption.REPLACE_EXISTING
                    //   java: Files.copy(zipInputStream, outputPath, StandardCopyOption.REPLACE_EXISTING);
                }
                // TODO(fabric2seer): `.closeEntry(..)` on a ZipInputStream
                //   java: zipInputStream.closeEntry();
            }
        }
    }

}

/// java: private record HypixelPack(String id, long lastUpdated, String deployId, List<HypixelPackVersion> versions)
#[derive(Clone, Debug, PartialEq)]
pub struct HypixelPack {
    /// java: String id
    pub id: String,
    /// java: long lastUpdated
    pub last_updated: i64,
    /// java: String deployId
    pub deploy_id: String,
    /// java: List<HypixelPackVersion> versions
    pub versions: Vec<HypixelPackVersion>,
}

impl HypixelPack {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut last_updated: i64, mut deploy_id: String, mut versions: Vec<HypixelPackVersion>) -> Self {
        Self { id, last_updated, deploy_id, versions }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `lastUpdated()`
    pub fn last_updated(&self) -> i64 {
        self.last_updated
    }

    /// java: the record accessor `deployId()`
    pub fn deploy_id(&self) -> String {
        self.deploy_id.clone()
    }

    /// java: the record accessor `versions()`
    pub fn versions(&self) -> Vec<HypixelPackVersion> {
        self.versions.clone()
    }

}

/// java: private record HypixelPackVersion(int packFormat, String hash, String url)
#[derive(Clone, Debug, PartialEq)]
pub struct HypixelPackVersion {
    /// java: int packFormat
    pub pack_format: i32,
    /// java: String hash
    pub hash: String,
    /// java: String url
    pub url: String,
}

impl HypixelPackVersion {
    /// java: the canonical record constructor
    pub fn new(mut pack_format: i32, mut hash: String, mut url: String) -> Self {
        Self { pack_format, hash, url }
    }

    /// java: the record accessor `packFormat()`
    pub fn pack_format(&self) -> i32 {
        self.pack_format
    }

    /// java: the record accessor `hash()`
    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    /// java: the record accessor `url()`
    pub fn url(&self) -> String {
        self.url.clone()
    }

}
