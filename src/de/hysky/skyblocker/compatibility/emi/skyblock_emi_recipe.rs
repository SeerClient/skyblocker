//! Ported from de.hysky.skyblocker.compatibility.emi.SkyblockEmiRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::emi::skyblocker_emi_plugin;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class SkyblockEmiRecipe implements EmiRecipe
#[derive(Clone)]
pub struct SkyblockEmiRecipe {
    /// java: private final Component craftText;
    pub craft_text: MutableComponent,
    /// java: private final SkyblockRecipe recipe;
    pub recipe: compat::Shared<dyn SkyblockRecipe>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl SkyblockEmiRecipe {
    /// java: public SkyblockEmiRecipe(SkyblockRecipe recipe)
    pub fn new(mut recipe: compat::Shared<dyn SkyblockRecipe>) -> compat::Shared<Self> {
        let __init_craft_text = ({ let __f = compat::dispatch(&recipe, |__t| __t.get_extra_text()); __f(&recipe) });
        let __init_recipe = recipe.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            craft_text: __init_craft_text,
            recipe: __init_recipe,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override public EmiRecipeCategory getCategory()
    pub fn get_category(__this: &compat::Shared<Self>) -> crate::unresolved::EmiRecipeCategory {
        return skyblocker_emi_plugin::IDENTIFIER_CATEGORY_MAP.get_mut().get(&({ let __r = &compat::read(__this, |__t| __t.recipe.clone()); let __f = compat::dispatch(__r, |__t| __t.get_category_identifier()); __f(__r) })).cloned().unwrap();
    }

    /// java: @Override public @Nullable Identifier getId()
    pub fn get_id(__this: &compat::Shared<Self>) -> Option<Identifier> {
        return Some(({ let __r = &compat::read(__this, |__t| __t.recipe.clone()); let __f = compat::dispatch(__r, |__t| __t.get_recipe_identifier()); __f(__r) }));
    }

    /// java: @Override public List<EmiIngredient> getInputs()
    pub fn get_inputs(__this: &compat::Shared<Self>) -> Vec<crate::unresolved::EmiIngredient> {
        // TODO(fabric2seer): `of` is not a method of FlexibleItemStack
        //   java: return recipe.getInputs().stream().map(EmiStack::of).map(EmiIngredient.class::cast).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public List<EmiStack> getOutputs()
    pub fn get_outputs(__this: &compat::Shared<Self>) -> Vec<crate::unresolved::EmiStack> {
        // TODO(fabric2seer): `of` is not a method of FlexibleItemStack
        //   java: return recipe.getOutputs().stream().map(EmiStack::of).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int getDisplayWidth()
    pub fn get_display_width(__this: &compat::Shared<Self>) -> i32 {
        return 118;
    }

    /// java: @Override public int getDisplayHeight()
    pub fn get_display_height(__this: &compat::Shared<Self>) -> i32 {
        return 54i32.wrapping_add(if __this.lock().borrow().craft_text.get_string().is_empty() { 0 } else { 10 });
    }

    /// java: @Override public void addWidgets(WidgetHolder widgets)
    pub fn add_widgets(__this: &compat::Shared<Self>, widgets: &crate::unresolved::WidgetHolder) {
        let mut arrow_location: Option<ScreenPosition> = ({ let __r = &compat::read(__this, |__t| __t.recipe.clone()); let __f = compat::dispatch(__r, |__t| __t.get_arrow_location()); __f(__r, SkyblockEmiRecipe::get_display_width(__this), SkyblockEmiRecipe::get_display_height(__this)) });
        if let Some(arrow_location) = &arrow_location {
            // TODO(fabric2seer): `.addTexture(..)` on a WidgetHolder
            //   java: widgets.addTexture(EmiTexture.EMPTY_ARROW, arrowLocation.x(), arrowLocation.y());
        }
        // TODO(fabric2seer): `.addText(..)` on a WidgetHolder
        //   java: widgets.addText(craftText, 59 - Minecraft.getInstance().font.width(craftText) / 2, 55, 0xFFFFFF, true);
        for input_slot in ({ let __r = &compat::read(__this, |__t| __t.recipe.clone()); let __f = compat::dispatch(__r, |__t| __t.get_input_slots()); __f(__r, SkyblockEmiRecipe::get_display_width(__this), SkyblockEmiRecipe::get_display_height(__this)) }) {
            // TODO(fabric2seer): `.addSlot(..)` on a WidgetHolder
            //   java: widgets.addSlot(EmiStack.of(inputSlot.stack()), inputSlot.x(), inputSlot.y());
        }
        for output_slot in ({ let __r = &compat::read(__this, |__t| __t.recipe.clone()); let __f = compat::dispatch(__r, |__t| __t.get_output_slots()); __f(__r, SkyblockEmiRecipe::get_display_width(__this), SkyblockEmiRecipe::get_display_height(__this)) }) {
            // TODO(fabric2seer): `.addSlot(..)` on a WidgetHolder
            //   java: widgets.addSlot(EmiStack.of(outputSlot.stack()), outputSlot.x(), outputSlot.y()).recipeContext(this);
        }
        // TODO(fabric2seer): `.addDrawable(..)` on a WidgetHolder
        //   java: widgets.addDrawable(0, 0, getDisplayWidth(), getDisplayHeight(), (draw, mouseX, mouseY, delta) -> recipe.render(draw, getDisplayWidth(), getDisplayHeight(), mouseX, mouseY));
    }

}
