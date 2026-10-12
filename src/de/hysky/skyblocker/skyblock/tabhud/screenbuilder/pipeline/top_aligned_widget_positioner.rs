//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.pipeline.TopAlignedWidgetPositioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositionerBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::screen_const;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::org::joml::vector2i::Vector2i;

/// java: private static final int START_Y = 0;
pub const START_Y: i32 = 0;

/// java: public class TopAlignedWidgetPositioner extends WidgetPositioner
#[derive(Clone)]
pub struct TopAlignedWidgetPositioner {
    /// java: extends WidgetPositioner
    pub base: WidgetPositionerBase,
    /// java: private int totalWidth = 0;
    pub total_width: i32,
    /// java: private int totalHeight = 0;
    pub total_height: i32,
    /// java: private int currentWidth = 0;
    pub current_width: i32,
    /// java: private int currentY = START_Y;
    pub current_y: i32,
}

impl std::ops::Deref for TopAlignedWidgetPositioner {
    type Target = WidgetPositionerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TopAlignedWidgetPositioner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TopAlignedWidgetPositioner {
    /// java: public TopAlignedWidgetPositioner(float maxHeight, int screenHeight)
    pub fn new(mut max_height: f32, mut screen_height: i32) -> compat::Shared<Self> {
        let (_, __this) = WidgetPositionerBase::new_in(max_height, screen_height, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                total_width: 0,
                total_height: 0,
                current_width: 0,
                current_y: START_Y,
            });
            ((__h.clone() as compat::Shared<dyn WidgetPositioner>), __h)
        });
        __this
    }

}

impl WidgetPositioner for TopAlignedWidgetPositioner {
    fn widget_positioner_base(&self) -> &WidgetPositionerBase {
        &self.base
    }
    fn widget_positioner_base_mut(&mut self) -> &mut WidgetPositionerBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void positionWidget(HudWidget hudWidget) (dispatch: the body for this class)
    fn position_widget(&self) -> fn(&compat::Shared<dyn WidgetPositioner>, &compat::Shared<dyn HudWidget>) {
        TopAlignedWidgetPositioner::position_widget_dyn
    }

    /// java: @Override public Vector2i finalizePositioning() (dispatch: the body for this class)
    fn finalize_positioning(&self) -> fn(&compat::Shared<dyn WidgetPositioner>) -> Vector2i {
        TopAlignedWidgetPositioner::finalize_positioning_dyn
    }

}

impl TopAlignedWidgetPositioner {
    /// java: @Override public void positionWidget(HudWidget hudWidget)
    pub fn position_widget_impl(__this: &compat::Shared<Self>, hud_widget: &compat::Shared<dyn HudWidget>) {
        if ((compat::read(__this, |__t| __t.current_y)).wrapping_add({ let __f = compat::dispatch(hud_widget, |__t| __t.get_height()); __f(hud_widget) }) as f32) > ((compat::read(__this, |__t| __t.screen_height) as f32) * compat::read(__this, |__t| __t.max_height)) {
            { let __v = (compat::read(__this, |__t| __t.total_height)).max(compat::read(__this, |__t| __t.current_y)); compat::write(__this, |__t| __t.total_height = __v); }
            { let __v = (compat::read(__this, |__t| __t.total_width)).wrapping_add((compat::read(__this, |__t| __t.current_width)).wrapping_add(screen_const::WIDGET_PAD)); compat::write(__this, |__t| __t.total_width = __v); }
            { let __v = START_Y; compat::write(__this, |__t| __t.current_y = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.current_width = __v); }
        }
        // TODO(fabric2seer): `setPosition` is not a method of HudWidget
        //   java: hudWidget.setPosition(totalWidth, currentY);
        { let __v = (compat::read(__this, |__t| __t.current_y)).wrapping_add(({ let __f = compat::dispatch(hud_widget, |__t| __t.get_height()); __f(hud_widget) }).wrapping_add(screen_const::WIDGET_PAD)); compat::write(__this, |__t| __t.current_y = __v); }
        { let __v = (compat::read(__this, |__t| __t.current_width)).max(({ let __f = compat::dispatch(hud_widget, |__t| __t.get_width()); __f(hud_widget) })); compat::write(__this, |__t| __t.current_width = __v); }
    }

    /// TopAlignedWidgetPositioner.positionWidget as the dispatch of WidgetPositioner.positionWidget
    pub fn position_widget_dyn(__this: &compat::Shared<dyn WidgetPositioner>, __p0: &compat::Shared<dyn HudWidget>) {
        Self::position_widget_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Vector2i finalizePositioning()
    pub fn finalize_positioning_impl(__this: &compat::Shared<Self>) -> Vector2i {
        { let __v = (compat::read(__this, |__t| __t.total_height)).max(compat::read(__this, |__t| __t.current_y)); compat::write(__this, |__t| __t.total_height = __v); }
        { let __v = (compat::read(__this, |__t| __t.total_width)).wrapping_add((compat::read(__this, |__t| __t.current_width)).wrapping_add(screen_const::WIDGET_PAD)); compat::write(__this, |__t| __t.total_width = __v); }
        return Vector2i::new(compat::read(__this, |__t| __t.total_width), compat::read(__this, |__t| __t.total_height));
    }

    /// TopAlignedWidgetPositioner.finalizePositioning as the dispatch of WidgetPositioner.finalizePositioning
    pub fn finalize_positioning_dyn(__this: &compat::Shared<dyn WidgetPositioner>) -> Vector2i {
        Self::finalize_positioning_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
