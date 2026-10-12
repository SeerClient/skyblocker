//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.RangedValueWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::FinderSettingsContainer;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::OptionWidget;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetAdapter;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetHalf;

use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::serverbound_sign_update_packet::ServerboundSignUpdatePacket;
use mcrust::net::minecraft::world::level::block::entity::sign_block_entity::SignBlockEntity;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class RangedValueWidget extends AbstractContainerWidget
pub struct RangedValueWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final PartyFinderScreen screen;
    pub screen: compat::Shared<PartyFinderScreen>,
    /// java: private final int slotId;
    pub slot_id: i32,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private int minSlotId = -1;
    pub min_slot_id: i32,
    /// java: private int maxSlotId = -1;
    pub max_slot_id: i32,
    /// java: private int backSlotId = -1;
    pub back_slot_id: i32,
    /// java: private int min = -1;
    pub min: i32,
    /// java: private int max = -1;
    pub max: i32,
    /// java: private State state = State.CLOSED;
    pub state: State,
    /// java: private final ModifiedTextFieldWidget input;
    pub input: ModifiedTextFieldWidget,
    /// java: private final Button okButton;
    pub ok_button: Option<AbstractWidget>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for RangedValueWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RangedValueWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RangedValueWidget {
    /// java: public RangedValueWidget(PartyFinderScreen screen, Component name, int x, int y, int width, int slotId)
    pub fn new(mut screen: compat::Shared<PartyFinderScreen>, mut name: MutableComponent, mut x: i32, mut y: i32, mut width: i32, mut slot_id: i32) -> compat::Shared<Self> {
        let __init_slot_id = slot_id;
        let __init_screen = screen.clone();
        let __init_name = name.clone();
        let __init_input = ModifiedTextFieldWidget::new(compat::font(), x, y.wrapping_add(25), width.wrapping_sub(15), 15, component::empty());
        // java: super(x, y, width, 45, Component.empty(), AbstractScrollArea.defaultSettings(4));
        let __this: compat::Shared<Self> = compat::shared(Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, 45, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty()), mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(4.0)),
            screen: __init_screen,
            slot_id: __init_slot_id,
            name: __init_name,
            min_slot_id: -1,
            max_slot_id: -1,
            back_slot_id: -1,
            min: -1,
            max: -1,
            state: State::Closed,
            input: __init_input,
            ok_button: None,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `EditBox.setVisible` has no Rust method in the port (looked for `set_visible` in EditBox and its supertypes)
        // rust: __this.lock().borrow().input.set_visible(false);
        __this.lock().borrow_mut().input.set_max_length(3);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { RangedValueWidget::update_confirm_button(&__this_1, __a0); } })),); __this.lock().borrow_mut().input.set_responder(__q0) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((button::builder((component::literal("✔")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { RangedValueWidget::send_packet(&__this_2); } })))).bounds((x.wrapping_add(width)).wrapping_sub(15), y.wrapping_add(25), 15, 15).build()); compat::write(&__this, |__t| __t.ok_button = __v); }
        __this.lock().borrow_mut().ok_button.as_mut().unwrap().visible = false;
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<RangedValueChild> {
        // java: return List.of(input, okButton); - references to the two child fields, not copies
        return vec![RangedValueChild::Input, RangedValueChild::OkButton];
    }

    /// java: public void updateConfirmButton(String string)
    pub fn update_confirm_button(__this: &compat::Shared<Self>, string: &str) {
        // java: try { int i = Integer.parseInt(string.trim()); .. } catch (NumberFormatException _) { .. } - the parse's
        // `Err` is the catch block
        let __parsed: Result<i32, std::num::ParseIntError> = compat::trim(&string).parse::<i32>();
        let __result: Result<(), ()> = __parsed.map(|i| {
            if i < 0 || i > 999 {
                __this.lock().borrow_mut().ok_button.as_mut().unwrap().active = false;
                __this.lock().borrow_mut().input.set_good(false);
            } else if compat::read(__this, |__t| __t.state) == State::ModifyingMin && i > compat::read(__this, |__t| __t.max) {
                __this.lock().borrow_mut().ok_button.as_mut().unwrap().active = false;
                __this.lock().borrow_mut().input.set_good(false);
            } else {
                let mut active1: bool = (compat::read(__this, |__t| __t.state) != State::ModifyingMax || i >= compat::read(__this, |__t| __t.min));
                __this.lock().borrow_mut().ok_button.as_mut().unwrap().active = active1;
                __this.lock().borrow_mut().input.set_good(active1);
            }
        }).map_err(|_| ());
        if let Err(_) = __result {
            __this.lock().borrow_mut().ok_button.as_mut().unwrap().active = false;
            __this.lock().borrow_mut().input.set_good(false);
        }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.name.clone())), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), 0xFFD0D0D0u32 as i32, false);
        let mut text_offset: i32 = 10;
        if !(compat::read(__this, |__t| __t.visible)) {
            return;
        }
        let mut text_renderer: Font = compat::font();
        if compat::get_state(&party_finder_screen::DEBUG) {
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.slot_id).to_string()), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_sub(10), 0xFFFF0000u32 as i32, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.min_slot_id).to_string()), (__this.lock().borrow().get_x()).wrapping_add(20), (__this.lock().borrow().get_y()).wrapping_sub(10), 0xFFFF0000u32 as i32, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.max_slot_id).to_string()), (__this.lock().borrow().get_x()).wrapping_add(40), (__this.lock().borrow().get_y()).wrapping_sub(10), 0xFFFF0000u32 as i32, true);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::read(__this, |__t| __t.back_slot_id).to_string()), (__this.lock().borrow().get_x()).wrapping_add(60), (__this.lock().borrow().get_y()).wrapping_sub(10), 0xFFFF0000u32 as i32, true);
        }
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().input.extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().ok_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        if compat::read(__this, |__t| __t.state) == State::Closed {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_add(text_offset), (__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width)), ((__this.lock().borrow().get_y()).wrapping_add(15)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("{} - {}", compat::read(__this, |__t| __t.min), compat::read(__this, |__t| __t.max))), (__this.lock().borrow().get_x()).wrapping_add(3), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32, false);
        } else {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_add(text_offset), (__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width)), ((__this.lock().borrow().get_y()).wrapping_add(15)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "-", (__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32)), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            let mut selected_color: i32 = 0xFFFFFF00u32 as i32;
            let mut unselected_color: i32 = 0xFFD0D0D0u32 as i32;
            let mut mouse_over_min: bool = RangedValueWidget::mouse_over_min_button(__this, mouse_x, mouse_y);
            let mut mouse_over_max: bool = RangedValueWidget::mouse_over_max_button(__this, mouse_x, mouse_y);
            let mut min_start_x: i32 = (__this.lock().borrow().get_x()).wrapping_add(1);
            let mut min_end_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_sub(6);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), min_start_x, ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), min_end_x, ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), (if compat::read(__this, |__t| __t.state) == State::ModifyingMin { selected_color } else { (if mouse_over_min { 0xFFFFFFFFu32 as i32 } else { unselected_color }) }));
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), min_start_x.wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(2)).wrapping_add(text_offset), min_end_x.wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(13)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.min).to_string(), (min_start_x.wrapping_add(min_end_x)).wrapping_shr((1) as u32), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            let mut max_start_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_add(5);
            let mut max_end_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), max_start_x, ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), max_end_x, ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), (if compat::read(__this, |__t| __t.state) == State::ModifyingMax { selected_color } else { (if mouse_over_max { 0xFFFFFFFFu32 as i32 } else { unselected_color }) }));
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), max_start_x.wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(2)).wrapping_add(text_offset), max_end_x.wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(13)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.max).to_string(), (max_start_x.wrapping_add(max_end_x)).wrapping_shr((1) as u32), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
        }
    }

    /// java: private boolean mouseOverMinButton(int mouseX, int mouseY)
    pub fn mouse_over_min_button(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        return ({ let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } && mouse_x < ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_sub(5) && mouse_y < (__this.lock().borrow().get_y()).wrapping_add(25) && mouse_y > (__this.lock().borrow().get_y()).wrapping_add(10));
    }

    /// java: private boolean mouseOverMaxButton(int mouseX, int mouseY)
    pub fn mouse_over_max_button(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        return ({ let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } && mouse_x > ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_add(5) && mouse_y < (__this.lock().borrow().get_y()).wrapping_add(25) && mouse_y > (__this.lock().borrow().get_y()).wrapping_add(10));
    }

    /// java: public void setState(State state)
    pub fn set_state(__this: &compat::Shared<Self>, mut state: State) {
        { let __v = state; compat::write(__this, |__t| __t.state = __v); }
        match state {
            State::Closed | State::Open => {
                // TODO(fabric2seer): `EditBox.setVisible` has no Rust method in the port (looked for `set_visible` in EditBox and its supertypes)
                // rust: __this.lock().borrow().input.set_visible(false);
                // TODO(fabric2seer): `set_focused_at` takes 2 argument(s) in the port (`focused: bool, millis: i64`), Java passes 1
                // rust: __this.lock().borrow_mut().input.set_focused_at(false);
                __this.lock().borrow_mut().ok_button.as_mut().unwrap().visible = false;
                // TODO(fabric2seer): `set_focused` takes 2 argument(s) in the port (`focused: bool, last_input_type: InputType`), Java passes 1
                // rust: __this.lock().borrow_mut().ok_button.as_mut().unwrap().set_focused(false);
            }
            State::ModifyingMin | State::ModifyingMax => {
                // TODO(fabric2seer): `EditBox.setVisible` has no Rust method in the port (looked for `set_visible` in EditBox and its supertypes)
                // rust: __this.lock().borrow().input.set_visible(true);
                // TODO(fabric2seer): `move_cursor_to` takes 3 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32, extend_selection: bool`), Java passes 2
                // rust: __this.lock().borrow_mut().input.move_cursor_to(&0, false);
                // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
                // rust: let __r0 = compat::read(__this, |__t| __t.min);
                // rust: let __r1 = compat::read(__this, |__t| __t.max);
                // rust: { let (__q0,) = (&(if state == State::ModifyingMin { __r0 } else { __r1 }).to_string(),); __this.lock().borrow_mut().input.set_value(__q0) };
                // TODO(fabric2seer): `set_focused_at` takes 2 argument(s) in the port (`focused: bool, millis: i64`), Java passes 1
                // rust: __this.lock().borrow_mut().input.set_focused_at(true);
                __this.lock().borrow_mut().ok_button.as_mut().unwrap().visible = true;
            }
            _ => {}
        }
    }

    /// java: public void setStateAndSlots(State state, int minSlotId, int maxSlotId, int backSlotId)
    pub fn set_state_and_slots(__this: &compat::Shared<Self>, mut state: State, mut min_slot_id: i32, mut max_slot_id: i32, mut back_slot_id: i32) {
        RangedValueWidget::set_state(__this, state);
        { let __v = min_slot_id; compat::write(__this, |__t| __t.min_slot_id = __v); }
        { let __v = max_slot_id; compat::write(__this, |__t| __t.max_slot_id = __v); }
        { let __v = back_slot_id; compat::write(__this, |__t| __t.back_slot_id = __v); }
    }

    /// java: public void setMinAndMax(int min, int max)
    pub fn set_min_and_max(__this: &compat::Shared<Self>, mut min: i32, mut max: i32) {
        { let __v = min; compat::write(__this, |__t| __t.min = __v); }
        { let __v = max; compat::write(__this, |__t| __t.max = __v); }
    }

    /// java: private void sendPacket()
    pub fn send_packet(__this: &compat::Shared<Self>) {
        // `sign` is the level's block entity object (a `SignBlockEntity`), as the screen keeps it
        let mut sign: Option<std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>> = PartyFinderScreen::get_sign(&compat::read(__this, |__t| __t.screen.clone()));
        let mut input_trimmed: String = compat::trim(&__this.lock().borrow().input.get_value());
        if compat::read(__this, |__t| __t.state) == State::ModifyingMin {
            // java: try { min = Integer.parseInt(inputTrimmed); } catch (NumberFormatException _) {}
            if let Ok(__v) = input_trimmed.parse::<i32>() {
                compat::write(__this, |__t| __t.min = __v);
            }
        } else if compat::read(__this, |__t| __t.state) == State::ModifyingMax {
            // java: try { max = Integer.parseInt(inputTrimmed); } catch (NumberFormatException _) {}
            if let Ok(__v) = input_trimmed.parse::<i32>() {
                compat::write(__this, |__t| __t.max = __v);
            }
        }
        if let Some(sign) = &sign {
            let sign: &SignBlockEntity = sign.as_sign().expect("ClassCastException: SignBlockEntity");
            let messages = (sign.get_text(PartyFinderScreen::is_sign_front(&compat::read(__this, |__t| __t.screen.clone())))).get_messages((PartyFinderScreen::get_client(&compat::read(__this, |__t| __t.screen.clone()))).is_text_filtering_enabled());
            // TODO(fabric2seer): `ClientPacketListener.send` has no Rust method in the port (looked for `send` in ClientPacketListener and its supertypes)
            // rust: mcrust::mcrust::modding::game::with_player(|player| { player.connection.as_ref().unwrap().send(&ServerboundSignUpdatePacket::new(sign.get_block_pos(), PartyFinderScreen::is_sign_front(&compat::read(__this, |__t| __t.screen.clone())), input_trimmed.clone(), (messages[(1) as usize]).get_string(), (messages[(2) as usize]).get_string(), (messages[(3) as usize]).get_string())); });
        }
        PartyFinderScreen::closed_sign(&compat::read(__this, |__t| __t.screen.clone()));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if PartyFinderScreen::is_waiting_for_server(&compat::read(__this, |__t| __t.screen.clone())) || !(FinderSettingsContainer::can_interact(&PartyFinderScreen::get_settings_container(&compat::read(__this, |__t| __t.screen.clone())), Some(&OptionWidget::Ranged(__this.clone())))) {
            return false;
        }
        if !(compat::read(__this, |__t| __t.visible)) {
            return false;
        }
        if !({ let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().is_mouse_over(__q0, __q1) }) {
            if compat::read(__this, |__t| __t.state) == State::Open && compat::read(__this, |__t| __t.back_slot_id) != -1 {
                PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.screen.clone()), compat::read(__this, |__t| __t.back_slot_id));
                return true;
            } else if compat::read(__this, |__t| __t.state) == State::ModifyingMin || compat::read(__this, |__t| __t.state) == State::ModifyingMax {
                if !(__this.lock().borrow().input.is_good) {
                    // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
                    // rust: let __r0 = compat::read(__this, |__t| __t.state);
                    // rust: let __r1 = compat::read(__this, |__t| __t.min);
                    // rust: let __r2 = compat::read(__this, |__t| __t.max);
                    // rust: { let (__q0,) = (&(if __r0 == State::ModifyingMin { __r1 } else { __r2 }).to_string(),); __this.lock().borrow_mut().input.set_value(__q0) };
                }
                RangedValueWidget::send_packet(__this);
                return true;
            } else {
                return false;
            }
        }
        match compat::read(__this, |__t| __t.state) {
            State::Closed => {
                if click.y() > ((__this.lock().borrow().get_y()).wrapping_add(25) as f64) {
                    return false;
                }
                PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.screen.clone()), compat::read(__this, |__t| __t.slot_id));
                return true;
            }
            State::Open => {
                if RangedValueWidget::mouse_over_min_button(__this, ((click.x()) as i32), ((click.y()) as i32)) {
                    if compat::read(__this, |__t| __t.min_slot_id) == -1 {
                        return false;
                    }
                    PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.screen.clone()), compat::read(__this, |__t| __t.min_slot_id));
                } else if RangedValueWidget::mouse_over_max_button(__this, ((click.x()) as i32), ((click.y()) as i32)) {
                    if compat::read(__this, |__t| __t.max_slot_id) == -1 {
                        return false;
                    }
                    PartyFinderScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.screen.clone()), compat::read(__this, |__t| __t.max_slot_id));
                } else {
                    return !(click.y() > ((__this.lock().borrow().get_y()).wrapping_add(25) as f64));
                }
                return true;
            }
            _ => {
                // java: return super.mouseClicked(click, doubled); - AbstractContainerWidget.mouseClicked:
                // `updateScrolling(event)` (false: contentHeight() is 0, so no scrollbar) `||`
                // ContainerEventHandler.mouseClicked over children() (the input and the OK button)
                // rust: port gap - the two children are an EditBox and a Button this object holds outside any screen,
                // and the port's EditBox / AbstractWidget take a click only through a screen's widget pass, so no
                // child takes it here (GAPS_batchF.md)
                let scrolling: bool = false;
                return scrolling;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        // TODO(fabric2seer): `set_x` takes 2 argument(s) in the port (`x: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let (__q0,) = (__this.lock().borrow().get_x(),); __this.lock().borrow_mut().input.set_x(__q0) };
        { let (__q0,) = (((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(15),); __this.lock().borrow_mut().ok_button.as_mut().unwrap().set_x(__q0) };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        // TODO(fabric2seer): `set_y` takes 2 argument(s) in the port (`y: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let (__q0,) = ((__this.lock().borrow().get_y()).wrapping_add(25),); __this.lock().borrow_mut().input.set_y(__q0) };
        { let (__q0,) = ((__this.lock().borrow().get_y()).wrapping_add(25),); __this.lock().borrow_mut().ok_button.as_mut().unwrap().set_y(__q0) };
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((0) as f64) as f64;
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseClicked, scrollRate, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: an element of `children()` (`List.of(input, okButton)`) - a reference to one of the widget's two child fields
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangedValueChild {
    /// `input`
    Input,
    /// `okButton`
    OkButton,
}

/// java: public enum State
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum State {
    Closed,
    Open,
    ModifyingMin,
    ModifyingMax,
}

impl State {
    pub const VALUES: [State; 4] = [State::Closed, State::Open, State::ModifyingMin, State::ModifyingMax];

    /// java: values()
    pub fn values() -> Vec<State> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            State::Closed => "CLOSED",
            State::Open => "OPEN",
            State::ModifyingMin => "MODIFYING_MIN",
            State::ModifyingMax => "MODIFYING_MAX",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> State {
        match name {
            "CLOSED" => State::Closed,
            "OPEN" => State::Open,
            "MODIFYING_MIN" => State::ModifyingMin,
            "MODIFYING_MAX" => State::ModifyingMax,
            _ => panic!("IllegalArgumentException: No enum constant State.{name}"),
        }
    }

}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: protected class ModifiedTextFieldWidget extends EditBox
#[derive(Clone)]
pub struct ModifiedTextFieldWidget {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: private boolean isGood = false;
    pub is_good: bool,
}

impl std::ops::Deref for ModifiedTextFieldWidget {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ModifiedTextFieldWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ModifiedTextFieldWidget {
    /// java: public ModifiedTextFieldWidget(Font textRenderer, int x, int y, int width, int height, Component text)
    pub fn new(mut text_renderer: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text: MutableComponent) -> Self {
        // java: super(textRenderer, x, y, width, height, text); - the port's EditBox measures through the font's
        // splitter; its message (`text`, the narration) is the widget half's, which this box is in none of yet
        let mut this: Self = Self { base: EditBox::new(&text_renderer.splitter(), x, y, width, height), is_good: false };
        this
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(&self, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // java: if (!isFocused()) return false;
        // rust: port gap - an EditBox's focus lives in the screen's widget tree (`Screen.focused`), and this one is
        // in no screen, so it is never focused (GAPS_batchF.md)
        let __is_focused: bool = false;
        if !__is_focused {
            return false;
        }
        if input.is_confirmation() && self.is_good {
            // TODO(fabric2seer): `sendPacket(..)` is a method of the enclosing instance (RangedValueWidget.this) - an inner class has no outer reference in Rust
            //   java: sendPacket();
            return true;
        }
        // TODO(fabric2seer): `key_pressed` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, event: KeyEvent, focused: bool, active: bool, clipboard: &mut dyn ClipboardAccess`), Java passes 1
        // rust: return self.base.key_pressed(input);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public void setGood(boolean good)
    pub fn set_good(&mut self, mut good: bool) {
        self.is_good = good;
    }

}

// NOTE(fabric2seer): keyPressed override(s) of EditBox - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// RangedValueWidget's container half (`super`'s fields)
impl ContainerWidgetHalf for RangedValueWidget {
    fn container_half(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }
}

/// RangedValueWidget as the game's widget: `ContainerWidgetAdapter::widget(handle)` where Java hands it on
impl mcrust::mcrust::modding::widgets::ModWidget for ContainerWidgetAdapter<RangedValueWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend(|__h| RangedValueWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend(|__h| RangedValueWidget::update_widget_narration(__h, &*output))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(|__h| RangedValueWidget::mouse_clicked(__h, event, double_click))
    }

    fn container_widget_mut(&mut self) -> Option<&mut AbstractContainerWidget> {
        Some(self.part_mut())
    }
}
