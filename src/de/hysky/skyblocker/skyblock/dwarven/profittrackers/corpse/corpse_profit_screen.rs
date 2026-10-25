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

/// `NumberFormat.getInstance()`: the default locale's NUMBER pattern `#,##0.###`. The port models no default
/// locale (`java::text` has ROOT and US, whose CLDR symbols are the same); US is used.
fn number_format_instance() -> mcrust::java::text::decimal_format::DecimalFormat {
    mcrust::java::text::decimal_format::DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us))
}

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
    /// The same widget as `corpse_list` or `reward_list`, by its index in the screen's widget list.
    pub selection_list: Option<compat::WidgetRef>,
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
    /// `parent` is `Option<Screen>` as the mod's other screens type it (`GreenhousePresetsScreen`); the screen is
    /// opened with `set_screen_keeping_parent`, and `onClose` returns to that parent through the game's stack.
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __this = Self::new_2(parent, true);
        __this
    }

    /// java: public CorpseProfitScreen(Screen parent, boolean summaryView)
    pub fn new_2(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut summary_view: bool) -> compat::Shared<Self> {
        let __init_summary_view = summary_view;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.corpseTracker.screenTitle").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent,
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

    /// One access to the list `selectionList` holds (the same widget as `corpseList` or `rewardList`, which the
    /// screen took into its widget list: the reference is its index there).
    fn with_selection_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> R) -> R {
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.selection_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, f) })
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.layout.is_none()) || compat::read(__this, |__t| __t.selection_list.is_none()) {
            return;
        }
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let (__w, __h, __x, __y) = compat::read(__this, |__t| { let __l = __t.layout.as_ref().unwrap(); (__l.get_width(), __l.get_content_height(), __l.get_x(), __l.get_header_height()) });
        // `selectionList.setSize(..)`, `setPosition(..)`, `refreshScrollAmount()`: the list's box is the widget's (the
        // game syncs the two before the list's next event); it is set on both so the scroll is clamped to the new box.
        CorpseProfitScreen::with_selection_list(__this, |__w0| {
            __w0.set_size(__w, __h);
            __w0.set_position(__x, __y);
            __w0.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| {
                __l.list.set_size(__w, __h);
                __l.list.set_position(__x, __y);
                // `selectionList.refreshScrollAmount()` is virtual: both lists override it (to reposition their rows).
                if let Some(__h) = __l.class::<compat::ListClass<CorpseList>>().map(|__c| __c.object().clone()) {
                    compat::lend_list(&__h, __l, || CorpseList::refresh_scroll_amount(&__h));
                } else if let Some(__h) = __l.class::<compat::ListClass<RewardList>>().map(|__c| __c.object().clone()) {
                    compat::lend_list(&__h, __l, || RewardList::refresh_scroll_amount(&__h));
                } else {
                    __l.list.refresh_scroll_amount();
                }
            })).expect("a mod list reached from inside its own method")
        });
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        { let __v = Some(HeaderAndFooterLayout::with_heights(0, 0, 25, 45)); compat::write(__this, |__t| __t.layout = __v); }
        // java: layout.addTitleHeader(title, font) - `HeaderAndFooterLayout.addTitleHeader:125-127` is
        // `headerFrame.addChild(new StringWidget(component, font))`; the port's layout holds its children as the screen's
        // widget indices, so the widget joins the screen first (as the game's own screens make this call)
        { let __w = compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.corpseTracker.screenTitle").with_style_format(ChatFormatting::Bold)); let (__ww, __wh) = (__w.get_width(), __w.get_height()); let __q0 = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        // `selectionList = summaryView ? getRewardList() : getCorpseList();` - the two lists are kept by the screen and
        // `selectionList` is the same widget as the chosen one.
        let summary_view = compat::read(__this, |__t| __t.summary_view);
        if summary_view {
            CorpseProfitScreen::get_reward_list(__this);
        } else {
            CorpseProfitScreen::get_corpse_list(__this);
        }
        // `layout.addToContents(selectionList)`: the widget joins the screen's list.
        let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = if summary_view { __s.reward_list.as_mut() } else { __s.corpse_list.as_mut() }.unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) });
        compat::write(__this, |__t| __t.selection_list = Some(compat::WidgetRef::Screen(__i)));
        __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(LayoutElement::widget(__i, __ww, __wh));
        let mut grid_widget: GridLayout = GridLayout::new_default();
        { let __s = grid_widget.default_cell_setting(); *__s = (*__s).padding_horizontal(5).padding_vertical(2); }
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        let total_profit = compat::read(__this, |__t| __t.total_profit).unwrap();
        // `NumberFormat.getInstance().format(x).formatted(GREEN or RED)`: `String.formatted` is `String.format(this,
        // args)`, and the number has no format specifier, so the colour argument is dropped - Java prints the number
        // in the parent's GOLD whatever its sign (the mod's own comment says so).
        let total_profit_string = compat::string_format(&number_format_instance().format(total_profit.0), &[compat::FormatArg::Str((if total_profit.0 >= 0.0 { ChatFormatting::Green } else { ChatFormatting::Red }).to_string())]);
        let mut total_profit_text: MutableComponent = component::translatable_args("skyblocker.corpseTracker.totalProfit", vec![
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(total_profit_string),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(if total_profit.1 { component::empty() } else { component::literal("skyblocker.corpseTracker.incompletePriceData").with_style_format(ChatFormatting::Red) }),
        ]).with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold]);
        {
            let mut __w = compat::string_widget(0, 0, button::DEFAULT_WIDTH * 2 + 10, ENTRY_HEIGHT, total_profit_text);
            __w.set_max_width(button::DEFAULT_WIDTH * 2 + 10, mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow::Scrolling);
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __q0 = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh);
            let __settings = grid_widget.new_cell_settings().align_horizontally_center();
            adder.add_child_spanning(&mut grid_widget, __q0, 2, __settings).unwrap();
        }
        let mut button_text: MutableComponent = (if summary_view { component::translatable("skyblocker.corpseTracker.historyView") } else { component::translatable("skyblocker.corpseTracker.summaryView") });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __w = (button::builder(button_text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { let __button = compat::write(&__this_1, |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.__widgets.with(&mut __s.base, __r, |__w| __w.clone()) }); CorpseProfitScreen::change_view(&__this_1, &__button); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); let __q0 = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh); adder.add_child(&mut grid_widget, __q0).unwrap(); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { CorpseProfitScreen::on_close(&__this_2); }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); let __q0 = LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh); adder.add_child(&mut grid_widget, __q0).unwrap(); }
        { let (__q0,) = (LayoutElement::Grid(grid_widget.clone()),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        CorpseProfitScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: private void changeView(Button button)
    pub fn change_view(__this: &compat::Shared<Self>, button: &AbstractWidget) {
        { let __v = !(compat::read(__this, |__t| __t.summary_view)); compat::write(__this, |__t| __t.summary_view = __v); }
        // `rebuildWidgets()` clears the screen's widget list, but `corpseList` / `rewardList` stay the same objects
        // in Java (lazy, kept across rebuilds): the list widgets the screen holds are taken back into the references
        // first, so the next `init` adds the same lists (their rows and scroll) again.
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            for __r in [&mut __s.corpse_list, &mut __s.reward_list] {
                if let Some(compat::WidgetRef::Screen(__i)) = __r.as_ref() {
                    let __widget = __s.__widgets.with(&mut __s.base, &mut compat::WidgetRef::Screen(*__i), |__w| __w.clone());
                    *__r = Some(compat::WidgetRef::value(__widget));
                }
            }
            __s.selection_list = None;
        });
        // `Screen.rebuildWidgets()` with this screen's `init` as its build step: the live screen (lent to this call)
        // is taken out of the object for the port's method, and handed back to the object for `init`.
        let mut __screen = compat::write(__this, |__t| std::mem::replace(&mut __t.base, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
        __screen.rebuild_widgets(&mut |__s: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen| {
            compat::write(__this, |__t| std::mem::swap(&mut __t.base, __s));
            CorpseProfitScreen::init(__this);
            compat::write(__this, |__t| std::mem::swap(&mut __t.base, __s));
        });
        compat::write(__this, |__t| __t.base = __screen);
    }

    /// java: private CorpseList getCorpseList()
    pub fn get_corpse_list(__this: &compat::Shared<Self>) -> compat::Shared<CorpseList> {
        // `corpseList == null ? corpseList = new CorpseList(..) : corpseList`
        if compat::read(__this, |__t| __t.corpse_list.is_none()) {
            let (__w, __h, __rewards) = compat::read(__this, |__t| (__t.width, __t.height, __t.rewards_list.clone()));
            let __v = Some(compat::WidgetRef::value(compat::attach_list(CorpseList::new(compat::minecraft(), __w, __h.wrapping_sub(96), ENTRY_HEIGHT, __rewards))));
            compat::write(__this, |__t| __t.corpse_list = __v);
        }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.corpse_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<CorpseList>(__w)) }).expect("a mod list reached from inside its own method")
    }

    /// java: private RewardList getRewardList()
    pub fn get_reward_list(__this: &compat::Shared<Self>) -> compat::Shared<RewardList> {
        // `rewardList == null ? rewardList = new RewardList(..) : rewardList`
        if compat::read(__this, |__t| __t.reward_list.is_none()) {
            let (__w, __h, __rewards) = compat::read(__this, |__t| (__t.width, __t.height, __t.rewards_list.clone()));
            let __v = Some(compat::WidgetRef::value(compat::attach_list(RewardList::new(compat::minecraft(), __w, __h.wrapping_sub(96), 0, ENTRY_HEIGHT, __rewards))));
            compat::write(__this, |__t| __t.reward_list = __v);
        }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.reward_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<RewardList>(__w)) }).expect("a mod list reached from inside its own method")
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
