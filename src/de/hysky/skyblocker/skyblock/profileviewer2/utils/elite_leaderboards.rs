//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.EliteLeaderboards by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer;
use mcrust::com::google::common::cache::loading_cache::LoadingCache;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::java::util::uuid::Uuid;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final LoadingCache<CacheKey, Map<String, Integer>> LEADERBOARDS_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(5L)) .build(new CacheLoader<>() { @Override public
/// (read from `fetchPlayerLeaderboards`' worker: a `LazyLock`, the cache being thread-safe as Guava's is)
pub static LEADERBOARDS_CACHE: LazyLock<LoadingCache<CacheKey, std::collections::HashMap<String, i32>, LeaderboardsLoader>> = LazyLock::new(|| {
    mcrust::com::google::common::cache::loading_cache::CacheBuilder::new_builder()
        .expire_after_write(std::time::Duration::from_secs(5 * 60))
        .build(LeaderboardsLoader)
});

/// java: the anonymous `new CacheLoader<>() { load(key) -> fetchPlayerLeaderboardsInternal(key) }` of `LEADERBOARDS_CACHE`
pub struct LeaderboardsLoader;

impl mcrust::com::google::common::cache::cache_loader::CacheLoader<CacheKey, std::collections::HashMap<String, i32>> for LeaderboardsLoader {
    fn load(&self, key: &CacheKey) -> std::collections::HashMap<String, i32> {
        EliteLeaderboards::fetch_player_leaderboards_internal(key)
    }
}
/// java: public static final int NO_POSITION = -1;
pub const NO_POSITION: i32 = -1;
/// java: private static Map<String, String> collectionLeaderboards = Map.of();
pub static COLLECTION_LEADERBOARDS: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
/// java: private static final Codec<LeaderboardInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.optionalFieldOf("collection").forGetter(LeaderboardInfo::collection), Codec.STRIN
pub static LEADERBOARD_INFO_CODEC: compat::MainThread<CodecRef<LeaderboardInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &LeaderboardInfo| o.collection(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "collection")), record_codec_builder::of(|o: &LeaderboardInfo| o.interval_type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "intervalType")), move |__a0, __a1| LeaderboardInfo::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<Map<String, LeaderboardInfo>> MAP_CODEC = Codec.unboundedMap(Codec.STRING, CODEC);
pub static LEADERBOARD_INFO_MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, LeaderboardInfo>>> = compat::MainThread::new(|| codec::xmap(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), LEADERBOARD_INFO_CODEC.get().clone()), |v: Vec<(String, LeaderboardInfo)>| v.into_iter().collect::<std::collections::HashMap<_, _>>(), |m: &std::collections::HashMap<String, LeaderboardInfo>| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()));

/// java: public class EliteLeaderboards
#[derive(Clone)]
pub struct EliteLeaderboards {}

impl EliteLeaderboards {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if !(*profile_viewer::ENABLED) {
            return;
        }
        // java: CompletableFuture.supplyAsync(EliteLeaderboards::fetchLeaderboardsList).thenAccept(EliteLeaderboards::collectCollectionsLeaderboards);
        // (`thenAccept`: on the thread that completes the future - an executor that runs the stage where it is handed)
        (CompletableFuture::supply_async(move || Ok(EliteLeaderboards::fetch_leaderboards_list()), compat::executor)).then_accept_async(|__a0: &Option<std::collections::HashMap<String, LeaderboardInfo>>| EliteLeaderboards::collect_collections_leaderboards(__a0.clone()), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| __t()));
    }

    /// java: public static Map<String, String> getCollectionLeaderboardMappings()
    pub fn get_collection_leaderboard_mappings() -> std::collections::HashMap<String, String> {
        return compat::get_state(&COLLECTION_LEADERBOARDS);
    }

    /// java: private static @Nullable Map<String, LeaderboardInfo> fetchLeaderboardsList()
    pub fn fetch_leaderboards_list() -> Option<std::collections::HashMap<String, LeaderboardInfo>> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://api.eliteskyblock.com/leaderboards");
            let mut leaderboards_object: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))), "leaderboards");
            let mut leaderboards: std::collections::HashMap<String, LeaderboardInfo> = (LEADERBOARD_INFO_MAP_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::Value::Object(leaderboards_object)))).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
            return Some(leaderboards);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load Elite leaderboards."));
                return None;
            }
        }
    }

    /// java: private static void collectCollectionsLeaderboards(@Nullable Map<String, LeaderboardInfo> leaderboards)
    pub fn collect_collections_leaderboards(mut leaderboards: Option<std::collections::HashMap<String, LeaderboardInfo>>) {
        let Some(mut leaderboards) = leaderboards.clone() else {
            return;
        };
        let mut collection_leaderboard_mapping: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for entry in leaderboards.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut id: String = entry.0.clone();
            let info = &entry.1;
            if info.interval_type().to_lowercase() == "current" && info.collection().is_some() {
                collection_leaderboard_mapping.insert(info.collection().unwrap(), id.clone());
            }
        }
        compat::set_state(&COLLECTION_LEADERBOARDS, collection_leaderboard_mapping.clone());
    }

    /// java: public static CompletableFuture<Map<String, Integer>> fetchPlayerLeaderboards(String uuid, UUID profileId)
    pub fn fetch_player_leaderboards(uuid: &str, profile_id: &Uuid) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<std::collections::HashMap<String, i32>> {
        let uuid: String = uuid.to_string();
        let profile_id: Uuid = profile_id.clone();
        return CompletableFuture::supply_async(move || Ok(LEADERBOARDS_CACHE.get_unchecked(&CacheKey::new(uuid, profile_id))), compat::executor);
    }

    /// java: private static Map<String, Integer> fetchPlayerLeaderboardsInternal(CacheKey key)
    pub fn fetch_player_leaderboards_internal(key: &CacheKey) -> std::collections::HashMap<String, i32> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut url: String = format!("https://api.eliteskyblock.com/leaderboards/{}/{}", key.uuid(), key.profile_id());
            let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request(&url);
            let mut data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))), "ranks");
            let mut positions: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
            for entry in data.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut id: String = entry.0.clone();
                if !(compat::json::is_null(&entry.1)) {
                    let mut position: i32 = compat::json::get_as_int(&compat::json::get_as_json_object(&entry.1).get("rank").cloned().unwrap());
                    positions.insert(id.clone(), position);
                }
            }
            return positions.clone();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load Elite leaderboard data for player {} with profile {}.", key.uuid(), compat::jstring(&key.profile_id())));
                return std::collections::HashMap::from([]);
            }
        }
    }

}

/// java: private record LeaderboardInfo(Optional<String> collection, String intervalType)
#[derive(Clone, Debug, PartialEq)]
pub struct LeaderboardInfo {
    /// java: Optional<String> collection
    pub collection: Option<String>,
    /// java: String intervalType
    pub interval_type: String,
}

impl LeaderboardInfo {
    /// java: the canonical record constructor
    pub fn new(mut collection: Option<String>, mut interval_type: String) -> Self {
        Self { collection, interval_type }
    }

    /// java: the record accessor `collection()`
    pub fn collection(&self) -> Option<String> {
        self.collection.clone()
    }

    /// java: the record accessor `intervalType()`
    pub fn interval_type(&self) -> String {
        self.interval_type.clone()
    }

}

/// java: private record CacheKey(String uuid, UUID profileId)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// java: String uuid
    pub uuid: String,
    /// java: UUID profileId
    pub profile_id: Uuid,
}

impl CacheKey {
    /// java: the canonical record constructor
    pub fn new(mut uuid: String, mut profile_id: Uuid) -> Self {
        Self { uuid, profile_id }
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }

    /// java: the record accessor `profileId()`
    pub fn profile_id(&self) -> Uuid {
        self.profile_id.clone()
    }

}
