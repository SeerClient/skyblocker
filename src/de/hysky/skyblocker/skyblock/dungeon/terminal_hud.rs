//! Ported from de.hysky.skyblocker.skyblock.dungeon.TerminalHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::goldor_waypoints_manager::GoldorWaypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::goldor_waypoints_manager::GoldorWaypointsManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::goldor_waypoints_manager::WaypointTargetKind;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::player::abstract_client_player::AbstractClientPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Supplier<DungeonsConfig.TerminalHud> CONFIG = () -> SkyblockerConfigManager.get().dungeons.terminalHud;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<crate::de::hysky::skyblocker::config::configs::dungeons_config::TerminalHud>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.terminal_hud.clone()));
/// java: public static TerminalHud INSTANCE;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<TerminalHud>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class TerminalHud extends ElementBasedWidget
#[derive(Clone)]
pub struct TerminalHud {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for TerminalHud {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TerminalHud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TerminalHud {
    /// java: public TerminalHud()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in((if FunUtils::should_enable_fun() { component::literal("P3 Guide") } else { component::literal("Goldor Tasks") }), mcrust::net::minecraft::util::common_colors::RED, Information::new3("terminal_hud".to_string(), component::literal("Terminal HUD"), Location::Dungeon), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        let __this_1 = compat::RenderThread::new(__this.clone());
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { TerminalHud::update_from_scheduler(&(*__this_1.get())); }), 50) });
        __this
    }

    /// java: private boolean playerIsNearTerminal(AbstractClientPlayer player, GoldorWaypointsManager.GoldorWaypoint waypoint)
    pub fn player_is_near_terminal(__this: &compat::Shared<Self>, player: &dyn AbstractClientPlayer, waypoint: &compat::Shared<GoldorWaypoint>) -> bool {
        return (player.base().distance_to_sqr_vec(&compat::read(waypoint, |__t| __t.center_pos.clone()).unwrap()) < (25 as f64));
    }

    /// java: private Component getStatusText(GoldorWaypointsManager.GoldorWaypoint waypoint)
    pub fn get_status_text(__this: &compat::Shared<Self>, waypoint: &compat::Shared<GoldorWaypoint>) -> MutableComponent {
        if { let __f = compat::dispatch(waypoint, |__t| __t.is_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) } {
            if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_player_at_terminal) && mcrust::mcrust::modding::game::level().is_some() {
                // TODO(fabric2seer): `with_players` takes 1 argument(s) in the port (`f: impl FnOnce(&[Shared<StoredAvatar>]) -> R`), Java passes 0
                // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
                // rust: let mut player: Option<Box<dyn AbstractClientPlayer>> = mcrust::mcrust::modding::game::level().unwrap().with_players().into_iter().filter(|pl| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; TerminalHud::player_is_near_terminal((&__this_1), pl, waypoint) }).next();
                let mut player: Option<Box<dyn AbstractClientPlayer>> = todo!("fabric2seer: the initialiser of `player` (TODO above)");
                if player.is_some() {
                    // TODO(fabric2seer): `get_plain_text_name` takes 1 argument(s) in the port (`player: &P`), Java passes 0
                    // rust: let mut player_name: String = player::get_plain_text_name();
                    let mut player_name: String = todo!("fabric2seer: the initialiser of `playerName` (TODO above)");
                    return (component::literal(&compat::substring2(&player_name, 0, 12i32.min(compat::str_len(&player_name))))).with_style_format(ChatFormatting::Yellow);
                }
            }
            return component::translatable("skyblocker.dungeons.terminalHud.incompleteStatus").with_style_format(ChatFormatting::Red);
        } else {
            return component::translatable("skyblocker.dungeons.terminalHud.completeStatus").with_style_format(ChatFormatting::Green);
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void updateFromScheduler()
    pub fn update_from_scheduler(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<WidgetsConfigurationScreen>>() && !(GoldorWaypointsManager::is_active()) {
            ({ let __f = compat::dispatch(__this, |__t| __t.update()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
        if !(GoldorWaypointsManager::is_active()) || !({ let __f = compat::dispatch(__this, |__t| __t.should_render()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) }) {
            return;
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
    }

}

impl ElementBasedWidget for TerminalHud {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        TerminalHud::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        TerminalHud::update_config_content_dyn
    }

}

impl HudWidget for TerminalHud {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        TerminalHud::should_render_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn ElementBasedWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for TerminalHud {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        // java: c.setParent(this);
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.hud_widget_base().as_parent()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl TerminalHud {
    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        let mut waypoints: Vec<compat::Shared<GoldorWaypoint>> = GoldorWaypointsManager::get_phase_waypoints();
        if waypoints.is_empty() {
            return;
        }
        for waypoint in waypoints.iter() {
            if !({ let __f = compat::dispatch(waypoint, |__t| __t.is_enabled()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) }) && !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminal_status)) {
                continue;
            }
            match compat::read(waypoint, |__t| __t.kind) {
                WaypointTargetKind::Terminal => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminals)) {
                        continue;
                    }
                }
                WaypointTargetKind::Device => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_device)) {
                        continue;
                    }
                }
                WaypointTargetKind::Lever => {
                    if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_gate)) {
                        continue;
                    }
                }
                _ => {}
            }
            let mut display_text: MutableComponent;
            if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminal_status) {
                display_text = { let (__q0,) = (TerminalHud::get_status_text(__this, waypoint),); waypoint.lock().borrow().name.as_ref().unwrap().copy().append_str(" ").append(__q0) };
            } else {
                display_text = compat::read(waypoint, |__t| __t.name.clone()).unwrap();
            }
            { let (__a0,) = ((PlainTextElement::new(Some(display_text.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_gate) && (GoldorWaypointsManager::get_current_phase() as i32) < 3 {
            if GoldorWaypointsManager::is_gate_destroyed() && !(compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminal_status)) {
                return;
            }
            let mut display_text: MutableComponent = component::literal("Gate");
            if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminal_status) {
                if GoldorWaypointsManager::is_gate_destroyed() {
                    display_text = display_text.append_str(" ").append(component::translatable("skyblocker.dungeons.terminalHud.destroyedStatus").with_style_format(ChatFormatting::Green));
                } else {
                    display_text = display_text.append_str(" ").append(component::translatable("skyblocker.dungeons.terminalHud.incompleteStatus").with_style_format(ChatFormatting::Red));
                }
            }
            { let (__a0,) = ((PlainTextElement::new(Some(display_text.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// TerminalHud.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        let mut status: MutableComponent = component::empty();
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminal_status) {
            status = component::literal(" ").append(component::translatable("skyblocker.dungeons.terminalHud.incompleteStatus").with_style_format(ChatFormatting::Red));
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_terminals) {
            for i in 0..5i32 {
                { let (__a0,) = ((PlainTextElement::new(Some((component::literal(&format!("Terminal #{}", i.wrapping_add(1)))).append(status.clone()))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
            }
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_device) {
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Device").append(status.clone()))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_levers) {
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Lever").append(status.clone()))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Lever").append(status.clone()))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(&(CONFIG.get_mut())(), |__t| __t.show_gate) {
            { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Gate").append(status.clone()))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// TerminalHud.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return GoldorWaypointsManager::is_active();
    }

    /// TerminalHud.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
