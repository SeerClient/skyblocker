//! Ported from de.hysky.skyblocker.utils.render.state.gui.GuiOutlinedTextRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::font::PreparedText;
use mcrust::net::minecraft::client::gui::font::glyphs::baked_glyph::BakedGlyph;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::GuiTextRenderState;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;
use mcrust::com::mojang::blaze3d::font::glyph_info::GlyphInfo as _;

/// java: public class GuiOutlinedTextRenderState extends GuiTextRenderState
#[derive(Clone)]
pub struct GuiOutlinedTextRenderState {
    /// java: extends net.minecraft.client.renderer.state.gui.GuiTextRenderState
    pub base: GuiTextRenderState,
    /// java: private final int outlineColor;
    pub outline_color: i32,
    /// java: private @Nullable OutlineGlyphDrawable preparation;
    pub preparation: Option<OutlineGlyphDrawable>,
    /// java: private @Nullable ScreenRectangle bounds;
    pub bounds: Option<ScreenRectangle>,
}

impl std::ops::Deref for GuiOutlinedTextRenderState {
    type Target = GuiTextRenderState;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GuiOutlinedTextRenderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GuiOutlinedTextRenderState {
    /// java: public GuiOutlinedTextRenderState( Font textRenderer, FormattedCharSequence orderedText, Matrix3x2f matrix, int x, int y, int color, int outlineColor, boolean shadow, boolean trackEmpty, ScreenRectang
    pub fn new(mut text_renderer: Font, mut ordered_text: Seq, mut matrix: Matrix3x2f, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32, mut shadow: bool, mut track_empty: bool, mut clip_bounds: ScreenRectangle) -> Self {
        let __init_outline_color = outline_color;
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`prepared_text: PreparedText, pose: Matrix3x2f, scissor: Option<ScreenRectangle>`), Java passes 10
        //   java: super(textRenderer, orderedText, matrix, x, y, color, 0, shadow, trackEmpty, clipBounds);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: private PreparedText prepareOutline()
    pub fn prepare_outline(&self) -> PreparedText {
        // java: FontAccessor accessor = (FontAccessor) this.font; - the font itself
        let mut accessor = compat::font();
        // TODO(fabric2seer): not translated (expected ';' at 'drawer' (offset 1330))
        //   java: Font.PreparedTextBuilder drawer = this.font.new PreparedTextBuilder(0.0f, 0.0f, this.outlineColor, false, this.includeEmpty);
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    // TODO(fabric2seer): `GuiTextRenderState.x` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
                    // rust: let mut fs: Vec<f32> = vec![(self.x) as f32];
                    let mut fs: Vec<f32> = todo!("fabric2seer: the initialiser of `fs` (TODO above)");
                    let mut k: i32 = i;
                    let mut l: i32 = j;
                    // TODO(fabric2seer): `GuiTextRenderState.text` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
                    // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.util.FormattedCharSink`)
                    // rust: self.text.accept(|index, style, code_point| {
                    // rust: let mut bl: bool = style.is_bold();
                    // rust: // TODO(fabric2seer): `.invokeGetGlyph(..)` on a FontAccessor
                    // rust: //   java: BakedGlyph bakedGlyph = accessor.invokeGetGlyph(codePoint, style);
                    // rust: let mut baked_glyph: Box<dyn BakedGlyph> = todo!("fabric2seer: the initialiser of `bakedGlyph` (TODO above)");
                    // rust: // TODO(fabric2seer): unknown name `drawer`
                    // rust: //   java: drawer.x = fs[0] + k * bakedGlyph.info().getShadowOffset();
                    // rust: // TODO(fabric2seer): unknown name `drawer`
                    // rust: //   java: drawer.y = y + l * bakedGlyph.info().getShadowOffset();
                    // rust: fs[(0) as usize] = (fs[(0) as usize] + baked_glyph.info().get_advance_bold(bl));
                    // rust: // TODO(fabric2seer): unknown name `drawer`
                    // rust: //   java: return drawer.accept(index, style.withColor(this.outlineColor), bakedGlyph);
                    // rust: });
                }
            }
        }
        // TODO(fabric2seer): unknown name `drawer`
        //   java: return drawer;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private PreparedText prepareText()
    pub fn prepare_text(&mut self) -> PreparedText {
        // TODO(fabric2seer): `GuiTextRenderState.text` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.x` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.y` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.color` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.dropShadow` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.includeEmpty` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): `GuiTextRenderState.backgroundColor` has no Rust field in the port (searched net.minecraft.client.renderer.state.gui.GuiTextRenderState and its supertypes)
        // TODO(fabric2seer): the port's `prepare_text` ports this Java method with other parameters (text, x, y, color, drop_shadow, background_color for Java's text, x, y, original_color, draw_shadow, include_empty, background_color)
        // TODO(fabric2seer): `prepare_text` takes 6 argument(s) in the port (`text: &crate::net::minecraft::network::chat::formatted_text::FormattedText, x: f32, y: f32, color: i32, drop_shadow: bool, background_color: i32`), Java passes 7
        // rust: let mut text_preparation: PreparedText = compat::font().prepare_text(&self.text, ((self.x) as f32), ((self.y) as f32), self.color, self.drop_shadow, self.include_empty, self.background_color);
        let mut text_preparation: PreparedText = todo!("fabric2seer: the initialiser of `textPreparation` (TODO above)");
        // TODO(fabric2seer): `PreparedText.bounds` has no Rust method in the port (looked for `bounds` in Font and its supertypes)
        // rust: let mut screen_rect: Option<ScreenRectangle> = Some(text_preparation.bounds());
        let mut screen_rect: ScreenRectangle = todo!("fabric2seer: the initialiser of `screenRect` (TODO above)");
        if true /* java: screen_rect != null: never null here */ {
            screen_rect = screen_rect.transform_max_bounds(&self.pose);
            self.bounds = (if self.scissor.is_some() { self.scissor.as_ref().unwrap().intersection(screen_rect.clone()) } else { Some(screen_rect.clone()) });
        }
        return text_preparation;
    }

    /// java: @Override public PreparedText ensurePrepared()
    pub fn ensure_prepared(&mut self) -> OutlineGlyphDrawable {
        // (Java's static type is PreparedText; the object is the OutlineGlyphDrawable this class prepares)
        if self.preparation.is_none() {
            self.preparation = Some(OutlineGlyphDrawable::new(self.prepare_outline(), self.prepare_text()));
        }
        return self.preparation.clone().unwrap();
    }

    /// java: @Override public @Nullable ScreenRectangle bounds()
    pub fn bounds(&mut self) -> Option<ScreenRectangle> {
        self.ensure_prepared();
        return self.bounds.clone();
    }

}

// NOTE(fabric2seer): bounds, ensurePrepared override(s) of GuiTextRenderState - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record OutlineGlyphDrawable(PreparedText outline, PreparedText text) implements PreparedText
#[derive(Clone, Debug)]
pub struct OutlineGlyphDrawable {
    /// java: PreparedText outline
    pub outline: PreparedText,
    /// java: PreparedText text
    pub text: PreparedText,
}

impl OutlineGlyphDrawable {
    /// java: the canonical record constructor
    pub fn new(mut outline: PreparedText, mut text: PreparedText) -> Self {
        Self { outline, text }
    }

    /// java: the record accessor `outline()`
    pub fn outline(&self) -> PreparedText {
        self.outline.clone()
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> PreparedText {
        self.text.clone()
    }

    /// java: @Override public void visit(GlyphVisitor glyphDrawer)
    pub fn visit(&self, glyph_drawer: &crate::unresolved::Font_GlyphVisitor) {
        // TODO(fabric2seer): `PreparedText.visit` has no Rust method in the port (looked for `visit` in Font and its supertypes)
        // rust: self.outline.visit(glyph_drawer);
        // TODO(fabric2seer): `PreparedText.visit` has no Rust method in the port (looked for `visit` in Font and its supertypes)
        // rust: self.text.visit(glyph_drawer);
    }

    /// java: @Override public ScreenRectangle bounds()
    pub fn bounds(&self) -> ScreenRectangle {
        panic!("UnsupportedOperationException: {}", "For drawing only");
    }

}
