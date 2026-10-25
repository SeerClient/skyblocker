//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.DyeSelectPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::Keyframe;
use crate::de::hysky::skyblocker::skyblock::item::custom::repo_dye_colors;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ScrollableLayoutElement;
use mcrust::net::minecraft::client::gui::components::scrollable_layout::ReserveStrategy;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int TEXT_OFFSET = 10;
pub const STATIC_DYE_BUTTON_TEXT_OFFSET: i32 = 10;

/// java: public class DyeSelectPopup extends AbstractPopupScreen
pub struct DyeSelectPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private ScrollableLayout scrollableLayout;
    /// (the port's `ScrollableLayout` layout is `ScrollableLayoutElement`; its scroll container is not a widget of a
    /// mod screen, so the content does not scroll - GAPS_O)
    pub scrollable_layout: Option<ScrollableLayoutElement>,
    /// java: private StringWidget titleWidget;
    pub title_widget: Option<compat::WidgetRef>,
    /// java: private Button closeButton;
    pub close_button: Option<compat::WidgetRef>,
    /// java: private final BiConsumer<Button, Integer> updateStatic;
    pub update_static: Arc<dyn Fn(AbstractWidget, i32)>,
    /// java: private final AnimatedDyeConsumer updateAnimated;
    pub update_animated: AnimatedDyeConsumer,
}

impl std::ops::Deref for DyeSelectPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DyeSelectPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DyeSelectPopup {
    /// java: protected DyeSelectPopup(Screen backgroundScreen, BiConsumer<Button, Integer> updateStatic, AnimatedDyeConsumer updateAnimated)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, update_static: Arc<dyn Fn(AbstractWidget, i32)>, update_animated: AnimatedDyeConsumer) -> compat::Shared<Self> {
        // java: super(Component.translatable("skyblocker.customization.armor.pickDye"), backgroundScreen);
        let (_, __this) = AbstractPopupScreenBase::new_in(component::translatable("skyblocker.customization.armor.pickDye"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                scrollable_layout: None,
                title_widget: None,
                close_button: None,
                update_static,
                update_animated,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        __this
    }

    /// `layout.addChild(button)`: the button joins this screen's widgets (the port's layouts hold a widget by its place
    /// there) and the layout gets its leaf.
    fn add_dye_button(__this: &compat::Shared<Self>, button: compat::Shared<dyn StaticDyeButton>) -> LayoutElement {
        let widget = compat::WidgetAdapter::<dyn StaticDyeButton>::widget(button);
        let (w, h) = (widget.get_width(), widget.get_height());
        LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(widget)), w, h)
    }

    /// java: private void selectStaticDye(Button button, int dyeColor)
    pub fn select_static_dye(__this: &compat::Shared<Self>, button: &AbstractWidget, mut dye_color: i32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.on_close()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        ((compat::read(__this, |__t| __t.update_static.clone())))(button.clone(), dye_color);
    }

    /// java: private void selectAnimatedDye(Button button, List<Integer> colors)
    pub fn select_animated_dye(__this: &compat::Shared<Self>, button: &AbstractWidget, colors: &Vec<i32>) {
        let mut cycle_back: bool = ((colors.len() as i32).wrapping_rem(2) == 0);
        let mut max: i32 = (if cycle_back { (colors.len() as i32).wrapping_div(2) } else { (colors.len() as i32) });
        let mut key_frames: Vec<Keyframe> = Vec::with_capacity(((colors.len() as i32)) as usize);
        if minecraft::has_shift_down() {
            let mut i: i32 = 0;
            for color in colors.iter().copied() {
                key_frames.push(Keyframe::new(color, (((i) as f32) / (max as f32))));
                i = i.wrapping_add(1);
                if cycle_back && i >= max {
                    break;
                }
            }
        } else {
            key_frames.push(Keyframe::new(colors[0], ((0) as f32) as f32));
            let mut next: Vec3 = (OkLabColor::linear_srgb2_ok_lab(argb::red_float(colors[(1) as usize]), argb::green_float(colors[(1) as usize]), argb::blue_float(colors[(1) as usize]))).to_vec3();
            let mut current: Vec3 = (OkLabColor::linear_srgb2_ok_lab(argb::red_float(colors[0]), argb::green_float(colors[0]), argb::blue_float(colors[0]))).to_vec3();
            let mut direction: Vec3 = next.subtract_vec(&current).normalize();
            for i in 1..max.wrapping_sub(1) {
                current = next.clone();
                next = (OkLabColor::linear_srgb2_ok_lab(argb::red_float(colors[(i.wrapping_add(1)) as usize]), argb::green_float(colors[(i.wrapping_add(1)) as usize]), argb::blue_float(colors[(i.wrapping_add(1)) as usize]))).to_vec3();
                let mut new_direction: Vec3 = next.subtract_vec(&current).normalize();
                if new_direction.dot(&direction) < 0.75 {
                    key_frames.push(Keyframe::new(colors[(i) as usize], (((i) as f32) / (max as f32))));
                    direction = new_direction.clone();
                }
            }
            key_frames.push(Keyframe::new(colors[(max.wrapping_sub(1)) as usize], ((1) as f32) as f32));
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.on_close()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        (compat::read(__this, |__t| __t.update_animated.clone()))(button.clone(), key_frames.clone(), cycle_back);
    }

}

impl AbstractPopupScreen for DyeSelectPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        DyeSelectPopup::extract_background_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        DyeSelectPopup::reposition_elements_dyn
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        DyeSelectPopup::init_dyn
    }

}

impl DyeSelectPopup {
    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        let (x, y, width, height) = compat::read(__this, |__t| { let __l = __t.scrollable_layout.as_ref().unwrap(); (__l.get_x(), __l.get_y(), __l.get_width(), __l.get_height()) });
        AbstractPopupScreenBase::extract_popup_background(graphics, x, y, width, height);
    }

    /// DyeSelectPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::reposition_elements_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        // java: scrollableLayout.setMaxHeight(Math.min(300, (int) (height * 0.68)));
        // java: scrollableLayout.arrangeElements();
        // java: scrollableLayout.setPosition((width - scrollableLayout.getWidth()) / 2, (height - scrollableLayout.getHeight()) / 2);
        compat::write(__this, |__t| {
            let (__width, __height) = (__t.width, __t.height);
            let __s = &mut *__t;
            let __l = __s.scrollable_layout.as_mut().unwrap();
            __l.set_max_height(300i32.min(((__height as f64) * 0.68) as i32));
            __l.arrange_elements();
            let (__x, __y) = ((__width.wrapping_sub(__l.get_width())).wrapping_div(2), (__height.wrapping_sub(__l.get_height())).wrapping_div(2));
            __l.set_x(__x);
            __l.set_y(__y);
            // the layout's widgets are this screen's: they take the places the layout gave them
            let mut __placed = Vec::new();
            __l.visit_widgets(&mut __placed);
            for (__i, __wx, __wy) in __placed {
                __s.base.base.widget_mut(__i).set_position(__wx, __wy);
            }
        });
        let (__layout_y, __layout_height) = compat::read(__this, |__t| { let __l = __t.scrollable_layout.as_ref().unwrap(); (__l.get_y(), __l.get_height()) });
        // java: closeButton.setPosition((width - closeButton.getWidth()) / 2, scrollableLayout.getY() + scrollableLayout.getHeight() + 20);
        compat::write(__this, |__t| {
            let __width = __t.width;
            let __s = &mut *__t;
            let __r = __s.close_button.as_mut().unwrap();
            __s.base.__widgets.with(&mut __s.base.base, __r, |__w| { let __x = (__width.wrapping_sub(__w.get_width())).wrapping_div(2); __w.set_position(__x, (__layout_y.wrapping_add(__layout_height)).wrapping_add(20)) })
        });
        // java: titleWidget.setPosition((width - titleWidget.getWidth()) / 2, scrollableLayout.getY() - 30);
        compat::write(__this, |__t| {
            let __width = __t.width;
            let __s = &mut *__t;
            let __r = __s.title_widget.as_mut().unwrap();
            __s.base.__widgets.with(&mut __s.base.base, __r, |__w| { let __x = (__width.wrapping_sub(__w.get_width())).wrapping_div(2); __w.set_position(__x, __layout_y.wrapping_sub(30)) })
        });
    }

    /// DyeSelectPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        let mut layout: LinearLayout = LinearLayout::vertical();
        { let __s = layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        let mut header_layout: LayoutSettings = layout.default_cell_setting().copy().padding_bottom(6);
        layout.add_child_with({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable_args("skyblocker.customization.armor.pickDye.dyes", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.customization.armor.pickDye.dyes.static"))])); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, header_layout.clone());
        for (name, hex) in compat::lock(&repo_dye_colors::STATIC_DYES).iter() {
            // `hex` borrows the locked map; the press lambda is 'static, so it takes a copy.
            let hex = *hex;
            let __this_1 = compat::RenderThread::new(__this.clone());
            layout.add_child(DyeSelectPopup::add_dye_button(__this, StaticDyeButtonBase::new(name.to_string(), hex, OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { DyeSelectPopup::select_static_dye(__this_1.get(), &compat::write(__this_1.get(), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.clone()) }), hex); } }) }))) as compat::Shared<dyn StaticDyeButton>));
        }
        layout.add_child(SpacerElement::height(15).as_layout_element());
        layout.add_child_with({ let __w = compat::string_widget(0, 0, -1, 9, component::translatable_args("skyblocker.customization.armor.pickDye.dyes", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.customization.armor.pickDye.dyes.animated"))])); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, header_layout.clone());
        for (name, colors) in compat::lock(&repo_dye_colors::ANIMATED_DYES).iter() {
            // `return` inside the forEach lambda: skip this entry
            if name.starts_with("FAIRY") {
                continue;
            }
            // `colors` borrows the locked map; the press lambda is 'static, so it owns a clone.
            let colors = colors.clone();
            let __this_2 = compat::RenderThread::new(__this.clone());
            layout.add_child(DyeSelectPopup::add_dye_button(__this, AnimatedDyeColor::new(name.to_string(), colors.clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { DyeSelectPopup::select_animated_dye(__this_2.get(), &compat::write(__this_2.get(), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.clone()) }), &colors); } }) }))) as compat::Shared<dyn StaticDyeButton>));
        }
        // java: scrollableLayout = new ScrollableLayout(client, layout, 0); - the three-argument constructor's strategy is BOTH
        { let __v = Some(ScrollableLayoutElement::new(LayoutElement::Grid(layout.wrapped.clone()), 0, ReserveStrategy::Both)); compat::write(__this, |__t| __t.scrollable_layout = __v); }
        // java: layout.visitWidgets(this::addRenderableWidget); - the widgets joined this screen when the layout took them
        { let mut __vw = Vec::new(); LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw); compat::write(__this, |__t| for (__i, _, _) in __vw { __t.base.base.mark_renderable(__i); }); }
        { let __v = (Some(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customization.armor.pickDye.title")))).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.title_widget = __v; }); }
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some(((button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ({ let __f = compat::dispatch((&__this_4), |__t| __t.on_close()); __f(&compat::view((&__this_4), |a| a as compat::Shared<dyn AbstractPopupScreen>)) }); } }) })))).width(75)).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.close_button = __v; }); }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.title_widget.as_mut().unwrap(); __s.base.__widgets.add(&mut __s.base.base, __r, true) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.close_button.as_mut().unwrap(); __s.base.__widgets.add(&mut __s.base.base, __r, true) });
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        ({ let __f = compat::dispatch(__this, |__t| __t.reposition_elements()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
    }

    /// DyeSelectPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// DyeSelectPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for DyeSelectPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// DyeSelectPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<DyeSelectPopup> {
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
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
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}

/// java: private static class StaticDyeButton extends Button.Plain
pub struct StaticDyeButtonBase {
    /// java: extends net.minecraft.client.gui.components.Button$Plain
    pub base: AbstractWidget,
    /// java: String name;
    pub name: String,
    /// java: final ItemStack dyeStack;
    pub dye_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// `AbstractButton.extractDefaultSprite` - the game's button sprite pass (see `compat::ButtonLook`)
    pub look: compat::ButtonLook,
}

impl std::ops::Deref for StaticDyeButtonBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StaticDyeButtonBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StaticDyeButtonBase {
    /// java: protected StaticDyeButton(String dyeId, int color, OnPress onPress) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut dye_id: String, mut color: i32, mut on_press: OnPress, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn StaticDyeButton>, __R)) -> (compat::Shared<dyn StaticDyeButton>, __R) {
        let __init_name = dye_id.clone();
        let mut item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&dye_id);
        // java: super(0, 0, 150, 20, Component.empty(), onPress, _ -> Component.empty()); - a plain button whose press is
        //   `onPress` (the game runs it on the widget's press) and whose narration is empty
        let this = Self {
            base: AbstractWidget::new(0, 0, 150, 20, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText, on_press),
            name: __init_name,
            dye_stack: None,
            look: compat::ButtonLook::new(),
        };
        let (__this, __r) = __wrap(this);
        if let Some(item) = &item {
            //   java: name = ChatFormatting.stripFormatting(item.getDisplayName());
            let __v = mcrust::net::minecraft::chat_formatting::strip_formatting(item.get_display_name());
            compat::write(&__this, |__t| __t.static_dye_button_base_mut().name = __v);
        }
        { let __v = Some(FlexibleItemStack::get_stack_or_throw(&ItemRepository::get_item_stack_2(&dye_id, ico::BARRIER.get_mut()))); compat::write(&__this, |__t| __t.static_dye_button_base_mut().dye_stack = __v); }
        let mut component: MutableComponent = (component::literal(&compat::read(&__this, |__t| __t.static_dye_button_base().name.clone()))).with_color(color);
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component.clone()),); __this.lock().borrow_mut().static_dye_button_base_mut().set_message_text(__q0) };
        (__this, __r)
    }

    /// java: protected StaticDyeButton(String dyeId, int color, OnPress onPress)
    pub fn new(mut dye_id: String, mut color: i32, mut on_press: OnPress) -> compat::Shared<Self> {
        Self::new_in(dye_id, color, on_press, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn StaticDyeButton>), __h)
        }).1
    }

}

/// java: the virtual methods of StaticDyeButton (implemented by StaticDyeButtonBase and every subclass)
pub trait StaticDyeButton: compat::DynAny {
    fn static_dye_button_base(&self) -> &StaticDyeButtonBase;
    fn static_dye_button_base_mut(&mut self) -> &mut StaticDyeButtonBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_contents(&self) -> fn(&compat::Shared<dyn StaticDyeButton>, &AbstractWidget, &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, i32, i32, f32) {
        <dyn StaticDyeButton>::extract_contents_body
    }

    /// java: protected void extractName(GuiGraphicsExtractor graphics, float a) (dispatch: the body for this class)
    fn extract_name(&self) -> fn(&compat::Shared<dyn StaticDyeButton>, &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, f32) {
        <dyn StaticDyeButton>::extract_name_body
    }

}

impl dyn StaticDyeButton {
    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// (`widget` / `context`: the game's widget and the frame, which the default sprite is drawn through)
    pub fn extract_contents_body(__this: &compat::Shared<dyn StaticDyeButton>, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // java: this.extractDefaultSprite(graphics);
        compat::write(__this, |__t| __t.static_dye_button_base_mut().look.draw_sprite(widget, context, mouse_x, mouse_y));
        let graphics = &context.graphics;
        graphics.item(&compat::read(__this, |__t| __t.static_dye_button_base().dye_stack.clone()).unwrap(), (__this.lock().borrow().static_dye_button_base().get_x()).wrapping_add(STATIC_DYE_BUTTON_TEXT_OFFSET), (__this.lock().borrow().static_dye_button_base().get_y()).wrapping_add(1));
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_name()); __f(__this, context, a) });
    }

    /// java: protected void extractName(GuiGraphicsExtractor graphics, float a)
    pub fn extract_name_body(__this: &compat::Shared<dyn StaticDyeButton>, context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut a: f32) {
        let _ = a;
        // java: graphics.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(getMessage(), getX() + TEXT_MARGIN + TEXT_OFFSET + 16, getRight() - TEXT_MARGIN, getY() + TEXT_MARGIN, getBottom() - TEXT_MARGIN);
        let text_margin = 2;
        let (message, x, y, right, bottom) = compat::read(__this, |__t| {
            let b = __t.static_dye_button_base();
            (b.message_text.clone().unwrap_or_else(|| mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(b.message.clone())), b.get_x(), b.get_y(), b.get_right(), b.get_bottom())
        });
        compat::accept_scrolling_with_default_center(context, &message, x + text_margin + STATIC_DYE_BUTTON_TEXT_OFFSET + 16, right - text_margin, y + text_margin, bottom - text_margin);
    }

}

impl StaticDyeButton for StaticDyeButtonBase {
    fn static_dye_button_base(&self) -> &StaticDyeButtonBase {
        self
    }
    fn static_dye_button_base_mut(&mut self) -> &mut StaticDyeButtonBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: private static class AnimatedDyeColor extends StaticDyeButton
pub struct AnimatedDyeColor {
    /// java: extends StaticDyeButton
    pub base: StaticDyeButtonBase,
    /// java: List<Component> animatedNames = new ArrayList<>();
    pub animated_names: Vec<MutableComponent>,
    /// java: int index = 0;
    pub index: i32,
    /// java: float lastChange = 0;
    pub last_change: f32,
}

impl std::ops::Deref for AnimatedDyeColor {
    type Target = StaticDyeButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnimatedDyeColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnimatedDyeColor {
    /// java: protected AnimatedDyeColor(String dyeId, List<Integer> colors, OnPress onPress)
    pub fn new(mut dye_id: String, mut colors: Vec<i32>, mut on_press: OnPress) -> compat::Shared<Self> {
        let (_, __this) = StaticDyeButtonBase::new_in(dye_id.clone(), colors[0], on_press, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                animated_names: Vec::new(),
                index: 0,
                last_change: ((0) as f32) as f32,
            });
            ((__h.clone() as compat::Shared<dyn StaticDyeButton>), __h)
        });
        for color in colors.iter().copied() {
            let __h0 = (component::literal(&compat::read(&__this, |__t| __t.name.clone()))).with_color(color);
            __this.lock().borrow_mut().animated_names.push(__h0);
        }
        __this
    }

}

impl StaticDyeButton for AnimatedDyeColor {
    fn static_dye_button_base(&self) -> &StaticDyeButtonBase {
        &self.base
    }
    fn static_dye_button_base_mut(&mut self) -> &mut StaticDyeButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractName(GuiGraphicsExtractor graphics, float a) (dispatch: the body for this class)
    fn extract_name(&self) -> fn(&compat::Shared<dyn StaticDyeButton>, &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, f32) {
        AnimatedDyeColor::extract_name_dyn
    }

}

impl AnimatedDyeColor {
    /// java: @Override protected void extractName(GuiGraphicsExtractor graphics, float a)
    pub fn extract_name_impl(__this: &compat::Shared<Self>, graphics: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut a: f32) {
        { let __v = (compat::read(__this, |__t| __t.last_change) + a); compat::write(__this, |__t| __t.last_change = __v); }
        if compat::read(__this, |__t| __t.last_change) > (2 as f32) {
            { let __v = ((0) as f32) as f32; compat::write(__this, |__t| __t.last_change = __v); }
            { let __v = ((compat::read(__this, |__t| __t.index)).wrapping_add(1)).min((__this.lock().borrow().animated_names.len() as i32).wrapping_sub(1)); compat::write(__this, |__t| __t.index = __v); }
            let __r0 = compat::read(__this, |__t| __t.index);
            { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().animated_names[(__r0) as usize].clone()),); __this.lock().borrow_mut().set_message_text(__q0) };
            if compat::read(__this, |__t| __t.index) == (__this.lock().borrow().animated_names.len() as i32).wrapping_sub(1) {
                { let __v = 0; compat::write(__this, |__t| __t.index = __v); }
            }
        }
        <dyn StaticDyeButton>::extract_name_body(&compat::view(__this, |a| a as compat::Shared<dyn StaticDyeButton>), graphics, a);
    }

    /// AnimatedDyeColor.extractName as the dispatch of StaticDyeButton.extractName
    pub fn extract_name_dyn(__this: &compat::Shared<dyn StaticDyeButton>, __p0: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, __p1: f32) {
        Self::extract_name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// java: public interface AnimatedDyeConsumer
/// a functional interface: `accept(Button button, List<CustomArmorAnimatedDyes.Keyframe> keyframes, boolean cycleBack)` - a lambda / method reference is this closure type
pub type AnimatedDyeConsumer = Arc<dyn Fn(AbstractWidget, Vec<Keyframe>, bool)>;

/// StaticDyeButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call - for
/// both classes (`StaticDyeButton`, `AnimatedDyeColor`)
impl compat::WidgetHalf for dyn StaticDyeButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.static_dye_button_base_mut().base
    }
}

/// A dye button as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on. Its press is the
/// widget's `onPress` (`Button.onPress`), which the game runs.
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<dyn StaticDyeButton> {
    /// java: AbstractButton.extractWidgetRenderState - `extractContents(..)`, then `handleCursor(graphics)`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_contents()); __f(__h, widget, context, mouse_x, mouse_y, a) }))
    }

    /// java: Button.Plain's narration - `_ -> Component.empty()`
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {}

    /// java: AbstractButton's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }
}
