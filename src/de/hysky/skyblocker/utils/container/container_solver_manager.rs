//! Ported from de.hysky.skyblocker.utils.container.ContainerSolverManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_container_solver;
use crate::de::hysky::skyblocker::skyblock::accessories::newyearcakes::new_year_cake_bag_helper::NewYearCakeBagHelper;
use crate::de::hysky::skyblocker::skyblock::accessories::newyearcakes::new_year_cakes_helper;
use crate::de::hysky::skyblocker::skyblock::auction::copy_underbid_price::CopyUnderbidPrice;
use crate::de::hysky::skyblocker::skyblock::bazaar::reorder_helper::ReorderHelper;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::chocolate_factory_solver;
use crate::de::hysky::skyblocker::skyblock::dungeon::croesus_helper;
use crate::de::hysky::skyblocker::skyblock::dungeon::croesus_profit;
use crate::de::hysky::skyblocker::skyblock::dungeon::salvage_helper::SalvageHelper;
use crate::de::hysky::skyblocker::skyblock::dungeon::sellable_items_highlighter::SellableItemsHighlighter;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::color_terminal::ColorTerminal;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::lights_on_terminal::LightsOnTerminal;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::order_terminal::OrderTerminal;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::same_color_terminal;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::starts_with_terminal::StartsWithTerminal;
use crate::de::hysky::skyblocker::skyblock::dwarven::commission_highlight::CommissionHighlight;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_solver::FossilSolver;
use crate::de::hysky::skyblocker::skyblock::end::end_stats_bestiary_updater::EndStatsBestiaryUpdater;
use crate::de::hysky::skyblocker::skyblock::experiment::chronomatron_solver::ChronomatronSolver;
use crate::de::hysky::skyblocker::skyblock::experiment::superpairs_solver::SuperpairsSolver;
use crate::de::hysky::skyblocker::skyblock::experiment::ultrasequencer_solver;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::tuner_solver;
use crate::de::hysky::skyblocker::skyblock::garden::stereo_harmony_helper;
use crate::de::hysky::skyblocker::skyblock::hunting::hunting_box_helper::HuntingBoxHelper;
use crate::de::hysky::skyblocker::skyblock::item::anvil_helper::AnvilHelper;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::bits_helper;
use crate::de::hysky::skyblocker::skyblock::raffle_reward_highlight::RaffleRewardHighlight;
use crate::de::hysky::skyblocker::skyblock::raffle_task_highlight::RaffleTaskHighlight;
use crate::de::hysky::skyblocker::skyblock::sky_block_equipment_updater::EquipmentWardrobe;
use crate::de::hysky::skyblocker::skyblock::sky_block_equipment_updater::SkyBlockEquipmentUpdater;
use crate::de::hysky::skyblocker::skyblock::sky_block_equipment_updater::SkyBlockEquipmentUpdaterBase;
use crate::de::hysky::skyblocker::skyblock::slayers::features::rewards_highlighter::RewardsHighlighter;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;

/// java: private static final List<ContainerSolver> solvers = new ArrayList<>(List.of( new ColorTerminal(), new OrderTerminal(), new StartsWithTerminal(), new LightsOnTerminal(), CroesusHelper.INSTANCE, Croesu
pub static SOLVERS: compat::MainThread<Vec<compat::Shared<dyn ContainerSolver>>> = compat::MainThread::new(|| vec![(ColorTerminal::new() as compat::Shared<dyn ContainerSolver>), (OrderTerminal::new() as compat::Shared<dyn ContainerSolver>), (StartsWithTerminal::new() as compat::Shared<dyn ContainerSolver>), (LightsOnTerminal::new() as compat::Shared<dyn ContainerSolver>), (croesus_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (croesus_profit::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (SalvageHelper::new() as compat::Shared<dyn ContainerSolver>), (ChronomatronSolver::new() as compat::Shared<dyn ContainerSolver>), (CommissionHighlight::new() as compat::Shared<dyn ContainerSolver>), (SuperpairsSolver::new() as compat::Shared<dyn ContainerSolver>), (ultrasequencer_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (NewYearCakeBagHelper::new() as compat::Shared<dyn ContainerSolver>), (new_year_cakes_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (chocolate_factory_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (tuner_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (ReorderHelper::new() as compat::Shared<dyn ContainerSolver>), (bits_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (RaffleTaskHighlight::new() as compat::Shared<dyn ContainerSolver>), (RaffleRewardHighlight::new() as compat::Shared<dyn ContainerSolver>), (FossilSolver::new() as compat::Shared<dyn ContainerSolver>), (same_color_terminal::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (CopyUnderbidPrice::new() as compat::Shared<dyn ContainerSolver>), (HuntingBoxHelper::new() as compat::Shared<dyn ContainerSolver>), (SellableItemsHighlighter::new() as compat::Shared<dyn ContainerSolver>), (stereo_harmony_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (EndStatsBestiaryUpdater::new() as compat::Shared<dyn ContainerSolver>), (accessories_container_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn ContainerSolver>), (AnvilHelper::new() as compat::Shared<dyn ContainerSolver>), (RewardsHighlighter::new() as compat::Shared<dyn ContainerSolver>), (SkyBlockEquipmentUpdaterBase::new() as compat::Shared<dyn ContainerSolver>), (EquipmentWardrobe::new() as compat::Shared<dyn ContainerSolver>)].into_iter().collect::<Vec<_>>());
thread_local! {
    /// How deep the render thread is inside `AbstractContainerScreen.init` (see `init`).
    static IN_CONTAINER_SCREEN_INIT: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

/// java: private static @Nullable ContainerSolver currentSolver = null;
pub static CURRENT_SOLVER: compat::MainThread<Option<compat::Shared<dyn ContainerSolver>>> = compat::MainThread::new(|| None);
/// java: private static @Nullable List<ColorHighlight> highlights;
pub static HIGHLIGHTS: LazyLock<Mutex<Option<Vec<ColorHighlight>>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static int screenId = 0;
pub static SCREEN_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class ContainerSolverManager
#[derive(Clone)]
pub struct ContainerSolverManager {}

impl ContainerSolverManager {
    /// java: private ContainerSolverManager()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static @Nullable ContainerSolver getCurrentSolver()
    pub fn get_current_solver() -> Option<compat::Shared<dyn ContainerSolver>> {
        return CURRENT_SOLVER.get_mut().clone();
    }

    /// java: @SuppressWarnings("unused") public static void registerSolver(ContainerSolver solver)
    pub fn register_solver(mut solver: compat::Shared<dyn ContainerSolver>) {
        SOLVERS.get_mut().push(solver.clone());
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ScreenEvents.BEFORE_INIT.register((_, screen, _, _) -> { if (screen instanceof AbstractContainerScreen<?>
        // containerScreen) {..} else clearScreen(); })
        // The port hands BEFORE_INIT the `Screen` husk only (a container screen's `AbstractContainerScreen` is the object
        // around it), so `instanceof AbstractContainerScreen` is the call of `AbstractContainerScreen.init` the husk's
        // `Screen.init(w, h)` runs inside: its HEAD is the container arm, and BEFORE_INIT outside it is the other arm.
        report(hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", At::Head, |ci: &mut CallbackInfo<'_>| {
            if let Some(container_screen) = ci.this_mut::<AbstractContainerScreen>() {
                IN_CONTAINER_SCREEN_INIT.with(|depth| depth.set(depth.get() + 1));
                ContainerSolverManager::on_set_screen(container_screen);
            }
        }));
        report(hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", At::Return, |_: &mut CallbackInfo<'_>| {
            IN_CONTAINER_SCREEN_INIT.with(|depth| depth.set((depth.get() - 1).max(0)));
        }));
        // java: ScreenEvents.remove(screen).register(_ -> clearScreen()); - registered for every container screen, so
        // the removal of any container screen
        report(hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#removed", At::Head, |_: &mut CallbackInfo<'_>| {
            ContainerSolverManager::clear_screen();
        }));
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            if IN_CONTAINER_SCREEN_INIT.with(|depth| depth.get()) == 0 {
                ContainerSolverManager::clear_screen();
            }
        }));
    }

    /// java: public static void onSetScreen(AbstractContainerScreen<?> screen)
    pub fn on_set_screen(screen: &AbstractContainerScreen) {
        let mut screen_name: String = screen.screen.get_title().to_string();
        let __solvers: Vec<compat::Shared<dyn ContainerSolver>> = SOLVERS.get().clone();
        for solver in __solvers.iter() {
            if (Utils::is_on_skyblock() || !({ let __f = compat::dispatch(solver, |__t| __t.skyblock_only()); __f(&compat::view(solver, |a| a as compat::Shared<dyn ContainerMatcher>)) })) && { let __f = compat::dispatch(solver, |__t| __t.is_enabled()); __f(&compat::view(solver, |a| a as compat::Shared<dyn ContainerMatcher>)) } {
                // java: if ((solver instanceof RegexContainerMatcher containerMatcher && containerMatcher.test(screenName)) || solver.test(screen))
                // A RegexContainerMatcher's `test(Screen)` is `test(screen.getTitle().getString())` (RegexContainerMatcher.test_body),
                // the same title as `screenName`, so the first operand adds nothing the second does not answer.
                let _ = &screen_name;
                if { let __f = compat::dispatch(solver, |__t| __t.test()); __f(&compat::view(solver, |a| a as compat::Shared<dyn ContainerMatcher>), &screen.screen) } {
                    // java: if (screen instanceof ContainerScreen || !solver.chestScreensOnly())
                    // (`screen` is being initialised by the game's open site, before `setScreen` writes `gui.screen_kind`,
                    // which then still names the PREVIOUS screen: the build site answers for the screen itself)
                    let __is_container_screen = mcrust::net::minecraft::client::gui::screens::menu_screens::building_chest_screen()
                        .unwrap_or_else(|| compat::minecraft().gui.screen_kind == mcrust::net::minecraft::client::gui::gui::ScreenKind::Container);
                    if __is_container_screen || !({ let __f = compat::dispatch(solver, |__t| __t.chest_screens_only()); __f(solver) }) {
                        { let mut __g = compat::lock(&SCREEN_ID); *__g = __g.wrapping_add(1); }
                        *CURRENT_SOLVER.get_mut() = Some(solver.clone());
                        ({ let __f = compat::dispatch(solver, |__t| __t.start()); __f(solver, screen) });
                        ContainerSolverManager::mark_highlights_dirty();
                        return;
                    }
                }
            }
        }
        ContainerSolverManager::clear_screen();
    }

    /// java: public static void clearScreen()
    pub fn clear_screen() {
        if CURRENT_SOLVER.get_mut().is_some() {
            ({ let __r = CURRENT_SOLVER.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.reset()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Resettable>)) });
            *CURRENT_SOLVER.get_mut() = None;
        }
    }

    /// java: public static void markHighlightsDirty()
    pub fn mark_highlights_dirty() {
        compat::set_state(&HIGHLIGHTS, None);
        if CURRENT_SOLVER.get_mut().is_some() {
            ({ let __r = CURRENT_SOLVER.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.mark_dirty()); __f(__r) });
        }
    }

    /// java: public static boolean onSlotClick(int slot, ItemStack stack, int button)
    pub fn on_slot_click(mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut button: i32) -> bool {
        return (CURRENT_SOLVER.get_mut().is_some() && { let __r = CURRENT_SOLVER.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.on_click_slot()); __f(__r, slot, stack, compat::get_state(&SCREEN_ID), button) });
    }

    /// java: public static void onExtract(GuiGraphicsExtractor context, AbstractContainerScreen<?> handledScreen, List<Slot> slots)
    ///
    /// `containers` is what the slots' `getItem()` reads through: the hooked `extractTooltip` call's own (its argument
    /// 2), lent by the game for the call. Java's `slots.get(i).getItem()` reads the menu the screen is drawing; the
    /// port's `with_open_menu` cannot lend that menu here, because the frame's extract is the thread holding it, and
    /// answered every slot EMPTY - so every solver saw an empty container (the equipment updater copied four EMPTYs
    /// into the drawn equipment).
    pub fn on_extract(context: &mut GuiGraphicsExtractor, handled_screen: &AbstractContainerScreen, slots: Vec<Slot>) {
        ContainerSolverManager::on_extract_in(context, handled_screen, slots, None, None)
    }

    /// `on_extract` with the containers the slots read through (see above).
    pub fn on_extract_in(context: &mut GuiGraphicsExtractor, handled_screen: &AbstractContainerScreen, mut slots: Vec<Slot>, containers: Option<&mcrust::net::minecraft::world::inventory::slot::Containers<'_>>, menu: Option<&mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>) {
        if CURRENT_SOLVER.get_mut().is_none() {
            return;
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((handled_screen.left_pos) as f32, (handled_screen.top_pos) as f32);
        if { let __r = CURRENT_SOLVER.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.chest_inventory_only()); __f(__r) } && let Some(row_count) = (match menu {
            // `handledScreen.getMenu() instanceof ChestMenu chestMenu` -> `chestMenu.getRowCount()`, off the menu the hooked
            // call lent: a chest menu is its `rows * 9` grid slots followed by the player's 36 (`ChestMenu:36-58`). The
            // frame holds the player's open menu here, so `with_chest_menu` cannot lend it and never trimmed.
            Some(menu) => compat::is_chest_menu(menu).then(|| ((menu.slots.len() as i32) - 36) / 9),
            None => mcrust::mcrust::modding::game::with_chest_menu(|chest_menu| chest_menu.get_row_count()),
        }) {
            // java: handledScreen.getMenu() instanceof ChestMenu chestMenu - the screen's menu is the player's open one
            slots = slots[(0) as usize..(row_count.wrapping_mul(9)) as usize].to_vec();
        }
        let mut highlights_safe: Option<Vec<ColorHighlight>> = compat::get_state(&HIGHLIGHTS);
        if highlights_safe.is_none() {
            highlights_safe = Some(({ let __r = CURRENT_SOLVER.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_colors()); __f(__r, &mut match containers { Some(containers) => ContainerSolverManager::slot_map_in(&slots, containers), None => ContainerSolverManager::slot_map(&slots) }) }));
            compat::set_state(&HIGHLIGHTS, highlights_safe.clone());
        }
        for highlight in highlights_safe.as_ref().unwrap().iter() {
            let slot = &slots[(highlight.slot()) as usize];
            let mut color: i32 = highlight.color();
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), slot.x, slot.y, slot.x.wrapping_add(16), slot.y.wrapping_add(16), color);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// `slotMap(slots)` over the containers a hooked call lent (`slots.get(i).getItem()` read through them).
    pub fn slot_map_in(slots: &Vec<Slot>, containers: &mcrust::net::minecraft::world::inventory::slot::Containers<'_>) -> std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut slot_map: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = std::collections::BTreeMap::new();
        for i in 0..(slots.len() as i32) {
            slot_map.insert(i, slots[(i) as usize].get_item(containers).duplicate());
        }
        return slot_map;
    }

    /// java: public static Int2ObjectMap<ItemStack> slotMap(List<Slot> slots)
    pub fn slot_map(slots: &Vec<Slot>) -> std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut slot_map: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = std::collections::BTreeMap::new();
        // java: for (int i = 0; i < slots.size(); i++) slotMap.put(i, slots.get(i).getItem());
        // (the port's `Slot::get_item` reads through the open menu's containers: one lend for the whole loop; `duplicate`
        // keeps the stack's identity, as `compat::slot_item`. Where the menu cannot be lent - no player, or this thread
        // holds it - every slot reads EMPTY, as `compat::slot_item` answers)
        let __filled = compat::with_open_menu(|_, containers| {
            for i in 0..(slots.len() as i32) {
                slot_map.insert(i, slots[(i) as usize].get_item(containers).duplicate());
            }
        });
        if __filled.is_none() {
            for i in 0..(slots.len() as i32) {
                slot_map.insert(i, mcrust::net::minecraft::world::item::item_stack::empty());
            }
        }
        return slot_map;
    }

}
