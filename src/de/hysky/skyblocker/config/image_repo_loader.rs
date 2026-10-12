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
        CompletableFuture::<()>::run_async(move || (|| {
            if retries < 3 {
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut start: i64 = compat::millis();
                    let mut saved_commit_hash: String = ImageRepoLoader::check_saved_commit_data();
                    // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                    //   java: JsonObject response = JsonParser.parseString(Http.sendGetRequest(BRANCH_INFO)).getAsJsonObject();
                    let mut response: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `response` (TODO above)");
                    let mut latest_commit_hash: String = compat::json::get_as_string(&(compat::json::get_as_json_object_of(&response, "commit")).get("sha").cloned().unwrap());
                    if !(saved_commit_hash == latest_commit_hash) {
                        // TODO(fabric2seer): `Http.downloadContent` is not a static method of Http
                        //   java: InputStream in = Http.downloadContent(REPO_DOWNLOAD);
                        let mut r#in: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `in` (TODO above)");
                        if REPO_DIRECTORY.get_mut().exists() {
                            ImageRepoLoader::delete_directories();
                        }
                        {
                            // TODO(fabric2seer): `new ZipInputStream(..)`
                            //   java: ZipInputStream zis = new ZipInputStream(in);
                            let mut zis: crate::unresolved::ZipInputStream = todo!("fabric2seer: the initialiser of `zis` (TODO above)");
                            let mut entry: crate::unresolved::ZipEntry;
                            // TODO(fabric2seer): the condition: `.getNextEntry(..)` on a ZipInputStream
                            //   java: (entry = zis.getNextEntry()) != null
                            while todo!("fabric2seer: the condition (TODO above)") {
                                // TODO(fabric2seer): `.getName(..)` on a ZipEntry
                                //   java: Path outputFile = REPO_DIRECTORY.resolve(entry.getName());
                                let mut output_file: std::path::PathBuf = todo!("fabric2seer: the initialiser of `outputFile` (TODO above)");
                                // TODO(fabric2seer): the condition: `.isDirectory(..)` on a ZipEntry
                                //   java: entry.isDirectory(..)
                                if todo!("fabric2seer: the condition (TODO above)") {
                                    std::fs::create_dir_all(&output_file).expect("IOException");
                                } else {
                                    std::fs::create_dir_all(&output_file.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
                                    // TODO(fabric2seer): static field StandardCopyOption.REPLACE_EXISTING
                                    //   java: Files.copy(zis, outputFile, StandardCopyOption.REPLACE_EXISTING);
                                }
                            }
                        }
                        ImageRepoLoader::write_commit_data(&latest_commit_hash);
                        let mut end: i64 = compat::millis();
                        logger().info(&format!("[Skyblocker] Successfully updated the Image Respository in {} ms! {} → {}", end.wrapping_sub(start), saved_commit_hash, latest_commit_hash));
                    } else {
                        logger().info("[Skyblocker] The Image Respository is up to date!");
                    }
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker] Error while downloading image repo on attempt {}!", retries));
                    ImageRepoLoader::update(retries.wrapping_add(1));
                }
            }
        })(), compat::executor);
    }

    /// java: private static String checkSavedCommitData() throws IOException
    pub fn check_saved_commit_data() -> String {
        let mut file: std::path::PathBuf = REPO_DIRECTORY.get_mut().join("image_repo.json");
        if file.exists() {
            {
                // TODO(fabric2seer): Files.newBufferedReader
                //   java: BufferedReader reader = Files.newBufferedReader(..);
                let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // TODO(fabric2seer): static call JsonParser.parseReader(..)
                //   java: CommitData commitData = CommitData.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
                let mut commit_data: CommitData = todo!("fabric2seer: the initialiser of `commitData` (TODO above)");
                return commit_data.commit();
            }
        }
        return PLACEHOLDER_HASH.to_string();
    }

    /// java: private static void writeCommitData(String newHash) throws IOException
    pub fn write_commit_data(new_hash: &str) {
        let mut file: std::path::PathBuf = REPO_DIRECTORY.get_mut().join("image_repo.json");
        let mut commit_data: CommitData = CommitData::new(new_hash.to_string(), compat::millis());
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // TODO(fabric2seer): `.toJson(..)` on a Gson
            //   java: SkyblockerMod.GSON.toJson(CommitData.CODEC.encodeStart(JsonOps.INSTANCE, commitData).getOrThrow(), writer);
        }
    }

    /// java: private static void deleteDirectories() throws IOException
    pub fn delete_directories() {
        for dir in std::fs::read_dir(REPO_DIRECTORY.get_mut()).expect("IOException").filter_map(|e| e.ok()).map(|e| e.path()).filter(|__a0| __a0.is_dir()) {
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
