//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomHelmetTextures by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;

/// java: public static final List<NamedTexture> TEXTURES = new ArrayList<>();
pub static TEXTURES: LazyLock<Mutex<Vec<NamedTexture>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: public static final Object2ObjectOpenHashMap<String, ResolvableProfile> PROFILE_CACHE = new Object2ObjectOpenHashMap<>();
pub static PROFILE_CACHE: compat::MainThread<std::collections::HashMap<String, ResolvableProfile>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Pattern LEVEL_PATTERN = Pattern.compile("\\[Lvl[^\\]]*\\]");
pub static LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[Lvl[^\\]]*\\]"));

/// java: public class CustomHelmetTextures
#[derive(Clone)]
pub struct CustomHelmetTextures {}

impl CustomHelmetTextures {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ItemRepository::run_async_after_import(Arc::new(move || { CustomHelmetTextures::load_textures(); }));
    }

    /// java: private static void loadTextures()
    pub fn load_textures() {
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if !(ItemRepository::files_imported()) {
                return;
            }
            compat::lock(&TEXTURES).clear();
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
            // TODO(fabric2seer): `is` is not a method of FlexibleItemStack
            //   java: ItemRepository.getItemsStream()
            //   java: 					.filter(stack -> stack.is(Items.PLAYER_HEAD))
            //   java: 					.forEach(stack -> {
            //   java: 						String texture = ItemUtils.getHeadTexture(stack);
            //   java: 						if (texture.isEmpty() || !seen.add(texture)) return;
            //   java: 						String name = cleanName(stack.get(DataComponents.CUSTOM_NAME).getString());
            //   java: 						TEXTURES.add(new NamedTexture(name, texture, stack.getNeuName()));
            //   java: 					});
            compat::lock(&TEXTURES).sort_by(|a, b| { let key = |__a0: &NamedTexture| __a0.internal_name(); compat::compare_to(&key(a), &key(b)).cmp(&0) });
            logger().info(&format!("[Skyblocker] Loaded and sorted {} helmet textures from repo", (compat::lock(&TEXTURES).len() as i32)));
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker] Failed to load helmet textures from repo", e);
        //   java: 		}
    }

    /// java: private static String cleanName(String name)
    pub fn clean_name(name: &str) -> String {
        return compat::trim(&LEVEL_PATTERN.matcher(name).replace_all(""));
    }

    /// java: public static List<NamedTexture> getTextures()
    pub fn get_textures() -> Vec<NamedTexture> {
        return compat::get_state(&TEXTURES);
    }

    /// java: public static ResolvableProfile getProfile(String texture)
    pub fn get_profile(mut texture: String) -> ResolvableProfile {
        return PROFILE_CACHE.get_mut().entry(texture.clone()).or_insert_with_key(|k| (|t: String| ResolvableProfile::create_resolved(GameProfile::new(Uuid::name_uuid_from_bytes(&t.as_bytes().to_vec()).unwrap(), "custom", ItemUtils::property_map_with_texture(&t))).unwrap())(k.clone())).clone();
    }

}

/// java: public record NamedTexture(String name, String texture, String internalName)
#[derive(Clone, Debug, PartialEq)]
pub struct NamedTexture {
    /// java: String name
    pub name: String,
    /// java: String texture
    pub texture: String,
    /// java: String internalName
    pub internal_name: String,
}

impl NamedTexture {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut texture: String, mut internal_name: String) -> Self {
        Self { name, texture, internal_name }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `texture()`
    pub fn texture(&self) -> String {
        self.texture.clone()
    }

    /// java: the record accessor `internalName()`
    pub fn internal_name(&self) -> String {
        self.internal_name.clone()
    }

}
