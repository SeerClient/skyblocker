//! Ported from de.hysky.skyblocker.utils.RegistryUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::core::holder_lookup::Provider;

// TODO(fabric2seer): static `VanillaRegistries.createLookup` has no Rust counterpart in the port
//   java: private static final HolderLookup.Provider LOOKUP = VanillaRegistries.createLookup();
/// java: private static final HolderLookup.Provider LOOKUP = VanillaRegistries.createLookup();
pub static LOOKUP: compat::MainThread<Arc<dyn Provider>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOOKUP` - see the TODO above"));

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
        let mut client: Option<mcrust::net::minecraft::client::minecraft::Minecraft> = Some(compat::minecraft());
        return (if client.is_some() && minecraft::get_connection(client.as_ref().unwrap()).is_some() && true /* java: minecraft::get_connection(client.as_ref().unwrap()).unwrap().registry_access() != null: never null here */ { minecraft::get_connection(client.as_ref().unwrap()).unwrap().registry_access().clone() } else { LOOKUP.get_mut().clone() });
    }

    /// java: public static HolderLookup.@Nullable Provider getCurrentRegistryWrapperLookup()
    pub fn get_current_registry_wrapper_lookup() -> Option<Arc<dyn Provider>> {
        let mut client: Option<mcrust::net::minecraft::client::minecraft::Minecraft> = Some(compat::minecraft());
        return (if client.is_some() && minecraft::get_connection(client.as_ref().unwrap()).is_some() { Some(minecraft::get_connection(client.as_ref().unwrap()).unwrap().registry_access().clone()) } else { None });
    }

}
