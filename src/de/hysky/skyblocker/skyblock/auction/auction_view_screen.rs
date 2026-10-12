//! Ported from de.hysky.skyblocker.skyblock.auction.AuctionViewScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUI;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUIBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;
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
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: protected static final Identifier BACKGROUND_TEXTURE = SkyblockerMod.id("textures/gui/auctions_gui/view.png");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/auctions_gui/view.png"));
/// java: public static final int BACK_BUTTON_SLOT = 49;
pub const BACK_BUTTON_SLOT: i32 = 49;

/// java: public class AuctionViewScreen extends AbstractCustomHypixelGUI<AuctionHouseScreenHandler>
pub struct AuctionViewScreen {
    /// java: extends AbstractCustomHypixelGUI
    pub base: AbstractCustomHypixelGUIBase<compat::Shared<AuctionHouseScreenHandler>>,
    /// java: LinearLayout verticalLayout = LinearLayout.vertical();
    pub vertical_layout: LinearLayout,
    /// java: public final boolean isBinAuction;
    pub is_bin_auction: bool,
    /// java: private StringWidget priceWidget;
    pub price_widget: Option<AbstractWidget>,
    /// java: private final Component clickToEditBidText = Component.translatable("skyblocker.fancyAuctionHouse.editBid").setStyle(Style.EMPTY.withUnderlined(true));
    pub click_to_edit_bid_text: MutableComponent,
    /// java: private StringWidget infoTextWidget;
    pub info_text_widget: Option<AbstractWidget>,
    /// java: public String minBid = "";
    pub min_bid: Option<String>,
    /// java: private @Nullable BuyState buyState = null;
    pub buy_state: Option<BuyState>,
    /// java: private MutableComponent priceText = Component.literal("?");
    pub price_text: MutableComponent,
    /// java: private Button buyButton;
    pub buy_button: Option<AbstractWidget>,
    /// java: private StringWidget priceTextWidget;
    pub price_text_widget: Option<AbstractWidget>,
    /// java: private int buySlotID = -1;
    pub buy_slot_id: i32,
    /// java: private boolean priceParsed = false;
    pub price_parsed: bool,
    /// java: private boolean wonAuction = true;
    pub won_auction: bool,
    /// java: private boolean changeProfile = false;
    pub change_profile: bool,
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
    pub fn new(mut handler: compat::Shared<AuctionHouseScreenHandler>, inventory: compat::PlayerInventory, mut title: MutableComponent) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractCustomHypixelGUIBase::<compat::Shared<AuctionHouseScreenHandler>>::new(handler.clone(), compat::PlayerInventory, title.clone(), 187),
            self_ref: std::sync::Weak::new(),
            vertical_layout: LinearLayout::vertical(),
            is_bin_auction: false,
            price_widget: None,
            click_to_edit_bid_text: component::translatable("skyblocker.fancyAuctionHouse.editBid").set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(true))),
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
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = __this.lock().borrow().screen.get_title().to_string().to_lowercase().contains("bin"); compat::write(&__this, |__t| __t.is_bin_auction = __v); }
        { let __v = 5; compat::write(&__this, |__t| __t.title_label_x = __v); }
        { let __v = 4; compat::write(&__this, |__t| __t.title_label_y = __v); }
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        { let __v = LinearLayout::vertical(); compat::write(__this, |__t| __t.vertical_layout = __v); }
        __this.lock().borrow_mut().vertical_layout.spacing(2).default_cell_setting().align_horizontally_center();
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: priceTextWidget = new StringWidget(isBinAuction ? Component.translatable("skyblocker.fancyAuctionHouse.price") : Component.translatable("skyblocker.fancyAuctionHouse.newBid"), font).setMaxWidth(imageWidth - 10, StringWidget.TextOverflow.SCROLLING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __r0 = compat::read(__this, |__t| __t.price_text_widget.clone());
        // rust: { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().vertical_layout.add_child(__q0) };
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: priceWidget = new StringWidget(Component.literal("?"), font).setMaxWidth(imageWidth - 10, StringWidget.TextOverflow.SCROLLING);
        __this.lock().borrow_mut().price_widget.as_mut().unwrap().active = true;
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __r0 = compat::read(__this, |__t| __t.price_widget.clone());
        // rust: { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().vertical_layout.add_child(__q0) };
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: infoTextWidget = new StringWidget(Component.literal("Can't Afford"), font).setMaxWidth(imageWidth - 10, StringWidget.TextOverflow.SCROLLING);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __r0 = compat::read(__this, |__t| __t.info_text_widget.clone());
        // rust: { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().vertical_layout.add_child(__q0) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some((button::builder((if compat::read(__this, |__t| __t.is_bin_auction) { component::translatable("skyblocker.fancyAuctionHouse.buy") } else { component::translatable("skyblocker.fancyAuctionHouse.bid") }).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            if compat::read((&__this_1), |__t| __t.buy_slot_id) == -1 {
                return;
            }
            let __r0 = compat::read((&__this_1), |__t| __t.buy_slot_id);
            { let (__a0,) = (__r0,); (&__this_1).lock().borrow_mut().click_slot_1(__a0) };
        } })))).size(60, 15).build()); compat::write(__this, |__t| __t.buy_button = __v); }
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: let __r0 = compat::read(__this, |__t| __t.buy_button.clone());
        // rust: { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().vertical_layout.add_child(__q0) };
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: layout::visit_widgets(|__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let (__q0,) = (__a0.clone(),); (&__this_2).lock().borrow_mut().screen.add_renderable_widget(__q0) }; } });
        AuctionViewScreen::update_layout(__this);
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut back_button: AbstractWidget = ({ let (__q0,) = (Some((Tooltip::create((component::literal("or press ESC!")).get_string())).message.clone()),); ({ let (__q0, __q1,) = (((compat::read(__this, |__t| __t.left_pos)).wrapping_add(compat::read(__this, |__t| __t.image_width))).wrapping_sub(16), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(4),); (Builder::new((component::literal("<")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { (&__this_3).lock().borrow_mut().click_slot_1(BACK_BUTTON_SLOT); } })))).pos(__q0, __q1) }).size(12, 12).tooltip_text(__q0) }).build();
        // TODO(fabric2seer): `Button.setTooltipDelay` has no Rust method in the port (looked for `set_tooltip_delay` in Button and its supertypes)
        // rust: back_button.set_tooltip_delay(&std::time::Duration::from_secs((1) as u64));
        { let (__q0,) = (back_button.clone(),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
    }

    /// java: private void changeState(BuyState newState)
    pub fn change_state(__this: &compat::Shared<Self>, mut new_state: BuyState) {
        if new_state == compat::read(__this, |__t| __t.buy_state.clone()).unwrap() {
            return;
        }
        { let __v = Some(new_state); compat::write(__this, |__t| __t.buy_state = __v); }
        match compat::read(__this, |__t| __t.buy_state.clone()) {
            Some(BuyState::CantAfford) => {
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.cantAfford").with_color(mcrust::net::minecraft::util::common_colors::RED)),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().active = false;
            }
            Some(BuyState::TopBid) => {
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.alreadyTopBid").with_color(mcrust::net::minecraft::util::common_colors::SOFT_YELLOW)),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
            }
            Some(BuyState::Afford) => {
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty()),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().active = true;
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().visible = true;
            }
            Some(BuyState::CollectAuction) => {
                let __r0 = compat::read(__this, |__t| __t.change_profile);
                let __r1 = compat::read(__this, |__t| __t.won_auction);
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if __r0 { component::translatable("skyblocker.fancyAuctionHouse.differentProfile") } else { (if __r1 { component::empty() } else { component::translatable("skyblocker.fancyAuctionHouse.didntWin") }) })),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
                __this.lock().borrow_mut().price_widget.as_mut().unwrap().active = false;
                if compat::read(__this, |__t| __t.change_profile) {
                    { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.changeProfile").set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(Some(ChatFormatting::Aqua)))),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_message_text(__q0) };
                } else if compat::read(__this, |__t| __t.won_auction) {
                    { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.collectAuction")),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_message_text(__q0) };
                } else {
                    { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.collectBid")),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_message_text(__q0) };
                }
                { let (__q0,) = ((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(__this.lock().borrow().buy_button.as_ref().unwrap().get_message_component())))).wrapping_add(4),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_width(__q0) };
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.auctionEnded")),); __this.lock().borrow_mut().price_text_widget.as_mut().unwrap().set_message_text(__q0) };
            }
            Some(BuyState::CancellableAuction) => {
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.cancelAuction").set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(Some(ChatFormatting::Red)))),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_message_text(__q0) };
                { let (__q0,) = ((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(__this.lock().borrow().buy_button.as_ref().unwrap().get_message_component())))).wrapping_add(4),); __this.lock().borrow_mut().buy_button.as_mut().unwrap().set_width(__q0) };
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().active = true;
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().visible = true;
            }
            Some(BuyState::OwnAuction) => {
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().visible = false;
                __this.lock().borrow_mut().price_widget.as_mut().unwrap().active = false;
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.fancyAuctionHouse.yourAuction")),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
            }
            Some(BuyState::GracePeriod) => {
                __this.lock().borrow_mut().buy_button.as_mut().unwrap().active = false;
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.fancyAuctionHouse.canBuyIn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of("Unknown".to_string())])),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
            }
            _ => {}
        }
        AuctionViewScreen::update_layout(__this);
    }

    /// java: private void updateLayout()
    pub fn update_layout(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().vertical_layout.arrange_elements();
        frame_layout::center_in_rectangle(&LayoutElement::Grid(compat::read(__this, |__t| __t.vertical_layout.clone()).wrapped), compat::read(__this, |__t| __t.left_pos), (compat::read(__this, |__t| __t.top_pos)).wrapping_add(36), compat::read(__this, |__t| __t.image_width), 60);
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        compat::call_super();
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), compat::read(__this, |__t| __t.left_pos), compat::read(__this, |__t| __t.top_pos), ((0) as f32) as f32, ((0) as f32) as f32, compat::read(__this, |__t| __t.image_width), compat::read(__this, |__t| __t.image_height), 256, 256);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        if compat::read(__this, |__t| __t.is_waiting_for_server) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("Waiting..."), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((compat::read(__this, |__t| __t.left_pos)).wrapping_add(77)) as f32, ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(14)) as f32);
        gui_graphics_extractor::pose_scale((1.375f32) as f32, (1.375f32) as f32);
        // TODO(fabric2seer): `.getSlot(..)` on a T
        //   java: ItemStack stack = menu.getSlot(13).getItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        graphics.item(&stack, 0, 0);
        graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &stack, 0, 0);
        gui_graphics_extractor::pop_pose(__pose_1);
        if !(compat::read(__this, |__t| __t.is_bin_auction)) && compat::read(__this, |__t| __t.buy_state.clone()).unwrap() != BuyState::CollectAuction {
            if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().price_widget.as_ref().unwrap().is_mouse_over(__q0, __q1) } && compat::read(__this, |__t| __t.buy_state.clone()).unwrap() != BuyState::CantAfford {
                let __r0 = compat::read(__this, |__t| __t.click_to_edit_bid_text.clone());
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__r0),); __this.lock().borrow_mut().price_widget.as_mut().unwrap().set_message_text(__q0) };
            } else {
                let __r0 = compat::read(__this, |__t| __t.price_text.clone());
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__r0),); __this.lock().borrow_mut().price_widget.as_mut().unwrap().set_message_text(__q0) };
            }
        }
        AuctionViewScreen::extract_tooltip(__this, graphics, mouse_x, mouse_y);
    }

    /// java: @Override protected void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        // TODO(fabric2seer): `AbstractContainerScreen.extractTooltip` has no Rust method in the port (looked for `extract_tooltip` in AbstractContainerScreen and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_tooltip(graphics, x, y);
        if x > (compat::read(__this, |__t| __t.left_pos)).wrapping_add(75) && x < ((compat::read(__this, |__t| __t.left_pos)).wrapping_add(75)).wrapping_add(26) && y > (compat::read(__this, |__t| __t.top_pos)).wrapping_add(13) && y < ((compat::read(__this, |__t| __t.top_pos)).wrapping_add(13)).wrapping_add(26) {
            // TODO(fabric2seer): `.getSlot(..)` on a T
            //   java: ItemStack itemStack = menu.getSlot(13).getItem();
            let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `itemStack` (TODO above)");
            // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &item_stack, x, y);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !(compat::read(__this, |__t| __t.is_bin_auction)) && { let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().price_widget.as_ref().unwrap().is_mouse_over(__q0, __q1) } {
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
        // TODO(fabric2seer): `new AtomicReference(..)`
        //   java: AtomicReference<String> stringAtomicReference = new AtomicReference<>("");
        let mut string_atomic_reference: crate::unresolved::AtomicReference = todo!("fabric2seer: the initialiser of `stringAtomicReference` (TODO above)");
        for text in tooltip.iter() {
            let mut string: String = chat_formatting::strip_formatting(&text.get_string());
            let mut thing_to_look_for: String = (if compat::read(__this, |__t| __t.is_bin_auction) { "price:" } else { "new bid:" }).to_string();
            let mut lower_case: String = string.to_lowercase();
            if lower_case.contains(&thing_to_look_for) {
                let mut split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                price_string = Some(compat::trim(&(split[(1) as usize])));
            } else if lower_case.contains("minimum bid:") && !(compat::read(__this, |__t| __t.is_bin_auction)) {
                let mut split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                min_bid = Some(compat::trim(&(((split[(1) as usize]).replace("coins", "")).replace(",", ""))));
            } else if lower_case.contains("you pay:") {
                let mut split: Vec<String> = compat::regex::split(&string, ":");
                if (split.len() as i32) < 2 {
                    continue;
                }
                if compat::read(__this, |__t| __t.buy_state.clone()).unwrap() != BuyState::CantAfford && !(compat::read(__this, |__t| __t.is_bin_auction)) {
                    { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.fancyAuctionHouse.youPay", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::trim(&(split[(1) as usize])))])),); __this.lock().borrow_mut().info_text_widget.as_mut().unwrap().set_message_text(__q0) };
                }
            } else if lower_case.contains("top bid:") {
                { let __v = false; compat::write(__this, |__t| __t.won_auction = __v); }
            } else if lower_case.contains("correct profile") {
                { let __v = true; compat::write(__this, |__t| __t.change_profile = __v); }
                { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty()),); __this.lock().borrow_mut().price_widget.as_mut().unwrap().set_message_text(__q0) };
            } else if lower_case.contains("own auction") {
                AuctionViewScreen::change_state(__this, BuyState::OwnAuction);
            }
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.network.chat.FormattedText.StyledContentConsumer<T>`)
            // rust: if let Some(s) = text.visit_styled(|style, as_string| {
            // rust: let mut res: String = (if style.get_color().unwrap() == TextColor::from_legacy_format(ChatFormatting::Gold).unwrap() && !(compat::regex::Pattern::matches(".*[]\\[+].*", &as_string)) && !(as_string.contains("Collect")) { Some(as_string.clone()) } else { None }).unwrap();
            // rust: return Some(res.clone());
            // rust: }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY) {
            // rust: // TODO(fabric2seer): `.set(..)` on a AtomicReference
            // rust: //   java: stringAtomicReference.set(..);
            // rust: }
        }
        if price_string.is_none() {
            // TODO(fabric2seer): `.get(..)` on a AtomicReference
            //   java: priceString = stringAtomicReference.get();
        }
        if let Some(min_bid) = &min_bid {
            { let __v = Some(min_bid.to_string()); compat::write(__this, |__t| __t.min_bid = __v); }
        } else {
            { let __v = price_string.clone(); compat::write(__this, |__t| __t.min_bid = __v); }
        }
        { let __v = component::literal(price_string.as_deref().unwrap()).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.apply_format(ChatFormatting::Bold).with_color_format(Some(ChatFormatting::Gold))); compat::write(__this, |__t| __t.price_text = __v); }
        let __r0 = compat::read(__this, |__t| __t.price_text.clone());
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__r0),); __this.lock().borrow_mut().price_widget.as_mut().unwrap().set_message_text(__q0) };
        { let __v = true; compat::write(__this, |__t| __t.price_parsed = __v); }
        AuctionViewScreen::update_layout(__this);
    }

    /// java: public PopupScreen getConfirmPurchasePopup(Component title)
    pub fn get_confirm_purchase_popup(__this: &compat::Shared<Self>, title: &MutableComponent) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        // TODO(fabric2seer): `onClose` on a void
        //   java: return new PopupScreen.Builder(this, title)
        //   java: 				.addButton(Component.translatable("text.skyblocker.confirm"), _ -> this.minecraft.gameMode.handleContainerInput(this.minecraft.player.containerMenu.containerId, 11, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player))
        //   java: 				.addButton(Component.translatable("gui.cancel"), PopupScreen::onClose)
        //   java: 				.addMessage((isBinAuction ? Component.translatable("skyblocker.fancyAuctionHouse.price") : Component.translatable("skyblocker.fancyAuctionHouse.newBid")).append(" ").append(priceText))
        //   java: 				.onClose(() -> {
        //   java: 					// This really shouldn't be possible to be null in its ACTUAL use case.
        //   java: 					//noinspection DataFlowIssue
        //   java: 					this.minecraft.gameMode.handleContainerInput(this.minecraft.player.containerMenu.containerId, 15, ContainerUtils.getContainerClickButton(InputConstants.MOUSE_BUTTON_LEFT), ContainerInput.PICKUP, minecraft.player);
        //   java: 				})
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractCustomHypixelGUI<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> for AuctionViewScreen {
    fn abstract_custom_hypixel_gui_base(&self) -> &AbstractCustomHypixelGUIBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> {
        &self.base
    }
    fn abstract_custom_hypixel_gui_base_mut(&mut self) -> &mut AbstractCustomHypixelGUIBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("deprecation") @Override public void onSlotChange(AuctionHouseScreenHandler handler, int slotId, ItemStack stack)
    fn on_slot_change(&mut self, handler: &compat::Shared<AuctionHouseScreenHandler>, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        // TODO(fabric2seer): `AbstractContainerMenu.getRowCount` has no Rust method in the port (looked for `get_row_count` in AbstractContainerMenu and JavaSrc)
        let __o1 = handler.clone();
        let __o2 = handler.clone();
        if stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) || slot_id >= (compat::write(&__o2, |__t| __t.get_row_count())).wrapping_mul(9) {
            return;
        }
        if slot_id == 13 {
            if self.buy_state.clone().unwrap() == BuyState::GracePeriod {
                let mut line: Option<String> = ItemUtils::get_lore_line_if(stack, Arc::new(move |s| compat::trim(&s).starts_with("Can buy in: ")));
                if let Some(line) = &line {
                    self.info_text_widget.as_mut().unwrap().set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable_args("skyblocker.fancyAuctionHouse.canBuyIn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&compat::trim(&((compat::regex::split(&line, ":"))[(1) as usize])))).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW))])));
                } else {
                    let mut price: f64 = ((0) as f64) as f64;
                    // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        price = (compat::trim(&(self.price_text.get_string().replace("coins", "")).replace(",", ""))).trim().parse::<f64>().expect("NumberFormatException");
                    }));
                    if let Err(_) = __result {
                    }
                    if price <= Utils::get_purse() {
                        AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::Afford);
                    } else {
                        AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::CantAfford);
                    }
                }
            }
            return;
        }
        if stack.is_item(mcrust::net::minecraft::world::item::items::dyed_terracotta().red()) {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::CancellableAuction);
            self.buy_slot_id = slot_id;
        }
        if self.price_parsed {
            return;
        }
        if stack.is_item(&items::items().get("poisonous_potato")) {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::CantAfford);
            AuctionViewScreen::get_price_from_tooltip(&self.self_ref.upgrade().unwrap(), &ItemUtils::get_lore(stack));
            self.buy_slot_id = slot_id;
        } else if stack.is_item(&items::items().get("gold_nugget")) {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::Afford);
            AuctionViewScreen::get_price_from_tooltip(&self.self_ref.upgrade().unwrap(), &ItemUtils::get_lore(stack));
            self.buy_slot_id = slot_id;
        } else if stack.is_item(&items::items().get("gold_block")) {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::TopBid);
            AuctionViewScreen::get_price_from_tooltip(&self.self_ref.upgrade().unwrap(), &ItemUtils::get_lore(stack));
            self.buy_slot_id = slot_id;
        } else if stack.is_item(&items::items().get("name_tag")) {
            AuctionViewScreen::get_price_from_tooltip(&self.self_ref.upgrade().unwrap(), &ItemUtils::get_lore(stack));
            self.change_profile = true;
            self.buy_slot_id = slot_id;
        } else if stack.is_in_tag(&mcrust::net::minecraft::tags::item_tags::beds()) {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::GracePeriod);
            AuctionViewScreen::get_price_from_tooltip(&self.self_ref.upgrade().unwrap(), &ItemUtils::get_lore(stack));
            self.buy_slot_id = slot_id;
        }
        let mut lower_case: String = compat::component_of(stack.get_hover_name()).get_string().to_lowercase();
        if self.price_parsed && lower_case.contains("collect auction") {
            AuctionViewScreen::change_state(&self.self_ref.upgrade().unwrap(), BuyState::CollectAuction);
        }
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
        self.lend(screen, |__h| AuctionViewScreen::init(__h))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        compat::with_super(super_extract_background, || self.lend_ref(screen, |__h| AuctionViewScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| AuctionViewScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut AbstractContainerScreen, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(screen, |__h| AuctionViewScreen::mouse_clicked(__h, event, double_click))
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
