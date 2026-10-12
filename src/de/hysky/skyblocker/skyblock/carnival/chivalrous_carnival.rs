//! Ported from de.hysky.skyblocker.skyblock.carnival.ChivalrousCarnival by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::area::Hub;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::mayor::mayor_utils::MayorUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;

/// java: private static boolean isCarnivalActive = false;
pub static IS_CARNIVAL_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class ChivalrousCarnival
#[derive(Clone)]
pub struct ChivalrousCarnival {}

impl ChivalrousCarnival {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::MAYOR_CHANGE.register(Arc::new(move || { compat::set_state(&IS_CARNIVAL_ACTIVE, MayorUtils::get_active_perks().into_iter().any(|perk| (perk == "Chivalrous Carnival"))); }));
    }

    /// java: public static boolean isCarnivalActive()
    pub fn is_carnival_active() -> bool {
        return compat::get_state(&IS_CARNIVAL_ACTIVE);
    }

    /// java: protected static boolean isInCarnival()
    pub fn is_in_carnival() -> bool {
        return (ChivalrousCarnival::is_carnival_active() && Utils::is_on_skyblock() && Utils::get_location() == Location::Hub && compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Carnival)));
    }

}
