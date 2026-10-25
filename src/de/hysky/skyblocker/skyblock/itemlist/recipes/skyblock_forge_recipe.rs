//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.SkyblockForgeRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::RecipeSlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::vector2i::Vector2i;

/// java: public static final Identifier ID = SkyblockerMod.id("skyblock_forge");
pub static ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("skyblock_forge"));

/// java: public class SkyblockForgeRecipe implements SkyblockRecipe
#[derive(Clone)]
pub struct SkyblockForgeRecipe {
    /// java: private final List<FlexibleItemStack> inputs;
    pub inputs: Vec<compat::Shared<FlexibleItemStack>>,
    /// java: private final FlexibleItemStack output;
    pub output: compat::Shared<FlexibleItemStack>,
    /// java: private final String durationString;
    pub duration_string: String,
}

impl SkyblockForgeRecipe {
    /// java: public SkyblockForgeRecipe(NEUForgeRecipe forgeRecipe)
    pub fn new(mut forge_recipe: crate::io::github::moulberry::repo::data::neu_forge_recipe::NEUForgeRecipe) -> compat::Shared<Self> {
        //   java: inputs = forgeRecipe.getInputs().stream().map(SkyblockRecipe::getItemStack).toList();
        let __init_inputs: Vec<compat::Shared<FlexibleItemStack>> = forge_recipe.get_inputs().iter().map(|__a0| crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(__a0)).collect();
        //   java: output = SkyblockRecipe.getItemStack(forgeRecipe.getOutputStack());
        let __init_output: compat::Shared<FlexibleItemStack> = crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(forge_recipe.get_output_stack().expect("java.lang.NullPointerException: the NEU recipe has no output stack"));
        //   java: int duration = forgeRecipe.getDuration();
        let mut duration: i32 = forge_recipe.get_duration();
        let __init_duration_string = SkyblockTime::format_time_string((duration) as f64);
        let __this = compat::shared(Self {
            inputs: __init_inputs,
            output: __init_output,
            duration_string: __init_duration_string,
        });
        __this
    }

    /// java: public FlexibleItemStack getResult()
    pub fn get_result(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return compat::read(__this, |__t| __t.output.clone());
    }

    /// java: public Vector2i getGridSize()
    pub fn get_grid_size(__this: &compat::Shared<Self>) -> Vector2i {
        let mut grid_width: i32;
        let mut grid_height: i32;
        let mut sqrt: i32 = { grid_height = { grid_width = ((((__this.lock().borrow().inputs.len() as i32) as f64).sqrt()) as i32); grid_width }; grid_height };
        let mut percentage: f32 = (((__this.lock().borrow().inputs.len() as i32).wrapping_sub(sqrt.wrapping_mul(sqrt)) as f32) / ((((sqrt.wrapping_add(1)).wrapping_mul(sqrt.wrapping_add(1))).wrapping_sub(sqrt.wrapping_mul(sqrt))) as f32));
        if percentage > 0.005f32 {
            grid_width += 1;
            if (percentage as f64) > 0.6 {
                grid_height += 1;
            }
        }
        return Vector2i::new(grid_width, grid_height);
    }

    /// java: public String getDurationString()
    pub fn get_duration_string(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.duration_string.clone());
    }

}

impl SkyblockRecipe for SkyblockForgeRecipe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height) (dispatch: the body for this class)
    fn get_input_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockForgeRecipe::get_input_slots_dyn
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height) (dispatch: the body for this class)
    fn get_output_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockForgeRecipe::get_output_slots_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getInputs() (dispatch: the body for this class)
    fn get_inputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockForgeRecipe::get_inputs_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs() (dispatch: the body for this class)
    fn get_outputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockForgeRecipe::get_outputs_dyn
    }

    /// java: @Override public Component getExtraText() (dispatch: the body for this class)
    fn get_extra_text(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        SkyblockForgeRecipe::get_extra_text_dyn
    }

    /// java: @Override public Identifier getCategoryIdentifier() (dispatch: the body for this class)
    fn get_category_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockForgeRecipe::get_category_identifier_dyn
    }

    /// java: public Identifier getRecipeIdentifier() (dispatch: the body for this class)
    fn get_recipe_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockForgeRecipe::get_recipe_identifier_dyn
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height) (dispatch: the body for this class)
    fn get_arrow_location(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Option<ScreenPosition> {
        SkyblockForgeRecipe::get_arrow_location_dyn
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, &GuiGraphicsExtractor, i32, i32, f64, f64) {
        SkyblockForgeRecipe::extract_render_state_dyn
    }

}

impl SkyblockForgeRecipe {
    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height)
    pub fn get_input_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        let mut out: Vec<RecipeSlot> = Vec::new();
        let mut center_x: i32 = width.wrapping_div(2);
        let mut center_y: i32 = height.wrapping_div(2);
        let mut radius: Vector2i = SkyblockForgeRecipe::get_grid_size(__this);
        let mut start_x: i32 = (((((center_x as f32) / 2.0f32) - (((radius.x as f32) / 2.0f32) * (18 as f32)))) as i32);
        let mut start_y: i32 = ((((center_y as f32) - (((radius.y as f32) / 2.0f32) * (18 as f32)))) as i32);
        for i in { 0..(__this.lock().borrow().inputs.len() as i32) } {
            let mut x: i32 = start_x.wrapping_add((i.wrapping_rem(radius.x)).wrapping_mul(18));
            let mut y: i32 = start_y.wrapping_add((i.wrapping_div(radius.x)).wrapping_mul(18));
            out.push(RecipeSlot::new(x, y, __this.lock().borrow().inputs[(i) as usize].clone()));
        }
        return out;
    }

    /// SkyblockForgeRecipe.getInputSlots as the dispatch of SkyblockRecipe.getInputSlots
    pub fn get_input_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_input_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height)
    pub fn get_output_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        return vec![RecipeSlot::new(((3i32.wrapping_mul(width)).wrapping_div(4)).max((width.wrapping_div(2)).wrapping_add(30)), (height.wrapping_div(2)).wrapping_sub(9), compat::read(__this, |__t| __t.output.clone()))];
    }

    /// SkyblockForgeRecipe.getOutputSlots as the dispatch of SkyblockRecipe.getOutputSlots
    pub fn get_output_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_output_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<FlexibleItemStack> getInputs()
    pub fn get_inputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.inputs.clone());
    }

    /// SkyblockForgeRecipe.getInputs as the dispatch of SkyblockRecipe.getInputs
    pub fn get_inputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_inputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs()
    pub fn get_outputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return vec![compat::read(__this, |__t| __t.output.clone())];
    }

    /// SkyblockForgeRecipe.getOutputs as the dispatch of SkyblockRecipe.getOutputs
    pub fn get_outputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_outputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component getExtraText()
    pub fn get_extra_text_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return component::empty();
    }

    /// SkyblockForgeRecipe.getExtraText as the dispatch of SkyblockRecipe.getExtraText
    pub fn get_extra_text_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        Self::get_extra_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getCategoryIdentifier()
    pub fn get_category_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return ID.get_mut().clone();
    }

    /// SkyblockForgeRecipe.getCategoryIdentifier as the dispatch of SkyblockRecipe.getCategoryIdentifier
    pub fn get_category_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_category_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: public Identifier getRecipeIdentifier()
    pub fn get_recipe_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return Identifier::from_namespace_and_path("skyblock", &format!("{}_{}", compat::read(&compat::read(__this, |__t| __t.output.clone()), |__s| __s.get_skyblock_id()).to_lowercase().replace(';', &'_'.to_string()), FlexibleItemStack::count(&compat::read(__this, |__t| __t.output.clone()))));
    }

    /// SkyblockForgeRecipe.getRecipeIdentifier as the dispatch of SkyblockRecipe.getRecipeIdentifier
    pub fn get_recipe_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_recipe_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height)
    pub fn get_arrow_location_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Option<ScreenPosition> {
        return Some(ScreenPosition::new(width.wrapping_div(2), (height.wrapping_div(2)).wrapping_sub(9)));
    }

    /// SkyblockForgeRecipe.getArrowLocation as the dispatch of SkyblockRecipe.getArrowLocation
    pub fn get_arrow_location_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Option<ScreenPosition> {
        Self::get_arrow_location_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut width: i32, mut height: i32, mut mouse_x: f64, mut mouse_y: f64) {
        let mut arrow_location: Option<ScreenPosition> = ({ let __f = compat::dispatch(__this, |__t| __t.get_arrow_location()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipe>), width, height) });
        if let Some(arrow_location) = &arrow_location {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.duration_string.clone()), arrow_location.x().wrapping_add(12), arrow_location.y().wrapping_sub(10), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// SkyblockForgeRecipe.extractRenderState as the dispatch of SkyblockRecipe.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f64, __p4: f64) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}
