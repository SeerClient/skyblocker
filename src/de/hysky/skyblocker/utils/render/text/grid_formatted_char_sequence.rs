//! Ported from de.hysky.skyblocker.utils.render.text.GridFormattedCharSequence by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents;
use mcrust::net::minecraft::util::formatted_char_sequence;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::util::formatted_char_sink::FormattedCharSink;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public record GridFormattedCharSequence(GridComponent.Contents gridContents, FormattedCharSequence composite) implements FormattedCharSequence
#[derive(Clone)]
pub struct GridFormattedCharSequence {
    /// java: GridComponent.Contents gridContents
    pub grid_contents: Contents,
    /// java: FormattedCharSequence composite
    pub composite: Seq,
}

impl GridFormattedCharSequence {
    /// java: public GridFormattedCharSequence(GridComponent.Contents gridContents)
    pub fn new(grid_contents: Contents) -> Self {
        let columns: Vec<Seq> = grid_contents.components().iter().map(|__a0| __a0.get_visual_order_text()).collect::<Vec<_>>();
        let composite: Seq = formatted_char_sequence::composite(&columns);
        // java: this(gridContents, composite);
        Self::new_canonical(grid_contents, composite)
    }

    /// java: the canonical record constructor
    pub fn new_canonical(grid_contents: Contents, composite: Seq) -> Self {
        Self { grid_contents, composite }
    }

    /// java: the record accessor `gridContents()`
    pub fn grid_contents(&self) -> Contents {
        self.grid_contents.clone()
    }

    /// java: the record accessor `composite()`
    pub fn composite(&self) -> Seq {
        self.composite.clone()
    }

    /// java: @Override public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        // java: return o instanceof GridFormattedCharSequence other && this.gridContents.equals(other.gridContents);
        match o.downcast_ref::<GridFormattedCharSequence>() {
            Some(other) => self.grid_contents == other.grid_contents,
            None => false,
        }
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return compat::hash_code(&self.grid_contents);
    }

}

impl FormattedCharSequence for GridFormattedCharSequence {
    /// java: @Override public boolean accept(FormattedCharSink output)
    ///
    /// Logically, this should never be called. As rendering is done by `GridTooltipComponent`.
    /// But this is present as a fail-safe.
    fn accept(&self, output: &mut dyn FormattedCharSink) -> bool {
        return self.composite.accept(output);
    }
}

/// java: equals(Object) / hashCode() - the record compares by its grid contents only
impl PartialEq for GridFormattedCharSequence {
    fn eq(&self, other: &Self) -> bool {
        self.grid_contents == other.grid_contents
    }
}

impl Eq for GridFormattedCharSequence {}

impl std::hash::Hash for GridFormattedCharSequence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid_contents.hash(state);
    }
}
