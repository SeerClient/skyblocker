//! Ported from de.hysky.skyblocker.compatibility.rei.SkyblockTransferHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;

/// java: private static final int MAX_FAIL_COUNT = 5;
pub const MAX_FAIL_COUNT: i32 = 5;

/// java: public class SkyblockTransferHandler implements TransferHandler
#[derive(Clone)]
pub struct SkyblockTransferHandler {
    /// java: private String FAILED_ITEM;
    pub failed_item: String,
    /// java: private int FAIL_COUNT;
    pub fail_count: i32,
}

impl SkyblockTransferHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            failed_item: Default::default(),
            fail_count: 0,
        };
        this
    }

    /// java: @Override public ApplicabilityResult checkApplicable(Context context)
    pub fn check_applicable(&self, context: &crate::unresolved::Context) -> crate::unresolved::ApplicabilityResult {
        //   java: CategoryIdentifier<?> identifier = context.getDisplay().getCategoryIdentifier();
        let identifier: Option<String> = rei_display_category(context);
        //   java: if (identifier != CategoryIdentifier.of(SkyblockCraftingRecipe.ID) && identifier != CategoryIdentifier.of(SkyblockInfoCategory.IDENTIFIER))
        let crafting: String = crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::ID.get().to_string();
        let info: String = crate::de::hysky::skyblocker::compatibility::rei::info::skyblock_info_category::IDENTIFIER.get().to_string();
        if identifier.as_deref() != Some(crafting.as_str()) && identifier.as_deref() != Some(info.as_str()) {
            //   java: return ApplicabilityResult.createNotApplicable();
            return rei_applicability_result();
        }
        //   java: EntryIngredient ingredient = context.getDisplay().getOutputEntries().getFirst();
        //   java: EntryStack<?> entryStack = ingredient.getFirst();
        //   java: if (!(entryStack.getValue() instanceof ItemStack itemStack)) return ApplicabilityResult.createNotApplicable();
        let Some(item_stack) = rei_display_output_item(context) else {
            return rei_applicability_result();
        };
        //   java: String neuId = itemStack.getNeuName();
        let neu_id: String = item_stack.get_neu_name();
        if !(NEURepoManager::get_recipes().contains_key(&neu_id)) || !NEURepoManager::get_recipes().get(&neu_id).unwrap().iter().any(|recipe| recipe.is::<crate::io::github::moulberry::repo::data::neu_crafting_recipe::NEUCraftingRecipe>()) {
            //   java: return ApplicabilityResult.createApplicableWithError(Component.translatable("skyblocker.rei.transfer.noRecipe"));
            return rei_applicability_result();
        }
        //   java: return ApplicabilityResult.createApplicable();
        return rei_applicability_result();
    }

    /// java: private boolean hasFailed(String skyblockId)
    pub fn has_failed(&mut self, skyblock_id: &str) -> bool {
        if skyblock_id == self.failed_item {
            if self.fail_count < MAX_FAIL_COUNT {
                self.fail_count = self.fail_count.wrapping_add(1);
                return true;
            }
            self.failed_item = "".to_string();
            self.fail_count = 0;
        }
        return false;
    }

    /// java: private void checkScreen(String skyblockId)
    pub fn check_screen(&mut self, mut skyblock_id: String) {
        // `currentScreen instanceof ContainerScreen`: the screen's kind
        if !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
            self.fail_count = 0;
            self.failed_item = skyblock_id.clone();
        }
    }

    /// java: @Override public Result handle(Context context)
    pub fn handle(__this: &compat::Shared<Self>, context: &crate::unresolved::Context) -> crate::unresolved::Result {
        //   java: EntryIngredient ingredient = context.getDisplay().getOutputEntries().getFirst();
        //   java: EntryStack<?> entryStack = ingredient.getFirst();
        //   java: if (!(entryStack.getValue() instanceof ItemStack itemStack)) return Result.createNotApplicable();
        let Some(item_stack) = rei_display_output_item(context) else {
            return rei_result();
        };
        //   java: String skyblockId = itemStack.getSkyblockId();
        let skyblock_id: String = item_stack.get_skyblock_id();
        if compat::write(__this, |__t| __t.has_failed(&skyblock_id)) {
            //   java: return Result.createFailed(Component.translatable("skyblocker.rei.transfer.failed"));
            return rei_result();
        }
        //   java: if (!context.isActuallyCrafting()) return Result.createSuccessful();
        if !rei_is_actually_crafting(context) {
            return rei_result();
        }
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/viewrecipe {}", skyblock_id), false);
        // java: Scheduler.INSTANCE.schedule(() -> checkScreen(skyblockId), 5);
        let __this_1 = __this.clone();
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { let __id = skyblock_id.clone(); compat::write(&__this_1, |__t| __t.check_screen(__id)); }), 5) });
        //   java: return Result.createSuccessful();
        return rei_result();
    }

}

// The REI accessors this handler reads.
//
// REI (`me.shedaniel.rei`) is a separate mod that SeerClient never loads, so this handler is only reached when REI
// hands it a `Context` - which does not happen. Its API types are the `crate::unresolved` names; the accessors below
// are REI's side of the boundary and answer what a context without a display answers: no category, no output
// entry, not crafting; the result value stands for each `create*()` answer.

/// `context.getDisplay().getCategoryIdentifier()` as its identifier's string.
fn rei_display_category(context: &crate::unresolved::Context) -> Option<String> {
    let _ = context;
    None
}

/// `context.getDisplay().getOutputEntries().getFirst().getFirst().getValue()` when it is an `ItemStack`.
fn rei_display_output_item(context: &crate::unresolved::Context) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
    let _ = context;
    None
}

/// `context.isActuallyCrafting()`.
fn rei_is_actually_crafting(context: &crate::unresolved::Context) -> bool {
    let _ = context;
    false
}

/// `ApplicabilityResult.create*()`.
fn rei_applicability_result() -> crate::unresolved::ApplicabilityResult {
    crate::unresolved::ApplicabilityResult
}

/// `Result.create*()`.
fn rei_result() -> crate::unresolved::Result {
    crate::unresolved::Result
}
