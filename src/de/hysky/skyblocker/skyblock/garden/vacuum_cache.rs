//! Ported from de.hysky.skyblocker.skyblock.garden.VacuumCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("vacuum_cache.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("vacuum_cache.json"));
/// java: private static final ProfiledData<String> CACHED_VINYL = new ProfiledData<>(FILE, Codec.STRING);
pub static CACHED_VINYL: compat::MainThread<compat::Shared<ProfiledData<String>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), mcrust::com::mojang::serialization::codec::string_codec()));

thread_local! {
    /// The open "Stereo Harmony" screen registered `ScreenEvents.remove(screen).register(_ -> VacuumCache.update(containerScreen))`.
    static REMOVE_REGISTERED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

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
        // java: ScreenEvents.remove(screen).register(..) - fired as the screen it was registered for is removed
        // (`compat::batch_s_on_container_screen_remove`: the screen's `onClose`, with its menu's stacks)
        compat::batch_s_on_container_screen_remove(|stacks| {
            if REMOVE_REGISTERED.with(|r| r.replace(false)) {
                VacuumCache::update(stacks);
            }
        });
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // a new screen (or a re-init) has registered no per-screen listener yet
            REMOVE_REGISTERED.with(|r| r.set(false));
            // java: if (!Utils.isOnSkyblock() || !(screen instanceof ContainerScreen containerScreen)) { return; }
            // (`instanceof ContainerScreen`: the open screen's kind - batch G pattern 2)
            if !Utils::is_on_skyblock() || !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                return;
            }
            // java: if (!containerScreen.getTitle().getString().equals("Stereo Harmony")) { return; }
            if screen.get_title().to_string() != "Stereo Harmony" {
                return;
            }
            // java: ScreenEvents.remove(screen).register(_ -> VacuumCache.update(containerScreen));
            REMOVE_REGISTERED.with(|r| r.set(true));
        }));
    }

    /// java: private static void update(ContainerScreen containerScreen)
    /// (`stacks` is `containerScreen.getMenu().slots`' items, in slot order)
    pub fn update(stacks: &[mcrust::net::minecraft::world::item::item_stack::ItemStack]) {
        // java: for (Slot slot : containerScreen.getMenu().slots) {
        for stack in stacks {
            // java: ItemStack stack = slot.getItem();
            // java: if (!stack.isEmpty() && ItemUtils.getLoreLineIf(stack, line -> line.equals("Click to stop playing!")) != null) {
            if !stack.is_empty() && ItemUtils::get_lore_line_if(stack, Arc::new(|line: String| line == "Click to stop playing!")).is_some() {
                VacuumCache::set_vinyl(stack.get_skyblock_id());
                return;
            }
        }
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
