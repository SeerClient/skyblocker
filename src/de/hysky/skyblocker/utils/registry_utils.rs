//! Ported from de.hysky.skyblocker.utils.RegistryUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::core::holder_lookup::Provider;

/// java: private static final HolderLookup.Provider LOOKUP = VanillaRegistries.createLookup();
///
/// The game's port of `VanillaRegistries.createLookup()` (`RegistrySetBuilder.build` over
/// `RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY)`): the static registries plus the datapack
/// registries whose vanilla bootstrap the game carries - enchantment, enchantment_provider, damage_type, trim_material,
/// trim_pattern, banner_pattern, jukebox_song, instrument, chat_type.
pub static LOOKUP: compat::MainThread<Arc<dyn Provider>> =
    compat::MainThread::new(mcrust::net::minecraft::data::registries::vanilla_registries::create_world_lookup);

/// java: public final class RegistryUtils
#[derive(Clone)]
pub struct RegistryUtils {}

impl RegistryUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static HolderLookup.Provider getRegistryWrapperLookup()
    pub fn get_registry_wrapper_lookup() -> Arc<dyn Provider> {
        // java: client != null && client.getConnection() != null && client.getConnection().registryAccess() != null
        //   ? client.getConnection().registryAccess() : LOOKUP
        return match RegistryUtils::connection_registry_access() {
            Some(registry_access) => registry_access,
            None => LOOKUP.get_mut().clone(),
        };
    }

    /// java: public static HolderLookup.@Nullable Provider getCurrentRegistryWrapperLookup()
    pub fn get_current_registry_wrapper_lookup() -> Option<Arc<dyn Provider>> {
        // java: client != null && client.getConnection() != null ? client.getConnection().registryAccess() : null
        return RegistryUtils::connection_registry_access();
    }

    /// `Minecraft.getInstance().getConnection().registryAccess()`, or null without a connection: the port's
    /// connection keeps it in `PlayListenerSinks.registry_access` (`None` before the registry sync).
    fn connection_registry_access() -> Option<Arc<dyn Provider>> {
        let client = compat::minecraft();
        let connection = compat::get_connection(client)?;
        let registry_access = connection.registry_access.lock().unwrap_or_else(|p| p.into_inner()).clone()?;
        Some(Arc::new(registry_access) as Arc<dyn Provider>)
    }

}
