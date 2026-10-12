//! Ported from de.hysky.skyblocker.utils.ScreenUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::navigation::screen_axis::ScreenAxis;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public final class ScreenUtils
#[derive(Clone)]
pub struct ScreenUtils {}

impl ScreenUtils {
    /// java: private ScreenUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static @Nullable Screen getUnderlyingScreen()
    pub fn get_underlying_screen() -> Option<&'static mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        return ScreenUtils::get_underlying_screen_1(compat::minecraft().gui.screen());
    }

    /// java: @Contract("null -> null; !null -> !null") public static @Nullable Screen getUnderlyingScreen(@Nullable Screen screen)
    pub fn get_underlying_screen_1<'a>(mut screen: Option<&'a mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> Option<&'a mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (screen) {
        //   java: 			case PopupScreen popupScreen -> getUnderlyingScreen(((PopupScreenAccessor) popupScreen).getUnderlyingScreen());
        //   java: 			case AbstractPopupScreen popupScreen -> getUnderlyingScreen(popupScreen.backgroundScreen);
        //   java: 			case null, default -> screen;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static ScreenRectangle getSnapBox(ScreenRectangle rect, ScreenDirection side)
    pub fn get_snap_box(rect: &ScreenRectangle, side: &ScreenDirection) -> ScreenRectangle {
        let mut extra_x: i32 = rect.width().wrapping_div(2);
        let mut extra_y: i32 = rect.height().wrapping_div(2);
        let mut primary_pos: i32 = rect.get_bound_in_direction(*side);
        let mut primary_size: i32 = 5i32.wrapping_add(if side.get_axis() == ScreenAxis::Horizontal { extra_x } else { extra_y });
        let mut other_axis: ScreenAxis = side.get_axis().orthogonal();
        let mut secondary_pos: i32 = rect.get_bound_in_direction(other_axis.get_negative());
        let mut secondary_size: i32 = rect.get_length(other_axis);
        let mut screen_rect: ScreenRectangle = ScreenRectangle::of(side.get_axis(), primary_pos, secondary_pos, primary_size, secondary_size);
        let mut offset_x: i32 = (if side.get_axis() == ScreenAxis::Horizontal { (if side.is_positive() { (-extra_x).wrapping_add(1) } else { -5 }) } else { 0 });
        let mut offset_y: i32 = (if side.get_axis() == ScreenAxis::Vertical { (if side.is_positive() { (-extra_y).wrapping_add(1) } else { -5 }) } else { 0 });
        return ScreenRectangle::new_at(screen_rect.left().wrapping_add(offset_x), screen_rect.top().wrapping_add(offset_y), screen_rect.width(), screen_rect.height());
    }

    /// java: public static StringWidget textWidget(int x, int y, Component text)
    pub fn text_widget(mut x: i32, mut y: i32, text: &MutableComponent) -> AbstractWidget {
        return compat::string_widget(x, y, compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone())), mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, text.clone());
    }

}
