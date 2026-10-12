//! Ported from de.hysky.skyblocker.skyblock.events.JacobEventToast by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::events::event_toast;
use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToast;
use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToastBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::jacobs_contest_widget;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::mth;

/// java: private static final Component CROPS = Component.translatable("skyblocker.events.crops");
pub static CROPS: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.events.crops"));

/// java: public class JacobEventToast extends EventToast
pub struct JacobEventToast {
    /// java: extends EventToast
    pub base: EventToastBase,
    /// java: private final List<String> crops;
    pub crops: Vec<String>,
    /// java: private final int cropsWidth;
    pub crops_width: i32,
}

impl std::ops::Deref for JacobEventToast {
    type Target = EventToastBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for JacobEventToast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl JacobEventToast {
    /// java: public JacobEventToast(long eventStartTime, long eventEndTime, String name, List<String> crops)
    pub fn new(mut event_start_time: i64, mut event_end_time: i64, mut name: String, mut crops: Vec<String>) -> compat::Shared<Self> {
        let __init_crops = crops.clone();
        let mut renderer: Font = compat::font();
        let __init_crops_width = renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(CROPS.get_mut().clone()));
        let (_, __this) = EventToastBase::new_in(event_start_time, event_end_time, name.clone(), ico::IRON_HOE.get_mut().clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                crops: __init_crops,
                crops_width: __init_crops_width,
            });
            ((__h.clone() as compat::Shared<dyn EventToast>), __h)
        });
        let mut i: i32 = ((compat::read(&__this, |__t| __t.crops_width)).wrapping_add(4)).wrapping_add((crops.len() as i32).wrapping_mul(24));
        { let __v = (compat::read(&__this, |__t| __t.message_width)).max(i); compat::write(&__this, |__t| __t.message_width = __v); }
        { let __v = (compat::read(&__this, |__t| __t.message_now_width)).max(i); compat::write(&__this, |__t| __t.message_now_width = __v); }
        __this
    }

}

impl EventToast for JacobEventToast {
    fn event_toast_base(&self) -> &EventToastBase {
        &self.base
    }
    fn event_toast_base_mut(&mut self) -> &mut EventToastBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, long startTime) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn EventToast>, &GuiGraphicsExtractor, &Font, i64) {
        JacobEventToast::extract_render_state_dyn
    }

}

impl JacobEventToast {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, long startTime)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, text_renderer: &Font, mut start_time: i64) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), event_toast::TEXTURE.get_mut(), 0, 0, ({ let __f = compat::dispatch(__this, |__t| __t.width()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }), ({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }));
        let mut y: i32 = (({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }).wrapping_sub({ let __f = compat::dispatch(__this, |__t| __t.get_inner_contents_height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) })).wrapping_div(2);
        if start_time < (3000 as i64) {
            let mut k: i32 = ((mth::floor_f32(((((3000 as i64).wrapping_sub(start_time) as f32) / 200.0f32).clamp(0.0f32, 1.0f32) * 255.0f32))).wrapping_shl((24) as u32) | 0x4000000);
            y = 2i32.wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.extract_message()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>), graphics, 30, y, (0xFFFFFF | k)) });
        } else {
            let mut k: i32 = ((!(mth::floor_f32((((start_time.wrapping_sub(3000 as i64) as f32) / 200.0f32).clamp(0.0f32, 1.0f32) * 255.0f32)))).wrapping_shl((24) as u32) | 0x4000000);
            let mut x: i32 = (30i32.wrapping_add(compat::read(__this, |__t| __t.crops_width))).wrapping_add(4);
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(CROPS.get_mut().clone()), 30, 7i32.wrapping_add((16i32.wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2)), mcrust::net::minecraft::util::common_colors::WHITE, false);
            for i in 0..(__this.lock().borrow().crops.len() as i32) {
                graphics.item(&FlexibleItemStack::get_stack_or_throw(&jacobs_contest_widget::FARM_DATA.get_mut().get(&__this.lock().borrow().crops[(i) as usize]).cloned().unwrap_or_else(|| ico::IRON_HOE.get_mut().clone())), x.wrapping_add(i.wrapping_mul(16i32.wrapping_add(8))), 7);
            }
            GuiHelper::nine_slice_colored(graphics, event_toast::TEXTURE.get_mut(), 0, 0, ({ let __f = compat::dispatch(__this, |__t| __t.width()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }), ({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }), argb::color_from_float(((k.wrapping_shr((24) as u32) as f32) / 255.0f32), 1.0f32, 1.0f32, 1.0f32));
            y = y.wrapping_add((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_mul(__this.lock().borrow().message.len() as i32));
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_timer()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>), graphics, 30, y) });
        graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone()).unwrap()), 8, (({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn EventToast>)) }).wrapping_div(2)).wrapping_sub(8));
    }

    /// JacobEventToast.extractRenderState as the dispatch of EventToast.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn EventToast>, __p0: &GuiGraphicsExtractor, __p1: &Font, __p2: i64) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
