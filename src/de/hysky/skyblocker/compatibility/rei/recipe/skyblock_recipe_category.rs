//! Ported from de.hysky.skyblocker.compatibility.rei.recipe.SkyblockRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::rei::recipe::skyblock_recipe_display::SkyblockRecipeDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;

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
        // java: return CategoryIdentifier.of(identifier);
        return crate::de::hysky::skyblocker::compatibility::rei::rei_category_identifier_of(&self.identifier);
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
        // java: return EntryStacks.of(icon);
        return crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stack_renderer(crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stacks_of(&self.icon));
    }

    /// java: @Override public List<Widget> setupDisplay(SkyblockRecipeDisplay display, Rectangle bounds)
    pub fn setup_display(&self, display: &SkyblockRecipeDisplay, bounds: &crate::unresolved::Rectangle) -> Vec<crate::unresolved::Widget> {
        let mut out: Vec<crate::unresolved::Widget> = Vec::new();
        let b = crate::de::hysky::skyblocker::compatibility::rei::rei_rectangle(bounds);
        // java: out.add(Widgets.createRecipeBase(bounds));
        out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_recipe_base(bounds));
        // java: SkyblockRecipe recipe = display.getRecipe();
        let recipe: compat::Shared<dyn SkyblockRecipe> = display.get_recipe();
        // java: for (SkyblockRecipe.RecipeSlot inputSlot : recipe.getInputSlots(bounds.getWidth(), bounds.getHeight())) {
        //           out.add(Widgets.createSlot(new Point(inputSlot.x() + bounds.getX(), inputSlot.y() + bounds.getY())).markInput()
        //               .backgroundEnabled(inputSlot.showBackground()).entry(EntryStacks.of(inputSlot.stack().getStackOrThrow())));
        //       }
        let input_slots = { let __f = compat::dispatch(&recipe, |__t| __t.get_input_slots()); __f(&recipe, b.width, b.height) };
        for input_slot in input_slots {
            out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_slot_widget(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_slot(input_slot.x + b.x, input_slot.y + b.y), Some(true), Some(input_slot.show_background), crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stacks_of(&FlexibleItemStack::get_stack_or_throw(&input_slot.stack))));
        }
        // java: for (SkyblockRecipe.RecipeSlot outputSlot : recipe.getOutputSlots(bounds.getWidth(), bounds.getHeight())) { .. .markOutput() .. }
        let output_slots = { let __f = compat::dispatch(&recipe, |__t| __t.get_output_slots()); __f(&recipe, b.width, b.height) };
        for output_slot in output_slots {
            out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_slot_widget(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_slot(output_slot.x + b.x, output_slot.y + b.y), Some(false), Some(output_slot.show_background), crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stacks_of(&FlexibleItemStack::get_stack_or_throw(&output_slot.stack))));
        }
        // java: out.add(Widgets.createDrawableWidget((context, mouseX, mouseY, _) -> {
        //           Matrix3x2fStack matrices = context.pose(); matrices.pushMatrix(); matrices.translate(bounds.getX(), bounds.getY());
        //           recipe.extractRenderState(context, bounds.getWidth(), bounds.getHeight(), mouseX - bounds.getX(), mouseY - bounds.getY());
        //           matrices.popMatrix(); }));
        let drawn_recipe = compat::RenderThread::new(recipe.clone());
        out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_drawable_widget(move |context, mouse_x, mouse_y, _| {
            let matrices = mcrust::net::minecraft::client::gui::gui_graphics_extractor::push_pose();
            mcrust::net::minecraft::client::gui::gui_graphics_extractor::pose_translate(b.x as f32, b.y as f32);
            let recipe = drawn_recipe.get();
            { let __f = compat::dispatch(recipe, |__t| __t.extract_render_state()); __f(recipe, context, b.width, b.height, (mouse_x - b.x) as f64, (mouse_y - b.y) as f64) };
            mcrust::net::minecraft::client::gui::gui_graphics_extractor::pop_pose(matrices);
        }));
        // java: ScreenPosition arrowLocation = recipe.getArrowLocation(bounds.getWidth(), bounds.getHeight());
        let arrow_location: Option<ScreenPosition> = { let __f = compat::dispatch(&recipe, |__t| __t.get_arrow_location()); __f(&recipe, b.width, b.height) };
        if let Some(arrow_location) = arrow_location {
            // java: out.add(Widgets.createArrow(new Point(arrowLocation.x() + bounds.getX(), arrowLocation.y() + bounds.getY())));
            out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_arrow(arrow_location.x() + b.x, arrow_location.y() + b.y));
        }
        // java: out.add(Widgets.createLabel(new Point(bounds.getCenterX(), bounds.getCenterY() + 24), recipe.getExtraText()));
        let extra_text = { let __f = compat::dispatch(&recipe, |__t| __t.get_extra_text()); __f(&recipe) };
        out.push(crate::de::hysky::skyblocker::compatibility::rei::rei_widgets_create_label(b.get_center_x(), b.get_center_y() + 24, &extra_text));
        return out;
    }

}
