//! Ported from de.hysky.skyblocker.events.SkyblockEvents by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::area::Area;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::purse::purse_change_cause::PurseChangeCause;
use mcrust::mcrust::modding::events::Event;

/// java: public static final Event<SkyblockJoin> JOIN = EventFactory.createArrayBacked(SkyblockJoin.class, callbacks -> () -> { for (SkyblockEvents.SkyblockJoin callback : callbacks) { callback.onSkyblockJoin(
pub static JOIN: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `JOIN` (`JOIN.invoker()`): the factory's lambda over the listeners
pub fn join_invoker() -> SkyblockJoin {
    let callbacks: Vec<SkyblockJoin> = JOIN.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}
/// java: public static final Event<SkyblockLeave> LEAVE = EventFactory.createArrayBacked(SkyblockLeave.class, callbacks -> () -> { for (SkyblockLeave callback : callbacks) { callback.onSkyblockLeave(); } });
pub static LEAVE: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `LEAVE` (`LEAVE.invoker()`): the factory's lambda over the listeners
pub fn leave_invoker() -> SkyblockLeave {
    let callbacks: Vec<SkyblockLeave> = LEAVE.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}
/// java: public static final Event<SkyblockLocationChange> LOCATION_CHANGE = EventFactory.createArrayBacked(SkyblockLocationChange.class, callbacks -> location -> { for (SkyblockLocationChange callback : callb
pub static LOCATION_CHANGE: compat::MainThread<Event<dyn Fn(Location)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `LOCATION_CHANGE` (`LOCATION_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn location_change_invoker() -> SkyblockLocationChange {
    let callbacks: Vec<SkyblockLocationChange> = LOCATION_CHANGE.listeners();
    Arc::new(move |location| {
        for callback in callbacks.iter() {
            (callback)(location);
        }
    })
}
/// java: public static final Event<SkyblockAreaChange> AREA_CHANGE = EventFactory.createArrayBacked(SkyblockAreaChange.class, callbacks -> area -> { for (SkyblockAreaChange callback : callbacks) { callback.onS
pub static AREA_CHANGE: compat::MainThread<Event<dyn Fn(compat::Shared<dyn Area>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `AREA_CHANGE` (`AREA_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn area_change_invoker() -> SkyblockAreaChange {
    let callbacks: Vec<SkyblockAreaChange> = AREA_CHANGE.listeners();
    Arc::new(move |area| {
        for callback in callbacks.iter() {
            (callback)(area.clone());
        }
    })
}
/// java: public static final Event<ProfileChange> PROFILE_CHANGE = EventFactory.createArrayBacked(ProfileChange.class, callbacks -> (prev, profile) -> { for (ProfileChange callback : callbacks) { callback.onSk
pub static PROFILE_CHANGE: compat::MainThread<Event<dyn Fn(String, String)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `PROFILE_CHANGE` (`PROFILE_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn profile_change_invoker() -> ProfileChange {
    let callbacks: Vec<ProfileChange> = PROFILE_CHANGE.listeners();
    Arc::new(move |prev, profile| {
        for callback in callbacks.iter() {
            (callback)(prev.clone(), profile.clone());
        }
    })
}
/// java: public static final Event<PurseChange> PURSE_CHANGE = EventFactory.createArrayBacked(PurseChange.class, callbacks -> (diff, cause) -> { for (PurseChange callback : callbacks) { callback.onPurseChange(
pub static PURSE_CHANGE: compat::MainThread<Event<dyn Fn(f64, PurseChangeCause)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `PURSE_CHANGE` (`PURSE_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn purse_change_invoker() -> PurseChange {
    let callbacks: Vec<PurseChange> = PURSE_CHANGE.listeners();
    Arc::new(move |diff, cause| {
        for callback in callbacks.iter() {
            (callback)(diff, cause);
        }
    })
}
/// java: public static final Event<MayorChange> MAYOR_CHANGE = EventFactory.createArrayBacked(MayorChange.class, callbacks -> () -> { for (MayorChange callback : callbacks) { callback.onMayorChange(); } });
pub static MAYOR_CHANGE: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `MAYOR_CHANGE` (`MAYOR_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn mayor_change_invoker() -> MayorChange {
    let callbacks: Vec<MayorChange> = MAYOR_CHANGE.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}

/// java: @Environment(EnvType.CLIENT) public final class SkyblockEvents
#[derive(Clone)]
pub struct SkyblockEvents {}

impl SkyblockEvents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SkyblockJoin
/// a functional interface: `onSkyblockJoin()` - a lambda / method reference is this closure type
pub type SkyblockJoin = Arc<dyn Fn()>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SkyblockLeave
/// a functional interface: `onSkyblockLeave()` - a lambda / method reference is this closure type
pub type SkyblockLeave = Arc<dyn Fn()>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SkyblockLocationChange
/// a functional interface: `onSkyblockLocationChange(Location location)` - a lambda / method reference is this closure type
pub type SkyblockLocationChange = Arc<dyn Fn(Location)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SkyblockAreaChange
/// a functional interface: `onSkyblockAreaChange(Area area)` - a lambda / method reference is this closure type
pub type SkyblockAreaChange = Arc<dyn Fn(compat::Shared<dyn Area>)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface ProfileChange
/// a functional interface: `onSkyblockProfileChange(String prevProfileId, String profileId)` - a lambda / method reference is this closure type
pub type ProfileChange = Arc<dyn Fn(String, String)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface PurseChange
/// a functional interface: `onPurseChange(double diff, PurseChangeCause cause)` - a lambda / method reference is this closure type
pub type PurseChange = Arc<dyn Fn(f64, PurseChangeCause)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface MayorChange
/// a functional interface: `onMayorChange()` - a lambda / method reference is this closure type
pub type MayorChange = Arc<dyn Fn()>;
