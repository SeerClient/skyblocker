//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.HeadSelectionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_animated_helmet_textures::CustomAnimatedHelmetTextures;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_helmet_textures::CustomHelmetTextures;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidgetBase;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener as _;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));

/// java: public class HeadSelectionWidget extends SearchableGridWidget
#[derive(Clone)]
pub struct HeadSelectionWidget {
    /// java: extends SearchableGridWidget
    pub base: SearchableGridWidgetBase,
    /// java: private final List<HeadButton> allButtons = new ArrayList<>();
    pub all_buttons: Vec<compat::Shared<dyn HeadButton>>,
    /// java: private final HeadButton noneButton;
    pub none_button: Option<compat::Shared<dyn HeadButton>>,
    /// java: private @Nullable ItemStack currentItem;
    pub current_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private HeadButton selectedButton;
    pub selected_button: Option<compat::Shared<dyn HeadButton>>,
}

impl std::ops::Deref for HeadSelectionWidget {
    type Target = SearchableGridWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HeadSelectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HeadSelectionWidget {
    /// java: public HeadSelectionWidget(int x, int y, int width, int height)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> compat::Shared<Self> {
        let (_, __this) = SearchableGridWidgetBase::new_7_in(x.wrapping_add(2), y.wrapping_add(2), width.wrapping_sub(4), height.wrapping_sub(4), component::null_to_empty(Some("HeadSelection")), 20, true, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                all_buttons: Vec::new(),
                none_button: None,
                current_item: None,
                selected_button: None,
            });
            ((__h.clone() as compat::Shared<dyn SearchableGridWidget>), __h)
        });
        for tex in CustomHelmetTextures::get_textures() {
            let mut head: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ProfileViewerUtils::create_skull(&tex.texture()));
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
            let mut button: compat::Shared<dyn HeadButton> = (HeadButtonBase::new_4(tex.name(), Some(tex.texture()), head.clone(), Arc::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { HeadSelectionWidget::on_click(&__this_1, __a0.clone()); } })) as compat::Shared<dyn HeadButton>);
            let __h0 = button.clone();
            __this.lock().borrow_mut().all_buttons.push(__h0);
        }
        for id in CustomAnimatedHelmetTextures::get_animated_head_ids() {
            let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
            let mut button: compat::Shared<AnimatedHeadButton> = AnimatedHeadButton::new(id.clone(), Arc::new(move |__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { HeadSelectionWidget::on_click(&__this_2, __a0.clone()); } }));
            let __h0 = (button.clone() as compat::Shared<dyn HeadButton>);
            __this.lock().borrow_mut().all_buttons.push(__h0);
        }
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((HeadButtonBase::new("".to_string(), Arc::new(move |__a0| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { HeadSelectionWidget::on_click(&__this_3, __a0.clone()); } })) as compat::Shared<dyn HeadButton>)); compat::write(&__this, |__t| __t.none_button = __v); }
        { let __v = compat::read(&__this, |__t| __t.none_button.clone()); compat::write(&__this, |__t| __t.selected_button = __v); }
        let __h0 = compat::read(&__this, |__t| __t.none_button.clone()).unwrap();
        __this.lock().borrow_mut().all_buttons.push(__h0);
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_search()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn SearchableGridWidget>), "") });
        __this
    }

    /// java: private void onClick(HeadButton button)
    pub fn on_click(__this: &compat::Shared<Self>, mut button: compat::Shared<dyn HeadButton>) {
        { let __v = Some(button.clone()); compat::write(__this, |__t| __t.selected_button = __v); }
        HeadSelectionWidget::update_config(__this);
        HeadSelectionWidget::update_buttons(__this);
    }

    /// java: private void updateConfig()
    pub fn update_config(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.current_item.clone()).is_none() {
            return;
        }
        let mut uuid: String = __this.lock().borrow().current_item.as_ref().unwrap().get_uuid();
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| {
            if let Some(__sel1) = compat::read(__this_1.get(), |__t| __t.selected_button.clone()).as_ref() && true && (std::ptr::addr_eq(std::sync::Arc::as_ptr(__sel1), std::sync::Arc::as_ptr(&compat::read(__this_1.get(), |__t| __t.none_button.clone()).unwrap())) || compat::read(__sel1, |__t| __t.head_button_base().texture.clone()).is_none()) {
                config.general.custom_helmet_textures.remove(&uuid);
                config.general.custom_animated_helmet_textures.remove(&uuid);
            } else if let Some(__sel3) = compat::read(__this_1.get(), |__t| __t.selected_button.clone()).as_ref() && let Some(__view4) = compat::try_view::<AnimatedHeadButton, _>(__sel3) {
                config.general.custom_animated_helmet_textures.insert(uuid.clone(), compat::read((&*__view4), |__t| __t.id.clone()));
                config.general.custom_helmet_textures.remove(&uuid);
            } else if let Some(__sel5) = compat::read(__this_1.get(), |__t| __t.selected_button.clone()).as_ref() && true {
                config.general.custom_helmet_textures.insert(uuid.clone(), compat::read(__sel5, |__t| __t.head_button_base().texture.clone()).unwrap());
                config.general.custom_animated_helmet_textures.remove(&uuid);
            }
        }));
    }

    /// java: private void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        for b in compat::read(__this, |__t| __t.all_buttons.clone()) {
            { let __v = std::ptr::addr_eq(std::sync::Arc::as_ptr(&b), std::sync::Arc::as_ptr(&compat::read(__this, |__t| __t.selected_button.clone()).unwrap())); compat::write(&b, |__t| __t.head_button_base_mut().selected = __v); }
        }
    }

    /// java: public void setCurrentItem(ItemStack item)
    pub fn set_current_item(__this: &compat::Shared<Self>, mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        { let __v = Some(item.clone()); compat::write(__this, |__t| __t.current_item = __v); }
        let mut uuid: String = item.get_uuid();
        let mut animated_head_id: Option<String> = SkyblockerConfigManager::get().general.custom_animated_helmet_textures.get(&uuid).cloned();
        let mut custom_head_texture: Option<String> = SkyblockerConfigManager::get().general.custom_helmet_textures.get(&uuid).cloned();
        let mut intended_selected: compat::Shared<dyn HeadButton>;
        if let Some(animated_head_id) = &animated_head_id {
            // intendedSelected = this.allButtons.stream().filter(AnimatedHeadButton.class::isInstance).map(AnimatedHeadButton.class::cast)
            //     .filter(animatedHead -> animatedHead.id.equals(animatedHeadId)).findFirst().map(HeadButton.class::cast).orElse(noneButton);
            intended_selected = compat::read(__this, |__t| __t.all_buttons.clone()).into_iter()
                .filter(|head| compat::try_view::<AnimatedHeadButton, _>(head).is_some_and(|animated_head| compat::read(&*animated_head, |__t| __t.id.clone()) == *animated_head_id))
                .next()
                .unwrap_or_else(|| compat::read(__this, |__t| __t.none_button.clone()).unwrap());
        } else {
            if let Some(custom_head_texture) = &custom_head_texture {
                // intendedSelected = this.allButtons.stream().filter(Predicate.not(AnimatedHeadButton.class::isInstance))
                //     .filter(head -> head.texture != null && head.texture.equals(customHeadTexture)).findFirst().orElse(noneButton);
                intended_selected = compat::read(__this, |__t| __t.all_buttons.clone()).into_iter()
                    .filter(|head| compat::try_view::<AnimatedHeadButton, _>(head).is_none())
                    .filter(|head| compat::read(head, |__t| __t.head_button_base().texture.clone()).is_some_and(|texture| texture == *custom_head_texture))
                    .next()
                    .unwrap_or_else(|| compat::read(__this, |__t| __t.none_button.clone()).unwrap());
            } else {
                intended_selected = compat::read(__this, |__t| __t.none_button.clone()).unwrap();
            }
        }
        { let __v = Some(intended_selected.clone()); compat::write(__this, |__t| __t.selected_button = __v); }
        HeadSelectionWidget::update_buttons(__this);
    }

}

impl SearchableGridWidget for HeadSelectionWidget {
    fn searchable_grid_widget_base(&self) -> &SearchableGridWidgetBase {
        &self.base
    }
    fn searchable_grid_widget_base_mut(&mut self) -> &mut SearchableGridWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setX(int x) (dispatch: the body for this class)
    fn set_x(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        HeadSelectionWidget::set_x_dyn
    }

    /// java: @Override public void setY(int y) (dispatch: the body for this class)
    fn set_y(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        HeadSelectionWidget::set_y_dyn
    }

    /// java: @Override public void setWidth(int width) (dispatch: the body for this class)
    fn set_width(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        HeadSelectionWidget::set_width_dyn
    }

    /// java: @Override public void setHeight(int height) (dispatch: the body for this class)
    fn set_height(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        HeadSelectionWidget::set_height_dyn
    }

    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String search) (dispatch: the body for this class)
    fn filter_widgets(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, String) -> Vec<AbstractWidget> {
        HeadSelectionWidget::filter_widgets_dyn
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &GuiGraphicsExtractor, i32, i32, f32) {
        HeadSelectionWidget::extract_widget_render_state_dyn
    }

}

impl HeadSelectionWidget {
    /// java: @Override public void setX(int x)
    pub fn set_x_impl(__this: &compat::Shared<Self>, mut x: i32) {
        <dyn SearchableGridWidget>::set_x_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), x.wrapping_add(2));
    }

    /// HeadSelectionWidget.setX as the dispatch of SearchableGridWidget.setX
    pub fn set_x_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: i32) {
        Self::set_x_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void setY(int y)
    pub fn set_y_impl(__this: &compat::Shared<Self>, mut y: i32) {
        <dyn SearchableGridWidget>::set_y_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), y.wrapping_add(2));
    }

    /// HeadSelectionWidget.setY as the dispatch of SearchableGridWidget.setY
    pub fn set_y_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: i32) {
        Self::set_y_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width_impl(__this: &compat::Shared<Self>, mut width: i32) {
        <dyn SearchableGridWidget>::set_width_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), width.wrapping_sub(4));
    }

    /// HeadSelectionWidget.setWidth as the dispatch of SearchableGridWidget.setWidth
    pub fn set_width_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: i32) {
        Self::set_width_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height_impl(__this: &compat::Shared<Self>, mut height: i32) {
        <dyn SearchableGridWidget>::set_height_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), height.wrapping_sub(4));
    }

    /// HeadSelectionWidget.setHeight as the dispatch of SearchableGridWidget.setHeight
    pub fn set_height_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: i32) {
        Self::set_height_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String search)
    pub fn filter_widgets_impl(__this: &compat::Shared<Self>, mut search: String) -> Vec<AbstractWidget> {
        ({ let __f = compat::dispatch(__this, |__t| __t.set_scroll_amount()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), ((0) as f64) as f64) });
        HeadSelectionWidget::update_buttons(__this);
        let mut s: String = search.to_lowercase();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // (each button handed to the grid as the game's widget over the same button object)
        return compat::read(__this, |__t| __t.all_buttons.clone()).into_iter().filter(|b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; (std::ptr::addr_eq(std::sync::Arc::as_ptr(b), std::sync::Arc::as_ptr(&compat::read((&__this_1), |__t| __t.none_button.clone()).unwrap())) || b.lock().borrow().head_button_base().name.to_lowercase().contains(&s)) }).map(|b| compat::WidgetAdapter::<dyn HeadButton>::widget(b)).collect::<Vec<_>>();
    }

    /// HeadSelectionWidget.filterWidgets as the dispatch of SearchableGridWidget.filterWidgets
    pub fn filter_widgets_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: String) -> Vec<AbstractWidget> {
        Self::filter_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), (__this.lock().borrow().get_x()).wrapping_sub(2), (__this.lock().borrow().get_y()).wrapping_sub(2), (__this.lock().borrow().get_width()).wrapping_add(4), (__this.lock().borrow().get_height()).wrapping_add(4));
        <dyn SearchableGridWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn SearchableGridWidget>), graphics, mouse_x, mouse_y, a);
    }

    /// HeadSelectionWidget.extractWidgetRenderState as the dispatch of SearchableGridWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// java: private static class HeadButton extends AbstractWidget
#[derive(Clone)]
pub struct HeadButtonBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final String name;
    pub name: String,
    /// java: private final @Nullable String texture;
    pub texture: Option<String>,
    /// java: private final ItemStack head;
    pub head: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final Consumer<HeadButton> onPress;
    pub on_press: Option<Arc<dyn Fn(compat::Shared<dyn HeadButton>)>>,
    /// java: private boolean selected = false;
    pub selected: bool,
}

impl std::ops::Deref for HeadButtonBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HeadButtonBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HeadButtonBase {
    /// java: private HeadButton(String name, Consumer<HeadButton> onPress) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut name: String, on_press: Arc<dyn Fn(compat::Shared<dyn HeadButton>)>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HeadButton>, __R)) -> (compat::Shared<dyn HeadButton>, __R) {
        let (__this, __r) = Self::new_4_in(name.clone(), None, FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut()), on_press.clone(), __wrap);
        (__this, __r)
    }

    /// java: private HeadButton(String name, Consumer<HeadButton> onPress)
    pub fn new(mut name: String, on_press: Arc<dyn Fn(compat::Shared<dyn HeadButton>)>) -> compat::Shared<Self> {
        Self::new_in(name, on_press, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn HeadButton>), __h)
        }).1
    }

    /// java: private HeadButton(String name, @Nullable String texture, ItemStack head, Consumer<HeadButton> onPress) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_4_in<__R>(mut name: String, mut texture: Option<String>, mut head: mcrust::net::minecraft::world::item::item_stack::ItemStack, on_press: Arc<dyn Fn(compat::Shared<dyn HeadButton>)>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HeadButton>, __R)) -> (compat::Shared<dyn HeadButton>, __R) {
        let __init_name = name.clone();
        let __init_texture = texture.clone();
        let __init_head = Some(head.clone());
        let __init_on_press = Some(on_press.clone());
        let this = Self {
            base: AbstractWidget::new(0, 0, 20, 20, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            name: __init_name,
            texture: __init_texture,
            head: __init_head,
            on_press: __init_on_press,
            selected: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        if !name.is_empty() {
            { let (__q0,) = (Tooltip::create(component::null_to_empty(Some(&name)).get_string()),); __this.lock().borrow_mut().head_button_base_mut().set_tooltip(__q0) };
        }
        (__this, __r)
    }

    /// java: private HeadButton(String name, @Nullable String texture, ItemStack head, Consumer<HeadButton> onPress)
    pub fn new_4(mut name: String, mut texture: Option<String>, mut head: mcrust::net::minecraft::world::item::item_stack::ItemStack, on_press: Arc<dyn Fn(compat::Shared<dyn HeadButton>)>) -> compat::Shared<Self> {
        Self::new_4_in(name, texture, head, on_press, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn HeadButton>), __h)
        }).1
    }

}

/// java: the virtual methods of HeadButton (implemented by HeadButtonBase and every subclass)
pub trait HeadButton: compat::DynAny {
    fn head_button_base(&self) -> &HeadButtonBase;
    fn head_button_base_mut(&mut self) -> &mut HeadButtonBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected ItemStack getHead() (dispatch: the body for this class)
    fn get_head(&self) -> fn(&compat::Shared<dyn HeadButton>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn HeadButton>::get_head_body
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HeadButton>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn HeadButton>::extract_widget_render_state_body
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn on_click(&self) -> fn(&compat::Shared<dyn HeadButton>, &MouseButtonEvent, bool) {
        <dyn HeadButton>::on_click_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn HeadButton>, &dyn NarrationElementOutput) {
        <dyn HeadButton>::update_widget_narration_body
    }

}

impl dyn HeadButton {
    /// java: protected ItemStack getHead()
    pub fn get_head_body(__this: &compat::Shared<dyn HeadButton>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return compat::read(__this, |__t| __t.head_button_base().head.clone()).unwrap();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn HeadButton>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.item(&({ let __f = compat::dispatch(__this, |__t| __t.get_head()); __f(__this) }), (__this.lock().borrow().head_button_base().get_x()).wrapping_add(2), (__this.lock().borrow().head_button_base().get_y()).wrapping_add(2));
        if compat::read(__this, |__t| __t.head_button_base().selected) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().head_button_base().get_x(), __this.lock().borrow().head_button_base().get_y(), (__this.lock().borrow().head_button_base().get_x()).wrapping_add(__this.lock().borrow().head_button_base().get_width()), (__this.lock().borrow().head_button_base().get_y()).wrapping_add(__this.lock().borrow().head_button_base().get_height()), 0x3000FF00);
        }
        if __this.lock().borrow().head_button_base().is_hovered() {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().head_button_base().get_x(), __this.lock().borrow().head_button_base().get_y(), (__this.lock().borrow().head_button_base().get_x()).wrapping_add(__this.lock().borrow().head_button_base().get_width()), (__this.lock().borrow().head_button_base().get_y()).wrapping_add(__this.lock().borrow().head_button_base().get_height()), 0x20FFFFFF);
        }
        // java: this.handleCursor(graphics); - the port asks every widget for its cursor after the frame
        //   (`ModWidget::handle_cursor` below answers it)
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_body(__this: &compat::Shared<dyn HeadButton>, click: &MouseButtonEvent, mut doubled: bool) {
        ((compat::read(__this, |__t| __t.head_button_base().on_press.clone()).unwrap()))(__this.clone());
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn HeadButton>, builder: &dyn NarrationElementOutput) {
    }

}

impl HeadButton for HeadButtonBase {
    fn head_button_base(&self) -> &HeadButtonBase {
        &self
    }
    fn head_button_base_mut(&mut self) -> &mut HeadButtonBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: private static class AnimatedHeadButton extends HeadButton
#[derive(Clone)]
pub struct AnimatedHeadButton {
    /// java: extends HeadButton
    pub base: HeadButtonBase,
    /// java: private final String id;
    pub id: String,
}

impl std::ops::Deref for AnimatedHeadButton {
    type Target = HeadButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnimatedHeadButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnimatedHeadButton {
    /// java: private AnimatedHeadButton(String id, Consumer<HeadButton> onPress)
    pub fn new(mut id: String, on_press: Arc<dyn Fn(compat::Shared<dyn HeadButton>)>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let (_, __this) = HeadButtonBase::new_in(CustomAnimatedHelmetTextures::format_name(&id), on_press.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                id: __init_id,
            });
            ((__h.clone() as compat::Shared<dyn HeadButton>), __h)
        });
        __this
    }

}

impl HeadButton for AnimatedHeadButton {
    fn head_button_base(&self) -> &HeadButtonBase {
        &self.base
    }
    fn head_button_base_mut(&mut self) -> &mut HeadButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected ItemStack getHead() (dispatch: the body for this class)
    fn get_head(&self) -> fn(&compat::Shared<dyn HeadButton>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        AnimatedHeadButton::get_head_dyn
    }

}

impl AnimatedHeadButton {
    /// java: @Override protected ItemStack getHead()
    pub fn get_head_impl(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        let mut profile: Option<ResolvableProfile> = CustomAnimatedHelmetTextures::animate_head_texture(&compat::read(__this, |__t| __t.id.clone()));
        if let Some(profile) = &profile {
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("player_head"));
            stack.set(mcrust::net::minecraft::core::component::data_components::profile().clone(), Some(mcrust::com::mojang::serialization::obj(profile.clone())));
            return stack;
        }
        return FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut());
    }

    /// AnimatedHeadButton.getHead as the dispatch of HeadButton.getHead
    pub fn get_head_dyn(__this: &compat::Shared<dyn HeadButton>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        Self::get_head_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// HeadButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call - for both
/// classes (`HeadButton`, `AnimatedHeadButton`), as Java's `filteredWidgets` hold either
impl compat::WidgetHalf for dyn HeadButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.head_button_base_mut().base
    }
}

/// A HeadButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<dyn HeadButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(__h, &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(__h, output) }))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_click()); __f(__h, event, double_click) }))
    }

    /// `this.handleCursor(graphics)` from `extractWidgetRenderState` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        widget.handle_cursor(cursors)
    }
}

/// HeadSelectionWidget as the game's widget (`layout.addChild(headSelectionWidget, ..)`): Java's `visible` is the
/// object's own (the armor tab sets it), the box the game's layouts move reaches the object through its `setX` /
/// `setY` overrides.
pub struct HeadSelectionWidgetSlot {
    object: compat::RenderThread<compat::Shared<HeadSelectionWidget>>,
    /// the game's x and y as of the last call
    snap: std::cell::Cell<(i32, i32)>,
}

impl HeadSelectionWidget {
    /// The game's widget over this object. Java's `getX()` is the layout's x plus 2 (`setX` override), so the layout
    /// cell starts 2 before it.
    pub fn widget(this: &compat::Shared<Self>) -> AbstractWidget {
        let (x, y, w, h) = compat::read(this, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
        let slot = HeadSelectionWidgetSlot { object: compat::RenderThread::new(this.clone()), snap: std::cell::Cell::new((x - 2, y - 2)) };
        mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x - 2, y - 2, w, h, "HeadSelection", Box::new(slot))
    }
}

impl HeadSelectionWidgetSlot {
    /// what the game changed (a layout moving the widget) through the object's `setX` / `setY`; the pass's hover
    fn sync(&self, widget: &AbstractWidget) {
        let object = self.object.get();
        let (sx, sy) = self.snap.get();
        if widget.get_x() != sx {
            HeadSelectionWidget::set_x_impl(object, widget.get_x());
        }
        if widget.get_y() != sy {
            HeadSelectionWidget::set_y_impl(object, widget.get_y());
        }
        self.snap.set((widget.get_x(), widget.get_y()));
        let hovered = widget.is_hovered();
        compat::write(object, |__t| __t.base.base.is_hovered = hovered);
    }

    fn visible(&self) -> bool {
        compat::read(self.object.get(), |__t| __t.base.base.visible)
    }
}

impl ModWidget for HeadSelectionWidgetSlot {
    /// java: AbstractWidget.extractRenderState - `if (visible)`, then `extractWidgetRenderState`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.sync(widget);
        if self.visible() {
            HeadSelectionWidget::extract_widget_render_state_impl(self.object.get(), &context.graphics, mouse_x, mouse_y, a);
        }
    }

    /// java: SearchableGridWidget.updateWidgetNarration - empty
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, _output: &mut dyn NarrationElementOutput) {}

    /// java: AbstractContainerWidget.mouseClicked - `ContainerEventHandler.super.mouseClicked(..)` over `children()`
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.sync(widget);
        if !self.visible() {
            return false;
        }
        let object = self.object.get();
        let handle = compat::view(object, |a| a as compat::Shared<dyn SearchableGridWidget>);
        let mut children = ({ let __f = compat::dispatch(&*handle, |__t| __t.children()); __f(&*handle) });
        for child in children.iter_mut() {
            if child.mouse_clicked(event.button(), event.x(), event.y(), double_click) {
                return true;
            }
        }
        false
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&mut self, widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.sync(widget);
        if !self.visible() {
            return false;
        }
        let object = self.object.get();
        let handle = compat::view(object, |a| a as compat::Shared<dyn SearchableGridWidget>);
        ({ let __f = compat::dispatch(&*handle, |__t| __t.mouse_scrolled()); __f(&*handle, x, y, scroll_x, scroll_y) })
    }

    /// java: AbstractWidget.isMouseOver - `isActive() && ..` over the object's box (its `visible`)
    fn is_mouse_over(&self, _widget: &AbstractWidget, mouse_x: f64, mouse_y: f64) -> bool {
        compat::read(self.object.get(), |__t| __t.base.base.is_mouse_over(mouse_x, mouse_y))
    }
}
