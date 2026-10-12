# Skyblocker TODO burn-down brief (batches J, K, L, M)

The mod now TYPE-CHECKS (0 errors). What is left are the converter's placeholders: about
1,140 `todo!()` (each panics at run time) and 1,700 `// rust: ...` comments (a Java statement
that was not translated, or a port gap). Your job is to replace them with real code that does
what the Java does, in your batch's packages, without breaking the build.

Everything in `FIXUP_BRIEF.md` still applies (read it in full first): the hard-won rules
(never lock player/level/entity Mutexes - use game::with_player / level() / with_entity /
with_chest_menu; borrow compat::Shared per statement; no per-frame allocations; WidgetRef;
Weak in self-stored lambdas), the faithful-to-Java rule, the forbidden list, the GAPS and
PATTERNS files, no cargo/rustc/game (the orchestrator compiles), rustfmt --check allowed,
no history-changing git, no subagents, never kill processes.

## Where things are
- The original Java of each placeholder sits right above it in a `TODO(fabric2seer)` / `java:`
  comment; the full Java source is under
  `C:\Users\y64\AppData\Local\Temp\claude\c--Users-y64-Desktop-McRust\b63c15a5-b680-4c4e-b573-c89b7f994ba8\scratchpad\ext_skyblocker\repo\src\main\java\de\hysky\skyblocker\...`
- `todo!("fabric2seer: the initialiser of X ...")`, `the value of this function ...`,
  `the condition ...`, `the field value ...`, `constructor`: translate the Java above it.
- `// rust: <statement>` lines: a statement the converter wrote but could not make compile
  against the port. Make it real (the port usually has the API under another shape - grep
  RustSrc, MODDING.md "What Skyblocker's accessors map to", compat.rs, and the earlier
  batches' PATTERNS files), or confirm it is a genuine port gap (GAPS file, keep the marker).

## Ownership
- Your packages are listed in `TODO_BATCH_<X>.txt` (same one-segment rule as before: an entry
  like `skyblock/` or `utils/` owns only the files directly in that directory).
- `src/compat.rs`, `src/prelude.rs`, `src/unresolved.rs`, `src/lib.rs` belong to batch L.
  Others: private helpers in your own files, listed in your report.
- DO NOT TOUCH `config/categories/`, `config/configs/`, `config/skyblocker_config.rs`,
  `config/skyblocker_config_manager.rs`: they will be REPLACED by a generated merge (Dandelion /
  YACL types + generated Gson codecs). If your code needs something from them, note it.

## Keep it compiling
- After each file, `rustfmt --edition 2024 --check <file>`. The orchestrator type-checks every
  ~30 minutes and sends you `ERRORS_live_<X>.txt` if you broke something - fix those FIRST.
- If a replacement needs a signature change visible to another package, prefer a form that
  keeps callers compiling; otherwise list it in your report.

## Silent bugs to fix while you are there (compiler warnings in your files)
- `unreachable pattern` in a `match`: usually a Java `case CONST` translated as a lowercase
  binding that matches ANYTHING - every later arm is dead. Make it a real constant/path pattern.
- `function cannot return without recursing` (`utils/render/frustum_utils.rs:21`).
- `path statement with no effect`, `unused borrow that must be used`, `unused arithmetic
  operation`, `value assigned is never read`: often a dropped statement or a write to a copy -
  check against the Java.

## Final report
Placeholders replaced (todo!/rust: counts before -> after in your packages); genuine port
gaps found (GAPS_<X>.md); silent bugs fixed; patterns (PATTERNS_<X>.md); signature changes;
helpers for compat.rs; uncertain spots.
