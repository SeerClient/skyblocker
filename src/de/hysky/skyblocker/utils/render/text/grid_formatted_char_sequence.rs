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
pub struct GridFormattedCharSequence {
    /// java: GridComponent.Contents gridContents
    pub grid_contents: Contents,
    /// java: FormattedCharSequence composite
    pub composite: Seq,
}

impl GridFormattedCharSequence {
    /// java: public GridFormattedCharSequence(GridComponent.Contents gridContents)
    pub fn new(mut grid_contents: Contents) -> Self {
        let mut columns: Vec<Seq> = grid_contents.components().into_iter().map(|__a0| __a0.get_visual_order_text()).collect::<Vec<_>>();
        let mut composite: Seq = formatted_char_sequence::composite(&columns);
        let mut this = Self {
            grid_contents: Default::default(),
            composite: Default::default(),
        };
        // TODO(fabric2seer): this statement
        //   java: this(gridContents, composite);
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut grid_contents: Contents, mut composite: Seq) -> Self {
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

    /// java: @Override public boolean accept(FormattedCharSink output)
    pub fn accept(&mut self, output: &dyn FormattedCharSink) -> bool {
        return self.composite.accept(&mut output);
    }

    /// java: @Override public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return o instanceof GridFormattedCharSequence other && this.gridContents.equals(other.gridContents);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return compat::hash_code(&self.grid_contents);
    }

}
