//! Ported from de.hysky.skyblocker.skyblock.auction.AuctionBrowserScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::auction_type_widget::AuctionTypeWidget;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::category_tab_widget::CategoryTabWidget;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::rarity_widget::RarityWidget;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::sort_widget::SortWidget;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUI;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUIBase;
use crate::de::hysky::skyblocker::utils::render::gui::side_tab_button_widget::SideTabButtonWidget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::container_screens::ModContainerScreen;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button::Builder;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::renderer::texture::texture_atlas_sprite::TextureAtlasSprite;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::slider_widget::SliderWidget as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/auctions_gui/browser.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/auctions_gui/browser.png"));
/// java: private static final Identifier SCROLLER_TEXTURE = Identifier.withDefaultNamespace("container/creative_inventory/scroller");
pub static SCROLLER_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/creative_inventory/scroller"));
/// java: private static final Identifier up_arrow_tex = SkyblockerMod.id("up_arrow_even");
pub static UP_ARROW_TEX: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("up_arrow_even"));
/// java: private static final Identifier down_arrow_tex = SkyblockerMod.id("down_arrow_even");
pub static DOWN_ARROW_TEX: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("down_arrow_even"));
//   java: public static final Supplier<TextureAtlasSprite> UP_ARROW = () -> Minecraft.getInstance().getAtlasManager().getAtlasOrThrow(AtlasIds.GUI).getSprite(up_arrow_tex);
/// java: public static final Supplier<TextureAtlasSprite> UP_ARROW = () -> Minecraft.getInstance().getAtlasManager().getAtlasOrThrow(AtlasIds.GUI).getSprite(up_arrow_tex);
pub static UP_ARROW: compat::MainThread<Arc<dyn Fn() -> TextureAtlasSprite>> = compat::MainThread::new(|| Arc::new(|| compat::minecraft().get_atlas_manager().expect("Minecraft.getAtlasManager() before the atlases loaded").get_atlas_or_throw(&mcrust::net::minecraft::data::atlas_ids::gui()).get_sprite(UP_ARROW_TEX.get()).clone()));
//   java: public static final Supplier<TextureAtlasSprite> DOWN_ARROW = () -> Minecraft.getInstance().getAtlasManager().getAtlasOrThrow(AtlasIds.GUI).getSprite(down_arrow_tex);
/// java: public static final Supplier<TextureAtlasSprite> DOWN_ARROW = () -> Minecraft.getInstance().getAtlasManager().getAtlasOrThrow(AtlasIds.GUI).getSprite(down_arrow_tex);
pub static DOWN_ARROW: compat::MainThread<Arc<dyn Fn() -> TextureAtlasSprite>> = compat::MainThread::new(|| Arc::new(|| compat::minecraft().get_atlas_manager().expect("Minecraft.getAtlasManager() before the atlases loaded").get_atlas_or_throw(&mcrust::net::minecraft::data::atlas_ids::gui()).get_sprite(DOWN_ARROW_TEX.get()).clone()));
/// java: public static final int RESET_BUTTON_SLOT = 47;
pub const RESET_BUTTON_SLOT: i32 = 47;
/// java: public static final int SEARCH_BUTTON_SLOT = 48;
pub const SEARCH_BUTTON_SLOT: i32 = 48;
/// java: public static final int BACK_BUTTON_SLOT = 49;
pub const BACK_BUTTON_SLOT: i32 = 49;
/// java: public static final int SORT_BUTTON_SLOT = 50;
pub const SORT_BUTTON_SLOT: i32 = 50;
/// java: public static final int RARITY_BUTTON_SLOT = 51;
pub const RARITY_BUTTON_SLOT: i32 = 51;
/// java: public static final int AUCTION_TYPE_BUTTON_SLOT = 52;
pub const AUCTION_TYPE_BUTTON_SLOT: i32 = 52;
/// java: public static final int PREV_PAGE_BUTTON = 46;
pub const PREV_PAGE_BUTTON: i32 = 46;
/// java: public static final int NEXT_PAGE_BUTTON = 53;
pub const NEXT_PAGE_BUTTON: i32 = 53;

/// java: public class AuctionBrowserScreen extends AbstractCustomHypixelGUI<AuctionHouseScreenHandler>
pub struct AuctionBrowserScreen {
    /// java: extends AbstractCustomHypixelGUI
    pub base: AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>>,
    /// java: private final Int2BooleanOpenHashMap isSlotHighlighted = new Int2BooleanOpenHashMap(24);
    pub is_slot_highlighted: std::collections::HashMap<i32, bool>,
    /// java: private SortWidget sortWidget;
    pub sort_widget: Option<compat::Shared<SortWidget>>,
    /// java: private AuctionTypeWidget auctionTypeWidget;
    pub auction_type_widget: Option<compat::Shared<AuctionTypeWidget>>,
    /// java: private RarityWidget rarityWidget;
    pub rarity_widget: Option<compat::Shared<RarityWidget>>,
    /// java: private Button resetFiltersButton; - its index in the screen's widgets (the screen holds the button)
    pub reset_filters_button: Option<usize>,
    /// whether `init` ran before (a second `init` is Java's `repositionElements`)
    pub initialized: bool,
    /// java: `AbstractContainerScreen.menu` - the handler this screen was built over
    pub menu: compat::Shared<AuctionHouseScreenHandler>,
    /// java: private final List<CategoryTabWidget> categoryTabWidgets = new ArrayList<>(6);
    pub category_tab_widgets: Vec<compat::Shared<CategoryTabWidget>>,
    /// java: private String search = "";
    pub search: String,
    /// java: int currentPage = 0;
    pub current_page: i32,
    /// java: int totalPages = 1;
    pub total_pages: i32,
    /// java: private boolean prevPageVisible = false;
    pub prev_page_visible: bool,
    /// java: private boolean nextPageVisible = false;
    pub next_page_visible: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for AuctionBrowserScreen {
    type Target = AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AuctionBrowserScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AuctionBrowserScreen {
    /// java: public AuctionBrowserScreen(AuctionHouseScreenHandler handler, Inventory inventory)
    pub fn new(mut handler: compat::Shared<AuctionHouseScreenHandler>, inventory: compat::PlayerInventory) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractCustomHypixelGUIBase::<compat::Shared<AuctionHouseScreenHandler>>::new(handler.clone(), compat::PlayerInventory, component::literal("Auctions Browser"), 187),
            self_ref: std::sync::Weak::new(),
            is_slot_highlighted: std::collections::HashMap::with_capacity((24) as usize),
            sort_widget: None,
            auction_type_widget: None,
            rarity_widget: None,
            reset_filters_button: None,
            initialized: false,
            menu: handler.clone(),
            category_tab_widgets: Vec::with_capacity((6) as usize),
            search: "".to_string(),
            current_page: 0,
            total_pages: 1,
            prev_page_visible: false,
            next_page_visible: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = 999; compat::write(&__this, |__t| __t.title_label_x = __v); }
        __this
    }

    /// `new AuctionBrowserScreen(handler, ..)` from the screen factory, which is handed the server's menu: the
    /// constructor's `handler.addSlotListener(this)` on it.
    pub fn new_listening(handler: compat::Shared<AuctionHouseScreenHandler>, chest: &mut mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu) -> compat::Shared<Self> {
        let __this = AuctionBrowserScreen::new(handler, compat::PlayerInventory);
        compat::write(&__this, |__t| __t.base.listen(chest));
        __this
    }

    /// java: private static int getOrdinal(List<String> tooltip)
    pub fn get_ordinal(tooltip: &Vec<String>) -> i32 {
        let mut ordinal: i32 = 0;
        for j in 0..(tooltip.len() as i32).wrapping_sub(4) {
            if j.wrapping_add(1) >= (tooltip.len() as i32) {
                break;
            }
            if (tooltip[(j.wrapping_add(1)) as usize]).contains("▶") {
                ordinal = j;
                break;
            }
        }
        return ordinal;
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(SortWidget::new((compat::read(__this, |__t| __t.left_pos)).wrapping_add(25), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(81), Arc::new(move |__a0: i32, __a1: i32| { let Some(__this_1) = __this_w1.get().upgrade() else { return; }; (&__this_1).lock().borrow_mut().click_slot(__a0, __a1) }))); compat::write(__this, |__t| __t.sort_widget = __v); }
        compat::write(&compat::read(__this, |__t| __t.sort_widget.clone()).unwrap(), |__t| __t.set_slot_id(SORT_BUTTON_SLOT));
        let __r0 = compat::read(__this, |__t| __t.sort_widget.clone());
        { let (__q0,) = (compat::WidgetAdapter::widget(__r0.unwrap()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(AuctionTypeWidget::new((compat::read(__this, |__t| __t.left_pos)).wrapping_add(134), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(77), Arc::new(move |__a0: i32, __a1: i32| { let Some(__this_2) = __this_w2.get().upgrade() else { return; }; (&__this_2).lock().borrow_mut().click_slot(__a0, __a1) }))); compat::write(__this, |__t| __t.auction_type_widget = __v); }
        compat::write(&compat::read(__this, |__t| __t.auction_type_widget.clone()).unwrap(), |__t| __t.set_slot_id(AUCTION_TYPE_BUTTON_SLOT));
        let __r0 = compat::read(__this, |__t| __t.auction_type_widget.clone());
        { let (__q0,) = (compat::WidgetAdapter::widget(__r0.unwrap()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(RarityWidget::new((compat::read(__this, |__t| __t.left_pos)).wrapping_add(73), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(80), Arc::new(move |__a0: i32, __a1: i32| { let Some(__this_3) = __this_w3.get().upgrade() else { return; }; (&__this_3).lock().borrow_mut().click_slot(__a0, __a1) }))); compat::write(__this, |__t| __t.rarity_widget = __v); }
        RarityWidget::set_slot_id(&compat::read(__this, |__t| __t.rarity_widget.clone()).unwrap(), RARITY_BUTTON_SLOT);
        let __r0 = compat::read(__this, |__t| __t.rarity_widget.clone());
        { let (__q0,) = (compat::WidgetAdapter::widget(__r0.unwrap()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut __reset: AbstractWidget = ScaledTextButtonWidget::new((compat::read(__this, |__t| __t.left_pos)).wrapping_add(10), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(77), 12, 12, component::literal("↻"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; { AuctionBrowserScreen::on_reset_pressed((&__this_4), (__screen.widget_mut(__index))); } }))).base;
        __reset.set_tooltip(Tooltip::create(component::literal("Reset Filters").get_string()));
        //   java: resetFiltersButton.setTooltipDelay(Duration.ofMillis(500));
        __reset.set_tooltip_delay(500);
        let __reset_index = compat::write(__this, |__t| __t.screen.add_renderable_widget(__reset));
        compat::write(__this, |__t| __t.reset_filters_button = Some(__reset_index));
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = (({ let (__q0, __q1,) = ((compat::read(__this, |__t| __t.left_pos)).wrapping_add(98), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(4),); (Builder::new((component::literal("<")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; { (&__this_5).lock().borrow_mut().click_slot_1(BACK_BUTTON_SLOT); } })))).pos(__q0, __q1) }).size(12, 12).build(),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        if __this.lock().borrow().category_tab_widgets.is_empty() {
            for i in 0..6 {
                let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
                let mut category_tab_widget: compat::Shared<CategoryTabWidget> = CategoryTabWidget::new(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("sponge")), Arc::new(move |__a0: i32, __a1: i32| { let Some(__this_6) = __this_w6.get().upgrade() else { return; }; (&__this_6).lock().borrow_mut().click_slot(__a0, __a1) }), i);
                let __h0 = category_tab_widget.clone();
                __this.lock().borrow_mut().category_tab_widgets.push(__h0);
                // (renderable: see `extract_render_state`)
                { let (__q0,) = (compat::WidgetAdapter::widget(category_tab_widget.clone()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
                let __wa1 = (compat::read(__this, |__t| __t.left_pos)).wrapping_sub(30);
                let __wa2 = ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(3)).wrapping_add(i.wrapping_mul(28));
                let __o7 = category_tab_widget.clone();
                compat::write(&__o7, |__t| __t.set_position(__wa1, __wa2));
            }
        } else {
            for i in { 0..(__this.lock().borrow().category_tab_widgets.len() as i32) } {
                let mut category_tab_widget: compat::Shared<CategoryTabWidget> = __this.lock().borrow().category_tab_widgets[(i) as usize].clone();
                let __wa3 = (compat::read(__this, |__t| __t.left_pos)).wrapping_sub(30);
                let __wa4 = ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(3)).wrapping_add(i.wrapping_mul(28));
                let __o8 = category_tab_widget.clone();
                compat::write(&__o8, |__t| __t.set_position(__wa3, __wa4));
                // (renderable: see `extract_render_state`)
                { let (__q0,) = (compat::WidgetAdapter::widget(category_tab_widget.clone()),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
            }
        }
    }

    /// java: private void onResetPressed(Button buttonWidget)
    pub fn on_reset_pressed(__this: &compat::Shared<Self>, button_widget: &mut AbstractWidget) {
        //   java: buttonWidget.setFocused(false);
        button_widget.set_focused(false, mcrust::net::minecraft::client::input_type::InputType::None);
        __this.lock().borrow_mut().click_slot(RESET_BUTTON_SLOT, 0);
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        compat::call_super();
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), compat::read(__this, |__t| __t.left_pos), compat::read(__this, |__t| __t.top_pos), ((0) as f32) as f32, ((0) as f32) as f32, compat::read(__this, |__t| __t.image_width), compat::read(__this, |__t| __t.image_height), 256, 256);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        //   java: for (CategoryTabWidget categoryTabWidget : categoryTabWidgets) categoryTabWidget.extractRenderState(graphics, mouseX, mouseY, a);
        // rust: port gap - a mod screen cannot draw one of its widgets from its own `extractRenderState` (the widget's draw
        // is the game's, over the screen's list); the tabs are drawn by the screen with its widgets instead (`init` adds
        // them renderable), i.e. under the container's slots rather than over them (GAPS_K)
        if compat::read(__this, |__t| __t.is_waiting_for_server) {
            let mut waiting: String = "Waiting for server...".to_string();
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&waiting), ((compat::read(__this, |__t| __t.screen.width)).wrapping_sub(compat::font().width(&waiting))).wrapping_sub(5), ((compat::read(__this, |__t| __t.screen.height)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_sub(2), mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((compat::read(__this, |__t| __t.left_pos)) as f32, (compat::read(__this, |__t| __t.top_pos)) as f32);
        graphics.enable_scissor(7, 4, 97, 16);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&compat::read(__this, |__t| __t.search.clone()))).with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(AuctionBrowserScreen::on_search_field(__this, mouse_x, mouse_y))))), 9, 6, mcrust::net::minecraft::util::common_colors::WHITE, true);
        graphics.disable_scissor();
        if compat::read(__this, |__t| __t.prev_page_visible) {
            if AuctionBrowserScreen::on_scrollbar_top(__this, mouse_x, mouse_y) {
                gui_graphics_extractor::blit_sprite_stretched_no_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(UP_ARROW.get_mut())(), 159, 13, 6, 3);
            } else {
                gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(UP_ARROW.get_mut())(), 159, 13, 6, 3, argb::color_rgb(137, 137, 137));
            }
        }
        if compat::read(__this, |__t| __t.next_page_visible) {
            if AuctionBrowserScreen::on_scrollbar_bottom(__this, mouse_x, mouse_y) {
                gui_graphics_extractor::blit_sprite_stretched_no_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(DOWN_ARROW.get_mut())(), 159, 72, 6, 3);
            } else {
                gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(DOWN_ARROW.get_mut())(), 159, 72, 6, 3, argb::color_rgb(137, 137, 137));
            }
        }
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("{}/{}", compat::read(__this, |__t| __t.current_page), compat::read(__this, |__t| __t.total_pages))), 111, 6, mcrust::net::minecraft::util::common_colors::GRAY, false);
        if compat::read(__this, |__t| __t.total_pages) <= 1 {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SCROLLER_TEXTURE.get_mut(), 156, 18, 12, 15);
        } else {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SCROLLER_TEXTURE.get_mut(), 156, ((((18 as f32) + ((((((compat::read(__this, |__t| __t.current_page)).min(compat::read(__this, |__t| __t.total_pages))).wrapping_sub(1)) as f32) / ((compat::read(__this, |__t| __t.total_pages)).wrapping_sub(1) as f32)) * (37 as f32)))) as i32), 12, 15);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        //   java: this.extractTooltip(graphics, mouseX, mouseY);
        // (`super.extractRenderState` - the game's `AbstractContainerScreen.extractRenderState:94` - has already run
        // `extractTooltip`, and `setTooltipForNextFrame` keeps the tooltip already set: the second call changes nothing)
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    pub fn extract_slot(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, slot: &Slot, mut mouse_x: i32, mut mouse_y: i32) {
        // TODO(fabric2seer): `has_item` takes 1 argument(s) in the port (`containers: &Containers<'_>`), Java passes 0
        if SkyblockerConfigManager::get().ui_and_visuals.fancy_auction_house.highlight_cheap_bin && compat::slot_has_item(slot) && __this.lock().borrow().is_slot_highlighted.get(&slot.index).cloned().unwrap_or_else(|| false) {
            GuiHelper::border(graphics, slot.x, slot.y, 16, 16, (Color::new_with_alpha(0, 255, 0, 100)).get_rgb());
        }
        compat::call_super();
    }

    /// java: @Override protected void slotClicked(Slot slot, int slotId, int button, ContainerInput actionType)
    pub fn slot_clicked(__this: &compat::Shared<Self>, slot: &Slot, mut slot_id: i32, mut button: i32, action_type: &ContainerInput) {
        //   java: if (slotId >= menu.getRowCount() * 9) return; - `menu` is the handler this screen was built over
        if slot_id >= compat::read(&compat::read(__this, |__t| __t.menu.clone()), |__h| __h.get_row_count()).wrapping_mul(9) {
            return;
        }
        compat::call_super();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.is_waiting_for_server) {
            return false;
        }
        if AuctionBrowserScreen::on_scrollbar_top(__this, ((click.x()) as i32), ((click.y()) as i32)) && compat::read(__this, |__t| __t.prev_page_visible) {
            __this.lock().borrow_mut().click_slot_1(PREV_PAGE_BUTTON);
            return true;
        }
        if AuctionBrowserScreen::on_scrollbar_bottom(__this, ((click.x()) as i32), ((click.y()) as i32)) && compat::read(__this, |__t| __t.next_page_visible) {
            __this.lock().borrow_mut().click_slot_1(NEXT_PAGE_BUTTON);
            return true;
        }
        if AuctionBrowserScreen::on_search_field(__this, ((click.x()) as i32), ((click.y()) as i32)) {
            __this.lock().borrow_mut().click_slot_1(SEARCH_BUTTON_SLOT);
            return true;
        }
        return false;
    }

    /// java: private boolean onScrollbarTop(int mouseX, int mouseY)
    pub fn on_scrollbar_top(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        let mut local_x: i32 = mouse_x.wrapping_sub(compat::read(__this, |__t| __t.left_pos));
        let mut local_y: i32 = mouse_y.wrapping_sub(compat::read(__this, |__t| __t.top_pos));
        return (local_x > 154 && local_x < 169 && local_y > 6 && local_y < 44);
    }

    /// java: private boolean onScrollbarBottom(int mouseX, int mouseY)
    pub fn on_scrollbar_bottom(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        let mut local_x: i32 = mouse_x.wrapping_sub(compat::read(__this, |__t| __t.left_pos));
        let mut local_y: i32 = mouse_y.wrapping_sub(compat::read(__this, |__t| __t.top_pos));
        return (local_x > 154 && local_x < 169 && local_y > 43 && local_y < 80);
    }

    /// java: private boolean onSearchField(int mouseX, int mouseY)
    pub fn on_search_field(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        let mut local_x: i32 = mouse_x.wrapping_sub(compat::read(__this, |__t| __t.left_pos));
        let mut local_y: i32 = mouse_y.wrapping_sub(compat::read(__this, |__t| __t.top_pos));
        return (local_x > 6 && local_x < 97 && local_y > 3 && local_y < 16);
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if input.is_up() && compat::read(__this, |__t| __t.prev_page_visible) {
            __this.lock().borrow_mut().click_slot_1(PREV_PAGE_BUTTON);
            return true;
        }
        if input.is_down() && compat::read(__this, |__t| __t.next_page_visible) {
            __this.lock().borrow_mut().click_slot_1(NEXT_PAGE_BUTTON);
            return true;
        }
        return false;
    }

    /// java: private void parsePage(ItemStack stack)
    pub fn parse_page(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tooltip: Vec<String> = stack.skyblocker_get_lore_strings();
            let mut str: String = chat_formatting::strip_formatting(&compat::trim(&tooltip[0]));
            str = compat::substring2(&str, 1, (compat::str_len(&str)).wrapping_sub(1));
            let mut parts: Vec<String> = compat::regex::split(&str, "/");
            // java: Integer.parseInt(parts[0].replace(",", "")) - no trim
            { let __v = ((parts[(0) as usize]).replace(",", "")).parse::<i32>().expect("java.lang.NumberFormatException"); compat::write(__this, |__t| __t.current_page = __v); }
            { let __v = ((parts[(1) as usize]).replace(",", "")).parse::<i32>().expect("java.lang.NumberFormatException"); compat::write(__this, |__t| __t.total_pages = __v); }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Fancy Auction House] Failed to parse page arrow"));
        }
    }

    /// java: @Override protected boolean hasClickedOutside(double mouseX, double mouseY, int left, int top)
    pub fn has_clicked_outside(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut left: i32, mut top: i32) -> bool {
        return (mouse_x < (((left) as f64) - (32 as f64)) || mouse_y < ((top) as f64) || mouse_x >= ((left.wrapping_add(compat::read(__this, |__t| __t.image_width))) as f64) || mouse_y >= ((top.wrapping_add(compat::read(__this, |__t| __t.image_height))) as f64));
    }

}

impl AbstractCustomHypixelGUI<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> for AuctionBrowserScreen {
    fn abstract_custom_hypixel_gui_base(&self) -> &AbstractCustomHypixelGUIBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> {
        &self.base
    }
    fn abstract_custom_hypixel_gui_base_mut(&mut self) -> &mut AbstractCustomHypixelGUIBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void onSlotChange(AuctionHouseScreenHandler handler, int slotId, ItemStack stack)
    fn on_slot_change(__this: &compat::Shared<Self>, handler: &compat::Shared<AuctionHouseScreenHandler>, slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        if stack.is_empty() {
            return;
        }
        compat::write(__this, |__t| __t.is_waiting_for_server = false);
        match slot_id {
            PREV_PAGE_BUTTON => {
                compat::write(__this, |__t| __t.prev_page_visible = false);
                if stack.is_item(&items::items().get("arrow")) {
                    compat::write(__this, |__t| __t.prev_page_visible = true);
                    AuctionBrowserScreen::parse_page(__this, stack);
                }
            }
            NEXT_PAGE_BUTTON => {
                compat::write(__this, |__t| __t.next_page_visible = false);
                if stack.is_item(&items::items().get("arrow")) {
                    compat::write(__this, |__t| __t.next_page_visible = true);
                    AuctionBrowserScreen::parse_page(__this, stack);
                }
            }
            SORT_BUTTON_SLOT => {
                { let (__a0,) = (crate::de::hysky::skyblocker::skyblock::auction::widgets::sort_widget::Option::get(AuctionBrowserScreen::get_ordinal(&stack.skyblocker_get_lore_strings())),); compat::read(__this, |__t| __t.sort_widget.clone()).unwrap().lock().borrow_mut().set_current(__a0) };
            }
            AUCTION_TYPE_BUTTON_SLOT => {
                { let (__a0,) = (crate::de::hysky::skyblocker::skyblock::auction::widgets::auction_type_widget::Option::get(AuctionBrowserScreen::get_ordinal(&stack.skyblocker_get_lore_strings())),); compat::read(__this, |__t| __t.auction_type_widget.clone()).unwrap().lock().borrow_mut().set_current(__a0) };
            }
            RARITY_BUTTON_SLOT => {
                let mut ordinal: i32 = AuctionBrowserScreen::get_ordinal(&stack.skyblocker_get_lore_strings());
                let mut tooltip: Vec<MutableComponent> = ItemUtils::get_lore(stack);
                let mut split: String = compat::substring(&((tooltip[(ordinal.wrapping_add(1)) as usize]).get_string()), 2);
                RarityWidget::set_text(&compat::read(__this, |__t| __t.rarity_widget.clone()).unwrap(), tooltip[(1) as usize..((tooltip.len() as i32).wrapping_sub(3)) as usize].to_vec(), split.clone());
            }
            RESET_BUTTON_SLOT => {
                //   java: resetFiltersButton.active = handler.getSlot(slotId).getItem().is(Items.ANVIL);
                // (the slot's item is the `stack` this slot-change callback carries; the field is the button's index in
                // the screen's widgets, see `init`)
                let __active = stack.is_item(&items::items().get("anvil"));
                if let Some(__i) = compat::read(__this, |__t| __t.reset_filters_button) {
                    compat::write(__this, |__t| __t.screen.widget_mut(__i).active = __active);
                }
            }
            SEARCH_BUTTON_SLOT => {
                let mut tooltip_search: Vec<String> = stack.skyblocker_get_lore_strings();
                for string in tooltip_search.iter() {
                    if string.contains("Filtered:") {
                        let string: String = chat_formatting::strip_formatting(string);
                        let mut split_search: Vec<String> = compat::regex::split(&string, ":");
                        if (split_search.len() as i32) < 2 {
                            compat::write(__this, |__t| __t.search = "".to_string());
                        } else {
                            { let __v = compat::trim(&(split_search[(1) as usize])); compat::write(__this, |__t| __t.search = __v); }
                        }
                        break;
                    }
                }
            }
            _ => {
                // java: slotId < this.menu.getRowCount() * 9 && slotId % 9 == 0 (`menu` is the handler this callback is given)
                let row_count: i32 = compat::read(handler, |__t| __t.get_row_count());
                if slot_id < row_count.wrapping_mul(9) && slot_id.wrapping_rem(9) == 0 {
                    let mut category_tab_widget: compat::Shared<CategoryTabWidget> = compat::read(__this, |__t| __t.category_tab_widgets[(slot_id.wrapping_div(9)) as usize].clone());
                    CategoryTabWidget::set_slot_id(&category_tab_widget, slot_id);
                    // java: categoryTabWidget.setIcon(handler.getSlot(slotId).getItem()); - the slot's item is the `stack` this
                    // slot-change callback carries (the menu's own slots hold no item in the port: they read the containers)
                    ({ let __f = compat::dispatch(&category_tab_widget, |__t| __t.set_icon()); __f(&compat::view(&category_tab_widget, |a| a as compat::Shared<dyn SideTabButtonWidget>), &mut stack.clone()) });
                    // java: List<String> tooltipDefault = handler.getSlot(slotId).getItem().skyblocker$getLoreStrings();
                    let mut tooltip_default: Vec<String> = stack.skyblocker_get_lore_strings();
                    {
                        let mut j: i32 = (tooltip_default.len() as i32).wrapping_sub(1);
                        while j >= 0 {
                            let mut lower_case: String = (tooltip_default[(j) as usize]).to_lowercase();
                            if lower_case.contains("currently") {
                                ({ let __f = compat::dispatch(&category_tab_widget, |__t| __t.select()); __f(&compat::view(&category_tab_widget, |a| a as compat::Shared<dyn SideTabButtonWidget>)) });
                                break;
                            } else if lower_case.contains("click") {
                                ({ let __f = compat::dispatch(&category_tab_widget, |__t| __t.unselect()); __f(&compat::view(&category_tab_widget, |a| a as compat::Shared<dyn SideTabButtonWidget>)) });
                                break;
                            } else {
                                ({ let __f = compat::dispatch(&category_tab_widget, |__t| __t.unselect()); __f(&compat::view(&category_tab_widget, |a| a as compat::Shared<dyn SideTabButtonWidget>)) });
                            }
                            j -= 1;
                        }
                    }
                } else if slot_id > 9 && slot_id < (row_count.wrapping_sub(1)).wrapping_mul(9) && slot_id.wrapping_rem(9) > 1 && slot_id.wrapping_rem(9) < 8 {
                    let mut data: Option<std::collections::HashMap<String, f64>> = tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow().get_data();
                    if !(SkyblockerConfigManager::get().ui_and_visuals.fancy_auction_house.highlight_cheap_bin) || data.is_none() {
                        return;
                    }
                    let mut tooltip: Vec<String> = stack.skyblocker_get_lore_strings();
                    {
                        let mut k: i32 = (tooltip.len() as i32).wrapping_sub(1);
                        while k >= 0 {
                            'continue_4: {
                                let mut string: String = tooltip[(k) as usize].clone();
                                if string.to_lowercase().contains("buy it now:") {
                                    let mut split: Vec<String> = compat::regex::split(&string, ":");
                                    if (split.len() as i32) < 2 {
                                        break 'continue_4;
                                    }
                                    let mut coins: String = compat::trim(&(((split[(1) as usize]).replace(",", "")).replace("coins", "")));
                                    //   java: long parsed = NumberUtils.toLong(coins, Long.MAX_VALUE); - `Long.parseLong`, the default on failure
                                    let mut parsed: i64 = coins.parse::<i64>().unwrap_or(i64::MAX);
                                    let mut price: f64 = data.as_ref().unwrap().get(&stack.get_neu_name()).copied().unwrap_or_default();
                                    compat::write(__this, |__t| __t.is_slot_highlighted.insert(slot_id, (price > (parsed as f64))));
                                }
                            }
                            k -= 1;
                        }
                    }
                }
            }
        }
    }

}

/// AuctionBrowserScreen's screen half (`super`'s fields), lent by the game for each call
impl compat::ContainerHalf for AuctionBrowserScreen {
    fn container_half(&mut self) -> &mut AbstractContainerScreen {
        &mut self.base.base
    }
}

/// AuctionBrowserScreen as the game's container screen (`container_screens::register`'s answer)
impl ModContainerScreen for compat::ContainerScreenAdapter<AuctionBrowserScreen> {
    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| {
            AuctionBrowserScreen::init(__h);
            // a second `init` is `repositionElements` (a popup over the screen closed): the menu's items again
            if compat::write(__h, |__t| std::mem::replace(&mut __t.initialized, true)) {
                abstract_custom_hypixel_gui::reposition_elements::<compat::Shared<AuctionHouseScreenHandler>, AuctionBrowserScreen>(__h);
            }
            abstract_custom_hypixel_gui::drain_slot_changes::<compat::Shared<AuctionHouseScreenHandler>, AuctionBrowserScreen>(__h);
        })
    }

    /// `containerTick` - the slot changes (`slotChanged`) the menu sent since the last call
    fn container_tick(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| abstract_custom_hypixel_gui::drain_slot_changes::<compat::Shared<AuctionHouseScreenHandler>, AuctionBrowserScreen>(__h))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        compat::with_super(super_extract_background, || self.lend_ref(screen, |__h| AuctionBrowserScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| AuctionBrowserScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut AbstractContainerScreen, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(screen, |__h| { abstract_custom_hypixel_gui::drain_slot_changes::<compat::Shared<AuctionHouseScreenHandler>, AuctionBrowserScreen>(__h); AuctionBrowserScreen::mouse_clicked(__h, event, double_click) })
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut AbstractContainerScreen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| AuctionBrowserScreen::key_pressed(__h, event))
    }

    /// java: @Override protected void slotClicked(Slot slot, int slotId, int button, ContainerInput actionType)
    fn slot_clicked(&mut self, screen: &mut AbstractContainerScreen, slot: Option<&Slot>, slot_id: i32, button_num: i32, container_input: ContainerInput, super_slot_clicked: &mut dyn FnMut(&mut AbstractContainerScreen, Option<i32>, i32, i32, ContainerInput)) {
        let __slot_index = slot.map(|s| s.index);
        // java's `null` slot (a click outside): the override's body takes a slot - the base runs
        let Some(slot) = slot else {
            super_slot_clicked(screen, None, slot_id, button_num, container_input);
            return;
        };
        let __object = self.object().clone();
        let mut __super = || compat::write(&__object, |__t| super_slot_clicked(compat::ContainerHalf::container_half(__t), __slot_index, slot_id, button_num, container_input));
        compat::with_super(&mut __super, || self.lend(screen, |__h| AuctionBrowserScreen::slot_clicked(__h, slot, slot_id, button_num, &container_input)))
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    fn extract_slot(&mut self, screen: &mut AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, slot: &Slot, mouse_x: i32, mouse_y: i32, super_extract_slot: &mut dyn FnMut(&mut AbstractContainerScreen)) {
        let __object = self.object().clone();
        let mut __super = || compat::write(&__object, |__t| super_extract_slot(compat::ContainerHalf::container_half(__t)));
        compat::with_super(&mut __super, || self.lend(screen, |__h| AuctionBrowserScreen::extract_slot(__h, &mut context.graphics, slot, mouse_x, mouse_y)))
    }

}

/// java: private static class ScaledTextButtonWidget extends Button
pub struct ScaledTextButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
}

impl std::ops::Deref for ScaledTextButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ScaledTextButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ScaledTextButtonWidget {
    /// java: protected ScaledTextButtonWidget(int x, int y, int width, int height, Component message, OnPress onPress)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut on_press: OnPress) -> Self {
        // java: super(x, y, width, height, message, onPress, Supplier::get); - a Button (the default narration supplier)
        let mut base = AbstractWidget::new(x, y, width, height, message.get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Button, on_press);
        base.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message));
        // rust: port gap (GAPS_batchB L4) - this Button subclass overrides extractContents (scaled text); the port's Button
        // draws itself and has no subclass slot, so the widget is drawn as a plain Button
        Self { base }
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `Button.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in Button and its supertypes)
        // rust: self.extract_default_sprite(graphics);
        let mut font: Font = compat::font();
        let mut text_scale: f32 = 2.0f32;
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((((((self.get_x() as f32) + ((self.width as f32) / 2.0f32)) - (((font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(self.get_message_component()))) as f32) * text_scale) / (2 as f32))) + (1 as f32))) as f32, (((((self.get_y()) as f32) + (((self.height as f32) - ((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) * text_scale)) / 2.0f32)) - (1 as f32))) as f32);
        gui_graphics_extractor::pose_scale((text_scale) as f32, (text_scale) as f32);
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(self.get_message_component())), 0, 0, (mcrust::net::minecraft::util::common_colors::WHITE | (mth::ceil_f32((self.alpha * 255.0f32))).wrapping_shl((24) as u32)), true);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

}

// NOTE(fabric2seer): extractContents override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
