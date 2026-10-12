//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.TableWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::table_element::TableElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class TableWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct TableWidgetBase {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: private final int columns;
    pub columns: i32,
    /// java: private final int lineColor;
    pub line_color: i32,
    /// java: private final boolean drawLines;
    pub draw_lines: bool,
}

impl std::ops::Deref for TableWidgetBase {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TableWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TableWidgetBase {
    /// java: protected TableWidget(MutableComponent title, int colorValue, int columns, int lineColor, boolean drawLines, Information information) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, mut color_value: i32, mut columns: i32, mut line_color: i32, mut draw_lines: bool, mut information: Information, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TableWidget>, __R)) -> (compat::Shared<dyn TableWidget>, __R) {
        let __init_columns = columns;
        let __init_line_color = line_color;
        let __init_draw_lines = draw_lines;
        let (_, (__this, __r)) = ElementBasedWidgetBase::new_in(title.clone(), color_value, information.clone(), move |__base| {
            let this = Self {
                base: __base,
                columns: __init_columns,
                line_color: __init_line_color,
                draw_lines: __init_draw_lines,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of TableWidget (implemented by TableWidgetBase and every subclass)
pub trait TableWidget: ElementBasedWidget {
    fn table_widget_base(&self) -> &TableWidgetBase;
    fn table_widget_base_mut(&mut self) -> &mut TableWidgetBase;

    /// java: protected abstract List<Row> buildRows(); (dispatch: the body for this class)
    fn build_rows(&self) -> fn(&compat::Shared<dyn TableWidget>) -> Vec<Row>;

}

impl dyn TableWidget {
    /// java: @Override public void updateContent()
    pub fn update_content_body(__this: &compat::Shared<dyn TableWidget>) {
        let mut rows: Vec<Row> = ({ let __f = compat::dispatch(__this, |__t| __t.build_rows()); __f(__this) });
        let mut table: compat::Shared<TableElement> = TableElement::new(compat::read(__this, |__t| __t.table_widget_base().columns), (rows.len() as i32), compat::read(__this, |__t| __t.table_widget_base().line_color), compat::read(__this, |__t| __t.table_widget_base().draw_lines));
        for y in 0..(rows.len() as i32) {
            let row = &rows[(y) as usize];
            for x in 0..(compat::read(__this, |__t| __t.table_widget_base().columns)).min((row.cells.len() as i32)) {
                TableElement::add_to_cell(&table, x, y, row.cells[(x) as usize].clone());
            }
            if row.border_color != 0 {
                TableElement::set_row_border(&table, y, row.border_color);
            }
        }
        { let (__a0,) = ((table.clone() as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// TableWidget.updateContent as the dispatch of ElementBasedWidget.updateContent for the concrete class `__C`
    pub fn update_content_from<__C: TableWidget + 'static>(__this: &compat::Shared<dyn ElementBasedWidget>) {
        <dyn TableWidget>::update_content_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn TableWidget>))
    }

}

// TableWidget.updateContent implements ElementBasedWidget.updateContent: it is emitted in each concrete subclass's `impl ElementBasedWidget` (Rust has no inherited impls)
/// java: public static class Row
#[derive(Clone)]
pub struct Row {
    /// java: public final List<Element> cells;
    pub cells: Vec<compat::Shared<dyn Element>>,
    /// java: public final int borderColor;
    pub border_color: i32,
}

impl Row {
    /// java: public Row(List<Element> cells, int borderColor)
    pub fn new(mut cells: Vec<compat::Shared<dyn Element>>, mut border_color: i32) -> Self {
        let __init_cells = cells.clone();
        let __init_border_color = border_color;
        let mut this = Self {
            cells: __init_cells,
            border_color: __init_border_color,
        };
        this
    }

}
