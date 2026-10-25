//! Ported from de.hysky.skyblocker.skyblock.garden.visitor.VisitorHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::garden::visitor::visitor::Visitor;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::overlay_screen::OverlayScreen;
use crate::de::hysky::skyblocker::utils::area::Hub;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters::Formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::renderer::rect2i::Rect2i;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Set<Visitor> activeVisitors = new HashSet<>();
///
/// A list: a `Visitor`'s record equality reaches `ItemStack.equals`, which is identity, and every visitor added holds
/// its own `visitorHead.copy()` - so the set never holds two equal visitors and `add` never finds one.
pub static ACTIVE_VISITORS: compat::MainThread<Vec<Visitor>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Map<String, FlexibleItemStack> cachedItems = new HashMap<>();
pub static CACHED_ITEMS: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2IntMap<Component> groupedItems = new Object2IntOpenHashMap<>();
pub static GROUPED_ITEMS: compat::MainThread<std::collections::HashMap<MutableComponent, i32>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Map<Component, List<Visitor>> visitorsByItem = new LinkedHashMap<>();
pub static VISITORS_BY_ITEM: compat::MainThread<std::collections::HashMap<MutableComponent, Vec<Visitor>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static int xOffset = 4;
pub static X_OFFSET: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(4));
/// java: private static int yOffset = 4;
pub static Y_OFFSET: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(4));
/// java: private static int exclusionZoneWidth = 215;
pub static EXCLUSION_ZONE_WIDTH: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(215));
/// java: private static int exclusionZoneHeight = 215;
pub static EXCLUSION_ZONE_HEIGHT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(215));
/// java: private static final int ICON_SIZE = 16;
pub const ICON_SIZE: i32 = 16;
/// java: private static final int LINE_HEIGHT = 3;
pub const LINE_HEIGHT: i32 = 3;
/// java: private static final int PADDING = 4;
pub const PADDING: i32 = 4;
/// java: private static final Object2LongMap<Component> copiedTimestamps = new Object2LongOpenHashMap<>();
pub static COPIED_TIMESTAMPS: compat::MainThread<std::collections::HashMap<MutableComponent, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());
thread_local! {
    /// whether the open screen's AFTER_INIT registered `ScreenEvents.afterTick(screen)` (`initialize`)
    static AFTER_TICK_UPDATE_VISITORS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// java: private static boolean processVisitor = false;
pub static PROCESS_VISITOR: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class VisitorHelper extends AbstractWidget
#[derive(Clone)]
pub struct VisitorHelper {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private int dragStartX, dragStartY;
    pub drag_start_x: i32,
    /// java: private int dragStartX, dragStartY;
    pub drag_start_y: i32,
}

impl std::ops::Deref for VisitorHelper {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for VisitorHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl VisitorHelper {
    /// java: public VisitorHelper(int x, int y)
    pub fn new(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 0, 0, (&component::literal("Visitor Helper").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            drag_start_x: 0,
            drag_start_y: 0,
        });
        __this
    }

    /// java: @Init public static void initialize()
    pub fn initialize() {
        VisitorHelper::install_after_tick();
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // `ScreenEvents.afterTick(screen)` belongs to the screen being initialised: a new screen starts without it
            AFTER_TICK_UPDATE_VISITORS.with(|f| f.set(false));
            if !(VisitorHelper::should_render()) {
                return;
            }
            // `screen instanceof AbstractContainerScreen` / `AbstractSignEditScreen` (both sign screens are `ScreenKind::SignEdit`)
            let is_sign_edit_screen = matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::SignEdit);
            if !(compat::container_screen_open() || is_sign_edit_screen || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<OverlayScreen>>()) {
                return;
            }
            if compat::container_screen_open() {
                compat::set_state(&PROCESS_VISITOR, true);
                // java: ScreenEvents.afterTick(screen).register(_ -> updateVisitors(handledScreen.getMenu()));
                // (Fabric's per-screen `afterTick` is the end of `Screen.tick`, whose last statement for a container screen is
                // `containerTick()`: this screen's listener is run by `install_after_tick`'s RETURN injector there)
                AFTER_TICK_UPDATE_VISITORS.with(|f| f.set(true));
            }
            // java: Screens.getWidgets(screen).add(new VisitorHelper(xOffset, yOffset));
            screen.add_renderable_widget(compat::WidgetAdapter::widget(VisitorHelper::new(compat::get_state(&X_OFFSET), compat::get_state(&Y_OFFSET))));
        }));
        // java: ClientCommandRegistrationCallback.EVENT.register((dispatcher, _) -> dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   .then(literal("garden").then(literal("visitors")
        //     .then(literal("removeAll").executes(ctx -> { activeVisitors.clear(); updateItems();
        //       ctx.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.farming.visitorHelper.command.clearedAllVisitors"))); return Command.SINGLE_SUCCESS; }))
        //     .then(literal("remove").then(argument("visitor", StringArgumentType.greedyString()).executes(ctx -> {
        //       String name = ctx.getArgument("visitor", String.class).toLowerCase(Locale.ENGLISH);
        //       Optional<Visitor> visitor = activeVisitors.stream().filter(v -> v.name().getString().toLowerCase(Locale.ENGLISH).equals(name)).findAny();
        //       if (visitor.isEmpty()) { ctx.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.farming.visitorHelper.command.unableToRemoveVisitor"))); return Command.SINGLE_SUCCESS; }
        //       activeVisitors.remove(visitor.get()); updateItems();
        //       ctx.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatableEscape("skyblocker.farming.visitorHelper.command.removedVisitor", visitor.get().name()))); return Command.SINGLE_SUCCESS;
        //     }).suggests((_, builder) -> SharedSuggestionProvider.suggest(activeVisitors.stream().map(Visitor::name).map(Component::getString), builder)))))));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
                use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
                type Source = client_command_registration_callback::FabricClientCommandSource;
                let mut remove_all = client_command_registration_callback::literal("removeAll");
                remove_all.executes(Arc::new(|_: &CommandContext<Source>| {
                    ACTIVE_VISITORS.get_mut().clear();
                    VisitorHelper::update_items();
                    compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.farming.visitorHelper.command.clearedAllVisitors")));
                    Ok(1)
                }));
                let mut visitor = client_command_registration_callback::argument("visitor", Arc::new(StringArgumentType::greedy_string()));
                visitor.executes(Arc::new(|ctx: &CommandContext<Source>| {
                    let name: String = StringArgumentType::get_string(ctx, "visitor").to_lowercase();
                    let position = ACTIVE_VISITORS.get_mut().iter().position(|v| v.name().get_string().to_lowercase() == name);
                    let Some(position) = position else {
                        compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.farming.visitorHelper.command.unableToRemoveVisitor")));
                        return Ok(1);
                    };
                    let removed: Visitor = ACTIVE_VISITORS.get_mut().remove(position);
                    VisitorHelper::update_items();
                    compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable_escape("skyblocker.farming.visitorHelper.command.removedVisitor", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(removed.name())])));
                    Ok(1)
                }));
                let visitor_suggestions: mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<Source> = Arc::new(|_: &CommandContext<Source>, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| {
                    let names: Vec<String> = ACTIVE_VISITORS.get_mut().iter().map(|v| v.name().get_string()).collect();
                    Ok(mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&names, builder))
                });
                visitor.suggests(Some(visitor_suggestions));
                let mut remove = client_command_registration_callback::literal("remove");
                remove.then(&visitor);
                let mut visitors = client_command_registration_callback::literal("visitors");
                visitors.then(&remove_all);
                visitors.then(&remove);
                let mut garden = client_command_registration_callback::literal("garden");
                garden.then(&visitors);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&garden);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: `ScreenEvents.afterTick(screen).register(_ -> updateVisitors(handledScreen.getMenu()))` (see `initialize`): the
    /// end of the open container screen's tick (`AbstractContainerScreen.tick`'s `containerTick()`), while that screen's
    /// AFTER_INIT registered it.
    fn install_after_tick() {
        let result = mcrust::mcrust::modding::hook::inject(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#containerTick",
            mcrust::mcrust::modding::hook::At::Return,
            |_ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                if !AFTER_TICK_UPDATE_VISITORS.with(|f| f.get()) {
                    return;
                }
                // java: updateVisitors(handledScreen.getMenu());
                let Some(slots) = compat::with_open_menu(|menu, _| menu.slots.to_vec()) else {
                    return;
                };
                VisitorHelper::update_visitors(&slots);
            },
        );
        if let Err(e) = result {
            mcrust::mcrust::modding::log::warn_once("skyblocker-visitor-helper-after-tick", &format!("{e:?}"));
        }
    }

    /// java: public static boolean shouldRender()
    pub fn should_render() -> bool {
        let mut is_helper_enabled: bool = SkyblockerConfigManager::get().farming.visitor_helper.enabled;
        let mut is_garden_mode: bool = SkyblockerConfigManager::get().farming.visitor_helper.show_in_garden_only;
        return (is_helper_enabled && (!is_garden_mode || Utils::is_in_garden() || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Bazaar))) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:visitorHelper")));
    }

    /// java: public static List<Rect2i> getExclusionZones()
    pub fn get_exclusion_zones() -> Vec<Rect2i> {
        if ACTIVE_VISITORS.get_mut().is_empty() {
            return vec![];
        }
        return vec![Rect2i::new(compat::get_state(&X_OFFSET), compat::get_state(&Y_OFFSET), compat::get_state(&EXCLUSION_ZONE_WIDTH), compat::get_state(&EXCLUSION_ZONE_HEIGHT))];
    }

    /// java: private static void updateVisitors(AbstractContainerMenu handler)
    ///
    /// (`handler` is the open menu's slot list - `handler.getSlot(i)` is `handler[i]`; the slots' items are read through
    /// the player's open menu, which the caller does not hold)
    pub fn update_visitors(handler: &[Slot]) {
        if !(compat::get_state(&PROCESS_VISITOR)) {
            return;
        }
        // java: ItemStack visitorHead = handler.getSlot(13).getItem();
        let mut visitor_head: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&handler[13]);
        if visitor_head.is_empty() || !(visitor_head.has(mcrust::net::minecraft::core::component::data_components::lore())) || ItemUtils::get_lore_line_if(&visitor_head, Arc::new(move |t| t.contains("Times Visited"))).is_none() {
            return;
        }
        let mut visitor_name: MutableComponent = compat::component_of(visitor_head.get_hover_name());
        if ACTIVE_VISITORS.get_mut().iter().map(|__a0| __a0.name()).any(|__a0| (visitor_name == __a0)) {
            return;
        }
        let mut new_visitor: Visitor = Visitor::new(visitor_name.clone(), visitor_head.copy());
        VisitorHelper::extract_required_items(handler, &mut new_visitor);
        if !(new_visitor.required_items().is_empty()) {
            ACTIVE_VISITORS.get_mut().push(new_visitor);
        }
        VisitorHelper::update_items();
    }

    /// java: private static void extractRequiredItems(AbstractContainerMenu handler, Visitor visitor)
    pub fn extract_required_items(handler: &[Slot], visitor: &mut Visitor) {
        // java: ItemStack acceptButton = handler.getSlot(29).getItem();
        let mut accept_button: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&handler[29]);
        if accept_button.is_empty() || ItemUtils::get_lore_line_if(&accept_button, Arc::new(move |t| t.contains("Items Required"))).is_none() {
            return;
        }
        for requirement in accept_button.skyblocker_get_lore_strings().into_iter().map(|__a0| compat::trim(&__a0)).map(|__a0| chat_formatting::strip_formatting(&__a0)).skip_while(|lore| !(lore.contains("Items Required"))).skip((1) as usize).take_while(|lore| !lore.is_empty()) {
            let mut split: Vec<String> = compat::regex::split(&requirement, " x");
            let mut item: MutableComponent = component::null_to_empty(Some(&compat::trim(&(split[(0) as usize]))));
            if (split.len() as i32) == 1 {
                visitor.add_required_item(item.clone(), 1);
            } else {
                visitor.add_required_item(item.clone(), Formatters::parse_number(&compat::trim(&(split[(1) as usize]))).int_value());
            }
        }
    }

    /// java: private static void updateItems()
    pub fn update_items() {
        GROUPED_ITEMS.get_mut().clear();
        VISITORS_BY_ITEM.get_mut().clear();
        for visitor in ACTIVE_VISITORS.get_mut().iter() {
            for entry in visitor.required_items().iter().map(|(k, v)| (k.clone(), v.clone())) {
                let item_name = &entry.0;
                let mut amount: i32 = entry.1;
                GROUPED_ITEMS.get_mut().insert(item_name.clone(), (GROUPED_ITEMS.get_mut().get(item_name).cloned().unwrap_or_else(|| 0)).wrapping_add(amount));
                (VISITORS_BY_ITEM.get_mut().entry(item_name.clone()).or_insert_with_key(|k| (|_| Vec::new())(k.clone()))).push(visitor.clone());
            }
        }
    }

    /// java: private static FlexibleItemStack getCachedItem(String itemName)
    pub fn get_cached_item(item_name: &str) -> compat::Shared<FlexibleItemStack> {
        let mut clean_name: String = chat_formatting::strip_formatting(item_name);
        return CACHED_ITEMS.get_mut().entry(clean_name.clone()).or_insert_with_key(|k| (|_| {
            if NEURepoManager::is_loading() || !(ItemRepository::files_imported()) {
                return ItemUtils::get_named_placeholder(item_name);
            }
            return NEURepoManager::get_item_by_name(item_name)
                .into_iter()
                .next()
                .map(|item| item.get_skyblock_item_id().to_string())
                .and_then(|neu_id| ItemRepository::get_item_stack(&neu_id))
                .unwrap_or_else(|| ItemUtils::get_named_placeholder(item_name));
            //   java: return NEURepoManager.getItemByName(itemName)
            //   java: 					.stream()
            //   java: 					.filter(Objects::nonNull)
            //   java: 					.findFirst()
            //   java: 					.map(NEUItem::getSkyblockItemId)
            //   java: 					.map(ItemRepository::getItemStack)
            //   java: 					.orElseGet(() -> ItemUtils.getNamedPlaceholder(itemName));
        })(k.clone())).clone();
    }

    /// java: public static void onSlotClick(Slot slot, int slotId, String title, Slot visitorHeadSlot)
    pub fn on_slot_click(slot: &Slot, mut slot_id: i32, title: &str, visitor_head_slot: &Slot) {
        if (slot_id == 29 || slot_id == 13 || slot_id == 33) && compat::slot_has_item(slot) && ItemUtils::get_lore_line_if(&compat::slot_item(slot), Arc::new(move |s| (s == "Click to give!" || s == "Click to refuse!"))).is_some() {
            let visitor_head_has_item = compat::slot_has_item(visitor_head_slot);
            let visitor_head_item = compat::slot_item(visitor_head_slot);
            ACTIVE_VISITORS.get_mut().retain(|entry| !((entry.name().get_string() == title && visitor_head_has_item && ItemUtils::get_head_texture(&visitor_head_item) == ItemUtils::get_head_texture(&entry.head()))));
            compat::set_state(&PROCESS_VISITOR, false);
        }
        VisitorHelper::update_items();
    }

    /// java: private static void extractTextWithHoverUnderline(GuiGraphicsExtractor graphics, Font textRenderer, Component text, int x, int y, double mouseX, double mouseY)
    pub fn extract_text_with_hover_underline(graphics: &GuiGraphicsExtractor, text_renderer: &Font, text: &MutableComponent, mut x: i32, mut y: i32, mut mouse_x: f64, mut mouse_y: f64) {
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), x, y, mcrust::net::minecraft::util::common_colors::WHITE, true);
        if VisitorHelper::is_mouse_over_text(text_renderer, text, x, y, mouse_x, mouse_y) {
            gui_graphics_extractor::horizontal_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, x.wrapping_add(text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()))), y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: private static boolean isMouseOverText(Font textRenderer, Component text, int x, int y, double mouseX, double mouseY)
    pub fn is_mouse_over_text(text_renderer: &Font, text: &MutableComponent, mut x: i32, mut y: i32, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return GuiHelper::point_is_in_area(mouse_x, mouse_y, (x) as f64, (y) as f64, ((x.wrapping_add(text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone())))) as f64) as f64, ((y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)) as f64) as f64);
    }

    /// java: public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if ACTIVE_VISITORS.get_mut().is_empty() {
            return;
        }
        let mut text_renderer: Font = compat::font();
        let mut index: i32 = 0;
        let mut new_width: i32 = 0;
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(PADDING);
        let mut y: i32 = ((__this.lock().borrow().get_y()).wrapping_sub(((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) / 2.0f32) - (((ICON_SIZE as f32) * 0.95f32) / (2 as f32)))) as i32)).wrapping_add(PADDING);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), 0x18808080);
        for entry in GROUPED_ITEMS.get_mut().iter().map(|(k, v)| (k.clone(), v.clone())) {
            let item_name = &entry.0;
            let mut total_amount: i32 = entry.1;
            let mut visitors: Option<Vec<Visitor>> = VISITORS_BY_ITEM.get_mut().get(item_name).cloned();
            if visitors.is_none() || visitors.as_ref().unwrap().is_empty() {
                continue;
            }
            for visitor in visitors.as_ref().unwrap().iter() {
                let mut y_position: i32 = y.wrapping_add(index.wrapping_mul(LINE_HEIGHT.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)));
                let __pose_1 = gui_graphics_extractor::push_pose();
                gui_graphics_extractor::pose_translate((x) as f32, ((((y_position as f32) + (((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) as f32) / (2 as f32))) - (((ICON_SIZE as f32) * 0.95f32) / (2 as f32)))) as f32);
                gui_graphics_extractor::pose_scale((0.95f32) as f32, (0.95f32) as f32);
                graphics.item(&visitor.head(), 0, 0);
                gui_graphics_extractor::pop_pose(__pose_1);
                gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(visitor.name()), (x.wrapping_add((((ICON_SIZE as f32) * 0.95f32)) as i32)).wrapping_add(4), y_position, mcrust::net::minecraft::util::common_colors::WHITE, true);
                index += 1;
            }
            let mut icon_x: i32 = x.wrapping_add(12);
            let mut text_x: i32 = (icon_x.wrapping_add((((ICON_SIZE as f32) * 0.95f32)) as i32)).wrapping_add(4);
            let mut y_position: i32 = y.wrapping_add(index.wrapping_mul(LINE_HEIGHT.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)));
            let mut cached_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&VisitorHelper::get_cached_item(&item_name.get_string()));
            let __pose_2 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((icon_x) as f32, ((((y_position as f32) + (((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) as f32) / (2 as f32))) - (((ICON_SIZE as f32) * 0.95f32) / (2 as f32)))) as f32);
            gui_graphics_extractor::pose_scale((0.95f32) as f32, (0.95f32) as f32);
            graphics.item(&cached_stack, 0, 0);
            gui_graphics_extractor::pop_pose(__pose_2);
            let mut name: MutableComponent = compat::component_of(cached_stack.get_hover_name()).copy();
            let mut item_text: MutableComponent = (if SkyblockerConfigManager::get().farming.visitor_helper.show_in_stacks && total_amount >= 64 { name.append_str(&format!(" x{} stacks + {}", total_amount.wrapping_div(64), total_amount.wrapping_rem(64))) } else { name.append_str(&format!(" x{}", total_amount)) });
            if COPIED_TIMESTAMPS.get_mut().contains_key(item_name) {
                let mut time_since_copy: i64 = (compat::millis()).wrapping_sub(COPIED_TIMESTAMPS.get_mut().get(item_name).copied().unwrap_or_default());
                if time_since_copy < (1000 as i64) {
                    item_text = item_text.append((component::literal(" ✔ ")).with_style_format(ChatFormatting::Green));
                } else {
                    COPIED_TIMESTAMPS.get_mut().remove(item_name);
                }
            }
            new_width = new_width.max((text_x.wrapping_add(text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(item_text.clone())))).wrapping_sub(x));
            VisitorHelper::extract_text_with_hover_underline(graphics, &text_renderer, &item_text, text_x, y_position, (mouse_x) as f64, (mouse_y) as f64);
            index += 1;
        }
        { let (__q0,) = ((index.wrapping_mul(LINE_HEIGHT.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT))).wrapping_add(PADDING.wrapping_mul(2)),); __this.lock().borrow_mut().set_height(__q0) };
        { let (__q0,) = (new_width.wrapping_add(PADDING.wrapping_mul(2)),); __this.lock().borrow_mut().set_width(__q0) };
        compat::set_state(&EXCLUSION_ZONE_WIDTH, __this.lock().borrow().get_width());
        compat::set_state(&EXCLUSION_ZONE_HEIGHT, __this.lock().borrow().get_height());
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) {
        if ACTIVE_VISITORS.get_mut().is_empty() {
            return;
        }
        // java: setPosition(xOffset = (int) click.x() - dragStartX, yOffset = (int) click.y() - dragStartY);
        let (drag_start_x, drag_start_y) = compat::read(__this, |__t| (__t.drag_start_x, __t.drag_start_y));
        let x_offset: i32 = ((click.x()) as i32).wrapping_sub(drag_start_x);
        compat::set_state(&X_OFFSET, x_offset);
        let y_offset: i32 = ((click.y()) as i32).wrapping_sub(drag_start_y);
        compat::set_state(&Y_OFFSET, y_offset);
        compat::write(__this, |__t| __t.set_position(x_offset, y_offset));
    }

    /// java: public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if ACTIVE_VISITORS.get_mut().is_empty() {
            return;
        }
        let mut text_renderer: Font = compat::font();
        { let __v = ((click.x()) as i32).wrapping_sub(__this.lock().borrow().get_x()); compat::write(__this, |__t| __t.drag_start_x = __v); }
        { let __v = ((click.y()) as i32).wrapping_sub(__this.lock().borrow().get_y()); compat::write(__this, |__t| __t.drag_start_y = __v); }
        let mut index: i32 = 0;
        let mut y: i32 = ((__this.lock().borrow().get_y()).wrapping_sub(((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) / 2.0f32) - (((ICON_SIZE as f32) * 0.95f32) / (2 as f32)))) as i32)).wrapping_add(PADDING);
        for entry in GROUPED_ITEMS.get_mut().iter().map(|(k, v)| (k.clone(), v.clone())) {
            let item_name = &entry.0;
            let mut total_amount: i32 = entry.1;
            let mut visitors: Option<Vec<Visitor>> = VISITORS_BY_ITEM.get_mut().get(item_name).cloned();
            if visitors.is_some() && !(visitors.as_ref().unwrap().is_empty()) {
                index = index.wrapping_add(visitors.as_ref().unwrap().len() as i32);
                let mut icon_x: i32 = (__this.lock().borrow().get_x()).wrapping_add(12);
                let mut text_x: i32 = (icon_x.wrapping_add((((ICON_SIZE as f32) * 0.95f32)) as i32)).wrapping_add(4);
                let mut y_position: i32 = y.wrapping_add(index.wrapping_mul(LINE_HEIGHT.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)));
                let mut name: MutableComponent = item_name.copy();
                let mut item_text: MutableComponent = (if SkyblockerConfigManager::get().farming.visitor_helper.show_in_stacks && total_amount >= 64 { name.append_str(&format!(" x{} stacks + {}", total_amount.wrapping_div(64), total_amount.wrapping_rem(64))) } else { name.append_str(&format!(" x{}", total_amount)) });
                if VisitorHelper::is_mouse_over_text(&text_renderer, &item_text, text_x, y_position, click.x(), click.y()) {
                    // java: Minecraft.getInstance().keyboardHandler.setClipboard(String.valueOf(totalAmount));
                    {
                        let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
                        match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
                            Some(mut __glfw) => compat::minecraft().keyboard_handler.set_clipboard(&mut __glfw, &total_amount.to_string()),
                            None => compat::minecraft().keyboard_handler.set_clipboard(&mut __memory, &total_amount.to_string()),
                        }
                    }
                    COPIED_TIMESTAMPS.get_mut().insert(item_name.clone(), compat::millis());
                    MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/bz {}", item_name.get_string()), true);
                    return;
                }
                index += 1;
            }
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// VisitorHelper's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for VisitorHelper {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// VisitorHelper as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<VisitorHelper> {
    /// java: public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| VisitorHelper::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| VisitorHelper::update_widget_narration(__h, output))
    }

    /// java: public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| VisitorHelper::on_click(__h, event, double_click))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| VisitorHelper::on_drag(__h, event, dx, dy))
    }

}

// NOTE(fabric2seer): onDrag, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
