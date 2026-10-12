//! Ported from de.hysky.skyblocker.compatibility.rei.recipe.SkyblockRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::rei::recipe::skyblock_recipe_display::SkyblockRecipeDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class SkyblockRecipeCategory implements DisplayCategory<SkyblockRecipeDisplay>
#[derive(Clone)]
pub struct SkyblockRecipeCategory {
    /// java: private final Identifier identifier;
    pub identifier: Identifier,
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final int height;
    pub height: i32,
}

impl SkyblockRecipeCategory {
    /// java: public SkyblockRecipeCategory(Identifier identifier, Component title, ItemStack icon, int height)
    pub fn new(mut identifier: Identifier, mut title: MutableComponent, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut height: i32) -> Self {
        let __init_identifier = identifier.clone();
        let __init_title = title.clone();
        let __init_icon = icon.clone();
        let __init_height = height;
        let mut this = Self {
            identifier: __init_identifier,
            title: __init_title,
            icon: __init_icon,
            height: __init_height,
        };
        this
    }

    /// java: @Override public CategoryIdentifier<? extends SkyblockRecipeDisplay> getCategoryIdentifier()
    pub fn get_category_identifier(&self) -> crate::unresolved::CategoryIdentifier {
        // TODO(fabric2seer): static call CategoryIdentifier.of(..)
        //   java: return CategoryIdentifier.of(identifier);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int getDisplayHeight()
    pub fn get_display_height(&self) -> i32 {
        return self.height;
    }

    /// java: @Override public Component getTitle()
    pub fn get_title(&self) -> MutableComponent {
        return self.title.clone();
    }

    /// java: @Override public Renderer getIcon()
    pub fn get_icon(&self) -> crate::unresolved::Renderer {
        // TODO(fabric2seer): static call EntryStacks.of(..)
        //   java: return EntryStacks.of(icon);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public List<Widget> setupDisplay(SkyblockRecipeDisplay display, Rectangle bounds)
    pub fn setup_display(&self, display: &SkyblockRecipeDisplay, bounds: &crate::unresolved::Rectangle) -> Vec<crate::unresolved::Widget> {
        let mut out: Vec<crate::unresolved::Widget> = Vec::new();
        // TODO(fabric2seer): static call Widgets.createRecipeBase(..)
        //   java: out.add(Widgets.createRecipeBase(bounds));
        let mut recipe: compat::Shared<dyn SkyblockRecipe> = display.get_recipe();
        // TODO(fabric2seer): `.getWidth(..)` on a Rectangle
        //   java: for (SkyblockRecipe.RecipeSlot inputSlot : recipe.getInputSlots(bounds.getWidth(), bounds.getHeight())) {
        //   java: 			out.add(Widgets.createSlot(new Point(inputSlot.x() + bounds.getX(), inputSlot.y() + bounds.getY()))
        //   java: 					.markInput()
        //   java: 					.backgroundEnabled(inputSlot.showBackground())
        //   java: 					.entry(EntryStacks.of(inputSlot.stack().getStackOrThrow())));
        //   java: 		}
        // TODO(fabric2seer): `.getWidth(..)` on a Rectangle
        //   java: for (SkyblockRecipe.RecipeSlot outputSlot : recipe.getOutputSlots(bounds.getWidth(), bounds.getHeight())) {
        //   java: 			out.add(Widgets.createSlot(new Point(outputSlot.x() + bounds.getX(), outputSlot.y() + bounds.getY()))
        //   java: 					.markOutput()
        //   java: 					.backgroundEnabled(outputSlot.showBackground())
        //   java: 					.entry(EntryStacks.of(outputSlot.stack().getStackOrThrow())));
        //   java: 		}
        // TODO(fabric2seer): static call Widgets.createDrawableWidget(..)
        //   java: out.add(Widgets.createDrawableWidget((context, mouseX, mouseY, _) -> {
        //   java: 			Matrix3x2fStack matrices = context.pose();
        //   java: 			matrices.pushMatrix();
        //   java: 			matrices.translate(bounds.getX(), bounds.getY());
        //   java: 			recipe.extractRenderState(context, bounds.getWidth(), bounds.getHeight(), mouseX - bounds.getX(), mouseY - bounds.getY());
        //   java: 			matrices.popMatrix();
        //   java: 		}));
        // TODO(fabric2seer): `.getWidth(..)` on a Rectangle
        //   java: ScreenPosition arrowLocation = recipe.getArrowLocation(bounds.getWidth(), bounds.getHeight());
        let mut arrow_location: ScreenPosition = todo!("fabric2seer: the initialiser of `arrowLocation` (TODO above)");
        if true /* java: arrow_location != null: never null here */ {
            // TODO(fabric2seer): static call Widgets.createArrow(..)
            //   java: out.add(Widgets.createArrow(new Point(arrowLocation.x() + bounds.getX(), arrowLocation.y() + bounds.getY())));
        }
        // TODO(fabric2seer): static call Widgets.createLabel(..)
        //   java: out.add(Widgets.createLabel(new Point(bounds.getCenterX(), bounds.getCenterY() + 24), recipe.getExtraText()));
        return out;
    }

}
