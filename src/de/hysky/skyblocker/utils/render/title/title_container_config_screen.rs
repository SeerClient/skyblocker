//! Ported from de.hysky.skyblocker.utils.render.title.TitleContainerConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Alignment;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Direction;
use crate::de::hysky::skyblocker::config::hud_config_screen::HudConfigScreen;
use crate::de::hysky::skyblocker::config::hud_config_screen::HudConfigScreenBase;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::enum_utils::EnumUtils;
use crate::de::hysky::skyblocker::utils::render::gui::basic_widget::BasicWidget;
use crate::de::hysky::skyblocker::utils::render::gui::empty_widget::EmptyWidget;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;

/// java: public static final float MIN_TITLE_SCALE = 30f;
pub const MIN_TITLE_SCALE: f32 = 30.0f32;
/// java: public static final float MAX_TITLE_SCALE = 140f;
pub const MAX_TITLE_SCALE: f32 = 140.0f32;
/// java: private static final Set<Title> EXAMPLES = ImmutableSet.of( new Title(Component.literal("Test1").withStyle(ChatFormatting.RED)), new Title(Component.literal("Test23").withStyle(ChatFormatting.AQUA)), 
///
/// (an identity set in insertion order, as `TitleContainer`'s titles are: `ImmutableSet.of` keeps its argument order)
pub static EXAMPLES: compat::MainThread<Vec<compat::Shared<Title>>> = compat::MainThread::new(|| Vec::from([Title::new_1(component::literal("Test1").with_style_format(ChatFormatting::Red)), Title::new_1(component::literal("Test23").with_style_format(ChatFormatting::Aqua)), Title::new_1(component::literal("Testing1234").with_style_format(ChatFormatting::DarkGreen))]));

/// java: public class TitleContainerConfigScreen extends HudConfigScreen
pub struct TitleContainerConfigScreen {
    /// java: extends HudConfigScreen
    pub base: HudConfigScreenBase,
    /// java: private UIAndVisualsConfig.Direction direction = UIAndVisualsConfig.Direction.VERTICAL;
    pub direction: Direction,
    /// java: private UIAndVisualsConfig.Alignment alignment = UIAndVisualsConfig.Alignment.MIDDLE;
    pub alignment: Alignment,
    /// java: private float titleContainerScale;
    pub title_container_scale: f32,
    /// java: private float renderScale;
    pub render_scale: f32,
}

impl std::ops::Deref for TitleContainerConfigScreen {
    type Target = HudConfigScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TitleContainerConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TitleContainerConfigScreen {
    /// java: protected TitleContainerConfigScreen()
    pub fn new() -> compat::Shared<Self> {
        let __this = Self::new_1(None);
        __this
    }

    /// java: public TitleContainerConfigScreen(@Nullable Screen parent)
    pub fn new_1(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let (_, __this) = HudConfigScreenBase::new_in(component::null_to_empty(Some("Title Container HUD Config")), parent, (EmptyWidget::new() as compat::Shared<dyn BasicWidget>), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                direction: Direction::Vertical,
                alignment: Alignment::Middle,
                title_container_scale: 0.0,
                render_scale: 0.0,
            });
            ((__h.clone() as compat::Shared<dyn HudConfigScreen>), __h)
        });
        __this
    }

    /// java: private void updateWidgetDimensions()
    pub fn update_widget_dimensions(__this: &compat::Shared<Self>) {
        { let __v = (compat::read(__this, |__t| __t.title_container_scale) * title_container::RENDER_SCALE); compat::write(__this, |__t| __t.render_scale = __v); }
        // java: widgets.getFirst().setDimensions(..) - the widget handle is read out before the call re-enters the screen
        ({ let __r = &compat::read(__this, |__t| __t.widgets[0].clone()); let __f = compat::dispatch(__r, |__t| __t.set_dimensions_2()); __f(__r, TitleContainerConfigScreen::get_selection_width(__this), TitleContainerConfigScreen::get_selection_height(__this)) });
    }

    /// java: private int getSelectionWidth()
    pub fn get_selection_width(__this: &compat::Shared<Self>) -> i32 {
        return TitleContainer::get_width(&compat::font(), compat::read(__this, |__t| __t.direction), compat::read(__this, |__t| __t.render_scale), EXAMPLES.get_mut());
    }

    /// java: private int getSelectionHeight()
    pub fn get_selection_height(__this: &compat::Shared<Self>) -> i32 {
        return TitleContainer::get_height(&compat::font(), compat::read(__this, |__t| __t.direction), compat::read(__this, |__t| __t.render_scale), EXAMPLES.get_mut());
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        match input.key() {
            81 => {
                { let __v = EnumUtils::cycle(&compat::read(__this, |__t| __t.alignment)); compat::write(__this, |__t| __t.alignment = __v); }
            }
            69 => {
                { let __v = EnumUtils::cycle_backwards(&compat::read(__this, |__t| __t.alignment)); compat::write(__this, |__t| __t.alignment = __v); }
            }
            82 => {
                { let __v = EnumUtils::cycle(&compat::read(__this, |__t| __t.direction)); compat::write(__this, |__t| __t.direction = __v); }
                TitleContainerConfigScreen::update_widget_dimensions(__this);
            }
            mcrust::com::mojang::blaze3d::platform::input_constants::KEY_EQUALS => {
                { let __v = MAX_TITLE_SCALE.min((compat::read(__this, |__t| __t.title_container_scale) + (10 as f32))); compat::write(__this, |__t| __t.title_container_scale = __v); }
                TitleContainerConfigScreen::update_widget_dimensions(__this);
            }
            45 => {
                { let __v = MIN_TITLE_SCALE.max((compat::read(__this, |__t| __t.title_container_scale) - (10 as f32))); compat::write(__this, |__t| __t.title_container_scale = __v); }
                TitleContainerConfigScreen::update_widget_dimensions(__this);
            }
            _ => {}
        }
        return (if input.is_escape() && true { ({ let __f = compat::dispatch(__this, |__t| __t.on_close()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudConfigScreen>)) }); true } else { false });
    }

}

impl HudConfigScreen for TitleContainerConfigScreen {
    fn hud_config_screen_base(&self) -> &HudConfigScreenBase {
        &self.base
    }
    fn hud_config_screen_base_mut(&mut self) -> &mut HudConfigScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn HudConfigScreen>) {
        TitleContainerConfigScreen::init_dyn
    }

    /// java: @Override protected void extractRenderState(GuiGraphicsExtractor context, List<BasicWidget> widgets, float delta) (dispatch: the body for this class)
    fn extract_render_state_3(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &GuiGraphicsExtractor, &Vec<compat::Shared<dyn BasicWidget>>, f32) {
        TitleContainerConfigScreen::extract_render_state_3_dyn
    }

    /// java: @Override protected int getWidgetXOffset(BasicWidget widget) (dispatch: the body for this class)
    fn get_widget_x_offset(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &compat::Shared<dyn BasicWidget>) -> i32 {
        TitleContainerConfigScreen::get_widget_x_offset_dyn
    }

    /// java: @Override protected List<IntIntMutablePair> getConfigPos(SkyblockerConfig config) (dispatch: the body for this class)
    fn get_config_pos(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &SkyblockerConfig) -> Vec<(i32, i32)> {
        TitleContainerConfigScreen::get_config_pos_dyn
    }

    /// java: @Override protected void savePos(SkyblockerConfig fullConfig, List<BasicWidget> widgets) (dispatch: the body for this class)
    fn save_pos(&self) -> fn(&compat::Shared<dyn HudConfigScreen>, &mut SkyblockerConfig, &Vec<compat::Shared<dyn BasicWidget>>) {
        TitleContainerConfigScreen::save_pos_dyn
    }

}

impl TitleContainerConfigScreen {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        <dyn HudConfigScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn HudConfigScreen>));
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update(Arc::new(move |full_config| {
            let config = &full_config.ui_and_visuals.title_container;
            { let __v = config.direction; compat::write(__this_1.get(), |__t| __t.direction = __v); }
            { let __v = config.alignment; compat::write(__this_1.get(), |__t| __t.alignment = __v); }
            { let __v = config.title_container_scale; compat::write(__this_1.get(), |__t| __t.title_container_scale = __v); }
            let mut x: i32 = config.x;
            let mut y: i32 = config.y;
            if x >= 0 && y >= 0 {
                // java: widgets.getFirst().setPosition(x, y); - `LayoutElement.setPosition`: `setX(x); setY(y);`
                let __widget = compat::read(__this_1.get(), |__t| __t.widgets[0].clone());
                ({ let __f = compat::dispatch(&__widget, |__t| __t.set_x()); __f(&__widget, x) });
                ({ let __f = compat::dispatch(&__widget, |__t| __t.set_y()); __f(&__widget, y) });
            }
            TitleContainerConfigScreen::update_widget_dimensions(__this_1.get());
        }));
    }

    /// TitleContainerConfigScreen.init as the dispatch of HudConfigScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn HudConfigScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void extractRenderState(GuiGraphicsExtractor context, List<BasicWidget> widgets, float delta)
    pub fn extract_render_state_3_impl(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, widgets: &Vec<compat::Shared<dyn BasicWidget>>, mut delta: f32) {
        <dyn HudConfigScreen>::extract_render_state_3_body(&compat::view(__this, |a| a as compat::Shared<dyn HudConfigScreen>), context, widgets, delta);
        TitleContainer::render_8(context, EXAMPLES.get_mut(), ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }), ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }), delta, compat::read(__this, |__t| __t.render_scale), compat::read(__this, |__t| __t.direction), compat::read(__this, |__t| __t.alignment));
        let __width: i32 = compat::read(__this, |__t| __t.width);
        let __line_height: i32 = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        // java: context.centeredText(font, "Press Q/E to change Alignment: " + alignment, width / 2, font.lineHeight * 2, Color.WHITE.getRGB());
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &format!("Press Q/E to change Alignment: {}", compat::jstring(&compat::read(__this, |__t| __t.alignment))), __width.wrapping_div(2), __line_height.wrapping_mul(2), mcrust::java::awt::color::white().get_rgb());
        // java: context.centeredText(font, "Press R to change Direction: " + direction, width / 2, font.lineHeight * 3 + 5, Color.WHITE.getRGB());
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &format!("Press R to change Direction: {}", compat::jstring(&compat::read(__this, |__t| __t.direction))), __width.wrapping_div(2), (__line_height.wrapping_mul(3)).wrapping_add(5), mcrust::java::awt::color::white().get_rgb());
        // java: context.centeredText(font, "Press +/- to change Scale", width / 2, font.lineHeight * 4 + 10, Color.WHITE.getRGB());
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Press +/- to change Scale", __width.wrapping_div(2), (__line_height.wrapping_mul(4)).wrapping_add(10), mcrust::java::awt::color::white().get_rgb());
        // java: context.centeredText(font, "Right Click To Reset Position", width / 2, font.lineHeight * 5 + 15, Color.GRAY.getRGB());
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Right Click To Reset Position", __width.wrapping_div(2), (__line_height.wrapping_mul(5)).wrapping_add(15), mcrust::java::awt::color::gray().get_rgb());
        let mut selection_width: i32 = TitleContainerConfigScreen::get_selection_width(__this);
        let mut x1: i32 = match compat::read(__this, |__t| __t.alignment) {
            Alignment::Left => ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }),
            Alignment::Middle => ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_sub(selection_width.wrapping_div(2)),
            Alignment::Right => ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_sub(selection_width),
            _ => unreachable!("switch expression: no case matched"),
        };
        let mut y1: i32 = ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) });
        let mut x2: i32 = x1.wrapping_add(selection_width);
        let mut y2: i32 = y1.wrapping_add(TitleContainerConfigScreen::get_selection_height(__this));
        // java: context.horizontalLine(x1, x2, y1, Color.RED.getRGB()); (and the other three sides)
        let __red: i32 = mcrust::java::awt::color::red().get_rgb();
        gui_graphics_extractor::horizontal_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x1, x2, y1, __red);
        gui_graphics_extractor::horizontal_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x1, x2, y2, __red);
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x1, y1, y2, __red);
        gui_graphics_extractor::vertical_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x2, y1, y2, __red);
    }

    /// TitleContainerConfigScreen.extractRenderState as the dispatch of HudConfigScreen.extractRenderState
    pub fn extract_render_state_3_dyn(__this: &compat::Shared<dyn HudConfigScreen>, __p0: &GuiGraphicsExtractor, __p1: &Vec<compat::Shared<dyn BasicWidget>>, __p2: f32) {
        Self::extract_render_state_3_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override protected int getWidgetXOffset(BasicWidget widget)
    pub fn get_widget_x_offset_impl(__this: &compat::Shared<Self>, widget: &compat::Shared<dyn BasicWidget>) -> i32 {
        return match compat::read(__this, |__t| __t.alignment) {
            Alignment::Left => 0,
            Alignment::Middle => (-TitleContainerConfigScreen::get_selection_width(__this)).wrapping_div(2),
            Alignment::Right => (-TitleContainerConfigScreen::get_selection_width(__this)),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// TitleContainerConfigScreen.getWidgetXOffset as the dispatch of HudConfigScreen.getWidgetXOffset
    pub fn get_widget_x_offset_dyn(__this: &compat::Shared<dyn HudConfigScreen>, __p0: &compat::Shared<dyn BasicWidget>) -> i32 {
        Self::get_widget_x_offset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected List<IntIntMutablePair> getConfigPos(SkyblockerConfig config)
    pub fn get_config_pos_impl(__this: &compat::Shared<Self>, config: &SkyblockerConfig) -> Vec<(i32, i32)> {
        return vec![((compat::read(__this, |__t| __t.width)).wrapping_div(2), ((((compat::read(__this, |__t| __t.height) as f64) * 0.6)) as i32))];
    }

    /// TitleContainerConfigScreen.getConfigPos as the dispatch of HudConfigScreen.getConfigPos
    pub fn get_config_pos_dyn(__this: &compat::Shared<dyn HudConfigScreen>, __p0: &SkyblockerConfig) -> Vec<(i32, i32)> {
        Self::get_config_pos_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void savePos(SkyblockerConfig fullConfig, List<BasicWidget> widgets)
    pub fn save_pos_impl(__this: &compat::Shared<Self>, full_config: &mut SkyblockerConfig, widgets: &Vec<compat::Shared<dyn BasicWidget>>) {
        let mut default_pos: Vec<(i32, i32)> = ({ let __f = compat::dispatch(__this, |__t| __t.get_config_pos()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudConfigScreen>), full_config) });
        let config = &mut full_config.ui_and_visuals.title_container;
        config.x = (if ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }) != default_pos[0].0 { ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }) } else { -1 });
        config.y = (if ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) != default_pos[0].1 { ({ let __r = &widgets[0]; let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }) } else { -1 });
        config.direction = compat::read(__this, |__t| __t.direction);
        config.alignment = compat::read(__this, |__t| __t.alignment);
        config.title_container_scale = compat::read(__this, |__t| __t.title_container_scale);
    }

    /// TitleContainerConfigScreen.savePos as the dispatch of HudConfigScreen.savePos
    pub fn save_pos_dyn(__this: &compat::Shared<dyn HudConfigScreen>, __p0: &mut SkyblockerConfig, __p1: &Vec<compat::Shared<dyn BasicWidget>>) {
        Self::save_pos_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// TitleContainerConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for TitleContainerConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// TitleContainerConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<TitleContainerConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public final void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>), &context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| TitleContainerConfigScreen::key_pressed(__h, event))
    }

    /// java: @Override public final boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>), &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false) }))
    }

    /// java: @Override public final boolean mouseReleased(MouseButtonEvent click)
    fn mouse_released(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_released()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>), &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0))) }))
    }

    /// java: @Override public final boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    fn mouse_dragged(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_dragged()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>), &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), dx, dy) }))
    }

    /// java: @Override public final void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn HudConfigScreen>)) }))
    }

}

/// `Enum<UIAndVisualsConfig.Direction>` for generic code (this screen's `EnumUtils.cycle(direction)`).
impl crate::de::hysky::skyblocker::utils::enum_utils::JavaEnum for Direction {
    fn java_name(&self) -> &'static str {
        self.name()
    }
    fn java_ordinal(&self) -> i32 {
        self.ordinal()
    }
    fn java_values() -> Vec<Self> {
        Self::values()
    }
}

/// `Enum<UIAndVisualsConfig.Alignment>` for generic code (this screen's `EnumUtils.cycle(alignment)` /
/// `EnumUtils.cycleBackwards(alignment)`).
impl crate::de::hysky::skyblocker::utils::enum_utils::JavaEnum for Alignment {
    fn java_name(&self) -> &'static str {
        self.name()
    }
    fn java_ordinal(&self) -> i32 {
        self.ordinal()
    }
    fn java_values() -> Vec<Self> {
        Self::values()
    }
}
