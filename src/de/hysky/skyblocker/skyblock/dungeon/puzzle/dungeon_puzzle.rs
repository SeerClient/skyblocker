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
        let (__this, __r) = Self::new2_in(puzzle_name.clone(), room_name.iter().cloned().collect::<std::collections::HashSet<String>>(), __wrap);
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
                { let (__a0, __a1) = (((*__this_1.get()).clone() as compat::Shared<dyn Tickable>), ((*__this_1.get()).clone() as compat::Shared<dyn Renderable>)); room.lock().borrow_mut().add_sub_process(__a0, __a1) };
                { let __v = true; compat::write(&(*__this_1.get()), |__t| __t.dungeon_puzzle_base_mut().should_solve = __v); }
            }
        }));
        let __this_3 = compat::RenderThread::new(__this.clone());
        client_command_registration_callback::EVENT.register(Arc::new(
            move |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use crate::prelude::client_command_registration_callback as ccr;
                type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<ccr::FabricClientCommandSource>;
                fn prefixed(text: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
                    (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(text)
                }
                // java: dispatcher.register(literal(NAMESPACE).then(literal("dungeons").then(literal("puzzle").then(literal(puzzleName).then(literal("solve").executes(context -> {
                // java:     Room currentRoom = DungeonManager.getCurrentRoom(); if (currentRoom != null) { reset(); currentRoom.addSubProcess(this); sendFeedback(..); } else sendError(..); return 1; }))))));
                let __this_3 = __this_3.clone();
                let puzzle_name: String = compat::read(&*__this_3.get(), |__t| __t.dungeon_puzzle_base().puzzle_name.clone());
                let mut solve = ccr::literal("solve");
                let __solve_name = puzzle_name.clone();
                solve.executes(Arc::new(move |_: &Ctx| {
                    let this = (*__this_3.get()).clone();
                    match crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::get_current_room() {
                        Some(current_room) => {
                            { let __r = this.clone() as compat::Shared<dyn Resettable>; let __f = compat::dispatch(&__r, |__t| __t.reset()); __f(&__r); }
                            { let (__a0, __a1) = ((this.clone() as compat::Shared<dyn Tickable>), (this.clone() as compat::Shared<dyn Renderable>)); current_room.lock().borrow_mut().add_sub_process(__a0, __a1) };
                            compat::batch_w_send_feedback(&prefixed(mcrust::net::minecraft::network::chat::component::literal(&format!("Solving {} puzzle in the current room.", __solve_name)).with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value())));
                        }
                        None => {
                            compat::batch_w_send_error(&prefixed(mcrust::net::minecraft::network::chat::component::literal("Current room is null.").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
                        }
                    }
                    Ok(1)
                }));
                let mut puzzle = ccr::literal(&puzzle_name);
                puzzle.then(&solve);
                let mut __n0 = ccr::literal("puzzle");
                __n0.then(&puzzle);
                let mut __n1 = ccr::literal("dungeons");
                __n1.then(&__n0);
                let mut __n2 = ccr::literal(crate::entry::skyblocker_mod::NAMESPACE);
                __n2.then(&__n1);
                dispatcher.dispatcher().register(&__n2);
            },
        ));
        // java: ClientPlayConnectionEvents.JOIN.register(this); - `Resettable.onPlayReady` is `reset()`
        let __this_2 = compat::RenderThread::new(__this.clone() as compat::Shared<dyn Resettable>);
        client_play_connection_events::JOIN.register(Arc::new(move |_listener: &mut ClientPacketListener| {
            let __r = __this_2.get();
            let __f = compat::dispatch(__r, |__t| __t.reset());
            __f(__r);
        }));
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
