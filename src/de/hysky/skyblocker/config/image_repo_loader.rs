//! Ported from de.hysky.skyblocker.config.ImageRepoLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::file_utils::FileUtils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: static final Path REPO_DIRECTORY = SkyblockerMod.CONFIG_DIR.resolve("image-repo");
pub static REPO_DIRECTORY: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("image-repo"));
/// java: private static final String BRANCH_INFO = "https://api.github.com/repos/SkyblockerMod/Skyblocker-Assets/branches/images";
pub const BRANCH_INFO: &str = "https://api.github.com/repos/SkyblockerMod/Skyblocker-Assets/branches/images";
/// java: private static final String REPO_DOWNLOAD = "https://github.com/SkyblockerMod/Skyblocker-Assets/archive/refs/heads/images.zip";
pub const REPO_DOWNLOAD: &str = "https://github.com/SkyblockerMod/Skyblocker-Assets/archive/refs/heads/images.zip";
/// java: private static final String PLACEHOLDER_HASH = "None!";
pub const PLACEHOLDER_HASH: &str = "None!";
/// java: static final Codec<CommitData> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("commit").forGetter(CommitData::commit), Codec.LONG.fieldOf("lastUpdated").forGetter(C
pub static COMMIT_DATA_CODEC: compat::MainThread<CodecRef<CommitData>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &CommitData| o.commit(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "commit")), record_codec_builder::of(|o: &CommitData| o.last_updated(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "lastUpdated")), move |__a0, __a1| CommitData::new(__a0.clone(), __a1.clone()))));

/// java: public class ImageRepoLoader
#[derive(Clone)]
pub struct ImageRepoLoader {}

impl ImageRepoLoader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void init()
    pub fn init() {
        ImageRepoLoader::update(0);
    }

    /// java: private static void update(int retries)
    pub fn update(mut retries: i32) {
        // The body runs on a worker (`CompletableFuture.runAsync`); the two render-thread statics it reads
        // (`compat::MainThread`) are taken here and handed in.
        let repo_directory: std::path::PathBuf = REPO_DIRECTORY.get().clone();
        let codec: CodecRef<CommitData> = COMMIT_DATA_CODEC.get().clone();
        ImageRepoLoader::update_with(retries, repo_directory, codec);
    }

    /// `update(retries)` with `REPO_DIRECTORY` and `CommitData.CODEC` already read (a retry runs on the worker).
    fn update_with(mut retries: i32, repo_directory: std::path::PathBuf, codec: CodecRef<CommitData>) {
        CompletableFuture::<()>::run_async(move || (|| {
            if retries < 3 {
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut start: i64 = compat::millis();
                    let mut saved_commit_hash: String = ImageRepoLoader::check_saved_commit_data_in(&repo_directory, &codec);
                    // java: JsonObject response = JsonParser.parseString(Http.sendGetRequest(BRANCH_INFO)).getAsJsonObject();
                    let __json: serde_json::Value = serde_json::from_str(&crate::de::hysky::skyblocker::utils::http::Http::send_get_request(BRANCH_INFO)).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    let mut response: serde_json::Map<String, serde_json::Value> = __json.as_object().cloned().unwrap_or_else(|| panic!("IllegalStateException: Not a JSON Object: {__json}"));
                    let mut latest_commit_hash: String = compat::json::get_as_string(&(compat::json::get_as_json_object_of(&response, "commit")).get("sha").cloned().unwrap());
                    if !(saved_commit_hash == latest_commit_hash) {
                        // java: InputStream in = Http.downloadContent(REPO_DOWNLOAD);
                        let r#in: Vec<u8> = crate::de::hysky::skyblocker::utils::http::Http::download_content(REPO_DOWNLOAD);
                        if repo_directory.exists() {
                            ImageRepoLoader::delete_directories_in(&repo_directory);
                        }
                        {
                            // java: ZipInputStream zis = new ZipInputStream(in); while ((entry = zis.getNextEntry()) != null) {..}
                            // - the archive's LOCAL headers, front to back (`lwjgl::inflate::zip_index`, the walk a
                            // `ZipInputStream` makes)
                            for (name, loc) in mcrust::lwjgl::inflate::zip_index(&r#in) {
                                // java: Path outputFile = REPO_DIRECTORY.resolve(entry.getName());
                                let mut output_file: std::path::PathBuf = repo_directory.join(&name);
                                // java: entry.isDirectory() - a name ending in '/'
                                if name.ends_with('/') {
                                    std::fs::create_dir_all(&output_file).expect("IOException");
                                } else {
                                    std::fs::create_dir_all(&output_file.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
                                    // java: Files.copy(zis, outputFile, StandardCopyOption.REPLACE_EXISTING);
                                    let data: Vec<u8> = mcrust::lwjgl::inflate::zip_entry_data(&r#in, loc).unwrap_or_else(|| panic!("ZipException: invalid entry {name}"));
                                    std::fs::write(&output_file, data).expect("IOException");
                                }
                            }
                        }
                        ImageRepoLoader::write_commit_data_in(&repo_directory, &codec, &latest_commit_hash);
                        let mut end: i64 = compat::millis();
                        logger().info(&format!("[Skyblocker] Successfully updated the Image Respository in {} ms! {} → {}", end.wrapping_sub(start), saved_commit_hash, latest_commit_hash));
                    } else {
                        logger().info("[Skyblocker] The Image Respository is up to date!");
                    }
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker] Error while downloading image repo on attempt {}! {}", retries, compat::panic_message(&e)));
                    ImageRepoLoader::update_with(retries.wrapping_add(1), repo_directory.clone(), codec.clone());
                }
            }
        })(), compat::executor);
    }

    /// java: private static String checkSavedCommitData() throws IOException
    pub fn check_saved_commit_data() -> String {
        ImageRepoLoader::check_saved_commit_data_in(REPO_DIRECTORY.get(), COMMIT_DATA_CODEC.get())
    }

    fn check_saved_commit_data_in(repo_directory: &std::path::Path, codec: &CodecRef<CommitData>) -> String {
        use mcrust::com::mojang::serialization::decoder::Decoder as _;
        let mut file: std::path::PathBuf = repo_directory.join("image_repo.json");
        if file.exists() {
            {
                // java: BufferedReader reader = Files.newBufferedReader(file);
                let reader: String = std::fs::read_to_string(&file).expect("IOException");
                // java: CommitData commitData = CommitData.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
                let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                let mut commit_data: CommitData = codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
                return commit_data.commit();
            }
        }
        return PLACEHOLDER_HASH.to_string();
    }

    /// java: private static void writeCommitData(String newHash) throws IOException
    pub fn write_commit_data(new_hash: &str) {
        ImageRepoLoader::write_commit_data_in(REPO_DIRECTORY.get(), COMMIT_DATA_CODEC.get(), new_hash)
    }

    fn write_commit_data_in(repo_directory: &std::path::Path, codec: &CodecRef<CommitData>, new_hash: &str) {
        use mcrust::com::mojang::serialization::encoder::Encoder as _;
        let mut file: std::path::PathBuf = repo_directory.join("image_repo.json");
        let mut commit_data: CommitData = CommitData::new(new_hash.to_string(), compat::millis());
        {
            // java: BufferedWriter writer = Files.newBufferedWriter(file);
            // java: SkyblockerMod.GSON.toJson(CommitData.CODEC.encodeStart(JsonOps.INSTANCE, commitData).getOrThrow(), writer);
            // (`GSON` pretty-prints: `serde_json::to_string_pretty`)
            let encoded = codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &commit_data).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
            let writer: String = serde_json::to_string_pretty(&compat::json_from_port(&encoded)).expect("Gson.toJson");
            std::fs::write(&file, writer).expect("IOException");
        }
    }

    /// java: private static void deleteDirectories() throws IOException
    pub fn delete_directories() {
        ImageRepoLoader::delete_directories_in(REPO_DIRECTORY.get())
    }

    fn delete_directories_in(repo_directory: &std::path::Path) {
        for dir in std::fs::read_dir(repo_directory).expect("IOException").filter_map(|e| e.ok()).map(|e| e.path()).filter(|__a0| __a0.is_dir()) {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                FileUtils::recursive_delete(&dir);
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Encountered an exception while deleting a directory! Path: {}", std::path::absolute(&dir).unwrap_or_else(|_| dir.clone()).display()));
            }
        }
    }

}

/// java: record CommitData(String commit, long lastUpdated)
#[derive(Clone, Debug, PartialEq)]
pub struct CommitData {
    /// java: String commit
    pub commit: String,
    /// java: long lastUpdated
    pub last_updated: i64,
}

impl CommitData {
    /// java: the canonical record constructor
    pub fn new(mut commit: String, mut last_updated: i64) -> Self {
        Self { commit, last_updated }
    }

    /// java: the record accessor `commit()`
    pub fn commit(&self) -> String {
        self.commit.clone()
    }

    /// java: the record accessor `lastUpdated()`
    pub fn last_updated(&self) -> i64 {
        self.last_updated
    }

}
