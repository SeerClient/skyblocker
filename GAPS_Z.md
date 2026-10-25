# GAPS - sweep Z (TODO_BATCH_Z packages)

Every `todo!()` and `// rust:` marker in batch Z's packages is gone. What could not be closed is marked at its
site as `NEEDS-GAME-API:` (a game hook point / API that does not exist yet) or `BLOCKED (cross-package)` (the
blocker is a file another batch owns), and listed here. Each row was re-checked against RustSrc / the mod tree.

## NEEDS-GAME-API - calls written against the API they need (these do NOT compile until the API lands)

| # | Java | Needed API (the call written at the site) | Site |
|---|---|---|---|
| Z1 | `AbstractPopupScreen.extractBackground`: `backgroundScreen.extractBackground(..)` / `.extractRenderState(..)` | `game::extract_underlying_screen_background(graphics, mx, my, a)` and `game::extract_underlying_screen_render_state(graphics, mx, my, a)` - draw the held parent (`set_screen_keeping_parent`) under the up mod screen | `utils/render/gui/abstract_popup_screen.rs` |
| Z2 | `RecipeBookComponent.pirateSpeechForThePeople("excitedze")` | `game::pirate_speech_for_the_people()` - park `recipe_book_component::pirate_speech_for_the_people(gfx, options)` for the frame loop | `skyblock/itemlist/recipebook/skyblock_recipe_book_component.rs` |

## NEEDS-GAME-API - documented only (no call can be written: the hook point / slot does not exist)

| # | Java | What the port has (verified) | Site |
|---|---|---|---|
| Z3 | `DropdownList.enableScissor` override | `ModList` has no `enable_scissor` slot; the list body uses its own box | `utils/render/gui/dropdown_widget.rs` |
| Z4 | `HeaderAndFooterLayout.addTitleHeader` | not ported; the body is inlined (a hook on it sees nothing) | `dwarven/profittrackers/corpse/corpse_profit_screen.rs` |
| Z5 | `instanceof CropBlock` | no class identity on `Block`; answered by `AGE_7` (exact for the Crop table) | `garden/greenhouse_paste.rs` |
| Z6 | 29 mixin handlers (`@Local` of `extractSlot`, `extractTooltip` tail, `onDone(&self)`, private `extract_bar`, unhooked `setBlock` / `setup_fog_with_fluid_in_camera` / `DataComponentGetter::get` / `isInvisibleTo` / `Options.dataFix` / `processLegacySkin` / `unpack_textures`, `ClientTooltipComponent` without `as_any`, hotbar `HotbarItem`, `RecipeBookComponent` subclassing, `lambda$useItem$0`, `Screen` attachments, the loading-screen background, `ServerReconfigScreen`, `LevelLoadingScreen#tick`) | each documented at its handler | `src/mixins/*.rs` |

## BLOCKED (cross-package / library)

| # | Java | Blocker | Site |
|---|---|---|---|
| Z7 | `DisableAll.disableEntries` (reflection over the config classes) | the config classes are being replaced by the generated merge (do-not-touch `config/configs/`); needs a field visitor there | `disable_all.rs` |
| Z8 | `GuiHelper.equipment(..)` PiP state | the only caller (`skyblock/item/custom/screen/trim_element_button.rs`) hands a `Model` value; the state takes the shared `AnimatedModel` handle | `utils/render/gui_helper.rs` |
| Z9 | Fabric `McVersionLookup` snapshot table | unreachable for this client's release id | `update_notifications.rs` (doc note) |

## Written without the library source (review)

* `compat::nea_lisp` (end of `compat.rs`, `// ---- sweep Z helpers ----`): NeaLisp (`moe.nea:nealisp`) is not on
  disk. The module is the library surface Skyblocker calls, with the core bindings the NEU `hotmlayout.json` uses
  (checked against the repo copy in `.minecraft/config/notenoughupdates/repo`: `defun`, `if`, `true`, `lt`, `gt`,
  `=`, `+ - * /`, `list.new`, `list.at`, atoms, strings, numbers). Errors are logged and answer `LispNil`.

## Closed in this sweep (were markers / todo!)

Profile viewer 2: the Gson model graph (`from_json` / `read_json_fields` on all 129 model structs, generated from
the `@SerializedName` docs), pages' widget layouts (`profile_viewer_page::layout_widget` / `collect_widgets`, a
page's widgets in a `Screen`), clicks and positions, every widget constructor, tooltips, the HOTM tree.
Dropdown: `Entry` reaches its widget through a `Weak<DropdownLink>` (`selected == entry`, `select(entry)`,
`formatter`), the list is drawn and has its class overrides. Credits screen music/sound ticks, museum styled
feedback, `ConfigSchema` recursive types, `ScreenUtils` underlying screen, the inventory-screen factory
(`MinecraftMixin`), ARGB input, colour picker, cycling icon button, grid component (`ComponentContents::Mod`),
grid tooltip text, atlas texture, title container, scissor `clip`, slider/popup sounds, corpse tracker
(reward storage, ALLOW_GAME, commands). Also: 7 `ALLOW_GAME` listeners, 6 command trees, 3 interaction callbacks,
the visitor helper widget + after-tick, ~44 system messages that dropped their click/hover events.
