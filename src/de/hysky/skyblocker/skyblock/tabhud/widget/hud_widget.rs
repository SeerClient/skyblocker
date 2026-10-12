//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.HudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class HudWidget implements LayoutElement
#[derive(Clone)]
pub struct HudWidgetBase {
    /// java: protected int w = 0, h = 0;
    pub w: i32,
    /// java: protected int w = 0, h = 0;
    pub h: i32,
    /// java: protected int x = 0, y = 0;
    pub x: i32,
    /// java: protected int x = 0, y = 0;
    pub y: i32,
    /// java: protected float scale = 1;
    pub scale: f32,
    /// java: private final Information information;
    pub information: Information,
    /// the object's own shared handle: Java's `this` handed to other objects (`element.setParent(this)`)
    pub self_ref: Option<compat::WeakShared<dyn HudWidget>>,
}

impl HudWidgetBase {
    /// java: public HudWidget(Information information) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut information: Information, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HudWidget>, __R)) -> (compat::Shared<dyn HudWidget>, __R) {
        let __init_information = information.clone();
        let this = Self {
            w: 0,
            h: 0,
            x: 0,
            y: 0,
            scale: ((1) as f32) as f32,
            information: __init_information,
            self_ref: None,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __w = std::sync::Arc::downgrade(&__this); compat::write(&__this, |__t| __t.hud_widget_base_mut().self_ref = Some(__w)); }
        (__this, __r)
    }

    /// `this` as the `LayoutElement` an element's `setParent(this)` is handed
    pub fn as_parent(&self) -> crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementParent {
        crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementParent::Widget(
            self.self_ref.clone().expect("a HudWidget's handle is recorded by its constructor"),
        )
    }

    /// java: public static String nameToId(String name)
    pub fn name_to_id(name: &str) -> String {
        return (name.to_lowercase().replace(' ', &'_'.to_string())).replace("'", "");
    }

}

/// java: the virtual methods of HudWidget (implemented by HudWidgetBase and every subclass)
pub trait HudWidget: compat::DynAny {
    fn hud_widget_base(&self) -> &HudWidgetBase;
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected abstract void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta); (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32);

    /// java: protected abstract void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta); (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32);

    /// java: public final void extractRenderState(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn HudWidget>::extract_render_state_body
    }

    /// java: public final void extractRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn HudWidget>::extract_render_state_for_config_body
    }

    /// java: public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        <dyn HudWidget>::should_render_body
    }

    /// java: public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn HudWidget>::load_body
    }

    /// java: public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn HudWidget>::save_body
    }

    /// java: public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn HudWidget>::get_option_widgets_body
    }

    /// java: public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn HudWidget>::update_config_preview_body
    }

    /// java: @Override public final boolean equals(Object object) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn HudWidget>, &compat::Shared<dyn HudWidget>) -> bool {
        <dyn HudWidget>::equals_body
    }

    /// java: @Override public final int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn HudWidget>) -> i32 {
        <dyn HudWidget>::hash_code_body
    }

    /// java: public final String getInternalID() (dispatch: the body for this class)
    fn get_internal_id(&self) -> fn(&compat::Shared<dyn HudWidget>) -> String {
        <dyn HudWidget>::get_internal_id_body
    }

    /// java: public final Information getInformation() (dispatch: the body for this class)
    fn get_information(&self) -> fn(&compat::Shared<dyn HudWidget>) -> Information {
        <dyn HudWidget>::get_information_body
    }

    /// java: public final int getX() (dispatch: the body for this class)
    fn get_x(&self) -> fn(&compat::Shared<dyn HudWidget>) -> i32 {
        <dyn HudWidget>::get_x_body
    }

    /// java: public final void setX(int x) (dispatch: the body for this class)
    fn set_x(&self) -> fn(&compat::Shared<dyn HudWidget>, i32) {
        <dyn HudWidget>::set_x_body
    }

    /// java: public final int getY() (dispatch: the body for this class)
    fn get_y(&self) -> fn(&compat::Shared<dyn HudWidget>) -> i32 {
        <dyn HudWidget>::get_y_body
    }

    /// java: public final void setY(int y) (dispatch: the body for this class)
    fn set_y(&self) -> fn(&compat::Shared<dyn HudWidget>, i32) {
        <dyn HudWidget>::set_y_body
    }

    /// java: public final int getWidth() (dispatch: the body for this class)
    fn get_width(&self) -> fn(&compat::Shared<dyn HudWidget>) -> i32 {
        <dyn HudWidget>::get_width_body
    }

    /// java: public final int getHeight() (dispatch: the body for this class)
    fn get_height(&self) -> fn(&compat::Shared<dyn HudWidget>) -> i32 {
        <dyn HudWidget>::get_height_body
    }

    /// java: public final boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn HudWidget>, f64, f64) -> bool {
        <dyn HudWidget>::is_mouse_over_body
    }

    /// java: @Override public final void visitWidgets(Consumer<AbstractWidget> widgetVisitor) (dispatch: the body for this class)
    fn visit_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, Arc<dyn Fn(AbstractWidget)>) {
        <dyn HudWidget>::visit_widgets_body
    }

}

impl dyn HudWidget {
    /// java: public final void extractRenderState(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn HudWidget>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        // java: graphics.pose().scale(scale);
        { let __s = compat::read(__this, |__t| __t.hud_widget_base().scale); gui_graphics_extractor::pose_scale(__s, __s); }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_widget_render_state()); __f(__this, graphics, delta) });
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: public final void extractRenderStateForConfig(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_render_state_for_config_body(__this: &compat::Shared<dyn HudWidget>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        // java: graphics.pose().scale(scale);
        { let __s = compat::read(__this, |__t| __t.hud_widget_base().scale); gui_graphics_extractor::pose_scale(__s, __s); }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_widget_render_state_for_config()); __f(__this, graphics, delta) });
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: public boolean shouldRender()
    pub fn should_render_body(__this: &compat::Shared<dyn HudWidget>) -> bool {
        return true;
    }

    /// java: public void load(JsonValueInput input)
    pub fn load_body(__this: &compat::Shared<dyn HudWidget>, input: &JsonValueInput) {
        { let __v = input.read_float_or("scale", ((1) as f32) as f32); compat::write(__this, |__t| __t.hud_widget_base_mut().scale = __v); }
    }

    /// java: public void save(JsonValueOutput output)
    pub fn save_body(__this: &compat::Shared<dyn HudWidget>, output: &mut JsonValueOutput) {
        output.write_number("scale", &mcrust::java::lang::number::Number::Float(compat::read(__this, |__t| __t.hud_widget_base().scale)));
    }

    /// java: public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_body(__this: &compat::Shared<dyn HudWidget>, collector: &mut OptionWidgetCollector) {
        // java: collector.slider(Component.translatable("skyblocker.config.hud.widget.base.scale"), d -> scale = (float) d, scale, 0.1, 5);
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.slider(&component::translatable("skyblocker.config.hud.widget.base.scale"), Arc::new(move |d: f64| { if let Some(__this) = __this_w.get().upgrade() { compat::write(&__this, |__t| __t.hud_widget_base_mut().scale = d as f32); } }), compat::read(__this, |__t| __t.hud_widget_base().scale) as f64, 0.1, 5.0);
    }

    /// java: public void updateConfigPreview()
    pub fn update_config_preview_body(__this: &compat::Shared<dyn HudWidget>) {
    }

    /// java: @Override public final boolean equals(Object object)
    /// (`object` is a HudWidget at every caller: `PositionedWidget.equals`, `LayerBuilderEditor`)
    pub fn equals_body(__this: &compat::Shared<dyn HudWidget>, object: &compat::Shared<dyn HudWidget>) -> bool {
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(__this), std::sync::Arc::as_ptr(object)) {
            return true;
        }
        // java: object == null || getClass() != object.getClass()
        if compat::read(__this, |__t| __t.dyn_any().type_id()) != compat::read(object, |__t| __t.dyn_any().type_id()) {
            return false;
        }
        let widget: &compat::Shared<dyn HudWidget> = object;
        return (({ let __f = compat::dispatch(__this, |__t| __t.get_internal_id()); __f(__this) }) == ({ let __f = compat::dispatch(widget, |__t| __t.get_internal_id()); __f(widget) }));
    }

    /// java: a `Set<HudWidget>` / `List.contains` membership test, which reads `equals` (the internal id, the same class)
    pub fn contains_equal(set: &[compat::Shared<dyn HudWidget>], widget: &compat::Shared<dyn HudWidget>) -> bool {
        set.iter().any(|w| <dyn HudWidget>::equals_body(w, widget))
    }

    /// java: @Override public final int hashCode()
    pub fn hash_code_body(__this: &compat::Shared<dyn HudWidget>) -> i32 {
        return compat::string_hash_code(&({ let __f = compat::dispatch(__this, |__t| __t.get_internal_id()); __f(__this) }));
    }

    /// java: public final String getInternalID()
    pub fn get_internal_id_body(__this: &compat::Shared<dyn HudWidget>) -> String {
        return __this.lock().borrow().hud_widget_base().information.id().unwrap();
    }

    /// java: public final Information getInformation()
    pub fn get_information_body(__this: &compat::Shared<dyn HudWidget>) -> Information {
        return compat::read(__this, |__t| __t.hud_widget_base().information.clone());
    }

    /// java: public final int getX()
    pub fn get_x_body(__this: &compat::Shared<dyn HudWidget>) -> i32 {
        return compat::read(__this, |__t| __t.hud_widget_base().x);
    }

    /// java: public final void setX(int x)
    pub fn set_x_body(__this: &compat::Shared<dyn HudWidget>, mut x: i32) {
        { let __v = x; compat::write(__this, |__t| __t.hud_widget_base_mut().x = __v); }
    }

    /// java: public final int getY()
    pub fn get_y_body(__this: &compat::Shared<dyn HudWidget>) -> i32 {
        return compat::read(__this, |__t| __t.hud_widget_base().y);
    }

    /// java: public final void setY(int y)
    pub fn set_y_body(__this: &compat::Shared<dyn HudWidget>, mut y: i32) {
        { let __v = y; compat::write(__this, |__t| __t.hud_widget_base_mut().y = __v); }
    }

    /// java: public final int getWidth()
    pub fn get_width_body(__this: &compat::Shared<dyn HudWidget>) -> i32 {
        return compat::java_round_f32(((compat::read(__this, |__t| __t.hud_widget_base().w) as f32) * compat::read(__this, |__t| __t.hud_widget_base().scale)));
    }

    /// java: public final int getHeight()
    pub fn get_height_body(__this: &compat::Shared<dyn HudWidget>) -> i32 {
        return compat::java_round_f32(((compat::read(__this, |__t| __t.hud_widget_base().h) as f32) * compat::read(__this, |__t| __t.hud_widget_base().scale)));
    }

    /// java: public final boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_body(__this: &compat::Shared<dyn HudWidget>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_x >= (({ let __f = compat::dispatch(__this, |__t| __t.get_x()); __f(__this) }) as f64) && mouse_x <= (({ let __f = compat::dispatch(__this, |__t| __t.get_x()); __f(__this) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_width()); __f(__this) }) as f64) && mouse_y >= (({ let __f = compat::dispatch(__this, |__t| __t.get_y()); __f(__this) }) as f64) && mouse_y < (({ let __f = compat::dispatch(__this, |__t| __t.get_y()); __f(__this) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_height()); __f(__this) }) as f64));
    }

    /// java: @Override public final void visitWidgets(Consumer<AbstractWidget> widgetVisitor)
    pub fn visit_widgets_body(__this: &compat::Shared<dyn HudWidget>, widget_visitor: Arc<dyn Fn(AbstractWidget)>) {
    }

}

/// java: public record Information(String id, Component displayName, Predicate<Location> available)
#[derive(Clone)]
pub struct Information {
    /// java: String id
    pub id: Option<String>,
    /// java: Component displayName
    pub display_name: Option<MutableComponent>,
    /// java: Predicate<Location> available
    pub available: Arc<dyn Fn(Location) -> bool>,
}

/// java: the record's `toString()`
impl std::fmt::Debug for Information {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Information").field("id", &self.id).field("display_name", &self.display_name).finish_non_exhaustive()
    }
}

impl Information {
    /// java: the canonical record constructor `Information(String id, Component displayName, Predicate<Location> available)`
    pub fn new_canonical(id: Option<String>, display_name: Option<MutableComponent>, available: Arc<dyn Fn(Location) -> bool>) -> Self {
        Self { id, display_name, available }
    }

    /// java: public Information(String id, Component displayName)
    pub fn new(id: Option<String>, display_name: Option<MutableComponent>) -> Self {
        Self::new_canonical(id, display_name, Arc::new(move |_| true))
    }

    /// java: public Information(String id, Component displayName, Set<Location> allowedLocations)
    pub fn new2(id: String, display_name: MutableComponent, allowed_locations: std::collections::HashSet<Location>) -> Self {
        Self::new_canonical(Some(id), Some(display_name), Arc::new(move |__a0| allowed_locations.contains(&__a0)))
    }

    /// java: public Information(String id, Component displayName, Location allowedLocation)
    pub fn new3(mut id: String, mut display_name: MutableComponent, mut allowed_location: Location) -> Self {
        let mut this = Self::new2(id.clone(), display_name.clone(), std::collections::HashSet::from([allowed_location]));
        this
    }

    /// java: public Information(String id, Component displayName, Location allowedLocation, Location... allowedLocations)
    pub fn new_4(mut id: String, mut display_name: MutableComponent, mut allowed_location: Location, mut allowed_locations: Vec<Location>) -> Self {
        let mut this = Self::new2(id.clone(), display_name.clone(), std::iter::once(allowed_location).chain(allowed_locations.iter().copied()).collect());
        this
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    /// java: the record accessor `displayName()`
    pub fn display_name(&self) -> Option<MutableComponent> {
        self.display_name.clone()
    }

    /// java: the record accessor `available()`
    pub fn available(&self) -> Arc<dyn Fn(Location) -> bool> {
        self.available.clone()
    }

}
