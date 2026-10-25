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
    /// java: private final ModifiedTextFieldWidget input; - the subclass's own field (`isGood`); the box itself is the
    /// game's `EditBox` widget, `children`' `INPUT`
    pub input: ModifiedTextFieldWidget,
    /// java: `children()` = `List.of(input, okButton)` - the two child widgets, held the way a container row holds its
    /// own (`INPUT` the edit box, `OK_BUTTON` the button), drawn and clicked through the game's widget bodies
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
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
        // java: this.input = new ModifiedTextFieldWidget(Minecraft.getInstance().font, x, y + 25, width - 15, 15, Component.empty());
        let __init_input = ModifiedTextFieldWidget::new(compat::font(), x, y.wrapping_add(25), width.wrapping_sub(15), 15, component::empty());
        let mut __children = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        __children.add_renderable_widget(compat::edit_box(x, y.wrapping_add(25), width.wrapping_sub(15), 15, component::empty()));
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
            children: __children,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // java: this.input.setVisible(false); this.input.setMaxLength(3);
        compat::write(&__this, |__t| {
            let input = __t.children.widget_mut(INPUT);
            input.visible = false;
            compat::with_edit_box(input, |e| e.set_max_length(3));
        });
        // java: input.setResponder(this::updateConfirmButton); - the box answers its value changes to this object, which
        // holds the box: the change is seen where this object hands the box an event (`after_input_event`), which calls
        // `updateConfirmButton` with the new value, as the responder would
        // java: this.okButton = Button.builder(Component.literal("✔"), _ -> sendPacket()).bounds(x + width - 15, y + 25, 15, 15).build();
        // (the press is this object's: `mouse_clicked` runs `sendPacket` when the button takes the click)
        let mut __ok_button = button::builder(component::literal("✔").get_string(), OnPress::Inert).bounds((x.wrapping_add(width)).wrapping_sub(15), y.wrapping_add(25), 15, 15).build();
        // java: this.okButton.visible = false;
        __ok_button.visible = false;
        compat::write(&__this, |__t| { __t.children.add_renderable_widget(__ok_button); });
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
                __this.lock().borrow_mut().children.widget_mut(OK_BUTTON).active = false;
                __this.lock().borrow_mut().input.set_good(false);
            } else if compat::read(__this, |__t| __t.state) == State::ModifyingMin && i > compat::read(__this, |__t| __t.max) {
                __this.lock().borrow_mut().children.widget_mut(OK_BUTTON).active = false;
                __this.lock().borrow_mut().input.set_good(false);
            } else {
                let mut active1: bool = (compat::read(__this, |__t| __t.state) != State::ModifyingMax || i >= compat::read(__this, |__t| __t.min));
                __this.lock().borrow_mut().children.widget_mut(OK_BUTTON).active = active1;
                __this.lock().borrow_mut().input.set_good(active1);
            }
        }).map_err(|_| ());
        if let Err(_) = __result {
            __this.lock().borrow_mut().children.widget_mut(OK_BUTTON).active = false;
            __this.lock().borrow_mut().input.set_good(false);
        }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// (`context` is the frame's render context: the two children are drawn by the game's widget bodies, which take it)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
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
        // java: this.input.extractRenderState(graphics, mouseX, mouseY, a); this.okButton.extractRenderState(graphics, mouseX, mouseY, a);
        // (each draws only while visible - `AbstractWidget.extractRenderState:56`)
        compat::write(__this, |__t| context.extract_widgets(&mut __t.children, mouse_x, mouse_y));
        let _ = a;
        if compat::read(__this, |__t| __t.state) == State::Closed {
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_add(text_offset), (__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width)), ((__this.lock().borrow().get_y()).wrapping_add(15)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("{} - {}", compat::read(__this, |__t| __t.min), compat::read(__this, |__t| __t.max))), (__this.lock().borrow().get_x()).wrapping_add(3), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32, false);
        } else {
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_add(text_offset), (__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width)), ((__this.lock().borrow().get_y()).wrapping_add(15)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "-", (__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32)), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            let mut selected_color: i32 = 0xFFFFFF00u32 as i32;
            let mut unselected_color: i32 = 0xFFD0D0D0u32 as i32;
            let mut mouse_over_min: bool = RangedValueWidget::mouse_over_min_button(__this, mouse_x, mouse_y);
            let mut mouse_over_max: bool = RangedValueWidget::mouse_over_max_button(__this, mouse_x, mouse_y);
            let mut min_start_x: i32 = (__this.lock().borrow().get_x()).wrapping_add(1);
            let mut min_end_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_sub(6);
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), min_start_x, ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), min_end_x, ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), (if compat::read(__this, |__t| __t.state) == State::ModifyingMin { selected_color } else { (if mouse_over_min { 0xFFFFFFFFu32 as i32 } else { unselected_color }) }));
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), min_start_x.wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(2)).wrapping_add(text_offset), min_end_x.wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(13)).wrapping_add(text_offset), 0xFF000000u32 as i32);
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.min).to_string(), (min_start_x.wrapping_add(min_end_x)).wrapping_shr((1) as u32), ((__this.lock().borrow().get_y()).wrapping_add(3)).wrapping_add(text_offset), 0xFFFFFFFFu32 as i32);
            let mut max_start_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_shr((1) as u32))).wrapping_add(5);
            let mut max_end_x: i32 = ((__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width))).wrapping_sub(1);
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), max_start_x, ((__this.lock().borrow().get_y()).wrapping_add(1)).wrapping_add(text_offset), max_end_x, ((__this.lock().borrow().get_y()).wrapping_add(14)).wrapping_add(text_offset), (if compat::read(__this, |__t| __t.state) == State::ModifyingMax { selected_color } else { (if mouse_over_max { 0xFFFFFFFFu32 as i32 } else { unselected_color }) }));
            context.graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), max_start_x.wrapping_add(1), ((__this.lock().borrow().get_y()).wrapping_add(2)).wrapping_add(text_offset), max_end_x.wrapping_sub(1), ((__this.lock().borrow().get_y()).wrapping_add(13)).wrapping_add(text_offset), 0xFF000000u32 as i32);
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
                // java: this.input.setVisible(false); this.input.setFocused(false); this.okButton.visible = false; this.okButton.setFocused(false);
                compat::write(__this, |__t| {
                    let input = __t.children.widget_mut(INPUT);
                    input.visible = false;
                    RangedValueWidget::set_child_focused(input, false);
                    let ok_button = __t.children.widget_mut(OK_BUTTON);
                    ok_button.visible = false;
                    RangedValueWidget::set_child_focused(ok_button, false);
                    __t.children.set_focused(None);
                });
            }
            State::ModifyingMin | State::ModifyingMax => {
                // java: this.input.setVisible(true); this.input.moveCursorTo(0, false);
                // java: this.input.setValue(String.valueOf(state == State.MODIFYING_MIN ? min : max)); this.input.setFocused(true);
                // java: this.okButton.visible = true;
                let value: String = compat::read(__this, |__t| if state == State::ModifyingMin { __t.min } else { __t.max }).to_string();
                compat::write(__this, |__t| {
                    let font = compat::font();
                    let input = __t.children.widget_mut(INPUT);
                    input.visible = true;
                    compat::with_edit_box(input, |e| {
                        e.move_cursor_to(&font.splitter(), 0, false);
                        e.set_value(&font.splitter(), &value);
                    });
                    RangedValueWidget::set_child_focused(input, true);
                    // the box is the child keys go to (`ContainerEventHandler.getFocused()`)
                    __t.children.set_focused(Some(INPUT));
                    __t.children.widget_mut(OK_BUTTON).visible = true;
                });
                // (`setValue` calls the responder: `updateConfirmButton(value)`)
                RangedValueWidget::update_confirm_button(__this, &value);
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
        let mut input_trimmed: String = compat::trim(&compat::read(__this, |__t| compat::edit_box_value(__t.children.widget(INPUT))));
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
            let __sign_pos = sign.base().get_block_pos();
            let sign: &SignBlockEntity = sign.as_sign().expect("ClassCastException: SignBlockEntity");
            let __sign_text = sign.get_text(compat::sign_slot(PartyFinderScreen::is_sign_front(&compat::read(__this, |__t| __t.screen.clone()))));
            let messages = __sign_text.get_messages((PartyFinderScreen::get_client(&compat::read(__this, |__t| __t.screen.clone()))).is_text_filtering_enabled());
            // java: screen.getClient().player.connection.send(new ServerboundSignUpdatePacket(sign.getBlockPos(), screen.isSignFront(), inputTrimmed, messages[1].getString(), messages[2].getString(), messages[3].getString()));
            compat::send_packet(ServerboundSignUpdatePacket::new(__sign_pos, vec![input_trimmed.clone(), messages[1].get_string(), messages[2].get_string(), messages[3].get_string()], compat::sign_slot(PartyFinderScreen::is_sign_front(&compat::read(__this, |__t| __t.screen.clone())))));
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
                    // java: input.setValue(String.valueOf(state == State.MODIFYING_MIN ? min : max));
                    let value: String = compat::read(__this, |__t| if __t.state == State::ModifyingMin { __t.min } else { __t.max }).to_string();
                    compat::write(__this, |__t| compat::set_edit_box_value(__t.children.widget_mut(INPUT), &value));
                    RangedValueWidget::update_confirm_button(__this, &value);
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
                let scrolling: bool = false;
                let (took, focused) = compat::write(__this, |__t| {
                    let took = compat::children_mouse_clicked(&mut __t.children, click);
                    (took, __t.children.get_focused())
                });
                // the OK button's press (`_ -> sendPacket()`)
                if took && click.button() == 1 && focused == Some(OK_BUTTON) {
                    RangedValueWidget::send_packet(__this);
                }
                return took || scrolling;
            }
        }
    }

    /// `input.setFocused(b)` / `okButton.setFocused(b)` - `AbstractWidget.setFocused`, and an edit box's caret clock
    /// (`EditBox.setFocused:503-509` restarts the blink).
    fn set_child_focused(widget: &mut AbstractWidget, focused: bool) {
        widget.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None);
        let now: i64 = compat::millis();
        compat::with_edit_box(widget, |e| e.set_focused_at(focused, now));
    }

    /// java: `ContainerEventHandler.keyPressed(event)` - the focused child's: the input box, a
    /// `ModifiedTextFieldWidget`, whose override sends the value on Enter (`input.keyPressed`).
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if compat::read(__this, |__t| __t.children.get_focused()) == Some(INPUT) {
            return ModifiedTextFieldWidget::key_pressed(__this, event);
        }
        compat::write(__this, |__t| compat::children_key_pressed(&mut __t.children, event))
    }

    /// java: `ContainerEventHandler.charTyped(event)` - the focused child's (the input box).
    pub fn char_typed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        let before: String = compat::read(__this, |__t| compat::edit_box_value(__t.children.widget(INPUT)));
        let typed: bool = compat::write(__this, |__t| compat::children_char_typed(&mut __t.children, event));
        RangedValueWidget::after_input_event(__this, &before);
        typed
    }

    /// The input box's responder (`updateConfirmButton`), for an event that may have changed its value: called with the
    /// new value when it did, as the box's `onValueChange` calls the responder.
    fn after_input_event(__this: &compat::Shared<Self>, before: &str) {
        let after: String = compat::read(__this, |__t| compat::edit_box_value(__t.children.widget(INPUT)));
        if after != before {
            RangedValueWidget::update_confirm_button(__this, &after);
        }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        // java: this.input.setX(getX()); this.okButton.setX(getX() + getWidth() - 15);
        compat::write(__this, |__t| {
            let (x, width) = (__t.base.get_x(), __t.base.width);
            let font = compat::font();
            let input = __t.children.widget_mut(INPUT);
            input.set_x(x);
            compat::with_edit_box(input, |e| e.set_x(x, &font.splitter()));
            __t.children.widget_mut(OK_BUTTON).set_x((x.wrapping_add(width)).wrapping_sub(15));
        });
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        // java: this.input.setY(getY() + 25); this.okButton.setY(getY() + 25);
        compat::write(__this, |__t| {
            let y = __t.base.get_y().wrapping_add(25);
            let font = compat::font();
            let input = __t.children.widget_mut(INPUT);
            input.set_y(y);
            compat::with_edit_box(input, |e| e.set_y(y, &font.splitter()));
            __t.children.widget_mut(OK_BUTTON).set_y(y);
        });
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
/// (the subclass's own state; the box it extends is the game's `EditBox` widget, `RangedValueWidget.children`' `INPUT`,
/// and its `keyPressed` override is run on that box with the enclosing widget, `RangedValueWidget.this`)
#[derive(Clone)]
pub struct ModifiedTextFieldWidget {
    /// java: private boolean isGood = false;
    pub is_good: bool,
}

impl ModifiedTextFieldWidget {
    /// java: public ModifiedTextFieldWidget(Font textRenderer, int x, int y, int width, int height, Component text)
    /// (`super(textRenderer, x, y, width, height, text)` is the box `RangedValueWidget`'s constructor adds to its
    /// children)
    pub fn new(text_renderer: Font, x: i32, y: i32, width: i32, height: i32, text: MutableComponent) -> Self {
        let _ = (text_renderer, x, y, width, height, text);
        ModifiedTextFieldWidget { is_good: false }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    /// (`outer` is `RangedValueWidget.this`, whose `children` hold the box)
    pub fn key_pressed(outer: &compat::Shared<RangedValueWidget>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // java: if (!isFocused()) return false;
        if !compat::read(outer, |__t| __t.children.widget(INPUT).is_focused()) {
            return false;
        }
        // java: if (input.isConfirmation() && isGood) { sendPacket(); return true; }
        if input.is_confirmation() && compat::read(outer, |__t| __t.input.is_good) {
            RangedValueWidget::send_packet(outer);
            return true;
        }
        // java: return super.keyPressed(input); - `EditBox.keyPressed`, with the window's clipboard
        let before: String = compat::read(outer, |__t| compat::edit_box_value(__t.children.widget(INPUT)));
        let pressed: bool = compat::write(outer, |__t| compat::children_key_pressed(&mut __t.children, input));
        RangedValueWidget::after_input_event(outer, &before);
        pressed
    }

    /// java: public void setGood(boolean good)
    pub fn set_good(&mut self, good: bool) {
        self.is_good = good;
    }
}

/// `children()`' first element: the input box.
pub const INPUT: usize = 0;
/// `children()`' second element: the OK button.
pub const OK_BUTTON: usize = 1;

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
        self.lend(|__h| RangedValueWidget::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend(|__h| RangedValueWidget::update_widget_narration(__h, &*output))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(|__h| RangedValueWidget::mouse_clicked(__h, event, double_click))
    }

    /// java: `ContainerEventHandler.keyPressed(event)`
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(|__h| RangedValueWidget::key_pressed(__h, event))
    }

    /// java: `ContainerEventHandler.charTyped(event)`
    fn char_typed(&mut self, _widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        self.lend(|__h| RangedValueWidget::char_typed(__h, event))
    }

    fn container_widget_mut(&mut self) -> Option<&mut AbstractContainerWidget> {
        Some(self.part_mut())
    }
}
