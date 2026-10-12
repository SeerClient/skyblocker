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

// TODO(fabric2seer): static call CacheBuilder.newBuilder(..)
//   java: private static final LoadingCache<CacheKey, Map<String, Integer>> LEADERBOARDS_CACHE = CacheBuilder.newBuilder()
//   java: 			.expireAfterWrite(Duration.ofMinutes(5L))
//   java: 			.build(new CacheLoader<>() {
//   java: 				@Override
//   java: 				public Map<String, Integer> load(CacheKey key) throws Exception {
//   java: 					return fetchPlayerLeaderboardsInternal(key);
//   java: 				}
//   java: 			});
/// java: private static final LoadingCache<CacheKey, Map<String, Integer>> LEADERBOARDS_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(5L)) .build(new CacheLoader<>() { @Override public
pub static LEADERBOARDS_CACHE: compat::MainThread<LoadingCache<CacheKey, std::collections::HashMap<String, i32>, compat::Object>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LEADERBOARDS_CACHE` - see the TODO above"));
/// java: public static final int NO_POSITION = -1;
pub const NO_POSITION: i32 = -1;
/// java: private static Map<String, String> collectionLeaderboards = Map.of();
pub static COLLECTION_LEADERBOARDS: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
/// java: private static final Codec<LeaderboardInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.optionalFieldOf("collection").forGetter(LeaderboardInfo::collection), Codec.STRIN
pub static LEADERBOARD_INFO_CODEC: compat::MainThread<CodecRef<LeaderboardInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &LeaderboardInfo| o.collection(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "collection")), record_codec_builder::of(|o: &LeaderboardInfo| o.interval_type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "intervalType")), move |__a0, __a1| LeaderboardInfo::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<Map<String, LeaderboardInfo>> MAP_CODEC = Codec.unboundedMap(Codec.STRING, CODEC);
pub static LEADERBOARD_INFO_MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, LeaderboardInfo>>> = compat::MainThread::new(|| codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), LEADERBOARD_INFO_CODEC.get_mut().clone()));

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
        // TODO(fabric2seer): `CompletableFuture.thenAccept` has no Rust method in the port (looked for `then_accept` in CompletableFuture and JavaSrc)
        // rust: (CompletableFuture::supply_async(move || Ok((|| EliteLeaderboards::fetch_leaderboards_list())()), compat::executor)).then_accept(&Arc::new(move |__a0| EliteLeaderboards::collect_collections_leaderboards(Some(__a0.clone()))));
    }

    /// java: public static Map<String, String> getCollectionLeaderboardMappings()
    pub fn get_collection_leaderboard_mappings() -> std::collections::HashMap<String, String> {
        return compat::get_state(&COLLECTION_LEADERBOARDS);
    }

    /// java: private static @Nullable Map<String, LeaderboardInfo> fetchLeaderboardsList()
    pub fn fetch_leaderboards_list() -> Option<std::collections::HashMap<String, LeaderboardInfo>> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String response = Http.sendGetRequest("https://api.eliteskyblock.com/leaderboards");
            let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
            let mut leaderboards_object: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))), "leaderboards");
            let mut leaderboards: std::collections::HashMap<String, LeaderboardInfo> = (LEADERBOARD_INFO_MAP_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&leaderboards_object))).get_or_throw().unwrap();
            return Some(leaderboards);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load Elite leaderboards."));
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
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
        return CompletableFuture::supply_async(move || Ok((|| LEADERBOARDS_CACHE.get_mut().get_unchecked(&CacheKey::new(uuid.to_string(), profile_id.clone())))()), compat::executor);
    }

    /// java: private static Map<String, Integer> fetchPlayerLeaderboardsInternal(CacheKey key)
    pub fn fetch_player_leaderboards_internal(key: &CacheKey) -> std::collections::HashMap<String, i32> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut url: String = format!("https://api.eliteskyblock.com/leaderboards/{}/{}", key.uuid(), key.profile_id());
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String response = Http.sendGetRequest(url);
            let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
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
        todo!("fabric2seer: the value of this function is in a TODO above")
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
#[derive(Clone, Debug, PartialEq)]
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
