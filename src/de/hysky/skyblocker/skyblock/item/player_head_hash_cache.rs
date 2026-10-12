//! Ported from de.hysky.skyblocker.skyblock.item.PlayerHeadHashCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use mcrust::java::net::uri::Uri;
use mcrust::java::util::base64;

/// java: private static final Set<String> MANUAL_CACHES = Set.of( HeadTextures.ADAPTIVE_BELT_HEALER, HeadTextures.ADAPTIVE_BELT_MAGE, HeadTextures.ADAPTIVE_BELT_BERSERK, HeadTextures.ADAPTIVE_BELT_ARCHER, Head
pub static MANUAL_CACHES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from([head_textures::ADAPTIVE_BELT_HEALER.to_string(), head_textures::ADAPTIVE_BELT_MAGE.to_string(), head_textures::ADAPTIVE_BELT_BERSERK.to_string(), head_textures::ADAPTIVE_BELT_ARCHER.to_string(), head_textures::ADAPTIVE_BELT_TANK.to_string()]));
// TODO(fabric2seer): static call IntSets.emptySet(..)
//   java: private static IntSet CACHE = IntSets.emptySet();
/// java: private static IntSet CACHE = IntSets.emptySet();
pub static CACHE: LazyLock<Mutex<std::collections::HashSet<i32>>> = LazyLock::new(|| Mutex::new(todo!("fabric2seer: the initialiser of `CACHE` - see the TODO above")));

/// java: public class PlayerHeadHashCache
#[derive(Clone)]
pub struct PlayerHeadHashCache {}

impl PlayerHeadHashCache {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void loadSkins(JsonArray items)
    pub fn load_skins(items: &Vec<serde_json::Value>) {
        let mut cache: std::collections::HashSet<i32> = std::collections::HashSet::new();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut api_item_textures = items.clone().into_iter().map(|__a0| compat::json::get_as_json_object(&__a0)).filter(|item| (compat::json::get_as_string(&item.get("material").cloned().unwrap()) == "SKULL_ITEM")).filter(|item| compat::json::has(&item, "skin")).map(|item| compat::json::get_as_string(&(compat::json::get_as_json_object_of(&item, "skin")).get("value").cloned().unwrap()));
            let mut override_textures = MANUAL_CACHES.iter().cloned();
            for __a0 in api_item_textures.chain(override_textures).map(|__a0| PlayerHeadHashCache::get_skin_hash_from_base64(&__a0)).filter(|hash| (true /* java: hash != null: never null here */ && !hash.is_empty())).map(|__a0| compat::string_hash_code(&__a0)) {
                cache.insert(__a0);
            }
            logger().info("[Skyblocker Player Head Hash Cache] Successfully cached the hashes of all player head items!");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Player Head Hash Cache] Failed to cache skin hashes!"));
        }
        compat::set_state(&CACHE, cache.clone());
    }

    /// java: public static String getSkinHashFromBase64(String base64)
    pub fn get_skin_hash_from_base64(base64: &str) -> String {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: String decoded = new String(Base64.getDecoder().decode(base64)); - `new String(bytes)` decodes UTF-8,
            //   a malformed sequence becoming U+FFFD
            let bytes: Vec<u8> = mcrust::java::util::base64::get_decoder().decode(base64).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
            let mut decoded: String = String::from_utf8_lossy(&bytes).into_owned();
            let mut profile: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&decoded).expect("JsonSyntaxException")));
            let mut url: String = compat::json::get_as_string(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&profile, "textures")), "SKIN")).get("url").cloned().unwrap());
            return PlayerHeadHashCache::get_skin_hash_from_url(&url);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Player Head Hash Cache] Error parsing head's base64 '{}'.", base64));
            }
        }
        return "".to_string();
    }

    /// java: public static String getSkinHashFromUrl(String url)
    pub fn get_skin_hash_from_url(url: &str) -> String {
        if true /* java: url != null: never null here */ && url == "ETF pre test, skin check" {
            return "".to_string();
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `new URI(..)`: the port has no `URI::new`
            // TODO(fabric2seer): `URI.getPath` has no Rust method in the port (looked for `get_path` in URI and JavaSrc)
            // TODO(fabric2seer): static `FilenameUtils.getBaseName` has no Rust counterpart in the port
            // rust: return FilenameUtils::get_base_name(Uri::new(url.to_string()).get_path());
            return todo!("fabric2seer: see the TODO above");
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Player Head Hash Cache] Malformed Skin URL! URL: {}", url));
            }
        }
        return "".to_string();
    }

    /// java: public static boolean contains(int hash)
    pub fn contains(mut hash: i32) -> bool {
        return compat::lock(&CACHE).contains(&hash);
    }

}
