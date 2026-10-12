//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.SidePanelWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::copy_to_popup::CopyToPopup;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::position_rule_widget::PositionRuleWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::cycle_button::CycleButton;
use mcrust::net::minecraft::client::gui::components::events::container_event_handler;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("menu_outer_space");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_outer_space"));
/// java: private static final int TOP_MARGIN = 16;
pub const TOP_MARGIN: i32 = 16;
/// java: private static final int SCROLLBAR_AREA = SCROLLBAR_WIDTH + 1;
pub static SCROLLBAR_AREA: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH.wrapping_add(1));

/// java: class SidePanelWidget extends AbstractContainerWidget
pub struct SidePanelWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> optionWidgets = new ArrayList<>();
    pub option_widgets: Vec<AbstractWidget>,
    /// java: private LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
    /// java: boolean rightSide = false;
    pub right_side: bool,
    /// java: private int targetX = 0;
    pub target_x: i32,
    /// java: private float animation = 0.0f;
    pub animation: f32,
    /// java: private int animationStart = 0;
    pub animation_start: i32,
    /// java: private int animationEnd = 0;
    pub animation_end: i32,
    /// java: private boolean isOpen = false;
    pub is_open: bool,
    /// java: private final WidgetsConfigurationScreen configScreen;
    pub config_screen: Option<compat::Shared<WidgetsConfigurationScreen>>,
    /// java: private @Nullable PositionedWidget positionedWidget;
    pub positioned_widget: Option<compat::Shared<PositionedWidget>>,
}

impl std::ops::Deref for SidePanelWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SidePanelWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SidePanelWidget {
    /// java: SidePanelWidget(int width, int height, WidgetsConfigurationScreen configScreen)
    pub fn new(mut width: i32, mut height: i32, mut config_screen: compat::Shared<WidgetsConfigurationScreen>) -> Self {
        // java: super(0, TOP_MARGIN, width, height - TOP_MARGIN, Component.literal("Side Panel"), defaultSettings(5));
        let mut this: Self = Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, TOP_MARGIN, width, height.wrapping_sub(TOP_MARGIN), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Side Panel")), mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(5.0)),
            option_widgets: Vec::new(),
            layout: LinearLayout::vertical(),
            right_side: false,
            target_x: 0,
            animation: 0.0f32,
            animation_start: 0,
            animation_end: 0,
            is_open: false,
            config_screen: None,
            positioned_widget: None,
        };
        this.visible = false;
        this.config_screen = Some(config_screen.clone());
        this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        // rust: port gap (GAPS_batchA #9) - `children()` lending the live child widgets: the port answers owned
        // listeners, and these children are this object's own values
        // rust: return self.option_widgets.clone();
        return Vec::new();
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.layout.get_height();
    }

    /// java: private boolean isNotVisible(int top, int bottom)
    pub fn is_not_visible(&self, mut top: i32, mut bottom: i32) -> bool {
        return !(bottom >= self.get_y() && top <= self.get_bottom());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&mut self, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        if self.animation >= (0 as f32) {
            if self.animation < 1.0f32 {
                { let __s0 = self.animation_start.wrapping_add((((self.animation_end.wrapping_sub(self.animation_start) as f32) * self.animation)) as i32); self.set_x(__s0) };
                // TODO(fabric2seer): `DeltaTracker.getRealtimeDeltaTicks` has no Rust method in the port (looked for `get_realtime_delta_ticks` in DeltaTracker and its supertypes)
                // rust: self.animation = (self.animation + (((compat::minecraft().get_delta_tracker().get_realtime_delta_ticks() * (50 as f32)) * 7.5f32) / 1000.0f32));
            } else {
                { let __s0 = self.animation_end; self.set_x(__s0) };
                self.animation = -1.0f32;
            }
        }
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), self.get_x().wrapping_sub(4), (self.get_y().wrapping_sub(4)).wrapping_sub(TOP_MARGIN), self.get_width().wrapping_add(8), (self.get_height().wrapping_add(8)).wrapping_add(TOP_MARGIN));
        context.enable_scissor(self.get_x(), self.get_y(), self.get_right(), self.get_bottom());
        for clickable_widget in self.option_widgets.iter() {
            if self.is_not_visible(clickable_widget.get_y(), clickable_widget.get_bottom()) {
                continue;
            }
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: clickable_widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
        context.disable_scissor();
        // TODO(fabric2seer): `AbstractContainerWidget.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in AbstractContainerWidget and its supertypes)
        // rust: self.extract_scrollbar(context, mouse_x, mouse_y);
    }

    /// java: public void open()
    pub fn open(&mut self) {
        if self.is_open() {
            return;
        }
        self.visible = true;
        self.animation = 0.0f32;
        self.animation_end = self.target_x;
        self.animation_start = self.target_x.wrapping_add(if self.right_side { self.get_width().wrapping_add(8) } else { (-self.get_width()).wrapping_sub(8) });
        self.is_open = true;
    }

    /// java: public void open(PositionedWidget positionedWidget, boolean rightSide)
    pub fn open_2(&mut self, mut positioned_widget: compat::Shared<PositionedWidget>, mut right_side: bool) {
        self.positioned_widget = Some(positioned_widget.clone());
        let mut hud_widget: compat::Shared<dyn HudWidget> = compat::read(&positioned_widget, |__t| __t.widget.clone());
        self.layout = LinearLayout::vertical().spacing(5);
        self.layout.default_cell_setting().align_horizontally_center();
        self.option_widgets.clear();
        // TODO(fabric2seer): `new StringWidget(..) {..}` (an anonymous class with state or several methods)
        //   java: add(new StringWidget(0, 15, hudWidget.getInformation().displayName().copy().withStyle(ChatFormatting.UNDERLINE), client.font) {
        //   java: 			@Override
        //   java: 			public void setWidth(int width) {
        //   java: 				setMaxWidth(width, TextOverflow.SCROLLING);
        //   java: 			}
        //   java: 		});
        if !(compat::read(&positioned_widget, |__t| __t.from_tab)) {
            // java: add(Button.builder(.., _ -> configScreen.removeWidget(positionedWidget)).build());
            // (the screen holds this panel: the action holds the screen weakly)
            let __cs = compat::RenderThread::new(std::sync::Arc::downgrade(self.config_screen.as_ref().unwrap()));
            let __pw = compat::RenderThread::new(positioned_widget.clone());
            let __s0 = (button::builder(component::translatable("skyblocker.config.hud.widget.remove").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
                let Some(__config_screen) = __cs.get().upgrade() else { return; };
                WidgetsConfigurationScreen::remove_widget(&__config_screen, __pw.get());
            })))).build();
            self.add(__s0);
        }
        // java: if (WidgetManager.ALLOWED_LOCATIONS.stream().noneMatch(l -> hudWidget.getInformation().available().test(l) && l != configScreen.getCurrentLocation()))
        // (read before the button is added: it reads neither the button nor the list)
        let __available = ({ let __f = compat::dispatch(&hud_widget, |__t| __t.get_information()); __f(&hud_widget) }).available();
        let __current_location = WidgetsConfigurationScreen::get_current_location(self.config_screen.as_ref().unwrap());
        let __none_match: bool = !widget_manager::ALLOWED_LOCATIONS.iter().copied().any(|l| (__available)(l) && l != __current_location);
        // java: Button copyButton = add(Button.builder(.., _ -> configScreen.openPopup(screen -> new CopyToPopup(screen, positionedWidget, ..))).tooltip(..).build());
        let __s0 = (button::builder(component::translatable("skyblocker.config.hud.copy.copyTo").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            // rust: port gap (GAPS_batchC #C4) - `new CopyToPopup(screen, ..)`: the popup's `AbstractPopupScreen` takes its
            // background as a game `Screen` value (GAPS_batchA #19), which a mod screen object is not
            // rust: WidgetsConfigurationScreen::open_popup(self.config_screen.as_ref().unwrap(), Arc::new(move |screen| CopyToPopup::new(screen.clone(), positioned_widget.clone(), WidgetsConfigurationScreen::get_current_location(&screen), WidgetsConfigurationScreen::get_current_screen_layer(&screen))));
        })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.hud.copy.copyTo.@Tooltip")).get_string())).message.clone())).build();
        let copy_button: &mut AbstractWidget = self.add(__s0);
        if __none_match {
            copy_button.active = false;
            copy_button.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.hud.copy.copyTo.disabled.@Tooltip")).get_string()));
        }
        self.layout.add_child(SpacerElement::height(10).as_layout_element());
        let mut available_width: i32 = self.get_width().wrapping_sub(*SCROLLBAR_AREA);
        if !(compat::read(&positioned_widget, |__t| __t.from_tab)) {
            // rust: port gap (GAPS_batchC #L3) - `PositionRuleWidget` is a mod composite widget (`AbstractContainerWidget`):
            // it is not a game widget, so it cannot join this panel's widget list or its layout
            // rust: { let __s0 = PositionRuleWidget::new(self.config_screen.clone().unwrap(), positioned_widget.clone()); self.add(__s0) };
            self.layout.add_child(SpacerElement::height(10).as_layout_element());
        }
        // java: List<AbstractWidget> collector = new ArrayList<>(); new OptionWidgetCollector(collector, ..) - the collector
        // fills Java's list object: the port's collector owns it (`collector_list`), read back below
        let mut collector: Vec<AbstractWidget> = Vec::new();
        let __preview_widget = hud_widget.clone();
        let mut widget_collector: OptionWidgetCollector = OptionWidgetCollector::new(collector, Arc::new(move || { ({ let __f = compat::dispatch(&__preview_widget, |__t| __t.update_config_preview()); __f(&__preview_widget) }); }), self.config_screen.clone().unwrap(), WidgetsConfigurationScreen::get_current_location(self.config_screen.as_ref().unwrap()));
        ({ let __f = compat::dispatch(&hud_widget, |__t| __t.get_option_widgets()); __f(&hud_widget, &mut widget_collector) });
        widget_collector.build_deferred();
        // java: collector.forEach(this::add);
        for __a0 in std::mem::take(&mut widget_collector.collector_list) {
            self.add(__a0);
        }
        self.layout.add_child(SpacerElement::height(10).as_layout_element());
        for widget in self.option_widgets.iter_mut() {
            widget.set_width(available_width);
        }
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: { let __s0 = (if right_side { self.get_x() } else { self.get_x().wrapping_add(*SCROLLBAR_AREA) }); let __s1 = self.get_y().wrapping_sub((self.scroll_amount()) as i32); self.layout.set_position(__s0, __s1) };
        self.layout.arrange_elements();
        let mut open_x: i32 = (if right_side { (compat::read(self.config_screen.as_ref().unwrap(), |__t| __t.width)).wrapping_sub(self.get_width()) } else { 0 });
        // java: if (isOpen() && (openX != targetX || rightSide != this.rightSide))
        if self.is_open() && (open_x != self.target_x || right_side != self.right_side) {
            self.is_open = false;
        }
        self.right_side = right_side;
        self.target_x = open_x;
        self.open();
    }

    /// java: public void close()
    pub fn close(&mut self) {
        if !self.is_open() {
            return;
        }
        self.animation_start = self.get_x();
        self.animation_end = self.get_x().wrapping_add(if self.right_side { self.get_width().wrapping_add(8) } else { (-self.get_width()).wrapping_sub(8) });
        self.animation = ((0) as f32) as f32;
        self.is_open = false;
    }

    /// java: public boolean isOpen()
    pub fn is_open(&self) -> bool {
        return self.is_open;
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&mut self, mut scroll_y: f64) {
        // TODO(fabric2seer): `set_scroll_amount` takes 2 argument(s) in the port (`content_height: i32, scroll_amount: f64`), Java passes 1
        // TODO(fabric2seer): `ScrollArea::set_scroll_amount` ports `setScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: self.base.set_scroll_amount(scroll_y);
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: { let __s0 = self.get_y().wrapping_sub((self.scroll_amount()) as i32); self.layout.set_y(__s0) };
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        if self.right_side {
            self.layout.set_x(x);
        } else {
            self.layout.set_x(x.wrapping_add(*SCROLLBAR_AREA));
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // rust: port gap (GAPS_batchA #9) - `getChildAt` over `children()`, which cannot lend this panel's live widgets
        // rust: if container_event_handler::get_child_at(&mouse_x, mouse_y).filter(|child| !(child.as_any().is::<CycleButton>())).filter(|element| element.mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount)).is_some() {
        // rust:     return true;
        // rust: }
        // TODO(fabric2seer): `mouse_scrolled` takes 2 argument(s) in the port (`content_height: i32, scroll_y: f64`), Java passes 4
        // TODO(fabric2seer): `ScrollArea::mouse_scrolled` ports `mouseScrolled`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: return self.base.mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        let __width = self.get_width().wrapping_sub(*SCROLLBAR_AREA);
        for widget in self.option_widgets.iter_mut() {
            widget.set_width(__width);
        }
        self.layout.arrange_elements();
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height(&mut self, mut height: i32) {
        self.base.set_height(height.wrapping_sub(TOP_MARGIN));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(&self, output: &dyn NarrationElementOutput) {
    }

    /// java: public @Nullable PositionedWidget getPositionedWidget()
    pub fn get_positioned_widget(&self) -> Option<compat::Shared<PositionedWidget>> {
        return self.positioned_widget.clone();
    }

    /// java: private <T extends AbstractWidget> T add(T widget)
    /// (the list holds the widget itself: the answer is the held one, as Java's `add` returns its argument)
    pub fn add(&mut self, widget: AbstractWidget) -> &mut AbstractWidget {
        self.option_widgets.push(widget);
        // rust: port gap (GAPS_batchC #L3) - `layout.addChild(widget)`: the port's layouts hold a widget by its index in
        // a screen's widget list, and this panel's widgets are its own values
        // rust: return self.layout.add_child(widget.clone());
        return self.option_widgets.last_mut().unwrap();
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(&self) -> i32 {
        // TODO(fabric2seer): `AbstractContainerWidget.scrollBarX` has no Rust method in the port (looked for `scroll_bar_x` in AbstractContainerWidget and its supertypes)
        // rust: return (if self.right_side { self.base.scroll_bar_x() } else { self.get_x() });
        return todo!("fabric2seer: see the TODO above");
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseScrolled, scrollBarX, setHeight, setScrollAmount, setWidth, setX, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
