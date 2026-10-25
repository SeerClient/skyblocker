//! Ported from de.hysky.skyblocker.utils.scheduler.Scheduler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::util::profiling::profiler;
use mcrust::net::minecraft::util::profiling::profiler_filler::ProfilerFiller;
use mcrust::net::minecraft::util::thread::task_scheduler::TaskScheduler as _;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::mcrust::modding::screens::ModScreen;

//// java: public static final Scheduler INSTANCE = new Scheduler();
pub static INSTANCE: compat::MainThread<compat::Shared<dyn Scheduler>> = compat::MainThread::new(|| (SchedulerBase::new() as compat::Shared<dyn Scheduler>));

/// java: public class Scheduler
pub struct SchedulerBase {
    /// java: private int currentTick = 0;
    pub current_tick: i32,
    /// java: private final AbstractInt2ObjectMap<List<Runnable>> tasks = new Int2ObjectOpenHashMap<>();
    pub tasks: std::collections::HashMap<i32, Vec<Arc<dyn Fn()>>>,
    /// java: private final ExecutorService executors = SkyblockerMod.VIRTUAL_THREAD_EXECUTOR;
    /// (a new thread per task: `compat::executor`)
    pub executors: fn(Box<dyn FnOnce() + Send>),
}

impl SchedulerBase {
    /// java: protected Scheduler() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn Scheduler>, __R)) -> (compat::Shared<dyn Scheduler>, __R) {
        let this = Self {
            current_tick: 0,
            tasks: std::collections::HashMap::new(),
            executors: compat::executor,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected Scheduler()
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Scheduler>), __h)
        }).1
    }

    /// java: public static Command<FabricClientCommandSource> queueOpenScreenFactoryCommand(Function<CommandContext<FabricClientCommandSource>, Screen> screenFactory)
    pub fn queue_open_screen_factory_command(screen_factory: Arc<dyn Fn(&CommandContext<FabricClientCommandSource>) -> Box<dyn ModScreen> + Send + Sync>) -> Command<FabricClientCommandSource> {
        // java: return context -> queueOpenScreen(screenFactory.apply(context));
        return Arc::new(move |context: &CommandContext<FabricClientCommandSource>| Ok(SchedulerBase::queue_open_screen_screen((screen_factory)(context))));
    }

    /// java: public static Command<FabricClientCommandSource> queueOpenScreenCommand(Supplier<Screen> screenSupplier)
    pub fn queue_open_screen_command(screen_supplier: Arc<dyn Fn() -> Box<dyn ModScreen> + Send + Sync>) -> Command<FabricClientCommandSource> {
        // java: return _ -> queueOpenScreen(screenSupplier.get());
        return Arc::new(move |_: &CommandContext<FabricClientCommandSource>| Ok(SchedulerBase::queue_open_screen_screen((screen_supplier)())));
    }

    /// java: public static Command<FabricClientCommandSource> queueOpenScreenCommand(Screen screen)
    /// Java reopens the SAME screen object on every run: the object is the mod's `compat::Shared` screen, and each run
    /// opens a new game-side handle (`compat::ScreenAdapter`) over that one object.
    pub fn queue_open_screen_command_screen<S: compat::ScreenHalf + 'static>(screen: compat::Shared<S>) -> Command<FabricClientCommandSource>
    where
        compat::ScreenAdapter<S>: ModScreen,
    {
        // java: return _ -> queueOpenScreen(screen);
        let screen = compat::RenderThread::new(screen);
        return Arc::new(move |_: &CommandContext<FabricClientCommandSource>| Ok(SchedulerBase::queue_open_screen_screen(Box::new(compat::ScreenAdapter::new(screen.get().clone())))));
    }

    /// java: @Deprecated(forRemoval = true) public static int queueOpenScreen(Supplier<Screen> screenSupplier)
    pub fn queue_open_screen(screen_supplier: Arc<dyn Fn() -> Box<dyn ModScreen>>) -> i32 {
        return SchedulerBase::queue_open_screen_screen((screen_supplier)());
    }

    /// java: public static int queueOpenScreen(Screen screen)
    pub fn queue_open_screen_screen(screen: Box<dyn ModScreen>) -> i32 {
        // java: Minecraft.getInstance().schedule(() -> Minecraft.getInstance().gui.setScreen(screen));
        mcrust::net::minecraft::client::minecraft::execute(Box::new(move |_minecraft| mcrust::mcrust::modding::game::set_screen(screen)));
        // java: return Command.SINGLE_SUCCESS;
        return 1;
    }

}

/// java: the virtual methods of Scheduler (implemented by SchedulerBase and every subclass)
pub trait Scheduler: compat::DynAny {
    fn scheduler_base(&self) -> &SchedulerBase;
    fn scheduler_base_mut(&mut self) -> &mut SchedulerBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void schedule(Runnable task, int delay) (dispatch: the body for this class)
    fn schedule(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>, i32) {
        <dyn Scheduler>::schedule_body
    }

    /// java: public void schedule(Runnable task, int delay, boolean multithreaded) (dispatch: the body for this class)
    fn schedule_3(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>, i32, bool) {
        <dyn Scheduler>::schedule_3_body
    }

    /// java: public void scheduleCyclic(Runnable task, int period) (dispatch: the body for this class)
    fn schedule_cyclic(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>, i32) {
        <dyn Scheduler>::schedule_cyclic_body
    }

    /// java: public void scheduleCyclic(Runnable task, int period, boolean multithreaded) (dispatch: the body for this class)
    fn schedule_cyclic_3(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>, i32, bool) {
        <dyn Scheduler>::schedule_cyclic_3_body
    }

    /// java: public void tick() (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Scheduler>) {
        <dyn Scheduler>::tick_body
    }

    /// java: protected boolean runTask(Runnable task) (dispatch: the body for this class)
    fn run_task(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>) -> bool {
        <dyn Scheduler>::run_task_body
    }

    /// java: private void addTask(Runnable task, int schedule) (dispatch: the body for this class)
    fn add_task(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>, i32) {
        <dyn Scheduler>::add_task_body
    }

}

impl dyn Scheduler {
    /// java: public void schedule(Runnable task, int delay)
    pub fn schedule_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>, mut delay: i32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.schedule_3()); __f(__this, task.clone(), delay, false) });
    }

    /// java: public void schedule(Runnable task, int delay, boolean multithreaded)
    pub fn schedule_3_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>, mut delay: i32, mut multithreaded: bool) {
        RenderHelper::assert_on_render_thread("Scheduler called from outside the Render Thread");
        if delay >= 0 {
            // java: addTask(multithreaded ? new ScheduledTask(task, true) : task, currentTick + delay);
            let task: Arc<dyn Fn()> = if multithreaded { ScheduledTask::new_2(task.clone(), true).into_runnable() } else { task.clone() };
            ({ let __f = compat::dispatch(__this, |__t| __t.add_task()); __f(__this, task, (compat::read(__this, |__t| __t.scheduler_base().current_tick)).wrapping_add(delay)) });
        } else {
            logger().warn("Scheduled a task with negative delay");
        }
    }

    /// java: public void scheduleCyclic(Runnable task, int period)
    pub fn schedule_cyclic_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>, mut period: i32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.schedule_cyclic_3()); __f(__this, task.clone(), period, false) });
    }

    /// java: public void scheduleCyclic(Runnable task, int period, boolean multithreaded)
    pub fn schedule_cyclic_3_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>, mut period: i32, mut multithreaded: bool) {
        RenderHelper::assert_on_render_thread("Scheduler called from outside the Render Thread");
        if period > 0 {
            // java: addTask(new ScheduledTask(task, period, true, multithreaded), currentTick);
            let task: Arc<dyn Fn()> = ScheduledTask::new(task.clone(), period, true, multithreaded).into_runnable();
            ({ let __f = compat::dispatch(__this, |__t| __t.add_task()); __f(__this, task, compat::read(__this, |__t| __t.scheduler_base().current_tick)) });
        } else {
            logger().error("Attempted to schedule a cyclic task with period lower than 1");
        }
    }

    /// java: public void tick()
    pub fn tick_body(__this: &compat::Shared<dyn Scheduler>) {
        let profiler = profiler::get();
        profiler.push("skyblockerSchedulerTick");
        let current_tick: i32 = compat::read(__this, |__t| __t.scheduler_base().current_tick);
        if compat::read(__this, |__t| __t.scheduler_base().tasks.contains_key(&current_tick)) {
            // java: List<Runnable> currentTickTasks = tasks.get(currentTick);
            //       for (int i = 0; i < currentTickTasks.size(); i++) - the list ITSELF: a task scheduled for this tick
            //       while it runs is added to it and runs in this loop too; every task runs with nothing borrowed
            let mut i: usize = 0;
            loop {
                let Some(task) = compat::read(__this, |__t| __t.scheduler_base().tasks.get(&current_tick).and_then(|list| list.get(i).cloned())) else {
                    break;
                };
                if !({ let __f = compat::dispatch(__this, |__t| __t.run_task()); __f(__this, task.clone()) }) {
                    // java: tasks.computeIfAbsent(currentTick + 1, _ -> new ArrayList<>()).add(task);
                    compat::write(__this, |__t| __t.scheduler_base_mut().tasks.entry(current_tick.wrapping_add(1)).or_insert_with(Vec::new).push(task.clone()));
                }
                i += 1;
            }
            // java: tasks.remove(currentTick);
            // PERF (port-only, same result): the removed list's buffer is kept for the next `addTask` that needs a new
            // list (cyclic tasks re-add themselves every period), instead of one free + one allocation per tick.
            let removed: Option<Vec<Arc<dyn Fn()>>> = compat::write(__this, |__t| __t.scheduler_base_mut().tasks.remove(&current_tick));
            if let Some(list) = removed {
                recycle_task_list(list);
            }
        }
        { let __v = (compat::read(__this, |__t| __t.scheduler_base().current_tick)).wrapping_add(1); compat::write(__this, |__t| __t.scheduler_base_mut().current_tick = __v); }
        profiler.pop();
    }

    /// java: protected boolean runTask(Runnable task)
    pub fn run_task_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>) -> bool {
        (task)();
        return true;
    }

    /// java: private void addTask(Runnable task, int schedule)
    pub fn add_task_body(__this: &compat::Shared<dyn Scheduler>, task: Arc<dyn Fn()>, mut schedule: i32) {
        compat::write(__this, |__t| {
            let tasks = &mut __t.scheduler_base_mut().tasks;
            if tasks.contains_key(&schedule) {
                // java: tasks.get(schedule).add(task);
                tasks.get_mut(&schedule).unwrap().push(task.clone());
            } else {
                let mut list: Vec<Arc<dyn Fn()>> = take_task_list();
                list.push(task.clone());
                // java: tasks.put(schedule, list);
                tasks.insert(schedule, list);
            }
        });
    }

}

thread_local! {
    /// PERF (port-only): emptied task lists, reused by `addTask` (see `tick`). Bounded; the render thread's only.
    static TASK_LIST_POOL: std::cell::RefCell<Vec<Vec<Arc<dyn Fn()>>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// A list for `addTask`'s `new ArrayList<>()`: an emptied one from the pool when there is one.
fn take_task_list() -> Vec<Arc<dyn Fn()>> {
    TASK_LIST_POOL.with(|pool| pool.borrow_mut().pop()).unwrap_or_default()
}

/// Returns a removed list's buffer to the pool (its tasks are dropped here, as the removal dropped them).
fn recycle_task_list(mut list: Vec<Arc<dyn Fn()>>) {
    list.clear();
    TASK_LIST_POOL.with(|pool| {
        let mut pool = pool.borrow_mut();
        if pool.len() < 64 {
            pool.push(list);
        }
    });
}

impl Scheduler for SchedulerBase {
    fn scheduler_base(&self) -> &SchedulerBase {
        self
    }
    fn scheduler_base_mut(&mut self) -> &mut SchedulerBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: protected record ScheduledTask(Runnable task, int interval, boolean cyclic, boolean multithreaded) implements Runnable
#[derive(Clone)]
pub struct ScheduledTask {
    /// java: Runnable task
    pub task: Arc<dyn Fn()>,
    /// java: int interval
    pub interval: i32,
    /// java: boolean cyclic
    pub cyclic: bool,
    /// java: boolean multithreaded
    pub multithreaded: bool,
}

impl ScheduledTask {
    /// java: private ScheduledTask(Runnable task, boolean multithreaded)
    pub fn new_2(task: Arc<dyn Fn()>, mut multithreaded: bool) -> Self {
        let mut this = Self::new(task.clone(), -1, false, multithreaded);
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut task: Arc<dyn Fn()>, mut interval: i32, mut cyclic: bool, mut multithreaded: bool) -> Self {
        Self { task, interval, cyclic, multithreaded }
    }

    /// The record as the `Runnable` it implements (what the scheduler's task lists hold).
    ///
    /// The runnable knows its own handle (a `Weak`), so a cyclic task re-adds THAT object - Java's
    /// `INSTANCE.addTask(this, ..)` - instead of allocating a fresh runnable on every run (PERF, port-only).
    pub fn into_runnable(self) -> Arc<dyn Fn()> {
        let slot: std::rc::Rc<std::cell::OnceCell<std::sync::Weak<dyn Fn()>>> = std::rc::Rc::new(std::cell::OnceCell::new());
        let slot_in = slot.clone();
        let runnable: Arc<dyn Fn()> = Arc::new(move || self.run_as(slot_in.get().and_then(|weak| weak.upgrade())));
        let _ = slot.set(Arc::downgrade(&runnable));
        runnable
    }

    /// java: the record accessor `task()`
    pub fn task(&self) -> Arc<dyn Fn()> {
        self.task.clone()
    }

    /// java: the record accessor `interval()`
    pub fn interval(&self) -> i32 {
        self.interval
    }

    /// java: the record accessor `cyclic()`
    pub fn cyclic(&self) -> bool {
        self.cyclic
    }

    /// java: the record accessor `multithreaded()`
    pub fn multithreaded(&self) -> bool {
        self.multithreaded
    }

    /// java: @Override public void run()
    pub fn run(&self) {
        self.run_as(None);
    }

    /// `run()` with the runnable that holds this record (`this` as the scheduler's lists hold it), if known.
    fn run_as(&self, this_runnable: Option<Arc<dyn Fn()>>) {
        if self.multithreaded {
            // java: INSTANCE.executors.execute(this.task);
            // Verified gap (GAPS_batchA.md): a scheduled task is an `Arc<dyn Fn()>` over the mod's render-thread objects
            // (not `Send`), so it cannot be handed to another thread; it runs here
            (self.task)();
        } else {
            (self.task)();
        }
        if self.cyclic {
            // java: INSTANCE.addTask(this, INSTANCE.currentTick + this.interval);
            let __r = INSTANCE.get_mut();
            let current_tick: i32 = compat::read(__r, |__t| __t.scheduler_base().current_tick);
            let runnable: Arc<dyn Fn()> = this_runnable.unwrap_or_else(|| self.clone().into_runnable());
            ({ let __f = compat::dispatch(__r, |__t| __t.add_task()); __f(__r, runnable, current_tick.wrapping_add(self.interval)) });
        }
    }

}
