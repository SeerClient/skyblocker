# GAPS - batch P (burn-down: startup / tick-path todo!s)

Each gap was re-checked against RustSrc / the crate. RustSrc was not edited. Sites keep their `todo!` / `// rust:` marker.

| # | Java member / mechanism | What the port has | What is needed | Sites |
|---|---|---|---|---|
| 1 | `RenderPipelines.register(..)` and `DEBUG_FILLED_SNIPPET` / `LINES_SNIPPET` / `GUI_SNIPPET` / `GUI_TEXTURED_SNIPPET` | still private in `RustSrc/src/net/minecraft/client/renderer/render_pipelines.rs` (`fn register` :60, snippets :682-739), re-verified | `pub` register + snippets | `utils/render/skyblocker_render_pipelines.rs` (13 `todo!`). Reached only by `IrisCompatibility::assign_pipelines` (gated on `IRIS_ENABLED`) and by the primitive feature renderers, which are never dispatched (GAPS_N #9) |
| 2 | `net.azureaaron.networth.utils.PetConstants` (`RARITY_OFFSETS`, `PET_LEVELS`) | library not ported, no data in the crate | the two tables | `skyblock/pet_cache.rs` `on_message` (2 `todo!`). **Reached on every autopet chat message on SkyBlock - panics.** |
| 3 | JGit (`Git.open`, `stashCreate`, `fetch`, `reset`) | `crate::unresolved::Git` placeholder | a git client | `utils/neu_repo_manager.rs` `load_repository` (3 `todo!`). Reached at startup when the local repo dir exists; inside `catch_unwind`, so the panic is caught and logged as Java's catch would |
| 4 | NEU repo (`NEUItem.getSkyblockItemId/getMinecraftItemId/getLore`, `NEUConstants.getBazaarStocks`) | placeholders (GAPS_L/M) | the library | `utils/enchanted_book_utils.rs` `init` (4 `todo!`): inside `run_async_after_load`, which never runs (REPO_LOADING never completes), so not reached |
| 5 | Reflection over config fields (`getDeclaredFields`) and holding the game's `LiteralArgumentBuilder` | none | - | `config/config_commands.rs` (3 `todo!`); not reached: `init` registers no command tree |
