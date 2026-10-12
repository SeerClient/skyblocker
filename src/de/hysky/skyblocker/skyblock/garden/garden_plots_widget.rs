//! Ported from de.hysky.skyblocker.skyblock.garden.GardenPlotsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots::GardenPlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::item_button_widget::ItemButtonWidget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final Supplier<ScreenPosition> POSITION = () -> new ScreenPosition(SkyblockerConfigManager.get().farming.plotsWidget.x, SkyblockerConfigManager.get().farming.plotsWidget.y);
pub static POSITION: compat::MainThread<Arc<dyn Fn() -> ScreenPosition>> = compat::MainThread::new(|| Arc::new(move || ScreenPosition::new(SkyblockerConfigManager::get().farming.plots_widget.x, SkyblockerConfigManager::get().farming.plots_widget.y)));
/// java: private static final int SPACING = 4;
pub const SPACING: i32 = 4;
/// java: private static final Identifier SLOT_HIGHLIGHT_BACK_SPRITE = Identifier.withDefaultNamespace("container/slot_highlight_back");
pub static SLOT_HIGHLIGHT_BACK_SPRITE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/slot_highlight_back"));
/// java: private static final Identifier SLOT_HIGHLIGHT_FRONT_SPRITE = Identifier.withDefaultNamespace("container/slot_highlight_front");
pub static SLOT_HIGHLIGHT_FRONT_SPRITE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/slot_highlight_front"));
// TODO(fabric2seer): static call Int2IntMap.ofEntries(..)
//   java: public static final Int2IntMap GARDEN_PLOT_TO_SLOT = Int2IntMap.ofEntries(
//   java: 			Int2IntMap.entry(0, 12), // The Barn
//   java: 			Int2IntMap.entry(1, 7),
//   java: 			Int2IntMap.entry(2, 11),
//   java: 			Int2IntMap.entry(3, 13),
//   java: 			Int2IntMap.entry(4, 17),
//   java: 			Int2IntMap.entry(5, 6),
//   java: 			Int2IntMap.entry(6, 8),
//   java: 			Int2IntMap.entry(7, 16),
//   java: 			Int2IntMap.entry(8, 18),
//   java: 			Int2IntMap.entry(9, 2),
//   java: 			Int2IntMap.entry(10, 10),
//   java: 			Int2IntMap.entry(11, 14),
//   java: 			Int2IntMap.entry(12, 22),
//   java: 			Int2IntMap.entry(13, 1),
//   java: 			Int2IntMap.entry(14, 3),
//   java: 			Int2IntMap.entry(15, 5),
//   java: 			Int2IntMap.entry(16, 9),
//   java: 			Int2IntMap.entry(17, 15),
//   java: 			Int2IntMap.entry(18, 19),
//   java: 			Int2IntMap.entry(19, 21),
//   java: 			Int2IntMap.entry(20, 23),
//   java: 			Int2IntMap.entry(21, 0),
//   java: 			Int2IntMap.entry(22, 4),
//   java: 			Int2IntMap.entry(23, 20),
//   java: 			Int2IntMap.entry(24, 24)
//   java: 	);
/// java: public static final Int2IntMap GARDEN_PLOT_TO_SLOT = Int2IntMap.ofEntries( Int2IntMap.entry(0, 12), // The Barn Int2IntMap.entry(1, 7), Int2IntMap.entry(2, 11), Int2IntMap.entry(3, 13), Int2IntMap.ent
pub static GARDEN_PLOT_TO_SLOT: compat::MainThread<std::collections::HashMap<i32, i32>> = compat::MainThread::new(|| std::collections::HashMap::from([(0, 12), (1, 7), (2, 11), (3, 13), (4, 17), (5, 6), (6, 8), (7, 16), (8, 18), (9, 2), (10, 10), (11, 14), (12, 22), (13, 1), (14, 3), (15, 5), (16, 9), (17, 15), (18, 19), (19, 21), (20, 23), (21, 0), (22, 4), (23, 20), (24, 24)]));
/// java: private static final @Nullable String[] CUSTOM_ICON_OPTIONS = new @Nullable String[] { null, "WHEAT", "CARROT_ITEM", "POTATO_ITEM", "SUGAR_CANE", "DOUBLE_PLANT", "MOONFLOWER", "WILD_ROSE", "NETHER_STA
///
/// The ELEMENTS are nullable (`null` is the "None" option).
pub static CUSTOM_ICON_OPTIONS: LazyLock<Vec<Option<String>>> = LazyLock::new(|| vec![None, Some("WHEAT".to_string()), Some("CARROT_ITEM".to_string()), Some("POTATO_ITEM".to_string()), Some("SUGAR_CANE".to_string()), Some("DOUBLE_PLANT".to_string()), Some("MOONFLOWER".to_string()), Some("WILD_ROSE".to_string()), Some("NETHER_STALK".to_string()), Some("RED_MUSHROOM".to_string()), Some("CACTUS".to_string()), Some("MELON".to_string()), Some("PUMPKIN".to_string()), Some("INK_SACK-3".to_string())]);
/// java: private static final Identifier BACKGROUND_TEXTURE = SkyblockerMod.id("textures/gui/garden_plots.png");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/garden_plots.png"));
/// java: private static final MutableComponent GROSS_PEST_TEXT = Component.translatable("skyblocker.gardenPlots.pests").withStyle(ChatFormatting.RED, ChatFormatting.BOLD);
pub static GROSS_PEST_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.gardenPlots.pests").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]));
/// java: private static final MutableComponent TP_TEXT = Component.translatable("skyblocker.gardenPlots.tp").withStyle(ChatFormatting.YELLOW, ChatFormatting.BOLD);
pub static TP_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.gardenPlots.tp").with_style_formats(&[ChatFormatting::Yellow, ChatFormatting::Bold]));

/// java: public class GardenPlotsWidget extends AbstractContainerWidget
pub struct GardenPlotsWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final ItemButtonWidget[] widgets;
    pub widgets: Vec<ItemButtonWidget>,
    /// java: private final IntList infectedPlots = new IntArrayList(8);
    pub infected_plots: Vec<i32>,
    /// java: @SuppressWarnings("deprecation") private final FlexibleItemStack noneItem = new FlexibleItemStack(new ItemStackTemplate(Items.BARRIER.builtInRegistryHolder(), 1
    pub none_item: compat::Shared<FlexibleItemStack>,
    /// java: private @Nullable ItemStack[] items;
    ///
    /// The ELEMENTS are nullable (a plot with no data); the array is null only until the constructor fills it.
    pub items: Option<Vec<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>>>,
    /// java: private int hoveredSlot = -1;
    pub hovered_slot: i32,
    /// java: private int editingSlotIcon = -1;
    pub editing_slot_icon: i32,
    /// java: private long updateFromTabTime = System.currentTimeMillis();
    pub update_from_tab_time: i64,
    /// java: private ItemStack[] customIconOptionsItems = new ItemStack[0];
    pub custom_icon_options_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private boolean dragAreaHovered;
    pub drag_area_hovered: bool,
    /// java: private @Nullable ScreenPosition dragging;
    pub dragging: Option<ScreenPosition>,
    /// java: private ScreenRectangle inventoryRectangle;
    pub inventory_rectangle: ScreenRectangle,
}

impl std::ops::Deref for GardenPlotsWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GardenPlotsWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GardenPlotsWidget {
    /// java: public GardenPlotsWidget(ScreenRectangle inventoryRectangle)
    pub fn new(mut inventory_rectangle: ScreenRectangle) -> Self {
        let __init_inventory_rectangle = inventory_rectangle.clone();
        // java: super(0, 0, 104, 132, Component.translatable("skyblocker.gardenPlots"), AbstractScrollArea.defaultSettings(0));
        // (the port's 5-argument constructor IS `defaultSettings(0)`, AbstractContainerWidget:16-18)
        let mut this: Self = Self {
            base: AbstractContainerWidget::new(0, 0, 104, 132, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.gardenPlots"))),
            widgets: Vec::new(),
            infected_plots: Vec::with_capacity(8),
            // new FlexibleItemStack(new ItemStackTemplate(Items.BARRIER.builtInRegistryHolder(), 1, DataComponentPatch.builder().set(DataComponents.ITEM_NAME, Component.literal("None")).build()))
            // (the template constructor forwards `item()`, `count()` and `components()` to this one)
            none_item: FlexibleItemStack::new(mcrust::net::minecraft::core::holder::Holder::Reference(items::items().get("barrier").built_in_registry_holder()), 1, Some({
                let mut builder = mcrust::net::minecraft::core::component::data_component_patch::builder();
                builder.set(mcrust::net::minecraft::core::component::data_components::item_name().clone(), mcrust::com::mojang::serialization::obj(component::literal("None")));
                builder.build()
            })),
            items: None,
            hovered_slot: -1,
            editing_slot_icon: -1,
            update_from_tab_time: compat::millis(),
            custom_icon_options_items: Vec::new(),
            drag_area_hovered: false,
            dragging: None,
            inventory_rectangle: __init_inventory_rectangle,
        };
        this.update_plot_items();
        this.update_infested_from_tab();
        let mut desk_button: ItemButtonWidget = ItemButtonWidget::new_5(this.get_x().wrapping_add(7), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("book")), component::translatable("skyblocker.gardenPlots.openDesk"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/desk", true); })));
        let mut spawn_button: ItemButtonWidget = ItemButtonWidget::new_5(((this.get_right().wrapping_sub(7)).wrapping_sub(40)).wrapping_sub(2), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("ender_eye")), component::translatable("skyblocker.gardenPlots.spawn"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/warp garden", true); })));
        let mut set_spawn_button: ItemButtonWidget = ItemButtonWidget::new_5((this.get_right().wrapping_sub(7)).wrapping_sub(20), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one((if compat::random() < 0.001 { mcrust::net::minecraft::world::item::items::bed().pink().clone() } else { mcrust::net::minecraft::world::item::items::bed().red().clone() })), component::translatable("skyblocker.gardenPlots.setSpawn"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/setspawn", true); })));
        this.widgets = vec![desk_button, spawn_button, set_spawn_button];
        this.set_position_from_config();
        this
    }

    /// java: private void updatePlotItems()
    pub fn update_plot_items(&mut self) {
        self.items = Some(garden_plots::GARDEN_PLOTS.get().iter().map(|garden_plot| {
            let Some(garden_plot) = garden_plot else {
                return None; // java: return null
            };
            let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = garden_plot
                .custom_icon()
                .map(|s| ItemRepository::get_item_stack_2(&s, &ItemUtils::get_item_id_placeholder(&s)))
                .map(|stack| FlexibleItemStack::get_stack_or_throw(&stack))
                .unwrap_or_else(|| {
                    // gardenPlot.icon().map(FlexibleItemStack::new, s -> ItemRepository.getItemStack(s, ItemUtils.getItemIdPlaceholder(s)))
                    let stack = match garden_plot.icon() {
                        Either::Left(item) => FlexibleItemStack::new2(item),
                        Either::Right(s) => ItemRepository::get_item_stack_2(&s, &ItemUtils::get_item_id_placeholder(&s)),
                    };
                    FlexibleItemStack::get_stack_or_throw(&stack)
                })
                .copy();
            item_stack.set(mcrust::net::minecraft::core::component::data_components::custom_name().clone(), Some(mcrust::com::mojang::serialization::obj(component::literal(&garden_plot.name()).with_style_formats(&[ChatFormatting::Green, ChatFormatting::Bold]))));
            return Some(item_stack);
        }).collect::<Vec<_>>());
        let mut barn_icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("lodestone"));
        barn_icon.set(mcrust::net::minecraft::core::component::data_components::item_name().clone(), Some(mcrust::com::mojang::serialization::obj(component::literal("The Barn"))));
        self.items.as_mut().unwrap()[(12) as usize] = Some(barn_icon);
    }

    /// java: @SuppressWarnings("deprecation") @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut text_renderer: Font = compat::font();
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((self.get_x()) as f32, (self.get_y()) as f32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), 0, 0, ((0) as f32) as f32, ((0) as f32) as f32, self.get_width(), self.get_height(), self.get_width(), self.get_height());
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if self.editing_slot_icon < 0 { compat::component_of(self.get_message().clone()) } else { component::literal("Custom Icon") })), 8, 6, mcrust::net::minecraft::util::common_colors::DARK_GRAY, false);
        self.hovered_slot = -1;
        let mut time_millis: i64 = compat::millis();
        let mut stacks: Vec<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = (if self.editing_slot_icon >= 0 { Some(self.custom_icon_options_items.iter().cloned().map(Some).collect::<Vec<_>>()) } else { self.items.clone() }).unwrap();
        for i in 0..(stacks.len() as i32) {
            let mut slot_x: i32 = 7i32.wrapping_add((i.wrapping_rem(5)).wrapping_mul(18));
            let mut slot_y: i32 = 17i32.wrapping_add((i.wrapping_div(5)).wrapping_mul(18));
            let mut hovered: bool = (slot_x.wrapping_add(self.get_x()) <= mouse_x && mouse_x < (slot_x.wrapping_add(self.get_x())).wrapping_add(18) && slot_y.wrapping_add(self.get_y()) <= mouse_y && mouse_y < (slot_y.wrapping_add(self.get_y())).wrapping_add(18));
            let mut infested: bool = self.infected_plots.iter().any(|e| *e == i);
            if hovered {
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_BACK_SPRITE.get_mut(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
            }
            let mut item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = stacks[(i) as usize].clone();
            let Some(mut item) = item.clone() else {
                if hovered {
                    gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_FRONT_SPRITE.get_mut(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
                }
                if infested && (time_millis & (512 as i64)) != (0 as i64) {
                    GuiHelper::border(graphics, slot_x.wrapping_add(1), slot_y.wrapping_add(1), 16, 16, mcrust::net::minecraft::util::common_colors::RED);
                }
                continue;
            };
            if hovered {
                // TODO(fabric2seer): the condition: Fabric API `ClientTags.isInLocal` has no SeerClient equivalent
                //   java: ClientTags.isInLocal(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    graphics.item(&item, slot_x.wrapping_add(1), slot_y.wrapping_add(1));
                } else {
                    let __pose_2 = gui_graphics_extractor::push_pose();
                    gui_graphics_extractor::pose_translate((slot_x) as f32, (slot_y) as f32);
                    gui_graphics_extractor::pose_scale((1.125f32) as f32, (1.125f32) as f32);
                    graphics.item(&item, 0, 0);
                    gui_graphics_extractor::pop_pose(__pose_2);
                }
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_FRONT_SPRITE.get_mut(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
                self.hovered_slot = i;
            } else {
                graphics.item(&item, slot_x.wrapping_add(1), slot_y.wrapping_add(1));
            }
            if self.editing_slot_icon >= 0 {
                if hovered {
                    // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
                    // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
                    // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut text_renderer, vec![compat::component_of(item.get_hover_name())], mouse_x, mouse_y);
                }
                continue;
            }
            if infested && (time_millis & (512 as i64)) != (0 as i64) {
                GuiHelper::border(graphics, slot_x.wrapping_add(1), slot_y.wrapping_add(1), 16, 16, mcrust::net::minecraft::util::common_colors::RED);
            }
            if hovered {
                let mut tooltip: Vec<MutableComponent> = (if infested { vec![component::translatable_args("skyblocker.gardenPlots.plot", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(item.get_hover_name()))]), GROSS_PEST_TEXT.get_mut().clone(), component::empty(), TP_TEXT.get_mut().clone()] } else { (if i == 12 { vec![compat::component_of(item.get_hover_name()), component::empty(), TP_TEXT.get_mut().clone()] } else { vec![component::translatable_args("skyblocker.gardenPlots.plot", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(item.get_hover_name()))]), component::empty(), TP_TEXT.get_mut().clone()] }) });
                // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
                // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
                // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut text_renderer, tooltip.clone(), mouse_x, mouse_y);
            }
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        for widget in self.widgets.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
        if time_millis.wrapping_sub(self.update_from_tab_time) > (3000 as i64) {
            self.update_from_tab_time = time_millis;
            self.update_infested_from_tab();
        }
        self.drag_area_hovered = (self.get_x().wrapping_add(4) < mouse_x && self.get_y().wrapping_add(4) < mouse_y && mouse_x < self.get_right().wrapping_sub(4) && mouse_y < self.get_y().wrapping_add(15));
        if self.drag_area_hovered {
            // TODO(fabric2seer): static `CursorTypes.RESIZE_ALL` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::RESIZE_ALL);
        }
    }

    /// java: private void updateInfestedFromTab()
    pub fn update_infested_from_tab(&mut self) {
        self.infected_plots.clear();
        for i in 0..(PlayerListManager::get_player_string_list().len() as i32) {
            let mut string: String = PlayerListManager::get_player_string_list()[(i) as usize].clone();
            if string.starts_with("Plots:") {
                let mut split: Vec<String> = compat::regex::split(&((compat::regex::split(&string, ":"))[(1) as usize]), ",");
                for s in split.iter() {
                    // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`
                    // java: try {..} catch (NumberFormatException _) {} - `Integer.parseInt(s.strip())`
                    if let Ok(plot) = s.trim_matches(is_java_whitespace).parse::<i32>() {
                        self.infected_plots.push(GARDEN_PLOT_TO_SLOT.get().get(&plot).cloned().unwrap_or(-1));
                    }
                }
                break;
            }
        }
    }

    /// java: private void setPositionFromConfig()
    pub fn set_position_from_config(&mut self) {
        { let __s0 = (self.inventory_rectangle.right().wrapping_add(SPACING)).wrapping_add((POSITION.get_mut())().x()); let __s1 = self.inventory_rectangle.top().wrapping_add((POSITION.get_mut())().y()); self.set_position(__s0, __s1) };
    }

    /// java: public void setInventoryRectangle(ScreenRectangle inventoryRectangle)
    pub fn set_inventory_rectangle(&mut self, mut inventory_rectangle: ScreenRectangle) {
        self.inventory_rectangle = inventory_rectangle.clone();
        self.set_position_from_config();
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(&mut self, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractContainerWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractContainerWidget and its supertypes)
        // rust: self.base.on_click(click, doubled);
        if self.drag_area_hovered {
            if click.button() == 1 && minecraft::has_shift_down() {
                { let __s0 = self.inventory_rectangle.right().wrapping_add(SPACING); let __s1 = self.inventory_rectangle.top(); self.set_position(__s0, __s1) };
                self.save_position_to_config();
            } else {
                self.dragging = Some(ScreenPosition::new(((click.x()) as i32).wrapping_sub(self.get_x()), ((click.y()) as i32).wrapping_sub(self.get_y())));
            }
        }
        if self.hovered_slot == -1 {
            return;
        }
        if self.editing_slot_icon >= 0 {
            let mut plot: Option<GardenPlot> = garden_plots::GARDEN_PLOTS.get()[(self.editing_slot_icon) as usize].clone();
            if let Some(plot) = &plot {
                garden_plots::GARDEN_PLOTS.get_mut()[(self.editing_slot_icon) as usize] = Some(plot.with_custom_icon(CUSTOM_ICON_OPTIONS[(self.hovered_slot) as usize].clone()));
            }
            self.editing_slot_icon = -1;
            self.update_plot_items();
            return;
        }
        if click.button() == 1 {
            self.editing_slot_icon = self.hovered_slot;
            self.custom_icon_options_items = CUSTOM_ICON_OPTIONS.iter().map(|s| {
                let Some(s) = s else {
                    return self.none_item.clone();
                };
                let Some(stack) = ItemRepository::get_item_stack(s) else {
                    return ItemUtils::get_item_id_placeholder(s);
                };
                stack
            }).map(|stack| FlexibleItemStack::get_stack_or_throw(&stack)).collect::<Vec<_>>();
            return;
        }
        if SkyblockerConfigManager::get().farming.plots_widget.close_screen_on_plot_click && compat::minecraft().gui.screen().is_some() {
            // rust: port gap - `minecraft.gui.screen().onClose()` from a mod: no game API runs the open screen's own onClose (GAPS_batchG)
            // TODO(fabric2seer): `on_close_for_interrupt` takes 12 argument(s) in the port (`gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, options_data: &mut crate::net::minecraft::client::options::OptionsData, open_sign_e ..`), Java passes 0
            // rust: screen::on_close_for_interrupt();
        }
        if self.hovered_slot == 12 {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/plottp barn", true);
        } else {
            // `GARDEN_PLOTS[hoveredSlot].name()`: a slot that shows an item has a plot
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/plottp {}", garden_plots::GARDEN_PLOTS.get()[(self.hovered_slot) as usize].as_ref().expect("NullPointerException").name()), true);
        }
    }

    /// java: @Override public void onRelease(MouseButtonEvent event)
    pub fn on_release(&mut self, event: &MouseButtonEvent) {
        // TODO(fabric2seer): `on_release` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): `ScrollArea::on_release` ports `onRelease`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: self.base.on_release(event.clone());
        if self.dragging.is_none() {
            return;
        }
        self.dragging = None;
        self.save_position_to_config();
    }

    /// java: private void savePositionToConfig()
    pub fn save_position_to_config(&self) {
        // the lambda runs before `update` returns: the values it reads from `this` are taken first
        let x = (self.get_x().wrapping_sub(self.inventory_rectangle.right())).wrapping_sub(SPACING);
        let y = self.get_y().wrapping_sub(self.inventory_rectangle.top());
        SkyblockerConfigManager::update(Arc::new(move |mut config| {
            config.farming.plots_widget.x = x;
            config.farming.plots_widget.y = y;
        }));
    }

    /// java: @Override protected void onDrag(MouseButtonEvent event, double dx, double dy)
    pub fn on_drag(&mut self, event: &MouseButtonEvent, mut dx: f64, mut dy: f64) {
        if self.dragging.is_none() {
            return;
        }
        { let __s0 = (((event.x()) as i32).wrapping_sub(self.dragging.as_ref().unwrap().x())).max(self.inventory_rectangle.right().wrapping_add(SPACING)); let __s1 = ((event.y()) as i32).wrapping_sub(self.dragging.as_ref().unwrap().y()); self.set_position(__s0, __s1) };
    }

    /// java: @Override protected boolean isValidClickButton(MouseButtonInfo input)
    pub fn is_valid_click_button(&self, input: &MouseButtonInfo) -> bool {
        // `super.isValidClickButton(input)` is AbstractWidget:137-139 (`AbstractContainerWidget` and `AbstractScrollArea`
        // do not override it): `input.button() == 0`
        return ((input.button() == 0 || input.button() == 1) && (self.hovered_slot != -1 || self.drag_area_hovered));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    ///
    /// The widgets themselves (Java's list holds the same objects).
    pub fn children(&self) -> &Vec<ItemButtonWidget> {
        return &self.widgets;
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.get_height();
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        let mut prev_x: i32 = self.get_x();
        self.base.set_x(x);
        let mut diff: i32 = x.wrapping_sub(prev_x);
        for widget in self.widgets.iter_mut() {
            let widget_x = widget.get_x();
            widget.set_x(widget_x.wrapping_add(diff));
        }
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        let mut prev_y: i32 = self.get_y();
        self.base.set_y(y);
        let mut diff: i32 = y.wrapping_sub(prev_y);
        for widget in self.widgets.iter_mut() {
            let widget_y = widget.get_y();
            widget.set_y(widget_y.wrapping_add(diff));
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if self.is_mouse_over(click.x(), click.y()) && self.is_valid_click_button(&click.button_info()) {
            self.on_click(click, doubled);
            return true;
        }
        // super.mouseClicked: AbstractContainerWidget:62-66 - the scrollbar pass, then ContainerEventHandler's children pass
        let widgets = &self.widgets;
        return self.base.mouse_clicked(
            // AbstractScrollArea.updateScrolling: `defaultSettings(0)` and contentHeight() == getHeight(), so no
            // scrollbar is visible and the click never arms one
            || false,
            // ContainerEventHandler.mouseClicked: the first child that takes the click
            // rust: port gap - running a child's `OnPress` needs the `Screen` it sits in, which a mod composite
            // widget is not lent (GAPS_batchG); the child is found, its press is not run
            || widgets.iter().any(|widget| widget.base.mouse_clicked(click.button(), click.x(), click.y()).is_some()),
        ).0;
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, isValidClickButton, mouseClicked, onClick, onDrag, onRelease, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// Java's `Character.isWhitespace` (what `String.strip` strips): not the no-break spaces.
fn is_java_whitespace(c: char) -> bool {
    match c {
        '\u{00A0}' | '\u{2007}' | '\u{202F}' | '\u{0085}' => false,
        '\u{001C}'..='\u{001F}' => true,
        _ => c.is_whitespace(),
    }
}
