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
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
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
        let mut desk_button: ItemButtonWidget = ItemButtonWidget::new_5(this.get_x().wrapping_add(7), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("book")), component::translatable("skyblocker.gardenPlots.openDesk"), Arc::new(move |_button: &mut ItemButtonWidget| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/desk", true); }));
        let mut spawn_button: ItemButtonWidget = ItemButtonWidget::new_5(((this.get_right().wrapping_sub(7)).wrapping_sub(40)).wrapping_sub(2), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("ender_eye")), component::translatable("skyblocker.gardenPlots.spawn"), Arc::new(move |_button: &mut ItemButtonWidget| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/warp garden", true); }));
        let mut set_spawn_button: ItemButtonWidget = ItemButtonWidget::new_5((this.get_right().wrapping_sub(7)).wrapping_sub(20), this.get_bottom().wrapping_sub(24), mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one((if compat::random() < 0.001 { mcrust::net::minecraft::world::item::items::bed().pink().clone() } else { mcrust::net::minecraft::world::item::items::bed().red().clone() })), component::translatable("skyblocker.gardenPlots.setSpawn"), Arc::new(move |_button: &mut ItemButtonWidget| { MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), "/setspawn", true); }));
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
    pub fn extract_widget_render_state(&mut self, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let (gl, client) = (context.gl, context.minecraft);
        let mut text_renderer: Font = compat::font();
        // java: Matrix3x2fStack matrices = graphics.pose(); matrices.pushMatrix(); matrices.translate(getX(), getY());
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((self.get_x()) as f32, (self.get_y()) as f32);
        gui_graphics_extractor::blit_identifier(gl, client, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get(), 0, 0, 0.0f32, 0.0f32, self.get_width(), self.get_height(), self.get_width(), self.get_height());
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(gl, client, &FormattedText::from((if self.editing_slot_icon < 0 { compat::component_of(self.get_message().clone()) } else { component::literal("Custom Icon") })), 8, 6, mcrust::net::minecraft::util::common_colors::DARK_GRAY, false);
        self.hovered_slot = -1;
        let mut time_millis: i64 = compat::millis();
        // java: @Nullable ItemStack[] stacks = editingSlotIcon >= 0 ? customIconOptionsItems : items;
        let mut stacks: Vec<Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = (if self.editing_slot_icon >= 0 { Some(self.custom_icon_options_items.iter().cloned().map(Some).collect::<Vec<_>>()) } else { self.items.clone() }).expect("NullPointerException");
        for i in 0..(stacks.len() as i32) {
            let mut slot_x: i32 = 7i32.wrapping_add((i.wrapping_rem(5)).wrapping_mul(18));
            let mut slot_y: i32 = 17i32.wrapping_add((i.wrapping_div(5)).wrapping_mul(18));
            let mut hovered: bool = (slot_x.wrapping_add(self.get_x()) <= mouse_x && mouse_x < (slot_x.wrapping_add(self.get_x())).wrapping_add(18) && slot_y.wrapping_add(self.get_y()) <= mouse_y && mouse_y < (slot_y.wrapping_add(self.get_y())).wrapping_add(18));
            let mut infested: bool = self.infected_plots.iter().any(|e| *e == i);
            if hovered {
                gui_graphics_extractor::blit_sprite_identifier(gl, client, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_BACK_SPRITE.get(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
            }
            let Some(mut item) = stacks[(i) as usize].clone() else {
                // Still show hover highlight & pest outline in empty slots.
                if hovered {
                    gui_graphics_extractor::blit_sprite_identifier(gl, client, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_FRONT_SPRITE.get(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
                }
                if infested && (time_millis & (512 as i64)) != (0 as i64) {
                    GuiHelper::border(&mut context.graphics, slot_x.wrapping_add(1), slot_y.wrapping_add(1), 16, 16, mcrust::net::minecraft::util::common_colors::RED);
                }
                continue;
            };
            if hovered {
                // java: if (ClientTags.isInLocal(ConventionalItemTags.GLASS_PANES, item.getItem().builtInRegistryHolder().key())) {
                if is_in_local_glass_panes(&item) {
                    context.graphics.item(&item, slot_x.wrapping_add(1), slot_y.wrapping_add(1));
                } else {
                    let __pose_2 = gui_graphics_extractor::push_pose();
                    gui_graphics_extractor::pose_translate((slot_x) as f32, (slot_y) as f32);
                    gui_graphics_extractor::pose_scale((1.125f32) as f32, (1.125f32) as f32);
                    context.graphics.item(&item, 0, 0);
                    gui_graphics_extractor::pop_pose(__pose_2);
                }
                gui_graphics_extractor::blit_sprite_identifier(gl, client, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_HIGHLIGHT_FRONT_SPRITE.get(), slot_x.wrapping_sub(3), slot_y.wrapping_sub(3), 24, 24);
                self.hovered_slot = i;
            } else {
                context.graphics.item(&item, slot_x.wrapping_add(1), slot_y.wrapping_add(1));
            }
            if self.editing_slot_icon >= 0 {
                if hovered {
                    // java: graphics.setComponentTooltipForNextFrame(textRenderer, List.of(item.getHoverName()), mouseX, mouseY);
                    compat::batch_s_set_tooltip_for_next_frame(vec![FormattedText::from(compat::component_of(item.get_hover_name()))]);
                }
                continue;
            }
            if infested && (time_millis & (512 as i64)) != (0 as i64) {
                GuiHelper::border(&mut context.graphics, slot_x.wrapping_add(1), slot_y.wrapping_add(1), 16, 16, mcrust::net::minecraft::util::common_colors::RED);
            }
            // tooltip
            if hovered {
                let mut tooltip: Vec<MutableComponent> = (if infested { vec![component::translatable_args("skyblocker.gardenPlots.plot", vec![FormattedText::from(compat::component_of(item.get_hover_name()))]), GROSS_PEST_TEXT.get().clone(), component::empty(), TP_TEXT.get().clone()] } else { (if i == 12 { vec![compat::component_of(item.get_hover_name()), component::empty(), TP_TEXT.get().clone()] } else { vec![component::translatable_args("skyblocker.gardenPlots.plot", vec![FormattedText::from(compat::component_of(item.get_hover_name()))]), component::empty(), TP_TEXT.get().clone()] }) });
                // java: graphics.setComponentTooltipForNextFrame(textRenderer, tooltip, mouseX, mouseY);
                compat::batch_s_set_tooltip_for_next_frame(tooltip.into_iter().map(FormattedText::from).collect());
            }
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        // java: for (ItemButtonWidget widget : widgets) widget.extractRenderState(graphics, mouseX, mouseY, a);
        for widget in self.widgets.iter_mut() {
            widget.extract_render_state(context, mouse_x, mouse_y, a);
        }
        if time_millis.wrapping_sub(self.update_from_tab_time) > (3000 as i64) {
            self.update_from_tab_time = time_millis;
            self.update_infested_from_tab();
        }
        self.drag_area_hovered = (self.get_x().wrapping_add(4) < mouse_x && self.get_y().wrapping_add(4) < mouse_y && mouse_x < self.get_right().wrapping_sub(4) && mouse_y < self.get_y().wrapping_add(15));
        // java: if (dragAreaHovered) graphics.requestCursor(CursorTypes.RESIZE_ALL); - asked after the frame (`handle_cursor`)
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
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick:100-101`, EMPTY (neither
        // `AbstractScrollArea` nor `AbstractContainerWidget` overrides it)
        if self.drag_area_hovered {
            if click.button() == 3 && minecraft::has_shift_down() {
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
        if click.button() == 3 {
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
            // java: Minecraft.getInstance().gui.screen().onClose();
            // The open screen is the `InventoryScreen` this widget sits on: `AbstractContainerScreen.onClose:580-588` is
            // `minecraft.player.closeContainer()` then `Screen.onClose:195-197`'s `minecraft.setScreen(null)`. The click
            // runs inside the screen's own mouse pass (the player may be held there), so the two statements run on the
            // client thread right after it, in Java's order.
            mcrust::mcrust::modding::game::execute(|| {
                mcrust::mcrust::modding::game::with_player(|player| player.close_container());
                mcrust::mcrust::modding::game::set_screen_null();
            });
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
        // java: super.onRelease(event); - `AbstractScrollArea.onRelease:59-61`, `scrolling = false` (this widget never
        // scrolls: `defaultSettings(0)` and `contentHeight() == getHeight()`)
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
        return ((input.button() == 1 || input.button() == 3) && (self.hovered_slot != -1 || self.drag_area_hovered));
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
        let widgets = &mut self.widgets;
        return self.base.mouse_clicked(
            // AbstractScrollArea.updateScrolling: `defaultSettings(0)` and contentHeight() == getHeight(), so no
            // scrollbar is visible and the click never arms one
            || false,
            // ContainerEventHandler.mouseClicked: the first child that takes the click
            || widgets.iter_mut().any(|widget| widget.mouse_clicked(click, doubled)),
        ).0;
    }

    /// java: `setPosition(x, y)` - `LayoutElement.setPosition:23-26`: `setX(x); setY(y);`, this class's overrides (which
    /// move the inner widgets too)
    pub fn set_position(&mut self, x: i32, y: i32) {
        GardenPlotsWidget::set_x(self, x);
        GardenPlotsWidget::set_y(self, y);
    }
}

impl compat::ObjectWidget for GardenPlotsWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, x: i32) {
        GardenPlotsWidget::set_x(self, x);
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, y: i32) {
        GardenPlotsWidget::set_y(self, y);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        GardenPlotsWidget::extract_widget_render_state(self, context, mouse_x, mouse_y, a);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) {}
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {}

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        GardenPlotsWidget::on_click(self, event, double_click);
    }

    /// java: @Override public void onRelease(MouseButtonEvent event)
    fn on_release(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) {
        GardenPlotsWidget::on_release(self, event);
    }

    /// java: @Override protected void onDrag(MouseButtonEvent event, double dx, double dy)
    fn on_drag(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        GardenPlotsWidget::on_drag(self, event, dx, dy);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        GardenPlotsWidget::mouse_clicked(self, event, double_click)
    }

    /// java: AbstractContainerWidget.mouseReleased:67-71 - `super.mouseReleased(event)` (`AbstractWidget:127-135`:
    /// `onRelease` for a valid button) then ContainerEventHandler's children pass (the `ItemButtonWidget`s: `Button` has no release work, see
    /// `mouse_clicked`)
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        if GardenPlotsWidget::is_valid_click_button(self, &event.button_info()) {
            GardenPlotsWidget::on_release(self, event);
        }
        false
    }

    /// java: AbstractContainerWidget.mouseDragged:74-77 - `super.mouseDragged` (`AbstractScrollArea:39-57`, never
    /// scrolling: `AbstractWidget:141-149`'s `onDrag` for a valid button) then the children pass (a `Button` does nothing on a drag)
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        if GardenPlotsWidget::is_valid_click_button(self, &event.button_info()) {
            GardenPlotsWidget::on_drag(self, event, dx, dy);
        }
        false
    }

    /// java: @Override protected boolean isValidClickButton(MouseButtonInfo input)
    fn is_valid_click_button(&self, button_info: MouseButtonInfo) -> bool {
        GardenPlotsWidget::is_valid_click_button(self, &button_info)
    }

    /// The class's `mouseClicked` override calls `onClick` directly: no `playDownSound` of its own.
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        None
    }

    /// java: `if (dragAreaHovered) graphics.requestCursor(CursorTypes.RESIZE_ALL);` in `extractWidgetRenderState`
    fn handle_cursor(&self, _widget: &AbstractWidget, cursors: &CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        if self.drag_area_hovered { Some(cursors.resize_all) } else { None }
    }
}

/// java: `GardenPlots.widget` - the widget object the inventory screen's list holds (`Screens.getWidgets(screen).add(widget)`),
/// read by other objects between the game's calls
#[derive(Clone)]
pub struct GardenPlotsWidgetRef(pub compat::Shared<GardenPlotsWidget>);

impl GardenPlotsWidgetRef {
    pub fn get_x(&self) -> i32 {
        compat::read(&self.0, |w| w.base.get_x())
    }

    pub fn get_y(&self) -> i32 {
        compat::read(&self.0, |w| w.base.get_y())
    }

    pub fn get_width(&self) -> i32 {
        compat::read(&self.0, |w| w.base.get_width())
    }

    pub fn get_height(&self) -> i32 {
        compat::read(&self.0, |w| w.base.get_height())
    }

    /// java: public void setInventoryRectangle(ScreenRectangle inventoryRectangle)
    pub fn set_inventory_rectangle(&self, inventory_rectangle: ScreenRectangle) {
        compat::write(&self.0, |w| w.set_inventory_rectangle(inventory_rectangle));
    }
}

/// java: `ClientTags.isInLocal(ConventionalItemTags.GLASS_PANES, item.getItem().builtInRegistryHolder().key())` - Fabric's
/// client tag lookup of the `c:glass_panes` item tag in the tag files the installed mods bundle. Fabric API's
/// `fabric-convention-tags-v2` (2.17.3) ships `data/c/tags/item/glass_panes.json`: the sixteen stained glass panes and
/// `#c:glass_panes/colorless` (`required: false`), which is `minecraft:glass_pane`.
fn is_in_local_glass_panes(item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    const GLASS_PANES: [&str; 17] = [
        "white_stained_glass_pane",
        "orange_stained_glass_pane",
        "magenta_stained_glass_pane",
        "light_blue_stained_glass_pane",
        "yellow_stained_glass_pane",
        "lime_stained_glass_pane",
        "pink_stained_glass_pane",
        "gray_stained_glass_pane",
        "light_gray_stained_glass_pane",
        "cyan_stained_glass_pane",
        "purple_stained_glass_pane",
        "blue_stained_glass_pane",
        "brown_stained_glass_pane",
        "green_stained_glass_pane",
        "black_stained_glass_pane",
        "red_stained_glass_pane",
        // #c:glass_panes/colorless
        "glass_pane",
    ];
    let held = item.get_item();
    GLASS_PANES.iter().any(|name| Arc::ptr_eq(&held, &items::items().get(name)))
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
