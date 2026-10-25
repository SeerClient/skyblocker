//! Ported from de.hysky.skyblocker.skyblock.auction.AuctionViewScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUI;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUIBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::mcrust::modding::container_screens::ModContainerScreen;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::button::Builder;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::item::item_stack::ItemStack;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: protected static final Identifier BACKGROUND_TEXTURE = SkyblockerMod.id("textures/gui/auctions_gui/view.png");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/auctions_gui/view.png"));
/// java: public static final int BACK_BUTTON_SLOT = 49;
pub const BACK_BUTTON_SLOT: i32 = 49;

/// java: public class AuctionViewScreen extends AbstractCustomHypixelGUI<AuctionHouseScreenHandler>
///
/// The four widgets the screen keeps are the screen's own (`addRenderableWidget`): the fields hold their indices in
/// the screen's widget list, and every write to them is made while the game lends the screen (its `init`, `tick`
/// and input calls - the slot changes are run there, see `abstract_custom_hypixel_gui::drain_slot_changes`).
pub struct AuctionViewScreen {
    /// java: extends AbstractCustomHypixelGUI
    pub base: AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>>,
    /// java: LinearLayout verticalLayout = LinearLayout.vertical();
    pub vertical_layout: LinearLayout,
    /// java: public final boolean isBinAuction;
    pub is_bin_auction: bool,
    /// java: private StringWidget priceWidget;
    pub price_widget: Option<usize>,
    /// java: private final Component clickToEditBidText = Component.translatable("skyblocker.fancyAuctionHouse.editBid").setStyle(Style.EMPTY.withUnderlined(true));
    pub click_to_edit_bid_text: MutableComponent,
    /// java: private StringWidget infoTextWidget;
    pub info_text_widget: Option<usize>,
    /// java: public String minBid = "";
    pub min_bid: Option<String>,
    /// java: private @Nullable BuyState buyState = null;
    pub buy_state: Option<BuyState>,
    /// java: private MutableComponent priceText = Component.literal("?");
    pub price_text: MutableComponent,
    /// java: private Button buyButton;
    pub buy_button: Option<usize>,
    /// java: private StringWidget priceTextWidget;
    pub price_text_widget: Option<usize>,
    /// java: private int buySlotID = -1;
    pub buy_slot_id: i32,
    /// java: private boolean priceParsed = false;
    pub price_parsed: bool,
    /// java: private boolean wonAuction = true;
    pub won_auction: bool,
    /// java: private boolean changeProfile = false;
    pub change_profile: bool,
    /// whether `init` ran before (a second `init` is Java's `repositionElements`: the popup over the screen closed)
    pub initialized: bool,
    /// the mouse as of the last frame: `extractRenderState`'s `priceWidget.setMessage(..)` is made in the next lent
    /// call (the game draws the screen with its widgets read-only)
    pub last_mouse: (i32, i32),
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for AuctionViewScreen {
    type Target = AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AuctionViewScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AuctionViewScreen {
    /// java: public AuctionViewScreen(AuctionHouseScreenHandler handler, Inventory inventory, Component title)
    pub fn new(handler: compat::Shared<AuctionHouseScreenHandler>, _inventory: compat::PlayerInventory, title: MutableComponent) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractCustomHypixelGUIBase::<compat::Shared<AuctionHouseScreenHandler>>::new(handler.clone(), compat::PlayerInventory, title.clone(), 187),
            self_ref: std::sync::Weak::new(),
            vertical_layout: LinearLayout::vertical(),
            is_bin_auction: false,
            price_widget: None,
            click_to_edit_bid_text: component::translatable("skyblocker.fancyAuctionHouse.editBid").set_style(Style::EMPTY.with_underlined(Some(true))),
            info_text_widget: None,
            min_bid: Some("".to_string()),
            buy_state: None,
            price_text: component::literal("?"),
            buy_button: None,
            price_text_widget: None,
            buy_slot_id: -1,
            price_parsed: false,
            won_auction: true,
            change_profile: false,
            initialized: false,
            last_mouse: (-1, -1),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // java: isBinAuction = this.getTitle().getString().toLowerCase(Locale.ENGLISH).contains("bin");
        { let __v = title.get_string().to_lowercase().contains("bin"); compat::write(&__this, |__t| __t.is_bin_auction = __v); }
        { let __v = 5; compat::write(&__this, |__t| __t.title_label_x = __v); }
        { let __v = 4; compat::write(&__this, |__t| __t.title_label_y = __v); }
        __this
    }

    /// `new AuctionViewScreen(handler, ..)` from the screen factory, which is handed the server's menu: the
    /// constructor's `handler.addSlotListener(this)` on it.
    pub fn new_listening(handler: compat::Shared<AuctionHouseScreenHandler>, chest: &mut ChestMenu, title: MutableComponent) -> compat::Shared<Self> {
        let __this = AuctionViewScreen::new(handler, compat::PlayerInventory, title);
        compat::write(&__this, |__t| __t.base.listen(chest));
        __this
    }

    /// `new StringWidget(message, font).setMaxWidth(imageWidth - 10, StringWidget.TextOverflow.SCROLLING)`
    fn string_widget(__this: &compat::Shared<Self>, message: MutableComponent) -> AbstractWidget {
        let max_width = compat::read(__this, |__t| __t.image_width) - 10;
        let mut widget = compat::string_widget(0, 0, -1, 9, message);
        widget.set_max_width(max_width, mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow::Scrolling);
        // `StringWidget.getWidth()` with a max width: `min(maxWidth, font.width(message))` (the port's widget keeps the
        // box in `width`)
        if widget.get_width() > max_width {
            widget.width = max_width;
        }
        widget
    }

    /// `widget.setMessage(message)` on one of the screen's widgets (a `StringWidget` keeps its `setMaxWidth` box)
    fn set_message(__this: &compat::Shared<Self>, index: Option<usize>, message: MutableComponent) {
        let Some(index) = index else { return; };
        // (`StringWidget.setMessage` marks the cached width dirty: `getWidth()` is `min(maxWidth, font.width(message))`
        // again)
        let text_width = compat::font().width_of(&FormattedText::from(message.clone()));
        compat::write(__this, |__t| {
            let widget = __t.screen.widget_mut(index);
            widget.set_message_text(FormattedText::from(message));
            if widget.max_width > 0 {
                widget.width = widget.max_width.min(text_width);
            }
        });
    }

    /// `widget.active = active` / `widget.visible = visible` on one of the screen's widgets
    fn with_widget(__this: &compat::Shared<Self>, index: Option<usize>, f: impl FnOnce(&mut AbstractWidget)) {
        let Some(index) = index else { return; };
        compat::write(__this, |__t| f(__t.screen.widget_mut(index)));
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: super.init();
        { let __v = LinearLayout::vertical(); compat::write(__this, |__t| __t.vertical_layout = __v); }
        // java: verticalLayout.spacing(2).defaultCellSetting().alignHorizontallyCenter(); - `spacing` answers `this`
        compat::write(__this, |__t| {
            let layout = std::mem::replace(&mut __t.vertical_layout, LinearLayout::vertical());
            __t.vertical_layout = layout.spacing(2);
            // (the port's `LayoutSettings` setters answer a new value: written back to the cell defaults)
            { let __s = __t.vertical_layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        });
        let is_bin = compat::read(__this, |__t| __t.is_bin_auction);
        // java: priceTextWidget = new StringWidget(isBinAuction ? ..price : ..newBid, font).setMaxWidth(..); verticalLayout.addChild(priceTextWidget);
        let __w = AuctionViewScreen::string_widget(__this, if is_bin { component::translatable("skyblocker.fancyAuctionHouse.price") } else { component::translatable("skyblocker.fancyAuctionHouse.newBid") });
        let __i = AuctionViewScreen::add_to_layout(__this, __w);
        compat::write(__this, |__t| __t.price_text_widget = Some(__i));
        // java: priceWidget = new StringWidget(Component.literal("?"), font).setMaxWidth(..); priceWidget.active = true;
        let mut __w = AuctionViewScreen::string_widget(__this, component::literal("?"));
        __w.active = true;
        let __i = AuctionViewScreen::add_to_layout(__this, __w);
        compat::write(__this, |__t| __t.price_widget = Some(__i));
        // java: infoTextWidget = new StringWidget(Component.literal("Can't Afford"), font).setMaxWidth(..);
        let __w = AuctionViewScreen::string_widget(__this, component::literal("Can't Afford"));
        let __i = AuctionViewScreen::add_to_layout(__this, __w);
        compat::write(__this, |__t| __t.info_text_widget = Some(__i));
        // java: buyButton = Button.builder(isBinAuction ? ..buy : ..bid, _ -> { if (buySlotID == -1) return; clickSlot(buySlotID); }).size(60, 15).build();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __w = button::builder((if is_bin { component::translatable("skyblocker.fancyAuctionHouse.buy") } else { component::translatable("skyblocker.fancyAuctionHouse.bid") }).get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            let Some(__this_1) = __this_w1.get().upgrade() else { return; };
            let buy_slot_id = compat::read(&__this_1, |__t| __t.buy_slot_id);
            if buy_slot_id == -1 {
                return;
            }
            __this_1.lock().borrow_mut().click_slot_1(buy_slot_id);
        }))).size(60, 15).build();
        let __i = AuctionViewScreen::add_to_layout(__this, __w);
        compat::write(__this, |__t| __t.buy_button = Some(__i));
        // java: verticalLayout.visitWidgets(this::addRenderableWidget);
        { let mut __vw = Vec::new(); LayoutElement::Grid(compat::read(__this, |__t| __t.vertical_layout.wrapped.clone())).visit_widgets(&mut __vw); compat::write(__this, |__t| for (__i, _, _) in __vw { __t.screen.mark_renderable(__i); }); }
        AuctionViewScreen::update_layout(__this);
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let (left_pos, top_pos, image_width) = compat::read(__this, |__t| (__t.left_pos, __t.top_pos, __t.image_width));
        let mut back_button: AbstractWidget = Builder::new((component::literal("<")).get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            let Some(__this_3) = __this_w3.get().upgrade() else { return; };
            __this_3.lock().borrow_mut().click_slot_1(BACK_BUTTON_SLOT);
        }))).pos(left_pos.wrapping_add(image_width).wrapping_sub(16), top_pos.wrapping_add(4)).size(12, 12).build();
        back_button.set_tooltip(Tooltip::create_text(FormattedText::from(component::literal("or press ESC!"))));
        // java: backButton.setTooltipDelay(Duration.ofSeconds(1));
        back_button.set_tooltip_delay(1000);
        compat::write(__this, |__t| __t.screen.add_renderable_widget(back_button));
        // a second `init` is `repositionElements` (the popup over the screen closed): the menu's items again
        if compat::write(__this, |__t| std::mem::replace(&mut __t.initialized, true)) {
            abstract_custom_hypixel_gui::reposition_elements::<compat::Shared<AuctionHouseScreenHandler>, Self>(__this);
        }
        abstract_custom_hypixel_gui::drain_slot_changes::<compat::Shared<AuctionHouseScreenHandler>, Self>(__this);
    }

    /// `verticalLayout.addChild(widget)`: the widget joins the screen's list (the layout holds it by index)
    fn add_to_layout(__this: &compat::Shared<Self>, widget: AbstractWidget) -> usize {
        let (__ww, __wh) = (widget.get_width(), widget.get_height());
        let __i = compat::write(__this, |__t| __t.screen.add_widget(widget));
        compat::write(__this, |__t| __t.vertical_layout.add_child(LayoutElement::widget(__i, __ww, __wh)));
        __i
    }

    /// java: private void changeState(BuyState newState)
    pub fn change_state(__this: &compat::Shared<Self>, new_state: BuyState) {
        if Some(new_state) == compat::read(__this, |__t| __t.buy_state) {
            return;
        }
        compat::write(__this, |__t| __t.buy_state = Some(new_state));
        let (info, buy, price, price_text_widget) = compat::read(__this, |__t| (__t.info_text_widget, __t.buy_button, __t.price_widget, __t.price_text_widget));
        match new_state {
            BuyState::CantAfford => {
                AuctionViewScreen::set_message(__this, info, component::translatable("skyblocker.fancyAuctionHouse.cantAfford").with_color(mcrust::net::minecraft::util::common_colors::RED));
                AuctionViewScreen::with_widget(__this, buy, |__w| __w.active = false);
            }
            BuyState::TopBid => {
                AuctionViewScreen::set_message(__this, info, component::translatable("skyblocker.fancyAuctionHouse.alreadyTopBid").with_color(mcrust::net::minecraft::util::common_colors::SOFT_YELLOW));
            }
            BuyState::Afford => {
                AuctionViewScreen::set_message(__this, info, component::empty());
                AuctionViewScreen::with_widget(__this, buy, |__w| { __w.active = true; __w.visible = true; });
            }
            BuyState::CollectAuction => {
                let (change_profile, won_auction) = compat::read(__this, |__t| (__t.change_profile, __t.won_auction));
                AuctionViewScreen::set_message(__this, info, if change_profile { component::translatable("skyblocker.fancyAuctionHouse.differentProfile") } else if won_auction { component::empty() } else { component::translatable("skyblocker.fancyAuctionHouse.didntWin") });
                AuctionViewScreen::with_widget(__this, price, |__w| __w.active = false);
                let message = if change_profile {
                    component::translatable("skyblocker.fancyAuctionHouse.changeProfile").set_style(Style::EMPTY.with_color_format(Some(ChatFormatting::Aqua)))
                } else if won_auction {
                    component::translatable("skyblocker.fancyAuctionHouse.collectAuction")
                } else {
                    component::translatable("skyblocker.fancyAuctionHouse.collectBid")
                };
                AuctionViewScreen::set_message(__this, buy, message);
                // java: buyButton.setWidth(font.width(buyButton.getMessage()) + 4);
                AuctionViewScreen::with_widget(__this, buy, |__w| { let __width = compat::font().width_of_cached(&FormattedText::from(compat::component_of(__w.get_message_component()))).wrapping_add(4); __w.set_width(__width); });
                AuctionViewScreen::set_message(__this, price_text_widget, component::translatable("skyblocker.fancyAuctionHouse.auctionEnded"));
            }
            BuyState::CancellableAuction => {
                AuctionViewScreen::set_message(__this, buy, component::translatable("skyblocker.fancyAuctionHouse.cancelAuction").set_style(Style::EMPTY.with_color_format(Some(ChatFormatting::Red))));
                AuctionViewScreen::with_widget(__this, buy, |__w| { let __width = compat::font().width_of_cached(&FormattedText::from(compat::component_of(__w.get_message_component()))).wrapping_add(4); __w.set_width(__width); __w.active = true; __w.visible = true; });
            }
            BuyState::OwnAuction => {
                AuctionViewScreen::with_widget(__this, buy, |__w| __w.visible = false);
                AuctionViewScreen::with_widget(__this, price, |__w| __w.active = false);
                AuctionViewScreen::set_message(__this, info, component::translatable("skyblocker.fancyAuctionHouse.yourAuction"));
            }
            BuyState::GracePeriod => {
                AuctionViewScreen::with_widget(__this, buy, |__w| __w.active = false);
                AuctionViewScreen::set_message(__this, info, component::translatable_args("skyblocker.fancyAuctionHouse.canBuyIn", vec![FormattedText::of("Unknown".to_string())]));
            }
        }
        AuctionViewScreen::update_layout(__this);
    }

    /// java: private void updateLayout()
    pub fn update_layout(__this: &compat::Shared<Self>) {
        // java: verticalLayout.arrangeElements(); FrameLayout.centerInRectangle(verticalLayout, leftPos, topPos + 36, imageWidth, 60);
        // (the port's layout leaf keeps the size it was given: brought up to the widgets' sizes first, and the widgets
        // take their positions from the centred layout)
        let (left_pos, top_pos, image_width) = compat::read(__this, |__t| (__t.left_pos, __t.top_pos, __t.image_width));
        let mut placed: Vec<(usize, i32, i32)> = Vec::new();
        compat::write(__this, |__t| {
            let mut indices: Vec<(usize, i32, i32)> = Vec::new();
            LayoutElement::Grid(__t.vertical_layout.wrapped.clone()).visit_widgets(&mut indices);
            let mut layout = LinearLayout::vertical().spacing(2);
            { let __s = layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
            for (__i, _, _) in &indices {
                let __w = __t.screen.widget(*__i);
                layout.add_child(LayoutElement::widget(*__i, __w.get_width(), __w.get_height()));
            }
            layout.arrange_elements();
            frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut layout.wrapped), left_pos, top_pos.wrapping_add(36), image_width, 60);
            LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut placed);
            __t.vertical_layout = layout;
        });
        compat::write(__this, |__t| for (__i, __x, __y) in placed { __t.screen.widget_mut(__i).set_position(__x, __y); });
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, _graphics: &GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        compat::call_super();
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), compat::read(__this, |__t| __t.left_pos), compat::read(__this, |__t| __t.top_pos), ((0) as f32) as f32, ((0) as f32) as f32, compat::read(__this, |__t| __t.image_width), compat::read(__this, |__t| __t.image_height), 256, 256);
    }

    /// `menu.getSlot(13).getItem()` - the auctioned item
    fn auctioned_item(__this: &compat::Shared<Self>) -> ItemStack {
        let container_id = compat::read(__this, |__t| __t.container_id);
        compat::with_open_menu(|menu, containers| (menu.container_id == container_id).then(|| menu.slots.get(13).map(|__s| __s.get_item(containers).duplicate())).flatten())
            .flatten()
            .unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty)
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, _a: f32) {
        // java: super.extractRenderState(graphics, mouseX, mouseY, a); - the game's
        if compat::read(__this, |__t| __t.is_waiting_for_server) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("Waiting..."), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((compat::read(__this, |__t| __t.left_pos)).wrapping_add(77)) as f32, ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(14)) as f32);
        gui_graphics_extractor::pose_scale((1.375f32) as f32, (1.375f32) as f32);
        // java: ItemStack stack = menu.getSlot(13).getItem();
        let stack: ItemStack = AuctionViewScreen::auctioned_item(__this);
        graphics.item(&stack, 0, 0);
        graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &stack, 0, 0);
        gui_graphics_extractor::pop_pose(__pose_1);
        // java: priceWidget.setMessage(priceWidget.isMouseOver(mouseX, mouseY) && .. ? clickToEditBidText : priceText);
        // (made in the next lent call: `apply_price_hover`)
        compat::write(__this, |__t| __t.last_mouse = (mouse_x, mouse_y));
        AuctionViewScreen::extract_tooltip(__this, graphics, mouse_x, mouse_y);
    }

    /// `extractRenderState`'s price-widget message swap, with the mouse of the last frame
    fn apply_price_hover(__this: &compat::Shared<Self>) {
        let (is_bin, buy_state, price, (mouse_x, mouse_y)) = compat::read(__this, |__t| (__t.is_bin_auction, __t.buy_state, __t.price_widget, __t.last_mouse));
        let Some(price) = price else { return; };
        if !is_bin && buy_state != Some(BuyState::CollectAuction) {
            let over = compat::read(__this, |__t| __t.screen.widget(price).is_mouse_over(mouse_x as f64, mouse_y as f64));
            let message = if over && buy_state != Some(BuyState::CantAfford) { compat::read(__this, |__t| __t.click_to_edit_bid_text.clone()) } else { compat::read(__this, |__t| __t.price_text.clone()) };
            AuctionViewScreen::set_message(__this, Some(price), message);
        }
    }

    /// java: @Override protected void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, _graphics: &mut GuiGraphicsExtractor, x: i32, y: i32) {
        // java: super.extractTooltip(graphics, x, y); - the game's (the hovered slot's tooltip)
        if x > (compat::read(__this, |__t| __t.left_pos)).wrapping_add(75) && x < ((compat::read(__this, |__t| __t.left_pos)).wrapping_add(75)).wrapping_add(26) && y > (compat::read(__this, |__t| __t.top_pos)).wrapping_add(13) && y < ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(13)).wrapping_add(26) {
            // java: ItemStack itemStack = menu.getSlot(13).getItem(); graphics.setTooltipForNextFrame(this.font, itemStack, x, y);
            let item_stack: ItemStack = AuctionViewScreen::auctioned_item(__this);
            // `setTooltipForNextFrame(font, stack, x, y)` is `setTooltipForNextFrame(font, Screen.getTooltipFromItem(minecraft,
            // stack), stack.getTooltipImage(), x, y, stack.get(TOOLTIP_STYLE))`: the lines parked in the frame's deferred
            // slot (`compat::batch_s_set_tooltip_for_next_frame`, which carries lines only - no image, default style)
            {
                let level = mcrust::mcrust::modding::game::level();
                let flag = mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::tooltip_flag_from_options();
                let lines = mcrust::mcrust::modding::game::with_player(|player| {
                    mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::get_tooltip_from_item(&item_stack, level.as_deref(), Some(&*player as &dyn mcrust::net::minecraft::world::entity::player::player::Player), flag)
                })
                .unwrap_or_else(|| mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::get_tooltip_from_item(&item_stack, level.as_deref(), None, flag));
                compat::batch_s_set_tooltip_for_next_frame(lines);
            }
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, _doubled: bool) -> bool {
        let (is_bin, price) = compat::read(__this, |__t| (__t.is_bin_auction, __t.price_widget));
        if !is_bin && price.is_some_and(|__p| compat::read(__this, |__t| __t.screen.widget(__p).is_mouse_over(click.x(), click.y()))) {
            __this.lock().borrow_mut().click_slot_1(31);
            return true;
        }
        return false;
    }

    /// java: private void getPriceFromTooltip(List<Component> tooltip)
    pub fn get_price_from_tooltip(__this: &compat::Shared<Self>, tooltip: &Vec<MutableComponent>) {
        if compat::read(__this, |__t| __t.price_parsed) {
            return;
        }
        let mut min_bid: Option<String> = None;
        let mut price_string: Option<String> = None;
        // java: AtomicReference<String> stringAtomicReference = new AtomicReference<>("");
        let mut string_atomic_reference: String = String::new();
        let is_bin = compat::read(__this, |__t| __t.is_bin_auction);
        for text in tooltip.iter() {
            let string: String = chat_formatting::strip_formatting(&text.get_string());
            let thing_to_look_for: String = (if is_bin { "price:" } else { "new bid:" }).to_string();
            let lower_case: String = string.to_lowercase();
            if lower_case.contains(&thing_to_look_for) {
                let split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                price_string = Some(compat::trim(&(split[(1) as usize])));
            } else if lower_case.contains("minimum bid:") && !is_bin {
                let split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                min_bid = Some(compat::trim(&(((split[(1) as usize]).replace("coins", "")).replace(",", ""))));
            } else if lower_case.contains("you pay:") {
                let split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                if compat::read(__this, |__t| __t.buy_state) != Some(BuyState::CantAfford) && !is_bin {
                    let info = compat::read(__this, |__t| __t.info_text_widget);
                    AuctionViewScreen::set_message(__this, info, component::translatable_args("skyblocker.fancyAuctionHouse.youPay", vec![FormattedText::of(compat::trim(&(split[(1) as usize])))]));
                }
            } else if lower_case.contains("top bid:") {
                compat::write(__this, |__t| __t.won_auction = false);
            } else if lower_case.contains("correct profile") {
                compat::write(__this, |__t| __t.change_profile = true);
                let price = compat::read(__this, |__t| __t.price_widget);
                AuctionViewScreen::set_message(__this, price, component::empty());
            } else if lower_case.contains("own auction") {
                AuctionViewScreen::change_state(__this, BuyState::OwnAuction);
            }
            // java: text.visit((style, asString) -> { String res = gold && !asString.matches(".*[]\\[+].*") && !asString.contains("Collect") ? asString : null; return Optional.ofNullable(res); }, Style.EMPTY)
            //   .ifPresent(s -> stringAtomicReference.set(stringAtomicReference.get() + s));
            let gold = TextColor::from_legacy_format(ChatFormatting::Gold);
            let mut found: Option<String> = None;
            FormattedText::from(text.clone()).visit_styled(&mut |style: &Style, as_string: &str| {
                if style.get_color() == gold && !compat::regex::Pattern::matches(".*[]\\[+].*", as_string) && !as_string.contains("Collect") {
                    found = Some(as_string.to_string());
                    return false;
                }
                true
            }, &Style::EMPTY);
            if let Some(s) = found {
                string_atomic_reference.push_str(&s);
            }
        }
        if price_string.is_none() {
            // java: priceString = stringAtomicReference.get();
            price_string = Some(string_atomic_reference.clone());
        }
        if let Some(min_bid) = &min_bid {
            { let __v = Some(min_bid.to_string()); compat::write(__this, |__t| __t.min_bid = __v); }
        } else {
            { let __v = price_string.clone(); compat::write(__this, |__t| __t.min_bid = __v); }
        }
        { let __v = component::literal(price_string.as_deref().unwrap()).set_style(Style::EMPTY.apply_format(ChatFormatting::Bold).with_color_format(Some(ChatFormatting::Gold))); compat::write(__this, |__t| __t.price_text = __v); }
        let (price, price_text) = compat::read(__this, |__t| (__t.price_widget, __t.price_text.clone()));
        AuctionViewScreen::set_message(__this, price, price_text);
        compat::write(__this, |__t| __t.price_parsed = true);
        AuctionViewScreen::update_layout(__this);
    }

    /// java: public PopupScreen getConfirmPurchasePopup(Component title)
    ///
    /// `new PopupScreen.Builder(this, title).addButton(..confirm.., _ -> click slot 11).addButton(..cancel..,
    /// PopupScreen::onClose).addMessage(..price..).onClose(() -> click slot 15).build()` (the popup is opened over this
    /// screen, which the game's screen stack keeps as its background)
    pub fn get_confirm_purchase_popup(__this: &compat::Shared<Self>, title: &MutableComponent) -> compat::BatchUPopupScreen {
        let (is_bin, price_text) = compat::read(__this, |__t| (__t.is_bin_auction, __t.price_text.clone()));
        // This really shouldn't be possible to be null in its ACTUAL use case.
        // java: this.minecraft.gameMode.handleContainerInput(this.minecraft.player.containerMenu.containerId, slot,
        //   ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
        let click = |slot: i32| {
            use mcrust::net::minecraft::world::entity::player::player::Player as _;
            let Some(container_id) = mcrust::mcrust::modding::game::with_player(|player| player.player().container_menu_container_id()) else { return; };
            compat::handle_container_input(container_id, slot, ContainerUtils::get_container_click_button(0), ContainerInput::Pickup);
        };
        compat::BatchUPopupScreenBuilder::new(title.clone())
            .add_button(component::translatable("text.skyblocker.confirm"), move |_: &compat::BatchUPopupScreenRef| click(11))
            .add_button(component::translatable("gui.cancel"), |popup: &compat::BatchUPopupScreenRef| popup.on_close())
            .add_message((if is_bin { component::translatable("skyblocker.fancyAuctionHouse.price") } else { component::translatable("skyblocker.fancyAuctionHouse.newBid") }).append_str(" ").append(price_text))
            .on_close(move || click(15))
            .build()
    }

    /// java: @SuppressWarnings("deprecation") @Override public void onSlotChange(AuctionHouseScreenHandler handler, int slotId, ItemStack stack)
    pub fn on_slot_change_impl(__this: &compat::Shared<Self>, handler: &compat::Shared<AuctionHouseScreenHandler>, slot_id: i32, stack: &ItemStack) {
        if stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) || slot_id >= (compat::read(handler, |__t| __t.get_row_count())).wrapping_mul(9) {
            return;
        }
        if slot_id == 13 {
            if compat::read(__this, |__t| __t.buy_state) == Some(BuyState::GracePeriod) {
                let line: Option<String> = ItemUtils::get_lore_line_if(stack, Arc::new(move |s| compat::trim(&s).starts_with("Can buy in: ")));
                if let Some(line) = &line {
                    let info = compat::read(__this, |__t| __t.info_text_widget);
                    AuctionViewScreen::set_message(__this, info, component::translatable_args("skyblocker.fancyAuctionHouse.canBuyIn", vec![FormattedText::from((component::literal(&compat::trim(&((compat::regex::split(&line, ":"))[(1) as usize])))).with_color_text(TextColor::YELLOW))]));
                } else {
                    // java: try { price = Double.parseDouble(..); } catch (NumberFormatException _) {}
                    let price_text = compat::read(__this, |__t| __t.price_text.get_string());
                    let price: f64 = price_text.replace("coins", "").replace(",", "").trim().parse::<f64>().unwrap_or(0.0);
                    if price <= Utils::get_purse() {
                        AuctionViewScreen::change_state(__this, BuyState::Afford);
                    } else {
                        AuctionViewScreen::change_state(__this, BuyState::CantAfford);
                    }
                }
            }
            return;
        }
        if stack.is_item(mcrust::net::minecraft::world::item::items::dyed_terracotta().red()) {
            AuctionViewScreen::change_state(__this, BuyState::CancellableAuction);
            compat::write(__this, |__t| __t.buy_slot_id = slot_id);
        }
        if compat::read(__this, |__t| __t.price_parsed) {
            return;
        }
        if stack.is_item(&items::items().get("poisonous_potato")) {
            AuctionViewScreen::change_state(__this, BuyState::CantAfford);
            AuctionViewScreen::get_price_from_tooltip(__this, &ItemUtils::get_lore(stack));
            compat::write(__this, |__t| __t.buy_slot_id = slot_id);
        } else if stack.is_item(&items::items().get("gold_nugget")) {
            AuctionViewScreen::change_state(__this, BuyState::Afford);
            AuctionViewScreen::get_price_from_tooltip(__this, &ItemUtils::get_lore(stack));
            compat::write(__this, |__t| __t.buy_slot_id = slot_id);
        } else if stack.is_item(&items::items().get("gold_block")) {
            AuctionViewScreen::change_state(__this, BuyState::TopBid);
            AuctionViewScreen::get_price_from_tooltip(__this, &ItemUtils::get_lore(stack));
            compat::write(__this, |__t| __t.buy_slot_id = slot_id);
        } else if stack.is_item(&items::items().get("name_tag")) {
            AuctionViewScreen::get_price_from_tooltip(__this, &ItemUtils::get_lore(stack));
            compat::write(__this, |__t| { __t.change_profile = true; __t.buy_slot_id = slot_id; });
        } else if stack.is_in_tag(&mcrust::net::minecraft::tags::item_tags::beds()) {
            // An item is in grace period for 20 seconds after the BIN auction started.
            AuctionViewScreen::change_state(__this, BuyState::GracePeriod);
            AuctionViewScreen::get_price_from_tooltip(__this, &ItemUtils::get_lore(stack));
            compat::write(__this, |__t| __t.buy_slot_id = slot_id);
        }
        let lower_case: String = compat::component_of(stack.get_hover_name()).get_string().to_lowercase();
        if compat::read(__this, |__t| __t.price_parsed) && lower_case.contains("collect auction") {
            AuctionViewScreen::change_state(__this, BuyState::CollectAuction);
        }
    }

    /// The start of every lent call: the slot changes the menu sent since the last one, and the last frame's hover
    fn lent_call(__this: &compat::Shared<Self>) {
        abstract_custom_hypixel_gui::drain_slot_changes::<compat::Shared<AuctionHouseScreenHandler>, Self>(__this);
        AuctionViewScreen::apply_price_hover(__this);
    }

}

impl AbstractCustomHypixelGUI<compat::Shared<AuctionHouseScreenHandler>> for AuctionViewScreen {
    fn abstract_custom_hypixel_gui_base(&self) -> &AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>> {
        &self.base
    }
    fn abstract_custom_hypixel_gui_base_mut(&mut self) -> &mut AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("deprecation") @Override public void onSlotChange(AuctionHouseScreenHandler handler, int slotId, ItemStack stack)
    fn on_slot_change(__this: &compat::Shared<Self>, handler: &compat::Shared<AuctionHouseScreenHandler>, slot_id: i32, stack: &ItemStack) {
        AuctionViewScreen::on_slot_change_impl(__this, handler, slot_id, stack)
    }

}

/// AuctionViewScreen's screen half (`super`'s fields), lent by the game for each call
impl compat::ContainerHalf for AuctionViewScreen {
    fn container_half(&mut self) -> &mut AbstractContainerScreen {
        &mut self.base.base
    }
}

/// AuctionViewScreen as the game's container screen (`container_screens::register`'s answer)
impl ModContainerScreen for compat::ContainerScreenAdapter<AuctionViewScreen> {
    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut AbstractContainerScreen) {
        // `lend_init`: the constructor's geometry on the game's (vanilla chest) screen, and `super.init()`'s centring
        self.lend_init(screen, |__h| AuctionViewScreen::init(__h))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        // `super.extractBackground` is `AbstractContainerScreen`'s (= `Screen`'s: the dimming), not the vanilla chest
        // screen's `generic_54` panel the game hands as `super_extract_background`
        let _ = super_extract_background;
        let __ctx: &WidgetRenderContext<'_> = &*context;
        let mut __super = || mcrust::mcrust::modding::container_screens::extract_abstract_container_screen_background(__ctx);
        compat::with_super(&mut __super, || self.lend_ref(screen, |__h| AuctionViewScreen::extract_background(__h, &__ctx.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| AuctionViewScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut AbstractContainerScreen, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(screen, |__h| { AuctionViewScreen::lent_call(__h); AuctionViewScreen::mouse_clicked(__h, event, double_click) })
    }

    /// `containerTick` - the slot changes (`slotChanged`) the menu sent since the last call
    fn container_tick(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| AuctionViewScreen::lent_call(__h))
    }

}

/// java: private enum BuyState
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BuyState {
    CantAfford,
    Afford,
    TopBid,
    CollectAuction,
    CancellableAuction,
    OwnAuction,
    GracePeriod,
}

impl BuyState {
    pub const VALUES: [BuyState; 7] = [BuyState::CantAfford, BuyState::Afford, BuyState::TopBid, BuyState::CollectAuction, BuyState::CancellableAuction, BuyState::OwnAuction, BuyState::GracePeriod];

    /// java: values()
    pub fn values() -> Vec<BuyState> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            BuyState::CantAfford => "CANT_AFFORD",
            BuyState::Afford => "AFFORD",
            BuyState::TopBid => "TOP_BID",
            BuyState::CollectAuction => "COLLECT_AUCTION",
            BuyState::CancellableAuction => "CANCELLABLE_AUCTION",
            BuyState::OwnAuction => "OWN_AUCTION",
            BuyState::GracePeriod => "GRACE_PERIOD",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> BuyState {
        match name {
            "CANT_AFFORD" => BuyState::CantAfford,
            "AFFORD" => BuyState::Afford,
            "TOP_BID" => BuyState::TopBid,
            "COLLECT_AUCTION" => BuyState::CollectAuction,
            "CANCELLABLE_AUCTION" => BuyState::CancellableAuction,
            "OWN_AUCTION" => BuyState::OwnAuction,
            "GRACE_PERIOD" => BuyState::GracePeriod,
            _ => panic!("IllegalArgumentException: No enum constant BuyState.{name}"),
        }
    }

}

impl std::fmt::Display for BuyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
