//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetPositioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::Point;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::positioned_widget::PositionedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::org::joml::vector2i::Vector2i;

/// java: public abstract class WidgetPositioner
#[derive(Clone)]
pub struct WidgetPositionerBase {
    /// java: protected final float maxHeight;
    pub max_height: f32,
    /// java: protected final int screenHeight;
    pub screen_height: i32,
}

impl WidgetPositionerBase {
    /// java: public WidgetPositioner(float maxHeight, int screenHeight) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut max_height: f32, mut screen_height: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn WidgetPositioner>, __R)) -> (compat::Shared<dyn WidgetPositioner>, __R) {
        let __init_max_height = max_height;
        let __init_screen_height = screen_height;
        let this = Self {
            max_height: __init_max_height,
            screen_height: __init_screen_height,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: public static void applyRuleToWidget(PositionedWidget widget, int screenWidth, int screenHeight, Function<String, PositionedWidget> widgetProvider)
    pub fn apply_rule_to_widget(widget: &compat::Shared<PositionedWidget>, mut screen_width: i32, mut screen_height: i32, widget_provider: Arc<dyn Fn(String) -> compat::Shared<PositionedWidget>>) {
        { let __v = true; compat::write(widget, |__t| __t.positioned = __v); }
        let mut rule: PositionRule = compat::read(widget, |__t| __t.rule.clone());
        let mut start_x: i32;
        let mut start_y: i32;
        if rule.parent().is_none() {
            start_x = (((rule.parent_point().horizontal_point().get_percentage() * (screen_width as f32))) as i32);
            start_y = (((rule.parent_point().vertical_point().get_percentage() * (screen_height as f32))) as i32);
        } else {
            let mut parent_widget: compat::Shared<PositionedWidget> = (widget_provider)(rule.parent().unwrap());
            if compat::read(&parent_widget, |__t| __t.from_tab) {
                return;
            }
            if !(compat::read(&parent_widget, |__t| __t.positioned)) {
                WidgetPositionerBase::apply_rule_to_widget(&parent_widget, screen_width, screen_height, widget_provider.clone());
            }
            if compat::read(&parent_widget, |__t| __t.visible) {
                start_x = ({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) }).wrapping_add(((rule.parent_point().horizontal_point().get_percentage() * (({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }) as f32))) as i32);
                start_y = ({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) }).wrapping_add(((rule.parent_point().vertical_point().get_percentage() * (({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }) as f32))) as i32);
            } else {
                start_x = ({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_x()); __f(__r) });
                start_y = ({ let __r = &compat::read(&parent_widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_y()); __f(__r) });
            }
        }
        if compat::read(widget, |__t| __t.visible) {
            ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, (start_x.wrapping_add(rule.relative_x())).wrapping_sub(((rule.this_point().horizontal_point().get_percentage() * (({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_width()); __f(__r) }) as f32))) as i32)) });
            ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_y()); __f(__r, (start_y.wrapping_add(rule.relative_y())).wrapping_sub(((rule.this_point().vertical_point().get_percentage() * (({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_height()); __f(__r) }) as f32))) as i32)) });
        } else {
            ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_x()); __f(__r, start_x.wrapping_add(rule.relative_x())) });
            ({ let __r = &compat::read(widget, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_y()); __f(__r, start_y.wrapping_add(rule.relative_y())) });
        }
    }

    /// java: public static ScreenPosition getStartPosition(@Nullable String parent, int screenWidth, int screenHeight, PositionRule.Point parentPoint)
    pub fn get_start_position(mut parent: Option<String>, mut screen_width: i32, mut screen_height: i32, parent_point: &Point) -> ScreenPosition {
        if let Some(parent) = &parent {
            let Some(mut parent_widget) = widget_manager::WIDGET_INSTANCES.get_mut().get(parent).cloned() else {
                return ScreenPosition::new(0, 0);
            };
            return ScreenPosition::new(({ let __f = compat::dispatch(&parent_widget, |__t| __t.get_x()); __f(&parent_widget) }).wrapping_add(((parent_point.horizontal_point().get_percentage() * (({ let __f = compat::dispatch(&parent_widget, |__t| __t.get_width()); __f(&parent_widget) }) as f32))) as i32), ({ let __f = compat::dispatch(&parent_widget, |__t| __t.get_y()); __f(&parent_widget) }).wrapping_add(((parent_point.vertical_point().get_percentage() * (({ let __f = compat::dispatch(&parent_widget, |__t| __t.get_height()); __f(&parent_widget) }) as f32))) as i32));
        } else {
            return ScreenPosition::new((((parent_point.horizontal_point().get_percentage() * (screen_width as f32))) as i32), (((parent_point.vertical_point().get_percentage() * (screen_height as f32))) as i32));
        }
    }

}

/// java: the virtual methods of WidgetPositioner (implemented by WidgetPositionerBase and every subclass)
pub trait WidgetPositioner: compat::DynAny {
    fn widget_positioner_base(&self) -> &WidgetPositionerBase;
    fn widget_positioner_base_mut(&mut self) -> &mut WidgetPositionerBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void positionWidget(HudWidget hudWidget); (dispatch: the body for this class)
    fn position_widget(&self) -> fn(&compat::Shared<dyn WidgetPositioner>, &compat::Shared<dyn HudWidget>);

    /// java: public abstract Vector2i finalizePositioning(); (dispatch: the body for this class)
    fn finalize_positioning(&self) -> fn(&compat::Shared<dyn WidgetPositioner>) -> Vector2i;

}
