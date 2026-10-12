//! Ported from de.hysky.skyblocker.config.HudConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::gui::basic_widget::BasicWidget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class HudConfigScreen extends Screen
pub struct HudConfigScreenBase {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: protected final List<BasicWidget> widgets;
    pub widgets: Vec<compat::Shared<dyn BasicWidget>>,
    /// java: private @Nullable BasicWidget draggingWidget;
    pub dragging_widget: Option<compat::Shared<dyn BasicWidget>>,
    /// java: private double mouseClickRelativeX;
    pub mouse_click_relative_x: f64,
    /// java: private double mouseClickRelativeY;
    pub mouse_click_relative_y: f64,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for HudConfigScreenBase {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HudConfigScreenBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HudConfigScreenBase {
    /// java: public HudConfigScreen(Component title, @Nullable Screen parent, BasicWidget widget) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut widget: compat::Shared<dyn BasicWidget>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HudConfigScreen>, __R)) -> (compat::Shared<dyn HudConfigScreen>, __R) {
        let (__this, __r) = Self::new2_in(title.clone(), parent, vec![widget.clone()], __wrap);
        (__this, __r)
    }

    /// java: public HudConfigScreen(Component title, @Nullable Screen parent, List<BasicWidget> widgets) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut title: MutableComponent, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut widgets: Vec<compat::Shared<dyn BasicWidget>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HudConfigScreen>, __R)) -> (compat::Shared<dyn HudConfigScreen>, __R) {
        let __init_parent = parent;
        let __init_widgets = widgets.clone();
        let this = Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            __widgets: Default::default(),
            parent: __init_parent,
            widgets: __init_widgets,
            dragging_widget: None,
            mouse_click_relative_x: 0.0,
            mouse_click_relative_y: 0.0,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of HudConfigScreen (implemented by HudConfigScreenBase and every subclass)
pub trait HudConfigScreen: compat::DynAny {
    fn hud_config_screen_base(&self) -> &HudConfigScreenBase;
    fn hud_config_screen_base_mut(&mut self) -> &mut HudConfigScreenBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn HudConfigScreen>) {
        <dyn HudConfigScreen>::init_body
    }

    /// java: @Override public final void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn HudConfigScreen>::extract_render_state_body
    }

    /// java: protected void extractRenderState(GuiGraphicsExtractor graphics, List<BasicWidget> widgets, float delta) (dispatch: the body for this class)
    fn extract_render_state_3(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &GuiGraphicsExtractor, &Vec<compat::Shared<dyn BasicWidget>>, f32) {
        <dyn HudConfigScreen>::extract_render_state_3_body
    }

    /// java: @Override public final boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY) (dispatch: the body for this class)
    fn mouse_dragged(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &MouseButtonEvent, f64, f64) -> bool {
        <dyn HudConfigScreen>::mouse_dragged_body
    }

    /// java: @Override public final boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &MouseButtonEvent, bool) -> bool {
        <dyn HudConfigScreen>::mouse_clicked_body
    }

    /// java: @Override public final boolean mouseReleased(MouseButtonEvent click) (dispatch: the body for this class)
    fn mouse_released(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &MouseButtonEvent) -> bool {
        <dyn HudConfigScreen>::mouse_released_body
    }

    /// java: protected int getWidgetXOffset(BasicWidget widget) (dispatch: the body for this class)
    fn get_widget_x_offset(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &compat::Shared<dyn BasicWidget>) -> i32 {
        <dyn HudConfigScreen>::get_widget_x_offset_body
    }

    /// java: protected void resetPos() (dispatch: the body for this class)
    fn reset_pos(&self) -> fn(&compat::Shared<dyn HudConfigScreen>) {
        <dyn HudConfigScreen>::reset_pos_body
    }

    /// java: protected abstract List<IntIntMutablePair> getConfigPos(SkyblockerConfig config); (dispatch: the body for this class)
    fn get_config_pos(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &SkyblockerConfig) -> Vec<(i32, i32)>;

    /// java: @Override public final void onClose() (dispatch: the body for this class)
    fn on_close(&self) -> fn(&compat::Shared<dyn HudConfigScreen>) {
        <dyn HudConfigScreen>::on_close_body
    }

    /// java: protected abstract void savePos(SkyblockerConfig configManager, List<BasicWidget> widgets); (dispatch: the body for this class)
    ///
    /// `fullConfig` is the live config, mutated in place (`&mut`), as Java's does.
    fn save_pos(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &mut SkyblockerConfig, &Vec<compat::Shared<dyn BasicWidget>>);

}

impl dyn HudConfigScreen {
    /// java: @Override protected void init()
    pub fn init_body(__this: &compat::Shared<dyn HudConfigScreen>) {
        ();
        ({ let __f = compat::dispatch(__this, |__t| __t.reset_pos()); __f(__this) });
    }

    /// java: @Override public final void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn HudConfigScreen>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        ();
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_render_state_3()); __f(__this, graphics, &compat::read(__this, |__t| __t.hud_config_screen_base().widgets.clone()), delta) });
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.centeredText(font, "Right Click To Reset Position", width / 2, height / 2, Color.GRAY.getRGB());
    }

    /// java: protected void extractRenderState(GuiGraphicsExtractor graphics, List<BasicWidget> widgets, float delta)
    pub fn extract_render_state_3_body(__this: &compat::Shared<dyn HudConfigScreen>, graphics: &GuiGraphicsExtractor, widgets: &Vec<compat::Shared<dyn BasicWidget>>, mut delta: f32) {
        for widget in widgets.iter() {
            // TODO(fabric2seer): `extractRenderState` is not a method of BasicWidget
            //   java: widget.extractRenderState(graphics, -1, -1, delta);
        }
    }

    /// java: @Override public final boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn mouse_dragged_body(__this: &compat::Shared<dyn HudConfigScreen>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) -> bool {
        if click.button() == 0 && compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, (((click.x() - compat::read(__this, |__t| __t.hud_config_screen_base().mouse_click_relative_x)).clamp((0 as f64), ((compat::read(__this, |__t| __t.hud_config_screen_base().width)).wrapping_sub({ let __r = &compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }) as f64))) as i32).wrapping_sub({ let __f = compat::dispatch(__this, |__t| __t.get_widget_x_offset()); __f(__this, &compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).unwrap()) })) });
            ({ let __r = &compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_y()); __f(__r, (((click.y() - compat::read(__this, |__t| __t.hud_config_screen_base().mouse_click_relative_y)).clamp((0 as f64), ((compat::read(__this, |__t| __t.hud_config_screen_base().height)).wrapping_sub({ let __r = &compat::read(__this, |__t| __t.hud_config_screen_base().dragging_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }) as f64))) as i32)) });
        }
        return false;
    }

    /// java: @Override public final boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn HudConfigScreen>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if click.button() == 0 {
            for widget in compat::read(__this, |__t| __t.hud_config_screen_base().widgets.clone()) {
                if GuiHelper::point_is_in_area(click.x(), click.y(), ((({ let __f = compat::dispatch(&widget, |__t| __t.get_x()); __f(&widget) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_widget_x_offset()); __f(__this, &widget) })) as f64) as f64, ((({ let __f = compat::dispatch(&widget, |__t| __t.get_y()); __f(&widget) })) as f64) as f64, (((({ let __f = compat::dispatch(&widget, |__t| __t.get_x()); __f(&widget) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_widget_x_offset()); __f(__this, &widget) })).wrapping_add({ let __f = compat::dispatch(&widget, |__t| __t.get_width()); __f(&widget) })) as f64) as f64, ((({ let __f = compat::dispatch(&widget, |__t| __t.get_y()); __f(&widget) }).wrapping_add({ let __f = compat::dispatch(&widget, |__t| __t.get_height()); __f(&widget) })) as f64) as f64) {
                    { let __v = Some(widget.clone()); compat::write(__this, |__t| __t.hud_config_screen_base_mut().dragging_widget = __v); }
                    { let __v = ((click.x() - (({ let __f = compat::dispatch(&widget, |__t| __t.get_x()); __f(&widget) }) as f64)) - (({ let __f = compat::dispatch(__this, |__t| __t.get_widget_x_offset()); __f(__this, &widget) }) as f64)); compat::write(__this, |__t| __t.hud_config_screen_base_mut().mouse_click_relative_x = __v); }
                    { let __v = (click.y() - (({ let __f = compat::dispatch(&widget, |__t| __t.get_y()); __f(&widget) }) as f64)); compat::write(__this, |__t| __t.hud_config_screen_base_mut().mouse_click_relative_y = __v); }
                    break;
                }
            }
        } else if click.button() == 1 {
            ({ let __f = compat::dispatch(__this, |__t| __t.reset_pos()); __f(__this) });
        }
        return false;
    }

    /// java: @Override public final boolean mouseReleased(MouseButtonEvent click)
    pub fn mouse_released_body(__this: &compat::Shared<dyn HudConfigScreen>, click: &MouseButtonEvent) -> bool {
        { let __v = None; compat::write(__this, |__t| __t.hud_config_screen_base_mut().dragging_widget = __v); }
        return false;
    }

    /// java: protected int getWidgetXOffset(BasicWidget widget)
    pub fn get_widget_x_offset_body(__this: &compat::Shared<dyn HudConfigScreen>, widget: &compat::Shared<dyn BasicWidget>) -> i32 {
        return 0;
    }

    /// java: protected void resetPos()
    pub fn reset_pos_body(__this: &compat::Shared<dyn HudConfigScreen>) {
        let mut config_positions: Vec<(i32, i32)> = ({ let __f = compat::dispatch(__this, |__t| __t.get_config_pos()); __f(__this, &SkyblockerConfigManager::get()) });
        if (config_positions.len() as i32) != (__this.lock().borrow().hud_config_screen_base().widgets.len() as i32) {
            panic!("IllegalStateException: {}", format!("The number of positions ({}) does not match the number of widgets ({})", (config_positions.len() as i32), (__this.lock().borrow().hud_config_screen_base().widgets.len() as i32)).to_string());
        }
        for i in 0..(__this.lock().borrow().hud_config_screen_base().widgets.len() as i32) {
            let mut widget: compat::Shared<dyn BasicWidget> = __this.lock().borrow().hud_config_screen_base().widgets[(i) as usize].clone();
            let config_pos = &config_positions[(i) as usize];
            ({ let __f = compat::dispatch(&widget, |__t| __t.set_x()); __f(&widget, config_pos.0) });
            ({ let __f = compat::dispatch(&widget, |__t| __t.set_y()); __f(&widget, config_pos.1) });
        }
    }

    /// java: @Override public final void onClose()
    pub fn on_close_body(__this: &compat::Shared<dyn HudConfigScreen>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update(Arc::new(move |config| { ({ let __f = compat::dispatch(__this_1.get(), |__t| __t.save_pos()); __f(__this_1.get(), config, &compat::read(__this_1.get(), |__t| __t.hud_config_screen_base().widgets.clone())) }); }));
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}
