//! Ported from de.hysky.skyblocker.compatibility.rei.info.SkyblockInfoCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::rei::info::skyblock_info_display::SkyblockInfoDisplay;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_custom_hypixel_gui::AbstractCustomHypixelGUI;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int REI_SLOT_HEIGHT = 18;
pub const REI_SLOT_HEIGHT: i32 = 18;
/// java: private static final int OFFSET = 10;
pub const OFFSET: i32 = 10;
/// java: private static final int RED_ERROR_COLOR = 0xFFFF5555;
pub const RED_ERROR_COLOR: i32 = 0xFFFF5555u32 as i32;
// TODO(fabric2seer): static call EntryStacks.of(..)
//   java: private static final EntryStack<ItemStack> ICON = EntryStacks.of(new ItemStack(Items.CHEST));
/// java: private static final EntryStack<ItemStack> ICON = EntryStacks.of(new ItemStack(Items.CHEST));
pub static ICON: compat::MainThread<crate::unresolved::EntryStack> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ICON` - see the TODO above"));
/// java: public static final Identifier IDENTIFIER = SkyblockerMod.id("skyblock_info");
pub static IDENTIFIER: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("skyblock_info"));

/// java: public class SkyblockInfoCategory implements DisplayCategory<SkyblockInfoDisplay>
#[derive(Clone)]
pub struct SkyblockInfoCategory {}

impl SkyblockInfoCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public CategoryIdentifier<? extends SkyblockInfoDisplay> getCategoryIdentifier()
    pub fn get_category_identifier(&self) -> crate::unresolved::CategoryIdentifier {
        // TODO(fabric2seer): static call CategoryIdentifier.of(..)
        //   java: return CategoryIdentifier.of(IDENTIFIER);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Component getTitle()
    pub fn get_title(&self) -> MutableComponent {
        return component::translatable("emi.category.skyblocker.skyblock_info");
    }

    /// java: @Override public Renderer getIcon()
    ///
    /// `ICON` itself (an `EntryStack` is a `Renderer` in REI).
    pub fn get_icon(&self) -> &'static crate::unresolved::EntryStack {
        return ICON.get();
    }

    /// java: private Button getWikiLookupButton(Component text, ItemStack itemStack, LocalPlayer player)
    pub fn get_wiki_lookup_button(&self, text: &MutableComponent, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &LocalPlayer) -> AbstractWidget {
        // the lambda captures the stack (the same object: `duplicate`) and `player` - `CLIENT.player`, reached when pressed
        let item_stack_ref = item_stack.duplicate();
        let mut btn: AbstractWidget = (button::builder(text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { mcrust::mcrust::modding::game::with_player(|player| { WikiLookupManager::open_wiki(&item_stack_ref, player); }); })))).build();
        if ItemRepository::get_wiki_link(&item_stack.get_neu_name()).is_none() {
            btn.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(btn.get_message_component()).copy().with_color(RED_ERROR_COLOR)));
            btn.active = false;
        }
        return btn;
    }

    /// java: private boolean checkScreen()
    pub fn check_screen(&self) -> bool {
        // `currentScreen instanceof ContainerScreen || instanceof AbstractCustomHypixelGUI`: the screen's kind - a mod's
        // container screen rides on the vanilla `ContainerScreen` it replaced
        return matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container);
    }

    /// java: @Override public List<Widget> setupDisplay(SkyblockInfoDisplay display, Rectangle bounds)
    pub fn setup_display(&self, display: &SkyblockInfoDisplay, bounds: &crate::unresolved::Rectangle) -> Vec<crate::unresolved::Widget> {
        let mut widgets: Vec<crate::unresolved::Widget> = Vec::new();
        // TODO(fabric2seer): `.getFirst(..)` on a EntryIngredient
        //   java: EntryStack<?> entryStack = display.getInputEntries().getFirst().getFirst();
        let mut entry_stack: crate::unresolved::EntryStack = todo!("fabric2seer: the initialiser of `entryStack` (TODO above)");
        // TODO(fabric2seer): `.getValue(..)` on a EntryStack
        //   java: if (!(entryStack.getValue() instanceof ItemStack itemStack)) return widgets;
        // TODO(fabric2seer): static call Widgets.createRecipeBase(..)
        //   java: widgets.add(Widgets.createRecipeBase(bounds));
        // TODO(fabric2seer): static call Widgets.createSlot(..)
        //   java: Slot slot = Widgets.createSlot(new Point(bounds.getCenterX() - 9 + 1, bounds.y + 1 + OFFSET / 2)).entry(entryStack);
        let mut slot: crate::unresolved::Slot = todo!("fabric2seer: the initialiser of `slot` (TODO above)");
        // rust: port gap - `widgets.add(slot)`: REI's `Slot extends Widget`, and the REI library is the `crate::unresolved`
        // placeholders here, where `Slot` is no `Widget` (GAPS_batchG)
        let _ = &slot;
        let mut layout_widget: LinearLayout = LinearLayout::vertical();
        // TODO(fabric2seer): field `x` of a Rectangle
        //   java: layoutWidget.setPosition(bounds.x + OFFSET, bounds.y + OFFSET + REI_SLOT_HEIGHT);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout_widget.add_child((button::builder(component::translatable("key.skyblocker.itemPriceLookup").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: // TODO(fabric2seer): unknown name `itemStack`
        // rust: //   java: ItemPrice.itemPriceLookup(player, itemStack);
        // rust: ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
        // rust: if self.check_screen() {
        // rust: return;
        // rust: }
        // rust: (__screen.widget_mut(__index)).set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.rei.skyblockInfo.failedToFind").with_color(RED_ERROR_COLOR)));
        // rust: (__screen.widget_mut(__index)).active = false;
        // rust: }), 10) });
        // rust: })))).build());
        // TODO(fabric2seer): unknown name `itemStack`
        //   java: layoutWidget.addChild(getWikiLookupButton(Component.translatable("key.skyblocker.wikiLookup.independent"), itemStack, player));
        // TODO(fabric2seer): static call Widgets.wrapVanillaWidget(..)
        //   java: layoutWidget.visitWidgets(child -> widgets.add(Widgets.wrapVanillaWidget(child)));
        layout_widget.arrange_elements();
        return widgets;
    }

    /// java: @Override public int getDisplayHeight()
    pub fn get_display_height(&self) -> i32 {
        return ((3i32.wrapping_mul(mcrust::net::minecraft::client::gui::components::button::DEFAULT_HEIGHT)).wrapping_add(REI_SLOT_HEIGHT)).wrapping_add(2i32.wrapping_mul(OFFSET));
    }

    /// java: @Override public int getDisplayWidth(SkyblockInfoDisplay display)
    pub fn get_display_width(&self, display: &SkyblockInfoDisplay) -> i32 {
        return 150i32.wrapping_add(20);
    }

}
