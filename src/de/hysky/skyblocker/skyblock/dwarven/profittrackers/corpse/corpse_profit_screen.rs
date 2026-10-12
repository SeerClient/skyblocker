//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.CorpseProfitScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_list::CorpseList;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_loot::CorpseLoot;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker::CorpseProfitTracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::RewardList;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int ENTRY_HEIGHT = 11;
pub const ENTRY_HEIGHT: i32 = 11;

/// java: public class CorpseProfitScreen extends Screen
pub struct CorpseProfitScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private final List<CorpseLoot> rewardsList = CorpseProfitTracker.getCurrentProfileRewards();
    pub rewards_list: Vec<compat::Shared<CorpseLoot>>,
    /// java: private @Nullable CorpseList corpseList = null;
    pub corpse_list: Option<compat::WidgetRef>,
    /// java: private @Nullable RewardList rewardList = null;
    pub reward_list: Option<compat::WidgetRef>,
    /// java: private final DoubleBooleanPair totalProfit = calculateTotalProfit(rewardsList);
    pub total_profit: Option<(f64, bool)>,
    /// java: private boolean summaryView;
    pub summary_view: bool,
    /// java: private @Nullable HeaderAndFooterLayout layout;
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private @Nullable ContainerObjectSelectionList<?> selectionList;
    pub selection_list: Option<AbstractSelectionList>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for CorpseProfitScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CorpseProfitScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CorpseProfitScreen {
    /// java: public CorpseProfitScreen(Screen parent)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this(parent, true);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public CorpseProfitScreen(Screen parent, boolean summaryView)
    pub fn new_2(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut summary_view: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __init_summary_view = summary_view;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.corpseTracker.screenTitle").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent: todo!("fabric2seer: the field value (TODO above)"),
            rewards_list: CorpseProfitTracker::get_current_profile_rewards(),
            corpse_list: None,
            reward_list: None,
            total_profit: None,
            summary_view: __init_summary_view,
            layout: None,
            selection_list: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(CorpseProfitScreen::calculate_total_profit(&compat::read(&__this, |__t| __t.rewards_list.clone()))); compat::write(&__this, |__t| __t.total_profit = __v); }
        __this
    }

    /// java: private static DoubleBooleanPair calculateTotalProfit(List<CorpseLoot> list)
    pub fn calculate_total_profit(list: &Vec<compat::Shared<CorpseLoot>>) -> (f64, bool) {
        let mut total: f64 = ((0) as f64) as f64;
        let mut is_price_complete: bool = true;
        for loot in list.iter() {
            total = (total + CorpseLoot::profit(loot));
            if !(CorpseLoot::is_price_data_complete(loot)) {
                is_price_complete = false;
            }
        }
        return (total, is_price_complete);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.layout.clone()).is_none() || compat::read(__this, |__t| __t.selection_list.clone()).is_none() {
            return;
        }
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { let (__q0, __q1,) = (__this.lock().borrow().layout.as_ref().unwrap().get_width(), __this.lock().borrow().layout.as_ref().unwrap().get_content_height(),); __this.lock().borrow_mut().selection_list.as_mut().unwrap().set_size(__q0, __q1) };
        { let (__q0, __q1,) = (__this.lock().borrow().layout.as_ref().unwrap().get_x(), __this.lock().borrow().layout.as_ref().unwrap().get_header_height(),); __this.lock().borrow_mut().selection_list.as_mut().unwrap().set_position(__q0, __q1) };
        __this.lock().borrow_mut().selection_list.as_mut().unwrap().refresh_scroll_amount();
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        { let __v = Some(HeaderAndFooterLayout::with_heights(0, 0, 25, 45)); compat::write(__this, |__t| __t.layout = __v); }
        // TODO(fabric2seer): `HeaderAndFooterLayout.addTitleHeader` has no Rust method in the port (looked for `add_title_header` in HeaderAndFooterLayout and its supertypes)
        // rust: let __h0 = &component::translatable("skyblocker.corpseTracker.screenTitle").with_style_format(ChatFormatting::Bold);
        // rust: let __h1 = &compat::font();
        // rust: __this.lock().borrow().layout.as_ref().unwrap().add_title_header(__h0, __h1);
        { let __v = Some((if compat::read(__this, |__t| __t.summary_view) { CorpseProfitScreen::get_reward_list(__this) } else { CorpseProfitScreen::get_corpse_list(__this) })); compat::write(__this, |__t| __t.selection_list = __v); }
        let __r0 = compat::read(__this, |__t| __t.selection_list.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.default_cell_setting().padding_horizontal(5).padding_vertical(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        // TODO(fabric2seer): `.format(..)` on a value of unknown type
        //   java: Component totalProfitText = Component.translatable("skyblocker.corpseTracker.totalProfit",
        //   java: 				NumberFormat.getInstance().format(totalProfit.leftDouble()).formatted(totalProfit.leftDouble() >= 0 ? ChatFormatting.GREEN : ChatFormatting.RED), // Formatting.GOLD is filled in from parent if it's 0
        //   java: 				totalProfit.rightBoolean() ? Component.empty() : Component.literal("skyblocker.corpseTracker.incompletePriceData").withStyle(ChatFormatting.RED)
        //   java: 		).withStyle(ChatFormatting.GOLD, ChatFormatting.BOLD);
        let mut total_profit_text: MutableComponent = todo!("fabric2seer: the initialiser of `totalProfitText` (TODO above)");
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: adder.addChild(new StringWidget(Button.DEFAULT_WIDTH * 2 + 10, ENTRY_HEIGHT, totalProfitText, minecraft.font).setMaxWidth(Button.DEFAULT_WIDTH * 2 + 10, StringWidget.TextOverflow.SCROLLING), 2, gridWidget.newCellSettings().alignHorizontallyCenter());
        let mut button_text: MutableComponent = (if compat::read(__this, |__t| __t.summary_view) { component::translatable("skyblocker.corpseTracker.historyView") } else { component::translatable("skyblocker.corpseTracker.summaryView") });
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_1 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(button_text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { CorpseProfitScreen::change_view(__this_1.get(), &compat::write(__this_1.get(), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| __w.clone()) })); } }) })))).build()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_2 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { CorpseProfitScreen::on_close(__this_2.get()); } }) })))).build()).unwrap();
        { let (__q0,) = (LayoutElement::Grid(grid_widget.clone()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        CorpseProfitScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: private void changeView(Button button)
    pub fn change_view(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        { let __v = !(compat::read(__this, |__t| __t.summary_view)); compat::write(__this, |__t| __t.summary_view = __v); }
        // TODO(fabric2seer): `rebuild_widgets` takes 1 argument(s) in the port (`build: &mut dyn FnMut(&mut Screen)`), Java passes 0
        // rust: __this.lock().borrow_mut().rebuild_widgets();
    }

    /// java: private CorpseList getCorpseList()
    pub fn get_corpse_list(__this: &compat::Shared<Self>) -> compat::Shared<CorpseList> {
        // TODO(fabric2seer): not translated (expected ':' at '=' (offset 3935))
        //   java: return corpseList == null ? corpseList = new CorpseList(Minecraft.getInstance(), width, height - 96, ENTRY_HEIGHT, rewardsList) : corpseList;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private RewardList getRewardList()
    pub fn get_reward_list(__this: &compat::Shared<Self>) -> compat::Shared<RewardList> {
        // TODO(fabric2seer): not translated (expected ':' at '=' (offset 4135))
        //   java: return rewardList == null ? rewardList = new RewardList(Minecraft.getInstance(), width, height - 96, 0, ENTRY_HEIGHT, rewardsList) : rewardList;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// CorpseProfitScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for CorpseProfitScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// CorpseProfitScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<CorpseProfitScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| CorpseProfitScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| CorpseProfitScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| CorpseProfitScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
