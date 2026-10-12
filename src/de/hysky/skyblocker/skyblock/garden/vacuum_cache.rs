//! Ported from de.hysky.skyblocker.skyblock.garden.VacuumCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("vacuum_cache.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("vacuum_cache.json"));
/// java: private static final ProfiledData<String> CACHED_VINYL = new ProfiledData<>(FILE, Codec.STRING);
pub static CACHED_VINYL: compat::MainThread<compat::Shared<ProfiledData<String>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), mcrust::com::mojang::serialization::codec::string_codec()));

/// java: public class VacuumCache
#[derive(Clone)]
pub struct VacuumCache {}

impl VacuumCache {
    /// java: private VacuumCache()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CACHED_VINYL.get_mut().lock().borrow_mut().load();
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!Utils.isOnSkyblock() || !(screen instanceof ContainerScreen containerScreen)) {
            //   java: 				return;
            //   java: 			}
            // TODO(fabric2seer): the condition: unknown name `containerScreen`
            //   java: !containerScreen.getTitle().getString().equals("Stereo Harmony")
            if todo!("fabric2seer: the condition (TODO above)") {
                return;
            }
            // TODO(fabric2seer): ScreenEvents.remove(..).register: no SeerClient equivalent yet
            //   java: ScreenEvents.remove(screen).register(_ -> VacuumCache.update(containerScreen));
        }));
    }

    /// java: private static void update(ContainerScreen containerScreen)
    pub fn update(container_screen: &ContainerScreen) {
        // TODO(fabric2seer): field `slots` of a value of unknown type
        //   java: for (Slot slot : containerScreen.getMenu().slots) {
        //   java: 			ItemStack stack = slot.getItem();
        //   java:
        //   java: 			if (!stack.isEmpty() && ItemUtils.getLoreLineIf(stack, line -> line.equals("Click to stop playing!")) != null) {
        //   java: 				setVinyl(stack.getSkyblockId());
        //   java:
        //   java: 				return;
        //   java: 			}
        //   java: 		}
        VacuumCache::set_vinyl("".to_string());
    }

    /// java: private static void setVinyl(String skyblockId)
    pub fn set_vinyl(mut skyblock_id: String) {
        if Utils::get_profile_id().is_empty() {
            return;
        }
        if skyblock_id.is_empty() {
            ProfiledData::remove(CACHED_VINYL.get_mut());
        } else {
            ProfiledData::put(CACHED_VINYL.get_mut(), skyblock_id.clone());
        }
        CACHED_VINYL.get_mut().lock().borrow().save();
    }

    /// java: public static String getVinyl()
    pub fn get_vinyl() -> String {
        return (if ProfiledData::contains_key(CACHED_VINYL.get_mut()) { ProfiledData::get(CACHED_VINYL.get_mut()) } else { Some("".to_string()) }).unwrap();
    }

}
