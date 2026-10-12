//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockRecipeResults by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::item_price;
use crate::de::hysky::skyblocker::skyblock::item::item_price::ItemPrice;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_result_button::SkyblockRecipeResultButton;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_result_button::SkyblockRecipeResultButtonBase;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe::CenteredRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::SkyblockCraftingRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page::RecipeBookPage;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::vector2i::Vector2i;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int MAX_TEXT_WIDTH = 124;
pub const MAX_TEXT_WIDTH: i32 = 124;
/// java: private static final String ELLIPSIS_STRING = CommonComponents.ELLIPSIS.getString();
pub static ELLIPSIS_STRING: LazyLock<String> = LazyLock::new(|| mcrust::net::minecraft::network::chat::common_components::ELLIPSIS.to_string());

/// java: public class SkyblockRecipeResults implements RecipeAreaDisplay
pub struct SkyblockRecipeResults {
    /// java: private final List<SkyblockRecipeResultButton> resultButtons = Lists.newArrayListWithCapacity(20);
    pub result_buttons: Vec<compat::Shared<dyn SkyblockRecipeResultButton>>,
    /// java: private final List<SkyblockRecipeResultButton> recipeSlotButtons = Lists.newArrayListWithCapacity(16);
    pub recipe_slot_buttons: Vec<compat::Shared<dyn SkyblockRecipeResultButton>>,
    /// java: private @Nullable ItemStack recipeIcon = null;
    pub recipe_icon: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private @Nullable ImageButton nextPageButton;
    pub next_page_button: Option<AbstractWidget>,
    /// java: private @Nullable ImageButton prevPageButton;
    pub prev_page_button: Option<AbstractWidget>,
    /// java: private SkyblockRecipeResultButton hoveredResultButton;
    pub hovered_result_button: Option<compat::Shared<dyn SkyblockRecipeResultButton>>,
    /// java: private @Nullable String lastSearchQuery = null;
    pub last_search_query: Option<String>,
    /// java: private final List<ItemStack> searchResults = new ArrayList<>();
    pub search_results: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private Component hoveredText;
    pub hovered_text: Option<MutableComponent>,
    /// java: private List<SkyblockRecipe> recipeResults = new ArrayList<>();
    pub recipe_results: Vec<compat::Shared<dyn SkyblockRecipe>>,
    /// java: private int pageCount = 0;
    pub page_count: i32,
    /// java: private int currentPage = 0;
    pub current_page: i32,
    /// java: private boolean recipeView = false;
    pub recipe_view: bool,
}

impl SkyblockRecipeResults {
    /// java: protected SkyblockRecipeResults()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `resultButtons`: static call Lists.newArrayListWithCapacity(..)
        //   java: private final List<SkyblockRecipeResultButton> resultButtons = Lists.newArrayListWithCapacity(20);
        // TODO(fabric2seer): the initialiser of `recipeSlotButtons`: static call Lists.newArrayListWithCapacity(..)
        //   java: private final List<SkyblockRecipeResultButton> recipeSlotButtons = Lists.newArrayListWithCapacity(16);
        let __this = compat::shared(Self {
            result_buttons: Default::default(),
            recipe_slot_buttons: Default::default(),
            recipe_icon: None,
            next_page_button: None,
            prev_page_button: None,
            hovered_result_button: None,
            last_search_query: None,
            search_results: Vec::new(),
            hovered_text: None,
            recipe_results: Vec::new(),
            page_count: 0,
            current_page: 0,
            recipe_view: false,
        });
        for i in 0..20 {
            let __h0 = (SkyblockRecipeResultButtonBase::new() as compat::Shared<dyn SkyblockRecipeResultButton>);
            __this.lock().borrow_mut().result_buttons.push(__h0);
        }
        __this
    }

    /// java: private void extractRecipeDisplay(GuiGraphicsExtractor graphics, Font textRenderer, int x, int y, int mouseX, int mouseY)
    pub fn extract_recipe_display(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, text_renderer: &Font, mut x: i32, mut y: i32, mut mouse_x: i32, mut mouse_y: i32) {
        let mut recipe: compat::Shared<dyn SkyblockRecipe> = __this.lock().borrow().recipe_results[(compat::read(__this, |__t| __t.current_page)) as usize].clone();
        let mut craft_text: String = ({ let __f = compat::dispatch(&recipe, |__t| __t.get_extra_text()); __f(&recipe) }).get_string();
        if !craft_text.is_empty() {
            if text_renderer.width(&craft_text) > MAX_TEXT_WIDTH {
                if SkyblockRecipeResults::is_mouse_hovering_text(__this, x.wrapping_add(11), y.wrapping_add(31), mouse_x, mouse_y) {
                    { let __v = Some(component::literal(&craft_text)); compat::write(__this, |__t| __t.hovered_text = __v); }
                }
                craft_text = format!("{}{}", text_renderer.plain_substr_by_width(&craft_text, MAX_TEXT_WIDTH), *ELLIPSIS_STRING);
            }
            gui_graphics_extractor::text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&craft_text), x.wrapping_add(11), y.wrapping_add(31), mcrust::net::minecraft::util::common_colors::WHITE);
        }
        let mut item_name: MutableComponent = compat::component_of((FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(&recipe, |__t| __t.get_outputs()); __f(&recipe) })[0])).get_hover_name());
        if text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(item_name.clone())) > MAX_TEXT_WIDTH {
            let mut trimmed: mcrust::net::minecraft::network::chat::formatted_text::FormattedText = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::composite(vec![text_renderer.substr_by_width(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(item_name.clone()), MAX_TEXT_WIDTH), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(mcrust::net::minecraft::network::chat::common_components::ELLIPSIS)]);
            // java: FormattedCharSequence ordered = Language.getInstance().getVisualOrder(trimmed);
            let mut ordered: Seq = language::get_instance().get_visual_order(&trimmed);
            gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &sequence_text(&*ordered), x.wrapping_add(11), y.wrapping_add(43), mcrust::net::minecraft::util::common_colors::WHITE);
            if SkyblockRecipeResults::is_mouse_hovering_text(__this, x.wrapping_add(11), y.wrapping_add(43), mouse_x, mouse_y) {
                { let __v = Some(item_name.clone()); compat::write(__this, |__t| __t.hovered_text = __v); }
            }
        } else {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x.wrapping_add(11), y.wrapping_add(43), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(item_name.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        }
        gui_graphics_extractor::text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("▶"), x.wrapping_add(96), y.wrapping_add(90), 0xAAFFFFFFu32 as i32);
        if compat::read(__this, |__t| __t.hovered_text.clone()).is_none() && mouse_x >= x.wrapping_add(86) && mouse_y >= y.wrapping_add(81) && mouse_x < (x.wrapping_add(86)).wrapping_add(25) && mouse_y < (y.wrapping_add(81)).wrapping_add(25) && let Some(forge_recipe) = recipe.lock().borrow().as_any().downcast_ref::<compat::Shared<SkyblockForgeRecipe>>() {
            { let __v = Some(component::null_to_empty(Some(&SkyblockForgeRecipe::get_duration_string(forge_recipe)))); compat::write(__this, |__t| __t.hovered_text = __v); }
        }
        if compat::read(__this, |__t| __t.recipe_icon.clone()).is_some() {
            graphics.item(&compat::read(__this, |__t| __t.recipe_icon.clone()).unwrap(), x.wrapping_add(115), y.wrapping_add(61));
        }
    }

    /// java: private boolean isMouseHoveringText(int textX, int textY, int mouseX, int mouseY)
    pub fn is_mouse_hovering_text(__this: &compat::Shared<Self>, mut text_x: i32, mut text_y: i32, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        return GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (text_x) as f64, (text_y) as f64, (((text_x.wrapping_add(MAX_TEXT_WIDTH)).wrapping_add(4)) as f64) as f64, ((text_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)) as f64) as f64);
    }

    /// java: protected void closeRecipeView()
    pub fn close_recipe_view(__this: &compat::Shared<Self>) {
        { let __v = 0; compat::write(__this, |__t| __t.current_page = __v); }
        { let __v = (((__this.lock().borrow().search_results.len() as i32).wrapping_sub(1)).wrapping_div(__this.lock().borrow().result_buttons.len() as i32)).wrapping_add(1); compat::write(__this, |__t| __t.page_count = __v); }
        { let __v = false; compat::write(__this, |__t| __t.recipe_view = __v); }
        SkyblockRecipeResults::update_result_buttons(__this);
    }

    /// java: private void updateResultButtons()
    pub fn update_result_buttons(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.recipe_view) {
            let mut recipe: Option<compat::Shared<dyn SkyblockRecipe>> = Some(__this.lock().borrow().recipe_results[(compat::read(__this, |__t| __t.current_page)) as usize].clone());
            for button in compat::read(__this, |__t| __t.result_buttons.clone()) {
                ({ let __f = compat::dispatch(&button, |__t| __t.clear_display_stack()); __f(&button) });
            }
            __this.lock().borrow_mut().recipe_slot_buttons.clear();
            if let Some(__sel1) = recipe.as_ref() && let Some(__view2) = compat::try_view::<SkyblockCraftingRecipe, _>(__sel1) {
                { let __v = Some(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("crafting_table"))); compat::write(__this, |__t| __t.recipe_icon = __v); }
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(5) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[0])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(6) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(1) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(7) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(2) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(10) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(3) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(11) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(4) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(12) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(5) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(15) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(6) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(16) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(7) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(17) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_grid((&*__view2))[(8) as usize])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(14) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockCraftingRecipe::get_result((&*__view2)))) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
            } else if let Some(__sel3) = recipe.as_ref() && let Some(__view4) = compat::try_view::<SkyblockForgeRecipe, _>(__sel3) {
                { let __v = Some(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("lava_bucket"))); compat::write(__this, |__t| __t.recipe_icon = __v); }
                let mut grid_size: Vector2i = SkyblockForgeRecipe::get_grid_size((&*__view4));
                let mut button: compat::Shared<dyn SkyblockRecipeResultButton> = __this.lock().borrow().result_buttons[(11) as usize].clone();
                let __o1 = button.clone();
                let __o2 = button.clone();
                let __o3 = button.clone();
                let __o4 = button.clone();
                let mut start_x: i32 = ((compat::write(&__o3, |__t| __t.skyblock_recipe_result_button_base_mut().get_x())).wrapping_add((compat::write(&__o4, |__t| __t.skyblock_recipe_result_button_base_mut().get_width())).wrapping_div(2))).wrapping_sub((grid_size.x.wrapping_mul(25)).wrapping_div(2));
                let __o5 = button.clone();
                let __o6 = button.clone();
                let __o7 = button.clone();
                let __o8 = button.clone();
                let mut start_y: i32 = ((compat::write(&__o7, |__t| __t.skyblock_recipe_result_button_base_mut().get_y())).wrapping_add((compat::write(&__o8, |__t| __t.skyblock_recipe_result_button_base_mut().get_height())).wrapping_div(2))).wrapping_sub((grid_size.y.wrapping_mul(25)).wrapping_div(2));
                for i in 0..((({ let __r = (&*__view4); let __f = compat::dispatch(__r, |__t| __t.get_inputs()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).len() as i32) {
                    let mut x: i32 = start_x.wrapping_add((i.wrapping_rem(grid_size.x)).wrapping_mul(25));
                    let mut y: i32 = start_y.wrapping_add((i.wrapping_div(grid_size.x)).wrapping_mul(25));
                    let __h0 = ({ let __r = &(SkyblockRecipeResultButtonBase::new_2(x, y) as compat::Shared<dyn SkyblockRecipeResultButton>); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&(({ let __r = (&*__view4); let __f = compat::dispatch(__r, |__t| __t.get_inputs()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SkyblockRecipe>)) }))[(i) as usize])) });
                    __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                }
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(14) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&SkyblockForgeRecipe::get_result((&*__view4)))) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
            } else if let Some(__centered) = recipe.as_ref().and_then(as_centered_recipe) {
                let centered_recipe = &__centered;
                { let __v = Some(FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_icon()); __f(centered_recipe) }))); compat::write(__this, |__t| __t.recipe_icon = __v); }
                if ({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_representative()); __f(centered_recipe) }).is_some() {
                    let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(8) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_representative()); __f(centered_recipe) }).unwrap())) });
                    __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                }
                let mut slots_per_row: i32 = 3;
                let mut rows: i32 = (((({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_inputs()); __f(&compat::view(centered_recipe, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).len() as i32).wrapping_div(slots_per_row)).wrapping_add(1);
                let mut button: compat::Shared<dyn SkyblockRecipeResultButton> = __this.lock().borrow().result_buttons[(11) as usize].clone();
                let __o9 = __this.lock().borrow().result_buttons[0].clone();
                let mut start_x: i32 = compat::write(&__o9, |__t| __t.skyblock_recipe_result_button_base_mut().get_x());
                let __o10 = button.clone();
                let __o11 = button.clone();
                let __o12 = button.clone();
                let __o13 = button.clone();
                let mut start_y: i32 = ((compat::write(&__o12, |__t| __t.skyblock_recipe_result_button_base_mut().get_y())).wrapping_add((compat::write(&__o13, |__t| __t.skyblock_recipe_result_button_base_mut().get_height())).wrapping_div(2))).wrapping_sub((rows.wrapping_mul(25)).wrapping_div(2));
                for i in 0..((({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_inputs()); __f(&compat::view(centered_recipe, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).len() as i32) {
                    let mut x: i32 = start_x.wrapping_add((i.wrapping_rem(slots_per_row)).wrapping_mul(25));
                    let mut y: i32 = start_y.wrapping_add((i.wrapping_div(slots_per_row)).wrapping_mul(25));
                    let __h0 = ({ let __r = &(SkyblockRecipeResultButtonBase::new_2(x, y) as compat::Shared<dyn SkyblockRecipeResultButton>); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&(({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_inputs()); __f(&compat::view(centered_recipe, |a| a as compat::Shared<dyn SkyblockRecipe>)) }))[(i) as usize])) });
                    __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
                }
                let __h0 = ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(14) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, FlexibleItemStack::get_stack_or_throw(&(({ let __f = compat::dispatch(centered_recipe, |__t| __t.get_outputs()); __f(&compat::view(centered_recipe, |a| a as compat::Shared<dyn SkyblockRecipe>)) }))[0])) });
                __this.lock().borrow_mut().recipe_slot_buttons.push(__h0);
            } else {
            }
        } else {
            { let __v = None; compat::write(__this, |__t| __t.recipe_icon = __v); }
            for i in 0..(__this.lock().borrow().result_buttons.len() as i32) {
                let mut index: i32 = ((compat::read(__this, |__t| __t.current_page)).wrapping_mul(__this.lock().borrow().result_buttons.len() as i32)).wrapping_add(i);
                if index < (__this.lock().borrow().search_results.len() as i32) {
                    ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(i) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.set_display_stack()); __f(__r, compat::read(__this, |__t| __t.search_results[(index) as usize].clone())) });
                } else {
                    ({ let __r = &compat::read(__this, |__t| __t.result_buttons[(i) as usize].clone()); let __f = compat::dispatch(__r, |__t| __t.clear_display_stack()); __f(__r) });
                }
            }
        }
        SkyblockRecipeResults::hide_show_page_buttons(__this);
    }

    /// java: private void hideShowPageButtons()
    pub fn hide_show_page_buttons(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| __t.current_page);
        __this.lock().borrow_mut().prev_page_button.as_mut().unwrap().active = (__r0 > 0);
        let __r0 = compat::read(__this, |__t| __t.current_page);
        let __r1 = compat::read(__this, |__t| __t.page_count);
        __this.lock().borrow_mut().next_page_button.as_mut().unwrap().active = (__r0 < (__r1).wrapping_sub(1));
    }

    /// java: private @Nullable ItemStack getHoveredItemStack(double mouseX, double mouseY)
    pub fn get_hovered_item_stack(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let __wa1 = mouse_x;
        let __wa2 = mouse_y;
        return compat::read(__this, |__t| __t.result_buttons.clone()).into_iter().filter(|btn| compat::write(btn, |__t| __t.skyblock_recipe_result_button_base_mut().is_mouse_over(__wa1, __wa2))).next().and_then(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_display_stack()); __f(&__a0) }));
    }

}

impl RecipeAreaDisplay for SkyblockRecipeResults {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop) (dispatch: the body for this class)
    fn initialize(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut mcrust::net::minecraft::client::minecraft::Minecraft, i32, i32) {
        SkyblockRecipeResults::initialize_dyn
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &GuiGraphicsExtractor, i32, i32, i32, i32, f32) {
        SkyblockRecipeResults::extract_render_state_dyn
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut GuiGraphicsExtractor, i32, i32) {
        SkyblockRecipeResults::extract_tooltip_dyn
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh) (dispatch: the body for this class)
    fn update_search_results_3(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, String, FilterOption, bool) {
        SkyblockRecipeResults::update_search_results_3_dyn
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &MouseButtonEvent, bool) -> bool {
        SkyblockRecipeResults::mouse_clicked_dyn
    }

    /// java: @Override public boolean keyPressed(double mouseX, double mouseY, KeyEvent input) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, f64, f64, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        SkyblockRecipeResults::key_pressed_dyn
    }

}

impl SkyblockRecipeResults {
    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop)
    pub fn initialize_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut parent_left: i32, mut parent_top: i32) {
        for i in 0..(__this.lock().borrow().result_buttons.len() as i32) {
            let __wa1 = (parent_left.wrapping_add(11)).wrapping_add(25i32.wrapping_mul(i.wrapping_rem(5)));
            let __wa2 = (parent_top.wrapping_add(31)).wrapping_add(25i32.wrapping_mul(i.wrapping_div(5)));
            let __o1 = __this.lock().borrow().result_buttons[(i) as usize].clone();
            compat::write(&__o1, |__t| __t.skyblock_recipe_result_button_base_mut().set_position(__wa1, __wa2));
        }
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_FORWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        // rust: { let __v = Some(AbstractWidget::new(parent_left.wrapping_add(93), parent_top.wrapping_add(137), 12, 17, RecipeBookPage::PAGE_FORWARD_SPRITES.clone(), Arc::new(move |_| {
        // rust: 
        // rust: }))); compat::write(__this, |__t| __t.next_page_button = __v); }
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_BACKWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        // rust: { let __v = Some(AbstractWidget::new(parent_left.wrapping_add(38), parent_top.wrapping_add(137), 12, 17, RecipeBookPage::PAGE_BACKWARD_SPRITES.clone(), Arc::new(move |_| {
        // rust: 
        // rust: }))); compat::write(__this, |__t| __t.prev_page_button = __v); }
        SkyblockRecipeResults::update_result_buttons(__this);
    }

    /// SkyblockRecipeResults.initialize as the dispatch of RecipeAreaDisplay.initialize
    pub fn initialize_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft, __p1: i32, __p2: i32) {
        Self::initialize_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let mut text_renderer: Font = compat::font();
        { let __v = None; compat::write(__this, |__t| __t.hovered_text = __v); }
        if compat::read(__this, |__t| __t.recipe_view) {
            SkyblockRecipeResults::extract_recipe_display(__this, graphics, &text_renderer, x, y, mouse_x, mouse_y);
        }
        if compat::read(__this, |__t| __t.page_count) > 1 {
            let mut text: MutableComponent = component::translatable_args("gui.recipebook.page", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of((compat::read(__this, |__t| __t.current_page)).wrapping_add(1).to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.page_count).to_string())]);
            let mut width: i32 = text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), (x.wrapping_sub(width.wrapping_div(2))).wrapping_add(73), y.wrapping_add(141), mcrust::net::minecraft::util::common_colors::WHITE, false);
        }
        { let __v = None; compat::write(__this, |__t| __t.hovered_result_button = __v); }
        for result_button in (if compat::read(__this, |__t| __t.recipe_view) { compat::read(__this, |__t| __t.recipe_slot_buttons.clone()) } else { compat::read(__this, |__t| __t.result_buttons.clone()) }) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa1 = graphics.clone();
            // rust: let __wa2 = mouse_x;
            // rust: let __wa3 = mouse_y;
            // rust: let __wa4 = delta;
            // rust: let __o1 = result_button.clone();
            // rust: compat::write(&__o1, |__t| __t.skyblock_recipe_result_button_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
            let __o2 = result_button.clone();
            let __o3 = result_button.clone();
            if compat::read(&result_button, |__t| __t.skyblock_recipe_result_button_base().visible) && compat::write(&__o3, |__t| __t.skyblock_recipe_result_button_base_mut().is_hovered_or_focused()) {
                { let __v = Some(result_button.clone()); compat::write(__this, |__t| __t.hovered_result_button = __v); }
            }
        }
        if compat::read(__this, |__t| __t.prev_page_button.clone()).is_some() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta,); __this.lock().borrow_mut().prev_page_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        }
        if compat::read(__this, |__t| __t.next_page_button.clone()).is_some() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta,); __this.lock().borrow_mut().next_page_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        }
    }

    /// SkyblockRecipeResults.extractRenderState as the dispatch of RecipeAreaDisplay.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: i32, __p5: f32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        if compat::minecraft().gui.screen().is_some() {
            if compat::read(__this, |__t| __t.hovered_result_button.clone()).is_some() && !(({ let __r = &compat::read(__this, |__t| __t.hovered_result_button.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_display_stack()); __f(__r) }).unwrap().is_empty()) {
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = ({ let __r = &compat::read(__this, |__t| __t.hovered_result_button.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_display_stack()); __f(__r) }).unwrap();
                // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
                // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &stack, x, y);
            } else if compat::read(__this, |__t| __t.hovered_text.clone()).is_some() {
                // TODO(fabric2seer): the port's `set_tooltip_for_next_frame_text` ports this Java method with other parameters (tooltip_for_next_frame, text for Java's text, xo, yo)
                // TODO(fabric2seer): `set_tooltip_for_next_frame_text` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, text: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 4
                // rust: gui_graphics_extractor::set_tooltip_for_next_frame_text(&mut compat::font(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.hovered_text.clone()).unwrap()), x, y);
            }
        }
    }

    /// SkyblockRecipeResults.extractTooltip as the dispatch of RecipeAreaDisplay.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh)
    pub fn update_search_results_3_impl(__this: &compat::Shared<Self>, mut query: String, mut filter_option: FilterOption, mut refresh: bool) {
        if !(ItemRepository::files_imported()) {
            return;
        }
        if !(Some(&*(query)) == compat::read(__this, |__t| __t.last_search_query.clone()).as_deref()) || refresh {
            { let __v = Some(query.clone()); compat::write(__this, |__t| __t.last_search_query = __v); }
            __this.lock().borrow_mut().search_results.clear();
            for stack in ItemRepository::get_items() {
                let mut name: String = compat::component_of(FlexibleItemStack::get_stack_or_throw(&stack).get_hover_name()).get_string().to_lowercase();
                if !(filter_option.test(&name)) {
                    continue;
                }
                let mut lore: Vec<String> = stack.lock().borrow().skyblocker_get_lore_strings();
                if name.contains(&query) || lore.iter().cloned().map(|string| string.to_lowercase()).any(|line| line.contains(&query)) {
                    let __h0 = FlexibleItemStack::get_stack_or_throw(&stack);
                    __this.lock().borrow_mut().search_results.push(__h0);
                }
            }
            SkyblockRecipeResults::close_recipe_view(__this);
        } else {
            SkyblockRecipeResults::hide_show_page_buttons(__this);
        }
    }

    /// SkyblockRecipeResults.updateSearchResults as the dispatch of RecipeAreaDisplay.updateSearchResults
    pub fn update_search_results_3_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: String, __p1: FilterOption, __p2: bool) {
        Self::update_search_results_3_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_impl(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().next_page_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() } {
            { let __v = compat::read(__this, |__t| __t.current_page) + 1; compat::write(__this, |__t| __t.current_page = __v); }
            SkyblockRecipeResults::update_result_buttons(__this);
            return true;
        } else if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow().prev_page_button.as_ref().unwrap().mouse_clicked(__q0.button(), __q0.x(), __q0.y()).is_some() } {
            { let __v = compat::read(__this, |__t| __t.current_page) - 1; compat::write(__this, |__t| __t.current_page = __v); }
            SkyblockRecipeResults::update_result_buttons(__this);
            return true;
        }
        if compat::read(__this, |__t| __t.recipe_view) && click.button() == 1 {
            let mut result = __this.lock().borrow().result_buttons[(14) as usize].clone();
            let mut raw_id = ({ let __f = compat::dispatch(&result, |__t| __t.get_display_stack()); __f(&result) }).unwrap().get_skyblock_id();
            let __wa1 = click.x();
            let __wa2 = click.y();
            let __o1 = result.clone();
            let __wa3 = click.x();
            let __wa4 = click.y();
            let __o2 = result.clone();
            if compat::write(&__o2, |__t| __t.skyblock_recipe_result_button_base_mut().is_mouse_over(__wa3, __wa4)) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/viewrecipe {}", raw_id), true);
                return true;
            }
        }
        for result_button in (if compat::read(__this, |__t| __t.recipe_view) { compat::read(__this, |__t| __t.recipe_slot_buttons.clone()) } else { compat::read(__this, |__t| __t.result_buttons.clone()) }) {
            // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
            let __wa5 = click.clone();
            let __wa6 = doubled;
            let __o3 = result_button.clone();
            let __wa7 = click.clone();
            let __wa8 = doubled;
            let __o4 = result_button.clone();
            if compat::write(&__o4, |__t| __t.skyblock_recipe_result_button_base_mut().mouse_clicked(__wa7.button(), __wa7.x(), __wa7.y()).is_some()) {
                let mut item_id: String = ({ let __f = compat::dispatch(&result_button, |__t| __t.get_display_stack()); __f(&result_button) }).unwrap().get_skyblock_id();
                if item_id.is_empty() {
                    continue;
                }
                let mut recipes: Vec<compat::Shared<dyn SkyblockRecipe>> = ItemRepository::get_recipes_and_usages(&({ let __f = compat::dispatch(&result_button, |__t| __t.get_display_stack()); __f(&result_button) }).unwrap());
                if !recipes.is_empty() {
                    { let __v = recipes.clone(); compat::write(__this, |__t| __t.recipe_results = __v); }
                    { let __v = 0; compat::write(__this, |__t| __t.current_page = __v); }
                    { let __v = (recipes.len() as i32); compat::write(__this, |__t| __t.page_count = __v); }
                    { let __v = true; compat::write(__this, |__t| __t.recipe_view = __v); }
                    SkyblockRecipeResults::update_result_buttons(__this);
                }
                return true;
            }
        }
        return false;
    }

    /// SkyblockRecipeResults.mouseClicked as the dispatch of RecipeAreaDisplay.mouseClicked
    pub fn mouse_clicked_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &MouseButtonEvent, __p1: bool) -> bool {
        Self::mouse_clicked_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public boolean keyPressed(double mouseX, double mouseY, KeyEvent input)
    pub fn key_pressed_impl(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let Some(mut hovered) = SkyblockRecipeResults::get_hovered_item_stack(__this, mouse_x, mouse_y) else {
            return false;
        };
        if mcrust::mcrust::modding::game::with_player(|player| WikiLookupManager::handle_wiki_lookup(&Either::right(hovered.clone()), player, input)).unwrap_or(false) {
            return true;
        }
        if SkyblockerConfigManager::get().helpers.item_price.enable_item_price_lookup && item_price::ITEM_PRICE_LOOKUP.get_mut().matches(input) {
            mcrust::mcrust::modding::game::with_player(|player| { ItemPrice::item_price_lookup_local_player_item_stack(player, &hovered); });
            return true;
        }
        return false;
    }

    /// SkyblockRecipeResults.keyPressed as the dispatch of RecipeAreaDisplay.keyPressed
    pub fn key_pressed_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: f64, __p1: f64, __p2: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        Self::key_pressed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: `recipe instanceof CenteredRecipe centeredRecipe` - the recipe as a `CenteredRecipe` when its class is one of
/// the interface's implementers
fn as_centered_recipe(recipe: &compat::Shared<dyn SkyblockRecipe>) -> Option<compat::Shared<dyn CenteredRecipe>> {
    use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_kat_upgrade_recipe::SkyblockKatUpgradeRecipe;
    use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::SkyblockNpcShopRecipe;
    if compat::try_view::<SkyblockKatUpgradeRecipe, _>(recipe).is_some() {
        return Some(compat::handle_as::<SkyblockKatUpgradeRecipe, _>(recipe) as compat::Shared<dyn CenteredRecipe>);
    }
    if compat::try_view::<SkyblockNpcShopRecipe, _>(recipe).is_some() {
        return Some(compat::handle_as::<SkyblockNpcShopRecipe, _>(recipe) as compat::Shared<dyn CenteredRecipe>);
    }
    None
}

/// The port draws a `FormattedText` where Java draws a `FormattedCharSequence`: the sequence's styled
/// characters, gathered back into runs of one style each.
fn sequence_text(seq: &dyn FormattedCharSequence) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    use mcrust::net::minecraft::network::chat::style::Style;
    let mut runs: Vec<(Style, String)> = Vec::new();
    seq.accept(&mut |_position: i32, style: &Style, codepoint: u32| {
        if let Some(c) = char::from_u32(codepoint) {
            match runs.last_mut() {
                Some((s, text)) if s == style => text.push(c),
                _ => runs.push((style.clone(), c.to_string())),
            }
        }
        true
    });
    FormattedText::composite(runs.into_iter().map(|(style, text)| FormattedText::of_styled(text, style)).collect())
}
