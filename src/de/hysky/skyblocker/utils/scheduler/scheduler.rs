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

/// java: public static final Scheduler INSTANCE = new Scheduler();
pub static INSTANCE: compat::MainThread<compat::Shared<dyn Scheduler>> = compat::MainThread::new(|| (SchedulerBase::new() as compat::Shared<dyn Scheduler>));

/// java: public class Scheduler
pub struct SchedulerBase {
    /// java: private int currentTick = 0;
    pub current_tick: i32,
    /// java: private final AbstractInt2ObjectMap<List<Runnable>> tasks = new Int2ObjectOpenHashMap<>();
    pub tasks: crate::unresolved::AbstractInt2ObjectMap,
    /// java: private final ExecutorService executors = SkyblockerMod.VIRTUAL_THREAD_EXECUTOR;
    pub executors: crate::unresolved::ExecutorService,
}

impl SchedulerBase {
    /// java: protected Scheduler() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn Scheduler>, __R)) -> (compat::Shared<dyn Scheduler>, __R) {
        let this = Self {
            current_tick: 0,
            tasks: std::collections::HashMap::new(),
            executors: skyblocker_mod::VIRTUAL_THREAD_EXECUTOR.get_mut().clone(),
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
    pub fn queue_open_screen_factory_command(screen_factory: Arc<dyn Fn(CommandContext<crate::unresolved::FabricClientCommandSource>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> Command {
        return Arc::new(move |context| SchedulerBase::queue_open_screen_screen(&(screen_factory)(context.clone())));
    }

    /// java: public static Command<FabricClientCommandSource> queueOpenScreenCommand(Supplier<Screen> screenSupplier)
    pub fn queue_open_screen_command(screen_supplier: Arc<dyn Fn() -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> Command {
        return Arc::new(move |_| SchedulerBase::queue_open_screen_screen(&(screen_supplier)()));
    }

    /// java: public static Command<FabricClientCommandSource> queueOpenScreenCommand(Screen screen)
    pub fn queue_open_screen_command_screen(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Command {
        return Arc::new(move |_| SchedulerBase::queue_open_screen_screen(screen));
    }

    /// java: @Deprecated(forRemoval = true) public static int queueOpenScreen(Supplier<Screen> screenSupplier)
    pub fn queue_open_screen(screen_supplier: Arc<dyn Fn() -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> i32 {
        return SchedulerBase::queue_open_screen_screen(&(screen_supplier)());
    }

    /// java: public static int queueOpenScreen(Screen screen)
    pub fn queue_open_screen_screen(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> i32 {
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`R`)
        // TODO(fabric2seer): `schedule` is a method of the port's trait `TaskScheduler`, which the port's `Minecraft` does not implement
        // rust: compat::minecraft().schedule(|| gui::set_screen(&mut screen));
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
            ({ let __f = compat::dispatch(__this, |__t| __t.add_task()); __f(__this, (if multithreaded { ScheduledTask::new(task.clone(), true) } else { task.clone() }), (compat::read(__this, |__t| __t.scheduler_base().current_tick)).wrapping_add(delay)) });
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
            ({ let __f = compat::dispatch(__this, |__t| __t.add_task()); __f(__this, ScheduledTask::new(task.clone(), period, true, multithreaded), compat::read(__this, |__t| __t.scheduler_base().current_tick)) });
        } else {
            logger().error("Attempted to schedule a cyclic task with period lower than 1");
        }
    }

    /// java: public void tick()
    pub fn tick_body(__this: &compat::Shared<dyn Scheduler>) {
        let mut profiler: Arc<dyn ProfilerFiller> = profiler::get();
        profiler.push("skyblockerSchedulerTick");
        // TODO(fabric2seer): the condition: `.containsKey(..)` on a AbstractInt2ObjectMap
        //   java: tasks.containsKey(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.get(..)` on a AbstractInt2ObjectMap
            //   java: List<Runnable> currentTickTasks = tasks.get(currentTick);
            let mut current_tick_tasks: Vec<Arc<dyn Fn()>> = todo!("fabric2seer: the initialiser of `currentTickTasks` (TODO above)");
            for i in 0..(current_tick_tasks.len() as i32) {
                let mut task: Arc<dyn Fn()> = current_tick_tasks[(i) as usize].clone();
                if !({ let __f = compat::dispatch(__this, |__t| __t.run_task()); __f(__this, task.clone()) }) {
                    // TODO(fabric2seer): `.computeIfAbsent(..)` on a AbstractInt2ObjectMap
                    //   java: tasks.computeIfAbsent(currentTick + 1, _ -> new ArrayList<>()).add(task);
                }
            }
            // TODO(fabric2seer): `.remove(..)` on a AbstractInt2ObjectMap
            //   java: tasks.remove(currentTick);
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
        // TODO(fabric2seer): the condition: `.containsKey(..)` on a AbstractInt2ObjectMap
        //   java: tasks.containsKey(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.get(..)` on a AbstractInt2ObjectMap
            //   java: tasks.get(schedule).add(task);
        } else {
            let mut list: Vec<Arc<dyn Fn()>> = Vec::new();
            list.push(task.clone());
            // TODO(fabric2seer): `.put(..)` on a AbstractInt2ObjectMap
            //   java: tasks.put(schedule, list);
        }
    }

}

impl Scheduler for SchedulerBase {
    fn scheduler_base(&self) -> &SchedulerBase {
        &self
    }
    fn scheduler_base_mut(&mut self) -> &mut SchedulerBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: protected record ScheduledTask(Runnable task, int interval, boolean cyclic, boolean multithreaded) implements Runnable
#[derive(Clone, Debug)]
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
    pub fn new(task: Arc<dyn Fn()>, mut multithreaded: bool) -> Self {
        let mut this = Self::new(task.clone(), -1, false, multithreaded);
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut task: Arc<dyn Fn()>, mut interval: i32, mut cyclic: bool, mut multithreaded: bool) -> Self {
        Self { task, interval, cyclic, multithreaded }
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
        if self.multithreaded {
            // TODO(fabric2seer): `.execute(..)` on a ExecutorService
            //   java: INSTANCE.executors.execute(this.task);
        } else {
            (self.task)();
        }
        if self.cyclic {
            ({ let __r = INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.add_task()); __f(__r, self.clone(), (compat::read(INSTANCE.get_mut(), |__t| __t.scheduler_base().current_tick)).wrapping_add(self.interval)) });
        }
    }

}
