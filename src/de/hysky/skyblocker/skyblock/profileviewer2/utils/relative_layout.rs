//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.RelativeLayout by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;

/// java: public class RelativeLayout implements Layout
#[derive(Clone)]
pub struct RelativeLayout {
    /// java: private int x, y;
    pub x: i32,
    /// java: private int x, y;
    pub y: i32,
    /// java: private final List<Child> children = new ArrayList<>();
    pub children: Vec<Child>,
}

impl RelativeLayout {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            x: 0,
            y: 0,
            children: Vec::new(),
        };
        this
    }

    /// java: public <T extends LayoutElement> T addChild(T element, int relativeX, int relativeY)
    pub fn add_child<T: Clone + 'static>(&mut self, element: &T, mut relative_x: i32, mut relative_y: i32) -> T {
        self.children.push(Child::new(element.clone(), relative_x, relative_y));
        return element.clone();
    }

    /// java: @Override public void setX(int i)
    pub fn set_x(&mut self, mut i: i32) {
        self.x = i;
        for child in self.children.iter() {
            child.layout_element.set_x(self.x.wrapping_add(child.relative_x));
        }
    }

    /// java: @Override public void setY(int i)
    pub fn set_y(&mut self, mut i: i32) {
        self.y = i;
        for child in self.children.iter() {
            child.layout_element.set_y(self.y.wrapping_add(child.relative_y));
        }
    }

    /// java: @Override public int getX()
    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    /// java: @Override public int getY()
    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override public void arrangeElements()
    pub fn arrange_elements(&self) {
        for element in self.children.iter() {
            element.layout_element.set_position(self.x.wrapping_add(element.relative_x), self.y.wrapping_add(element.relative_y));
        }
    }

    /// java: @Override public void visitChildren(Consumer<LayoutElement> consumer)
    pub fn visit_children(&self, consumer: Arc<dyn Fn(LayoutElement)>) {
        for e in self.children.iter() {
            (consumer)(e.layout_element);
        }
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(&mut self) {
        self.children.clear();
    }

}

/// java: private record Child(LayoutElement layoutElement, int relativeX, int relativeY)
#[derive(Clone, Debug)]
pub struct Child {
    /// java: LayoutElement layoutElement
    pub layout_element: LayoutElement,
    /// java: int relativeX
    pub relative_x: i32,
    /// java: int relativeY
    pub relative_y: i32,
}

impl Child {
    /// java: the canonical record constructor
    pub fn new(mut layout_element: LayoutElement, mut relative_x: i32, mut relative_y: i32) -> Self {
        Self { layout_element, relative_x, relative_y }
    }

    /// java: the record accessor `layoutElement()`
    pub fn layout_element(&self) -> LayoutElement {
        self.layout_element
    }

    /// java: the record accessor `relativeX()`
    pub fn relative_x(&self) -> i32 {
        self.relative_x
    }

    /// java: the record accessor `relativeY()`
    pub fn relative_y(&self) -> i32 {
        self.relative_y
    }

}
