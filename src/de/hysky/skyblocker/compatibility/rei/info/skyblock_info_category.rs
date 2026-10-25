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
use crate::de::hysky::skyblocker::compatibility::rei::{rei_category_identifier_of, rei_entry_ingredient_first, rei_entry_stack_item, rei_entry_stacks_of, rei_rectangle, rei_slot_widget, rei_widgets_create_recipe_base, rei_widgets_create_slot, rei_widgets_wrap_vanilla_widget};
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int REI_SLOT_HEIGHT = 18;
pub const REI_SLOT_HEIGHT: i32 = 18;
/// java: private static final int OFFSET = 10;
pub const OFFSET: i32 = 10;
/// java: private static final int RED_ERROR_COLOR = 0xFFFF5555;
pub const RED_ERROR_COLOR: i32 = 0xFFFF5555u32 as i32;
/// java: private static final EntryStack<ItemStack> ICON = EntryStacks.of(new ItemStack(Items.CHEST));
pub static ICON: compat::MainThread<crate::unresolved::EntryStack> = compat::MainThread::new(|| rei_entry_stacks_of(&mcrust::net::minecraft::world::item::item_stack::ItemStack::of(mcrust::net::minecraft::world::item::items::items().get("chest"), 1)));
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
        // java: return CategoryIdentifier.of(IDENTIFIER);
        return rei_category_identifier_of(IDENTIFIER.get());
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
    pub fn get_wiki_lookup_button(&self, text: &MutableComponent, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: Option<&LocalPlayer>) -> AbstractWidget {
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
        // java: EntryStack<?> entryStack = display.getInputEntries().getFirst().getFirst();
        let entry_stack: crate::unresolved::EntryStack = rei_entry_ingredient_first(&display.get_input_entries()[0]);
        // java: if (!(entryStack.getValue() instanceof ItemStack itemStack)) return widgets;
        let Some(item_stack) = rei_entry_stack_item(&entry_stack) else {
            return widgets;
        };
        let bounds_geometry = rei_rectangle(bounds);
        // java: widgets.add(Widgets.createRecipeBase(bounds));
        widgets.push(rei_widgets_create_recipe_base(bounds));
        // java: Slot slot = Widgets.createSlot(new Point(bounds.getCenterX() - 9 + 1, bounds.y + 1 + OFFSET / 2)).entry(entryStack);
        let slot: crate::unresolved::Widget = rei_slot_widget(rei_widgets_create_slot(bounds_geometry.get_center_x() - 9 + 1, bounds_geometry.y + 1 + OFFSET / 2), None, None, entry_stack);
        // java: widgets.add(slot);
        widgets.push(slot);
        // java: LocalPlayer player = Minecraft.getInstance().player; - read where the buttons use it
        let mut layout_widget: LinearLayout = LinearLayout::vertical();
        // java: layoutWidget.setPosition(bounds.x + OFFSET, bounds.y + OFFSET + REI_SLOT_HEIGHT);
        layout_widget.set_x(bounds_geometry.x + OFFSET);
        layout_widget.set_y(bounds_geometry.y + OFFSET + REI_SLOT_HEIGHT);
        // java: layoutWidget.addChild(Button.builder(Component.translatable("key.skyblocker.itemPriceLookup"), button -> {..}).build());
        let price_item_stack = item_stack.duplicate();
        let mut price_button: AbstractWidget = (button::builder(component::translatable("key.skyblocker.itemPriceLookup").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            // java: ItemPrice.itemPriceLookup(player, itemStack);
            mcrust::mcrust::modding::game::with_player(|player| crate::de::hysky::skyblocker::skyblock::item::item_price::ItemPrice::item_price_lookup_local_player_item_stack(player, &price_item_stack));
            // java: Scheduler.INSTANCE.schedule(() -> {..}, 10); - `button` is this widget of the screen that is up then
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
                // java: if (checkScreen()) return;
                if SkyblockInfoCategory::new().check_screen() {
                    return;
                }
                // java: button.setMessage(Component.translatable("skyblocker.rei.skyblockInfo.failedToFind").withColor(RED_ERROR_COLOR)); button.active = false;
                // NEEDS-GAME-API: a handle to a widget of a screen that is not a mod screen (REI's own screen), to write
                // its message and `active` from a scheduled task
            }), 10) });
        })))).build();
        // (the port's layouts hold a widget by an index: 0 is this button, 1 the wiki one)
        layout_widget.add_child(LayoutElement::widget(0, price_button.get_width(), price_button.get_height()));
        // java: layoutWidget.addChild(getWikiLookupButton(Component.translatable("key.skyblocker.wikiLookup.independent"), itemStack, player));
        let mut wiki_button: AbstractWidget = mcrust::mcrust::modding::game::with_player(|player| self.get_wiki_lookup_button(&component::translatable("key.skyblocker.wikiLookup.independent"), &item_stack, Some(&*player))).unwrap_or_else(|| self.get_wiki_lookup_button(&component::translatable("key.skyblocker.wikiLookup.independent"), &item_stack, None));
        layout_widget.add_child(LayoutElement::widget(1, wiki_button.get_width(), wiki_button.get_height()));
        // java: layoutWidget.arrangeElements(); - after the wrap in Java, which keeps the same objects: the positions
        // are written back to the two buttons before they are wrapped here
        layout_widget.arrange_elements();
        layout_widget.visit_children(&mut |child| {
            if let LayoutElement::Widget { index, x, y, .. } = child {
                if *index == 0 { price_button.set_position(*x, *y); } else { wiki_button.set_position(*x, *y); }
            }
        });
        // java: layoutWidget.visitWidgets(child -> widgets.add(Widgets.wrapVanillaWidget(child)));
        widgets.push(rei_widgets_wrap_vanilla_widget(price_button));
        widgets.push(rei_widgets_wrap_vanilla_widget(wiki_button));
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
