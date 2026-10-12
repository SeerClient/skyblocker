//! Ported from de.hysky.skyblocker.events.DungeonEvents by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::SecretWaypoint;
use mcrust::mcrust::modding::events::Event;

/// java: public static final Event<DungeonLoaded> DUNGEON_LOADED = EventFactory.createArrayBacked(DungeonLoaded.class, callbacks -> () -> { for (DungeonLoaded callback : callbacks) { callback.onDungeonLoaded()
pub static DUNGEON_LOADED: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `DUNGEON_LOADED` (`DUNGEON_LOADED.invoker()`): the factory's lambda over the listeners
pub fn dungeon_loaded_invoker() -> DungeonLoaded {
    let callbacks: Vec<DungeonLoaded> = DUNGEON_LOADED.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}
/// java: public static final Event<DungeonStarted> DUNGEON_STARTED = EventFactory.createArrayBacked(DungeonStarted.class, callbacks -> () -> { for (DungeonStarted callback : callbacks) { callback.onDungeonStar
pub static DUNGEON_STARTED: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `DUNGEON_STARTED` (`DUNGEON_STARTED.invoker()`): the factory's lambda over the listeners
pub fn dungeon_started_invoker() -> DungeonStarted {
    let callbacks: Vec<DungeonStarted> = DUNGEON_STARTED.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}
/// java: public static final Event<DungeonEnded> DUNGEON_ENDED = EventFactory.createArrayBacked(DungeonEnded.class, callbacks -> () -> { for (DungeonEnded callback : callbacks) { callback.onDungeonEnded(); } }
pub static DUNGEON_ENDED: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `DUNGEON_ENDED` (`DUNGEON_ENDED.invoker()`): the factory's lambda over the listeners
pub fn dungeon_ended_invoker() -> DungeonEnded {
    let callbacks: Vec<DungeonEnded> = DUNGEON_ENDED.listeners();
    Arc::new(move || {
        for callback in callbacks.iter() {
            (callback)();
        }
    })
}
/// java: public static final Event<RoomMatched> PUZZLE_MATCHED = EventFactory.createArrayBacked(RoomMatched.class, callbacks -> room -> { for (RoomMatched callback : callbacks) { callback.onRoomMatched(room); 
pub static PUZZLE_MATCHED: compat::MainThread<Event<dyn Fn(compat::Shared<dyn Room>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `PUZZLE_MATCHED` (`PUZZLE_MATCHED.invoker()`): the factory's lambda over the listeners
pub fn puzzle_matched_invoker() -> RoomMatched {
    let callbacks: Vec<RoomMatched> = PUZZLE_MATCHED.listeners();
    Arc::new(move |room| {
        for callback in callbacks.iter() {
            (callback)(room.clone());
        }
    })
}
/// java: public static final Event<RoomMatched> ROOM_MATCHED = EventFactory.createArrayBacked(RoomMatched.class, callbacks -> room -> { for (RoomMatched callback : callbacks) { callback.onRoomMatched(room); } 
pub static ROOM_MATCHED: compat::MainThread<Event<dyn Fn(compat::Shared<dyn Room>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `ROOM_MATCHED` (`ROOM_MATCHED.invoker()`): the factory's lambda over the listeners
pub fn room_matched_invoker() -> RoomMatched {
    let callbacks: Vec<RoomMatched> = ROOM_MATCHED.listeners();
    Arc::new(move |room| {
        for callback in callbacks.iter() {
            (callback)(room.clone());
        }
        if ({ let __f = compat::dispatch(&room, |__t| __t.get_type()); __f(&room) }) == Type::Puzzle {
            (puzzle_matched_invoker())(room.clone());
        }
    })
}
/// java: public static final Event<SecretFound> SECRET_FOUND = EventFactory.createArrayBacked(SecretFound.class, callbacks -> (room, secretWaypoint) -> { for (SecretFound callback : callbacks) { callback.onSec
pub static SECRET_FOUND: compat::MainThread<Event<dyn Fn(compat::Shared<dyn Room>, compat::Shared<SecretWaypoint>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `SECRET_FOUND` (`SECRET_FOUND.invoker()`): the factory's lambda over the listeners
pub fn secret_found_invoker() -> SecretFound {
    let callbacks: Vec<SecretFound> = SECRET_FOUND.listeners();
    Arc::new(move |room, secret_waypoint| {
        for callback in callbacks.iter() {
            (callback)(room.clone(), secret_waypoint.clone());
        }
    })
}
/// java: public static final Event<SecretCountUpdate> SECRET_COUNT_UPDATED = EventFactory.createArrayBacked(SecretCountUpdate.class, callbacks -> (room, fromWS) -> { for (SecretCountUpdate callback : callbacks
pub static SECRET_COUNT_UPDATED: compat::MainThread<Event<dyn Fn(compat::Shared<dyn Room>, bool)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `SECRET_COUNT_UPDATED` (`SECRET_COUNT_UPDATED.invoker()`): the factory's lambda over the listeners
pub fn secret_count_updated_invoker() -> SecretCountUpdate {
    let callbacks: Vec<SecretCountUpdate> = SECRET_COUNT_UPDATED.listeners();
    Arc::new(move |room, from_ws| {
        for callback in callbacks.iter() {
            (callback)(room.clone(), from_ws);
        }
    })
}

/// java: public class DungeonEvents
#[derive(Clone)]
pub struct DungeonEvents {}

impl DungeonEvents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface DungeonLoaded
/// a functional interface: `onDungeonLoaded()` - a lambda / method reference is this closure type
pub type DungeonLoaded = Arc<dyn Fn()>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface DungeonStarted
/// a functional interface: `onDungeonStarted()` - a lambda / method reference is this closure type
pub type DungeonStarted = Arc<dyn Fn()>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface DungeonEnded
/// a functional interface: `onDungeonEnded()` - a lambda / method reference is this closure type
pub type DungeonEnded = Arc<dyn Fn()>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface RoomMatched
/// a functional interface: `onRoomMatched(Room room)` - a lambda / method reference is this closure type
pub type RoomMatched = Arc<dyn Fn(compat::Shared<dyn Room>)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SecretFound
/// a functional interface: `onSecretFound(Room room, SecretWaypoint secretWaypoint)` - a lambda / method reference is this closure type
pub type SecretFound = Arc<dyn Fn(compat::Shared<dyn Room>, compat::Shared<SecretWaypoint>)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface SecretCountUpdate
/// a functional interface: `onSecretCountUpdate(Room room, boolean fromWS)` - a lambda / method reference is this closure type
pub type SecretCountUpdate = Arc<dyn Fn(compat::Shared<dyn Room>, bool)>;
