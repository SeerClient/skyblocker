//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.DungeonPuzzle by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;

/// java: public abstract class DungeonPuzzle implements Tickable, Renderable, Resettable
#[derive(Clone)]
pub struct DungeonPuzzleBase {
    /// java: protected final String puzzleName;
    pub puzzle_name: String,
    /// java: private final Set<String> roomNames;
    pub room_names: std::collections::HashSet<String>,
    /// java: private boolean shouldSolve;
    pub should_solve: bool,
}

impl DungeonPuzzleBase {
    /// java: public DungeonPuzzle(String puzzleName, String... roomName) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut puzzle_name: String, mut room_name: Vec<String>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DungeonPuzzle>, __R)) -> (compat::Shared<dyn DungeonPuzzle>, __R) {
        let (__this, __r) = Self::new2_in(puzzle_name.clone(), room_name.to_vec(), __wrap);
        (__this, __r)
    }

    /// java: public DungeonPuzzle(String puzzleName, Set<String> roomNames) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut puzzle_name: String, mut room_names: std::collections::HashSet<String>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DungeonPuzzle>, __R)) -> (compat::Shared<dyn DungeonPuzzle>, __R) {
        let __init_puzzle_name = puzzle_name.clone();
        let __init_room_names = room_names.clone();
        let this = Self {
            puzzle_name: __init_puzzle_name,
            room_names: __init_room_names,
            should_solve: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        let __this_1 = compat::RenderThread::new(__this.clone());
        dungeon_events::PUZZLE_MATCHED.register(Arc::new(move |room| {
            if (*__this_1.get()).lock().borrow().dungeon_puzzle_base().room_names.contains(({ let __f = compat::dispatch(&room, |__t| __t.get_name()); __f(&room) }).as_ref().unwrap()) {
                { let (__a0,) = (((*__this_1.get()).clone() as compat::Shared<dyn Tickable>),); room.lock().borrow_mut().add_sub_process(__a0) };
                { let __v = true; compat::write(&(*__this_1.get()), |__t| __t.dungeon_puzzle_base_mut().should_solve = __v); }
            }
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        // TODO(fabric2seer): a listener that is neither a lambda nor a method reference
        //   java: ClientPlayConnectionEvents.JOIN.register(this);
        (__this, __r)
    }

}

/// java: the virtual methods of DungeonPuzzle (implemented by DungeonPuzzleBase and every subclass)
pub trait DungeonPuzzle: Tickable + Renderable + Resettable {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase;
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase;

    /// java: public boolean shouldSolve() (dispatch: the body for this class)
    fn should_solve(&self) -> fn(&compat::Shared<dyn DungeonPuzzle>) -> bool {
        <dyn DungeonPuzzle>::should_solve_body
    }

}

impl dyn DungeonPuzzle {
    /// java: public boolean shouldSolve()
    pub fn should_solve_body(__this: &compat::Shared<dyn DungeonPuzzle>) -> bool {
        return compat::read(__this, |__t| __t.dungeon_puzzle_base().should_solve);
    }

    /// java: @Override public void reset()
    pub fn reset_body(__this: &compat::Shared<dyn DungeonPuzzle>) {
        { let __v = false; compat::write(__this, |__t| __t.dungeon_puzzle_base_mut().should_solve = __v); }
    }

    /// DungeonPuzzle.reset as the dispatch of Resettable.reset for the concrete class `__C`
    pub fn reset_from<__C: DungeonPuzzle + 'static>(__this: &compat::Shared<dyn Resettable>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn DungeonPuzzle>))
    }

}

// DungeonPuzzle.reset implements Resettable.reset: it is emitted in each concrete subclass's `impl Resettable` (Rust has no inherited impls)
