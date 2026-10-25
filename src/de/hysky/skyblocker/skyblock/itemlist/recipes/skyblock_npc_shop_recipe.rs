//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.SkyblockNpcShopRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe::CenteredRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::RecipeSlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final Identifier ID = SkyblockerMod.id("skyblock_npc_shop");
pub static ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("skyblock_npc_shop"));

/// java: public class SkyblockNpcShopRecipe implements CenteredRecipe
#[derive(Clone)]
pub struct SkyblockNpcShopRecipe {
    /// java: private final FlexibleItemStack npcShop; (null when the item repository has no stack for the NPC)
    pub npc_shop: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final List<FlexibleItemStack> inputs;
    pub inputs: Vec<compat::Shared<FlexibleItemStack>>,
    /// java: private final FlexibleItemStack output;
    pub output: compat::Shared<FlexibleItemStack>,
}

impl SkyblockNpcShopRecipe {
    /// java: public SkyblockNpcShopRecipe(NEUNpcShopRecipe shopRecipe)
    pub fn new(mut shop_recipe: crate::io::github::moulberry::repo::data::neu_npc_shop_recipe::NEUNpcShopRecipe) -> compat::Shared<Self> {
        //   java: npcShop = ItemRepository.getItemStack(shopRecipe.getIsSoldBy().getSkyblockItemId());
        let __init_npc_shop: Option<compat::Shared<FlexibleItemStack>> = crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_item_stack(
            shop_recipe.get_is_sold_by().expect("java.lang.NullPointerException: NEUNpcShopRecipe.isSoldBy").get_skyblock_item_id(),
        );
        //   java: inputs = shopRecipe.getCost().stream().map(SkyblockRecipe::getItemStack).toList();
        let __init_inputs: Vec<compat::Shared<FlexibleItemStack>> = shop_recipe.get_cost().iter().map(|__a0| crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(__a0)).collect();
        //   java: output = SkyblockRecipe.getItemStack(shopRecipe.getResult());
        let __init_output: compat::Shared<FlexibleItemStack> = crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(shop_recipe.get_result().expect("java.lang.NullPointerException: the NEU recipe has no result"));
        let __this = compat::shared(Self {
            npc_shop: __init_npc_shop,
            inputs: __init_inputs,
            output: __init_output,
        });
        __this
    }

}

impl CenteredRecipe for SkyblockNpcShopRecipe {
    /// java: @Override public FlexibleItemStack getRepresentative() (dispatch: the body for this class)
    fn get_representative(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> Option<compat::Shared<FlexibleItemStack>> {
        SkyblockNpcShopRecipe::get_representative_dyn
    }

    /// java: @Override public FlexibleItemStack getIcon() (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> compat::Shared<FlexibleItemStack> {
        SkyblockNpcShopRecipe::get_icon_dyn
    }

}

impl SkyblockRecipe for SkyblockNpcShopRecipe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height) (dispatch: the body for this class)
    fn get_input_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockNpcShopRecipe::get_input_slots_dyn
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height) (dispatch: the body for this class)
    fn get_output_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockNpcShopRecipe::get_output_slots_dyn
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height) (dispatch: the body for this class)
    fn get_arrow_location(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Option<ScreenPosition> {
        SkyblockNpcShopRecipe::get_arrow_location_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getInputs() (dispatch: the body for this class)
    fn get_inputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockNpcShopRecipe::get_inputs_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs() (dispatch: the body for this class)
    fn get_outputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockNpcShopRecipe::get_outputs_dyn
    }

    /// java: @Override public Component getExtraText() (dispatch: the body for this class)
    fn get_extra_text(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        SkyblockNpcShopRecipe::get_extra_text_dyn
    }

    /// java: @Override public Identifier getCategoryIdentifier() (dispatch: the body for this class)
    fn get_category_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockNpcShopRecipe::get_category_identifier_dyn
    }

    /// java: @Override public Identifier getRecipeIdentifier() (dispatch: the body for this class)
    fn get_recipe_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockNpcShopRecipe::get_recipe_identifier_dyn
    }

}

impl SkyblockNpcShopRecipe {
    /// java: @Override public FlexibleItemStack getRepresentative()
    pub fn get_representative_impl(__this: &compat::Shared<Self>) -> Option<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.npc_shop.clone());
    }

    /// SkyblockNpcShopRecipe.getRepresentative as the dispatch of CenteredRecipe.getRepresentative
    pub fn get_representative_dyn(__this: &compat::Shared<dyn CenteredRecipe>) -> Option<compat::Shared<FlexibleItemStack>> {
        Self::get_representative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public FlexibleItemStack getIcon()
    pub fn get_icon_impl(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return ico::GOLD_NUGGET.get_mut().clone();
    }

    /// SkyblockNpcShopRecipe.getIcon as the dispatch of CenteredRecipe.getIcon
    pub fn get_icon_dyn(__this: &compat::Shared<dyn CenteredRecipe>) -> compat::Shared<FlexibleItemStack> {
        Self::get_icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height)
    pub fn get_input_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        return centered_recipe::arrange_inputs(width, height, compat::read(__this, |__t| __t.npc_shop.clone()), &compat::read(__this, |__t| __t.inputs.clone()));
    }

    /// SkyblockNpcShopRecipe.getInputSlots as the dispatch of SkyblockRecipe.getInputSlots
    pub fn get_input_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_input_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height)
    pub fn get_output_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        return centered_recipe::arrange_outputs(width, height, false, (__this.lock().borrow().inputs.len() as i32), &compat::read(__this, |__t| __t.output.clone()));
    }

    /// SkyblockNpcShopRecipe.getOutputSlots as the dispatch of SkyblockRecipe.getOutputSlots
    pub fn get_output_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_output_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height)
    pub fn get_arrow_location_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Option<ScreenPosition> {
        return Some(centered_recipe::get_arrow_location(width, height, false, (__this.lock().borrow().inputs.len() as i32)));
    }

    /// SkyblockNpcShopRecipe.getArrowLocation as the dispatch of SkyblockRecipe.getArrowLocation
    pub fn get_arrow_location_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Option<ScreenPosition> {
        Self::get_arrow_location_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<FlexibleItemStack> getInputs()
    pub fn get_inputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.inputs.clone());
    }

    /// SkyblockNpcShopRecipe.getInputs as the dispatch of SkyblockRecipe.getInputs
    pub fn get_inputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_inputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs()
    pub fn get_outputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return vec![compat::read(__this, |__t| __t.output.clone())];
    }

    /// SkyblockNpcShopRecipe.getOutputs as the dispatch of SkyblockRecipe.getOutputs
    pub fn get_outputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_outputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component getExtraText()
    pub fn get_extra_text_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return component::empty();
    }

    /// SkyblockNpcShopRecipe.getExtraText as the dispatch of SkyblockRecipe.getExtraText
    pub fn get_extra_text_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        Self::get_extra_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getCategoryIdentifier()
    pub fn get_category_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return ID.get_mut().clone();
    }

    /// SkyblockNpcShopRecipe.getCategoryIdentifier as the dispatch of SkyblockRecipe.getCategoryIdentifier
    pub fn get_category_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_category_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getRecipeIdentifier()
    pub fn get_recipe_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return Identifier::from_namespace_and_path("skyblock", &format!("{}_{}", compat::read(&compat::read(__this, |__t| __t.output.clone()), |__s| __s.get_skyblock_id()).to_lowercase().replace(';', &'_'.to_string()), FlexibleItemStack::count(&compat::read(__this, |__t| __t.output.clone()))));
    }

    /// SkyblockNpcShopRecipe.getRecipeIdentifier as the dispatch of SkyblockRecipe.getRecipeIdentifier
    pub fn get_recipe_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_recipe_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
