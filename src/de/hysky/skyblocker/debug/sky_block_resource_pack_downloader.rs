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
/// (read on the executor's thread by `downloadResourcePackInternal`: a codec is `Send + Sync`, so a `LazyLock`)
pub static HYPIXEL_PACK_CODEC: LazyLock<CodecRef<HypixelPack>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &HypixelPack| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &HypixelPack| o.last_updated(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "lastUpdated")), record_codec_builder::of(|o: &HypixelPack| o.deploy_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "deployId")), record_codec_builder::of(|o: &HypixelPack| o.versions(), codec::field_of(codec::list(HYPIXEL_PACK_VERSION_CODEC.clone()), "versions")), move |__a0, __a1, __a2, __a3| HypixelPack::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));
/// java: private static final Codec<List<HypixelPack>> LIST_CODEC = CODEC.listOf();
/// (a `LazyLock`, see `HYPIXEL_PACK_CODEC`)
pub static HYPIXEL_PACK_LIST_CODEC: LazyLock<CodecRef<Vec<HypixelPack>>> = LazyLock::new(|| codec::list(HYPIXEL_PACK_CODEC.clone()));
/// java: private static final Codec<HypixelPackVersion> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("packFormat").forGetter(HypixelPackVersion::packFormat), Codec.STRING.fie
/// (a `LazyLock`, see `HYPIXEL_PACK_CODEC`)
pub static HYPIXEL_PACK_VERSION_CODEC: LazyLock<CodecRef<HypixelPackVersion>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &HypixelPackVersion| o.pack_format(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "packFormat")), record_codec_builder::of(|o: &HypixelPackVersion| o.hash(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "hash")), record_codec_builder::of(|o: &HypixelPackVersion| o.url(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "url")), move |__a0, __a1, __a2| HypixelPackVersion::new(__a0.clone(), __a1.clone(), __a2.clone()))));

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
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return`
        // is the closure's value
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> bool {
            //   java: Path resourcePackDestination = CLIENT.getResourcePackDirectory().resolve("Hypixel SkyBlock");
            // (`Minecraft.resourcePackDirectory` is `GameConfig.location.resourcePackDirectory`, `<gameDir>/resourcepacks`)
            let mut resource_pack_destination: std::path::PathBuf = mcrust::mcrust::modding::config::game_dir().join("resourcepacks").join("Hypixel SkyBlock");
            //   java: String packsResponse = Http.sendGetRequest(HYPIXEL_RESOURCE_PACKS_API);
            let mut packs_response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request(HYPIXEL_RESOURCE_PACKS_API);
            let mut packs: Vec<HypixelPack> = (HYPIXEL_PACK_LIST_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&packs_response).expect("JsonSyntaxException")))).get("packs").cloned().unwrap()))).get_or_throw().unwrap();
            let mut skyblock_resource_pack: HypixelPack = packs.iter().cloned().filter(|pack| (pack.id() == SKYBLOCK_RESOURCE_PACK_ID)).next().unwrap();
            //   java: boolean shouldUpdatePack = !Files.exists(resourcePackDestination) || skyblockResourcePack.lastUpdated() > Files.getLastModifiedTime(resourcePackDestination).toMillis();
            let mut should_update_pack: bool = !resource_pack_destination.exists() || skyblock_resource_pack.last_updated() > {
                let modified = std::fs::metadata(&resource_pack_destination).and_then(|m| m.modified()).unwrap_or_else(|e| panic!("IOException: {e}"));
                modified.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or_else(|e| -(e.duration().as_millis() as i64))
            };
            let mut pack_url: Option<String> = skyblock_resource_pack.versions().into_iter().filter(|version| (version.pack_format() == *CURRENT_RESOURCE_PACK_VERSION)).map(|__a0| __a0.url()).next();
            if should_update_pack && pack_url.is_some() {
                FileUtils::recursive_delete(&resource_pack_destination);
                //   java: InputStream packInputStream = Http.downloadContent(packUrl.get());
                let mut pack_input_stream: Vec<u8> = crate::de::hysky::skyblocker::utils::http::Http::download_content(pack_url.as_deref().unwrap());
                SkyBlockResourcePackDownloader::extract_resource_pack(&resource_pack_destination, &pack_input_stream);
                //   java: Files.setLastModifiedTime(resourcePackDestination, FileTime.fromMillis(skyblockResourcePack.lastUpdated()));
                set_last_modified_time(&resource_pack_destination, skyblock_resource_pack.last_updated());
                logger().info("[Skyblocker Resource Pack Downloader] Successfully downloaded the SkyBlock pack!");
                return true;
            } else if pack_url.is_none() {
                logger().warn(&format!("[Skyblocker Resource Pack Downloader] Could not find a version of the SkyBlock pack for format {}.", *CURRENT_RESOURCE_PACK_VERSION));
            }
            false
        }));
        match __result {
            Ok(true) => return true,
            Ok(false) => {}
            Err(e) => {
                logger().error(&format!("[Skyblocker Resource Pack Downloader] Could not download the SkyBlock pack. {}", compat::panic_message(&e)));
            }
        }
        return false;
    }

    /// java: private static void extractResourcePack(Path packDirectory, InputStream packInputStream) throws Exception
    ///
    /// `packInputStream` is the downloaded archive's bytes: `new ZipInputStream(packInputStream)` walks its entries in
    /// archive order, as `lwjgl::inflate::zip_entries` answers them (an exception is a panic).
    pub fn extract_resource_pack(pack_directory: &std::path::PathBuf, pack_input_stream: &[u8]) {
        //   java: ZipInputStream zipInputStream = new ZipInputStream(packInputStream);
        //   java: while ((entry = zipInputStream.getNextEntry()) != null) {
        for (entry_name, entry_bytes) in mcrust::lwjgl::inflate::zip_entries(pack_input_stream) {
            //   java: Path outputPath = packDirectory.resolve(entry.getName()).normalize();
            let mut output_path: std::path::PathBuf = normalize(&pack_directory.join(&entry_name));
            if !(output_path.starts_with(pack_directory)) {
                panic!("IllegalStateException: Malicious ZIP entry detected: {}", entry_name);
            }
            //   java: if (entry.isDirectory()) - `ZipEntry.isDirectory`: the name ends with '/'
            if entry_name.ends_with('/') {
                std::fs::create_dir_all(&output_path).expect("IOException");
            } else {
                std::fs::create_dir_all(&output_path.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
                //   java: Files.copy(zipInputStream, outputPath, StandardCopyOption.REPLACE_EXISTING);
                std::fs::write(&output_path, &entry_bytes).expect("IOException");
            }
            //   java: zipInputStream.closeEntry();
        }
    }

}

/// `Path.normalize()`: `.` dropped and `..` folded into its parent (lexically, as Java's).
fn normalize(path: &std::path::Path) -> std::path::PathBuf {
    let mut out = std::path::PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// `Files.setLastModifiedTime(path, FileTime.fromMillis(millis))` (the path is the pack's directory).
fn set_last_modified_time(path: &std::path::Path, millis: i64) {
    let time = if millis >= 0 { std::time::UNIX_EPOCH + std::time::Duration::from_millis(millis as u64) } else { std::time::UNIX_EPOCH - std::time::Duration::from_millis(millis.unsigned_abs()) };
    let mut options = std::fs::OpenOptions::new();
    options.write(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // FILE_FLAG_BACKUP_SEMANTICS: what opening a directory needs on Windows
        options.custom_flags(0x0200_0000);
    }
    let file = options.open(path).unwrap_or_else(|e| panic!("IOException: {e}"));
    file.set_modified(time).unwrap_or_else(|e| panic!("IOException: {e}"));
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
