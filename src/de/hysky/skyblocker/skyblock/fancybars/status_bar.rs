//! Ported from de.hysky.skyblocker.skyblock.fancybars.StatusBar by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::IntelligenceDisplay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarAnchor;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar_type::StatusBarType;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::EstimatedResource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::Resource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::StatusBarTracker;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier BAR_FILL = SkyblockerMod.id("bars/bar_fill");
pub static BAR_FILL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_fill"));
/// java: private static final Identifier BAR_BACK = SkyblockerMod.id("bars/bar_back");
pub static BAR_BACK: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_back"));
/// java: public static final int ICON_SIZE = 9;
pub const ICON_SIZE: i32 = 9;
/// java: private static final Identifier CLOCK_ICON = SkyblockerMod.id("bars/icons/rift_time");
pub static EXPERIENCE_STATUS_BAR_CLOCK_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/icons/rift_time"));
/// java: private static final Color WITHER_COLOR = new Color(76, 48, 57);
pub static HEALTH_STATUS_BAR_WITHER_COLOR: compat::MainThread<Color> = compat::MainThread::new(|| Color::new(76, 48, 57));
/// java: private static final Color POISON_COLOR = new Color(94, 78, 18);
pub static HEALTH_STATUS_BAR_POISON_COLOR: compat::MainThread<Color> = compat::MainThread::new(|| Color::new(94, 78, 18));
/// java: private static final Identifier WITHER_ICON = SkyblockerMod.id("bars/icons/health_wither");
pub static HEALTH_STATUS_BAR_WITHER_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/icons/health_wither"));
/// java: private static final Identifier POISON_ICON = SkyblockerMod.id("bars/icons/health_poison");
pub static HEALTH_STATUS_BAR_POISON_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/icons/health_poison"));

/// java: public class StatusBar implements LayoutElement, Renderable, GuiEventListener, NarratableEntry
#[derive(Clone)]
pub struct StatusBarBase {
    /// java: private final Identifier icon;
    pub icon: Option<Identifier>,
    /// java: private final StatusBarType type;
    pub r#type: Option<StatusBarType>,
    /// java: private Color[] colors;
    pub colors: Vec<Color>,
    /// java: private @Nullable Color textColor;
    pub text_color: Option<Color>,
    /// java: private @Nullable OnClick onClick = null;
    pub on_click: Option<OnClick>,
    /// java: public int gridX = 0;
    pub grid_x: i32,
    /// java: public int gridY = 0;
    pub grid_y: i32,
    /// java: public float x = 0;
    pub x: f32,
    /// java: public float y = 0;
    pub y: f32,
    /// java: public float width = 0;
    pub width: f32,
    /// java: public BarPositioner.@Nullable BarAnchor anchor = null;
    pub anchor: Option<BarAnchor>,
    /// java: public int size = 1;
    pub size: i32,
    /// java: public float fill = 0;
    pub fill: f32,
    /// java: public float overflowFill = 0;
    pub overflow_fill: f32,
    /// java: public boolean inMouse = false;
    pub in_mouse: bool,
    /// java: public boolean visible = true;
    pub visible: bool,
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: private @Nullable Integer value = null;
    pub value: Option<i32>,
    /// java: private @Nullable Integer max = null;
    pub max: Option<i32>,
    /// java: private @Nullable Integer overflow = null;
    pub overflow: Option<i32>,
    /// java: private final Function<Integer, String> toDisplay;
    pub to_display: Option<Arc<dyn Fn(i32) -> String>>,
    /// java: private int renderX = 0;
    pub render_x: i32,
    /// java: private int renderY = 0;
    pub render_y: i32,
    /// java: private int renderWidth = 0;
    pub render_width: i32,
    /// java: private IconPosition iconPosition = IconPosition.LEFT;
    pub icon_position: IconPosition,
    /// java: private TextPosition textPosition = TextPosition.BAR_CENTER;
    pub text_position: TextPosition,
    /// java: public boolean showMax = false;
    pub show_max: bool,
    /// java: public boolean showOverflow = false;
    pub show_overflow: bool,
    /// java: private Direction direction = Direction.LEFT_TO_RIGHT;
    pub direction: Direction,
}

impl StatusBarBase {
    /// java: public StatusBar(StatusBarType type) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut r#type: StatusBarType, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn StatusBar>, __R)) -> (compat::Shared<dyn StatusBar>, __R) {
        let (__this, __r) = Self::new_2_in(r#type, Arc::new(move |__a0| __a0.to_string()), __wrap);
        (__this, __r)
    }

    /// java: public StatusBar(StatusBarType type)
    pub fn new(mut r#type: StatusBarType) -> compat::Shared<Self> {
        Self::new_in(r#type, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        }).1
    }

    /// java: public StatusBar(StatusBarType type, Function<Integer, String> toDisplay) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_2_in<__R>(mut r#type: StatusBarType, to_display: Arc<dyn Fn(i32) -> String>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn StatusBar>, __R)) -> (compat::Shared<dyn StatusBar>, __R) {
        let __init_icon = Some(crate::entry::skyblocker_mod::id(&format!("bars/icons/{}", (r#type).get_serialized_name())));
        let __init_colors = (r#type).get_colors();
        let __init_text_color = (r#type).get_text_color();
        let __init_type = Some(r#type);
        let __init_to_display = Some(to_display.clone());
        let this = Self {
            icon: __init_icon,
            r#type: __init_type,
            colors: __init_colors,
            text_color: __init_text_color,
            on_click: None,
            grid_x: 0,
            grid_y: 0,
            x: ((0) as f32) as f32,
            y: ((0) as f32) as f32,
            width: ((0) as f32) as f32,
            anchor: None,
            size: 1,
            fill: ((0) as f32) as f32,
            overflow_fill: ((0) as f32) as f32,
            in_mouse: false,
            visible: true,
            enabled: true,
            value: None,
            max: None,
            overflow: None,
            to_display: __init_to_display,
            render_x: 0,
            render_y: 0,
            render_width: 0,
            icon_position: IconPosition::Left,
            text_position: TextPosition::BarCenter,
            show_max: false,
            show_overflow: false,
            direction: Direction::LeftToRight,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: public StatusBar(StatusBarType type, Function<Integer, String> toDisplay)
    pub fn new_2(mut r#type: StatusBarType, to_display: Arc<dyn Fn(i32) -> String>) -> compat::Shared<Self> {
        Self::new_2_in(r#type, to_display, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        }).1
    }

}

impl std::fmt::Display for StatusBarBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// java: the virtual methods of StatusBar (implemented by StatusBarBase and every subclass)
pub trait StatusBar: compat::DynAny {
    fn status_bar_base(&self) -> &StatusBarBase;
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public Color[] getColors() (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Vec<Color> {
        <dyn StatusBar>::get_colors_body
    }

    /// java: public boolean hasOverflow() (dispatch: the body for this class)
    fn has_overflow(&self) -> fn(&compat::Shared<dyn StatusBar>) -> bool {
        <dyn StatusBar>::has_overflow_body
    }

    /// java: public boolean hasMax() (dispatch: the body for this class)
    fn has_max(&self) -> fn(&compat::Shared<dyn StatusBar>) -> bool {
        <dyn StatusBar>::has_max_body
    }

    /// java: public @Nullable Color getTextColor() (dispatch: the body for this class)
    fn get_text_color(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Option<Color> {
        <dyn StatusBar>::get_text_color_body
    }

    /// java: public void setTextColor(@Nullable Color textColor) (dispatch: the body for this class)
    fn set_text_color(&self) -> fn(&compat::Shared<dyn StatusBar>, Option<Color>) {
        <dyn StatusBar>::set_text_color_body
    }

    /// java: public Component getName() (dispatch: the body for this class)
    fn get_name(&self) -> fn(&compat::Shared<dyn StatusBar>) -> MutableComponent {
        <dyn StatusBar>::get_name_body
    }

    /// java: protected int transparency(int color) (dispatch: the body for this class)
    fn transparency(&self) -> fn(&compat::Shared<dyn StatusBar>, i32) -> i32 {
        <dyn StatusBar>::transparency_body
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn StatusBar>::extract_render_state_body
    }

    /// java: protected Identifier getIcon() (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Identifier {
        <dyn StatusBar>::get_icon_body
    }

    /// java: @SuppressWarnings("incomplete-switch") public void extractBar(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract_bar(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor) {
        <dyn StatusBar>::extract_bar_body
    }

    /// java: protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth) (dispatch: the body for this class)
    fn extract_bar_fill(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32) {
        <dyn StatusBar>::extract_bar_fill_body
    }

    /// java: protected void renderBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth, float fill, int argb) (dispatch: the body for this class)
    fn render_bar_fill(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32, f32, i32) {
        <dyn StatusBar>::render_bar_fill_body
    }

    /// java: public void updateValues(float fill, float overflowFill, int value, @Nullable Integer max, @Nullable Integer overflow) (dispatch: the body for this class)
    fn update_values(&self) -> fn(&compat::Shared<dyn StatusBar>, f32, f32, i32, i32, i32) {
        <dyn StatusBar>::update_values_body
    }

    /// java: public void updateWithResource(StatusBarTracker.Resource resource) (dispatch: the body for this class)
    fn update_with_resource(&self) -> fn(&compat::Shared<dyn StatusBar>, &Resource) {
        <dyn StatusBar>::update_with_resource_body
    }

    /// java: public void extractText(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract_text(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor) {
        <dyn StatusBar>::extract_text_body
    }

    /// java: public void extractCursor(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_cursor(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn StatusBar>::extract_cursor_body
    }

    /// java: @Override public void setX(int x) (dispatch: the body for this class)
    fn set_x(&self) -> fn(&compat::Shared<dyn StatusBar>, i32) {
        <dyn StatusBar>::set_x_body
    }

    /// java: @Override public void setY(int y) (dispatch: the body for this class)
    fn set_y(&self) -> fn(&compat::Shared<dyn StatusBar>, i32) {
        <dyn StatusBar>::set_y_body
    }

    /// java: @Override public int getX() (dispatch: the body for this class)
    fn get_x(&self) -> fn(&compat::Shared<dyn StatusBar>) -> i32 {
        <dyn StatusBar>::get_x_body
    }

    /// java: @Override public int getY() (dispatch: the body for this class)
    fn get_y(&self) -> fn(&compat::Shared<dyn StatusBar>) -> i32 {
        <dyn StatusBar>::get_y_body
    }

    /// java: @Override public int getWidth() (dispatch: the body for this class)
    fn get_width(&self) -> fn(&compat::Shared<dyn StatusBar>) -> i32 {
        <dyn StatusBar>::get_width_body
    }

    /// java: public void setWidth(int width) (dispatch: the body for this class)
    fn set_width(&self) -> fn(&compat::Shared<dyn StatusBar>, i32) {
        <dyn StatusBar>::set_width_body
    }

    /// java: @Override public int getHeight() (dispatch: the body for this class)
    fn get_height(&self) -> fn(&compat::Shared<dyn StatusBar>) -> i32 {
        <dyn StatusBar>::get_height_body
    }

    /// java: @Override public ScreenRectangle getRectangle() (dispatch: the body for this class)
    fn get_rectangle(&self) -> fn(&compat::Shared<dyn StatusBar>) -> ScreenRectangle {
        <dyn StatusBar>::get_rectangle_body
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn StatusBar>, f64, f64) -> bool {
        <dyn StatusBar>::is_mouse_over_body
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer) (dispatch: the body for this class)
    fn visit_widgets(&self) -> fn(&compat::Shared<dyn StatusBar>, Arc<dyn Fn(AbstractWidget)>) {
        <dyn StatusBar>::visit_widgets_body
    }

    /// java: @Override public void setFocused(boolean focused) (dispatch: the body for this class)
    fn set_focused(&self) -> fn(&compat::Shared<dyn StatusBar>, bool) {
        <dyn StatusBar>::set_focused_body
    }

    /// java: @Override public boolean isFocused() (dispatch: the body for this class)
    fn is_focused(&self) -> fn(&compat::Shared<dyn StatusBar>) -> bool {
        <dyn StatusBar>::is_focused_body
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn StatusBar>) -> crate::unresolved::NarrationPriority {
        <dyn StatusBar>::narration_priority_body
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn StatusBar>, &MouseButtonEvent, bool) -> bool {
        <dyn StatusBar>::mouse_clicked_body
    }

    /// java: public void setOnClick(@Nullable OnClick onClick) (dispatch: the body for this class)
    fn set_on_click(&self) -> fn(&compat::Shared<dyn StatusBar>, OnClick) {
        <dyn StatusBar>::set_on_click_body
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn StatusBar>, &dyn NarrationElementOutput) {
        <dyn StatusBar>::update_narration_body
    }

    /// java: @Override public String toString() (dispatch: the body for this class)
    fn to_string(&self) -> fn(&compat::Shared<dyn StatusBar>) -> String {
        <dyn StatusBar>::to_string_body
    }

    /// java: public IconPosition getIconPosition() (dispatch: the body for this class)
    fn get_icon_position(&self) -> fn(&compat::Shared<dyn StatusBar>) -> IconPosition {
        <dyn StatusBar>::get_icon_position_body
    }

    /// java: public void setIconPosition(IconPosition iconPosition) (dispatch: the body for this class)
    fn set_icon_position(&self) -> fn(&compat::Shared<dyn StatusBar>, IconPosition) {
        <dyn StatusBar>::set_icon_position_body
    }

    /// java: public boolean showText() (dispatch: the body for this class)
    fn show_text(&self) -> fn(&compat::Shared<dyn StatusBar>) -> bool {
        <dyn StatusBar>::show_text_body
    }

    /// java: public TextPosition getTextPosition() (dispatch: the body for this class)
    fn get_text_position(&self) -> fn(&compat::Shared<dyn StatusBar>) -> TextPosition {
        <dyn StatusBar>::get_text_position_body
    }

    /// java: public void setTextPosition(TextPosition textPosition) (dispatch: the body for this class)
    fn set_text_position(&self) -> fn(&compat::Shared<dyn StatusBar>, TextPosition) {
        <dyn StatusBar>::set_text_position_body
    }

    /// java: public Direction getDirection() (dispatch: the body for this class)
    fn get_direction(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Direction {
        <dyn StatusBar>::get_direction_body
    }

    /// java: public void setDirection(Direction direction) (dispatch: the body for this class)
    fn set_direction(&self) -> fn(&compat::Shared<dyn StatusBar>, Direction) {
        <dyn StatusBar>::set_direction_body
    }

    /// java: public void loadFromJson(JsonObject object) (dispatch: the body for this class)
    fn load_from_json(&self) -> fn(&compat::Shared<dyn StatusBar>, &serde_json::Map<String, serde_json::Value>) {
        <dyn StatusBar>::load_from_json_body
    }

    /// java: public JsonObject toJson() (dispatch: the body for this class)
    fn to_json(&self) -> fn(&compat::Shared<dyn StatusBar>) -> serde_json::Map<String, serde_json::Value> {
        <dyn StatusBar>::to_json_body
    }

}

impl dyn StatusBar {
    /// java: public Color[] getColors()
    pub fn get_colors_body(__this: &compat::Shared<dyn StatusBar>) -> Vec<Color> {
        return compat::read(__this, |__t| __t.status_bar_base().colors.clone());
    }

    /// java: public boolean hasOverflow()
    pub fn has_overflow_body(__this: &compat::Shared<dyn StatusBar>) -> bool {
        return __this.lock().borrow().status_bar_base().r#type.as_ref().unwrap().has_overflow();
    }

    /// java: public boolean hasMax()
    pub fn has_max_body(__this: &compat::Shared<dyn StatusBar>) -> bool {
        return __this.lock().borrow().status_bar_base().r#type.as_ref().unwrap().has_max();
    }

    /// java: public @Nullable Color getTextColor()
    pub fn get_text_color_body(__this: &compat::Shared<dyn StatusBar>) -> Option<Color> {
        return compat::read(__this, |__t| __t.status_bar_base().text_color.clone());
    }

    /// java: public void setTextColor(@Nullable Color textColor)
    pub fn set_text_color_body(__this: &compat::Shared<dyn StatusBar>, mut text_color: Option<Color>) {
        { let __v = text_color.clone(); compat::write(__this, |__t| __t.status_bar_base_mut().text_color = __v); }
    }

    /// java: public Component getName()
    pub fn get_name_body(__this: &compat::Shared<dyn StatusBar>) -> MutableComponent {
        return __this.lock().borrow().status_bar_base().r#type.as_ref().unwrap().get_name();
    }

    /// java: protected int transparency(int color)
    pub fn transparency_body(__this: &compat::Shared<dyn StatusBar>, mut color: i32) -> i32 {
        if compat::read(__this, |__t| __t.status_bar_base().in_mouse) {
            return ((color & 0x00FFFFFF) | 0x44000000);
        }
        return color;
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_bar()); __f(__this, graphics) });
        if compat::read(__this, |__t| __t.status_bar_base().enabled) {
            ({ let __f = compat::dispatch(__this, |__t| __t.extract_text()); __f(__this, graphics) });
        }
    }

    /// java: protected Identifier getIcon()
    pub fn get_icon_body(__this: &compat::Shared<dyn StatusBar>) -> Identifier {
        return compat::read(__this, |__t| __t.status_bar_base().icon.clone()).unwrap();
    }

    /// java: @SuppressWarnings("incomplete-switch") public void extractBar(GuiGraphicsExtractor graphics)
    pub fn extract_bar_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor) {
        if compat::read(__this, |__t| __t.status_bar_base().render_width) <= 0 {
            return;
        }
        let mut transparency: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.transparency()); __f(__this, -1) });
        match compat::read(__this, |__t| __t.status_bar_base().icon_position) {
            IconPosition::Left => {
                gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &({ let __f = compat::dispatch(__this, |__t| __t.get_icon()); __f(__this) }), compat::read(__this, |__t| __t.status_bar_base().render_x), compat::read(__this, |__t| __t.status_bar_base().render_y), ICON_SIZE, ICON_SIZE, (transparency) as f32);
            }
            IconPosition::Right => {
                gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &({ let __f = compat::dispatch(__this, |__t| __t.get_icon()); __f(__this) }), ((compat::read(__this, |__t| __t.status_bar_base().render_x)).wrapping_add(compat::read(__this, |__t| __t.status_bar_base().render_width))).wrapping_sub(ICON_SIZE), compat::read(__this, |__t| __t.status_bar_base().render_y), ICON_SIZE, ICON_SIZE, (transparency) as f32);
            }
            _ => {}
        }
        let mut bar_width: i32 = (if compat::read(__this, |__t| __t.status_bar_base().icon_position) == IconPosition::Off { compat::read(__this, |__t| __t.status_bar_base().render_width) } else { ((compat::read(__this, |__t| __t.status_bar_base().render_width)).wrapping_sub(ICON_SIZE)).wrapping_sub(1) });
        let mut bar_x: i32 = (if compat::read(__this, |__t| __t.status_bar_base().icon_position) == IconPosition::Left { ((compat::read(__this, |__t| __t.status_bar_base().render_x)).wrapping_add(ICON_SIZE)).wrapping_add(1) } else { compat::read(__this, |__t| __t.status_bar_base().render_x) });
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_BACK.get_mut(), bar_x, (compat::read(__this, |__t| __t.status_bar_base().render_y)).wrapping_add(1), bar_width, 7, (transparency) as f32);
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_bar_fill()); __f(__this, graphics, bar_x, bar_width) });
    }

    /// java: protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth)
    pub fn extract_bar_fill_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor, mut bar_x: i32, mut bar_width: i32) {
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: renderBarFill(graphics, barX, barWidth, fill, transparency(colors[0].getRGB()));
        if { let __f = compat::dispatch(__this, |__t| __t.has_overflow()); __f(__this) } && compat::read(__this, |__t| __t.status_bar_base().overflow_fill) > (0 as f32) {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: renderBarFill(graphics, barX, barWidth, Math.min(overflowFill, 1), transparency(colors[1].getRGB()));
        }
    }

    /// java: protected void renderBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth, float fill, int argb)
    pub fn render_bar_fill_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor, mut bar_x: i32, mut bar_width: i32, mut fill: f32, mut argb: i32) {
        let mut fill_width: i32 = ((((bar_width.wrapping_sub(2) as f32) * fill)) as i32);
        let mut x: i32 = match compat::read(__this, |__t| __t.status_bar_base().direction) {
            Direction::LeftToRight => bar_x.wrapping_add(1),
            Direction::RightToLeft => ((bar_x.wrapping_add(bar_width)).wrapping_sub(fill_width)).wrapping_sub(1),
            Direction::FromMiddle => bar_x.wrapping_add((bar_width.wrapping_sub(fill_width)).wrapping_div(2)),
            _ => unreachable!("switch expression: no case matched"),
        };
        GuiHelper::nine_slice_colored(graphics, BAR_FILL.get_mut(), x, (compat::read(__this, |__t| __t.status_bar_base().render_y)).wrapping_add(2), fill_width, 5, argb);
    }

    /// java: public void updateValues(float fill, float overflowFill, int value, @Nullable Integer max, @Nullable Integer overflow)
    pub fn update_values_body(__this: &compat::Shared<dyn StatusBar>, mut fill: f32, mut overflow_fill: f32, mut value: i32, mut max: i32, mut overflow: i32) {
        { let __v = Some(value); compat::write(__this, |__t| __t.status_bar_base_mut().value = __v); }
        { let __v = fill.clamp((0 as f32), (1 as f32)); compat::write(__this, |__t| __t.status_bar_base_mut().fill = __v); }
        { let __v = overflow_fill.clamp((0 as f32), (1 as f32)); compat::write(__this, |__t| __t.status_bar_base_mut().overflow_fill = __v); }
        { let __v = Some(max); compat::write(__this, |__t| __t.status_bar_base_mut().max = __v); }
        { let __v = Some(overflow); compat::write(__this, |__t| __t.status_bar_base_mut().overflow = __v); }
    }

    /// java: public void updateWithResource(StatusBarTracker.Resource resource)
    pub fn update_with_resource_body(__this: &compat::Shared<dyn StatusBar>, resource: &Resource) {
        ({ let __f = compat::dispatch(__this, |__t| __t.update_values()); __f(__this, ((resource.value() as f32) / ((resource.max()) as f32)), ((resource.overflow() as f32) / ((resource.max()) as f32)), resource.value(), resource.max(), (if resource.overflow() > 0 { Some(resource.overflow()) } else { None }).unwrap()) });
    }

    /// java: public void extractText(GuiGraphicsExtractor graphics)
    pub fn extract_text_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.show_text()); __f(__this) }) {
            return;
        }
        let mut text_renderer: Font = compat::font();
        let mut bar_width: i32 = (if compat::read(__this, |__t| __t.status_bar_base().icon_position) == IconPosition::Off { compat::read(__this, |__t| __t.status_bar_base().render_width) } else { ((compat::read(__this, |__t| __t.status_bar_base().render_width)).wrapping_sub(ICON_SIZE)).wrapping_sub(1) });
        let mut bar_x: i32 = (if compat::read(__this, |__t| __t.status_bar_base().icon_position) == IconPosition::Left { ((compat::read(__this, |__t| __t.status_bar_base().render_x)).wrapping_add(ICON_SIZE)).wrapping_add(2) } else { compat::read(__this, |__t| __t.status_bar_base().render_x) });
        let mut string_value: String = (if compat::read(__this, |__t| __t.status_bar_base().value.clone()).is_none() { "???".to_string() } else { ((compat::read(__this, |__t| __t.status_bar_base().to_display.clone()).unwrap()))((if compat::read(__this, |__t| __t.status_bar_base().overflow.clone()).is_none() || compat::read(__this, |__t| __t.status_bar_base().show_overflow) { compat::read(__this, |__t| __t.status_bar_base().value.clone()) } else { Some((compat::read(__this, |__t| __t.status_bar_base().value.clone()).unwrap()).wrapping_add(compat::read(__this, |__t| __t.status_bar_base().overflow.clone()).unwrap())) })) });
        let mut display_color: Color = (if compat::read(__this, |__t| __t.status_bar_base().text_color.clone()).is_none() { Some(__this.lock().borrow().status_bar_base().colors[(0) as usize].clone()) } else { compat::read(__this, |__t| __t.status_bar_base().text_color.clone()) }).unwrap();
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: MutableComponent text = Component.literal(stringValue).withStyle(style -> style.withColor(displayColor.getRGB()));
        let mut text: MutableComponent = todo!("fabric2seer: the initialiser of `text` (TODO above)");
        if { let __f = compat::dispatch(__this, |__t| __t.has_max()); __f(__this) } && compat::read(__this, |__t| __t.status_bar_base().show_max) && compat::read(__this, |__t| __t.status_bar_base().max.clone()).is_some() {
            (text.append_str("/")).append_str(&compat::opt_str(&compat::read(__this, |__t| __t.status_bar_base().max.clone())));
        }
        if { let __f = compat::dispatch(__this, |__t| __t.has_overflow()); __f(__this) } && compat::read(__this, |__t| __t.status_bar_base().show_overflow) && compat::read(__this, |__t| __t.status_bar_base().overflow.clone()).is_some() {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: MutableComponent literal = Component.literal(" + ").withStyle(style -> style.withColor(colors[1].getRGB()));
            let mut literal: MutableComponent = todo!("fabric2seer: the initialiser of `literal` (TODO above)");
            literal.append_str(&((compat::read(__this, |__t| __t.status_bar_base().to_display.clone()).unwrap()))(compat::read(__this, |__t| __t.status_bar_base().overflow.clone())));
            text.append(literal.clone());
        }
        let mut text_width: i32 = text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        let mut x: i32;
        match compat::read(__this, |__t| __t.status_bar_base().text_position) {
            TextPosition::Right => {
                x = (bar_x.wrapping_add(bar_width)).wrapping_sub(text_width);
            }
            TextPosition::Center => {
                x = (compat::read(__this, |__t| __t.status_bar_base().render_x)).wrapping_add(((compat::read(__this, |__t| __t.status_bar_base().render_width)).wrapping_sub(text_width)).wrapping_div(2));
            }
            TextPosition::BarCenter => {
                x = bar_x.wrapping_add((bar_width.wrapping_sub(text_width)).wrapping_div(2));
            }
            _ => {
                x = bar_x;
            }
        }
        let mut y: i32 = (compat::read(__this, |__t| __t.status_bar_base().render_y)).wrapping_sub(3);
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: int color = transparency(displayColor.getRGB());
        let mut color: i32 = todo!("fabric2seer: the initialiser of `color` (TODO above)");
        let mut outline_color: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.transparency()); __f(__this, mcrust::net::minecraft::util::common_colors::BLACK) });
        GuiHelper::outlined_text(graphics, &component::translation_arg_message(&text), x, y, color, outline_color);
    }

    /// java: public void extractCursor(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_cursor_body(__this: &compat::Shared<dyn StatusBar>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut temp_x: i32 = compat::read(__this, |__t| __t.status_bar_base().render_x);
        let mut temp_y: i32 = compat::read(__this, |__t| __t.status_bar_base().render_y);
        let mut temp_ghost: bool = compat::read(__this, |__t| __t.status_bar_base().in_mouse);
        { let __v = mouse_x; compat::write(__this, |__t| __t.status_bar_base_mut().render_x = __v); }
        { let __v = mouse_y; compat::write(__this, |__t| __t.status_bar_base_mut().render_y = __v); }
        { let __v = false; compat::write(__this, |__t| __t.status_bar_base_mut().in_mouse = __v); }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_render_state()); __f(__this, graphics, mouse_x, mouse_y, a) });
        { let __v = temp_x; compat::write(__this, |__t| __t.status_bar_base_mut().render_x = __v); }
        { let __v = temp_y; compat::write(__this, |__t| __t.status_bar_base_mut().render_y = __v); }
        { let __v = temp_ghost; compat::write(__this, |__t| __t.status_bar_base_mut().in_mouse = __v); }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x_body(__this: &compat::Shared<dyn StatusBar>, mut x: i32) {
        { let __v = x; compat::write(__this, |__t| __t.status_bar_base_mut().render_x = __v); }
    }

    /// java: @Override public void setY(int y)
    pub fn set_y_body(__this: &compat::Shared<dyn StatusBar>, mut y: i32) {
        { let __v = y; compat::write(__this, |__t| __t.status_bar_base_mut().render_y = __v); }
    }

    /// java: @Override public int getX()
    pub fn get_x_body(__this: &compat::Shared<dyn StatusBar>) -> i32 {
        return compat::read(__this, |__t| __t.status_bar_base().render_x);
    }

    /// java: @Override public int getY()
    pub fn get_y_body(__this: &compat::Shared<dyn StatusBar>) -> i32 {
        return compat::read(__this, |__t| __t.status_bar_base().render_y);
    }

    /// java: @Override public int getWidth()
    pub fn get_width_body(__this: &compat::Shared<dyn StatusBar>) -> i32 {
        return compat::read(__this, |__t| __t.status_bar_base().render_width);
    }

    /// java: public void setWidth(int width)
    pub fn set_width_body(__this: &compat::Shared<dyn StatusBar>, mut width: i32) {
        { let __v = width; compat::write(__this, |__t| __t.status_bar_base_mut().render_width = __v); }
    }

    /// java: @Override public int getHeight()
    pub fn get_height_body(__this: &compat::Shared<dyn StatusBar>) -> i32 {
        return 9;
    }

    /// java: @Override public ScreenRectangle getRectangle()
    pub fn get_rectangle_body(__this: &compat::Shared<dyn StatusBar>) -> ScreenRectangle {
        // TODO(fabric2seer): `.getRectangle(..)` on a value of unknown type
        //   java: return LayoutElement.super.getRectangle();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_body(__this: &compat::Shared<dyn StatusBar>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_x >= (compat::read(__this, |__t| __t.status_bar_base().render_x) as f64) && mouse_x <= ((compat::read(__this, |__t| __t.status_bar_base().render_x)).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_width()); __f(__this) }) as f64) && mouse_y >= (compat::read(__this, |__t| __t.status_bar_base().render_y) as f64) && mouse_y <= ((compat::read(__this, |__t| __t.status_bar_base().render_y)).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_height()); __f(__this) }) as f64));
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer)
    pub fn visit_widgets_body(__this: &compat::Shared<dyn StatusBar>, consumer: Arc<dyn Fn(AbstractWidget)>) {
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused_body(__this: &compat::Shared<dyn StatusBar>, mut focused: bool) {
    }

    /// java: @Override public boolean isFocused()
    pub fn is_focused_body(__this: &compat::Shared<dyn StatusBar>) -> bool {
        return false;
    }

    /// java: @Override public NarrationPriority narrationPriority()
    pub fn narration_priority_body(__this: &compat::Shared<dyn StatusBar>) -> crate::unresolved::NarrationPriority {
        // TODO(fabric2seer): static field NarrationPriority.NONE
        //   java: return NarrationPriority.NONE;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn StatusBar>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(__this, click.x(), click.y()) }) {
            return false;
        }
        if compat::read(__this, |__t| __t.status_bar_base().on_click.clone()).is_some() {
            (compat::read(__this, |__t| __t.status_bar_base().on_click.clone()).unwrap())(__this.clone(), click.clone());
        }
        return true;
    }

    /// java: public void setOnClick(@Nullable OnClick onClick)
    pub fn set_on_click_body(__this: &compat::Shared<dyn StatusBar>, on_click: OnClick) {
        { let __v = Some(on_click.clone()); compat::write(__this, |__t| __t.status_bar_base_mut().on_click = __v); }
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration_body(__this: &compat::Shared<dyn StatusBar>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public String toString()
    pub fn to_string_body(__this: &compat::Shared<dyn StatusBar>) -> String {
        // TODO(fabric2seer): `new ToStringBuilder(..)`
        //   java: return new ToStringBuilder(this)
        //   java: 				.append("name", getName())
        //   java: 				.append("gridX", gridX)
        //   java: 				.append("gridY", gridY)
        //   java: 				.append("size", size)
        //   java: 				.append("x", renderX)
        //   java: 				.append("y", renderY)
        //   java: 				.append("width", renderWidth)
        //   java: 				.append("anchor", anchor)
        //   java: 				.toString();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public IconPosition getIconPosition()
    pub fn get_icon_position_body(__this: &compat::Shared<dyn StatusBar>) -> IconPosition {
        return compat::read(__this, |__t| __t.status_bar_base().icon_position);
    }

    /// java: public void setIconPosition(IconPosition iconPosition)
    pub fn set_icon_position_body(__this: &compat::Shared<dyn StatusBar>, mut icon_position: IconPosition) {
        { let __v = icon_position; compat::write(__this, |__t| __t.status_bar_base_mut().icon_position = __v); }
    }

    /// java: public boolean showText()
    pub fn show_text_body(__this: &compat::Shared<dyn StatusBar>) -> bool {
        return (compat::read(__this, |__t| __t.status_bar_base().text_position) != TextPosition::Off);
    }

    /// java: public TextPosition getTextPosition()
    pub fn get_text_position_body(__this: &compat::Shared<dyn StatusBar>) -> TextPosition {
        return compat::read(__this, |__t| __t.status_bar_base().text_position);
    }

    /// java: public void setTextPosition(TextPosition textPosition)
    pub fn set_text_position_body(__this: &compat::Shared<dyn StatusBar>, mut text_position: TextPosition) {
        { let __v = text_position; compat::write(__this, |__t| __t.status_bar_base_mut().text_position = __v); }
    }

    /// java: public Direction getDirection()
    pub fn get_direction_body(__this: &compat::Shared<dyn StatusBar>) -> Direction {
        return compat::read(__this, |__t| __t.status_bar_base().direction);
    }

    /// java: public void setDirection(Direction direction)
    pub fn set_direction_body(__this: &compat::Shared<dyn StatusBar>, mut direction: Direction) {
        { let __v = direction; compat::write(__this, |__t| __t.status_bar_base_mut().direction = __v); }
    }

    /// java: public void loadFromJson(JsonObject object)
    pub fn load_from_json_body(__this: &compat::Shared<dyn StatusBar>, object: &serde_json::Map<String, serde_json::Value>) {
        if compat::json::has(&object, "colors") {
            let mut colors1: Vec<serde_json::Value> = compat::json::get_as_json_array(&object.get("colors").cloned().unwrap());
            if (colors1.len() as i32) < 2 && { let __f = compat::dispatch(__this, |__t| __t.has_overflow()); __f(__this) } {
                panic!("IllegalStateException: {}", "Missing second color of bar that has overflow");
            }
            let mut new_colors: Vec<Color> = (0..((colors1.len() as i32)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
            for i in 0..(colors1.len() as i32) {
                let json_element = &colors1[(i) as usize];
                // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
                // rust: new_colors[(i) as usize] = Color::new(i32::from_str_radix(&compat::json::get_as_string(&json_element), (16) as u32).expect("NumberFormatException"));
            }
            { let __v = new_colors.clone(); compat::write(__this, |__t| __t.status_bar_base_mut().colors = __v); }
        }
        if compat::json::has(&object, "text_color") {
            // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
            // rust: { let __v = Some(Color::new(i32::from_str_radix(&compat::json::get_as_string(&object.get("text_color").cloned().unwrap()), (16) as u32).expect("NumberFormatException"))); compat::write(__this, |__t| __t.status_bar_base_mut().text_color = __v); }
        }
        let mut maybe_anchor: String = compat::trim(&compat::json::get_as_string(&object.get("anchor").cloned().unwrap()));
        { let __v = (if maybe_anchor == "null" { None } else { Some(BarAnchor::value_of(&maybe_anchor)) }); compat::write(__this, |__t| __t.status_bar_base_mut().anchor = __v); }
        if !(compat::json::has(&object, "enabled")) {
            { let __v = compat::read(__this, |__t| __t.status_bar_base().anchor.clone()).is_some(); compat::write(__this, |__t| __t.status_bar_base_mut().enabled = __v); }
        } else {
            { let __v = compat::json::get_as_boolean(&object.get("enabled").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().enabled = __v); }
        }
        if compat::read(__this, |__t| __t.status_bar_base().anchor.clone()).is_some() {
            { let __v = compat::json::get_as_int(&object.get("size").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().size = __v); }
            { let __v = compat::json::get_as_int(&object.get("x").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().grid_x = __v); }
            { let __v = compat::json::get_as_int(&object.get("y").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().grid_y = __v); }
        } else {
            { let __v = compat::json::get_as_float(&object.get("size").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().width = __v); }
            { let __v = compat::json::get_as_float(&object.get("x").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().x = __v); }
            { let __v = compat::json::get_as_float(&object.get("y").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().y = __v); }
        }
        if compat::json::has(&object, "icon_position") {
            { let __v = IconPosition::value_of(&compat::trim(&compat::json::get_as_string(&object.get("icon_position").cloned().unwrap()))); compat::write(__this, |__t| __t.status_bar_base_mut().icon_position = __v); }
        }
        if compat::json::has(&object, "show_text") {
            { let __v = (if compat::json::get_as_boolean(&object.get("show_text").cloned().unwrap()) { TextPosition::BarCenter } else { TextPosition::Off }); compat::write(__this, |__t| __t.status_bar_base_mut().text_position = __v); }
        }
        if compat::json::has(&object, "text_position") {
            { let __v = TextPosition::value_of(&compat::trim(&compat::json::get_as_string(&object.get("text_position").cloned().unwrap()))); compat::write(__this, |__t| __t.status_bar_base_mut().text_position = __v); }
        }
        if compat::json::has(&object, "show_max") {
            { let __v = compat::json::get_as_boolean(&object.get("show_max").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().show_max = __v); }
        }
        if compat::json::has(&object, "show_overflow") {
            { let __v = compat::json::get_as_boolean(&object.get("show_overflow").cloned().unwrap()); compat::write(__this, |__t| __t.status_bar_base_mut().show_overflow = __v); }
        }
        if compat::json::has(&object, "direction") {
            { let __v = Direction::value_of(&compat::trim(&compat::json::get_as_string(&object.get("direction").cloned().unwrap()))); compat::write(__this, |__t| __t.status_bar_base_mut().direction = __v); }
        }
    }

    /// java: public JsonObject toJson()
    pub fn to_json_body(__this: &compat::Shared<dyn StatusBar>) -> serde_json::Map<String, serde_json::Value> {
        let mut object: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut colors1: Vec<serde_json::Value> = Vec::new();
        for color in compat::read(__this, |__t| __t.status_bar_base().colors.clone()) {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: colors1.add(Integer.toHexString(color.getRGB()).substring(2));
        }
        object.insert("colors".to_string(), compat::json::to_value(colors1.clone()));
        if compat::read(__this, |__t| __t.status_bar_base().text_color.clone()).is_some() {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: object.addProperty("text_color", Integer.toHexString(textColor.getRGB()).substring(2));
        }
        if compat::read(__this, |__t| __t.status_bar_base().anchor.clone()).is_some() {
            object.insert("anchor".to_string(), serde_json::json!(compat::opt_str(&compat::read(__this, |__t| __t.status_bar_base().anchor.clone()))));
        } else {
            object.insert("anchor".to_string(), serde_json::json!("null".to_string()));
        }
        if compat::read(__this, |__t| __t.status_bar_base().anchor.clone()).is_some() {
            object.insert("x".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().grid_x)));
            object.insert("y".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().grid_y)));
            object.insert("size".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().size)));
        } else {
            object.insert("size".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().width)));
            object.insert("x".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().x)));
            object.insert("y".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().y)));
        }
        object.insert("icon_position".to_string(), serde_json::json!(__this.lock().borrow().status_bar_base().icon_position.get_serialized_name()));
        object.insert("text_position".to_string(), serde_json::json!(__this.lock().borrow().status_bar_base().text_position.get_serialized_name()));
        object.insert("show_max".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().show_max)));
        object.insert("show_overflow".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().show_overflow)));
        object.insert("direction".to_string(), serde_json::json!(__this.lock().borrow().status_bar_base().direction.get_serialized_name()));
        object.insert("enabled".to_string(), serde_json::json!(compat::read(__this, |__t| __t.status_bar_base().enabled)));
        return object;
    }

}

impl StatusBar for StatusBarBase {
    fn status_bar_base(&self) -> &StatusBarBase {
        &self
    }
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public enum IconPosition implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IconPosition {
    Left,
    Right,
    Off,
}

impl IconPosition {
    pub const VALUES: [IconPosition; 3] = [IconPosition::Left, IconPosition::Right, IconPosition::Off];

    /// java: values()
    pub fn values() -> Vec<IconPosition> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            IconPosition::Left => "LEFT",
            IconPosition::Right => "RIGHT",
            IconPosition::Off => "OFF",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> IconPosition {
        match name {
            "LEFT" => IconPosition::Left,
            "RIGHT" => IconPosition::Right,
            "OFF" => IconPosition::Off,
            _ => panic!("IllegalArgumentException: No enum constant IconPosition.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.bars.config.commonPosition." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for IconPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", IconPosition::to_string(*self))
    }
}

/// java: public enum TextPosition implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextPosition {
    Left,
    Center,
    BarCenter,
    Right,
    Off,
}

impl TextPosition {
    pub const VALUES: [TextPosition; 5] = [TextPosition::Left, TextPosition::Center, TextPosition::BarCenter, TextPosition::Right, TextPosition::Off];

    /// java: values()
    pub fn values() -> Vec<TextPosition> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TextPosition::Left => "LEFT",
            TextPosition::Center => "CENTER",
            TextPosition::BarCenter => "BAR_CENTER",
            TextPosition::Right => "RIGHT",
            TextPosition::Off => "OFF",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TextPosition {
        match name {
            "LEFT" => TextPosition::Left,
            "CENTER" => TextPosition::Center,
            "BAR_CENTER" => TextPosition::BarCenter,
            "RIGHT" => TextPosition::Right,
            "OFF" => TextPosition::Off,
            _ => panic!("IllegalArgumentException: No enum constant TextPosition.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        if *self == CENTER || *self == BAR_CENTER {
            // TODO(fabric2seer): call to `name(..)` (not a method of this class)
            //   java: return I18n.get("skyblocker.bars.config.textPosition." + name());
        }
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.bars.config.commonPosition." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", TextPosition::to_string(*self))
    }
}

/// java: public enum Direction implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    FromMiddle,
}

impl Direction {
    pub const VALUES: [Direction; 3] = [Direction::LeftToRight, Direction::RightToLeft, Direction::FromMiddle];

    /// java: values()
    pub fn values() -> Vec<Direction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Direction::LeftToRight => "LEFT_TO_RIGHT",
            Direction::RightToLeft => "RIGHT_TO_LEFT",
            Direction::FromMiddle => "FROM_MIDDLE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Direction {
        match name {
            "LEFT_TO_RIGHT" => Direction::LeftToRight,
            "RIGHT_TO_LEFT" => Direction::RightToLeft,
            "FROM_MIDDLE" => Direction::FromMiddle,
            _ => panic!("IllegalArgumentException: No enum constant Direction.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.bars.config.direction." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Direction::to_string(*self))
    }
}

/// java: @FunctionalInterface public interface OnClick
/// a functional interface: `onClick(StatusBar statusBar, MouseButtonEvent click)` - a lambda / method reference is this closure type
pub type OnClick = Arc<dyn Fn(compat::Shared<dyn StatusBar>, MouseButtonEvent)>;

/// java: public static class ManaStatusBar extends StatusBar
#[derive(Clone)]
pub struct ManaStatusBar {
    /// java: extends StatusBar
    pub base: StatusBarBase,
}

impl std::ops::Deref for ManaStatusBar {
    type Target = StatusBarBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ManaStatusBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ManaStatusBar {
    /// java: public ManaStatusBar(StatusBarType type)
    pub fn new(mut r#type: StatusBarType) -> compat::Shared<Self> {
        let (_, __this) = StatusBarBase::new_2_in(r#type, Arc::new(move |mana| (if EstimatedResource::is_estimated(&StatusBarTracker::get_mana()) && SkyblockerConfigManager::get().ui_and_visuals.bars.show_estimated_tilde { format!("~{}", mana) } else { mana.to_string() })), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        });
        __this
    }

}

impl StatusBar for ManaStatusBar {
    fn status_bar_base(&self) -> &StatusBarBase {
        &self.base
    }
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth) (dispatch: the body for this class)
    fn extract_bar_fill(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32) {
        ManaStatusBar::extract_bar_fill_dyn
    }

}

impl ManaStatusBar {
    /// java: @Override protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth)
    pub fn extract_bar_fill_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut bar_x: i32, mut bar_width: i32) {
        if { let __f = compat::dispatch(__this, |__t| __t.has_overflow()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>)) } && compat::read(__this, |__t| __t.overflow_fill) > (0 as f32) {
            if compat::read(__this, |__t| __t.overflow_fill) > compat::read(__this, |__t| __t.fill) && SkyblockerConfigManager::get().ui_and_visuals.bars.intelligence_display == IntelligenceDisplay::InFront {
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: renderBarFill(graphics, barX, barWidth, Math.min(overflowFill, 1), transparency(getColors()[1].getRGB()));
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: renderBarFill(graphics, barX, barWidth, fill, transparency(getColors()[0].getRGB()));
            } else {
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: renderBarFill(graphics, barX, barWidth, fill, transparency(getColors()[0].getRGB()));
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: renderBarFill(graphics, barX, barWidth, Math.min(overflowFill, 1), transparency(getColors()[1].getRGB()));
            }
        } else {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: renderBarFill(graphics, barX, barWidth, fill, transparency(getColors()[0].getRGB()));
        }
    }

    /// ManaStatusBar.extractBarFill as the dispatch of StatusBar.extractBarFill
    pub fn extract_bar_fill_dyn(__this: &compat::Shared<dyn StatusBar>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_bar_fill_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: public static class ExperienceStatusBar extends StatusBar
#[derive(Clone)]
pub struct ExperienceStatusBar {
    /// java: extends StatusBar
    pub base: StatusBarBase,
}

impl std::ops::Deref for ExperienceStatusBar {
    type Target = StatusBarBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ExperienceStatusBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ExperienceStatusBar {
    /// java: public ExperienceStatusBar(StatusBarType type)
    pub fn new(mut r#type: StatusBarType) -> compat::Shared<Self> {
        let (_, __this) = StatusBarBase::new_2_in(r#type, Arc::new(move |time| {
            if Utils::is_in_the_rift() {
                return (if time < 60 { format!("{}s", time) } else { format!("{}m{:02}s", time.wrapping_div(60), time.wrapping_rem(60)) });
            }
            return time.to_string();
        }), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        });
        __this
    }

}

impl StatusBar for ExperienceStatusBar {
    fn status_bar_base(&self) -> &StatusBarBase {
        &self.base
    }
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected Identifier getIcon() (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Identifier {
        ExperienceStatusBar::get_icon_dyn
    }

}

impl ExperienceStatusBar {
    /// java: @Override protected Identifier getIcon()
    pub fn get_icon_impl(__this: &compat::Shared<Self>) -> Identifier {
        return (if Utils::is_in_the_rift() { EXPERIENCE_STATUS_BAR_CLOCK_ICON.get_mut().clone() } else { <dyn StatusBar>::get_icon_body(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>)) });
    }

    /// ExperienceStatusBar.getIcon as the dispatch of StatusBar.getIcon
    pub fn get_icon_dyn(__this: &compat::Shared<dyn StatusBar>) -> Identifier {
        Self::get_icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class HealthStatusBar extends StatusBar
#[derive(Clone)]
pub struct HealthStatusBar {
    /// java: extends StatusBar
    pub base: StatusBarBase,
}

impl std::ops::Deref for HealthStatusBar {
    type Target = StatusBarBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HealthStatusBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HealthStatusBar {
    /// java: public HealthStatusBar(StatusBarType type)
    pub fn new(mut r#type: StatusBarType) -> compat::Shared<Self> {
        let (_, __this) = StatusBarBase::new_in(r#type, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        });
        __this
    }

}

impl StatusBar for HealthStatusBar {
    fn status_bar_base(&self) -> &StatusBarBase {
        &self.base
    }
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth) (dispatch: the body for this class)
    fn extract_bar_fill(&self) -> fn(&compat::Shared<dyn StatusBar>, &GuiGraphicsExtractor, i32, i32) {
        HealthStatusBar::extract_bar_fill_dyn
    }

    /// java: @Override protected Identifier getIcon() (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn StatusBar>) -> Identifier {
        HealthStatusBar::get_icon_dyn
    }

}

impl HealthStatusBar {
    /// java: @Override protected void extractBarFill(GuiGraphicsExtractor graphics, int barX, int barWidth)
    pub fn extract_bar_fill_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut bar_x: i32, mut bar_width: i32) {
        let client = compat::minecraft();
        let mut withering: bool = (mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.has_effect(mcrust::net::minecraft::world::effect::mob_effects::wither())).unwrap_or(false));
        let mut poisoned: bool = (mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.has_effect(mcrust::net::minecraft::world::effect::mob_effects::poison())).unwrap_or(false));
        let mut fill_color: i32;
        if withering {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: fillColor = WITHER_COLOR.getRGB();
        } else if poisoned {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: fillColor = POISON_COLOR.getRGB();
        } else {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: fillColor = getColors()[0].getRGB();
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.render_bar_fill()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>), graphics, bar_x, bar_width, compat::read(__this, |__t| __t.fill), ({ let __f = compat::dispatch(__this, |__t| __t.transparency()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>), fill_color) })) });
        if { let __f = compat::dispatch(__this, |__t| __t.has_overflow()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>)) } && compat::read(__this, |__t| __t.overflow_fill) > (0 as f32) {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: renderBarFill(graphics, barX, barWidth, Math.min(overflowFill, 1), transparency(getColors()[1].getRGB()));
        }
    }

    /// HealthStatusBar.extractBarFill as the dispatch of StatusBar.extractBarFill
    pub fn extract_bar_fill_dyn(__this: &compat::Shared<dyn StatusBar>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_bar_fill_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override protected Identifier getIcon()
    pub fn get_icon_impl(__this: &compat::Shared<Self>) -> Identifier {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            if mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.has_effect(mcrust::net::minecraft::world::effect::mob_effects::wither())).unwrap_or(false) {
                return HEALTH_STATUS_BAR_WITHER_ICON.get_mut().clone();
            } else if mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.has_effect(mcrust::net::minecraft::world::effect::mob_effects::poison())).unwrap_or(false) {
                return HEALTH_STATUS_BAR_POISON_ICON.get_mut().clone();
            }
        }
        return <dyn StatusBar>::get_icon_body(&compat::view(__this, |a| a as compat::Shared<dyn StatusBar>));
    }

    /// HealthStatusBar.getIcon as the dispatch of StatusBar.getIcon
    pub fn get_icon_dyn(__this: &compat::Shared<dyn StatusBar>) -> Identifier {
        Self::get_icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class VitalityStatusBar extends StatusBar
#[derive(Clone)]
pub struct VitalityStatusBar {
    /// java: extends StatusBar
    pub base: StatusBarBase,
}

impl std::ops::Deref for VitalityStatusBar {
    type Target = StatusBarBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for VitalityStatusBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl VitalityStatusBar {
    /// java: public VitalityStatusBar(StatusBarType type)
    pub fn new(mut r#type: StatusBarType) -> compat::Shared<Self> {
        let (_, __this) = StatusBarBase::new_2_in(r#type, Arc::new(move |vitality| (if EstimatedResource::is_estimated(&StatusBarTracker::get_vitality()) && SkyblockerConfigManager::get().ui_and_visuals.bars.show_estimated_tilde { format!("~{}", vitality) } else { vitality.to_string() })), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn StatusBar>), __h)
        });
        __this
    }

}

impl StatusBar for VitalityStatusBar {
    fn status_bar_base(&self) -> &StatusBarBase {
        &self.base
    }
    fn status_bar_base_mut(&mut self) -> &mut StatusBarBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
