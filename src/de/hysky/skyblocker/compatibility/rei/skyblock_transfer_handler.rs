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
        // TODO(fabric2seer): `.getDisplay(..)` on a Context
        //   java: CategoryIdentifier<?> identifier = context.getDisplay().getCategoryIdentifier();
        let mut identifier: crate::unresolved::CategoryIdentifier = todo!("fabric2seer: the initialiser of `identifier` (TODO above)");
        // TODO(fabric2seer): the condition: static call CategoryIdentifier.of(..)
        //   java: identifier != CategoryIdentifier.of(SkyblockCraftingRecipe.ID) && identifier != CategoryIdentifier.of(SkyblockInfoCategory.IDENTIFIER)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): static call ApplicabilityResult.createNotApplicable(..)
            //   java: return ApplicabilityResult.createNotApplicable();
        }
        // TODO(fabric2seer): `.getDisplay(..)` on a Context
        //   java: EntryIngredient ingredient = context.getDisplay().getOutputEntries().getFirst();
        let mut ingredient: crate::unresolved::EntryIngredient = todo!("fabric2seer: the initialiser of `ingredient` (TODO above)");
        // TODO(fabric2seer): `.getFirst(..)` on a EntryIngredient
        //   java: EntryStack<?> entryStack = ingredient.getFirst();
        let mut entry_stack: crate::unresolved::EntryStack = todo!("fabric2seer: the initialiser of `entryStack` (TODO above)");
        // TODO(fabric2seer): `.getValue(..)` on a EntryStack
        //   java: if (!(entryStack.getValue() instanceof ItemStack itemStack))
        //   java: 			return ApplicabilityResult.createNotApplicable();
        // TODO(fabric2seer): unknown name `itemStack`
        //   java: String neuId = itemStack.getNeuName();
        let mut neu_id: String = todo!("fabric2seer: the initialiser of `neuId` (TODO above)");
        if !(NEURepoManager::get_recipes().contains_key(&neu_id)) || !NEURepoManager::get_recipes().get(&neu_id).cloned().unwrap().iter().cloned().any(|recipe| recipe.as_any().is::<crate::unresolved::NEUCraftingRecipe>()) {
            // TODO(fabric2seer): static call ApplicabilityResult.createApplicableWithError(..)
            //   java: return ApplicabilityResult.createApplicableWithError(Component.translatable("skyblocker.rei.transfer.noRecipe"));
        }
        // TODO(fabric2seer): static call ApplicabilityResult.createApplicable(..)
        //   java: return ApplicabilityResult.createApplicable();
        todo!("fabric2seer: the value of this function is in a TODO above")
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
        let mut current_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen = compat::minecraft().gui.screen().unwrap();
        if !(current_screen.as_any().is::<ContainerScreen>()) {
            self.fail_count = 0;
            self.failed_item = skyblock_id.clone();
        }
    }

    /// java: @Override public Result handle(Context context)
    pub fn handle(&mut self, context: &crate::unresolved::Context) -> crate::unresolved::Result {
        // TODO(fabric2seer): `.getDisplay(..)` on a Context
        //   java: EntryIngredient ingredient = context.getDisplay().getOutputEntries().getFirst();
        let mut ingredient: crate::unresolved::EntryIngredient = todo!("fabric2seer: the initialiser of `ingredient` (TODO above)");
        // TODO(fabric2seer): `.getFirst(..)` on a EntryIngredient
        //   java: EntryStack<?> entryStack = ingredient.getFirst();
        let mut entry_stack: crate::unresolved::EntryStack = todo!("fabric2seer: the initialiser of `entryStack` (TODO above)");
        // TODO(fabric2seer): `.getValue(..)` on a EntryStack
        //   java: if (!(entryStack.getValue() instanceof ItemStack itemStack)) return Result.createNotApplicable();
        // TODO(fabric2seer): unknown name `itemStack`
        //   java: String skyblockId = itemStack.getSkyblockId();
        let mut skyblock_id: String = todo!("fabric2seer: the initialiser of `skyblockId` (TODO above)");
        if self.has_failed(&skyblock_id) {
            // TODO(fabric2seer): static call Result.createFailed(..)
            //   java: return Result.createFailed(Component.translatable("skyblocker.rei.transfer.failed"));
        }
        // TODO(fabric2seer): the condition: `.isActuallyCrafting(..)` on a Context
        //   java: !context.isActuallyCrafting()
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): static call Result.createSuccessful(..)
            //   java: return Result.createSuccessful();
        }
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/viewrecipe {}", skyblock_id), false);
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { self.check_screen(skyblock_id.clone()); }), 5) });
        // TODO(fabric2seer): static call Result.createSuccessful(..)
        //   java: return Result.createSuccessful();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
