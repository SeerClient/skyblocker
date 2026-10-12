//! Ported from de.hysky.skyblocker.utils.render.gui.ToggleableLayoutWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;

/// java: public class ToggleableLayoutWidget implements Layout
#[derive(Clone)]
pub struct ToggleableLayoutWidget {
    /// java: private final LayoutElement widget;
    pub widget: LayoutElement,
    /// java: private final BooleanSupplier isEnabled;
    pub is_enabled: Arc<dyn Fn() -> bool>,
    /// java: private boolean enabled;
    pub enabled: bool,
}

impl ToggleableLayoutWidget {
    /// java: public ToggleableLayoutWidget(LayoutElement widget, BooleanSupplier isEnabled)
    pub fn new(mut widget: LayoutElement, is_enabled: Arc<dyn Fn() -> bool>) -> Self {
        let __init_widget = widget;
        let __init_is_enabled = is_enabled.clone();
        let mut this = Self {
            widget: __init_widget,
            is_enabled: __init_is_enabled,
            enabled: false,
        };
        this
    }

    /// java: @Override public void visitChildren(Consumer<LayoutElement> consumer)
    ///
    /// (the port's layouts visit their children by reference, as `GridLayout.visitChildren` does)
    pub fn visit_children(&self, consumer: &mut dyn FnMut(&LayoutElement)) {
        if self.enabled {
            (consumer)(&self.widget);
        }
    }

    /// java: @Override public void arrangeElements()
    pub fn arrange_elements(&mut self) {
        self.enabled = (self.is_enabled)();
        // java: Layout.super.arrangeElements(); - `visitChildren(child -> { if (child instanceof Layout layout)
        // layout.arrangeElements(); })`, and this class's `visitChildren` visits `widget` only while enabled
        if self.enabled {
            match &mut self.widget {
                LayoutElement::Grid(layout) => layout.arrange_elements(),
                LayoutElement::Frame(layout) => layout.arrange_elements(),
                LayoutElement::EqualSpacing(layout) => layout.arrange_elements(),
                LayoutElement::Scrollable(layout) => layout.arrange_elements(),
                LayoutElement::Widget { .. } | LayoutElement::Spacer { .. } => {}
            }
        }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.widget.set_x(x);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.widget.set_y(y);
    }

    /// java: @Override public int getX()
    pub fn get_x(&self) -> i32 {
        return self.widget.get_x();
    }

    /// java: @Override public int getY()
    pub fn get_y(&self) -> i32 {
        return self.widget.get_y();
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return (if self.enabled { self.widget.get_width() } else { 0 });
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return (if self.enabled { self.widget.get_height() } else { 0 });
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(&mut self) {
        // java: if (this.enabled && this.widget instanceof Layout layout) layout.removeChildren();
        if self.enabled {
            match &mut self.widget {
                LayoutElement::Grid(layout) => layout.remove_children(),
                LayoutElement::Frame(layout) => layout.remove_children(),
                LayoutElement::EqualSpacing(layout) => layout.remove_children(),
                // rust: port gap - `ScrollableLayout.removeChildren:82-85` has no port on `ScrollableLayoutElement`
                // (GAPS_batchA.md)
                LayoutElement::Scrollable(_) => {}
                LayoutElement::Widget { .. } | LayoutElement::Spacer { .. } => {}
            }
        }
    }

}
