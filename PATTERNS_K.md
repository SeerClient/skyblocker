# Batch K patterns

- LayoutSettings is Copy. `x.default_cell_setting().align_x()` silently does nothing. Write it back instead: `{ let s = x.default_cell_setting(); *s = (*s).align_x(); }`, or use compat::write for a shared value.
- ContainerListener: queue the change in an Arc<Mutex<Vec>> and drain the queue in the screen's lent tick/render. Never touch the screen from inside the listener.
- Per-screen Fabric input events become a global listener plus a SCREEN_ACCEPTED flag set in AFTER_INIT. Do not register a listener per init, which leaks.
- Autocomplete command nodes: LazyLock<Mutex<Option<Arc<CommandNode<FabricClientCommandSource>>>>>.
- A static read on worker or async threads must be a LazyLock, never compat::MainThread (CONFIG_DIR became PathStatic).
- An env var read with `.unwrap()` panics when it is missing. Use `.is_ok_and(..)`.
- A Java try/catch around a parse becomes catch_unwind or a Result check, never a bare unwrap.
- NumberUtils.toLong(s, d) becomes `s.parse::<i64>().unwrap_or(d)`.
- A slot-changed callback already carries the stack, so use it instead of handler.getSlot(i).getItem().
- AbstractContainerScreen.menu: keep the handler as a `menu` field on the screen.
