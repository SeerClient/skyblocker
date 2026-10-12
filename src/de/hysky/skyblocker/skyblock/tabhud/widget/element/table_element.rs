//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.TableElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: private static final int EXTRA_PAD = PAD_L * 2 - PAD_S;
pub static EXTRA_PAD: LazyLock<i32> = LazyLock::new(|| ((element::PAD_L).wrapping_mul(2)).wrapping_sub(element::PAD_S));

/// java: public class TableElement extends Element
#[derive(Clone)]
pub struct TableElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final Element[][] comps;
    pub comps: Vec<Vec<compat::Shared<dyn Element>>>,
    /// java: private final int color;
    pub color: i32,
    /// java: private final int cols, rows;
    pub cols: i32,
    /// java: private final int cols, rows;
    pub rows: i32,
    /// java: private final int[] colWidths;
    pub col_widths: Vec<i32>,
    /// java: private final int[] rowBorders;
    pub row_borders: Vec<i32>,
    /// java: private final int[] rowBaseHeights;
    pub row_base_heights: Vec<i32>,
    /// java: private final int[] rowHeights;
    pub row_heights: Vec<i32>,
    /// java: private final boolean drawLines;
    pub draw_lines: bool,
}

impl std::ops::Deref for TableElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TableElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TableElement {
    /// java: public TableElement(int w, int h, int col, boolean drawLines)
    pub fn new(mut w: i32, mut h: i32, mut col: i32, mut draw_lines: bool) -> compat::Shared<Self> {
        let __init_comps = vec![(0..(h) as usize).map(|_| Default::default()).collect::<Vec<_>>(); (w) as usize];
        let __init_color = (if draw_lines && col != 0 { (0xFF000000u32 as i32 | col) } else { 0 });
        let __init_draw_lines = (draw_lines && col != 0);
        let __init_cols = w;
        let __init_rows = h;
        let __init_col_widths = vec![0; (w) as usize];
        let __init_row_borders = vec![0; (h) as usize];
        let __init_row_base_heights = vec![0; (h) as usize];
        let __init_row_heights = vec![0; (h) as usize];
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                comps: __init_comps,
                color: __init_color,
                cols: __init_cols,
                rows: __init_rows,
                col_widths: __init_col_widths,
                row_borders: __init_row_borders,
                row_base_heights: __init_row_base_heights,
                row_heights: __init_row_heights,
                draw_lines: __init_draw_lines,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        __this
    }

    /// java: public void addToCell(int x, int y, Element c)
    pub fn add_to_cell(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut c: compat::Shared<dyn Element>) {
        (__this.lock().borrow().comps[(x) as usize])[(y) as usize] = c.clone();
        let mut pad: i32 = (if x == 0 { (element::PAD_L).wrapping_mul(2) } else { element::PAD_L });
        let __r0 = compat::read(&c, |__t| __t.element_base().width);
        __this.lock().borrow_mut().col_widths[(x) as usize] = (__this.lock().borrow().col_widths[(x) as usize]).max((__r0).wrapping_add(pad));
        let mut base_h: i32 = (compat::read(&c, |__t| __t.element_base().height)).wrapping_add(element::PAD_S);
        if base_h > __this.lock().borrow().row_base_heights[(y) as usize] {
            __this.lock().borrow_mut().row_base_heights[(y) as usize] = base_h;
            __this.lock().borrow_mut().row_heights[(y) as usize] = base_h.wrapping_add(if __this.lock().borrow().row_borders[(y) as usize] != 0 { *EXTRA_PAD } else { 0 });
        } else if __this.lock().borrow().row_borders[(y) as usize] != 0 {
            __this.lock().borrow_mut().row_heights[(y) as usize] = (__this.lock().borrow().row_heights[(y) as usize]).max((__this.lock().borrow().row_base_heights[(y) as usize]).wrapping_add(*EXTRA_PAD));
        } else {
            __this.lock().borrow_mut().row_heights[(y) as usize] = (__this.lock().borrow().row_heights[(y) as usize]).max(__this.lock().borrow().row_base_heights[(y) as usize]);
        }
        TableElement::recalc_dimensions(__this);
    }

    /// java: public void setRowBorder(int row, int borderColor)
    pub fn set_row_border(__this: &compat::Shared<Self>, mut row: i32, mut border_color: i32) {
        if row >= 0 && row < (__this.lock().borrow().row_borders.len() as i32) {
            let mut had_border: bool = (__this.lock().borrow().row_borders[(row) as usize] != 0);
            __this.lock().borrow_mut().row_borders[(row) as usize] = (if border_color == 0 { 0 } else { (0xFF000000u32 as i32 | border_color) });
            let mut has_border: bool = (__this.lock().borrow().row_borders[(row) as usize] != 0);
            if has_border && !had_border {
                __this.lock().borrow_mut().row_heights[(row) as usize] = (__this.lock().borrow().row_heights[(row) as usize]).max((__this.lock().borrow().row_base_heights[(row) as usize]).wrapping_add(*EXTRA_PAD));
            } else if !has_border && had_border {
                __this.lock().borrow_mut().row_heights[(row) as usize] = __this.lock().borrow().row_base_heights[(row) as usize];
            }
            TableElement::recalc_dimensions(__this);
        }
    }

    /// java: private void recalcDimensions()
    pub fn recalc_dimensions(__this: &compat::Shared<Self>) {
        let mut total_w: i32 = 0;
        for w in compat::read(__this, |__t| __t.col_widths.clone()) {
            total_w = total_w.wrapping_add(w);
        }
        { let __v = total_w; compat::write(__this, |__t| __t.width = __v); }
        let mut total_h: i32 = (-element::PAD_S).wrapping_div(2);
        for h in compat::read(__this, |__t| __t.row_heights.clone()) {
            total_h = total_h.wrapping_add(h);
        }
        { let __v = total_h; compat::write(__this, |__t| __t.height = __v); }
    }

}

impl Element for TableElement {
    fn element_base(&self) -> &ElementBase {
        &self.base
    }
    fn element_base_mut(&mut self) -> &mut ElementBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int xpos, int ypos) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn Element>, &mut GuiGraphicsExtractor, i32, i32) {
        TableElement::extract_render_state_dyn
    }

}

impl TableElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int xpos, int ypos)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut xpos: i32, mut ypos: i32) {
        let mut y_off: i32 = 0;
        for y in 0..compat::read(__this, |__t| __t.rows) {
            let mut col: i32 = __this.lock().borrow().row_borders[(y) as usize];
            if col != 0 {
                GuiHelper::border(graphics, xpos, ypos.wrapping_add(y_off), (compat::read(__this, |__t| __t.width)).wrapping_add(element::PAD_S), __this.lock().borrow().row_heights[(y) as usize], col);
            }
            y_off = y_off.wrapping_add(__this.lock().borrow().row_heights[(y) as usize]);
        }
        let mut x_off: i32 = 0;
        for x in 0..compat::read(__this, |__t| __t.cols) {
            if compat::read(__this, |__t| __t.draw_lines) && x != 0 {
                let mut line_x1: i32 = ((xpos.wrapping_add(x_off)).wrapping_sub(element::PAD_S)).wrapping_sub(1);
                let mut line_x2: i32 = (xpos.wrapping_add(x_off)).wrapping_sub(element::PAD_S);
                let mut line_y1: i32 = ypos.wrapping_add(1);
                let mut line_y2: i32 = ((ypos.wrapping_add(compat::read(__this, |__t| __t.height))).wrapping_sub(element::PAD_S)).wrapping_sub(1);
                graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), line_x1, line_y1, line_x2, line_y2, compat::read(__this, |__t| __t.color));
            }
            y_off = 0;
            for y in 0..compat::read(__this, |__t| __t.rows) {
                let mut comp: Option<compat::Shared<dyn Element>> = Some((__this.lock().borrow().comps[(x) as usize])[(y) as usize].clone());
                if let Some(comp) = &comp {
                    let mut pad: i32 = (if x == 0 && __this.lock().borrow().row_borders[(y) as usize] != 0 { (element::PAD_L).wrapping_div(2) } else { 0 });
                    ({ let __f = compat::dispatch(comp, |__t| __t.extract_render_state()); __f(comp, graphics, (xpos.wrapping_add(x_off)).wrapping_add(pad), (ypos.wrapping_add(y_off)).wrapping_add((((__this.lock().borrow().row_heights[(y) as usize]).wrapping_div(2)).wrapping_sub((compat::read(comp, |__t| __t.element_base().height)).wrapping_div(2))).wrapping_add(1))) });
                }
                y_off = y_off.wrapping_add(__this.lock().borrow().row_heights[(y) as usize]);
            }
            x_off = x_off.wrapping_add(__this.lock().borrow().col_widths[(x) as usize]);
        }
    }

    /// TableElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
