//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.SkyblockKatUpgradeRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe::CenteredRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::RecipeSlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final Identifier ID = SkyblockerMod.id("skyblock_kat_upgrade");
pub static ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("skyblock_kat_upgrade"));
/// java: public static @Nullable FlexibleItemStack katIcon = null;
pub static KAT_ICON: compat::MainThread<Option<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| None);

/// java: public class SkyblockKatUpgradeRecipe implements CenteredRecipe
#[derive(Clone)]
pub struct SkyblockKatUpgradeRecipe {
    /// java: private final FlexibleItemStack basePet;
    pub base_pet: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final List<FlexibleItemStack> inputs;
    pub inputs: Vec<compat::Shared<FlexibleItemStack>>,
    /// java: private final FlexibleItemStack upgradedPet;
    pub upgraded_pet: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final Component upgradeTime;
    pub upgrade_time: Option<MutableComponent>,
}

impl SkyblockKatUpgradeRecipe {
    /// java: public SkyblockKatUpgradeRecipe(NEUKatUpgradeRecipe recipe)
    pub fn new(mut recipe: crate::unresolved::NEUKatUpgradeRecipe) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base_pet: None,
            inputs: Default::default(),
            upgraded_pet: None,
            upgrade_time: None,
        });
        if KAT_ICON.get_mut().is_none() {
            *KAT_ICON.get_mut() = ItemRepository::get_item_stack("KAT_NPC");
        }
        // TODO(fabric2seer): `.getSeconds(..)` on a NEUKatUpgradeRecipe
        //   java: upgradeTime = SkyblockTime.formatTime(recipe.getSeconds());
        // TODO(fabric2seer): `.getInput(..)` on a NEUKatUpgradeRecipe
        //   java: basePet = SkyblockRecipe.getItemStack(recipe.getInput());
        // TODO(fabric2seer): `.getOutput(..)` on a NEUKatUpgradeRecipe
        //   java: upgradedPet = SkyblockRecipe.getItemStack(recipe.getOutput());
        { let __v = Vec::new(); compat::write(&__this, |__t| __t.inputs = __v); }
        let __h0 = compat::read(&__this, |__t| __t.base_pet.clone()).unwrap();
        __this.lock().borrow_mut().inputs.push(__h0);
        // TODO(fabric2seer): `.getItems(..)` on a NEUKatUpgradeRecipe
        //   java: recipe.getItems().stream().map(SkyblockRecipe::getItemStack).forEach(inputs::add);
        // TODO(fabric2seer): static call NEUIngredient.ofCoins(..)
        //   java: inputs.add(SkyblockRecipe.getItemStack(NEUIngredient.ofCoins(recipe.getCoins())));
        __this
    }

}

impl CenteredRecipe for SkyblockKatUpgradeRecipe {
    /// java: @Override public FlexibleItemStack getIcon() (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> compat::Shared<FlexibleItemStack> {
        SkyblockKatUpgradeRecipe::get_icon_dyn
    }

    /// java: @Override public @Nullable FlexibleItemStack getRepresentative() (dispatch: the body for this class)
    fn get_representative(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> Option<compat::Shared<FlexibleItemStack>> {
        SkyblockKatUpgradeRecipe::get_representative_dyn
    }

}

impl SkyblockRecipe for SkyblockKatUpgradeRecipe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height) (dispatch: the body for this class)
    fn get_input_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockKatUpgradeRecipe::get_input_slots_dyn
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height) (dispatch: the body for this class)
    fn get_output_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockKatUpgradeRecipe::get_output_slots_dyn
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height) (dispatch: the body for this class)
    fn get_arrow_location(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Option<ScreenPosition> {
        SkyblockKatUpgradeRecipe::get_arrow_location_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getInputs() (dispatch: the body for this class)
    fn get_inputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockKatUpgradeRecipe::get_inputs_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs() (dispatch: the body for this class)
    fn get_outputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockKatUpgradeRecipe::get_outputs_dyn
    }

    /// java: @Override public Component getExtraText() (dispatch: the body for this class)
    fn get_extra_text(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        SkyblockKatUpgradeRecipe::get_extra_text_dyn
    }

    /// java: @Override public Identifier getCategoryIdentifier() (dispatch: the body for this class)
    fn get_category_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockKatUpgradeRecipe::get_category_identifier_dyn
    }

    /// java: @Override public Identifier getRecipeIdentifier() (dispatch: the body for this class)
    fn get_recipe_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockKatUpgradeRecipe::get_recipe_identifier_dyn
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, &GuiGraphicsExtractor, i32, i32, f64, f64) {
        SkyblockKatUpgradeRecipe::extract_render_state_dyn
    }

}

impl SkyblockKatUpgradeRecipe {
    /// java: @Override public FlexibleItemStack getIcon()
    pub fn get_icon_impl(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return ico::BONE.get_mut().clone();
    }

    /// SkyblockKatUpgradeRecipe.getIcon as the dispatch of CenteredRecipe.getIcon
    pub fn get_icon_dyn(__this: &compat::Shared<dyn CenteredRecipe>) -> compat::Shared<FlexibleItemStack> {
        Self::get_icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @Nullable FlexibleItemStack getRepresentative()
    pub fn get_representative_impl(__this: &compat::Shared<Self>) -> Option<compat::Shared<FlexibleItemStack>> {
        return KAT_ICON.get_mut().clone();
    }

    /// SkyblockKatUpgradeRecipe.getRepresentative as the dispatch of CenteredRecipe.getRepresentative
    pub fn get_representative_dyn(__this: &compat::Shared<dyn CenteredRecipe>) -> Option<compat::Shared<FlexibleItemStack>> {
        Self::get_representative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height)
    pub fn get_input_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        return centered_recipe::arrange_inputs(width, height, None, &compat::read(__this, |__t| __t.inputs.clone()));
    }

    /// SkyblockKatUpgradeRecipe.getInputSlots as the dispatch of SkyblockRecipe.getInputSlots
    pub fn get_input_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_input_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height)
    pub fn get_output_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        return centered_recipe::arrange_outputs(width, height, true, (__this.lock().borrow().inputs.len() as i32), &compat::read(__this, |__t| __t.upgraded_pet.clone()).unwrap());
    }

    /// SkyblockKatUpgradeRecipe.getOutputSlots as the dispatch of SkyblockRecipe.getOutputSlots
    pub fn get_output_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_output_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height)
    pub fn get_arrow_location_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Option<ScreenPosition> {
        return Some(centered_recipe::get_arrow_location(width, height, true, (__this.lock().borrow().inputs.len() as i32)));
    }

    /// SkyblockKatUpgradeRecipe.getArrowLocation as the dispatch of SkyblockRecipe.getArrowLocation
    pub fn get_arrow_location_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Option<ScreenPosition> {
        Self::get_arrow_location_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<FlexibleItemStack> getInputs()
    pub fn get_inputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.inputs.clone());
    }

    /// SkyblockKatUpgradeRecipe.getInputs as the dispatch of SkyblockRecipe.getInputs
    pub fn get_inputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_inputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs()
    pub fn get_outputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return vec![compat::read(__this, |__t| __t.upgraded_pet.clone()).unwrap()];
    }

    /// SkyblockKatUpgradeRecipe.getOutputs as the dispatch of SkyblockRecipe.getOutputs
    pub fn get_outputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_outputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component getExtraText()
    pub fn get_extra_text_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return component::empty();
    }

    /// SkyblockKatUpgradeRecipe.getExtraText as the dispatch of SkyblockRecipe.getExtraText
    pub fn get_extra_text_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        Self::get_extra_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getCategoryIdentifier()
    pub fn get_category_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return ID.get_mut().clone();
    }

    /// SkyblockKatUpgradeRecipe.getCategoryIdentifier as the dispatch of SkyblockRecipe.getCategoryIdentifier
    pub fn get_category_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_category_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getRecipeIdentifier()
    pub fn get_recipe_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return Identifier::from_namespace_and_path("skyblock", &compat::read(&compat::read(__this, |__t| __t.upgraded_pet.clone()).unwrap(), |__s| __s.get_skyblock_id()).to_lowercase().replace(';', &'_'.to_string()));
    }

    /// SkyblockKatUpgradeRecipe.getRecipeIdentifier as the dispatch of SkyblockRecipe.getRecipeIdentifier
    pub fn get_recipe_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_recipe_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut width: i32, mut height: i32, mut mouse_x: f64, mut mouse_y: f64) {
        let Some(mut arrow_location) = ({ let __f = compat::dispatch(__this, |__t| __t.get_arrow_location()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipe>), width, height) }) else {
            return;
        };
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.upgrade_time.clone()).unwrap()), arrow_location.x().wrapping_add(12), arrow_location.y().wrapping_sub(10), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// SkyblockKatUpgradeRecipe.extractRenderState as the dispatch of SkyblockRecipe.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f64, __p4: f64) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}
