//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.pipeline.CenteredWidgetPositioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositionerBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::screen_const;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::org::joml::vector2i::Vector2i;

/// java: public class CenteredWidgetPositioner extends WidgetPositioner
#[derive(Clone)]
pub struct CenteredWidgetPositioner {
    /// java: extends WidgetPositioner
    pub base: WidgetPositionerBase,
    /// java: int totalWidth = 0;
    pub total_width: i32,
    /// java: int totalHeight = 0;
    pub total_height: i32,
    /// java: final int maxY;
    pub max_y: i32,
    /// java: List<ObjectIntPair<List<HudWidget>>> columns = new ArrayList<>();
    pub columns: Vec<(Vec<compat::Shared<dyn HudWidget>>, i32)>,
    /// java: int currentY = 0;
    pub current_y: i32,
    /// java: int currentWidth = 0;
    pub current_width: i32,
}

impl std::ops::Deref for CenteredWidgetPositioner {
    type Target = WidgetPositionerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CenteredWidgetPositioner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CenteredWidgetPositioner {
    /// java: public CenteredWidgetPositioner(float maxHeight, int screenHeight)
    pub fn new(mut max_height: f32, mut screen_height: i32) -> compat::Shared<Self> {
        let (_, __this) = WidgetPositionerBase::new_in(max_height, screen_height, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                total_width: 0,
                total_height: 0,
                max_y: 0,
                columns: Vec::new(),
                current_y: 0,
                current_width: 0,
            });
            ((__h.clone() as compat::Shared<dyn WidgetPositioner>), __h)
        });
        let __h0 = (Vec::new(), 0);
        __this.lock().borrow_mut().columns.push(__h0);
        { let __v = 400i32.min(((((screen_height as f32) * max_height)) as i32)); compat::write(&__this, |__t| __t.max_y = __v); }
        __this
    }

}

impl WidgetPositioner for CenteredWidgetPositioner {
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
        CenteredWidgetPositioner::position_widget_dyn
    }

    /// java: @Override public Vector2i finalizePositioning() (dispatch: the body for this class)
    fn finalize_positioning(&self) -> fn(&compat::Shared<dyn WidgetPositioner>) -> Vector2i {
        CenteredWidgetPositioner::finalize_positioning_dyn
    }

}

impl CenteredWidgetPositioner {
    /// java: @Override public void positionWidget(HudWidget hudWidget)
    pub fn position_widget_impl(__this: &compat::Shared<Self>, hud_widget: &compat::Shared<dyn HudWidget>) {
        if (compat::read(__this, |__t| __t.current_y)).wrapping_add({ let __f = compat::dispatch(hud_widget, |__t| __t.get_height()); __f(hud_widget) }) > compat::read(__this, |__t| __t.max_y) {
            { let __v = (compat::read(__this, |__t| __t.total_height)).max(compat::read(__this, |__t| __t.current_y)); compat::write(__this, |__t| __t.total_height = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.current_y = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.current_width = __v); }
            let __h0 = (Vec::new(), 0);
            __this.lock().borrow_mut().columns.push(__h0);
        }
        ({ let __f = compat::dispatch(hud_widget, |__t| __t.set_y()); __f(hud_widget, compat::read(__this, |__t| __t.current_y)) });
        { let __v = (compat::read(__this, |__t| __t.current_y)).wrapping_add(({ let __f = compat::dispatch(hud_widget, |__t| __t.get_height()); __f(hud_widget) }).wrapping_add(screen_const::WIDGET_PAD)); compat::write(__this, |__t| __t.current_y = __v); }
        { let __v = (compat::read(__this, |__t| __t.current_width)).max(({ let __f = compat::dispatch(hud_widget, |__t| __t.get_width()); __f(hud_widget) })); compat::write(__this, |__t| __t.current_width = __v); }
        __this.lock().borrow().columns[__this.lock().borrow().columns.len() - 1].1;
        (__this.lock().borrow().columns[__this.lock().borrow().columns.len() - 1].0).push(hud_widget.clone());
    }

    /// CenteredWidgetPositioner.positionWidget as the dispatch of WidgetPositioner.positionWidget
    pub fn position_widget_dyn(__this: &compat::Shared<dyn WidgetPositioner>, __p0: &compat::Shared<dyn HudWidget>) {
        Self::position_widget_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Vector2i finalizePositioning()
    pub fn finalize_positioning_impl(__this: &compat::Shared<Self>) -> Vector2i {
        for i in 0..(__this.lock().borrow().columns.len() as i32) {
            let list_object_int_pair = &__this.lock().borrow().columns[(i) as usize];
            let mut column_width: i32 = list_object_int_pair.1;
            let column = &list_object_int_pair.0;
            let mut height: i32 = ((column.len() as i32).wrapping_sub(1)).wrapping_mul(screen_const::WIDGET_PAD);
            for tab_hud_widget in column.iter() {
                height = height.wrapping_add({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.get_height()); __f(tab_hud_widget) });
            }
            let mut offset: i32 = ((compat::read(__this, |__t| __t.total_height)).wrapping_sub(height)).wrapping_div(2);
            for tab_hud_widget in column.iter_mut() {
                ({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.set_y()); __f(tab_hud_widget, ({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.get_y()); __f(tab_hud_widget) }).wrapping_add(offset)) });
                if i < (__this.lock().borrow().columns.len() as i32).wrapping_div(2) {
                    ({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.set_x()); __f(tab_hud_widget, ((compat::read(__this, |__t| __t.total_width)).wrapping_add(column_width)).wrapping_sub({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.get_width()); __f(tab_hud_widget) })) });
                } else {
                    ({ let __f = compat::dispatch(tab_hud_widget, |__t| __t.set_x()); __f(tab_hud_widget, compat::read(__this, |__t| __t.total_width)) });
                }
            }
            { let __v = (compat::read(__this, |__t| __t.total_width)).wrapping_add(column_width.wrapping_add(screen_const::WIDGET_PAD)); compat::write(__this, |__t| __t.total_width = __v); }
        }
        return Vector2i::new(compat::read(__this, |__t| __t.total_width), compat::read(__this, |__t| __t.total_height));
    }

    /// CenteredWidgetPositioner.finalizePositioning as the dispatch of WidgetPositioner.finalizePositioning
    pub fn finalize_positioning_dyn(__this: &compat::Shared<dyn WidgetPositioner>) -> Vector2i {
        Self::finalize_positioning_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
